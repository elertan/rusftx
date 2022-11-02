#[derive(Debug)]
pub enum WebSocketApiError {
    Ws(tokio_tungstenite::tungstenite::Error),
    Serde(serde_json::Error),
    UnsupportedMessageType,
}
