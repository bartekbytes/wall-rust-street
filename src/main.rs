mod models;

use models::portfolio::Portfolio;
use models::position::Position;

fn main() {
    println!("Hello, world!");

    let mut portfolio = Portfolio::new(1000.0);

    let apple = Position {
        ticker: String::from("AAPL"),
        price: 180.0,
        quantity: 2,
    };

    portfolio.add_share(apple);

    println!(
        "Total value: {} | Cash: {}",
        portfolio.total_value(),
        portfolio.cash
    );

    portfolio.buy(String::from("MSTF"), 200.0, 2);

    println!(
        "Total value {} | Cash: {}",
        portfolio.total_value(),
        portfolio.cash
    );
}
