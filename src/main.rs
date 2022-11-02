use lightpool::{Order, BidOrAsk, OrderBook};

fn main() {

    let mut orderbook = OrderBook::new();
    orderbook.add_order(65.300, Order::new(12.2, BidOrAsk::Bid));
    orderbook.add_order(65.300, Order::new(13.0, BidOrAsk::Bid));
    orderbook.add_order(62.300, Order::new(13.2, BidOrAsk::Ask));

    println!("{:?}", orderbook);
}
