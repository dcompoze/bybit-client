//! Position-related types for position management.

use serde::{Deserialize, Serialize};

use crate::types::{Category, OrderType, PositionIdx, Side, TriggerBy};


/// Parameters for getting position info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPositionInfoParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Settle coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetPositionInfoParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            limit: None,
            cursor: None,
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

    /// Set settle coin filter.
    pub fn settle_coin(mut self, coin: impl Into<String>) -> Self {
        self.settle_coin = Some(coin.into());
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

/// Parameters for setting leverage.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetLeverageParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Buy leverage (must be same as sell for one-way mode).
    pub buy_leverage: String,
    /// Sell leverage (must be same as buy for one-way mode).
    pub sell_leverage: String,
}

impl SetLeverageParams {
    /// Create new parameters.
    pub fn new(
        category: Category,
        symbol: impl Into<String>,
        buy_leverage: impl Into<String>,
        sell_leverage: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            buy_leverage: buy_leverage.into(),
            sell_leverage: sell_leverage.into(),
        }
    }

    /// Create parameters with same leverage for buy and sell (one-way mode).
    pub fn uniform(category: Category, symbol: impl Into<String>, leverage: impl Into<String>) -> Self {
        let lev = leverage.into();
        Self::new(category, symbol, lev.clone(), lev)
    }
}

/// Parameters for switching isolated margin mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchMarginModeParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Trade mode (0=cross margin, 1=isolated margin).
    pub trade_mode: i32,
    /// Buy leverage.
    pub buy_leverage: String,
    /// Sell leverage.
    pub sell_leverage: String,
}

impl SwitchMarginModeParams {
    /// Create parameters for cross margin mode.
    pub fn cross_margin(
        category: Category,
        symbol: impl Into<String>,
        buy_leverage: impl Into<String>,
        sell_leverage: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            trade_mode: 0,
            buy_leverage: buy_leverage.into(),
            sell_leverage: sell_leverage.into(),
        }
    }

    /// Create parameters for isolated margin mode.
    pub fn isolated_margin(
        category: Category,
        symbol: impl Into<String>,
        buy_leverage: impl Into<String>,
        sell_leverage: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            trade_mode: 1,
            buy_leverage: buy_leverage.into(),
            sell_leverage: sell_leverage.into(),
        }
    }
}

/// Parameters for switching position mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SwitchPositionModeParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol (for linear, required if coin is not set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Coin (for inverse, required if symbol is not set).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Position mode (0=one-way, 3=hedge).
    pub mode: i32,
}

impl SwitchPositionModeParams {
    /// Create parameters for one-way mode with symbol.
    pub fn one_way_by_symbol(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: Some(symbol.into()),
            coin: None,
            mode: 0,
        }
    }

    /// Create parameters for hedge mode with symbol.
    pub fn hedge_by_symbol(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: Some(symbol.into()),
            coin: None,
            mode: 3,
        }
    }

    /// Create parameters for one-way mode with coin.
    pub fn one_way_by_coin(category: Category, coin: impl Into<String>) -> Self {
        Self {
            category,
            symbol: None,
            coin: Some(coin.into()),
            mode: 0,
        }
    }

    /// Create parameters for hedge mode with coin.
    pub fn hedge_by_coin(category: Category, coin: impl Into<String>) -> Self {
        Self {
            category,
            symbol: None,
            coin: Some(coin.into()),
            mode: 3,
        }
    }
}

/// Parameters for setting trading stop (TP/SL/trailing stop).
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTradingStopParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Position index.
    pub position_idx: PositionIdx,
    /// Take profit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Trailing stop distance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trailing_stop: Option<String>,
    /// Take profit trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Active price for trailing stop.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_price: Option<String>,
    /// TP/SL mode (Full or Partial).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    /// TP size for partial mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_size: Option<String>,
    /// SL size for partial mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_size: Option<String>,
    /// TP limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    /// SL limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    /// TP order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    /// SL order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
}

