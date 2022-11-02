use crate::rest::model::fill::Fill;
use crate::rest::request::{AuthenticatedRequest, Request};
use crate::rest::requests::QueryOrder;
use chrono::serde::ts_seconds_option;
use chrono::{DateTime, Utc};
use http::Method;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetFillsRequest {
    #[default(None)]
    pub market: Option<String>,
    #[default(None)]
    pub order: Option<QueryOrder>,
    #[default(None)]
    pub order_id: Option<u64>,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetFillsRequestQuery {
    pub market: Option<String>,
    pub order: Option<QueryOrder>,
    #[serde(rename = "orderId")]
    pub order_id: Option<u64>,
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetFillsRequest {
    type Response = Vec<Fill>;
    type Query = GetFillsRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "fills".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetFillsRequestQuery {
            market: self.market.clone(),
            order: self.order,
            order_id: self.order_id,
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetFillsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_fills::GetFillsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_fills() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetFillsRequest::new().build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
