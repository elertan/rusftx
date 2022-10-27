use crate::rest::model::order::Order;
use crate::rest::request::{AuthenticatedRequest, Request};
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, builder_pattern::Builder)]
pub struct GetOpenOrdersRequest {
    pub market: Option<String>,
}

#[derive(Serialize)]
pub struct GetOpenOrdersRequestQuery {
    pub market: Option<String>,
}

impl Request for GetOpenOrdersRequest {
    type Response = Vec<Order>;
    type Query = GetOpenOrdersRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "orders".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetOpenOrdersRequestQuery {
            market: self.market.clone(),
        })
    }
}

impl AuthenticatedRequest for GetOpenOrdersRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_open_orders() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenOrdersRequest::default();
        let result = rest_api.request(request).await;
        assert!(result.is_ok());
    }
}
