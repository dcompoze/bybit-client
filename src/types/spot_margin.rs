//! Types for the UTA spot margin trade endpoints.

use serde::{Deserialize, Serialize};

/// Parameters for switching spot margin mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchSpotMarginModeParams {
    /// Spot margin mode ("1": on, "0": off).
    pub spot_margin_mode: String,
}

impl SwitchSpotMarginModeParams {
    /// Turn spot margin trading on.
    pub fn on() -> Self {
        Self {
            spot_margin_mode: "1".to_string(),
        }
    }

    /// Turn spot margin trading off.
    pub fn off() -> Self {
        Self {
            spot_margin_mode: "0".to_string(),
        }
    }
}

/// Spot margin mode result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarginModeResult {
    /// Spot margin mode ("1": on, "0": off).
    pub spot_margin_mode: String,
}

/// Parameters for setting spot margin leverage.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetSpotMarginLeverageParams {
    /// Leverage, for example "2" to "10".
    pub leverage: String,
}

impl SetSpotMarginLeverageParams {
    /// Create new parameters.
    pub fn new(leverage: impl Into<String>) -> Self {
        Self {
            leverage: leverage.into(),
        }
    }
}

/// Spot margin status and leverage.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarginState {
    /// Maximum leverage setting.
    pub spot_leverage: String,
    /// Spot margin mode ("1": on, "0": off).
    pub spot_margin_mode: String,
    /// Actual leverage ratio in use.
    #[serde(default)]
    pub effective_leverage: Option<String>,
}

/// Parameters for VIP margin data.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetVipMarginDataParams {
    /// VIP level filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    /// Coin name filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetVipMarginDataParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set VIP level filter.
    pub fn vip_level(mut self, level: impl Into<String>) -> Self {
        self.vip_level = Some(level.into());
        self
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// VIP margin data per coin.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipMarginCoin {
    /// Coin name.
    pub currency: String,
    /// Whether the coin can be borrowed.
    #[serde(default)]
    pub borrowable: Option<bool>,
    /// Collateral ratio.
    #[serde(default)]
    pub collateral_ratio: Option<String>,
    /// Hourly borrow interest rate.
    #[serde(default)]
    pub hourly_borrow_rate: Option<String>,
    /// Liquidation order.
    #[serde(default)]
    pub liquidation_order: Option<String>,
    /// Whether the coin can be used as margin collateral.
    #[serde(default)]
    pub margin_collateral: Option<bool>,
    /// Maximum borrowing amount.
    #[serde(default)]
    pub max_borrowing_amount: Option<String>,
}

/// VIP margin data grouped by VIP level.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipCoinGroup {
    /// VIP level.
    pub vip_level: String,
    /// Coin margin data list.
    pub list: Vec<VipMarginCoin>,
}

/// VIP margin data result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipMarginDataResult {
    /// VIP coin groups.
    pub vip_coin_list: Vec<VipCoinGroup>,
}

/// Parameters for historical interest rate.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInterestRateHistoryParams {
    /// Coin name, uppercase only.
    pub currency: String,
    /// VIP level filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

impl GetInterestRateHistoryParams {
    /// Create new parameters.
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            vip_level: None,
            start_time: None,
            end_time: None,
        }
    }

    /// Set VIP level filter.
    pub fn vip_level(mut self, level: impl Into<String>) -> Self {
        self.vip_level = Some(level.into());
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
}

/// Historical interest rate entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateEntry {
    /// Timestamp (ms).
    pub timestamp: i64,
    /// Coin name.
    pub currency: String,
    /// Hourly borrow rate.
    pub hourly_borrow_rate: String,
    /// VIP level.
    pub vip_level: String,
}

/// Historical interest rate result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterestRateHistoryResult {
    /// Interest rate entries.
    pub list: Vec<InterestRateEntry>,
}

/// Parameters for tiered collateral ratio.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTieredCollateralRatioParams {
    /// Coin name filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetTieredCollateralRatioParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Collateral ratio tier.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralRatioTier {
    /// Minimum quantity of the tier.
    #[serde(default)]
    pub min_qty: Option<String>,
    /// Maximum quantity of the tier.
    #[serde(default)]
    pub max_qty: Option<String>,
    /// Collateral ratio for the tier.
    #[serde(default)]
    pub collateral_ratio: Option<String>,
}

/// Tiered collateral ratio per coin.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TieredCollateralRatio {
    /// Coin name.
    pub currency: String,
    /// Collateral ratio tiers.
    #[serde(default)]
    pub collateral_ratio_list: Vec<CollateralRatioTier>,
}

/// Parameters for maximum borrowable amount.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxBorrowableParams {
    /// Coin name.
    pub currency: String,
}

impl GetMaxBorrowableParams {
    /// Create new parameters.
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
        }
    }
}

/// Maximum borrowable amount result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxBorrowableResult {
    /// Coin name.
    pub currency: String,
    /// Maximum loan amount.
    pub max_loan: String,
}

