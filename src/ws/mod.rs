use crate::endpoint::WsEndpoint;
use crate::ws::error::WebSocketApiError;
use crate::ws::incoming_message::{IncomingWebSocketApiMessage, RawIncomingWebSocketApiMessage};
use crate::ws::message::login::{LoginMessage, LoginMessageArgs};
use crate::ws::message::ping::PingMessage;
use crate::ws::message::WebSocketApiMessage;
use chrono::Utc;
use futures::{
    ready,
    task::{Context, Poll},
    Future, SinkExt, Stream, StreamExt,
};
use std::pin::Pin;
use tokio::net::TcpStream;
use tokio::time::Interval;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream};

pub mod error;
pub mod incoming_message;
pub mod message;

pub struct WebSocketApi<TEndpoint>
where
    TEndpoint: WsEndpoint,
{
    endpoint: TEndpoint,
    stream: WebSocketStream<MaybeTlsStream<TcpStream>>,
    ping_interval: Interval,
}

impl<TEndpoint> WebSocketApi<TEndpoint>
where
    TEndpoint: WsEndpoint,
{
    pub async fn connect(endpoint: TEndpoint) -> Result<Self, WebSocketApiError> {
        let (stream, _) = tokio_tungstenite::connect_async(endpoint.ws())
            .await
            .map_err(WebSocketApiError::Ws)?;

        Ok(Self {
            endpoint,
            stream,
            ping_interval: tokio::time::interval(std::time::Duration::from_secs(15)),
        })
    }

    pub async fn login(
        mut self,
        api_key: String,
        secret: String,
        subaccount: Option<String>,
    ) -> Result<WebSocketApiWithAuthentication<TEndpoint>, WebSocketApiError> {
        let now = Utc::now();
        let timestamp_millis = now.timestamp_millis();
        let payload = format!("{}websocket_login", timestamp_millis);
        let sign = hmac_sha256::HMAC::mac(payload.as_bytes(), secret.as_bytes());
        let sign = hex::encode(sign);

        let login_message = LoginMessage::new()
            .args(
                LoginMessageArgs::new()
                    .key(api_key)
                    .sign(sign)
                    .time(now)
                    .subaccount(subaccount)
                    .build(),
            )
            .build();
        self.send(&login_message).await?;

        Ok(WebSocketApiWithAuthentication {
            endpoint: self.endpoint,
            stream: self.stream,
            ping_interval: self.ping_interval,
        })
    }

    pub async fn send<T>(&mut self, web_socket_api_message: &T) -> Result<(), WebSocketApiError>
    where
        T: WebSocketApiMessage + serde::Serialize,
    {
        let message_text =
            serde_json::to_string(&web_socket_api_message).map_err(WebSocketApiError::Serde)?;
        let message = Message::Text(message_text);
        self.stream
            .send(message)
            .await
            .map_err(WebSocketApiError::Ws)?;

        Ok(())
    }

    async fn poll_async(&mut self) -> Result<IncomingWebSocketApiMessage, WebSocketApiError> {
        loop {
            tokio::select! {
                _ = self.ping_interval.tick() => {
                    self.send(&PingMessage).await?;
                },
                Some(message_result) = self.stream.next() => {
                    let message: Message = message_result.map_err(WebSocketApiError::Ws)?;
                    let text = match message {
                        Message::Text(text) => text,
                        Message::Binary(_) => return Err(WebSocketApiError::UnsupportedMessageType),
                        Message::Ping(_) => return Err(WebSocketApiError::UnsupportedMessageType),
                        Message::Pong(_) => return Err(WebSocketApiError::UnsupportedMessageType),
                        Message::Frame(_) => return Err(WebSocketApiError::UnsupportedMessageType),
                        Message::Close(_) => return Err(WebSocketApiError::UnsupportedMessageType),
                    };
                    let raw_incoming_message = serde_json::from_str::<RawIncomingWebSocketApiMessage>(&text).map_err(WebSocketApiError::Serde)?;
                    return Ok(raw_incoming_message.into());
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
    ping_interval: Interval,
}

impl<TEndpoint> Stream for WebSocketApi<TEndpoint>
where
    TEndpoint: WsEndpoint + Unpin,
{
    type Item = Result<IncomingWebSocketApiMessage, WebSocketApiError>;

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
