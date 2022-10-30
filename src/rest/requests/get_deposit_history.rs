use crate::rest::model::deposit::Deposit;
use crate::rest::request::{AuthenticatedRequest, Request};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use http::Method;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetDepositHistoryRequest {
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetDepositHistoryRequestQuery {
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetDepositHistoryRequest {
    type Response = Vec<Deposit>;
    type Query = GetDepositHistoryRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/deposits".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetDepositHistoryRequestQuery {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetDepositHistoryRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_deposit_history::GetDepositHistoryRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_deposit_history() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetDepositHistoryRequest::new().build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
