use crate::rest::model::trigger::Trigger;
use crate::rest::request::{AuthenticatedRequest, Request};
use http::Method;
use std::borrow::Cow;

#[derive(Debug, builder_pattern::Builder)]
pub struct GetTriggerOrderTriggersRequest {
    pub conditional_order_id: u64,
}

impl Request for GetTriggerOrderTriggersRequest {
    type Body = ();
    type Query = ();
    type Response = Vec<Trigger>;

    fn path(&self) -> Cow<str> {
        format!("conditional_orders/{}/triggers", self.conditional_order_id).into()
    }

    fn method(&self) -> Method {
        Method::GET
    }
}

impl AuthenticatedRequest for GetTriggerOrderTriggersRequest {}

#[cfg(test)]
mod tests {
    use crate::rest::requests::get_trigger_order_triggers::GetTriggerOrderTriggersRequest;
    use crate::rest::requests::test_utils;

    #[tokio::test]
    async fn get_trigger_order_triggers() {
        let rest_api = test_utils::get_rest_api_with_authentication_from_env();
        let request = GetTriggerOrderTriggersRequest::new()
            .conditional_order_id(1)
            .build();
        let result = rest_api.send(request).await;
        dbg!(&result);
        assert!(result.is_ok());
    }
}