impl SetTradingStopParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>, position_idx: PositionIdx) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            position_idx,
            take_profit: None,
            stop_loss: None,
            trailing_stop: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            active_price: None,
            tpsl_mode: None,
            tp_size: None,
            sl_size: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }

    /// Set take profit price.
    pub fn take_profit(mut self, tp: impl Into<String>) -> Self {
        self.take_profit = Some(tp.into());
        self
    }

    /// Set stop loss price.
    pub fn stop_loss(mut self, sl: impl Into<String>) -> Self {
        self.stop_loss = Some(sl.into());
        self
    }

    /// Set trailing stop distance.
    pub fn trailing_stop(mut self, ts: impl Into<String>) -> Self {
        self.trailing_stop = Some(ts.into());
        self
    }

    /// Set take profit trigger type.
    pub fn tp_trigger_by(mut self, by: TriggerBy) -> Self {
        self.tp_trigger_by = Some(by);
        self
    }

    /// Set stop loss trigger type.
    pub fn sl_trigger_by(mut self, by: TriggerBy) -> Self {
        self.sl_trigger_by = Some(by);
        self
    }

    /// Set TP/SL mode.
    pub fn tpsl_mode(mut self, mode: impl Into<String>) -> Self {
        self.tpsl_mode = Some(mode.into());
        self
    }
}

/// Parameters for setting auto add margin.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetAutoAddMarginParams {
    /// Product category (must be linear).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Auto add margin (0=off, 1=on).
    pub auto_add_margin: i32,
    /// Position index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl SetAutoAddMarginParams {
    /// Create parameters to enable auto add margin.
    pub fn enable(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            auto_add_margin: 1,
            position_idx: None,
        }
    }

    /// Create parameters to disable auto add margin.
    pub fn disable(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            auto_add_margin: 0,
            position_idx: None,
        }
    }

    /// Set position index.
    pub fn position_idx(mut self, idx: PositionIdx) -> Self {
        self.position_idx = Some(idx);
        self
    }
}

/// Parameters for adding or reducing margin.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AddReduceMarginParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Margin amount (positive to add, negative to reduce).
    pub margin: String,
    /// Position index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl AddReduceMarginParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>, margin: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            margin: margin.into(),
            position_idx: None,
        }
    }

    /// Set position index.
    pub fn position_idx(mut self, idx: PositionIdx) -> Self {
        self.position_idx = Some(idx);
        self
    }
}

/// Parameters for getting closed PnL.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedPnlParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
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

impl GetClosedPnlParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
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


