use crate::models::position::Position;

pub struct Portfolio {
    pub positions: Vec<Position>,
    pub cash: f64,
}

impl Portfolio {
    pub fn new(cash: f64) -> Self {
        Self {
            positions: Vec::new(),
            cash,
        }
    }

    pub fn add_share(&mut self, position: Position) {
        self.positions.push(position);
    }

    pub fn total_value(&self) -> f64 {
        let positions_value: f64 = self
            .positions
            .iter()
            .map(|p| p.price * p.quantity as f64)
            .sum();

        positions_value + self.cash
    }

    pub fn buy(&mut self, ticker: String, price: f64, quantity: u32) {
        let deducted_cash: f64 = price * quantity as f64;
        let added_position: Position = Position {
            ticker: ticker,
            price: price,
            quantity: quantity,
        };

        self.cash -= deducted_cash;
        self.positions.push(added_position)
    }
}
