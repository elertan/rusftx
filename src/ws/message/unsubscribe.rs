use crate::ws::message::{Channel, Operation, WebSocketApiMessage};

#[derive(Debug, builder_pattern::Builder)]
pub struct UnsubscribeMessage {
    #[into]
    pub market: String,
    pub channel: Channel,
}

impl WebSocketApiMessage for UnsubscribeMessage {}

impl serde::ser::Serialize for UnsubscribeMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("SubscribeMessage", 3)?;
        state.serialize_field("op", &Operation::Unsubscribe)?;
        state.serialize_field("channel", &self.channel)?;
        state.serialize_field("market", &self.market)?;
        state.end()
    }
}
