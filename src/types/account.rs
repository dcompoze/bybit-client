//! Account-related types for wallet and account management.

use serde::{Deserialize, Serialize};

use crate::types::{AccountType, Category};

// ============================================================================
// Query Parameters
// ============================================================================

/// Parameters for getting wallet balance.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWalletBalanceParams {
    /// Account type.
    pub account_type: AccountType,
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetWalletBalanceParams {
    /// Create new parameters.
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Parameters for getting fee rates.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFeeRatesParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
}

impl GetFeeRatesParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set base coin filter.
    pub fn base_coin(mut self, coin: impl Into<String>) -> Self {
        self.base_coin = Some(coin.into());
        self
    }
}

/// Parameters for getting borrow history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowHistoryParams {
    /// Currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetBorrowHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self {
            currency: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set currency filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Set start time.
    pub fn start_time(mut self, start: u64) -> Self {
        self.start_time = Some(start);
        self
    }

    /// Set end time.
    pub fn end_time(mut self, end: u64) -> Self {
        self.end_time = Some(end);
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl Default for GetBorrowHistoryParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for getting collateral info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralInfoParams {
    /// Currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetCollateralInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self { currency: None }
    }

    /// Set currency filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

impl Default for GetCollateralInfoParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for setting collateral coin.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCollateralCoinParams {
    /// Coin to set.
    pub coin: String,
    /// Collateral switch (ON or OFF).
    pub collateral_switch: String,
}

impl SetCollateralCoinParams {
    /// Create parameters to enable collateral.
    pub fn enable(coin: impl Into<String>) -> Self {
        Self {
            coin: coin.into(),
            collateral_switch: "ON".to_string(),
        }
    }

    /// Create parameters to disable collateral.
    pub fn disable(coin: impl Into<String>) -> Self {
        Self {
            coin: coin.into(),
            collateral_switch: "OFF".to_string(),
        }
    }
}

/// Parameters for getting coin greeks.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinGreeksParams {
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
}

impl GetCoinGreeksParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self { base_coin: None }
    }

    /// Set base coin filter.
    pub fn base_coin(mut self, coin: impl Into<String>) -> Self {
        self.base_coin = Some(coin.into());
        self
    }
}

impl Default for GetCoinGreeksParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for getting transaction log.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransactionLogParams {
    /// Account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// Product category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// Currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Transaction type filter.
    #[serde(skip_serializing_if = "Option::is_none", rename = "type")]
    pub transaction_type: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetTransactionLogParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self {
            account_type: None,
            category: None,
            currency: None,
            base_coin: None,
            transaction_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set account type.
    pub fn account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = Some(account_type);
        self
    }

    /// Set category.
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Set currency filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Set transaction type filter.
    pub fn transaction_type(mut self, t: impl Into<String>) -> Self {
        self.transaction_type = Some(t.into());
        self
    }

    /// Set start time.
    pub fn start_time(mut self, start: u64) -> Self {
        self.start_time = Some(start);
        self
    }

