use crate::rest::model::side::Side;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct CancelAllOrdersRequest {
    #[default(None)]
    pub market: Option<String>,
    #[default(None)]
    pub side: Option<Side>,
    #[default(None)]
    pub conditional_orders_only: Option<bool>,
    #[default(None)]
    pub limit_orders_only: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
pub struct CancelAllOrdersRequestBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    market: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    conditional_orders_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    limit_orders_only: Option<bool>,
}

impl Request for CancelAllOrdersRequest {
    type Response = String;
    type Query = ();
    type Body = CancelAllOrdersRequestBody;

    fn path(&self) -> Cow<str> {
        "orders".into()
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }

    fn body(&self) -> Option<Self::Body> {
        Some(CancelAllOrdersRequestBody {
            market: self.market.clone(),
            side: self.side,
            conditional_orders_only: self.conditional_orders_only,
            limit_orders_only: self.limit_orders_only,
        })
    }
}

impl AuthenticatedRequest for CancelAllOrdersRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::model::side::Side;
    use crate::rest::requests::cancel_all_orders::CancelAllOrdersRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_cancel_all_orders() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelAllOrdersRequest::new().build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cancel_all_orders_by_market() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelAllOrdersRequest::new()
            .market(Some("BTC-PERP".into()))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cancel_all_orders_by_side() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelAllOrdersRequest::new().side(Some(Side::Buy)).build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cancel_all_orders_conditional_orders_only() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelAllOrdersRequest::new()
            .conditional_orders_only(Some(true))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_cancel_all_orders_limit_orders_only() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CancelAllOrdersRequest::new()
            .limit_orders_only(Some(true))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
