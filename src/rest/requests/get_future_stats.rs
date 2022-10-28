use crate::rest::model::future_stats::FutureStats;
use crate::rest::request::{AuthenticatedRequest, Request, UnauthenticatedRequest};
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetFutureStatsRequest {
    #[into]
    pub future_name: String,
}

impl Request for GetFutureStatsRequest {
    type Response = FutureStats;
    type Query = ();
    type Body = ();

    fn path(&self) -> Cow<str> {
        format!("futures/{}/stats", self.future_name).into()
    }

    fn method(&self) -> http::Method {
        http::Method::GET
    }
}

impl AuthenticatedRequest for GetFutureStatsRequest {}
impl UnauthenticatedRequest for GetFutureStatsRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_future_stats::GetFutureStatsRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn test_get_future_stats() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetFutureStatsRequest::new().future_name("BTC-PERP").build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