/// Position information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionInfo {
    /// Position index.
    pub position_idx: i32,
    /// Risk ID.
    #[serde(default)]
    pub risk_id: Option<i32>,
    /// Risk limit value.
    #[serde(default)]
    pub risk_limit_value: Option<String>,
    /// Trading symbol.
    pub symbol: String,
    /// Position side (Buy, Sell, None).
    pub side: String,
    /// Position size.
    pub size: String,
    /// Average entry price.
    pub avg_price: String,
    /// Position value.
    pub position_value: String,
    /// Trade mode (0=cross, 1=isolated).
    pub trade_mode: i32,
    /// Auto add margin flag.
    #[serde(default)]
    pub auto_add_margin: Option<i32>,
    /// Position status.
    #[serde(default)]
    pub position_status: Option<String>,
    /// Leverage.
    pub leverage: String,
    /// Mark price.
    pub mark_price: String,
    /// Liquidation price.
    #[serde(default)]
    pub liq_price: Option<String>,
    /// Bankruptcy price.
    #[serde(default)]
    pub bust_price: Option<String>,
    /// Position initial margin.
    #[serde(default)]
    pub position_i_m: Option<String>,
    /// Position maintenance margin.
    #[serde(default)]
    pub position_m_m: Option<String>,
    /// Position balance.
    #[serde(default)]
    pub position_balance: Option<String>,
    /// TP/SL mode.
    #[serde(default)]
    pub tpsl_mode: Option<String>,
    /// Take profit price.
    #[serde(default)]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(default)]
    pub stop_loss: Option<String>,
    /// Trailing stop price.
    #[serde(default)]
    pub trailing_stop: Option<String>,
    /// Session average price.
    #[serde(default)]
    pub session_avg_price: Option<String>,
    /// Delta (options).
    #[serde(default)]
    pub delta: Option<String>,
    /// Gamma (options).
    #[serde(default)]
    pub gamma: Option<String>,
    /// Vega (options).
    #[serde(default)]
    pub vega: Option<String>,
    /// Theta (options).
    #[serde(default)]
    pub theta: Option<String>,
    /// Unrealised PnL.
    pub unrealised_pnl: String,
    /// Current session realised PnL.
    #[serde(default)]
    pub cur_realised_pnl: Option<String>,
    /// Cumulative realised PnL.
    pub cum_realised_pnl: String,
    /// ADL rank indicator.
    #[serde(default)]
    pub adl_rank_indicator: Option<i32>,
    /// Is reduce only.
    #[serde(default)]
    pub is_reduce_only: Option<bool>,
    /// MMR system updated time.
    #[serde(default)]
    pub mmr_sys_updated_time: Option<String>,
    /// Leverage system updated time.
    #[serde(default)]
    pub leverage_sys_updated_time: Option<String>,
    /// Created time (ms).
    pub created_time: String,
    /// Updated time (ms).
    pub updated_time: String,
    /// Sequence number.
    #[serde(default)]
    pub seq: Option<i64>,
    /// Break-even price (linear and inverse).
    #[serde(default)]
    pub break_even_price: Option<String>,
    /// Position open time (ms).
    #[serde(default)]
    pub open_time: Option<String>,
}

/// Position list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionListResult {
    /// Product category.
    pub category: Category,
    /// List of positions.
    pub list: Vec<PositionInfo>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Closed PnL record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnl {
    /// Trading symbol.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// Side.
    pub side: String,
    /// Quantity.
    pub qty: String,
    /// Order price.
    pub order_price: String,
    /// Order type.
    pub order_type: String,
    /// Execution type.
    pub exec_type: String,
    /// Closed size.
    pub closed_size: String,
    /// Opening fee.
    #[serde(default)]
    pub open_fee: Option<String>,
    /// Closing fee.
    #[serde(default)]
    pub close_fee: Option<String>,
    /// Cumulative entry value.
    pub cum_entry_value: String,
    /// Average entry price.
    pub avg_entry_price: String,
    /// Cumulative exit value.
    pub cum_exit_value: String,
    /// Average exit price.
    pub avg_exit_price: String,
    /// Closed PnL.
    pub closed_pnl: String,
    /// Fill count.
    #[serde(default)]
    pub fill_count: Option<String>,
    /// Leverage.
    pub leverage: String,
    /// Created time (ms).
    pub created_time: String,
    /// Updated time (ms).
    pub updated_time: String,
}

/// Closed PnL list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedPnlListResult {
    /// Product category.
    pub category: Category,
    /// List of closed PnL records.
    pub list: Vec<ClosedPnl>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Margin operation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MarginOperationResult {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Position index.
    pub position_idx: i32,
    /// Risk ID.
    #[serde(default)]
    pub risk_id: Option<i32>,
    /// Risk limit value.
    #[serde(default)]
    pub risk_limit_value: Option<String>,
    /// Position size.
    pub size: String,
    /// Average price.
    #[serde(default)]
    pub avg_price: Option<String>,
    /// Liquidation price.
    #[serde(default)]
    pub liq_price: Option<String>,
    /// Bankruptcy price.
    #[serde(default)]
    pub bust_price: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Position value.
    #[serde(default)]
    pub position_value: Option<String>,
    /// Leverage.
    pub leverage: String,
    /// Auto add margin flag.
    #[serde(default)]
    pub auto_add_margin: Option<i32>,
    /// Position status.
    #[serde(default)]
    pub position_status: Option<String>,
    /// Position initial margin.
    #[serde(default)]
    pub position_i_m: Option<String>,
    /// Position maintenance margin.
    #[serde(default)]
    pub position_m_m: Option<String>,
    /// Take profit price.
    #[serde(default)]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(default)]
    pub stop_loss: Option<String>,
    /// Trailing stop.
    #[serde(default)]
    pub trailing_stop: Option<String>,
    /// Unrealised PnL.
    #[serde(default)]
    pub unrealised_pnl: Option<String>,
    /// Cumulative realised PnL.
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    /// Created time.
    #[serde(default)]
    pub created_time: Option<String>,
    /// Updated time.
    #[serde(default)]
    pub updated_time: Option<String>,
}

