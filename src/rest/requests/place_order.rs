use std::borrow::Cow;
use crate::rest::model::order_type::OrderType;
use crate::rest::model::side::Side;
use crate::rest::request::{Request, AuthenticatedRequest};
use serde::{Serialize, Deserialize};
use crate::rest::model::order_status::OrderStatus;

#[derive(Debug, Clone, Serialize, builder_pattern::Builder)]
pub struct PlaceOrderRequest {
    market: String,
    side: Side,
    price: Option<f64>,
    r#type: OrderType,
    size: f64,
    reduce_only: Option<bool>,
    ioc: Option<bool>,
    post_only: Option<bool>,
    client_id: Option<String>,
    reject_on_price_band: Option<bool>,
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
    reduce_only: Option<bool>,
    ioc: Option<bool>,
    post_only: Option<bool>,
    client_id: Option<String>,
    reject_on_price_band: Option<bool>,
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
    use crate::rest::request::Request;
    use crate::rest::request::AuthenticatedRequest;
    use crate::rest::request::RestRequestBuilder;

    #[test]
    fn test_place_order_request() {
        // let rest_api = test_utils::get_authenticated_rest_api_from_env();
        let request = PlaceOrderRequest::new()
            .market("BTC-PERP".into())
            .side(Side::Buy)
            .r#type(OrderType::Limit)
            .size(0.001)
            .price(Some(1))
            .build()
            .unwrap();
    }
}