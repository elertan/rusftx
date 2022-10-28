use crate::rest::request::{AuthenticatedRequest, Request};
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct RequestHistoricalBalancesAndPositionsSnapshotRequest {
    #[into]
    pub accounts: Vec<String>,
    pub end_time: DateTime<Utc>,
}

#[derive(Debug, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestHistoricalBalancesAndPositionsSnapshotRequestBody {
    pub accounts: Vec<String>,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub end_time: DateTime<Utc>,
}

impl Request for RequestHistoricalBalancesAndPositionsSnapshotRequest {
    type Response = u64;
    type Query = ();
    type Body = RequestHistoricalBalancesAndPositionsSnapshotRequestBody;

    fn path(&self) -> Cow<str> {
        "historical_balances/requests".into()
    }

    fn method(&self) -> http::Method {
        http::Method::POST
    }

    fn body(&self) -> Option<Self::Body> {
        Some(RequestHistoricalBalancesAndPositionsSnapshotRequestBody {
            accounts: self.accounts.clone(),
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for RequestHistoricalBalancesAndPositionsSnapshotRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::request_historical_balances_and_positions_snapshot::RequestHistoricalBalancesAndPositionsSnapshotRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_request_historical_balances_and_positions_snapshot() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = RequestHistoricalBalancesAndPositionsSnapshotRequest::new()
            .accounts(vec!["sub1".to_string(), "sub2".to_string()])
            .end_time(chrono::Utc::now())
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
