use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct CancelOpenTriggerOrder {
    pub id: u64,
}

impl Request for CancelOpenTriggerOrder {
    type Response = String;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("conditional_orders/{}", self.id).into()
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}

impl AuthenticatedRequest for CancelOpenTriggerOrder {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::model::side::Side;
    use crate::rest::model::trigger_order::{TriggerOrder, TriggerOrderType};
    use crate::rest::requests::place_trigger_order::PlaceTriggerOrderRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_cancel_open_trigger_order_non_existing() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelOpenTriggerOrder::new().id(1).build();

        let result = rest_api.send(request).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cancel_open_trigger_order_by_placing_new_trigger_order_and_cancelling_it() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();

        let trigger_order: TriggerOrder = {
            let request = PlaceTriggerOrderRequest::new()
                .market("BTC-PERP")
                .side(Side::Sell)
                .size(rust_decimal_macros::dec!(0.0001))
                .r#type(TriggerOrderType::Stop)
                .reduce_only(Some(true))
                .retry_until_filled(Some(true))
                .trigger_price(Some(rust_decimal_macros::dec!(0.0001)))
                .build();
            let result = rest_api.send(request).await;
            let trigger_order = result.unwrap();
            trigger_order
        };

        let request = CancelOpenTriggerOrder::new().id(trigger_order.id).build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
