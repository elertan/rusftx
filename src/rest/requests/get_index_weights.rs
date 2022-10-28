use crate::rest::model::index_weights::IndexWeights;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetIndexWeightsRequest {
    #[into]
    pub index_name: String,
}

impl Request for GetIndexWeightsRequest {
    type Response = IndexWeights;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("indexes/{}/weights", self.index_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetIndexWeightsRequest {}
impl UnauthenticatedRequest for GetIndexWeightsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_index_weights::GetIndexWeightsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_index_weights() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetIndexWeightsRequest::new().index_name("ALT").build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
