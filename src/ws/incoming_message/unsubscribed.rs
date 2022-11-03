use crate::ws::message::Channel;

#[derive(Debug)]
pub struct UnsubscribedMessage {
    pub market: Option<String>,
    pub channel: Channel,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawUnsubscribedMessage {
    #[serde(rename = "type")]
    _type: UnsubscribedType,
    pub market: Option<String>,
    pub channel: Channel,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UnsubscribedType {
    #[serde(rename = "unsubscribed")]
    Unsubscribed,
}

impl From<RawUnsubscribedMessage> for UnsubscribedMessage {
    fn from(raw_unsubscribed_message: RawUnsubscribedMessage) -> Self {
        UnsubscribedMessage {
            market: raw_unsubscribed_message.market,
            channel: raw_unsubscribed_message.channel,
        }
    }
}
