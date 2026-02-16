//! Basic usage example for the Bybit client.
//!
//! Run with `cargo run --example basic_usage`.

use bybit_client::rest::market::{GetOrderbookParams, GetTickersParams};
use bybit_client::{BybitClient, Category};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for debug output.
    tracing_subscriber::fmt::init();

    // Create a public-only client for public endpoints.
    let client = BybitClient::public_only()?;

    // Get ticker data for BTCUSDT perpetual.
    println!("Fetching BTCUSDT ticker...\n");

    let params = GetTickersParams::new(Category::Linear).symbol("BTCUSDT");
    let tickers = client.market().get_tickers(&params).await?;

    for ticker in &tickers.list {
        println!("Symbol:       {}", ticker.symbol);
        println!(
            "Last Price:   ${}",
            ticker.last_price.as_deref().unwrap_or("N/A")
        );
        println!(
            "24h High:     ${}",
            ticker.high_price_24h.as_deref().unwrap_or("N/A")
        );
        println!(
            "24h Low:      ${}",
            ticker.low_price_24h.as_deref().unwrap_or("N/A")
        );
        println!(
            "24h Volume:   {}",
            ticker.volume_24h.as_deref().unwrap_or("N/A")
        );
        println!(
            "24h Turnover: ${}",
            ticker.turnover_24h.as_deref().unwrap_or("N/A")
        );
    }

    // Get the orderbook.
    println!("\nFetching BTCUSDT orderbook (top 5 levels)...\n");

    let orderbook_params = GetOrderbookParams::new(Category::Linear, "BTCUSDT").limit(5);
    let orderbook = client.market().get_orderbook(&orderbook_params).await?;

    println!("Asks (sell orders):");
    for ask in orderbook.asks.iter().rev() {
        println!("  {} @ ${}", ask.size, ask.price);
    }

    println!("\nBids (buy orders):");
    for bid in &orderbook.bids {
        println!("  {} @ ${}", bid.size, bid.price);
    }

    Ok(())
}
