use crate::rest::model::candle::Candle;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetHistoricalPricesRequest {
    #[into]
    pub market_name: String,
    pub resolution: u32,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetHistoricalPricesRequestQuery {
    pub resolution: u32,
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetHistoricalPricesRequest {
    type Response = Vec<Candle>;
    type Query = GetHistoricalPricesRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("markets/{}/candles", self.market_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetHistoricalPricesRequestQuery {
            resolution: self.resolution,
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetHistoricalPricesRequest {}
impl UnauthenticatedRequest for GetHistoricalPricesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_historical_prices::GetHistoricalPricesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_historical_prices() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetHistoricalPricesRequest::new()
            .market_name("BTC-PERP")
            .resolution(60)
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
