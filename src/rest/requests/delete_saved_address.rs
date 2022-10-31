use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct DeleteSavedAddressRequest {
    pub saved_address_id: u64,
}

impl Request for DeleteSavedAddressRequest {
    type Response = String;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("wallet/saved_addresses/{}", self.saved_address_id).into()
    }

    fn method(&self) -> http::Method {
        http::Method::DELETE
    }
}

impl AuthenticatedRequest for DeleteSavedAddressRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::delete_saved_address::DeleteSavedAddressRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn delete_saved_address() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = DeleteSavedAddressRequest::new().saved_address_id(1).build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
