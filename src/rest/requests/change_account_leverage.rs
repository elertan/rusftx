use crate::rest::request::{AuthenticatedRequest, Request};
use rust_decimal::Decimal;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct ChangeAccountLeverageRequest {
    pub leverage: Decimal,
}

#[derive(Debug, serde::Serialize)]
pub struct ChangeAccountLeverageRequestBody {
    pub leverage: Decimal,
}

impl Request for ChangeAccountLeverageRequest {
    type Response = ();
    type Query = ();
    type Body = ChangeAccountLeverageRequestBody;

    fn path(&self) -> Cow<str> {
        "account/leverage".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(ChangeAccountLeverageRequestBody {
            leverage: self.leverage,
        })
    }
}

impl AuthenticatedRequest for ChangeAccountLeverageRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::change_account_leverage::ChangeAccountLeverageRequest;
    use crate::rest::requests::test_utils;
    use rust_decimal_macros::dec;

    #[tokio::test]
    async fn change_account_leverage() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = ChangeAccountLeverageRequest::new()
            .leverage(dec!(10))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
