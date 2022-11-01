use crate::endpoint::WsEndpoint;
use crate::ws::error::{ConnectError, WebSocketApiError};
use crate::ws::message::login::{LoginMessage, LoginMessageArgs};
use chrono::Utc;
use futures::{
    ready,
    task::{Context, Poll},
    Future, SinkExt, Stream, StreamExt,
};
use std::collections::VecDeque;
use std::ops::DerefMut;
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio::time::Interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub mod error;
pub mod message;

pub struct WebSocketApi<TEndpoint>
where
    TEndpoint: WsEndpoint,
{
    endpoint: TEndpoint,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    heartbeat_interval: Interval,
}

impl<TEndpoint> WebSocketApi<TEndpoint>
where
    TEndpoint: WsEndpoint,
{
    pub async fn connect(endpoint: TEndpoint) -> Result<Self, ConnectError> {
        let (stream, _) = tokio_tungstenite::connect_async(endpoint.ws())
            .await
            .map_err(ConnectError::Ws)?;

        Ok(Self {
            endpoint,
            stream,
            heartbeat_interval: tokio::time::interval(std::time::Duration::from_secs(15)),
        })
    }

    pub async fn login(
        mut self,
        api_key: String,
        secret: String,
        subaccount: Option<String>,
    ) -> Result<WebSocketApiWithAuthentication<TEndpoint>, ()> {
        let now = Utc::now();
        let timestamp_millis = now.timestamp_millis();
        let payload = format!("{}websocket_login", timestamp_millis);
        let sign = hmac_sha256::HMAC::mac(payload.as_bytes(), secret.as_bytes());
        let sign = hex::encode(sign);

        // let login_message = message::Message::Login(LoginMessage {
        //     args: LoginMessageArgs {
        //         key: api_key,
        //         sign,
        //         time: now,
        //         subaccount,
        //     },
        // });
        // let message_text = serde_json::to_string(&login_message).map_err(|_| ())?;

        let json = serde_json::json!({
            "op": "login",
            "args": {
                "key": api_key,
                "sign": sign,
                "time": timestamp_millis as u64,
                "subaccount": subaccount,
            }
        });
        let message = Message::Text(json.to_string());
        // let message = Message::Text(message_text);
        self.stream.send(message).await.map_err(|_| ())?;

        Ok(WebSocketApiWithAuthentication {
            endpoint: self.endpoint,
            stream: self.stream,
            heartbeat_interval: self.heartbeat_interval,
        })
    }

    async fn poll_async(&mut self) -> Result<Message, WebSocketApiError> {
        loop {
            tokio::select! {
                _ = self.heartbeat_interval.tick() => {
                    let message = Message::Text("ping".to_string());
                    self.stream.send(message).await.map_err(WebSocketApiError::Ws)?;
                },
                Some(message_result) = self.stream.next() => {
                    return message_result.map_err(WebSocketApiError::Ws);
                }
            }
        }
    }
}

pub struct WebSocketApiWithAuthentication<TEndpoint>
where
    TEndpoint: WsEndpoint,
{
    endpoint: TEndpoint,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    heartbeat_interval: Interval,
}

impl<TEndpoint> Stream for WebSocketApi<TEndpoint>
where
    TEndpoint: WsEndpoint + Unpin,
{
    type Item = Result<Message, WebSocketApiError>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let fut = self.poll_async();
        futures::pin_mut!(fut);
        let poll = fut.poll(cx);
        match poll {
            Poll::Ready(value) => Poll::Ready(Some(value)),
            Poll::Pending => Poll::Pending,
        }
    }
}
