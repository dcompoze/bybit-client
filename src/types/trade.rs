//! Trade-related types for order management.

use serde::{Deserialize, Serialize};

use crate::types::{
    Category, OrderStatus, OrderType, PositionIdx, Side, StopOrderType, TimeInForce, TriggerBy,
};


/// Parameters for creating a new order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order side.
    pub side: Side,
    /// Order type.
    pub order_type: OrderType,
    /// Order quantity.
    pub qty: String,
    /// Whether to use leverage (spot margin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    /// Market unit for market orders (baseCoin or quoteCoin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
    /// Order price (required for limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Trigger direction for conditional orders (1=rise, 2=fall).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    /// Order filter for conditional/stop orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// Trigger price for conditional orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// Trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Implied volatility (options only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    /// Time in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Position index for hedge mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
    /// User-defined order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Take profit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Take profit trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Reduce only order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Close on trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    /// Self-match prevention type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>,
    /// Market maker protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// TP/SL mode: Full or Partial.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    /// Take profit limit price (for partial TP/SL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    /// Stop loss limit price (for partial TP/SL).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    /// Take profit order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    /// Stop loss order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
}

impl OrderParams {
    /// Create new order parameters.
    pub fn new(
        category: Category,
        symbol: impl Into<String>,
        side: Side,
        order_type: OrderType,
        qty: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            side,
            order_type,
            qty: qty.into(),
            is_leverage: None,
            market_unit: None,
            price: None,
            trigger_direction: None,
            order_filter: None,
            trigger_price: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }

    /// Create a market order.
    pub fn market(
        category: Category,
        symbol: impl Into<String>,
        side: Side,
        qty: impl Into<String>,
    ) -> Self {
        Self::new(category, symbol, side, OrderType::Market, qty)
    }

    /// Create a limit order.
    pub fn limit(
        category: Category,
        symbol: impl Into<String>,
        side: Side,
        qty: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self::new(category, symbol, side, OrderType::Limit, qty).price(price)
    }

    /// Set the price.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    /// Set time in force.
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set user-defined order ID.
    pub fn order_link_id(mut self, id: impl Into<String>) -> Self {
        self.order_link_id = Some(id.into());
        self
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

    /// Set reduce only.
    pub fn reduce_only(mut self, reduce: bool) -> Self {
        self.reduce_only = Some(reduce);
        self
    }

    /// Set position index for hedge mode.
    pub fn position_idx(mut self, idx: PositionIdx) -> Self {
        self.position_idx = Some(idx);
        self
    }

    /// Set leverage flag (spot margin).
    pub fn leverage(mut self, enabled: bool) -> Self {
        self.is_leverage = Some(if enabled { 1 } else { 0 });
        self
    }

    /// Set trigger price for conditional orders.
    pub fn trigger_price(mut self, price: impl Into<String>) -> Self {
        self.trigger_price = Some(price.into());
        self
    }

    /// Set trigger by type.
    pub fn trigger_by(mut self, by: TriggerBy) -> Self {
        self.trigger_by = Some(by);
        self
    }

    /// Set close on trigger.
    pub fn close_on_trigger(mut self, close: bool) -> Self {
        self.close_on_trigger = Some(close);
        self
    }

    /// Set TP/SL mode.
    pub fn tpsl_mode(mut self, mode: impl Into<String>) -> Self {
        self.tpsl_mode = Some(mode.into());
        self
    }
}


/// Parameters for amending an existing order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order ID (either this or order_link_id required).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Implied volatility (options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    /// New trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// New quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// New price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// TP/SL mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    /// New take profit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// New stop loss price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Take profit trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Take profit limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    /// Stop loss limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

impl AmendOrderParams {
    /// Create new amend order parameters using order ID.
    pub fn by_order_id(
        category: Category,
        symbol: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Create new amend order parameters using order link ID.
    pub fn by_order_link_id(
        category: Category,
        symbol: impl Into<String>,
        order_link_id: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            order_id: None,
            order_link_id: Some(order_link_id.into()),
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Set new quantity.
    pub fn qty(mut self, qty: impl Into<String>) -> Self {
        self.qty = Some(qty.into());
        self
    }

    /// Set new price.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    /// Set new take profit.
    pub fn take_profit(mut self, tp: impl Into<String>) -> Self {
        self.take_profit = Some(tp.into());
        self
    }

    /// Set new stop loss.
    pub fn stop_loss(mut self, sl: impl Into<String>) -> Self {
        self.stop_loss = Some(sl.into());
        self
    }

    /// Set new trigger price.
    pub fn trigger_price(mut self, price: impl Into<String>) -> Self {
        self.trigger_price = Some(price.into());
        self
    }
}


/// Parameters for canceling an order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order ID (either this or order_link_id required).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Order filter for conditional orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
}

impl CancelOrderParams {
    /// Create new cancel parameters using order ID.
    pub fn by_order_id(
        category: Category,
        symbol: impl Into<String>,
        order_id: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            order_link_id: None,
            order_filter: None,
        }
    }

