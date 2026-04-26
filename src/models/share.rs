pub struct Share {
    pub ticker: String,
    pub price: f64,
    pub quantity: u32,
}

impl Share {
    pub fn value(&self) -> f64 {
        self.price * self.quantity as f64
    }
}
