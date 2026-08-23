//! Types for the new crypto loan endpoints.
//!
//! Covers the common, flexible, and fixed loan groups.
//! The legacy `/v5/crypto-loan/*` endpoints are deprecated and not included.

use serde::{Deserialize, Serialize};

/// Collateral coin entry used in borrow requests.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralCoinItem {
    /// Collateral currency.
    pub currency: String,
    /// Collateral amount.
    pub amount: String,
}

impl CollateralCoinItem {
    /// Create a new collateral entry.
    pub fn new(currency: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            amount: amount.into(),
        }
    }
}

/// Parameters for getting collateral coin data.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCollateralDataParams {
    /// Collateral currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
}

impl GetCollateralDataParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set currency filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }
}

/// Collateral coin data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralCoin {
    /// Collateral currency.
    #[serde(default)]
    pub currency: Option<String>,
    /// Collateral precision.
    #[serde(default)]
    pub collateral_accuracy: Option<i32>,
    /// Initial LTV.
    #[serde(default)]
    pub initial_ltv: Option<String>,
    /// Margin call LTV.
    #[serde(default)]
    pub margin_call_ltv: Option<String>,
    /// Liquidation LTV.
    #[serde(default)]
    pub liquidation_ltv: Option<String>,
    /// Maximum collateral limit.
    #[serde(default)]
    pub max_limit: Option<String>,
}

/// Collateral coins grouped by VIP level.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipCollateralCoins {
    /// VIP level.
    #[serde(default)]
    pub vip_level: Option<String>,
    /// Collateral coin list.
    #[serde(default)]
    pub list: Vec<CollateralCoin>,
}

/// Collateral data result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CollateralDataResult {
    /// Collateral coins grouped by VIP level.
    #[serde(default)]
    pub vip_coin_list: Vec<VipCollateralCoins>,
}

/// Parameters for getting loanable coin data.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLoanableDataParams {
    /// Loan currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// VIP level filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub vip_level: Option<String>,
}

impl GetLoanableDataParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set currency filter.
    pub fn currency(mut self, currency: impl Into<String>) -> Self {
        self.currency = Some(currency.into());
        self
    }

    /// Set VIP level filter.
    pub fn vip_level(mut self, level: impl Into<String>) -> Self {
        self.vip_level = Some(level.into());
        self
    }
}

/// Loanable coin data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanableCoin {
    /// Loan currency.
    #[serde(default)]
    pub currency: Option<String>,
    /// Borrowing precision.
    #[serde(default)]
    pub borrowing_accuracy: Option<i32>,
    /// Hourly interest rate for flexible loans.
    #[serde(default)]
    pub flexible_hourly_interest_rate: Option<String>,
    /// Annual interest rate for 7 day fixed loans.
    #[serde(default)]
    pub fixed_borrowing_rate_7_d: Option<String>,
    /// Annual interest rate for 14 day fixed loans.
    #[serde(default)]
    pub fixed_borrowing_rate_14_d: Option<String>,
    /// Annual interest rate for 30 day fixed loans.
    #[serde(default)]
    pub fixed_borrowing_rate_30_d: Option<String>,
    /// Annual interest rate for 90 day fixed loans.
    #[serde(default)]
    pub fixed_borrowing_rate_90_d: Option<String>,
    /// Annual interest rate for 180 day fixed loans.
    #[serde(default)]
    pub fixed_borrowing_rate_180_d: Option<String>,
    /// Maximum borrowing amount.
    #[serde(default)]
    pub max_borrowing_amount: Option<String>,
    /// Minimum borrowing amount.
    #[serde(default)]
    pub min_borrowing_amount: Option<String>,
}

/// Loanable coins grouped by VIP level.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VipLoanableCoins {
    /// VIP level.
    #[serde(default)]
    pub vip_level: Option<String>,
    /// Loanable coin list.
    #[serde(default)]
    pub list: Vec<LoanableCoin>,
}

