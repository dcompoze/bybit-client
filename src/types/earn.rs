//! Types for the earn endpoints.

use serde::{Deserialize, Serialize};

/// Parameters for getting earn product info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnProductParams {
    /// Earn category (e.g. `FlexibleSaving`, `OnChain`).
    pub category: String,
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetEarnProductParams {
    /// Create new parameters.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            coin: None,
        }
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Earn product information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnProduct {
    /// Earn category.
    pub category: String,
    /// Estimated APR (e.g. `3%`).
    #[serde(default)]
    pub estimate_apr: Option<String>,
    /// Coin.
    pub coin: String,
    /// Minimum stake amount.
    #[serde(default)]
    pub min_stake_amount: Option<String>,
    /// Maximum stake amount.
    #[serde(default)]
    pub max_stake_amount: Option<String>,
    /// Amount precision.
    #[serde(default)]
    pub precision: Option<String>,
    /// Product ID.
    pub product_id: String,
    /// Product status (Available, NotAvailable).
    #[serde(default)]
    pub status: Option<String>,
}

/// Earn product list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnProductResult {
    /// List of products.
    pub list: Vec<EarnProduct>,
}

/// Parameters for placing an earn order (stake or redeem).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceEarnOrderParams {
    /// Earn category.
    pub category: String,
    /// Order type (`Stake` or `Redeem`).
    pub order_type: String,
    /// Account type (`FUND` or `UNIFIED`).
    pub account_type: String,
    /// Order amount.
    pub amount: String,
    /// Coin.
    pub coin: String,
    /// Product ID.
    pub product_id: String,
    /// User custom order ID.
    pub order_link_id: String,
    /// Target account type for redemption.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<String>,
}

impl PlaceEarnOrderParams {
    /// Create a stake order.
    pub fn stake(
        category: impl Into<String>,
        account_type: impl Into<String>,
        amount: impl Into<String>,
        coin: impl Into<String>,
        product_id: impl Into<String>,
        order_link_id: impl Into<String>,
    ) -> Self {
        Self {
            category: category.into(),
            order_type: "Stake".to_string(),
            account_type: account_type.into(),
            amount: amount.into(),
            coin: coin.into(),
            product_id: product_id.into(),
            order_link_id: order_link_id.into(),
            to_account_type: None,
        }
    }

    /// Create a redeem order.
    pub fn redeem(
        category: impl Into<String>,
        account_type: impl Into<String>,
        amount: impl Into<String>,
        coin: impl Into<String>,
        product_id: impl Into<String>,
        order_link_id: impl Into<String>,
    ) -> Self {
        Self {
            category: category.into(),
            order_type: "Redeem".to_string(),
            account_type: account_type.into(),
            amount: amount.into(),
            coin: coin.into(),
            product_id: product_id.into(),
            order_link_id: order_link_id.into(),
            to_account_type: None,
        }
    }

    /// Set target account type for redemption.
    pub fn to_account_type(mut self, account_type: impl Into<String>) -> Self {
        self.to_account_type = Some(account_type.into());
        self
    }
}

/// Earn order placement result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrderResult {
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    pub order_link_id: String,
}

