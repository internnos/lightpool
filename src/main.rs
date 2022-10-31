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
enum BidOrAsk {
    Bid,
    Ask
}

#[derive(Debug)]
struct Order {
    size: f64,
    bid_or_ask: BidOrAsk
}


impl Order {
    fn new(size: f64, bid_or_ask: BidOrAsk) -> Order {
        Order {
            size, 
            bid_or_ask
        }
    }
}

#[derive(Debug)]
struct Limit {
    price: Price,
    orders: Vec<Order>
}

impl Limit {
    fn new(price: f64) -> Limit {
        let price = Price::new(price);
        let orders =  Vec::new();
        Limit {
            price,
            orders
        }
    }
    fn add_order(&mut self, order: Order) {
        self.orders.push(order)
    }
}



fn main() {
    // let price = Price::new(65.300);
    // println!("{:?}", price);

    let mut limit = Limit::new(65.300);

    let buy_order = Order::new(12.2, BidOrAsk::Bid);
    // println!("{:?}", &limit);
    limit.add_order(buy_order);

    let mut sell_order = Order::new(13.0, BidOrAsk::Ask);
    println!("{:?}", &limit);

    limit.add_order(sell_order);
    println!("{:?}", &limit);
    
}
