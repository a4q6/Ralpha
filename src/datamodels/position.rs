use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::constants::constants;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Position {
    #[serde(with = "time::serde::rfc3339")]
    pub timestamp: OffsetDateTime,
    pub sym: String,
    pub venue: String,
    pub amount: f64,
    pub cost: f64,
    pub model_id: String,
    pub data_center: String,
    pub process_id: String,
    pub universal_id: String,
    pub latency_tracker: String,
    pub misc: String,
}

impl Position {
    fn new(sym: &str, venue: &str, model_id: &str) -> Position {
        return Position {
            timestamp: OffsetDateTime::now_utc(),
            sym: sym.to_string(),
            venue: venue.to_string(),
            amount: 0.0,
            cost: 0.0,
            model_id: model_id.to_string(),
            data_center: constants::MACHINE_ID.to_string(),
            process_id: constants::RUNTIME_ID.to_string(),
            universal_id: uuid::Uuid::new_v4().to_string(),
            latency_tracker: "".to_string(),
            misc: "".to_string(),
        };
    }
    fn side(&self) -> i16 {
        if self.amount > 0.0 {
            return 1;
        } else if self.amount < 0.0 {
            return -1;
        } else {
            return 0;
        }
    }
}
