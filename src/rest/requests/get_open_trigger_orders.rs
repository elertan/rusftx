use crate::rest::model::trigger_order::{TriggerOrder, TriggerOrderType, TriggerOrderTypeQuery};
use crate::rest::request::{AuthenticatedRequest, Request};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetOpenTriggerOrdersRequest {
    #[default(None)]
    pub market: Option<String>,
    #[default(None)]
    pub r#type: Option<TriggerOrderType>,
}

#[derive(Debug, serde::Serialize)]
pub struct GetOpenTriggerOrdersQuery {
    pub market: Option<String>,
    pub r#type: Option<TriggerOrderTypeQuery>,
}

impl Request for GetOpenTriggerOrdersRequest {
    type Response = Vec<TriggerOrder>;
    type Query = GetOpenTriggerOrdersQuery;
    type Body = ();

    fn path(&self) -> Cow<str> {
        "conditional_orders".into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }

    fn query(&self) -> Option<Self::Query> {
        Some(GetOpenTriggerOrdersQuery {
            market: self.market.clone(),
            r#type: self.r#type.map(TriggerOrderTypeQuery::from),
        })
    }
}

impl AuthenticatedRequest for GetOpenTriggerOrdersRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::model::trigger_order::TriggerOrderType;
    use crate::rest::requests::get_open_trigger_orders::GetOpenTriggerOrdersRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_open_trigger_orders() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenTriggerOrdersRequest::new().build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_open_trigger_orders_with_market() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenTriggerOrdersRequest::new()
            .market(Some("BTC-PERP".into()))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_open_trigger_orders_with_type() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenTriggerOrdersRequest::new()
            .r#type(Some(TriggerOrderType::TakeProfit))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_open_trigger_orders_with_market_and_type() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenTriggerOrdersRequest::new()
            .market(Some("BTC-PERP".into()))
            .r#type(Some(TriggerOrderType::TakeProfit))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_open_trigger_orders_with_market_and_type_stop() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenTriggerOrdersRequest::new()
            .market(Some("BTC-PERP".into()))
            .r#type(Some(TriggerOrderType::Stop))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_get_open_trigger_orders_with_market_and_type_traling_stop_limit() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetOpenTriggerOrdersRequest::new()
            .market(Some("BTC-PERP".into()))
            .r#type(Some(TriggerOrderType::TrailingStop))
            .build();
        let result = rest_api.send(request).await;
        assert!(result.is_ok());
    }
}