/// Loanable data result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanableDataResult {
    /// Loanable coins grouped by VIP level.
    #[serde(default)]
    pub vip_coin_list: Vec<VipLoanableCoins>,
}

/// Parameters for getting the maximum collateral reduction amount.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxCollateralAmountParams {
    /// Collateral currency.
    pub currency: String,
}

impl GetMaxCollateralAmountParams {
    /// Create new parameters.
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
        }
    }
}

/// Maximum collateral amount result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxCollateralAmountResult {
    /// Maximum collateral amount that can be reduced.
    #[serde(default)]
    pub max_collateral_amount: Option<String>,
}

/// Parameters for getting the maximum loan amount.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMaxLoanAmountParams {
    /// Loan currency.
    pub currency: String,
}

impl GetMaxLoanAmountParams {
    /// Create new parameters.
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
        }
    }
}

/// Maximum loan amount result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MaxLoanAmountResult {
    /// Maximum loanable amount for the account.
    #[serde(default)]
    pub max_loan_amount: Option<String>,
    /// Remaining loanable amount on the platform.
    #[serde(default)]
    pub remaining_platform_loanable: Option<String>,
}

/// Parameters for adjusting collateral LTV.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustLtvParams {
    /// Collateral currency.
    pub currency: String,
    /// Adjustment amount.
    pub amount: String,
    /// Direction (0: add collateral, 1: reduce collateral).
    pub direction: String,
}

impl AdjustLtvParams {
    /// Create parameters to add collateral.
    pub fn add(currency: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            amount: amount.into(),
            direction: "0".to_string(),
        }
    }

    /// Create parameters to reduce collateral.
    pub fn reduce(currency: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            amount: amount.into(),
            direction: "1".to_string(),
        }
    }
}

/// LTV adjustment result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustLtvResult {
    /// Adjustment ID.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub adjust_id: Option<String>,
}

/// Parameters for getting LTV adjustment history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAdjustmentHistoryParams {
    /// Adjustment ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub adjust_id: Option<String>,
    /// Collateral currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_currency: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetAdjustmentHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set adjustment ID filter.
    pub fn adjust_id(mut self, id: impl Into<String>) -> Self {
        self.adjust_id = Some(id.into());
        self
    }

    /// Set collateral currency filter.
    pub fn collateral_currency(mut self, currency: impl Into<String>) -> Self {
        self.collateral_currency = Some(currency.into());
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

/// LTV adjustment record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LtvAdjustment {
    /// Adjustment ID.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub adjust_id: Option<String>,
    /// Collateral currency.
    #[serde(default)]
    pub collateral_currency: Option<String>,
    /// Adjustment amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// Direction (0: add, 1: reduce).
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub direction: Option<String>,
    /// LTV before adjustment.
    #[serde(default)]
    pub pre_ltv: Option<String>,
    /// LTV after adjustment.
    #[serde(default)]
    pub after_ltv: Option<String>,
    /// Adjustment status.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub status: Option<String>,
    /// Adjustment time (ms).
    #[serde(default)]
    pub adjust_time: Option<String>,
}

/// LTV adjustment history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdjustmentHistoryResult {
    /// List of adjustments.
    #[serde(default)]
    pub list: Vec<LtvAdjustment>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Collateral coin entry in the loan position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanPositionCollateral {
    /// Collateral currency.
    #[serde(default)]
    pub currency: Option<String>,
    /// Collateral amount.
    #[serde(default)]
    pub amount: Option<String>,
}

/// Borrowed coin entry in the loan position.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoanPositionBorrow {
    /// Loan currency.
    #[serde(default)]
    pub currency: Option<String>,
    /// Total debt.
    #[serde(default)]
    pub amount: Option<String>,
}

