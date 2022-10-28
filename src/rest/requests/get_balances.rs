use crate::rest::model::balance::Balance;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

pub struct GetBalancesRequest;

impl Request for GetBalancesRequest {
    type Response = Vec<Balance>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/balances".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetBalancesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_balances::GetBalancesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_balances() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(GetBalancesRequest).await;
        assert!(result.is_ok());
    }
}
