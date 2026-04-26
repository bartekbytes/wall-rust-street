use std::collections::HashMap;

pub struct Market {
    pub prices: HashMap<String, f64>,
}

impl Market {
    pub fn new() -> Self {
        Self {
            prices: HashMap::new(),
        }
    }

    pub fn set_price(&mut self, ticker: String, price: f64) {
        self.prices.insert(ticker, price);
    }

    pub fn get_price(&self, ticker: &str) -> Option<&f64> {
        self.prices.get(ticker)
    }

    pub fn get_prices(&self) -> &HashMap<String, f64> {
        &self.prices
    }
}
