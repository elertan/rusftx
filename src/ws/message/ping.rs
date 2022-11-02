use crate::ws::incoming_message::IncomingWebSocketApiMessage;
use crate::ws::message::{Operation, WebSocketApiMessage};
use futures::Stream;

pub struct PingMessage;

impl WebSocketApiMessage for PingMessage {}

impl serde::ser::Serialize for PingMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("PingMessage", 1)?;
        state.serialize_field("op", &Operation::Ping)?;
        state.end()
    }
}
