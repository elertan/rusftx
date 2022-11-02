#[derive(Debug, serde::Deserialize)]
pub struct RawIncomingWebSocketApiPongMessage {
    #[serde(rename = "type")]
    _type: PongType,
}

#[derive(Debug, serde::Deserialize)]
enum PongType {
    #[serde(rename = "pong")]
    Pong,
}

#[cfg(test)]
mod tests {
    #[test]
    fn deserialize_json() {
        let json = include_str!("../../../tests/ws/pong_message.json");
        let _pong_message: super::RawIncomingWebSocketApiPongMessage =
            serde_json::from_str(json).unwrap();
    }
}