/// Crypto loan position result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CryptoLoanPositionResult {
    /// Current LTV.
    #[serde(default)]
    pub ltv: Option<String>,
    /// Total collateral value in USD.
    #[serde(default)]
    pub total_collateral_value: Option<String>,
    /// Total debt value in USD.
    #[serde(default)]
    pub total_debt_value: Option<String>,
    /// Collateral coin list.
    #[serde(default)]
    pub collateral_list: Vec<LoanPositionCollateral>,
    /// Borrowed coin list.
    #[serde(default)]
    pub borrow_list: Vec<LoanPositionBorrow>,
}

/// Parameters for borrowing a flexible loan.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleBorrowParams {
    /// Loan currency.
    pub loan_currency: String,
    /// Loan amount.
    pub loan_amount: String,
    /// Collateral coins to add with the borrow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_list: Option<Vec<CollateralCoinItem>>,
}

impl FlexibleBorrowParams {
    /// Create new parameters.
    pub fn new(loan_currency: impl Into<String>, loan_amount: impl Into<String>) -> Self {
        Self {
            loan_currency: loan_currency.into(),
            loan_amount: loan_amount.into(),
            collateral_list: None,
        }
    }

    /// Set collateral list.
    pub fn collateral_list(mut self, list: Vec<CollateralCoinItem>) -> Self {
        self.collateral_list = Some(list);
        self
    }
}

/// Flexible borrow result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleBorrowResult {
    /// Loan order ID.
    #[serde(default)]
    pub order_id: Option<String>,
}

/// Parameters for repaying a flexible loan.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleRepayParams {
    /// Loan currency.
    pub loan_currency: String,
    /// Repay amount.
    pub amount: String,
}

impl FlexibleRepayParams {
    /// Create new parameters.
    pub fn new(loan_currency: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            loan_currency: loan_currency.into(),
            amount: amount.into(),
        }
    }
}

/// Flexible repay result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleRepayResult {
    /// Repayment ID.
    #[serde(default)]
    pub repay_id: Option<String>,
}

/// Parameters for getting ongoing flexible loans.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFlexibleOngoingCoinParams {
    /// Loan currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
}

impl GetFlexibleOngoingCoinParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set loan currency filter.
    pub fn loan_currency(mut self, currency: impl Into<String>) -> Self {
        self.loan_currency = Some(currency.into());
        self
    }
}

/// Ongoing flexible loan.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleOngoingLoan {
    /// Loan currency.
    #[serde(default)]
    pub loan_currency: Option<String>,
    /// Total debt including interest.
    #[serde(default)]
    pub total_debt: Option<String>,
    /// Hourly interest rate.
    #[serde(default)]
    pub hourly_interest_rate: Option<String>,
}

/// Ongoing flexible loan list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleOngoingCoinResult {
    /// List of ongoing loans.
    #[serde(default)]
    pub list: Vec<FlexibleOngoingLoan>,
}

/// Parameters for getting flexible borrow history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFlexibleBorrowHistoryParams {
    /// Loan order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Loan currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFlexibleBorrowHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set order ID filter.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    /// Set loan currency filter.
    pub fn loan_currency(mut self, currency: impl Into<String>) -> Self {
        self.loan_currency = Some(currency.into());
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

/// Flexible borrow record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleBorrowRecord {
    /// Loan order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// Loan currency.
    #[serde(default)]
    pub loan_currency: Option<String>,
    /// Initial loan amount.
    #[serde(default)]
    pub initial_loan_amount: Option<String>,
    /// Hourly interest rate.
    #[serde(default)]
    pub hourly_interest_rate: Option<String>,
    /// Borrow time (ms).
    #[serde(default)]
    pub borrow_time: Option<String>,
    /// Loan status.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub status: Option<String>,
}

