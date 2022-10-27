use crate::rest::model::order::{Order, OrderType};
use crate::rest::model::side::Side;
use crate::rest::request::{AuthenticatedRequest, Request};
use ::chrono::{DateTime, Utc};
use chrono::serde::ts_milliseconds_option;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, Default, builder_pattern::Builder)]
pub struct GetOrderHistoryRequest {
    pub market: Option<String>,
    pub side: Option<Side>,
    pub order_type: Option<OrderType>,
    pub start_time: Option<DateTime<Utc>>,
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequestQuery {
    pub market: Option<String>,
    pub side: Option<Side>,
    pub order_type: Option<OrderType>,
    #[serde(with = "ts_milliseconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_milliseconds_option")]
    pub end_time: Option<DateTime<Utc>>,
}

impl Request for GetOrderHistoryRequest {
    type Response = Vec<Order>;
    type Query = GetOrderHistoryRequestQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "orders/history".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetOrderHistoryRequestQuery {
            market: self.market.clone(),
            side: self.side,
            order_type: self.order_type,
            start_time: self.start_time,
            end_time: self.end_time,
        })
    }
}

impl AuthenticatedRequest for GetOrderHistoryRequest {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_order_history() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderHistoryRequest::default();
        let result = rest_api.request(request).await;
        assert!(result.is_ok());
    }
}
