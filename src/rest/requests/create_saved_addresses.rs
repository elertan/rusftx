use crate::rest::model::saved_address::SavedAddress;
use crate::rest::model::withdrawal::Protocol;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct CreateSavedAddressesRequest {
    #[into]
    pub coin: String,
    #[into]
    pub address: String,
    #[default(None)]
    pub wallet: Option<String>,
    #[into]
    pub address_name: String,
    #[default(false)]
    pub is_primetrust: bool,
    #[default(None)]
    pub tag: Option<String>,
    #[default(false)]
    pub whitelist: bool,
    #[default(None)]
    pub code: Option<String>,
    #[default(None)]
    pub protocol: Option<Protocol>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSavedAddressesRequestBody {
    pub coin: String,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wallet: Option<String>,
    pub address_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_primetrust: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub whitelist: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub protocol: Option<Protocol>,
}

impl Request for CreateSavedAddressesRequest {
    type Response = SavedAddress;
    type Query = ();
    type Body = CreateSavedAddressesRequestBody;

    fn path(&self) -> Cow<str> {
        "wallet/saved_addresses".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(CreateSavedAddressesRequestBody {
            coin: self.coin.clone(),
            address: self.address.clone(),
            wallet: self.wallet.clone(),
            address_name: self.address_name.clone(),
            is_primetrust: self.is_primetrust.then_some(true),
            tag: self.tag.clone(),
            whitelist: self.whitelist.then_some(true),
            code: self.code.clone(),
            protocol: self.protocol,
        })
    }
}

impl AuthenticatedRequest for CreateSavedAddressesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::create_saved_addresses::CreateSavedAddressesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn create_saved_address() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = CreateSavedAddressesRequest::new()
            .coin("AVAX")
            .address("0x1234567890123456789012345678901234567890")
            .address_name("rusftx-test")
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
