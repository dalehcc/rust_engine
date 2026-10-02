#[derive(Debug)]
enum Side {
    Buy,
    Sell,
}

#[derive(Debug)]
enum OrderType {
    Market,
    Limit(f64),
}


#[derive(Debug)]
struct Order {
    instrument: String,
    client_id: u32,
    exchange_id: Option<u32>,
    side: Side,
    order_type: OrderType,
    quantity: u32,
}

fn main() {
    
    let Order1 = Order {
        instrument: String::from("XAL"),
        client_id: 123,
        exchange_id: std::option::Option::Some(123),
        side: Side::Buy,
        order_type: OrderType::Market,
        quantity: 4,
    };
    
    
    println!("My first order is {:?}", Order1)
}