/// Parameters for position tiers.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionTiersParams {
    /// Coin name filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetPositionTiersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Position tier entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionTier {
    /// Tier number.
    #[serde(default)]
    pub tier: Option<String>,
    /// Borrow limit for the tier.
    #[serde(default)]
    pub borrow_limit: Option<String>,
    /// Position maintenance margin rate.
    #[serde(default, rename = "positionMMR")]
    pub position_mmr: Option<String>,
    /// Position initial margin rate.
    #[serde(default, rename = "positionIMR")]
    pub position_imr: Option<String>,
    /// Maximum leverage for the tier.
    #[serde(default)]
    pub max_leverage: Option<String>,
}

/// Position tiers per coin.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrencyPositionTiers {
    /// Coin name.
    pub currency: String,
    /// Position tier entries.
    #[serde(default)]
    pub position_tiers_ratio_list: Vec<PositionTier>,
}

/// Parameters for coin state.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinStateParams {
    /// Coin name filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetCoinStateParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Coin leverage state.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinState {
    /// Coin name.
    pub currency: String,
    /// Spot leverage for the coin.
    pub spot_leverage: String,
}

/// Parameters for repayment available amount.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRepaymentAvailableAmountParams {
    /// Coin name.
    pub currency: String,
}

impl GetRepaymentAvailableAmountParams {
    /// Create new parameters.
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
        }
    }
}

/// Repayment available amount result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RepaymentAvailableAmount {
    /// Coin name.
    pub currency: String,
    /// Amount available for loss-less repayment.
    pub loss_less_repayment_amount: String,
}

/// Parameters for getting auto-repay mode.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAutoRepayModeParams {
    /// Coin name filter.
    /// When omitted, all currencies are returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetAutoRepayModeParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Parameters for setting auto-repay mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoRepayModeParams {
    /// Auto-repay mode ("1": on, "0": off).
    pub auto_repay_mode: String,
    /// Coin name.
    /// When omitted, the mode applies to all currencies.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl SetAutoRepayModeParams {
    /// Enable auto-repay.
    pub fn on() -> Self {
        Self {
            auto_repay_mode: "1".to_string(),
            currency: None,
        }
    }

    /// Disable auto-repay.
    pub fn off() -> Self {
        Self {
            auto_repay_mode: "0".to_string(),
            currency: None,
        }
    }

    /// Set coin.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Auto-repay mode entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoRepayModeItem {
    /// Coin name.
    pub currency: String,
    /// Auto-repay mode ("1": on, "0": off).
    pub auto_repay_mode: String,
}

/// Auto-repay mode result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AutoRepayModeResult {
    /// Auto-repay mode entries.
    #[serde(default)]
    pub data: Vec<AutoRepayModeItem>,
}

/// Parameters for liability info.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLiabilityParams {
    /// Coin name filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetLiabilityParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Liability information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiabilityInfo {
    /// Coin name.
    pub currency: String,
    /// Total borrowed amount.
    #[serde(default)]
    pub total_borrow_amount: Option<String>,
    /// Fixed-rate borrowed amount.
    #[serde(default)]
    pub fixed_borrow_amount: Option<String>,
    /// Flexible borrowed amount.
    #[serde(default)]
    pub flexible_borrow_amount: Option<String>,
    /// Total spot borrow.
    #[serde(default)]
    pub spot_total_borrow: Option<String>,
    /// Derivatives borrow.
    #[serde(default)]
    pub derivatives_borrow: Option<String>,
}

/// Parameters for currency data.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCurrencyDataParams {
    /// Coin name filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetCurrencyDataParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Borrowing currency data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotMarginCurrencyData {
    /// Coin name.
    pub currency: String,
    /// Whether flexible manual borrow is available.
    #[serde(default)]
    pub flexible_manual_borrowable: Option<bool>,
    /// Minimum flexible manual borrow quantity.
    #[serde(default)]
    pub min_flexible_manual_borrow_qty: Option<String>,
    /// Flexible manual borrow precision.
    #[serde(default)]
    pub flexible_manual_borrow_accuracy: Option<String>,
    /// Whether fixed manual borrow is available.
    #[serde(default)]
    pub fixed_manual_borrowable: Option<bool>,
    /// Minimum fixed manual borrow quantity.
    #[serde(default)]
    pub min_fixed_manual_borrow_qty: Option<String>,
    /// Fixed manual borrow precision.
    #[serde(default)]
    pub fixed_manual_borrow_accuracy: Option<String>,
    /// Fixed interest rate precision.
    #[serde(default)]
    pub fixed_interest_rate_accuracy: Option<String>,
    /// Minimum fixed interest rate.
    #[serde(default)]
    pub min_fixed_interest_rate: Option<String>,
    /// Maximum fixed interest rate.
    #[serde(default)]
    pub max_fixed_interest_rate: Option<String>,
}
