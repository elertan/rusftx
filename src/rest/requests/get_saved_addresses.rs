use crate::rest::model::saved_address::SavedAddress;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetSavedAddressesRequest {
    #[default(None)]
    pub coin: Option<String>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetSavedAddressesRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl Request for GetSavedAddressesRequest {
    type Response = Vec<SavedAddress>;
    type Query = GetSavedAddressesRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/saved_addresses".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetSavedAddressesRequestQuery {
            coin: self.coin.clone(),
        })
    }
}

impl AuthenticatedRequest for GetSavedAddressesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_saved_addresses::GetSavedAddressesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_saved_addresses() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetSavedAddressesRequest::new().build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_saved_addresses_for_coin() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetSavedAddressesRequest::new()
            .coin(Some("BTC".to_string()))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