/// Flexible borrow history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleBorrowHistoryResult {
    /// List of borrow records.
    #[serde(default)]
    pub list: Vec<FlexibleBorrowRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting flexible repayment history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFlexibleRepaymentHistoryParams {
    /// Repayment ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
    /// Loan currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFlexibleRepaymentHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set repayment ID filter.
    pub fn repay_id(mut self, id: impl Into<String>) -> Self {
        self.repay_id = Some(id.into());
        self
    }

    /// Set loan currency filter.
    pub fn loan_currency(mut self, currency: impl Into<String>) -> Self {
        self.loan_currency = Some(currency.into());
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

/// Flexible repayment record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleRepayRecord {
    /// Repayment ID.
    #[serde(default)]
    pub repay_id: Option<String>,
    /// Loan currency.
    #[serde(default)]
    pub loan_currency: Option<String>,
    /// Repay amount.
    #[serde(default)]
    pub repay_amount: Option<String>,
    /// Repay type.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub repay_type: Option<String>,
    /// Repay status.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub repay_status: Option<String>,
    /// Repay time (ms).
    #[serde(default)]
    pub repay_time: Option<String>,
}

/// Flexible repayment history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlexibleRepaymentHistoryResult {
    /// List of repayment records.
    #[serde(default)]
    pub list: Vec<FlexibleRepayRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting fixed loan order quotes (borrow or supply market).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedOrderQuoteParams {
    /// Loan currency.
    pub currency: String,
    /// Loan term in days (7, 14, 30, 90, 180).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// Quote amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Annual interest rate filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_rate: Option<String>,
}

impl GetFixedOrderQuoteParams {
    /// Create new parameters.
    pub fn new(currency: impl Into<String>) -> Self {
        Self {
            currency: currency.into(),
            term: None,
            amount: None,
            annual_rate: None,
        }
    }

    /// Set loan term.
    pub fn term(mut self, term: impl Into<String>) -> Self {
        self.term = Some(term.into());
        self
    }

    /// Set amount.
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Set annual rate filter.
    pub fn annual_rate(mut self, rate: impl Into<String>) -> Self {
        self.annual_rate = Some(rate.into());
        self
    }
}

/// Fixed loan order quote entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedOrderQuote {
    /// Loan currency.
    #[serde(default)]
    pub currency: Option<String>,
    /// Loan term in days.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub term: Option<String>,
    /// Annual interest rate.
    #[serde(default)]
    pub annual_rate: Option<String>,
    /// Quote amount.
    #[serde(default)]
    pub amount: Option<String>,
}

/// Fixed loan order quote list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedOrderQuoteResult {
    /// List of quotes.
    #[serde(default)]
    pub list: Vec<FixedOrderQuote>,
}

/// Parameters for creating a fixed borrow order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedBorrowParams {
    /// Loan currency.
    pub order_currency: String,
    /// Borrow amount.
    pub order_amount: String,
    /// Annual interest rate.
    pub annual_rate: String,
    /// Loan term in days (7, 14, 30, 90, 180).
    pub term: String,
    /// Auto repay on maturity (true or false).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_repay: Option<String>,
    /// Collateral coins to add with the borrow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collateral_list: Option<Vec<CollateralCoinItem>>,
}

impl FixedBorrowParams {
    /// Create new parameters.
    pub fn new(
        order_currency: impl Into<String>,
        order_amount: impl Into<String>,
        annual_rate: impl Into<String>,
        term: impl Into<String>,
    ) -> Self {
        Self {
            order_currency: order_currency.into(),
            order_amount: order_amount.into(),
            annual_rate: annual_rate.into(),
            term: term.into(),
            auto_repay: None,
            collateral_list: None,
        }
    }

    /// Set auto repay.
    pub fn auto_repay(mut self, enabled: bool) -> Self {
        self.auto_repay = Some(enabled.to_string());
        self
    }

    /// Set collateral list.
    pub fn collateral_list(mut self, list: Vec<CollateralCoinItem>) -> Self {
        self.collateral_list = Some(list);
        self
    }
}

/// Parameters for creating a fixed supply order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedSupplyParams {
    /// Supply currency.
    pub order_currency: String,
    /// Supply amount.
    pub order_amount: String,
    /// Annual interest rate.
    pub annual_rate: String,
    /// Supply term in days (7, 14, 30, 90, 180).
    pub term: String,
}

