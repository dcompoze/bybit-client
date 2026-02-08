//! # Bybit Client
//!
//! A Rust client library for the Bybit V5 API.
//!
//! ## Features
//!
//! - Full REST API V5 coverage.
//! - WebSocket support for real-time data.
//! - HMAC-SHA256 authentication.
//! - Async/await support with tokio.
//! - Strongly typed request/response structures.
//!
//! ## Quick Start
//!
//! ```no_run
//! use bybit_client::{BybitClient, ClientConfig};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create a public-only client for public endpoints.
//!     let client = BybitClient::public_only()?;
//!
//!     // Or create an authenticated client for private endpoints.
//!     let client = BybitClient::new("your_api_key", "your_api_secret")?;
//!
//!     // Use testnet for testing.
//!     let client = BybitClient::with_config(
//!         ClientConfig::new("api_key", "api_secret").testnet()
//!     )?;
//!
//!     Ok(())
//! }
//! ```
//!
//! ## Configuration
//!
//! The client supports various configuration options:
//!
//! ```no_run
//! use bybit_client::{ClientConfig, Environment, ApiRegion};
//!
//! let config = ClientConfig::new("api_key", "api_secret")
//!     .testnet()                    // Use testnet.
//!     .recv_window(10000)           // Set `recv_window` to 10 seconds.
//!     .debug(true)                  // Enable debug logging.
//!     .timeout_ms(30000);           // Set request timeout to 30 seconds.
//! ```

pub mod api;
pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod http;
pub mod types;
pub mod ws;

pub use client::{BybitClient, BybitClientBuilder};
pub use config::{ApiRegion, ClientConfig, Environment};
pub use error::{ApiResponse, BybitError, ListResult};
pub use types::*;

/// Prelude module for common imports.
pub mod prelude {
    pub use crate::client::{BybitClient, BybitClientBuilder};
    pub use crate::config::{ApiRegion, ClientConfig, Environment};
    pub use crate::error::{ApiResponse, BybitError, ListResult};
    pub use crate::types::*;
}
