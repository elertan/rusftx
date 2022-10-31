use crate::rest::model::coin::Method;
use crate::rest::model::withdrawal_fee::WithdrawalFee;
use crate::rest::request::{AuthenticatedRequest, Request};
use rust_decimal::Decimal;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetWithdrawalFeesRequest {
    #[into]
    pub coin: String,
    pub size: Decimal,
    #[into]
    pub address: String,
    #[default(None)]
    pub tag: Option<String>,
    #[default(None)]
    pub method: Option<Method>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalFeesRequestQuery {
    pub coin: String,
    pub size: Decimal,
    pub address: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
}

impl Request for GetWithdrawalFeesRequest {
    type Response = WithdrawalFee;
    type Body = ();
    type Query = GetWithdrawalFeesRequestQuery;

    fn path(&self) -> Cow<str> {
        "wallet/withdrawal_fee".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetWithdrawalFeesRequestQuery {
            coin: self.coin.clone(),
            size: self.size,
            address: self.address.clone(),
            tag: self.tag.clone(),
            method: self.method.clone(),
        })
    }
}

impl AuthenticatedRequest for GetWithdrawalFeesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_withdrawal_fees::GetWithdrawalFeesRequest;
    use crate::rest::requests::test_utils;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn get_withdrawal_fees() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetWithdrawalFeesRequest::new()
            .coin("USDC")
            .size(dec!(20000))
            .address("0x83a127952d266A6eA306c40Ac62A4a70668FE3BE")
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
