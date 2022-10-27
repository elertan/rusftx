use std::borrow::Cow;
use crate::rest::model::order_type::OrderType;
use crate::rest::model::side::Side;
use crate::rest::request::{Request, AuthenticatedRequest};
use serde::{Serialize, Deserialize};
use crate::rest::model::order_status::OrderStatus;

#[derive(Debug, Clone, Serialize, builder_pattern::Builder)]
pub struct PlaceOrderRequest {
    #[into]
    market: String,
    side: Side,
    #[default(None)]
    price: Option<f64>,
    r#type: OrderType,
    size: f64,
    #[default(None)]
    reduce_only: Option<bool>,
    #[default(None)]
    ioc: Option<bool>,
    #[default(None)]
    post_only: Option<bool>,
    #[default(None)]
    client_id: Option<String>,
    #[default(None)]
    reject_on_price_band: Option<bool>,
    #[default(None)]
    reject_after_ts: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequestBody {
    market: String,
    side: Side,
    price: Option<f64>,
    r#type: OrderType,
    size: f64,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    ioc: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    client_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reject_on_price_band: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reject_after_ts: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderOutput {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub filled_size: f64,
    pub future: String,
    pub id: u64,
    pub market: String,
    pub price: f64,
    pub remaining_size: f64,
    pub side: Side,
    pub size: f64,
    pub status: OrderStatus,
    pub r#type: OrderType,
    pub reduce_only: bool,
    pub ioc: bool,
    pub post_only: bool,
    pub client_id: Option<String>,
}

impl From<&PlaceOrderRequest> for PlaceOrderRequestBody {
    fn from(r: &PlaceOrderRequest) -> Self {
        Self {
            market: r.market.clone(),
            side: r.side.clone(),
            price: r.price,
            r#type: r.r#type.clone(),
            size: r.size,
            reduce_only: r.reduce_only,
            ioc: r.ioc,
            post_only: r.post_only,
            client_id: r.client_id.clone(),
            reject_on_price_band: r.reject_on_price_band,
            reject_after_ts: r.reject_after_ts,
        }
    }
}

impl Request for PlaceOrderRequest {
    type Response = PlaceOrderOutput;
    type Query = ();
    type Body = PlaceOrderRequestBody;

    fn path(&self) -> Cow<str> {
        "orders".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(self.into())
    }
}

impl AuthenticatedRequest for PlaceOrderRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::model::order_type::OrderType;
    use crate::rest::model::side::Side;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_place_order_request() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = PlaceOrderRequest::new()
            .market("BTC-PERP")
            .side(Side::Buy)
            .r#type(OrderType::Limit)
            .size(0.001)
            .price(Some(1.0))
            .build();
        let result = rest_api.request(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}