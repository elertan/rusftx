use crate::rest::model::order::Order;
use crate::ws::incoming_message::UpdateType;

#[derive(Debug)]
pub struct OrdersUpdateMessage {
    pub order: Order,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawOrdersUpdateMessage {
    #[serde(rename = "type")]
    _type: UpdateType,
    #[serde(rename = "channel")]
    _channel: OrdersChannel,
    data: Order,
}

#[derive(Debug, serde::Deserialize)]
pub enum OrdersChannel {
    #[serde(rename = "orders")]
    Orders,
}

impl From<RawOrdersUpdateMessage> for OrdersUpdateMessage {
    fn from(raw_orders_message: RawOrdersUpdateMessage) -> Self {
        Self {
            order: raw_orders_message.data,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_from_json() {
        let json = include_str!("../../../tests/ws/orders_message.json");
        let _message: super::RawOrdersUpdateMessage = serde_json::from_str(json).unwrap();
    }
}
