use crate::rest::model::balance::Balance;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetSubaccountBalancesRequest {
    #[into]
    pub nickname: String,
}

impl Request for GetSubaccountBalancesRequest {
    type Response = Vec<Balance>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("subaccounts/{}/balances", self.nickname).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetSubaccountBalancesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_subaccount_balances::GetSubaccountBalancesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_subaccount_balances() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetSubaccountBalancesRequest::new().nickname("test").build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
