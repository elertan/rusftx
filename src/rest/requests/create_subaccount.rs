use crate::rest::model::subaccount::Subaccount;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct CreateSubaccountRequest {
    #[into]
    pub nickname: String,
}

#[derive(Debug, serde::Serialize)]
pub struct CreateSubaccountRequestBody {
    pub nickname: String,
}

impl Request for CreateSubaccountRequest {
    type Response = Subaccount;
    type Query = ();
    type Body = CreateSubaccountRequestBody;

    fn path(&self) -> Cow<str> {
        "subaccounts".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(CreateSubaccountRequestBody {
            nickname: self.nickname.clone(),
        })
    }
}

impl AuthenticatedRequest for CreateSubaccountRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::create_subaccount::CreateSubaccountRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_create_subaccount() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CreateSubaccountRequest::new().nickname("sub1").build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
