//! WebSocket client implementation.
//!
//! This module provides WebSocket support for:
//! - Public streams (orderbook, trades, tickers, klines, liquidations).
//! - Private streams (positions, orders, executions, wallet updates).
//! - WebSocket Trade API for order management.
//! - Local orderbook management with delta updates.
//!
//! # Public Streams Example
//!
//! ```no_run
//! use bybit_client::ws::{WsClient, WsChannel, WsMessage};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect to linear perpetual public stream.
//!     let (client, mut receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;
//!
//!     // Subscribe to orderbook and trades.
//!     client.subscribe(&["orderbook.50.BTCUSDT", "publicTrade.BTCUSDT"]).await?;
//!
//!     // Process incoming messages.
//!     while let Some(msg) = receiver.recv().await {
//!         match msg {
//!             WsMessage::Orderbook(ob) => {
//!                 println!("Orderbook update for {}: {} bids, {} asks",
//!                     ob.data.symbol, ob.data.bids.len(), ob.data.asks.len());
//!             }
//!             WsMessage::Trade(trades) => {
//!                 for trade in &trades.data {
//!                     println!("Trade: {} {} @ {}",
//!                         trade.symbol, trade.size, trade.price);
//!                 }
//!             }
//!             _ => {}
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Local Orderbook Example
//!
//! ```no_run
//! use bybit_client::ws::{WsClient, WsChannel, WsMessage, LocalOrderbook};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (client, mut receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;
//!     client.subscribe(&["orderbook.50.BTCUSDT"]).await?;
//!
//!     let mut orderbook = LocalOrderbook::new("BTCUSDT");
//!
//!     while let Some(msg) = receiver.recv().await {
//!         if let WsMessage::Orderbook(update) = msg {
//!             orderbook.apply_update(&update)?;
//!
//!             if let (Some(bid), Some(ask)) = (orderbook.best_bid(), orderbook.best_ask()) {
//!                 println!("Best bid: {} @ {}, Best ask: {} @ {}",
//!                     bid.size, bid.price, ask.size, ask.price);
//!                 println!("Spread: {:.2}", orderbook.spread().unwrap_or(0.0));
//!             }
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # Private Streams Example
//!
//! ```no_run
//! use bybit_client::ws::{WsClient, WsChannel};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Connect with authentication.
//!     let (client, mut receiver) = WsClient::connect_private(
//!         "your_api_key",
//!         "your_api_secret",
//!     ).await?;
//!
//!     // Subscribe to private topics.
//!     client.subscribe(&["position", "order", "execution"]).await?;
//!
//!     while let Some(msg) = receiver.recv().await {
//!         println!("Private message: {:?}", msg);
//!     }
//!
//!     Ok(())
//! }
//! ```
//!
//! # WebSocket Topics
//!
//! ## Public Topics
//! - `orderbook.{depth}.{symbol}` - Orderbook (depth: 1, 50, 200, 500).
//! - `publicTrade.{symbol}` - Public trades.
//! - `tickers.{symbol}` - Ticker updates.
//! - `kline.{interval}.{symbol}` - Kline or candlestick data.
//! - `allLiquidation.{symbol}` - All liquidation events.
//! - `liquidation.{symbol}` - Liquidation events (deprecated, prefer `allLiquidation`).
//! - `insurance.{coin}` - Insurance pool updates.
//! - `priceLimit.{symbol}` - Price limit updates.
//!
//! ## Private Topics
//! - `position` - Position updates.
//! - `execution` - Execution updates.
//! - `execution.fast` - Low-latency execution updates.
//! - `order` - Order updates.
//! - `wallet` - Wallet balance updates.
//! - `greeks` - Options greeks updates.

pub mod client;
pub mod orderbook;
pub mod stream;
pub mod trade;
pub mod types;

pub use client::WsClient;
pub use orderbook::{LocalOrderbook, PriceLevel};
pub use stream::{IntoWsStream, WsStream};
pub use trade::{
    AmendOrderRequest, BatchOrderResult, CancelOrderRequest, CreateOrderRequest, OrderResult,
    WsTradeClient, WsTradeResponse,
};
pub use types::*;
