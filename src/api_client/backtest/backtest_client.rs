use std::collections::HashMap;
use std::time::Duration;
use time::OffsetDateTime;

use crate::api_client::execution_client::ExecutionClient;
use crate::datamodels::enums::MarketData;
use crate::datamodels::order::{Order, OrderStatus, OrderType};
use crate::datamodels::position::Position;

pub struct BacktestClient<'a> {
    // このClientにbacktestを全て任せる?
    // ... backtest clientからstrategy callbackを呼べるようにするひつようある.
    // ... clientとstrategyは相互参照する必要がある.
    // ... どのように実装するか...?
    // Market timestamp driven な nextに加えて、Execution driveなnextがある
    // Runnerを用意して、相互にnextを回すか...?
    // Executionを用意した方がよいか?
    pub venue: &'a str,
    pending_market_orders: HashMap<String, Order>,
    pending_limit_orders: HashMap<String, Order>,
    filled_market_orders: HashMap<String, Order>,
    filled_limit_orders: HashMap<String, Order>,
    best_bid: f64,
    best_ask: f64,
    market_order_submit_latency: Duration,
    market_order_receive_latency: Duration,
    limit_order_submit_latency: Duration,
    limit_order_receive_latency: Duration,
}

impl<'a> BacktestClient<'a> {
    pub fn new(venue: &'a str) -> Self {
        Self {
            venue: venue,
            pending_limit_orders: HashMap::new(),
            pending_market_orders: HashMap::new(),
            filled_market_orders: HashMap::new(),
            filled_limit_orders: HashMap::new(),
            best_bid: 0.0,
            best_ask: f64::INFINITY,
            market_order_submit_latency: Duration::from_millis(100),
            market_order_receive_latency: Duration::from_millis(1000),
            limit_order_submit_latency: Duration::from_millis(100),
            limit_order_receive_latency: Duration::from_millis(1000),
        }
    }

    /// Entry point of the backtest client. will be invoked by the runner.
    ///
    /// # Arguments
    /// * `market_data` -
    ///
    /// # Returns
    ///
    /// # Examples
    pub fn next(&mut self, market_data: MarketData) {
        self.process_market_orders(&market_data);
        self.process_limit_orders(&market_data);
        match market_data {
            MarketData::Rate(rate) => {
                self.best_bid = rate.best_bid;
                self.best_ask = rate.best_ask;
            }
            _ => {}
        }
    }

    /// [TODO]
    /// - add order created -> submit -> received latency
    /// - add order execution -> received latency
    fn process_limit_orders(&mut self, market_data: &MarketData) -> Vec<Order> {
        let mut filled_order_ids: Vec<String> = Vec::new();
        match market_data {
            MarketData::MarketTrade(market_trade) => {
                // check if pending limit orders are filled or not
                for (order_id, order) in self.pending_limit_orders.iter_mut() {
                    if order.sym == market_trade.sym && order.venue == market_trade.venue {
                        if order.side > 0 && market_trade.side < 0 {
                            if market_trade.price <= order.price
                                && order.market_created_timestamp
                                    < market_trade.market_created_timestamp
                            {
                                filled_order_ids.push(order_id.clone());
                            }
                        } else if order.side < 0 && market_trade.side > 0 {
                            if order.price <= market_trade.price
                                && order.market_created_timestamp
                                    < market_trade.market_created_timestamp
                            {
                                filled_order_ids.push(order_id.clone());
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        filled_order_ids
            .iter()
            .map(|order_id| self.pending_limit_orders.remove(order_id).unwrap())
            .collect()
    }

    /// [TODO]
    /// - add order created -> submit latency
    /// - add order execution -> received latency
    fn process_market_orders(&mut self, market_data: &MarketData) -> Vec<Order> {
        let mut filled_order_ids = Vec::new();
        match market_data {
            MarketData::Rate(rate) => {
                for (order_id, order) in self.pending_market_orders.iter_mut() {
                    if order.sym == rate.sym && order.venue == rate.venue {
                        if order.side > 0 {
                            order.price = rate.best_ask;
                            filled_order_ids.push(order_id.clone());
                        } else if order.side < 0 {
                            order.price = rate.best_bid;
                            order.executed_amount = order.amount;
                            order.received_timestamp = rate.timestamp;
                            filled_order_ids.push(order_id.clone());
                        }
                    }
                }
            }
            _ => {}
        }
        filled_order_ids
            .iter()
            .map(|order_id| self.pending_market_orders.remove(order_id).unwrap())
            .collect()
    }
}

impl<'a> ExecutionClient for BacktestClient<'a> {
    // TODO
    fn submit_order(
        &mut self,
        timestamp: OffsetDateTime,
        sym: &str,
        side: i16,
        price: f64,
        amount: f64,
        order_type: OrderType,
        model_id: &str,
    ) -> Order {
        let new_order = Order::new(
            timestamp, sym, side, price, amount, order_type, self.venue, model_id,
        );
        self.pending_limit_orders
            .insert(new_order.order_id.clone(), new_order.clone());
        new_order
    }

    // fn amend_order(&mut self, order_id: &String, price: f64, amount: f64) -> Order {
    //     let order = self.pending_limit_orders.get_mut(order_id);
    //     match order {
    //         Some(order) => {
    //             order.price = price;
    //             order.amount = amount;
    //             order
    //         }
    //         None => panic!("order not found: {}", order_id),
    //     }

    fn cancel_order(&mut self, timestamp: OffsetDateTime, order_id: &String) -> Order {
        let order = self.pending_limit_orders.remove(order_id);
        match order {
            Some(order) => {
                let mut o = order;
                o.order_status = OrderStatus::Canceled;
                o.timestamp = timestamp;
                o.market_created_timestamp = timestamp + self.market_order_submit_latency;
                o.received_timestamp =
                    o.market_created_timestamp + self.market_order_receive_latency;
                o
            }
            None => panic!("order not found: {}", order_id),
        }
    }

    fn get_order_status(&mut self, order_id: &String) -> Option<&Order> {
        let order = self.pending_limit_orders.get(order_id);
        return order;
    }

    fn get_positions() -> Position {
        Position::new("BTCUSD", "backtest", "backtest")
    }
}
