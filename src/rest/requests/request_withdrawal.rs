use crate::rest::model::coin::Method;
use crate::rest::model::withdrawal::Protocol;
use crate::rest::request::{AuthenticatedRequest, Request};
use rust_decimal::Decimal;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct RequestWithdrawalRequest {
    #[into]
    pub coin: String,
    pub size: Decimal,
    #[into]
    pub address: String,
    #[default(None)]
    pub tag: Option<String>,
    #[default(None)]
    pub method: Option<Method>,
    #[default(None)]
    pub password: Option<String>,
    #[default(None)]
    pub code: Option<String>,
    #[default(None)]
    pub protocol: Option<Protocol>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestWithdrawalRequestBody {
    pub coin: String,
    pub size: Decimal,
    pub address: String,
    pub tag: Option<String>,
    pub method: Option<Method>,
    pub password: Option<String>,
    pub code: Option<String>,
    pub protocol: Option<Protocol>,
}

impl Request for RequestWithdrawalRequest {
    type Response = ();
    type Query = ();
    type Body = RequestWithdrawalRequestBody;

    fn path(&self) -> Cow<str> {
        "wallet/withdrawals".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(RequestWithdrawalRequestBody {
            coin: self.coin.clone(),
            size: self.size,
            address: self.address.clone(),
            tag: self.tag.clone(),
            method: self.method.clone(),
            password: self.password.clone(),
            code: self.code.clone(),
            protocol: self.protocol,
        })
    }
}

impl AuthenticatedRequest for RequestWithdrawalRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::request_withdrawal::RequestWithdrawalRequest;
    use crate::rest::requests::test_utils;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn request_withdrawal_request() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = RequestWithdrawalRequest::new()
            .coin("BTC")
            .size(dec!(0.0001))
            .address("0000000000000000000000000000000000")
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
