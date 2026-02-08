//! Main Bybit client.

use crate::api::{AccountService, MarketService, PositionService, TradeService};
use crate::config::ClientConfig;
use crate::error::BybitError;
use crate::http::HttpClient;

/// Main client for interacting with the Bybit API.
///
/// # Example
///
/// ```no_run
/// use bybit_client::{BybitClient, ClientConfig};
///
/// #[tokio::main]
/// async fn main() -> Result<(), Box<dyn std::error::Error>> {
///     // Create a public-only client.
///     let client = BybitClient::public_only()?;
///
///     // Or create an authenticated client.
///     let client = BybitClient::new("api_key", "api_secret")?;
///
///     // Use testnet.
///     let client = BybitClient::with_config(
///         ClientConfig::new("api_key", "api_secret").testnet()
///     )?;
///
///     Ok(())
/// }
/// ```
#[derive(Debug, Clone)]
pub struct BybitClient {
    http: HttpClient,
}

impl BybitClient {
    /// Create a new client with API credentials.
    pub fn new(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
    ) -> Result<Self, BybitError> {
        let config = ClientConfig::new(api_key, api_secret);
        Self::with_config(config)
    }

    /// Create a client for public endpoints only (no authentication).
    pub fn public_only() -> Result<Self, BybitError> {
        let config = ClientConfig::public_only();
        Self::with_config(config)
    }

    /// Create a client with custom configuration.
    pub fn with_config(config: ClientConfig) -> Result<Self, BybitError> {
        let http = HttpClient::new(config)?;
        Ok(Self { http })
    }

    /// Get the underlying HTTP client.
    pub fn http(&self) -> &HttpClient {
        &self.http
    }

    /// Get the client configuration.
    pub fn config(&self) -> &ClientConfig {
        self.http.config()
    }

    /// Synchronize time with the server.
    ///
    /// This is useful when your system clock is not accurate.
    /// The client will adjust timestamps in authenticated requests
    /// based on the calculated offset.
    pub async fn sync_time(&self) -> Result<i64, BybitError> {
        self.http.sync_time().await
    }

    /// Check if the client has authentication credentials.
    pub fn has_credentials(&self) -> bool {
        self.config().has_credentials()
    }

    /// Get the market data service for public endpoints.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::api::market::GetTickersParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetTickersParams::new(Category::Linear).symbol("BTCUSDT");
    /// let tickers = client.market().get_tickers(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn market(&self) -> MarketService {
        MarketService::new(self.http.clone())
    }

    /// Get the trade service for order management endpoints.
    ///
    /// Note: Trading endpoints require authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category, Side, OrderType};
    /// # use bybit_client::types::trade::OrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Place a market order.
    /// let params = OrderParams::market(Category::Linear, "BTCUSDT", Side::Buy, "0.001");
    /// let result = client.trade().submit_order(&params).await?;
    /// println!("Order ID: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub fn trade(&self) -> TradeService {
        TradeService::new(self.http.clone())
    }

    /// Get the position service for position management endpoints.
    ///
    /// Note: Position endpoints require authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::GetPositionInfoParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetPositionInfoParams::new(Category::Linear).symbol("BTCUSDT");
    /// let result = client.position().get_position_info(&params).await?;
    /// for pos in &result.list {
    ///     println!("{}: {} @ {}", pos.symbol, pos.size, pos.avg_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn position(&self) -> PositionService {
        PositionService::new(self.http.clone())
    }

    /// Get the account service for wallet and account management endpoints.
    ///
    /// Note: Account endpoints require authentication.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, AccountType};
    /// # use bybit_client::types::account::GetWalletBalanceParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetWalletBalanceParams::new(AccountType::Unified);
    /// let result = client.account().get_wallet_balance(&params).await?;
    /// for wallet in &result.list {
    ///     println!("Total equity: {}", wallet.total_equity);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub fn account(&self) -> AccountService {
        AccountService::new(self.http.clone())
    }

    /// Make a public GET request.
    pub async fn get<T, P>(&self, endpoint: &str, params: Option<&P>) -> Result<T, BybitError>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        self.http.get(endpoint, params).await
    }

    /// Make an authenticated GET request.
    pub async fn get_signed<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> Result<T, BybitError>
    where
        T: serde::de::DeserializeOwned,
        P: serde::Serialize + ?Sized,
    {
        self.http.get_signed(endpoint, params).await
    }

    /// Make an authenticated POST request.
    pub async fn post_signed<T, B>(&self, endpoint: &str, body: Option<&B>) -> Result<T, BybitError>
    where
        T: serde::de::DeserializeOwned,
        B: serde::Serialize + ?Sized,
    {
        self.http.post_signed(endpoint, body).await
    }
}

/// Builder for creating a BybitClient with custom configuration.
#[derive(Debug, Default)]
pub struct BybitClientBuilder {
    config: ClientConfig,
}

impl BybitClientBuilder {
    /// Create a new builder.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set API credentials.
    pub fn credentials(
        mut self,
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
    ) -> Self {
        self.config = ClientConfig::new(api_key, api_secret);
        self
    }

    /// Use testnet environment.
    pub fn testnet(mut self) -> Self {
        self.config = self.config.testnet();
        self
    }

    /// Use demo trading environment.
    pub fn demo(mut self) -> Self {
        self.config = self.config.demo();
        self
    }

    /// Set recv_window.
    pub fn recv_window(mut self, ms: u32) -> Self {
        self.config = self.config.recv_window(ms);
        self
    }

    /// Enable debug mode.
    pub fn debug(mut self, enabled: bool) -> Self {
        self.config = self.config.debug(enabled);
        self
    }

    /// Set request timeout.
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.config = self.config.timeout_ms(ms);
        self
    }

    /// Build the client.
    pub fn build(self) -> Result<BybitClient, BybitError> {
        BybitClient::with_config(self.config)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_builder() {
        let client = match BybitClientBuilder::new()
            .credentials("test_key", "test_secret")
            .testnet()
            .debug(true)
            .build()
        {
            Ok(client) => client,
            Err(err) => panic!("Failed to build client: {}", err),
        };

        assert!(client.has_credentials());
        assert!(client.config().debug);
    }

    #[test]
    fn test_public_only_client() {
        let client = match BybitClient::public_only() {
            Ok(client) => client,
            Err(err) => panic!("Failed to build public client: {}", err),
        };
        assert!(!client.has_credentials());
    }
}
