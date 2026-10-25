use rust_engine::order::Order;
use rust_engine::order::OrderType;


fn main() {
    let test_order = Order::new(
        String::from("CL"),
        123,
        OrderType::Market,
        4);
    
    println!("My first order is {:?}", test_order);
    
}