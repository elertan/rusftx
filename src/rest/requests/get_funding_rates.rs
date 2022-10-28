use crate::rest::model::funding_rate::FundingRate;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetFundingRatesRequest {
    #[default(None)]
    pub future: Option<String>,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetFundingRatesRequestQuery {
    pub future: Option<String>,
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetFundingRatesRequest {
    type Response = Vec<FundingRate>;
    type Query = GetFundingRatesRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "funding_rates".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetFundingRatesRequestQuery {
            future: self.future.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetFundingRatesRequest {}
impl UnauthenticatedRequest for GetFundingRatesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_funding_rates::GetFundingRatesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_funding_rates() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetFundingRatesRequest::new().build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_funding_rates_for_future() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetFundingRatesRequest::new()
            .future(Some("BTC-PERP".to_string()))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_funding_rates_for_future_and_time_range() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetFundingRatesRequest::new()
            .start_time(Some(chrono::Utc::now()))
            .end_time(Some(chrono::Utc::now() + chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
