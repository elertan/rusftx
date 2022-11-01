#[derive(Debug)]
pub enum ConnectError {
    Ws(tokio_tungstenite::tungstenite::Error),
}

#[derive(Debug)]
pub enum WebSocketApiError {
    Ws(tokio_tungstenite::tungstenite::Error),
}