    /// Set end time.
    pub fn end_time(mut self, end: u64) -> Self {
        self.end_time = Some(end);
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl Default for GetTransactionLogParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for setting margin mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetMarginModeParams {
    /// Margin mode: REGULAR_MARGIN or PORTFOLIO_MARGIN.
    pub set_margin_mode: String,
}

impl SetMarginModeParams {
    /// Set regular margin mode.
    pub fn regular_margin() -> Self {
        Self {
            set_margin_mode: "REGULAR_MARGIN".to_string(),
        }
    }

    /// Set portfolio margin mode.
    pub fn portfolio_margin() -> Self {
        Self {
            set_margin_mode: "PORTFOLIO_MARGIN".to_string(),
        }
    }
}

// ============================================================================
// Response Types
// ============================================================================

/// Coin balance within a wallet.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalance {
    /// Coin name.
    pub coin: String,
    /// Equity.
    pub equity: String,
    /// USD value.
    pub usd_value: String,
    /// Wallet balance.
    pub wallet_balance: String,
    /// Free balance (spot only).
    #[serde(default)]
    pub free: Option<String>,
    /// Locked balance (spot only).
    #[serde(default)]
    pub locked: Option<String>,
    /// Borrow amount.
    #[serde(default)]
    pub borrow_amount: Option<String>,
    /// Available to borrow.
    #[serde(default)]
    pub available_to_borrow: Option<String>,
    /// Available to withdraw.
    #[serde(default)]
    pub available_to_withdraw: Option<String>,
    /// Accrued interest.
    #[serde(default)]
    pub accrued_interest: Option<String>,
    /// Total order initial margin.
    #[serde(default)]
    pub total_order_i_m: Option<String>,
    /// Total position initial margin.
    #[serde(default)]
    pub total_position_i_m: Option<String>,
    /// Total position maintenance margin.
    #[serde(default)]
    pub total_position_m_m: Option<String>,
    /// Unrealised PnL.
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    /// Cumulative realised PnL.
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    /// Bonus.
    #[serde(default)]
    pub bonus: Option<String>,
    /// Margin collateral.
    #[serde(default)]
    pub margin_collateral: Option<bool>,
    /// Collateral switch.
    #[serde(default)]
    pub collateral_switch: Option<bool>,
    /// Spot borrow.
    #[serde(default)]
    pub spot_borrow: Option<String>,
}

/// Wallet balance.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalance {
    /// Account type.
    pub account_type: String,
    /// Account LTV.
    #[serde(default)]
    pub account_l_t_v: Option<String>,
    /// Account initial margin rate.
    #[serde(default)]
    pub account_i_m_rate: Option<String>,
    /// Account maintenance margin rate.
    #[serde(default)]
    pub account_m_m_rate: Option<String>,
    /// Total equity.
    pub total_equity: String,
    /// Total wallet balance.
    pub total_wallet_balance: String,
    /// Total margin balance.
    #[serde(default)]
    pub total_margin_balance: Option<String>,
    /// Total available balance.
    pub total_available_balance: String,
    /// Total perpetual unrealised PnL.
    #[serde(default)]
    pub total_perp_u_p_l: Option<String>,
    /// Total initial margin.
    #[serde(default)]
    pub total_initial_margin: Option<String>,
    /// Total maintenance margin.
    #[serde(default)]
    pub total_maintenance_margin: Option<String>,
    /// Coin balances.
    pub coin: Vec<CoinBalance>,
}

/// Wallet balance list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletBalanceResult {
    /// List of wallet balances.
    pub list: Vec<WalletBalance>,
}

/// Account information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AccountInfo {
    /// Unified margin status.
    #[serde(default)]
    pub unified_margin_status: Option<i32>,
    /// Margin mode.
    pub margin_mode: String,
    /// Is master trader.
    #[serde(default)]
    pub is_master_trader: Option<bool>,
    /// Spot hedging status.
    #[serde(default)]
    pub spot_hedging_status: Option<String>,
    /// Updated time.
    pub updated_time: String,
}

/// Fee rate information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRate {
    /// Trading symbol.
    pub symbol: String,
    /// Base coin.
    #[serde(default)]
    pub base_coin: Option<String>,
    /// Taker fee rate.
    pub taker_fee_rate: String,
    /// Maker fee rate.
    pub maker_fee_rate: String,
}

/// Fee rate list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeRateResult {
    /// Product category.
    #[serde(default)]
    pub category: Option<Category>,
    /// List of fee rates.
    pub list: Vec<FeeRate>,
}

/// Borrow history record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryRecord {
    /// Currency.
    pub currency: String,
    /// Created time.
    pub created_time: u64,
    /// Borrow cost.
    pub borrow_cost: String,
    /// Hourly borrow rate.
    pub hourly_borrow_rate: String,
    /// Interest bearing borrow size.
    #[serde(rename = "InterestBearingBorrowSize")]
    #[serde(default)]
    pub interest_bearing_borrow_size: Option<String>,
    /// Cost exemption.
    #[serde(default)]
    pub cost_exemption: Option<String>,
    /// Borrow amount.
    #[serde(default)]
    pub borrow_amount: Option<String>,
    /// Unrealised loss.
    #[serde(default)]
    pub unrealised_loss: Option<String>,
    /// Free borrowed amount.
    #[serde(default)]
    pub free_borrowed_amount: Option<String>,
}

