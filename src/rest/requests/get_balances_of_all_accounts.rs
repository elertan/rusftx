use crate::rest::model::balance::BalancesPerAccount;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

pub struct GetBalancesOfAllAccountsRequest;

impl Request for GetBalancesOfAllAccountsRequest {
    type Response = BalancesPerAccount;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/all_balances".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetBalancesOfAllAccountsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_balances_of_all_accounts::GetBalancesOfAllAccountsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_balances_of_all_accounts() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(GetBalancesOfAllAccountsRequest).await;
        assert!(result.is_ok());
    }
}
