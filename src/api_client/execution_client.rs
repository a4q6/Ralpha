use time::OffsetDateTime;

use crate::datamodels::order::{Order, OrderType};
use crate::datamodels::position::Position;

pub trait ExecutionClient {
    fn submit_order(
        &mut self,
        timestamp: OffsetDateTime,
        sym: &str,
        side: i16,
        price: f64,
        amount: f64,
        order_type: OrderType,
        model_id: &str,
    ) -> Order;
    fn cancel_order(&mut self, timestamp: OffsetDateTime, order_id: &String) -> Order;
    // fn amend_order(&mut self, order_id: &String, price: f64, amount: f64) -> Order;
    fn get_order_status(&mut self, order_id: &String) -> Option<&Order>;
    fn get_positions() -> Position;
}
