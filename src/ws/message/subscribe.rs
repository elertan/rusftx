use crate::ws::message::Channel;

#[derive(Debug, serde::Serialize)]
pub struct SubscribeMessage {
    pub market: String,
    pub channel: Channel,
}
