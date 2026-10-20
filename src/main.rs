#[derive(Debug, PartialEq)]
enum OrderType {
    Market,
    Limit(u64),    //no f64 here to avoid float issue
}

#[derive(Debug)]
struct Fill {
    client_id: u32,
    fill_price: u64,
    quantity: i64,
}

#[derive(Debug, PartialEq)]
enum OrderError {
    InvalidQty,
    InvalidLimitPrice,
}

#[derive(Debug, PartialEq)]
struct Order {
    instrument: String,
    client_id: u32,
    exchange_id: Option<u32>,  //Order may not have this until exchange confirms
    order_type: OrderType,
    quantity: i64,   // sign of qty to represent bull/sell
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

fn main() {
    let test_order = Order::new(
        String::from("CL"),
        123,
        OrderType::Market,
        4);
    
    println!("My first order is {:?}", test_order);
    
}


#[cfg(test)]
mod tests {
    use super::*;


    // Tests for zero qty or limit price
    #[test]
    fn test_qty() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Market,
            0);
        assert_eq!(result, Err(OrderError::InvalidQty));
    }

    #[test]
    fn test_price() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Limit(0),
            1);
        assert_eq!(result, Err(OrderError::InvalidLimitPrice));
    }

    #[test]
    fn test_two_zeros() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Limit(0),
            0);
        assert_eq!(result, Err(OrderError::InvalidQty));
    }

    // Tests for Buy/Sell x Market/Limit
    #[test]
    fn test_buy_market() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Market,
            5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, 5);
        assert_eq!(order.order_type, OrderType::Market);
    }
    
    #[test]
    fn test_sell_market() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Market,
            -5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, -5);
        assert_eq!(order.order_type, OrderType::Market);
    }

    #[test]
    fn test_buy_limit() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Limit(100),
            5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, 5);
        assert_eq!(order.order_type, OrderType::Limit(100));
    }

    #[test]
    fn test_sell_limit() {
        let result = Order::new(
            String::from("CL"),
            123,
            OrderType::Limit(100),
            -5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, -5);
        assert_eq!(order.order_type, OrderType::Limit(100));
    }

}