use futures::StreamExt;
use rusftx::endpoint::EndpointCom;
use rusftx::ws::incoming_message::subscribed::SubscribedMessage;
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

    println!("Logging in...");
    match ws.login(api_key, secret, None).await {
        Ok(ws) => ws,
        Err(err) => {
            println!("Could not login to FTX websocket: {:?}", err);
            return;
        }
    };
    println!("Login data sent");

    // println!("Subscribing to ticker channel for BTC-PERP...");
    // let subscribe_ticker_btc_message = SubscribeMessage::new()
    //     .market(Some("BTC-PERP".to_string()))
    //     .channel(Channel::Ticker)
    //     .build();
    // if let Err(err) = ws.send(&subscribe_ticker_btc_message).await {
    //     println!("Could not send subscribe message: {:?}", err);
    //     return;
    // }
    //
    // println!("Subscribing to markets channel");
    // let subscribe_markets_message = SubscribeMessage::new().channel(Channel::Markets).build();
    // if let Err(err) = ws.send(&subscribe_markets_message).await {
    //     println!("Could not send subscribe message: {:?}", err);
    //     return;
    // }

    let subscribe_orders_message = SubscribeMessage::new().channel(Channel::Orders).build();
    if let Err(err) = ws.send(&subscribe_orders_message).await {
        println!("Could not send subscribe message for orders: {:?}", err);
        return;
    }

    while let Some(message_result) = ws.next().await {
        match message_result {
            Ok(message) => match message {
                IncomingWebSocketApiMessage::Pong => {}
                IncomingWebSocketApiMessage::Subscribed(data) => match data {
                    SubscribedMessage {
                        channel,
                        market: Some(market),
                    } => {
                        println!(
                            "Subscribed to channel '{}' for market '{}'",
                            channel, market,
                        );
                    }
                    SubscribedMessage {
                        channel,
                        market: None,
                    } => {
                        println!("Subscribed to channel '{}'", channel);
                    }
                },
                IncomingWebSocketApiMessage::TickerUpdate(data) => {
                    println!(
                        "'{}': last price: {:?}, bid: {:?}, ask: {:?}",
                        data.market, data.last, data.bid, data.ask
                    );
                }
                IncomingWebSocketApiMessage::OrdersUpdate(data) => {
                    let order = data.order;
                    println!("Orders update for market '{}'", order.market);
                }
                IncomingWebSocketApiMessage::Markets(_data) => {
                    println!("Received markets data");
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
