use crate::rest::model::order::{Order, OrderStatus, OrderType};
use crate::rest::model::side::Side;
use crate::ws::incoming_message::UpdateType;
use rust_decimal::Decimal;

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
    id: u64,
    client_id: Option<String>,
    market: String,
    r#type: OrderType,
    side: Side,
    price: Option<Decimal>,
    size: Decimal,
    status: OrderStatus,
    filled_size: Decimal,
    remaining_size: Decimal,
    reduce_only: bool,
    liquidation: Option<bool>,
    avg_fill_price: Option<Decimal>,
    post_only: bool,
    ioc: bool,
    created_at: chrono::DateTime<chrono::Utc>,
    future: Option<String>,
    twap_order_id: Option<String>,
}

#[derive(Debug, serde::Deserialize)]
pub enum OrdersChannel {
    #[serde(rename = "orders")]
    Orders,
}

impl From<RawOrdersUpdateMessage> for OrdersUpdateMessage {
    fn from(raw_orders_message: RawOrdersUpdateMessage) -> Self {
        Self {
            order: Order {
                id: raw_orders_message.id,
                client_id: raw_orders_message.client_id,
                market: raw_orders_message.market,
                r#type: raw_orders_message.r#type,
                side: raw_orders_message.side,
                price: raw_orders_message.price,
                size: raw_orders_message.size,
                status: raw_orders_message.status,
                filled_size: raw_orders_message.filled_size,
                remaining_size: raw_orders_message.remaining_size,
                reduce_only: raw_orders_message.reduce_only,
                liquidation: raw_orders_message.liquidation,
                avg_fill_price: raw_orders_message.avg_fill_price,
                post_only: raw_orders_message.post_only,
                ioc: raw_orders_message.ioc,
                created_at: raw_orders_message.created_at,
                future: raw_orders_message.future,
                twap_order_id: raw_orders_message.twap_order_id,
            },
        }
    }
}