/// Parameters for getting earn order history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnOrderHistoryParams {
    /// Earn category.
    pub category: String,
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Order link ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Product ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetEarnOrderHistoryParams {
    /// Create new parameters.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            order_id: None,
            order_link_id: None,
            product_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set order ID filter.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    /// Set order link ID filter.
    pub fn order_link_id(mut self, id: impl Into<String>) -> Self {
        self.order_link_id = Some(id.into());
        self
    }

    /// Set product ID filter.
    pub fn product_id(mut self, id: impl Into<String>) -> Self {
        self.product_id = Some(id.into());
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

/// Earn stake or redeem order record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrder {
    /// Coin.
    pub coin: String,
    /// Order value.
    #[serde(default)]
    pub order_value: Option<String>,
    /// Order type (Stake, Redeem).
    pub order_type: String,
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Order status (Success, Fail, Pending).
    pub status: String,
    /// Created time (ms).
    #[serde(default)]
    pub created_at: Option<String>,
    /// Product ID.
    pub product_id: String,
    /// Updated time (ms).
    #[serde(default)]
    pub updated_at: Option<String>,
    /// Swap order value.
    #[serde(default)]
    pub swap_order_value: Option<String>,
    /// Estimated redeem time (ms).
    #[serde(default)]
    pub estimate_redeem_time: Option<String>,
    /// Estimated stake time (ms).
    #[serde(default)]
    pub estimate_stake_time: Option<String>,
}

/// Earn order history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnOrderHistoryResult {
    /// List of orders.
    pub list: Vec<EarnOrder>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting staked positions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnPositionParams {
    /// Earn category.
    pub category: String,
    /// Product ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetEarnPositionParams {
    /// Create new parameters.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            product_id: None,
            coin: None,
        }
    }

    /// Set product ID filter.
    pub fn product_id(mut self, id: impl Into<String>) -> Self {
        self.product_id = Some(id.into());
        self
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Frozen amount detail on a staked position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnFreezeDetail {
    /// Frozen amount.
    pub amount: String,
    /// Freeze reason description.
    #[serde(default)]
    pub description: Option<String>,
}

/// Staked earn position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnPosition {
    /// Coin.
    pub coin: String,
    /// Product ID.
    pub product_id: String,
    /// Staked amount.
    pub amount: String,
    /// Total profit and loss.
    #[serde(default)]
    pub total_pnl: Option<String>,
    /// Claimable yield.
    #[serde(default)]
    pub claimable_yield: Option<String>,
    /// Position ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Position status.
    #[serde(default)]
    pub status: Option<String>,
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// Estimated redeem time (ms).
    #[serde(default)]
    pub estimate_redeem_time: Option<String>,
    /// Estimated stake time (ms).
    #[serde(default)]
    pub estimate_stake_time: Option<String>,
    /// Estimated interest calculation time (ms).
    #[serde(default)]
    pub estimate_interest_calculation_time: Option<String>,
    /// Settlement time (ms).
    #[serde(default)]
    pub settlement_time: Option<String>,
    /// Auto reinvest flag.
    #[serde(default)]
    pub auto_reinvest: Option<String>,
    /// Redeemable amount.
    #[serde(default)]
    pub available_amount: Option<String>,
    /// Frozen amount details.
    #[serde(default)]
    pub freeze_details: Option<Vec<EarnFreezeDetail>>,
}

/// Staked position list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnPositionResult {
    /// List of positions.
    pub list: Vec<EarnPosition>,
}

/// Parameters for modifying an OnChain earn position.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ModifyEarnPositionParams {
    /// Earn category (must be `OnChain`).
    pub category: String,
    /// Product ID.
    pub product_id: i64,
    /// Position ID.
    pub position_id: i64,
    /// Auto reinvest (0: off, 1: on).
    pub auto_reinvest: i32,
}

impl ModifyEarnPositionParams {
    /// Create new parameters for an OnChain position.
    pub fn new(product_id: i64, position_id: i64, auto_reinvest: bool) -> Self {
        Self {
            category: "OnChain".to_string(),
            product_id,
            position_id,
            auto_reinvest: if auto_reinvest { 1 } else { 0 },
        }
    }
}

