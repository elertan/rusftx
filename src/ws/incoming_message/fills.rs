use crate::rest::model::fill::Fill;
use crate::ws::incoming_message::UpdateType;

#[derive(Debug)]
pub struct FillsUpdatedMessage {
    pub fill: Fill,
}

#[derive(Debug, serde::Deserialize)]
pub struct RawFillsUpdatedMessage {
    #[serde(rename = "type")]
    pub _type: UpdateType,
    #[serde(rename = "channel")]
    pub _channel: FillsChannel,
    pub data: Fill,
}

#[derive(Debug, serde::Deserialize)]
pub enum FillsChannel {
    #[serde(rename = "fills")]
    Fills,
}

impl From<RawFillsUpdatedMessage> for FillsUpdatedMessage {
    fn from(raw_fills_updated_message: RawFillsUpdatedMessage) -> Self {
        FillsUpdatedMessage {
            fill: raw_fills_updated_message.data,
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_from_json() {
        let json = include_str!("../../../tests/ws/fills_message.json");
        let _fills_updated_message: super::RawFillsUpdatedMessage =
            serde_json::from_str(json).unwrap();
    }
}