    /// Create new cancel parameters using order link ID.
    pub fn by_order_link_id(
        category: Category,
        symbol: impl Into<String>,
        order_link_id: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            order_id: None,
            order_link_id: Some(order_link_id.into()),
            order_filter: None,
        }
    }

    /// Set order filter.
    pub fn order_filter(mut self, filter: impl Into<String>) -> Self {
        self.order_filter = Some(filter.into());
        self
    }
}


/// Parameters for canceling all orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllOrdersParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Settle coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settle_coin: Option<String>,
    /// Order filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// Stop order type filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_order_type: Option<StopOrderType>,
}

impl CancelAllOrdersParams {
    /// Create new cancel all parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_filter: None,
            stop_order_type: None,
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
}


/// Parameters for getting open/active orders.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenOrdersParams {
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
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Order link ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Open only filter (0=all, 1=open only, 2=conditional only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_only: Option<i32>,
    /// Order filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetOpenOrdersParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
            order_id: None,
            order_link_id: None,
            open_only: None,
            order_filter: None,
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

/// Parameters for getting order history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderHistoryParams {
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
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Order link ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Order filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_filter: Option<String>,
    /// Order status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_status: Option<OrderStatus>,
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

impl GetOrderHistoryParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            settle_coin: None,
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

    /// Set base coin filter.
    pub fn base_coin(mut self, coin: impl Into<String>) -> Self {
        self.base_coin = Some(coin.into());
        self
    }

    /// Set order status filter.
    pub fn order_status(mut self, status: OrderStatus) -> Self {
        self.order_status = Some(status);
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

/// Parameters for getting execution/trade history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetExecutionListParams {
    /// Product category.
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

impl GetExecutionListParams {
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

    /// Set order ID filter.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    /// Set base coin filter.
    pub fn base_coin(mut self, coin: impl Into<String>) -> Self {
        self.base_coin = Some(coin.into());
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

/// Parameters for spot borrow check.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetBorrowQuotaParams {
    /// Product category (must be spot).
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order side.
    pub side: Side,
}

impl GetBorrowQuotaParams {
    /// Create new parameters.
    pub fn new(symbol: impl Into<String>, side: Side) -> Self {
        Self {
            category: Category::Spot,
            symbol: symbol.into(),
            side,
        }
    }
}


/// Single order in a batch create request.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderParams {
    /// Trading symbol.
    pub symbol: String,
    /// Order side.
    pub side: Side,
    /// Order type.
    pub order_type: OrderType,
    /// Order quantity.
    pub qty: String,
    /// Whether to use leverage (spot margin only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    /// Order price (required for limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Trigger direction for conditional orders.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_direction: Option<i32>,
    /// Trigger price type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Implied volatility (options only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    /// Time in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// Position index for hedge mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<PositionIdx>,
    /// User-defined order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Take profit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Take profit trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Reduce only order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Close on trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    /// Self-match prevention type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smp_type: Option<String>,
    /// Market maker protection.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mmp: Option<bool>,
    /// TP/SL mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    /// Take profit limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    /// Stop loss limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
    /// Take profit order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_order_type: Option<OrderType>,
    /// Stop loss order type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_order_type: Option<OrderType>,
}

impl BatchOrderParams {
    /// Create new batch order parameters.
    pub fn new(
        symbol: impl Into<String>,
        side: Side,
        order_type: OrderType,
        qty: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            order_type,
            qty: qty.into(),
            is_leverage: None,
            price: None,
            trigger_direction: None,
            trigger_by: None,
            order_iv: None,
            time_in_force: None,
            position_idx: None,
            order_link_id: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            reduce_only: None,
            close_on_trigger: None,
            smp_type: None,
            mmp: None,
            tpsl_mode: None,
            tp_limit_price: None,
            sl_limit_price: None,
            tp_order_type: None,
            sl_order_type: None,
        }
    }

    /// Create a market order for batch.
    pub fn market(symbol: impl Into<String>, side: Side, qty: impl Into<String>) -> Self {
        Self::new(symbol, side, OrderType::Market, qty)
    }

    /// Create a limit order for batch.
    pub fn limit(
        symbol: impl Into<String>,
        side: Side,
        qty: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self::new(symbol, side, OrderType::Limit, qty).price(price)
    }

    /// Set the price.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }

    /// Set time in force.
    pub fn time_in_force(mut self, tif: TimeInForce) -> Self {
        self.time_in_force = Some(tif);
        self
    }

    /// Set order link ID.
    pub fn order_link_id(mut self, id: impl Into<String>) -> Self {
        self.order_link_id = Some(id.into());
        self
    }

    /// Set reduce only.
    pub fn reduce_only(mut self, reduce: bool) -> Self {
        self.reduce_only = Some(reduce);
        self
    }

    /// Set position index.
    pub fn position_idx(mut self, idx: PositionIdx) -> Self {
        self.position_idx = Some(idx);
        self
    }
}

/// Single order to amend in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchAmendOrderParams {
    /// Trading symbol.
    pub symbol: String,
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Implied volatility.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_iv: Option<String>,
    /// Trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_price: Option<String>,
    /// New quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// New price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// TP/SL mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    /// Take profit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// Take profit trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_trigger_by: Option<TriggerBy>,
    /// Stop loss trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_trigger_by: Option<TriggerBy>,
    /// Trigger type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trigger_by: Option<TriggerBy>,
    /// Take profit limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    /// Stop loss limit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

impl BatchAmendOrderParams {
    /// Create using order ID.
    pub fn by_order_id(symbol: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            order_link_id: None,
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Create using order link ID.
    pub fn by_order_link_id(symbol: impl Into<String>, order_link_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            order_link_id: Some(order_link_id.into()),
            order_iv: None,
            trigger_price: None,
            qty: None,
            price: None,
            tpsl_mode: None,
            take_profit: None,
            stop_loss: None,
            tp_trigger_by: None,
            sl_trigger_by: None,
            trigger_by: None,
            tp_limit_price: None,
            sl_limit_price: None,
        }
    }

    /// Set new quantity.
    pub fn qty(mut self, qty: impl Into<String>) -> Self {
        self.qty = Some(qty.into());
        self
    }

    /// Set new price.
    pub fn price(mut self, price: impl Into<String>) -> Self {
        self.price = Some(price.into());
        self
    }
}

/// Single order to cancel in a batch.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchCancelOrderParams {
    /// Trading symbol.
    pub symbol: String,
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User-defined order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

impl BatchCancelOrderParams {
    /// Create using order ID.
    pub fn by_order_id(symbol: impl Into<String>, order_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: Some(order_id.into()),
            order_link_id: None,
        }
    }

    /// Create using order link ID.
    pub fn by_order_link_id(symbol: impl Into<String>, order_link_id: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            order_id: None,
            order_link_id: Some(order_link_id.into()),
        }
    }
}


