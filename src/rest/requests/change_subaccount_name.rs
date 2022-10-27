use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct ChangeSubaccountNameRequest {
    #[into]
    pub nickname: String,
    #[into]
    pub new_nickname: String,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChangeSubaccountNameRequestBody {
    pub nickname: String,
    pub new_nickname: String,
}

impl Request for ChangeSubaccountNameRequest {
    type Response = ();
    type Query = ();
    type Body = ChangeSubaccountNameRequestBody;

    fn path(&self) -> Cow<str> {
        "subaccounts/update_name".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(ChangeSubaccountNameRequestBody {
            nickname: self.nickname.clone(),
            new_nickname: self.new_nickname.clone(),
        })
    }
}

impl AuthenticatedRequest for ChangeSubaccountNameRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::change_subaccount_name::ChangeSubaccountNameRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_change_subaccount_name() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = ChangeSubaccountNameRequest::new()
            .nickname("test")
            .new_nickname("test_renamed")
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
