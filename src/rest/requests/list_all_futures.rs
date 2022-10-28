use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

pub struct ListAllFuturesRequest;

impl Request for ListAllFuturesRequest {
    type Response = Vec<crate::rest::model::future::Future>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "futures".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for ListAllFuturesRequest {}
impl UnauthenticatedRequest for ListAllFuturesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::list_all_futures::ListAllFuturesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_list_all_futures() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(ListAllFuturesRequest).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
