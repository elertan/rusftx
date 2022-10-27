use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct DeleteSubaccountRequest {
    #[into]
    pub nickname: String,
}

#[derive(Debug, serde::Serialize)]
pub struct DeleteSubaccountRequestBody {
    pub nickname: String,
}

impl Request for DeleteSubaccountRequest {
    type Response = ();
    type Query = ();
    type Body = DeleteSubaccountRequestBody;

    fn path(&self) -> Cow<str> {
        "subaccounts".into()
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }

    fn body(&self) -> Option<Self::Body> {
        Some(DeleteSubaccountRequestBody {
            nickname: self.nickname.clone(),
        })
    }
}

impl AuthenticatedRequest for DeleteSubaccountRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::delete_subaccount::DeleteSubaccountRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn delete_subaccount() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = DeleteSubaccountRequest::new().nickname("test").build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