/// Borrow history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowHistoryResult {
    /// List of borrow history records.
    pub list: Vec<BorrowHistoryRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Collateral information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfo {
    /// Currency.
    pub currency: String,
    /// Hourly borrow rate.
    pub hourly_borrow_rate: String,
    /// Max borrowing amount.
    pub max_borrowing_amount: String,
    /// Free borrow amount.
    #[serde(default)]
    pub free_borrow_amount: Option<String>,
    /// Free borrowing limit.
    #[serde(default)]
    pub free_borrowing_limit: Option<String>,
    /// Borrow amount.
    #[serde(default)]
    pub borrow_amount: Option<String>,
    /// Available to borrow.
    #[serde(default)]
    pub available_to_borrow: Option<String>,
    /// Is borrowable.
    pub borrowable: bool,
    /// Borrow usage rate.
    #[serde(default)]
    pub borrow_usage_rate: Option<String>,
    /// Margin collateral.
    #[serde(default)]
    pub margin_collateral: Option<bool>,
    /// Collateral switch.
    #[serde(default)]
    pub collateral_switch: Option<bool>,
    /// Collateral ratio.
    #[serde(default)]
    pub collateral_ratio: Option<String>,
}

/// Collateral info result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralInfoResult {
    /// List of collateral info.
    pub list: Vec<CollateralInfo>,
}

/// Coin greeks.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinGreeks {
    /// Base coin.
    pub base_coin: String,
    /// Total delta.
    pub total_delta: String,
    /// Total gamma.
    pub total_gamma: String,
    /// Total vega.
    pub total_vega: String,
    /// Total theta.
    pub total_theta: String,
}

/// Coin greeks result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinGreeksResult {
    /// List of coin greeks.
    pub list: Vec<CoinGreeks>,
}

/// Transaction log entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLog {
    /// Trading symbol.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Product category.
    #[serde(default)]
    pub category: Option<String>,
    /// Side.
    #[serde(default)]
    pub side: Option<String>,
    /// Transaction time.
    pub transaction_time: String,
    /// Transaction type.
    #[serde(rename = "type")]
    pub transaction_type: String,
    /// Quantity.
    #[serde(default)]
    pub qty: Option<String>,
    /// Size.
    #[serde(default)]
    pub size: Option<String>,
    /// Currency.
    pub currency: String,
    /// Trade price.
    #[serde(default)]
    pub trade_price: Option<String>,
    /// Funding.
    #[serde(default)]
    pub funding: Option<String>,
    /// Fee.
    #[serde(default)]
    pub fee: Option<String>,
    /// Cash flow.
    pub cash_flow: String,
    /// Change.
    pub change: String,
    /// Cash balance.
    pub cash_balance: String,
    /// Fee rate.
    #[serde(default)]
    pub fee_rate: Option<String>,
    /// Bonus change.
    #[serde(default)]
    pub bonus_change: Option<String>,
    /// Trade ID.
    #[serde(default)]
    pub trade_id: Option<String>,
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// Order link ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
}

/// Transaction log result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransactionLogResult {
    /// List of transaction logs.
    pub list: Vec<TransactionLog>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Margin mode result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeResult {
    /// List of reasons (if failed).
    #[serde(default)]
    pub reasons: Option<Vec<MarginModeReason>>,
}

/// Margin mode change reason.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginModeReason {
    /// Reason code.
    pub reason_code: String,
    /// Reason message.
    pub reason_msg: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_wallet_balance_params() {
        let params = GetWalletBalanceParams::new(AccountType::Unified)
            .coin("BTC");

        assert_eq!(params.coin, Some("BTC".to_string()));
    }

    #[test]
    fn test_get_fee_rates_params() {
        let params = GetFeeRatesParams::new(Category::Linear)
            .symbol("BTCUSDT");

        assert_eq!(params.symbol, Some("BTCUSDT".to_string()));
    }

    #[test]
    fn test_set_collateral_coin_params() {
        let enable = SetCollateralCoinParams::enable("BTC");
        assert_eq!(enable.collateral_switch, "ON");

        let disable = SetCollateralCoinParams::disable("ETH");
        assert_eq!(disable.collateral_switch, "OFF");
    }

    #[test]
    fn test_set_margin_mode_params() {
        let regular = SetMarginModeParams::regular_margin();
        assert_eq!(regular.set_margin_mode, "REGULAR_MARGIN");

        let portfolio = SetMarginModeParams::portfolio_margin();
        assert_eq!(portfolio.set_margin_mode, "PORTFOLIO_MARGIN");
    }
}
