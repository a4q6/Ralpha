use crate::datamodels::enums::MarketData;
use crate::datamodels::marketdata::MarketBook;
use crate::feedhandler::bitflyer::datamodel::{Board, Execution};
use log::{error, info, warn};
use ordered_float::OrderedFloat;
use rust_socketio::{client::Client, ClientBuilder, Event, Payload, RawClient, TransportType};
use serde_json::from_str;
use std::collections::BTreeMap;
use std::thread::sleep;
use std::vec;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use time::OffsetDateTime;

type Shared<T> = Arc<Mutex<T>>;

pub struct BitFlyerSocketIo {
    pub callbacks: Shared<Vec<Box<dyn FnMut(MarketData) + Send>>>,
    client: Option<Client>,
    latest_book: Shared<BTreeMap<String, MarketBook>>,
    latest_bid: Shared<BTreeMap<String, f64>>,
    latest_ask: Shared<BTreeMap<String, f64>>,
}

impl BitFlyerSocketIo {
    pub fn new() -> Self {
        Self {
            client: None,
            callbacks: Arc::new(Mutex::new(vec![])),
            latest_book: Arc::new(Mutex::new(BTreeMap::new())),
            latest_bid: Arc::new(Mutex::new(BTreeMap::new())),
            latest_ask: Arc::new(Mutex::new(BTreeMap::new())),
        }
    }

    pub fn set_callback(&mut self, callback: Box<dyn FnMut(MarketData) + Send>) {
        self.callbacks.lock().unwrap().push(callback);
    }

