use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetFutureRequest {
    #[into]
    pub future_name: String,
}

impl Request for GetFutureRequest {
    type Response = crate::rest::model::future::Future;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("futures/{}", self.future_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetFutureRequest {}
impl UnauthenticatedRequest for GetFutureRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_future::GetFutureRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_future() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetFutureRequest::new().future_name("BTC-PERP").build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
