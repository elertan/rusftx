use crate::rest::model::historical_balances_and_positions_snapshot::HistoricalBalancesAndPositionsSnapshot;
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetHistoricalBalancesAndPositionsSnapshotRequest {
    pub request_id: u64,
}

impl Request for GetHistoricalBalancesAndPositionsSnapshotRequest {
    type Response = HistoricalBalancesAndPositionsSnapshot;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("historical_balances/requests/{}", self.request_id).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetHistoricalBalancesAndPositionsSnapshotRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_historical_balances_and_positions_snapshot::GetHistoricalBalancesAndPositionsSnapshotRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_historical_balances_and_positions_snapshot() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetHistoricalBalancesAndPositionsSnapshotRequest::new()
            .request_id(1)
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
