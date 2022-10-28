use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

pub struct GetExpiredFuturesRequest;

impl Request for GetExpiredFuturesRequest {
    type Response = Vec<crate::rest::model::future::Future>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "expired_futures".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetExpiredFuturesRequest {}
impl UnauthenticatedRequest for GetExpiredFuturesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_expired_futures::GetExpiredFuturesRequest;

    #[tokio::test]
    async fn test_get_expired_futures() {
        let rest_api =
            crate::rest::requests::test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(GetExpiredFuturesRequest).await;
        assert!(result.is_ok());
    }
}
