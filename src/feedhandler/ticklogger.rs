use std::collections::BTreeMap;
use std::path::Path;
use std::time::Duration;

use crate::datamodels::enums::MarketData;
use crate::feedhandler::textwriter::TextWriter;

pub struct TickLogger {
    book_logger: TextWriter,
    trad_logger: TextWriter,
    rate_logger: TextWriter,
    throttling_sec: BTreeMap<String, f64>,
}

impl TickLogger {
    pub fn new(venue: &str) -> TickLogger {
        TickLogger {
            book_logger: TextWriter::new(Path::new("MarketBook").join(venue).to_str().unwrap()),
            trad_logger: TextWriter::new(Path::new("MarketTrade").join(venue).to_str().unwrap()),
            rate_logger: TextWriter::new(Path::new("Rate").join(venue).to_str().unwrap()),
            throttling_sec: BTreeMap::new(),
        }
    }
    pub fn callback(&mut self, data: MarketData) {
        // unixtime
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs_f64();

        // write
        match data {
            MarketData::MarketTrade(data) => {
                self.trad_logger
                    .write(serde_json::to_string(&data).unwrap().as_str());
            }
            MarketData::MarketBook(data) => {
                if (now - self.throttling_sec.get(data.sym.as_str()).unwrap_or(&0.0))
                    > Duration::from_millis(100).as_secs_f64()
                {
                    self.book_logger
                        .write(serde_json::to_string(&data).unwrap().as_str());
                    self.throttling_sec.insert(data.sym, now);
                }
            }
            MarketData::Rate(data) => {
                self.rate_logger
                    .write(serde_json::to_string(&data).unwrap().as_str());
            }
        }
    }
}
