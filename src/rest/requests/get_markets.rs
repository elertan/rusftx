use crate::rest::model::market::Market;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

#[derive(Debug, Default)]
pub struct GetMarketsRequest {}

impl Request for GetMarketsRequest {
    type Response = Vec<Market>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "markets".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl UnauthenticatedRequest for GetMarketsRequest {}
impl AuthenticatedRequest for GetMarketsRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::endpoint::EndpointCom;
    use crate::rest::RestApi;

    #[tokio::test]
    async fn test_get_markets_request() {
        let rest_api = RestApi::<EndpointCom>::default();
        let request = GetMarketsRequest::default();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