/// Parameters for setting TP/SL mode.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTpslModeParams {
    /// Product category (linear, inverse).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// TP/SL mode (Full, Partial).
    pub tp_sl_mode: String,
}

impl SetTpslModeParams {
    /// Create parameters for full TP/SL mode.
    pub fn full(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            tp_sl_mode: "Full".to_string(),
        }
    }

    /// Create parameters for partial TP/SL mode.
    pub fn partial(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            tp_sl_mode: "Partial".to_string(),
        }
    }
}

/// Result of setting TP/SL mode.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetTpslModeResult {
    /// TP/SL mode after the change.
    pub tp_sl_mode: String,
}

/// Parameters for setting the risk limit.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitParams {
    /// Product category (linear, inverse).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Risk ID.
    pub risk_id: i32,
    /// Position index.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
}

impl SetRiskLimitParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>, risk_id: i32) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            risk_id,
            position_idx: None,
        }
    }

    /// Set position index.
    pub fn position_idx(mut self, idx: PositionIdx) -> Self {
        self.position_idx = Some(idx);
        self
    }
}

/// Result of setting the risk limit.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetRiskLimitResult {
    /// Product category.
    pub category: Category,
    /// Risk ID.
    pub risk_id: i32,
    /// Risk limit value.
    pub risk_limit_value: String,
}

/// Single position entry for a move-positions request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionItem {
    /// Product category (linear, spot, option, inverse).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Trade price.
    pub price: String,
    /// Side from the taker (toUid) perspective.
    pub side: Side,
    /// Quantity.
    pub qty: String,
}

impl MovePositionItem {
    /// Create a new move position item.
    pub fn new(
        category: Category,
        symbol: impl Into<String>,
        price: impl Into<String>,
        side: Side,
        qty: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            price: price.into(),
            side,
            qty: qty.into(),
        }
    }
}

/// Parameters for moving positions between UIDs.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsParams {
    /// Source UID.
    pub from_uid: String,
    /// Destination UID.
    pub to_uid: String,
    /// Positions to move (max 25).
    pub list: Vec<MovePositionItem>,
}

impl MovePositionsParams {
    /// Create new parameters.
    pub fn new(
        from_uid: impl Into<String>,
        to_uid: impl Into<String>,
        list: Vec<MovePositionItem>,
    ) -> Self {
        Self {
            from_uid: from_uid.into(),
            to_uid: to_uid.into(),
            list,
        }
    }
}

/// Result of a move-positions request.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionsResult {
    /// Block trade ID.
    pub block_trade_id: String,
    /// Status (Processing, Rejected).
    pub status: String,
    /// Rejecting party (empty on success).
    #[serde(default)]
    pub reject_party: Option<String>,
}

