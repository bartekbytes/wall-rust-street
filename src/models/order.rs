pub enum OrderSide {
    Buy,
    Sell,
}

pub struct Order {
    ticker: String,
    quantity: u32,
    price: f64,
    side: OrderSide,
}
