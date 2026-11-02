#[derive(Debug, PartialEq)]
pub enum OrderType {
    Market,
    Limit(u64),    //no f64 here to avoid float issue
}

#[derive(Debug)]
// Fill Confirmation
pub struct Fill {
    client_id: u32,
    fill_price: u64,
    quantity: i64,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum OrderStatus {
    PendingNew,
    Acked,
    PartiallyFilled,
    Filled,
    Rejected,
    Cancelled,
}

#[derive(Debug, PartialEq)]
pub enum OrderError {
    //Order Error
    InvalidQty,
    InvalidLimitPrice,
    
    //Order State Error
    InvalidFillPrice,
    InvalidFillQty,
    OverFill,
    OrderClosed(OrderStatus),
    NotPending(OrderStatus),
    ClientIdMismatch{expected:u32, received:u32},   
}

#[derive(Debug, PartialEq)]
pub struct Order {
    instrument: String,
    client_id: u32,
    exchange_id: Option<u32>,  //Orders don't acquire this until exchange acks it
    order_type: OrderType,
    quantity: i64,   // sign of qty to represent bull/sell,
    order_status: OrderStatus,
    cum_qty: i64,
}

impl Order {
    pub fn new(instrument: String, client_id: u32, order_type: OrderType, quantity: i64) -> Result<Order, OrderError> {
        use OrderStatus::*;
        use OrderError::*;

        if quantity == 0 {return Err(InvalidQty)}
        if let OrderType::Limit(price) = order_type {if price == 0 {return Err(InvalidLimitPrice)}}
        
        Ok(Order{
                instrument,
                client_id,
                exchange_id: None,
                order_type,
                quantity,
                order_status: PendingNew,
                cum_qty: 0,
            })
    }

    pub fn take_fill(&mut self, fill: &Fill) -> Result<(), OrderError> {
        use OrderStatus::*;
        use OrderError::*;

        // check order: Client id > closed order > zero price > wrong sign or zero qty > overfill
        if fill.client_id != self.client_id {return Err(ClientIdMismatch{expected: self.client_id,received:fill.client_id})};
        match self.order_status { 
            Filled | Rejected | Cancelled => return Err(OrderClosed(self.order_status)),
            _ => (), 
        };
        if fill.fill_price == 0 {return Err(InvalidFillPrice)}
        if fill.quantity.signum() != self.quantity.signum() {return Err(InvalidFillQty)};
        if self.cum_qty.abs()+fill.quantity.abs() > self.quantity.abs() {return Err(OverFill)};
        
        self.cum_qty += fill.quantity;
        if self.cum_qty.abs() < self.quantity.abs() {self.order_status = PartiallyFilled}
        else {self.order_status = Filled};
        Ok(())
    }

    pub fn cancel(&mut self) -> Result<(), OrderError> {
        use OrderStatus::*;
        use OrderError::*;

        match self.order_status {
            Filled | Rejected | Cancelled => return Err(OrderClosed(self.order_status)),
            PendingNew | Acked | PartiallyFilled => self.order_status = Cancelled,
        };
        Ok(())
    }

    pub fn reject(&mut self) -> Result<(), OrderError> {
        use OrderStatus::*;
        use OrderError::*;

        match self.order_status {
            Filled | Rejected | Cancelled => return Err(OrderClosed(self.order_status)),
            Acked | PartiallyFilled => return Err(NotPending(self.order_status)),
            PendingNew => self.order_status = Rejected,
        };
        Ok(())
    }

    pub fn ack(&mut self, id: u32) -> Result<(), OrderError> {
        use OrderStatus::*;
        use OrderError::*;

        match self.order_status {
            Filled | Rejected | Cancelled => return Err(OrderClosed(self.order_status)),
            Acked | PartiallyFilled => return Err(NotPending(self.order_status)),
            PendingNew => {self.order_status = Acked; self.exchange_id = Some(id)},
        };
        Ok(())
    }

}


#[cfg(test)]
mod tests {
    use super::*;
    use OrderType::*;

    // Tests for Unit 1
    fn order(t: OrderType, qty: i64) -> Result<Order, OrderError> {Order::new(String::from("CL"), 123, t, qty)}
    // Tests for zero qty or limit price
    #[test]
    fn test_qty() {
        let result = order(Market, 0);
        assert_eq!(result, Err(OrderError::InvalidQty));
    }

