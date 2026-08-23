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

    /// Upgrade the account to a unified trading account.
    pub async fn upgrade_to_uta(&self) -> Result<UpgradeToUtaResult, BybitError> {
        self.http
            .post_signed("/v5/account/upgrade-to-uta", None::<&()>)
            .await
    }

    /// Get the contract wallet transaction log (classic account).
    pub async fn get_contract_transaction_log(
        &self,
        params: &GetContractTransactionLogParams,
    ) -> Result<TransactionLogResult, BybitError> {
        self.http
            .get_signed("/v5/account/contract-transaction-log", Some(params))
            .await
    }

    /// Configure market maker protection.
    pub async fn mmp_modify(&self, params: &MmpModifyParams) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/mmp-modify", Some(params))
            .await?;
        Ok(())
    }

    /// Reset market maker protection after it has been triggered.
    pub async fn mmp_reset(&self, base_coin: impl Into<String>) -> Result<(), BybitError> {
        let params = BaseCoinParams::new(base_coin);
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/mmp-reset", Some(&params))
            .await?;
        Ok(())
    }

    /// Get market maker protection state.
    pub async fn get_mmp_state(
        &self,
        base_coin: impl Into<String>,
    ) -> Result<MmpStateResult, BybitError> {
        let params = BaseCoinParams::new(base_coin);
        self.http
            .get_signed("/v5/account/mmp-state", Some(&params))
            .await
    }

    /// Set spot hedging mode.
    pub async fn set_hedging_mode(&self, params: &SetHedgingModeParams) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/set-hedging-mode", Some(params))
            .await?;
        Ok(())
    }

    /// Set the limit price behaviour for orders that cross the price boundary.
    pub async fn set_limit_px_action(
        &self,
        params: &SetLimitPxActionParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/set-limit-px-action", Some(params))
            .await?;
        Ok(())
    }

    /// Set delta neutral mode.
    pub async fn set_delta_mode(&self, params: &SetDeltaModeParams) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/set-delta-mode", Some(params))
            .await?;
        Ok(())
    }

    /// Get disconnect-cancel-all configuration.
    pub async fn get_dcp_info(&self) -> Result<DcpInfoResult, BybitError> {
        self.http
            .get_signed("/v5/account/query-dcp-info", None::<&()>)
            .await
    }

    /// Get the SMP group ID for self match prevention.
    pub async fn get_smp_group(&self) -> Result<SmpGroupResult, BybitError> {
        self.http
            .get_signed("/v5/account/smp-group", None::<&()>)
            .await
    }

    /// Get user setting configuration.
    pub async fn get_user_setting_config(&self) -> Result<UserSettingConfig, BybitError> {
        self.http
            .get_signed("/v5/account/user-setting-config", None::<&()>)
            .await
    }

    /// Request demo trading funds (demo accounts only).
    pub async fn demo_apply_money(&self, params: &DemoApplyMoneyParams) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/account/demo-apply-money", Some(params))
            .await?;
        Ok(())
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

    #[test]
    fn test_mmp_modify_params_serialization() {
        let params = MmpModifyParams::new("BTC", "5000", "100000", "10", "20").vega_limit("30");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize MMP params: {}", err),
        };
        assert!(json.contains("\"baseCoin\":\"BTC\""));
        assert!(json.contains("\"window\":\"5000\""));
        assert!(json.contains("\"frozenPeriod\":\"100000\""));
        assert!(json.contains("\"qtyLimit\":\"10\""));
        assert!(json.contains("\"deltaLimit\":\"20\""));
        assert!(json.contains("\"vegaLimit\":\"30\""));
    }

    #[test]
    fn test_set_hedging_mode_params_serialization() {
        let params = SetHedgingModeParams::on();
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize hedging mode params: {}", err),
        };
        assert!(json.contains("\"setHedgingMode\":\"ON\""));
    }

    #[test]
    fn test_set_limit_px_action_params_serialization() {
        let params = SetLimitPxActionParams::new(Category::Spot, true);
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize limit px action params: {}", err),
        };
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"modifyEnable\":true"));
    }

    #[test]
    fn test_set_delta_mode_params_serialization() {
        let params = SetDeltaModeParams::enable();
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize delta mode params: {}", err),
        };
        assert!(json.contains("\"deltaEnable\":\"1\""));
    }

    #[test]
    fn test_demo_apply_money_params_serialization() {
        let params = DemoApplyMoneyParams::new()
            .adjust_type(0)
            .coin("USDT", "10000");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize demo apply money params: {}", err),
        };
        assert!(json.contains("\"adjustType\":0"));
        assert!(json.contains("\"utaDemoApplyMoney\""));
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"amountStr\":\"10000\""));
    }

    #[test]
    fn test_contract_transaction_log_params_serialization() {
        let params = GetContractTransactionLogParams::new()
            .currency("USDT")
            .transaction_type("TRADE")
            .limit(20);
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize contract transaction log params: {}", err),
        };
        assert!(query.contains("currency=USDT"));
        assert!(query.contains("type=TRADE"));
        assert!(query.contains("limit=20"));
    }
}
