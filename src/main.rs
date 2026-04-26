mod models;

use models::portfolio::Portfolio;
use models::position::Position;

mod services;

use services::market::Market;

fn main() {
    println!("Hello, world!");

    // Initialize Market Service
    let mut market = Market::new();
    market.set_price("AAPL".to_string(), 220.0);
    market.set_price("MSFT".to_string(), 400.0);

    println!("Market Service is up and running...");
    let market_prices = market.get_prices();
    for (ticker, price) in market_prices {
        println!("{} -> {}", ticker, price)
    }

    /*
    println!("HAHA: {}", market.get_price("AAPL"));
    println!("BOOO: {}", market.get_price("LOLL"));
    Above won't work as we expect Option<&f64> to be printed not f64, we need to handle the Option type properly.
    Quick fix:
    println!("{:?}", market.get_price("AAPL"));
    println!("{:?}", market.get_price("LOLL"));
    will print
    Some(220.0)
    None
    But to get proper one, matching needs to be implemented:

    */

    match market.get_price("AAPL") {
        Some(price) => println!("Price: {}", price),
        None => println!("Ticker not found!"),
    }

    match market.get_price("LOLL") {
        Some(price) => println!("Price: {}", price),
        None => println!("Ticker not found!"),
    }

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
