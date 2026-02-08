//! API endpoint implementations.
//!
//! This module contains service structs for each API category:
//! - Market data (public).
//! - Trading (private).
//! - Position management (private).
//! - Account information (private).
//! - Asset management (private).
//! - User management (private).

pub mod account;
pub mod market;
pub mod position;
pub mod trade;

pub use account::AccountService;
pub use market::MarketService;
pub use position::PositionService;
pub use trade::TradeService;