/// Result of order creation/amendment/cancellation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResult {
    /// Order ID assigned by the exchange.
    pub order_id: String,
    /// User-defined order ID.
    pub order_link_id: String,
}

/// Detailed order information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderInfo {
    /// Order ID.
    pub order_id: String,
    /// User-defined order ID.
    pub order_link_id: String,
    /// Block trade ID (if applicable).
    #[serde(default)]
    pub block_trade_id: Option<String>,
    /// Trading symbol.
    pub symbol: String,
    /// Order price.
    pub price: String,
    /// Order quantity.
    pub qty: String,
    /// Order side.
    pub side: Side,
    /// Whether leverage is used (spot margin).
    #[serde(default)]
    pub is_leverage: Option<String>,
    /// Position index.
    #[serde(default)]
    pub position_idx: Option<i32>,
    /// Order status.
    pub order_status: OrderStatus,
    /// Create type.
    #[serde(default)]
    pub create_type: Option<String>,
    /// Cancel type.
    #[serde(default)]
    pub cancel_type: Option<String>,
    /// Reject reason.
    #[serde(default)]
    pub reject_reason: Option<String>,
    /// Average filled price.
    #[serde(default)]
    pub avg_price: Option<String>,
    /// Remaining unfilled quantity.
    #[serde(default)]
    pub leaves_qty: Option<String>,
    /// Remaining unfilled value.
    #[serde(default)]
    pub leaves_value: Option<String>,
    /// Cumulative executed quantity.
    pub cum_exec_qty: String,
    /// Cumulative executed value.
    pub cum_exec_value: String,
    /// Cumulative executed fee.
    pub cum_exec_fee: String,
    /// Time in force.
    #[serde(default)]
    pub time_in_force: Option<TimeInForce>,
    /// Order type.
    pub order_type: OrderType,
    /// Stop order type.
    #[serde(default)]
    pub stop_order_type: Option<String>,
    /// Implied volatility.
    #[serde(default)]
    pub order_iv: Option<String>,
    /// Market unit.
    #[serde(default)]
    pub market_unit: Option<String>,
    /// Trigger price.
    #[serde(default)]
    pub trigger_price: Option<String>,
    /// Take profit price.
    #[serde(default)]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(default)]
    pub stop_loss: Option<String>,
    /// TP/SL mode.
    #[serde(default)]
    pub tpsl_mode: Option<String>,
    /// Take profit limit price.
    #[serde(default)]
    pub tp_limit_price: Option<String>,
    /// Stop loss limit price.
    #[serde(default)]
    pub sl_limit_price: Option<String>,
    /// Take profit trigger type.
    #[serde(default)]
    pub tp_trigger_by: Option<String>,
    /// Stop loss trigger type.
    #[serde(default)]
    pub sl_trigger_by: Option<String>,
    /// Trigger direction.
    #[serde(default)]
    pub trigger_direction: Option<i32>,
    /// Trigger type.
    #[serde(default)]
    pub trigger_by: Option<String>,
    /// Last price at creation.
    #[serde(default)]
    pub last_price_on_created: Option<String>,
    /// Reduce only flag.
    #[serde(default)]
    pub reduce_only: Option<bool>,
    /// Close on trigger flag.
    #[serde(default)]
    pub close_on_trigger: Option<bool>,
    /// SMP type.
    #[serde(default)]
    pub smp_type: Option<String>,
    /// SMP group.
    #[serde(default)]
    pub smp_group: Option<i32>,
    /// SMP order ID.
    #[serde(default)]
    pub smp_order_id: Option<String>,
    /// Created time (ms).
    pub created_time: String,
    /// Updated time (ms).
    pub updated_time: String,
}

