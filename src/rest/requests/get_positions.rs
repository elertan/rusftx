use crate::rest::model::position::Position;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetPositionsRequest {
    #[default(None)]
    show_avg_price: Option<bool>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionsRequestQuery {
    show_avg_price: Option<bool>,
}

impl Request for GetPositionsRequest {
    type Response = Vec<Position>;
    type Query = GetPositionsRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "positions".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetPositionsRequestQuery {
            show_avg_price: self.show_avg_price,
        })
    }
}

impl AuthenticatedRequest for GetPositionsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_positions::GetPositionsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_positions() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetPositionsRequest::new().build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_positions_with_avg_price() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetPositionsRequest::new()
            .show_avg_price(Some(true))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
