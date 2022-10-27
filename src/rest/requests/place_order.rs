use crate::rest::model::order::{Order, OrderType};
use crate::rest::model::side::Side;
use crate::rest::request::{AuthenticatedRequest, Request};
use rust_decimal::Decimal;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Clone, Serialize, builder_pattern::Builder)]
pub struct PlaceOrderRequest {
    #[into]
    pub market: String,
    pub side: Side,
    #[default(None)]
    pub price: Option<Decimal>,
    pub r#type: OrderType,
    pub size: Decimal,
    #[default(None)]
    pub reduce_only: Option<bool>,
    #[default(None)]
    pub ioc: Option<bool>,
    #[default(None)]
    pub post_only: Option<bool>,
    #[default(None)]
    pub client_id: Option<String>,
    #[default(None)]
    pub reject_on_price_band: Option<bool>,
    #[default(None)]
    pub reject_after_ts: Option<u64>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceOrderRequestBody {
    market: String,
    side: Side,
    price: Option<Decimal>,
    r#type: OrderType,
    size: Decimal,
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
    type Response = Order;
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
    use crate::rest::model::order::OrderType;
    use crate::rest::model::side::Side;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_place_order_request() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = PlaceOrderRequest::new()
            .market("BTC-PERP")
            .side(Side::Buy)
            .r#type(OrderType::Limit)
            .size(rust_decimal_macros::dec!(0.0001))
            .price(Some(rust_decimal_macros::dec!(1.0)))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
