#[derive(Debug)]
enum OrderType {
    Market,
    Limit(u64),    //no f64 here to avoid float issue
}


#[derive(Debug)]
struct Order {
    instrument: String,
    client_id: u32,
    exchange_id: Option<u32>,  //Order may not have this until exchange confirms
    order_type: OrderType,
    quantity: i64,   // sign of qty to represent bull/sell
}

#[derive(Debug)]
enum OrderError {
    InvalidQty,
    InvalidLimitPrice,
}

impl Order {
    fn new(instrument: String,
           client_id: u32,
           order_type: OrderType,
           quantity: i64) -> Result<Order, OrderError> {
            
            if quantity == 0 {return Err(OrderError::InvalidQty)}
            
            if let OrderType::Limit(price) = order_type {
                if price == 0 {return Err(OrderError::InvalidLimitPrice)}
            }

            Ok(Order{
                    instrument,
                    client_id,
                    exchange_id: None,
                    order_type,
                    quantity,
                })
    }
}


#[derive(Debug)]
struct Fill {
    client_id: u32,
    fill_price: u64,
    quantity: i64,
}

fn main() {
    
    let test_order = Order::new(
        String::from("XAL"),
        123,
        OrderType::Market,
        4);
    
    
    
    println!("My first order is {:?}", test_order);
    
}