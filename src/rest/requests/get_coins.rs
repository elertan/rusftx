use crate::rest::model::coin::Coin;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

pub struct GetCoinsRequest;

impl Request for GetCoinsRequest {
    type Response = Vec<Coin>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/coins".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetCoinsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_coins::GetCoinsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_coins() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(GetCoinsRequest).await;
        assert!(result.is_ok());
    }
}
