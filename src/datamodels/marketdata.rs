use ordered_float::OrderedFloat;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use time::OffsetDateTime;

use crate::constants::constants;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MarketBook {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub market_created_timestamp: OffsetDateTime,
    pub sym: String,
    pub venue: String,
    pub category: String,
    pub asks: BTreeMap<OrderedFloat<f64>, f64>,
    pub bids: BTreeMap<OrderedFloat<f64>, f64>,
    pub misc: String,
    pub universal_id: String,
    pub data_center: String,
    pub process_id: String,
}

impl MarketBook {
    pub fn to_rate(&self) -> Rate {
        let best_bid = self
            .bids
            .keys()
            .last()
            .map_or(f64::NEG_INFINITY, |&OrderedFloat(k)| k);

        let best_ask = self
            .asks
            .keys()
            .next()
            .map_or(f64::INFINITY, |&OrderedFloat(k)| k);

        return Rate {
            timestamp: self.timestamp.clone(),
            market_created_timestamp: self.market_created_timestamp.clone(),
            sym: self.sym.clone(),
            venue: self.venue.clone(),
            category: self.category.clone(),
            best_bid: best_bid,
            best_ask: best_ask,
            mid_price: (best_bid + best_ask) / 2.0,
            misc: "".to_string(),
            universal_id: uuid::Uuid::new_v4().to_string(),
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
        };
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Rate {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub market_created_timestamp: OffsetDateTime,
    pub sym: String,
    pub venue: String,
    pub category: String,
    pub best_bid: f64,
    pub best_ask: f64,
    pub mid_price: f64,
    pub misc: String,
    pub universal_id: String,
    pub data_center: String,
    pub process_id: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct MarketTrade {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub market_created_timestamp: OffsetDateTime,
    pub sym: String,
    pub venue: String,
    pub category: String,
    pub side: i16,
    pub price: f64,
    pub amount: f64,
    pub trade_id: String,
    pub order_ids: String,
    pub misc: String,
    pub universal_id: String,
    pub process_id: String,
    pub data_center: String,
}
