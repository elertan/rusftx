use crate::ws::message::Channel;

#[derive(Debug, serde::Serialize)]
pub struct UnsubscribeMessage {
    pub market: String,
    pub channel: Channel,
}
