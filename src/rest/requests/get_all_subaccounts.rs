use crate::rest::model::subaccount::Subaccount;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

pub struct GetAllSubaccountsRequest;

impl Request for GetAllSubaccountsRequest {
    type Response = Vec<Subaccount>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "subaccounts".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetAllSubaccountsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_all_subaccounts::GetAllSubaccountsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_all_subaccounts() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let result = rest_api.send(GetAllSubaccountsRequest).await;
        assert!(result.is_ok());
    }
}