/// Parameters for getting yield history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnYieldParams {
    /// Earn category.
    pub category: String,
    /// Product ID filter.
    /// Do not pass for the `OnChain` category.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_id: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetEarnYieldParams {
    /// Create new parameters.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            product_id: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set product ID filter.
    pub fn product_id(mut self, id: impl Into<String>) -> Self {
        self.product_id = Some(id.into());
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

/// Yield history record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnYield {
    /// Product ID.
    pub product_id: String,
    /// Coin.
    pub coin: String,
    /// Record ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Yield amount.
    pub amount: String,
    /// Yield type.
    #[serde(default)]
    pub yield_type: Option<String>,
    /// Distribution mode.
    #[serde(default)]
    pub distribution_mode: Option<String>,
    /// Effective staking amount.
    #[serde(default)]
    pub effective_staking_amount: Option<String>,
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// Status (Pending, Success, Fail).
    #[serde(default)]
    pub status: Option<String>,
    /// Created time (ms).
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Yield history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnYieldResult {
    /// List of yield records.
    #[serde(rename = "yield")]
    pub yields: Vec<EarnYield>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Hourly yield history record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnHourlyYield {
    /// Product ID.
    pub product_id: String,
    /// Coin.
    pub coin: String,
    /// Record ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Yield amount.
    pub amount: String,
    /// Effective staking amount.
    #[serde(default)]
    pub effective_staking_amount: Option<String>,
    /// Status (Pending, Success, Fail).
    #[serde(default)]
    pub status: Option<String>,
    /// Hourly date.
    #[serde(default)]
    pub hourly_date: Option<String>,
    /// Created time (ms).
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Hourly yield history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnHourlyYieldResult {
    /// List of hourly yield records.
    pub list: Vec<EarnHourlyYield>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting APR history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnAprHistoryParams {
    /// Earn category (`FlexibleSaving` or `OnChain`).
    pub category: String,
    /// Product ID.
    pub product_id: String,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

impl GetEarnAprHistoryParams {
    /// Create new parameters.
    pub fn new(category: impl Into<String>, product_id: impl Into<String>) -> Self {
        Self {
            category: category.into(),
            product_id: product_id.into(),
            start_time: None,
            end_time: None,
        }
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

/// APR history point.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnAprPoint {
    /// Timestamp (ms).
    pub timestamp: String,
    /// APR value.
    pub apr: String,
}

/// APR history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnAprHistoryResult {
    /// List of APR points.
    pub list: Vec<EarnAprPoint>,
}

/// Parameters for getting coupon list.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetEarnCouponsParams {
    /// Earn category (`FlexibleSaving` or `DualAssets`).
    pub category: String,
}

impl GetEarnCouponsParams {
    /// Create new parameters.
    pub fn new(category: impl Into<String>) -> Self {
        Self {
            category: category.into(),
        }
    }
}

/// Interest rate coupon card.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnInterestCard {
    /// Award ID.
    pub award_id: i64,
    /// Coupon spec code.
    #[serde(default)]
    pub spec_code: Option<String>,
    /// Coin.
    #[serde(default)]
    pub coin: Option<String>,
    /// Boosted APY.
    #[serde(default)]
    pub apy: Option<String>,
    /// Duration in days.
    #[serde(default)]
    pub duration: Option<i64>,
    /// Claimed time (ms).
    #[serde(default)]
    pub claimed_at: Option<i64>,
    /// Expiry time (ms).
    #[serde(default)]
    pub expire_at: Option<i64>,
    /// Used time (ms).
    #[serde(default)]
    pub used_at: Option<i64>,
    /// Coupon status (InUse, NotUse, Expired, AlreadyUsed).
    #[serde(default)]
    pub status: Option<String>,
    /// Current PnL.
    #[serde(default)]
    pub current_pnl: Option<String>,
    /// PnL limit.
    #[serde(default)]
    pub limit_pnl: Option<String>,
    /// Position effective amount.
    #[serde(default)]
    pub position_effective_amount: Option<String>,
    /// Product ID.
    #[serde(default)]
    pub product_id: Option<i64>,
    /// Earn category.
    #[serde(default)]
    pub category: Option<String>,
}

/// Award coupon card.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnAwardCard {
    /// Award ID.
    pub award_id: i64,
    /// Coupon spec code.
    #[serde(default)]
    pub spec_code: Option<String>,
    /// Claimed time (ms).
    #[serde(default)]
    pub claimed_at: Option<i64>,
    /// Used time (ms).
    #[serde(default)]
    pub used_at: Option<i64>,
    /// Expiry time (ms).
    #[serde(default)]
    pub expire_at: Option<i64>,
    /// Coupon status (InUse, NotUse, Expired, AlreadyUsed).
    #[serde(default)]
    pub status: Option<String>,
    /// Award amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// PnL limit percentage.
    #[serde(default)]
    pub limit_pnl_percentage: Option<String>,
    /// Base coin.
    #[serde(default)]
    pub base_coin: Option<String>,
    /// Quote coin.
    #[serde(default)]
    pub quote_coin: Option<String>,
    /// Direction (1 or 2).
    #[serde(default)]
    pub direction: Option<i32>,
    /// Earn category.
    #[serde(default)]
    pub category: Option<String>,
}

/// Coupon list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarnCouponsResult {
    /// Interest rate coupon cards.
    #[serde(default)]
    pub interest_cards: Vec<EarnInterestCard>,
    /// Award coupon cards.
    #[serde(default)]
    pub award_cards: Vec<EarnAwardCard>,
}
