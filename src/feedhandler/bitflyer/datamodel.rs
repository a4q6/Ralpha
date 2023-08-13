use std::collections::BTreeMap;

use crate::constants::constants;
use crate::datamodels::marketdata::{MarketBook, MarketTrade, Rate};

use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;
use uuid::Uuid;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct PriceSize {
    pub price: f64,
    pub size: f64,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Board {
    pub mid_price: f64,
    pub asks: Vec<PriceSize>,
    pub bids: Vec<PriceSize>,
}

impl Board {
    pub fn new() -> Self {
        Self {
            mid_price: 0.0,
            asks: vec![],
            bids: vec![],
        }
    }

    pub fn merge(&mut self, other: Board) {
        for ask in other.asks {
            self.asks.push(ask);
        }
    }

    pub fn to_market_book(self, sym: String) -> MarketBook {
        let mut asks = BTreeMap::new();
        let mut bids = BTreeMap::new();
        for ask in self.asks {
            asks.insert(OrderedFloat(ask.price), ask.size);
        }
        for bid in self.bids {
            bids.insert(OrderedFloat(bid.price), bid.size);
        }
        let t = OffsetDateTime::now_utc();
        MarketBook {
            timestamp: t,
            market_created_timestamp: t,
            sym: sym,
            venue: "bitflyer".to_string(),
            category: "lightning".to_string(),
            asks: asks,
            bids: bids,
            universal_id: Uuid::new_v4().to_string(),
            misc: "".to_string(),
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
        }
    }

    pub fn to_rate(self, sym: String) -> Rate {
        Rate {
            timestamp: OffsetDateTime::now_utc(),
            market_created_timestamp: OffsetDateTime::now_utc(),
            sym: sym,
            venue: "bitflyer".to_string(),
            category: "lightning".to_string(),
            best_bid: self
                .bids
                .iter()
                .max_by(|a, b| {
                    a.price
                        .partial_cmp(&b.price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|p| p.price)
                .unwrap_or(f64::NEG_INFINITY),
            best_ask: self
                .asks
                .iter()
                .min_by(|a, b| {
                    a.price
                        .partial_cmp(&b.price)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|p| p.price)
                .unwrap_or(f64::INFINITY),
            mid_price: self.mid_price,
            universal_id: Uuid::new_v4().to_string(),
            misc: "".to_string(),
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Execution {
    pub id: u32,
    pub side: String,
    pub price: f64,
    pub size: f64,
    #[serde(with = "time::serde::rfc3339")]
    pub exec_date: OffsetDateTime,
    pub buy_child_order_acceptance_id: String,
    pub sell_child_order_acceptance_id: String,
}

impl Execution {
    pub fn to_market_trade(&self, sym: String) -> MarketTrade {
        MarketTrade {
            timestamp: OffsetDateTime::now_utc(),
            market_created_timestamp: self.exec_date,
            sym: sym,
            venue: "bitflyer".to_string(),
            category: "lightning".to_string(),
            side: if self.side == "BUY" { 1 } else { -1 },
            price: self.price,
            amount: self.size,
            trade_id: self.id.to_string(),
            order_ids: [
                self.buy_child_order_acceptance_id.clone(),
                self.sell_child_order_acceptance_id.clone(),
            ]
            .join(";"),
            misc: "".to_string(),
            universal_id: Uuid::new_v4().to_string(),
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
        }
    }
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

impl Ticker {
    pub fn new() -> Self {
        Self {
            product_code: "".to_string(),
            state: "".to_string(),
            timestamp: OffsetDateTime::now_utc(),
            tick_id: 0,
            best_bid: 0.0,
            best_ask: 0.0,
            best_bid_size: 0.0,
            best_ask_size: 0.0,
            total_bid_depth: 0.0,
            total_ask_depth: 0.0,
            market_bid_size: 0.0,
            market_ask_size: 0.0,
            ltp: 0.0,
            volume: 0.0,
            volume_by_product: 0.0,
        }
    }
}
