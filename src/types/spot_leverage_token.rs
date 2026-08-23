//! Types for the spot leverage token endpoints.

use serde::{Deserialize, Serialize};

/// Parameters for leverage token info.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageTokenInfoParams {
    /// Leverage token coin filter (e.g. "BTC3L").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_coin: Option<String>,
}

impl GetLeverageTokenInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set leverage token coin filter.
    pub fn lt_coin(mut self, coin: impl Into<String>) -> Self {
        self.lt_coin = Some(coin.into());
        self
    }
}

/// Leverage token information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenInfo {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Leverage token name.
    pub lt_name: String,
    /// Maximum purchase amount per transaction.
    pub max_purchase: String,
    /// Minimum purchase amount per transaction.
    pub min_purchase: String,
    /// Maximum purchase amount per day.
    pub max_purchase_daily: String,
    /// Maximum redemption quantity per transaction.
    pub max_redeem: String,
    /// Minimum redemption quantity per transaction.
    pub min_redeem: String,
    /// Maximum redemption quantity per day.
    pub max_redeem_daily: String,
    /// Purchase fee rate.
    pub purchase_fee_rate: String,
    /// Redemption fee rate.
    pub redeem_fee_rate: String,
    /// Whether the token can be purchased or redeemed.
    pub lt_status: String,
    /// Funding fee charged daily to the fund.
    #[serde(default)]
    pub fund_fee: Option<String>,
    /// Funding fee timestamp.
    #[serde(default)]
    pub fund_fee_time: Option<String>,
    /// Management fee rate.
    #[serde(default)]
    pub manage_fee_rate: Option<String>,
    /// Management fee timestamp.
    #[serde(default)]
    pub manage_fee_time: Option<String>,
    /// Net asset value.
    #[serde(default)]
    pub value: Option<String>,
    /// Latest net asset value.
    #[serde(default)]
    pub net_value: Option<String>,
    /// Total circulation.
    #[serde(default)]
    pub total: Option<String>,
}

/// Parameters for leverage token market reference.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageTokenReferenceParams {
    /// Leverage token coin (e.g. "BTC3L").
    pub lt_coin: String,
}

impl GetLeverageTokenReferenceParams {
    /// Create new parameters.
    pub fn new(coin: impl Into<String>) -> Self {
        Self {
            lt_coin: coin.into(),
        }
    }
}

/// Leverage token market reference.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenReference {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Net asset value.
    pub nav: String,
    /// Update time of the net asset value (ms).
    pub nav_time: String,
    /// Circulating supply in the secondary market.
    pub circulation: String,
    /// Basket.
    pub basket: String,
    /// Real leverage calculated by the last traded price.
    pub leverage: String,
}

/// Parameters for purchasing a leverage token.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLeverageTokenParams {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Purchase amount.
    pub lt_amount: String,
    /// Custom serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl PurchaseLeverageTokenParams {
    /// Create new parameters.
    pub fn new(coin: impl Into<String>, amount: impl Into<String>) -> Self {
        Self {
            lt_coin: coin.into(),
            lt_amount: amount.into(),
            serial_no: None,
        }
    }

    /// Set custom serial number.
    pub fn serial_no(mut self, serial: impl Into<String>) -> Self {
        self.serial_no = Some(serial.into());
        self
    }
}

/// Leverage token purchase result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PurchaseLeverageTokenResult {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Order status.
    pub lt_order_status: String,
    /// Executed quantity.
    #[serde(default)]
    pub exec_qty: Option<String>,
    /// Executed amount.
    #[serde(default)]
    pub exec_amt: Option<String>,
    /// Purchase amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// Order ID.
    pub purchase_id: String,
    /// Serial number.
    #[serde(default)]
    pub serial_no: Option<String>,
    /// Quote coin.
    #[serde(default)]
    pub value_coin: Option<String>,
}

/// Parameters for redeeming a leverage token.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLeverageTokenParams {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Redemption quantity.
    pub quantity: String,
    /// Custom serial number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl RedeemLeverageTokenParams {
    /// Create new parameters.
    pub fn new(coin: impl Into<String>, quantity: impl Into<String>) -> Self {
        Self {
            lt_coin: coin.into(),
            quantity: quantity.into(),
            serial_no: None,
        }
    }

    /// Set custom serial number.
    pub fn serial_no(mut self, serial: impl Into<String>) -> Self {
        self.serial_no = Some(serial.into());
        self
    }
}

/// Leverage token redemption result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RedeemLeverageTokenResult {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Order status.
    pub lt_order_status: String,
    /// Redemption quantity.
    #[serde(default)]
    pub quantity: Option<String>,
    /// Executed quantity.
    #[serde(default)]
    pub exec_qty: Option<String>,
    /// Executed amount.
    #[serde(default)]
    pub exec_amt: Option<String>,
    /// Order ID.
    pub redeem_id: String,
    /// Serial number.
    #[serde(default)]
    pub serial_no: Option<String>,
    /// Quote coin.
    #[serde(default)]
    pub value_coin: Option<String>,
}

/// Parameters for leverage token order records.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLeverageTokenOrderRecordParams {
    /// Leverage token coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_coin: Option<String>,
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Order type (1: purchase, 2: redeem).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lt_order_type: Option<i32>,
    /// Serial number filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub serial_no: Option<String>,
}

impl GetLeverageTokenOrderRecordParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set leverage token coin filter.
    pub fn lt_coin(mut self, coin: impl Into<String>) -> Self {
        self.lt_coin = Some(coin.into());
        self
    }

    /// Set order ID filter.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    /// Set order type filter (1: purchase, 2: redeem).
    pub fn lt_order_type(mut self, order_type: i32) -> Self {
        self.lt_order_type = Some(order_type);
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Leverage token order record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageTokenOrderRecord {
    /// Leverage token coin.
    pub lt_coin: String,
    /// Order ID.
    pub order_id: String,
    /// Order type (1: purchase, 2: redeem).
    pub lt_order_type: i32,
    /// Order time (ms).
    pub order_time: i64,
    /// Update time (ms).
    pub update_time: i64,
    /// Order status.
    pub lt_order_status: String,
    /// Trading fees.
    #[serde(default)]
    pub fee: Option<String>,
    /// Order quantity.
    #[serde(default)]
    pub amount: Option<String>,
    /// Filled value.
    #[serde(default)]
    pub value: Option<String>,
    /// Quote coin.
    #[serde(default)]
    pub value_coin: Option<String>,
    /// Serial number.
    #[serde(default)]
    pub serial_no: Option<String>,
}
