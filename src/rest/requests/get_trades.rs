use crate::rest::model::trade::Trade;
use crate::rest::request::{AuthenticatedRequest, Request};
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetTradesRequest {
    #[into]
    pub market_name: String,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetTradesRequestQuery {
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetTradesRequest {
    type Response = Vec<Trade>;
    type Query = GetTradesRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("markets/{}/trades", self.market_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetTradesRequestQuery {
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetTradesRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_trades::GetTradesRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_trades() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetTradesRequest::new().market_name("BTC-PERP").build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_trades_with_start_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetTradesRequest::new()
            .market_name("BTC-PERP")
            .start_time(Some(chrono::Utc::now() - chrono::Duration::days(1)))
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