impl FixedSupplyParams {
    /// Create new parameters.
    pub fn new(
        order_currency: impl Into<String>,
        order_amount: impl Into<String>,
        annual_rate: impl Into<String>,
        term: impl Into<String>,
    ) -> Self {
        Self {
            order_currency: order_currency.into(),
            order_amount: order_amount.into(),
            annual_rate: annual_rate.into(),
            term: term.into(),
        }
    }
}

/// Fixed loan order operation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedOrderResult {
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
}

/// Parameters for cancelling a fixed loan order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelFixedOrderParams {
    /// Order ID.
    pub order_id: String,
}

impl CancelFixedOrderParams {
    /// Create new parameters.
    pub fn new(order_id: impl Into<String>) -> Self {
        Self {
            order_id: order_id.into(),
        }
    }
}

/// Parameters for getting fixed loan order info (borrow or supply).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedOrderInfoParams {
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_currency: Option<String>,
    /// Order state filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
    /// Term filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFixedOrderInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set order ID filter.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    /// Set currency filter.
    pub fn order_currency(mut self, currency: impl Into<String>) -> Self {
        self.order_currency = Some(currency.into());
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

/// Fixed loan order information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedOrderInfo {
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// Currency.
    #[serde(default)]
    pub order_currency: Option<String>,
    /// Order amount.
    #[serde(default)]
    pub order_amount: Option<String>,
    /// Filled amount.
    #[serde(default)]
    pub filled_amount: Option<String>,
    /// Annual interest rate.
    #[serde(default)]
    pub annual_rate: Option<String>,
    /// Term in days.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub term: Option<String>,
    /// Order state.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub state: Option<String>,
    /// Order time (ms).
    #[serde(default)]
    pub order_time: Option<String>,
}

/// Fixed loan order list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedOrderInfoResult {
    /// List of orders.
    #[serde(default)]
    pub list: Vec<FixedOrderInfo>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting fixed loan contract info (borrow or supply).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedContractInfoParams {
    /// Contract (loan) ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_currency: Option<String>,
    /// Term filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFixedContractInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set loan ID filter.
    pub fn loan_id(mut self, id: impl Into<String>) -> Self {
        self.loan_id = Some(id.into());
        self
    }

    /// Set currency filter.
    pub fn order_currency(mut self, currency: impl Into<String>) -> Self {
        self.order_currency = Some(currency.into());
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

/// Fixed loan contract information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedContractInfo {
    /// Contract (loan) ID.
    #[serde(default)]
    pub loan_id: Option<String>,
    /// Currency.
    #[serde(default)]
    pub order_currency: Option<String>,
    /// Contract amount.
    #[serde(default)]
    pub loan_amount: Option<String>,
    /// Interest paid so far.
    #[serde(default)]
    pub interest_paid: Option<String>,
    /// Annual interest rate.
    #[serde(default)]
    pub annual_rate: Option<String>,
    /// Term in days.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub term: Option<String>,
    /// Contract state.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub state: Option<String>,
    /// Borrow time (ms).
    #[serde(default)]
    pub borrow_time: Option<String>,
    /// Expiration time (ms).
    #[serde(default)]
    pub expiration_time: Option<String>,
    /// Auto repay flag.
    #[serde(default)]
    pub auto_repay: Option<String>,
}

/// Fixed loan contract list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedContractInfoResult {
    /// List of contracts.
    #[serde(default)]
    pub list: Vec<FixedContractInfo>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for fully repaying fixed loans.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedFullyRepayParams {
    /// Loan currency to repay all loans for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,
    /// Specific loan IDs to repay.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_ids: Option<Vec<String>>,
}

impl FixedFullyRepayParams {
    /// Repay all fixed loans of a currency.
    pub fn by_currency(currency: impl Into<String>) -> Self {
        Self {
            currency: Some(currency.into()),
            loan_ids: None,
        }
    }

    /// Repay specific loans by ID.
    pub fn by_loan_ids(loan_ids: Vec<String>) -> Self {
        Self {
            currency: None,
            loan_ids: Some(loan_ids),
        }
    }
}

/// Fixed loan repay result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedRepayResult {
    /// Repayment ID.
    #[serde(default)]
    pub repay_id: Option<String>,
}

