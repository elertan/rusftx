use crate::rest::model::withdrawal::Withdrawal;
use crate::rest::request::{AuthenticatedRequest, Request};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use http::Method;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetWithdrawalHistoryRequest {
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetWithdrawalHistoryRequestQuery {
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetWithdrawalHistoryRequest {
    type Response = Vec<Withdrawal>;
    type Query = GetWithdrawalHistoryRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/withdrawals".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetWithdrawalHistoryRequestQuery {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetWithdrawalHistoryRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_withdrawal_history::GetWithdrawalHistoryRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_withdrawal_history() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetWithdrawalHistoryRequest::new().build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_withdrawal_history_with_start_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetWithdrawalHistoryRequest::new()
            .start_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_withdrawal_history_with_end_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetWithdrawalHistoryRequest::new()
            .end_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
