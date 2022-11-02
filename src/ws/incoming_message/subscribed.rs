use crate::ws::message::Channel;

#[derive(Debug)]
pub struct SubscribedMessage {
    pub market: Option<String>,
    pub channel: Channel,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RawSubscribedMessage {
    #[serde(rename = "type")]
    _type: SubscribedType,
    pub market: Option<String>,
    pub channel: Channel,
}

#[derive(Debug, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum SubscribedType {
    #[serde(rename = "subscribed")]
    Subscribed,
}

impl From<RawSubscribedMessage> for SubscribedMessage {
    fn from(raw_subscribed_message: RawSubscribedMessage) -> Self {
        SubscribedMessage {
            market: raw_subscribed_message.market,
            channel: raw_subscribed_message.channel,
        }
    }
}
