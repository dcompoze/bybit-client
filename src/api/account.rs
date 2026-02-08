//! Account API endpoints for wallet and account management.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::account::*;

/// Account service for wallet and account management endpoints.
#[derive(Debug, Clone)]
pub struct AccountService {
    http: HttpClient,
}

impl AccountService {
    /// Create a new account service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get wallet balance.
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
    ///     for coin in &wallet.coin {
    ///         println!("  {}: {}", coin.coin, coin.wallet_balance);
    ///     }
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_wallet_balance(
        &self,
        params: &GetWalletBalanceParams,
    ) -> Result<WalletBalanceResult, BybitError> {
        self.http
            .get_signed("/v5/account/wallet-balance", Some(params))
            .await
    }

    /// Get account information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let info = client.account().get_account_info().await?;
    /// println!("Margin mode: {}", info.margin_mode);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_account_info(&self) -> Result<AccountInfo, BybitError> {
        self.http
            .get_signed("/v5/account/info", None::<&()>)
            .await
    }

    /// Get fee rates for trading.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::account::GetFeeRatesParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetFeeRatesParams::new(Category::Linear)
    ///     .symbol("BTCUSDT");
    /// let result = client.account().get_fee_rates(&params).await?;
    /// for fee in &result.list {
    ///     println!("{}: maker={}, taker={}",
    ///         fee.symbol, fee.maker_fee_rate, fee.taker_fee_rate);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_fee_rates(
        &self,
        params: &GetFeeRatesParams,
    ) -> Result<FeeRateResult, BybitError> {
        self.http
            .get_signed("/v5/account/fee-rate", Some(params))
            .await
    }

    /// Get borrow history.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::account::GetBorrowHistoryParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetBorrowHistoryParams::new()
    ///     .currency("USDT")
    ///     .limit(20);
    /// let result = client.account().get_borrow_history(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_borrow_history(
        &self,
        params: &GetBorrowHistoryParams,
    ) -> Result<BorrowHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/account/borrow-history", Some(params))
            .await
    }

    /// Get collateral information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::account::GetCollateralInfoParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetCollateralInfoParams::new();
    /// let result = client.account().get_collateral_info(&params).await?;
    /// for info in &result.list {
    ///     println!("{}: borrowable={}", info.currency, info.borrowable);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_collateral_info(
        &self,
        params: &GetCollateralInfoParams,
    ) -> Result<CollateralInfoResult, BybitError> {
        self.http
            .get_signed("/v5/account/collateral-info", Some(params))
            .await
    }

    /// Set collateral coin switch.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::account::SetCollateralCoinParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Enable BTC as collateral
    /// let params = SetCollateralCoinParams::enable("BTC");
    /// client.account().set_collateral_coin(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_collateral_coin(
        &self,
        params: &SetCollateralCoinParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/set-collateral-switch", Some(params))
            .await?;
        Ok(())
    }

    /// Get coin greeks (for options).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::account::GetCoinGreeksParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetCoinGreeksParams::new().base_coin("BTC");
    /// let result = client.account().get_coin_greeks(&params).await?;
    /// for greeks in &result.list {
    ///     println!("{}: delta={}", greeks.base_coin, greeks.total_delta);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_coin_greeks(
        &self,
        params: &GetCoinGreeksParams,
    ) -> Result<CoinGreeksResult, BybitError> {
        self.http
            .get_signed("/v5/asset/coin-greeks", Some(params))
            .await
    }

    /// Get transaction log.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::account::GetTransactionLogParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetTransactionLogParams::new()
    ///     .currency("USDT")
    ///     .limit(50);
    /// let result = client.account().get_transaction_log(&params).await?;
    /// for log in &result.list {
    ///     println!("{}: {} - {}", log.transaction_time, log.transaction_type, log.change);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_transaction_log(
        &self,
        params: &GetTransactionLogParams,
    ) -> Result<TransactionLogResult, BybitError> {
        self.http
            .get_signed("/v5/account/transaction-log", Some(params))
            .await
    }

    /// Set margin mode (regular or portfolio margin).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::account::SetMarginModeParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = SetMarginModeParams::portfolio_margin();
    /// client.account().set_margin_mode(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_margin_mode(
        &self,
        params: &SetMarginModeParams,
    ) -> Result<MarginModeResult, BybitError> {
        self.http
            .post_signed("/v5/account/set-margin-mode", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{AccountType, Category};

    #[test]
    fn test_get_wallet_balance_params_serialization() {
        let params = GetWalletBalanceParams::new(AccountType::Unified).coin("BTC");

        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize wallet balance params: {}", err),
        };
        assert!(query.contains("accountType=UNIFIED"));
        assert!(query.contains("coin=BTC"));
    }

    #[test]
    fn test_get_fee_rates_params_serialization() {
        let params = GetFeeRatesParams::new(Category::Linear).symbol("BTCUSDT");

        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize fee rates params: {}", err),
        };
        assert!(query.contains("category=linear"));
        assert!(query.contains("symbol=BTCUSDT"));
    }

    #[test]
    fn test_set_collateral_coin_params_serialization() {
        let params = SetCollateralCoinParams::enable("BTC");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize collateral params: {}", err),
        };
        assert!(json.contains("\"coin\":\"BTC\""));
        assert!(json.contains("\"collateralSwitch\":\"ON\""));
    }

    #[test]
    fn test_set_margin_mode_params_serialization() {
        let params = SetMarginModeParams::portfolio_margin();
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize margin mode params: {}", err),
        };
        assert!(json.contains("\"setMarginMode\":\"PORTFOLIO_MARGIN\""));
    }
}
