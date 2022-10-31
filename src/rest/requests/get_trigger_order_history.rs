use crate::rest::model::order::OrderType;
use crate::rest::model::side::Side;
use crate::rest::model::trigger_order::{TriggerOrder, TriggerOrderType};
use crate::rest::request::{AuthenticatedRequest, Request};
use chrono::{DateTime, Utc};
use http::Method;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetTriggerOrderHistoryRequest {
    #[default(None)]
    pub market: Option<String>,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub side: Option<Side>,
    #[default(None)]
    pub r#type: Option<TriggerOrderType>,
    #[default(None)]
    pub order_type: Option<OrderType>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetTriggerOrderHistoryRequestQuery {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "chrono::serde::ts_seconds_option"
    )]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        with = "chrono::serde::ts_seconds_option"
    )]
    pub end_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<Side>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#type: Option<TriggerOrderType>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "orderType")]
    pub order_type: Option<OrderType>,
}

impl Request for GetTriggerOrderHistoryRequest {
    type Response = Vec<TriggerOrder>;
    type Query = GetTriggerOrderHistoryRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "conditional_orders/history".into()
    }

    fn method(&self) -> Method {
        Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetTriggerOrderHistoryRequestQuery {
            market: self.market.clone(),
            start_time: self.start_time,
            end_time: self.end_time,
            side: self.side,
            r#type: self.r#type,
            order_type: self.order_type,
        })
    }
}

impl AuthenticatedRequest for GetTriggerOrderHistoryRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_trigger_order_history::GetTriggerOrderHistoryRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_trigger_order_history() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetTriggerOrderHistoryRequest::new().build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