    #[test]
    fn test_price() {
        let result =  order(Limit(0), 1);
        assert_eq!(result, Err(OrderError::InvalidLimitPrice));
    }

    #[test]
    fn test_two_zeros() {
        let result = order(Limit(0), 0);
        assert_eq!(result, Err(OrderError::InvalidQty));
    }

    #[test]
    fn test_buy_market() {
        let result = order(Market, 5); 
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, 5);
        assert_eq!(order.order_type, OrderType::Market);
    }
    
    #[test]
    fn test_sell_market() {
        let result = order(Market, -5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, -5);
        assert_eq!(order.order_type, OrderType::Market);
    }

    #[test]
    fn test_buy_limit() {
        let result = order(Limit(100), 5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, 5);
        assert_eq!(order.order_type, OrderType::Limit(100));
    }

    #[test]
    fn test_sell_limit() {
        let result = order(Limit(100), -5);
        let order = result.unwrap();
        assert_eq!(order.exchange_id, None);
        assert_eq!(order.quantity, -5);
        assert_eq!(order.order_type, OrderType::Limit(100));
    }

    // Tests for Unit 2
    use OrderStatus::*;
    use OrderError::*;

    fn fill(id: u32, price: u64, qty: i64) -> Fill {Fill{client_id: id, fill_price: price, quantity: qty}}

    #[test]
    fn test_overfill_rejected(){
        let mut o = order(Market, -5).unwrap();
        o.take_fill(&fill(123, 100, -3)).unwrap();
        let result = o.take_fill(&fill(123, 100, -3));
        assert_eq!(result, Err(OverFill));
        assert_eq!(o.cum_qty, -3);
        assert_eq!(o.order_status, PartiallyFilled);
    }

    #[test]
    fn test_filled(){
        let mut o = order(Market, -5).unwrap();
        o.take_fill(&fill(123, 100, -3)).unwrap();
        assert_eq!(o.order_status, PartiallyFilled);
        o.take_fill(&fill(123, 100, -2)).unwrap();
        assert_eq!(o.cum_qty, -5);
        assert_eq!(o.order_status, Filled);
    }

    #[test]
    fn test_cancel(){
        let mut o = order(Market, -5).unwrap();
        o.take_fill(&fill(123, 100, -3)).unwrap();
        assert_eq!(o.order_status, PartiallyFilled);
        o.cancel().unwrap();
        assert_eq!(o.cum_qty, -3);
        assert_eq!(o.order_status, Cancelled);

        let result = o.take_fill(&fill(123, 100, -1));
        assert_eq!(result, Err(OrderClosed(Cancelled)));
        assert_eq!(o.cum_qty, -3);
    }

    #[test]
    fn test_ack(){
        let mut o = order(Market, 5).unwrap();
        assert_eq!(o.order_status, PendingNew);
        assert_eq!(o.exchange_id, None);
        o.ack(987).unwrap();
        assert_eq!(o.order_status, Acked);
        assert_eq!(o.exchange_id, Some(987));
    }

    #[test]
    fn test_reject(){
        let mut o = order(Market, 5).unwrap();
        o.reject().unwrap();
        assert_eq!(o.order_status, Rejected);

        // a rejected order is closed, no fills after it.
        let result = o.take_fill(&fill(123, 100, 2));
        assert_eq!(result, Err(OrderClosed(Rejected)));
        assert_eq!(o.cum_qty, 0);
    }

    #[test]
    fn test_ack_reject_not_pending(){
        let mut o = order(Market, 5).unwrap();
        o.ack(987).unwrap();
        assert_eq!(o.ack(555), Err(NotPending(Acked)));
        assert_eq!(o.exchange_id, Some(987));   // a duplicate ack must not overwrite the id
        assert_eq!(o.reject(), Err(NotPending(Acked)));

        o.take_fill(&fill(123, 100, 2)).unwrap();
        assert_eq!(o.reject(), Err(NotPending(PartiallyFilled)));
        assert_eq!(o.order_status, PartiallyFilled);
    }

    #[test]
    fn test_ack_reject_closed(){
        let mut o = order(Market, 5).unwrap();
        o.cancel().unwrap();
        assert_eq!(o.ack(987), Err(OrderClosed(Cancelled)));
        assert_eq!(o.reject(), Err(OrderClosed(Cancelled)));
        assert_eq!(o.order_status, Cancelled);
        assert_eq!(o.exchange_id, None);
    }
}