/// Order list result (paginated).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderListResult {
    /// Product category.
    pub category: Category,
    /// List of orders.
    pub list: Vec<OrderInfo>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Trade execution record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Execution {
    /// Trading symbol.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// User-defined order ID.
    pub order_link_id: String,
    /// Order side.
    pub side: Side,
    /// Order price.
    pub order_price: String,
    /// Order quantity.
    pub order_qty: String,
    /// Remaining quantity.
    pub leaves_qty: String,
    /// Order type.
    pub order_type: OrderType,
    /// Stop order type.
    #[serde(default)]
    pub stop_order_type: Option<String>,
    /// Execution fee.
    pub exec_fee: String,
    /// Execution ID.
    pub exec_id: String,
    /// Execution price.
    pub exec_price: String,
    /// Execution quantity.
    pub exec_qty: String,
    /// Execution type.
    pub exec_type: String,
    /// Execution value.
    pub exec_value: String,
    /// Execution time (ms).
    pub exec_time: String,
    /// Is maker.
    pub is_maker: bool,
    /// Fee rate.
    #[serde(default)]
    pub fee_rate: Option<String>,
    /// Trade IV (options).
    #[serde(default)]
    pub trade_iv: Option<String>,
    /// Mark IV (options).
    #[serde(default)]
    pub mark_iv: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Index price.
    #[serde(default)]
    pub index_price: Option<String>,
    /// Underlying price (options).
    #[serde(default)]
    pub underlying_price: Option<String>,
    /// Block trade ID.
    #[serde(default)]
    pub block_trade_id: Option<String>,
    /// Closed size (for closed positions).
    #[serde(default)]
    pub closed_size: Option<String>,
    /// Sequence number.
    #[serde(default)]
    pub seq: Option<i64>,
}

