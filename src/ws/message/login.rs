use crate::ws::message::{Operation, WebSocketApiMessage};
use chrono::{DateTime, Utc};

#[derive(Debug, builder_pattern::Builder)]
pub struct LoginMessage {
    pub args: LoginMessageArgs,
}

impl WebSocketApiMessage for LoginMessage {}

#[derive(Debug, serde::Serialize, builder_pattern::Builder)]
#[serde(rename_all = "camelCase")]
pub struct LoginMessageArgs {
    #[into]
    pub key: String,
    #[into]
    pub sign: String,
    #[serde(with = "chrono::serde::ts_milliseconds")]
    pub time: DateTime<Utc>,
    #[default(None)]
    pub subaccount: Option<String>,
}

impl serde::ser::Serialize for LoginMessage {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        use serde::ser::SerializeStruct;

        let mut state = serializer.serialize_struct("LoginMessage", 2)?;
        state.serialize_field("op", &Operation::Login)?;
        state.serialize_field("args", &self.args)?;
        state.end()
    }
}