    pub fn connect(mut self, channels: Vec<String>) {
        let client = ClientBuilder::new("https://io.lightstream.bitflyer.com")
            .transport_type(TransportType::Websocket)
            .reconnect_on_disconnect(true)
            .reconnect_delay(5000, 30000)
            .on("open", move |_payload: Payload, _raw_client: RawClient| {
                info!("Open socket to BitFlyer");
            })
            .on("close", move |_payload: Payload, _raw_client: RawClient| {
                warn!("Close socket to BitFlyer");
            })
            .on("error", move |_payload: Payload, _raw_client: RawClient| {
                error!("Error on socket to BitFlyer : {:#?}", _payload);
                sleep(Duration::from_secs(1));

                // sleep within maintenance.
                let now = OffsetDateTime::now_utc();
                if now.hour() == 19 && now.minute() < 10 {
                    info!("Sleep within maintenance. ...");
                    let maintenance_end =
                        now.replace_minute(15).unwrap().replace_second(0).unwrap();
                    sleep(Duration::from_secs_f64(
                        (maintenance_end - now).as_seconds_f64(),
                    ));
                }
            })
            .on_any(move |event: Event, payload: Payload, _: RawClient| {
                if let Payload::String(message) = payload {
                    match event.as_str() {
                        evt if evt.contains("executions") => {
                            // parse message
                            let sym = evt.split("executions_").last().unwrap().replace("_", "");
                            let executions_msg: Vec<Execution> = from_str(&message).unwrap();

                            // pass to callback
                            for exec in executions_msg {
                                let market_trade = exec.to_market_trade(sym.clone());
                                for callback in self.callbacks.lock().unwrap().iter_mut() {
                                    callback(MarketData::MarketTrade(market_trade.clone()));
                                }
                            }
                        }

                        evt if evt.contains("ticker") => {
                            // let ticker_msg: Ticker = from_str(&message).unwrap();
                            // let mut t = self.ticker.lock().unwrap();
                            // *t = ticker_msg;
                        }

                        evt if evt.contains("board_snapshot") => {
                            // parse message
                            let sym = evt
                                .split("board_snapshot_")
                                .last()
                                .unwrap()
                                .replace("_", "");
                            let board_msg: Board = from_str(&message).unwrap();
                            let market_book = board_msg.to_market_book(sym.clone());
                            let rate = market_book.to_rate();

                            // update latest book
                            let mut books = self.latest_book.lock().unwrap();
                            books.insert(sym.clone(), market_book.clone());

                            // update latest rate
                            self.latest_ask
                                .lock()
                                .unwrap()
                                .insert(sym.clone(), rate.best_ask);
                            self.latest_bid
                                .lock()
                                .unwrap()
                                .insert(sym.clone(), rate.best_bid);

                            // pass to callback - Rate
                            for callback in self.callbacks.lock().unwrap().iter_mut() {
                                callback(MarketData::Rate(rate.clone()));
                            }

                            // pass to callback - MarketBook
                            for callback in self.callbacks.lock().unwrap().iter_mut() {
                                callback(MarketData::MarketBook(market_book.clone()));
                            }
                        }

                        evt if evt.contains("board") => {
                            // parse message
                            let sym = evt.split("board_").last().unwrap().replace("_", "");
                            let board_msg: Board = from_str(&message).unwrap();
                            let mut books = self.latest_book.lock().unwrap();

                            if books.contains_key(sym.as_str()) {
                                // update latest book if snapshot already exists
                                let latest_book = books.get(sym.as_str()).unwrap();
                                let merged_book =
                                    Self::merge_board_message(latest_book.clone(), board_msg);
                                let mut rate = merged_book.to_rate();
                                rate.mid_price = (rate.best_ask + rate.best_bid) / 2.0;
                                rate.misc = "diff".to_string();

                                // update latest book
                                *books.get_mut(&sym).unwrap() = merged_book.clone();

                                // update latest rate
                                let is_new_ask =
                                    (self.latest_ask.lock().unwrap().get(&sym).unwrap()
                                        - rate.best_ask)
                                        .abs()
                                        > 0.0;
                                let is_new_bid =
                                    (self.latest_bid.lock().unwrap().get(&sym).unwrap()
                                        - rate.best_bid)
                                        .abs()
                                        > 0.0;

                                if is_new_ask | is_new_bid {
                                    self.latest_bid
                                        .lock()
                                        .unwrap()
                                        .insert(sym.clone(), rate.best_bid);

                                    self.latest_ask
                                        .lock()
                                        .unwrap()
                                        .insert(sym.clone(), rate.best_ask);

                                    // pass to callback - Rate
                                    for callback in self.callbacks.lock().unwrap().iter_mut() {
                                        callback(MarketData::Rate(rate.clone()));
                                    }
                                }

                                // pass to callback - MarketBook
                                for callback in self.callbacks.lock().unwrap().iter_mut() {
                                    callback(MarketData::MarketBook(merged_book.clone()));
                                }
                            } else {
                                // ignore if snapshot does not exist
                            }
                        }
                        evt if evt.contains("kicked") => {
                            warn!("Event: 'kicked' detected. Sleep this thread for 300s.");
                            warn!("{:#?}", event);
                            sleep(Duration::from_secs(300));
                        }
                        _ => warn!("Unknown event: {}", event),
                    }
                }
            })
            .connect()
            .unwrap();

        let wait_connect_seconds = Duration::from_secs(3);
        std::thread::sleep(wait_connect_seconds);

        for channel in channels {
            client.emit("subscribe", channel).unwrap();
        }

        self.client = Option::from(client);
    }

    pub fn disconnect(&mut self) {
        if let Some(client) = &self.client {
            client.disconnect().expect("Failed to disconnect");
            info!("Disconnected bitflyer socket.")
        }
    }

    fn merge_board_message(latest_book: MarketBook, board: Board) -> MarketBook {
        /*merge board_message to the latest_book data.
         */
        let mut merged_book = latest_book.clone();
        let now = OffsetDateTime::now_utc();
        merged_book.timestamp = now;
        merged_book.market_created_timestamp = now;
        merged_book.universal_id = uuid::Uuid::new_v4().to_string();
        merged_book.misc = "diff".to_string();

        // insert new data
        for ask in board.asks {
            merged_book.asks.insert(OrderedFloat(ask.price), ask.size);
            if merged_book.asks.get(&OrderedFloat(ask.price)) == Some(&0f64) {
                merged_book.asks.remove(&OrderedFloat(ask.price));
            }
        }
        for bid in board.bids {
            merged_book
                .bids
                .insert(OrderedFloat(bid.price), bid.size as f64);
            if merged_book.bids.get(&OrderedFloat(bid.price)) == Some(&0f64) {
                merged_book.bids.remove(&OrderedFloat(bid.price));
            }
        }

        // drop 0 size ladder
        merged_book.asks.retain(|_, &mut value| value != 0.0);
        merged_book.bids.retain(|_, &mut value| value != 0.0);

        merged_book
    }
}