/// Parameters for querying move position history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMovePositionHistoryParams {
    /// Product category filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub category: Option<Category>,
    /// Trading symbol filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Status filter (Processing, Filled, Rejected).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Block trade ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub block_trade_id: Option<String>,
    /// Limit per page.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetMovePositionHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set category filter.
    pub fn category(mut self, category: Category) -> Self {
        self.category = Some(category);
        self
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set status filter.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set block trade ID filter.
    pub fn block_trade_id(mut self, id: impl Into<String>) -> Self {
        self.block_trade_id = Some(id.into());
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

/// Move position history entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionHistoryEntry {
    /// Block trade ID.
    pub block_trade_id: String,
    /// Product category.
    pub category: String,
    /// Order ID.
    pub order_id: String,
    /// User ID.
    pub user_id: i64,
    /// Trading symbol.
    pub symbol: String,
    /// Side.
    pub side: String,
    /// Trade price.
    pub price: String,
    /// Quantity.
    pub qty: String,
    /// Execution fee.
    #[serde(default)]
    pub exec_fee: Option<String>,
    /// Status (Processing, Filled, Rejected).
    pub status: String,
    /// Execution ID.
    #[serde(default)]
    pub exec_id: Option<String>,
    /// Result code (0 means success).
    #[serde(default)]
    pub result_code: Option<i32>,
    /// Result message.
    #[serde(default)]
    pub result_message: Option<String>,
    /// Created time (ms).
    pub created_at: i64,
    /// Updated time (ms).
    pub updated_at: i64,
    /// Rejecting party (empty on success).
    #[serde(default)]
    pub reject_party: Option<String>,
}

/// Move position history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MovePositionHistoryResult {
    /// List of moved positions.
    pub list: Vec<MovePositionHistoryEntry>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for querying closed options positions.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetClosedOptionsPositionsParams {
    /// Product category (must be option).
    pub category: Category,
    /// Trading symbol filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit per page (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetClosedOptionsPositionsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self {
            category: Category::Option,
            symbol: None,
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

impl Default for GetClosedOptionsPositionsParams {
    fn default() -> Self {
        Self::new()
    }
}

/// Closed options position record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedOptionsPosition {
    /// Trading symbol.
    pub symbol: String,
    /// Side.
    pub side: String,
    /// Total opening fee.
    #[serde(default)]
    pub total_open_fee: Option<String>,
    /// Delivery fee.
    #[serde(default)]
    pub delivery_fee: Option<String>,
    /// Total closing fee.
    #[serde(default)]
    pub total_close_fee: Option<String>,
    /// Quantity.
    pub qty: String,
    /// Close time (ms).
    pub close_time: i64,
    /// Average exit price.
    #[serde(default)]
    pub avg_exit_price: Option<String>,
    /// Delivery price.
    #[serde(default)]
    pub delivery_price: Option<String>,
    /// Open time (ms).
    pub open_time: i64,
    /// Average entry price.
    #[serde(default)]
    pub avg_entry_price: Option<String>,
    /// Total PnL.
    #[serde(default)]
    pub total_pnl: Option<String>,
}

/// Closed options positions result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClosedOptionsPositionsResult {
    /// Product category.
    pub category: String,
    /// List of closed options positions.
    pub list: Vec<ClosedOptionsPosition>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for confirming the pending MMR.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmPendingMmrParams {
    /// Product category (linear, inverse).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
}

impl ConfirmPendingMmrParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_position_info_params() {
        let params = GetPositionInfoParams::new(Category::Linear)
            .symbol("BTCUSDT")
            .limit(10);

        assert_eq!(params.category, Category::Linear);
        assert_eq!(params.symbol, Some("BTCUSDT".to_string()));
        assert_eq!(params.limit, Some(10));
    }

    #[test]
    fn test_set_leverage_params() {
        let params = SetLeverageParams::uniform(Category::Linear, "BTCUSDT", "10");
        assert_eq!(params.buy_leverage, "10");
        assert_eq!(params.sell_leverage, "10");
    }

    #[test]
    fn test_switch_position_mode_params() {
        let params = SwitchPositionModeParams::hedge_by_symbol(Category::Linear, "BTCUSDT");
        assert_eq!(params.mode, 3);
        assert_eq!(params.symbol, Some("BTCUSDT".to_string()));
    }

    #[test]
    fn test_trading_stop_params() {
        let params = SetTradingStopParams::new(Category::Linear, "BTCUSDT", PositionIdx::OneWay)
            .take_profit("55000")
            .stop_loss("45000");

        assert_eq!(params.take_profit, Some("55000".to_string()));
        assert_eq!(params.stop_loss, Some("45000".to_string()));
    }
}
