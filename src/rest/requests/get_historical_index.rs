use crate::rest::model::candle::Candle;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetHistoricalIndexRequest {
    #[into]
    pub market_name: String,
    pub resolution: u32,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetHistoricalIndexRequestQuery {
    pub resolution: u32,
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetHistoricalIndexRequest {
    type Response = Vec<Candle>;
    type Query = GetHistoricalIndexRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("indexes/{}/candles", self.market_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetHistoricalIndexRequestQuery {
            resolution: self.resolution,
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetHistoricalIndexRequest {}
impl UnauthenticatedRequest for GetHistoricalIndexRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_historical_index::GetHistoricalIndexRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_historical_index() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetHistoricalIndexRequest::new()
            .market_name("BTC-PERP")
            .resolution(60)
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_historical_index_with_start_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetHistoricalIndexRequest::new()
            .market_name("BTC-PERP")
            .resolution(60)
            .start_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_historical_index_with_end_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetHistoricalIndexRequest::new()
            .market_name("BTC-PERP")
            .resolution(60)
            .end_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_historical_index_with_start_and_end_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetHistoricalIndexRequest::new()
            .market_name("BTC-PERP")
            .resolution(60)
            .start_time(Some(chrono::Utc::now()))
            .end_time(Some(chrono::Utc::now() + chrono::Duration::seconds(1)))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
