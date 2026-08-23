//! Types for the pre-upgrade history endpoints.
//!
//! These endpoints return data generated before the account was upgraded
//! to a unified trading account.

use serde::{Deserialize, Serialize};

use crate::types::Category;

/// Parameters for pre-upgrade order history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeOrderHistoryParams {
    /// Product category (linear or inverse).
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Order link ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Order filter (Order or StopOrder).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// Order status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PreUpgradeOrderHistoryParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            order_id: None,
            order_link_id: None,
            order_filter: None,
            order_status: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
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

/// Parameters for pre-upgrade execution list.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeExecutionParams {
    /// Product category (linear or inverse).
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Order link ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Execution type filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exec_type: Option<String>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PreUpgradeExecutionParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            order_id: None,
            order_link_id: None,
            base_coin: None,
            start_time: None,
            end_time: None,
            exec_type: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
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

/// Parameters for pre-upgrade closed PnL.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeClosedPnlParams {
    /// Product category (linear or inverse).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
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

impl PreUpgradeClosedPnlParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
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

/// Parameters for pre-upgrade transaction log.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeTransactionLogParams {
    /// Product category (linear or option).
    pub category: Category,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Transaction type filter.
    #[serde(rename = "type", skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PreUpgradeTransactionLogParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            base_coin: None,
            transaction_type: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set base coin filter.
    pub fn base_coin(mut self, coin: impl Into<String>) -> Self {
        self.base_coin = Some(coin.into());
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

/// Parameters for pre-upgrade option delivery record.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeDeliveryRecordParams {
    /// Product category (option).
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Expiry date filter (e.g. `25MAR22`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PreUpgradeDeliveryRecordParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self {
            category: Category::Option,
            symbol: None,
            exp_date: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl Default for PreUpgradeDeliveryRecordParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Parameters for pre-upgrade USDC settlement record.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeSettlementParams {
    /// Product category (linear).
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PreUpgradeSettlementParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self {
            category: Category::Linear,
            symbol: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

impl Default for PreUpgradeSettlementParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Option delivery record entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeDeliveryRecord {
    /// Trading symbol.
    pub symbol: String,
    /// Delivery price.
    #[serde(default)]
    pub delivery_price: Option<String>,
    /// Delivery time (ms).
    #[serde(default)]
    pub delivery_time: Option<String>,
    /// Position side.
    #[serde(default)]
    pub side: Option<String>,
    /// Delivered position size.
    #[serde(default)]
    pub position: Option<String>,
    /// Strike price.
    #[serde(default)]
    pub strike: Option<String>,
    /// Delivery fee.
    #[serde(default)]
    pub fee: Option<String>,
    /// Realized PnL of the delivery.
    #[serde(default)]
    pub delivery_rpl: Option<String>,
}

/// Option delivery record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeDeliveryRecordResult {
    /// Product category.
    #[serde(default)]
    pub category: Option<String>,
    /// List of delivery records.
    pub list: Vec<PreUpgradeDeliveryRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// USDC settlement record entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeSettlementRecord {
    /// Trading symbol.
    pub symbol: String,
    /// Position side.
    #[serde(default)]
    pub side: Option<String>,
    /// Position size.
    #[serde(default)]
    pub size: Option<String>,
    /// Session average price.
    #[serde(default)]
    pub session_avg_price: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Realized PnL of the session.
    #[serde(default)]
    pub realised_pnl: Option<String>,
    /// Settlement created time (ms).
    #[serde(default)]
    pub created_time: Option<String>,
}

/// USDC settlement record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PreUpgradeSettlementResult {
    /// Product category.
    #[serde(default)]
    pub category: Option<String>,
    /// List of settlement records.
    pub list: Vec<PreUpgradeSettlementRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_history_params_serialization() {
        let params = PreUpgradeOrderHistoryParams::new(Category::Linear)
            .symbol("BTCUSDT")
            .limit(20);
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize params: {}", err),
        };
        assert!(query.contains("category=linear"));
        assert!(query.contains("symbol=BTCUSDT"));
        assert!(query.contains("limit=20"));
    }

    #[test]
    fn test_transaction_log_params_serialization() {
        let params = PreUpgradeTransactionLogParams::new(Category::Linear).base_coin("BTC");
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize params: {}", err),
        };
        assert!(query.contains("category=linear"));
        assert!(query.contains("baseCoin=BTC"));
        assert!(!query.contains("type="));
    }
}
