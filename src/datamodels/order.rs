use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::constants::constants;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Order {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub market_created_timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub received_timestamp: OffsetDateTime,
    pub sym: String,
    pub side: i16,
    pub price: f64,
    pub amount: f64,
    pub executed_amount: f64,
    pub order_type: OrderType,
    pub order_status: String,
    pub venue: String,
    pub order_id: String,
    pub model_id: String,
    pub data_center: String,
    pub process_id: String,
    pub universal_id: String,
    pub latency_tracker: String,
    pub misc: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum OrderType {
    Limit(String),
    Market(String),
}

impl Order {
    fn new(
        sym: &str,
        price: f64,
        amount: f64,
        order_type: OrderType,
        venue: &str,
        model_id: &str,
    ) -> Order {
        return Order {
            timestamp: OffsetDateTime::now_utc(),
            market_created_timestamp: OffsetDateTime::now_utc(),
            received_timestamp: OffsetDateTime::now_utc(),
            sym: sym.to_string(),
            side: 0,
            price: price,
            amount: amount,
            executed_amount: 0.0,
            order_type: order_type,
            order_status: "new".to_string(),
            venue: venue.to_string(),
            order_id: uuid::Uuid::new_v4().to_string(),
            model_id: model_id.to_string(),
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
            universal_id: uuid::Uuid::new_v4().to_string(),
            latency_tracker: "".to_string(),
            misc: "".to_string(),
        };
    }
}