/// Parameters for renewing a fixed loan.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedRenewParams {
    /// Contract (loan) ID.
    pub loan_id: String,
    /// New annual interest rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annual_rate: Option<String>,
    /// New term in days.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub term: Option<String>,
}

impl FixedRenewParams {
    /// Create new parameters.
    pub fn new(loan_id: impl Into<String>) -> Self {
        Self {
            loan_id: loan_id.into(),
            annual_rate: None,
            term: None,
        }
    }

    /// Set annual rate.
    pub fn annual_rate(mut self, rate: impl Into<String>) -> Self {
        self.annual_rate = Some(rate.into());
        self
    }

    /// Set term.
    pub fn term(mut self, term: impl Into<String>) -> Self {
        self.term = Some(term.into());
        self
    }
}

/// Parameters for getting fixed loan renewal orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedRenewInfoParams {
    /// Contract (loan) ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_id: Option<String>,
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFixedRenewInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set loan ID filter.
    pub fn loan_id(mut self, id: impl Into<String>) -> Self {
        self.loan_id = Some(id.into());
        self
    }

    /// Set order ID filter.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
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

/// Fixed loan renewal record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedRenewRecord {
    /// Contract (loan) ID.
    #[serde(default)]
    pub loan_id: Option<String>,
    /// Renewal order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// Currency.
    #[serde(default)]
    pub order_currency: Option<String>,
    /// Annual interest rate.
    #[serde(default)]
    pub annual_rate: Option<String>,
    /// Term in days.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub term: Option<String>,
    /// Renewal state.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub state: Option<String>,
}

/// Fixed loan renewal list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedRenewInfoResult {
    /// List of renewal records.
    #[serde(default)]
    pub list: Vec<FixedRenewRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for repaying with collateral (flexible or fixed).
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RepayCollateralParams {
    /// Loan currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    /// Repay amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<String>,
    /// Specific loan IDs to repay (fixed loans).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_ids: Option<Vec<String>>,
}

impl RepayCollateralParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set loan currency.
    pub fn loan_currency(mut self, currency: impl Into<String>) -> Self {
        self.loan_currency = Some(currency.into());
        self
    }

    /// Set amount.
    pub fn amount(mut self, amount: impl Into<String>) -> Self {
        self.amount = Some(amount.into());
        self
    }

    /// Set loan IDs.
    pub fn loan_ids(mut self, ids: Vec<String>) -> Self {
        self.loan_ids = Some(ids);
        self
    }
}

/// Parameters for getting fixed repayment history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFixedRepaymentHistoryParams {
    /// Repayment ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub repay_id: Option<String>,
    /// Loan currency filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub loan_currency: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFixedRepaymentHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set repayment ID filter.
    pub fn repay_id(mut self, id: impl Into<String>) -> Self {
        self.repay_id = Some(id.into());
        self
    }

    /// Set loan currency filter.
    pub fn loan_currency(mut self, currency: impl Into<String>) -> Self {
        self.loan_currency = Some(currency.into());
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

/// Fixed repayment record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedRepayRecord {
    /// Repayment ID.
    #[serde(default)]
    pub repay_id: Option<String>,
    /// Loan currency.
    #[serde(default)]
    pub loan_currency: Option<String>,
    /// Repay amount.
    #[serde(default)]
    pub repay_amount: Option<String>,
    /// Repay type.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub repay_type: Option<String>,
    /// Repay status.
    #[serde(default, deserialize_with = "crate::types::common::string_or_int")]
    pub repay_status: Option<String>,
    /// Repay time (ms).
    #[serde(default)]
    pub repay_time: Option<String>,
}

/// Fixed repayment history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FixedRepaymentHistoryResult {
    /// List of repayment records.
    #[serde(default)]
    pub list: Vec<FixedRepayRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}
