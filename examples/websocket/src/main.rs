use futures::StreamExt;
use rusftx::endpoint::EndpointCom;
use rusftx::ws::incoming_message::IncomingWebSocketApiMessage;
use rusftx::ws::message::subscribe::SubscribeMessage;
use rusftx::ws::message::Channel;
use rusftx::ws::WebSocketApi;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let api_key = std::env::var("FTX_API_KEY").expect("FTX_API_KEY is not set");
    let secret = std::env::var("FTX_SECRET").expect("FTX_SECRET is not set");

    println!("Connecting to websocket api...");
    let mut ws = match WebSocketApi::connect(EndpointCom).await {
        Ok(ws) => ws,
        Err(err) => {
            println!("Could not connect to FTX websocket: {:?}", err);
            return;
        }
    };
    println!("Connected to websocket api!");
    // let mut ws = match ws.login(api_key, secret).await {
    //     Ok(ws) => ws,
    //     Err(err) => {
    //         println!("Could not login to FTX websocket: {:?}", err);
    //         return;
    //     }
    // };

    println!("Subscribing to ticker channel for BTC-PERP...");
    let subscribe_message = SubscribeMessage::new()
        .market("BTC-PERP")
        .channel(Channel::Ticker)
        .build();
    if let Err(err) = ws.send(&subscribe_message).await {
        println!("Could not send subscribe message: {:?}", err);
        return;
    }

    while let Some(message_result) = ws.next().await {
        match message_result {
            Ok(message) => match message {
                IncomingWebSocketApiMessage::Pong => {}
                IncomingWebSocketApiMessage::Subscribed(data) => {
                    println!(
                        "Subscribed to channel '{}' for market '{}'",
                        data.channel, data.market
                    );
                }
                IncomingWebSocketApiMessage::TickerUpdate(data) => {
                    println!(
                        "'{}': last price: {:?}, bid: {:?}, ask: {:?}",
                        data.market, data.last, data.bid, data.ask
                    );
                }
                _ => println!("Received message: {:?}", message),
            },
            Err(err) => {
                println!("Error while receiving message: {:?}", err);
                return;
            }
        }
    }
}