/// Execution list result (paginated).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionListResult {
    /// Product category.
    pub category: Category,
    /// List of executions.
    pub list: Vec<Execution>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Spot borrow quota check result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BorrowQuotaResult {
    /// Trading symbol.
    pub symbol: String,
    /// Order side.
    pub side: Side,
    /// Maximum trade quantity.
    pub max_trade_qty: String,
    /// Maximum trade amount.
    pub max_trade_amount: String,
    /// Spot max trade quantity.
    #[serde(default)]
    pub spot_max_trade_qty: Option<String>,
    /// Spot max trade amount.
    #[serde(default)]
    pub spot_max_trade_amount: Option<String>,
    /// Borrow coin.
    #[serde(default)]
    pub borrow_coin: Option<String>,
}

/// Batch order result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// User-defined order ID.
    pub order_link_id: String,
    /// Created at (for batch create).
    #[serde(default)]
    pub create_at: Option<String>,
}

/// Batch operation result with error info.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOperationResult {
    /// List of successful results.
    pub list: Vec<BatchOrderResult>,
    /// Extended info with errors.
    #[serde(default)]
    pub ret_ext_info: Option<BatchRetExtInfo>,
}

/// Extended error info for batch operations.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchRetExtInfo {
    /// List of error codes and messages.
    pub list: Vec<BatchErrorInfo>,
}

/// Error info for a single item in a batch operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchErrorInfo {
    /// Error code.
    pub code: i32,
    /// Error message.
    pub msg: String,
}

/// Cancel all orders result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllResult {
    /// List of cancelled orders.
    pub list: Vec<OrderResult>,
    /// Success status (for some responses).
    #[serde(default)]
    pub success: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_order_params_market() {
        let params = OrderParams::market(Category::Linear, "BTCUSDT", Side::Buy, "0.001");
        assert_eq!(params.category, Category::Linear);
        assert_eq!(params.symbol, "BTCUSDT");
        assert_eq!(params.side, Side::Buy);
        assert_eq!(params.order_type, OrderType::Market);
        assert_eq!(params.qty, "0.001");
    }

    #[test]
    fn test_order_params_limit() {
        let params = OrderParams::limit(Category::Spot, "BTCUSDT", Side::Sell, "0.5", "50000")
            .time_in_force(TimeInForce::GTC)
            .order_link_id("my_order_123");

        assert_eq!(params.order_type, OrderType::Limit);
        assert_eq!(params.price, Some("50000".to_string()));
        assert_eq!(params.time_in_force, Some(TimeInForce::GTC));
        assert_eq!(params.order_link_id, Some("my_order_123".to_string()));
    }

    #[test]
    fn test_amend_order_params() {
        let params = AmendOrderParams::by_order_id(Category::Linear, "BTCUSDT", "order123")
            .price("51000")
            .qty("0.002");

        assert_eq!(params.order_id, Some("order123".to_string()));
        assert_eq!(params.price, Some("51000".to_string()));
        assert_eq!(params.qty, Some("0.002".to_string()));
    }

    #[test]
    fn test_cancel_order_params() {
        let params =
            CancelOrderParams::by_order_link_id(Category::Spot, "ETHUSDT", "my_order_456");

        assert_eq!(params.order_link_id, Some("my_order_456".to_string()));
        assert!(params.order_id.is_none());
    }

    #[test]
    fn test_batch_order_params() {
        let order1 = BatchOrderParams::limit("BTCUSDT", Side::Buy, "0.001", "50000")
            .order_link_id("batch_1");
        let order2 = BatchOrderParams::market("ETHUSDT", Side::Sell, "0.1")
            .reduce_only(true);

        assert_eq!(order1.price, Some("50000".to_string()));
        assert_eq!(order2.reduce_only, Some(true));
    }
}
