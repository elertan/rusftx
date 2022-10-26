pub trait RestEndpoint {
    fn rest(&self) -> &str;
}

pub trait WsEndpoint {
    fn ws(&self) -> &str;
}

#[derive(Default)]
pub struct EndpointCom;

impl RestEndpoint for EndpointCom {
    fn rest(&self) -> &str {
        "https://ftx.com/api"
    }
}

impl WsEndpoint for EndpointCom {
    fn ws(&self) -> &str {
        "wss://ftx.com/ws"
    }
}