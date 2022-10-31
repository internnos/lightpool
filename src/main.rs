use lightpool::{Limit, Order, BidOrAsk};

fn main() {
    // let price = Price::new(65.300);
    // println!("{:?}", price);

    let mut limit = Limit::new(65.300);

    let buy_order = Order::new(12.2, BidOrAsk::Bid);
    // println!("{:?}", &limit);
    limit.add_order(buy_order);

    let sell_order = Order::new(13.0, BidOrAsk::Ask);
    println!("{:?}", &limit);

    limit.add_order(sell_order);
    println!("{:?}", &limit);
    
}
