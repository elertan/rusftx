use crate::rest::model::market::Market;
use crate::rest::request::Request;

#[derive(Debug, Default)]
pub struct GetMarketsRequest {}

impl Request for GetMarketsRequest {
    type Response = Vec<Market>;

    fn path(&self) -> &str {
        "markets"
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

#[cfg(test)]
mod tests {
    use crate::endpoint::EndpointCom;
    use super::*;
    use crate::rest::RestApi;

    #[tokio::test]
    async fn test_get_markets_request() {
        let request = GetMarketsRequest::default();
        let rest_api = RestApi::<EndpointCom>::default();
        let result = rest_api.request(request).await;
        assert!(result.is_ok());
    }
}