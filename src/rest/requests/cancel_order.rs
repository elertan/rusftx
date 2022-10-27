use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct CancelOrderRequest {
    pub order_id: u64,
}

impl Request for CancelOrderRequest {
    type Response = String;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("orders/{}", self.order_id).into()
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}

impl AuthenticatedRequest for CancelOrderRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::model::order::OrderType;
    use crate::rest::model::side::Side;
    use crate::rest::requests::place_order::PlaceOrderRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_cancel_order_request_non_existing_should_fail() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelOrderRequest::new().order_id(1).build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_cancel_order_request_by_placing_dummy_and_canceling_after() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();

        let order = {
            let request = PlaceOrderRequest::new()
                .market("BTC-PERP")
                .side(Side::Buy)
                .r#type(OrderType::Limit)
                .size(0.0001)
                .price(Some(1.0))
                .build();
            let result = rest_api.send(request).await;
            let order = result.unwrap();
            order
        };

        let request = CancelOrderRequest::new().order_id(order.id).build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
