use std::fmt::Debug;

#[derive(Debug)]
struct Price {
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
    pub fn new(price: f64) -> Limit {
        let price = Price::new(price);
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

