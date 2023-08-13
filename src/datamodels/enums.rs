use crate::datamodels::marketdata::{MarketBook, MarketTrade, Rate};
use crate::datamodels::order::Order;
use crate::datamodels::position::Position;

pub enum MarketData {
    MarketBook(MarketBook),
    Rate(Rate),
    MarketTrade(MarketTrade),
}

pub enum InternalData {
    Order(Order),
    Position(Position),
    MarketBook(MarketBook),
    Rate(Rate),
    MarketTrade(MarketTrade),
}
