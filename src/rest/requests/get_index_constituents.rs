use crate::rest::model::index_constituent::IndexConstituent;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetIndexConstituentsRequest {
    #[into]
    pub underlying: String,
}

impl Request for GetIndexConstituentsRequest {
    type Response = Vec<IndexConstituent>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("index_constituents/{}", self.underlying).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetIndexConstituentsRequest {}
impl UnauthenticatedRequest for GetIndexConstituentsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_index_constituents::GetIndexConstituentsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_index_constituents() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetIndexConstituentsRequest::new().underlying("BTC").build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
