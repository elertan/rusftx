use crate::rest::model::transfer::Transfer;
use crate::rest::request::{AuthenticatedRequest, Request};
use rust_decimal::Decimal;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct TransferBetweenSubaccountsRequest {
    #[into]
    pub coin: String,
    pub size: Decimal,
    #[default(None)]
    pub source: Option<String>,
    #[into]
    pub destination: String,
}

#[derive(Debug, serde::Serialize)]
pub struct TransferBetweenSubaccountsRequestBody {
    pub coin: String,
    pub size: Decimal,
    pub source: Option<String>,
    pub destination: String,
}

impl Request for TransferBetweenSubaccountsRequest {
    type Response = Transfer;
    type Query = ();
    type Body = TransferBetweenSubaccountsRequestBody;

    fn path(&self) -> Cow<str> {
        "subaccounts/transfer".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(TransferBetweenSubaccountsRequestBody {
            coin: self.coin.clone(),
            size: self.size,
            source: self.source.clone(),
            destination: self.destination.clone(),
        })
    }
}

impl AuthenticatedRequest for TransferBetweenSubaccountsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::model::transfer::Transfer;
    use crate::rest::requests::test_utils;
    use crate::rest::requests::transfer_between_subaccounts::TransferBetweenSubaccountsRequest;

    #[tokio::test]
    async fn test_transfer_between_subaccounts() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = TransferBetweenSubaccountsRequest::new()
            .coin("BTC")
            .size(0.0001)
            .source(Some("sub1".to_string()))
            .destination("sub2")
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
        let transfer: Transfer = result.unwrap();
        assert_eq!(transfer.coin, "BTC");
        assert_eq!(transfer.size, 0.0001);
        assert_eq!(transfer.status, "complete");
    }
}
