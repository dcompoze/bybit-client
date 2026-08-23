//! API endpoint implementations.
//!
//! This module contains service structs for each API category:
//! - Market data (public).
//! - Trading (private).
//! - Position management (private).
//! - Account information (private).
//! - Asset management (private).
//! - User management (private).
//! - Spot margin trading (private).
//! - Spot leverage tokens (private).
//! - Crypto loans (private).
//! - Spread trading (public and private).
//! - Earn (private).
//! - Pre-upgrade history (private).

pub mod account;
pub mod asset;
pub mod crypto_loan;
pub mod earn;
pub mod market;
pub mod position;
pub mod pre_upgrade;
pub mod spot_leverage_token;
pub mod spot_margin;
pub mod spread;
pub mod trade;
pub mod user;

pub use account::AccountService;
pub use asset::AssetService;
pub use crypto_loan::CryptoLoanService;
pub use earn::EarnService;
pub use market::MarketService;
pub use position::PositionService;
pub use pre_upgrade::PreUpgradeService;
pub use spot_leverage_token::SpotLeverageTokenService;
pub use spot_margin::SpotMarginService;
pub use spread::SpreadService;
pub use trade::TradeService;
pub use user::UserService;
