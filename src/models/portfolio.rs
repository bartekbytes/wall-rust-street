use crate::models::share::Share;

pub struct Portfolio {
    pub shares: Vec<Share>,
    pub cash: f64,
}

impl Portfolio {
    pub fn new(cash: f64) -> Self {
        Self {
            shares: Vec::new(),
            cash,
        }
    }

    pub fn add_share(&mut self, share: Share) {
        self.shares.push(share);
    }

    pub fn total_value(&self) -> f64 {
        let shares_value: f64 = self
            .shares
            .iter()
            .map(|s| s.price * s.quantity as f64)
            .sum();
        //let shares_value: f64 = self.shares.iter().map(|s| s.value());

        shares_value + self.cash
    }

    pub fn buy(&mut self, ticker: String, price: f64, quantity: u32) {
        let deducted_cash: f64 = price * quantity as f64;
        let added_share: Share = Share {
            ticker: ticker,
            price: price,
            quantity: quantity,
        };

        self.cash -= deducted_cash;
        self.shares.push(added_share)
    }
}
