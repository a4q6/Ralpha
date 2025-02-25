use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::constants::constants;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Execution {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub market_created_timestamp: OffsetDateTime,
    #[serde(with = "time::serde::rfc3339")]
    pub received_timestamp: OffsetDateTime,
    pub venue: String,
    pub sym: String,
    pub side: i16,
    pub price: f64,
    pub amount: f64,
    pub execution_id: String,
    pub source_order_id: String,
    pub data_center: String,
    pub process_id: String,
    pub universal_id: String,
    pub latency_tracker: String,
    pub misc: String,
}

impl Execution {
    pub fn new(
        sym: &str,
        side: i16,
        price: f64,
        amount: f64,
        venue: &str,
        source_order_id: String,
        execution_id: String,
    ) -> Execution {
        return Execution {
            timestamp: OffsetDateTime::now_utc(),
            market_created_timestamp: OffsetDateTime::now_utc(),
            received_timestamp: OffsetDateTime::now_utc(),
            sym: sym.to_string(),
            side: side,
            price: price,
            amount: amount,
            venue: venue.to_string(),
            source_order_id: source_order_id,
            execution_id: execution_id,
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
            universal_id: uuid::Uuid::new_v4().to_string(),
            latency_tracker: "".to_string(),
            misc: "".to_string(),
        };
    }
}
