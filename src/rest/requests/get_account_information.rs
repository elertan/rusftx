use crate::rest::model::account_information::AccountInformation;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

pub struct GetAccountInformationRequest;

impl Request for GetAccountInformationRequest {
    type Response = AccountInformation;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "account".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetAccountInformationRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_account_information::GetAccountInformationRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_account_information() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(GetAccountInformationRequest).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
