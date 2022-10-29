use crate::rest::model::coin::Method;
use crate::rest::model::deposit_address::DepositAddress;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetDepositAddressRequest {
    #[into]
    pub coin: String,
    #[default(None)]
    pub method: Option<Method>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetDepositAddressRequestQuery {
    pub method: Option<Method>,
}

impl Request for GetDepositAddressRequest {
    type Response = DepositAddress;
    type Query = GetDepositAddressRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("wallet/deposit_address/{}", self.coin).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetDepositAddressRequestQuery {
            method: self.method.clone(),
        })
    }
}

impl AuthenticatedRequest for GetDepositAddressRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::model::coin::Method;
    use crate::rest::requests::get_deposit_address::GetDepositAddressRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_deposit_address() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetDepositAddressRequest::new().coin("BTC").build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_deposit_address_with_method() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetDepositAddressRequest::new()
            .coin("USDT")
            .method(Some(Method::ERC20))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
