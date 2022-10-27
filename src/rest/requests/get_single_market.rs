use std::borrow::Cow;
use crate::rest::model::market::Market;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};

#[derive(Debug, builder_pattern::Builder)]
pub struct GetSingleMarketRequest {
    market_name: String,
}

impl Request for GetSingleMarketRequest {
    type Response = Market;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("markets/{}", self.market_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}


impl UnauthenticatedRequest for GetSingleMarketRequest {}
impl AuthenticatedRequest for GetSingleMarketRequest {}

#[cfg(test)]
mod tests {
    use crate::endpoint::EndpointCom;
    use super::*;
    use crate::rest::RestApi;

    #[tokio::test]
    async fn test_get_single_market_request_btc_perp() {
        let rest_api = RestApi::<EndpointCom>::default();
        let request = GetSingleMarketRequest::new()
            .market_name("BTC-PERP".to_string())
            .build();
        let result = rest_api.request(request).await;
        assert!(result.is_ok());
        let market = result.unwrap();
        assert_eq!(market.name, "BTC-PERP");
    }
}