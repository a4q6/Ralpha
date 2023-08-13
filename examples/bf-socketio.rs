use chrono::Utc;
use rust_socketio::{ClientBuilder, Payload, TransportType};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::time::Duration;
use time::OffsetDateTime;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Execution {
    pub id: u32,
    pub side: String,
    pub price: f64,
    pub size: f64,
    pub exec_date: String,
    pub buy_child_order_acceptance_id: String,
    pub sell_child_order_acceptance_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Ticker {
    pub product_code: String,
    pub state: String,
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub tick_id: u32,
    pub best_bid: f64,
    pub best_ask: f64,
    pub best_bid_size: f64,
    pub best_ask_size: f64,
    pub total_bid_depth: f64,
    pub total_ask_depth: f64,
    pub market_bid_size: f64,
    pub market_ask_size: f64,
    pub ltp: f64,
    pub volume: f64,
    pub volume_by_product: f64,
}

fn main() {
    // subject.observable().subscribe(
    //     |ticker: Ticker| println!("{:?}", to_string_pretty(&ticker)),
    //     |e| println!("Error: {:?}", e),
    //     || println!("Completed"),
    // );

    let client = ClientBuilder::new("https://io.lightstream.bitflyer.com")
        .transport_type(TransportType::Websocket)
        .on("lightning_ticker_FX_BTC_JPY", move |payload: Payload, _| {
            if let Payload::String(message) = payload {
                let ticker: Ticker = from_str(&message).unwrap();
                println!("{:?}", ticker.clone());
            }
        })
        .connect()
        .expect("Connection failed");

    // wait a few seconds to allow the connection to be established
    let wait_connect_seconds = Duration::from_secs(3);
    std::thread::sleep(wait_connect_seconds);

    client
        .emit("subscribe", "lightning_ticker_FX_BTC_JPY")
        .expect("Emit failed");

    loop {
        // Keeps the main thread alive to allow message processing in the background
        std::thread::sleep(Duration::from_secs(3));
        println!("{:?}", Utc::now());
    }
}
