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
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_cancel_order_request() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelOrderRequest::new().order_id(1).build();
        let result = rest_api.request(request).await;
        assert!(result.is_ok());
    }
}
