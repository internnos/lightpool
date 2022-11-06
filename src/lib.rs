use std::{fmt::Debug, collections::HashMap};

#[derive(Debug, Eq, Hash, PartialEq)]
pub struct Price {
    integral: u64,
    fractional: u64,
    scalar: u64
}

impl Price {
    pub fn new(price: f64) -> Price{
        let scalar = 100000;
        let integral = price as u64;
        let fractional = ((price % 1.0) * scalar as f64) as u64;
        Price {
            integral,
            fractional,
            scalar
        }
    }
    
}

#[derive(Debug)]
pub enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
pub struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}


impl Order {
    pub fn new(size: f64, bid_or_ask: BidOrAsk) -> Order {
        Order {
            size, 
            bid_or_ask
        }
    }
}

#[derive(Debug)]
pub struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    pub fn new(price: Price) -> Limit {
        let orders =  Vec::new();
        Limit {
            price,
            orders
        }
    }
    pub fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }
}



#[derive(Debug)]
pub struct OrderBook{
    asks: HashMap<Price, Limit>,
    bids: HashMap<Price, Limit>
}

impl OrderBook {
    pub fn new() -> OrderBook {
        OrderBook { asks: HashMap::new(), bids: HashMap::new() }
    }
    pub fn add_order(&mut self, price: f64, order: Order) {
        match order.bid_or_ask {
            BidOrAsk::Ask => {
                let limit = self.asks
                .entry(Price::new(price))
                .or_insert(Limit::new(Price::new(price)));
                limit.add_order(order);
            },
            BidOrAsk::Bid => {
                let limit = self.bids
                .entry(Price::new(price))
                .or_insert(Limit::new(Price::new(price)));
                limit.add_order(order);
            }
        }
    }

}