use crate::rest::model::coin::Method;
use crate::rest::model::deposit_address::DepositAddressWithCoin;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetDepositAddressListRequest {
    pub items: Vec<GetDepositAddressItem>,
}

#[derive(Debug, builder_pattern::Builder)]
pub struct GetDepositAddressItem {
    #[into]
    pub coin: String,
    #[default(None)]
    pub method: Option<Method>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetDepositAddressListRequestBodyItem {
    pub coin: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<Method>,
}

impl Request for GetDepositAddressListRequest {
    type Response = Vec<DepositAddressWithCoin>;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/deposit_address/list".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }
}

impl AuthenticatedRequest for GetDepositAddressListRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::model::coin::Method;
    use crate::rest::model::deposit_address::DepositAddressWithCoin;
    use crate::rest::requests::get_deposit_address_list::{
        GetDepositAddressItem, GetDepositAddressListRequest,
    };
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_deposit_address_list() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetDepositAddressListRequest::new()
            .items(vec![
                GetDepositAddressItem::new()
                    .coin("USDT")
                    .method(Some(Method::ERC20))
                    .build(),
                GetDepositAddressItem::new().coin("ETH").build(),
            ])
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
        let response: Vec<DepositAddressWithCoin> = result.unwrap();
        assert_eq!(response.len(), 2);
        assert_eq!(response[0].coin, "USDT");

        assert_eq!(response[1].coin, "ETH");
    }
}
