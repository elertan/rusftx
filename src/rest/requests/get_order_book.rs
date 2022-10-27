use crate::rest::model::orderbook::Orderbook;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetOrderBookRequest {
    #[into]
    pub market_name: String,
    #[default(None)]
    pub depth: Option<u32>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetOrderBookRequestQuery {
    pub depth: u32,
}

impl Request for GetOrderBookRequest {
    type Response = Orderbook;
    type Query = GetOrderBookRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("markets/{}/orderbook", self.market_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetOrderBookRequestQuery {
            depth: self.depth.unwrap_or(20),
        })
    }
}

impl AuthenticatedRequest for GetOrderBookRequest {}
impl UnauthenticatedRequest for GetOrderBookRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_order_book::GetOrderBookRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_order_book_request() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderBookRequest::new().market_name("BTC-PERP").build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_order_book_request_with_depth() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderBookRequest::new()
            .market_name("BTC-PERP")
            .depth(Some(10))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
