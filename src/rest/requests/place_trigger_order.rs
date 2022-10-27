use crate::rest::model::side::Side;
use crate::rest::model::trigger_order::{TriggerOrder, TriggerOrderType};
use crate::rest::request::{AuthenticatedRequest, Request};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct PlaceTriggerOrderRequest {
    #[into]
    pub market: String,
    pub side: Side,
    pub size: f64,
    pub r#type: TriggerOrderType,
    #[default(None)]
    pub reduce_only: Option<bool>,
    #[default(None)]
    pub retry_until_filled: Option<bool>,
    #[default(None)]
    pub trigger_price: Option<f64>,
    #[default(None)]
    pub order_price: Option<f64>,
    #[default(None)]
    pub trail_value: Option<f64>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceTriggerOrderRequestBody {
    market: String,
    side: Side,
    size: f64,
    r#type: TriggerOrderType,
    #[serde(skip_serializing_if = "Option::is_none")]
    reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    retry_until_filled: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trigger_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    order_price: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    trail_value: Option<f64>,
}

impl Request for PlaceTriggerOrderRequest {
    type Response = TriggerOrder;
    type Query = ();
    type Body = PlaceTriggerOrderRequestBody;

    fn path(&self) -> Cow<str> {
        "conditional_orders".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(PlaceTriggerOrderRequestBody {
            market: self.market.clone(),
            side: self.side,
            size: self.size,
            r#type: self.r#type,
            reduce_only: self.reduce_only,
            retry_until_filled: self.retry_until_filled,
            trigger_price: self.trigger_price,
            order_price: self.order_price,
            trail_value: self.trail_value,
        })
    }
}

impl AuthenticatedRequest for PlaceTriggerOrderRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::model::side::Side;
    use crate::rest::model::trigger_order::TriggerOrderType;
    use crate::rest::requests::place_trigger_order::PlaceTriggerOrderRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_place_trigger_order_request_type_stop() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = PlaceTriggerOrderRequest::new()
            .market("BTC-PERP")
            .side(Side::Sell)
            .size(0.0001)
            .r#type(TriggerOrderType::Stop)
            .reduce_only(Some(true))
            .retry_until_filled(Some(true))
            .trigger_price(Some(0.0001))
            .build();

        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
