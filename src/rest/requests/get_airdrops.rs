use crate::rest::model::airdrop::Airdrop;
use crate::rest::request::{AuthenticatedRequest, Request};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetAirdropsRequest {
    #[default(None)]
    start_time: Option<DateTime<Utc>>,
    #[default(None)]
    end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetAirdropsRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    start_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none", with = "ts_seconds_option")]
    end_time: Option<DateTime<Utc>>,
}

impl Request for GetAirdropsRequest {
    type Response = Vec<Airdrop>;
    type Query = GetAirdropsRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "wallet/airdrops".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetAirdropsRequestQuery {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetAirdropsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_airdrops::GetAirdropsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_airdrops() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetAirdropsRequest::new().build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_airdrops_with_start_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetAirdropsRequest::new()
            .start_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn get_airdrops_with_end_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetAirdropsRequest::new()
            .end_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
