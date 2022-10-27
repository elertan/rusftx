use crate::rest::model::order::{Order, OrderType};
use crate::rest::model::side::Side;
use crate::rest::request::{AuthenticatedRequest, Request};
use ::chrono::{DateTime, Utc};
use chrono::serde::ts_seconds_option;
use serde::Serialize;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetOrderHistoryRequest {
    #[default(None)]
    pub market: Option<String>,
    #[default(None)]
    pub side: Option<Side>,
    #[default(None)]
    pub order_type: Option<OrderType>,
    #[default(None)]
    pub start_time: Option<DateTime<Utc>>,
    #[default(None)]
    pub end_time: Option<DateTime<Utc>>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryRequestQuery {
    pub market: Option<String>,
    pub side: Option<Side>,
    pub order_type: Option<OrderType>,
    #[serde(with = "ts_seconds_option")]
    pub start_time: Option<DateTime<Utc>>,
    #[serde(with = "ts_seconds_option")]
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
        let request = GetOrderHistoryRequest::new().build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_order_history_with_market() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderHistoryRequest::new()
            .market(Some("BTC-PERP".to_string()))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_order_history_with_side() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderHistoryRequest::new().side(Some(Side::Buy)).build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_order_history_with_order_type() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderHistoryRequest::new()
            .order_type(Some(OrderType::Limit))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_order_history_with_start_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderHistoryRequest::new()
            .start_time(Some(Utc::now()))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_order_history_with_end_time() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOrderHistoryRequest::new()
            .end_time(Some(Utc::now()))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
