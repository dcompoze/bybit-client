//! Types for the spread trading endpoints.

use serde::{Deserialize, Serialize};

use crate::types::enums::{OrderType, Side};

/// Parameters for getting spread instruments info.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadInstrumentsInfoParams {
    /// Spread combination symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Limit (max 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSpreadInstrumentsInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
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

/// Leg of a spread combination.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadLeg {
    /// Leg symbol.
    pub symbol: String,
    /// Leg contract type (LinearPerpetual, LinearFutures, Spot).
    #[serde(default)]
    pub contract_type: Option<String>,
}

/// Spread instrument information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadInstrument {
    /// Spread combination symbol.
    pub symbol: String,
    /// Spread contract type (FundingRateArb, CarryTrade, FutureSpread, PerpBasis).
    #[serde(default)]
    pub contract_type: Option<String>,
    /// Instrument status (Trading, Settling).
    #[serde(default)]
    pub status: Option<String>,
    /// Base coin.
    #[serde(default)]
    pub base_coin: Option<String>,
    /// Quote coin.
    #[serde(default)]
    pub quote_coin: Option<String>,
    /// Settlement coin.
    #[serde(default)]
    pub settle_coin: Option<String>,
    /// Price tick size.
    #[serde(default)]
    pub tick_size: Option<String>,
    /// Minimum price.
    #[serde(default)]
    pub min_price: Option<String>,
    /// Maximum price.
    #[serde(default)]
    pub max_price: Option<String>,
    /// Lot size.
    #[serde(default)]
    pub lot_size: Option<String>,
    /// Minimum order size.
    #[serde(default)]
    pub min_size: Option<String>,
    /// Maximum order size.
    #[serde(default)]
    pub max_size: Option<String>,
    /// Launch time (ms).
    #[serde(default)]
    pub launch_time: Option<String>,
    /// Delivery time (ms).
    #[serde(default)]
    pub delivery_time: Option<String>,
    /// Legs of the combination.
    #[serde(default)]
    pub legs: Vec<SpreadLeg>,
}

/// Spread instrument list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadInstrumentsInfoResult {
    /// List of instruments.
    pub list: Vec<SpreadInstrument>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting the spread orderbook.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadOrderbookParams {
    /// Spread combination symbol.
    pub symbol: String,
    /// Depth limit (max 25, default 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetSpreadOrderbookParams {
    /// Create new parameters.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            limit: None,
        }
    }

    /// Set depth limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for getting spread tickers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadTickersParams {
    /// Spread combination symbol.
    pub symbol: String,
}

impl GetSpreadTickersParams {
    /// Create new parameters.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
        }
    }
}

/// Spread ticker data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadTicker {
    /// Spread combination symbol.
    pub symbol: String,
    /// Best bid price.
    #[serde(default)]
    pub bid_price: Option<String>,
    /// Best bid size.
    #[serde(default)]
    pub bid_size: Option<String>,
    /// Best ask price.
    #[serde(default)]
    pub ask_price: Option<String>,
    /// Best ask size.
    #[serde(default)]
    pub ask_size: Option<String>,
    /// Last trade price.
    #[serde(default)]
    pub last_price: Option<String>,
    /// Highest price in 24h.
    #[serde(default)]
    pub high_price_24h: Option<String>,
    /// Lowest price in 24h.
    #[serde(default)]
    pub low_price_24h: Option<String>,
    /// Price 24 hours ago.
    #[serde(default)]
    pub prev_price_24h: Option<String>,
    /// 24h volume.
    #[serde(default)]
    pub volume_24h: Option<String>,
}

/// Spread ticker list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadTickersResult {
    /// List of tickers.
    pub list: Vec<SpreadTicker>,
}

/// Parameters for getting spread recent trades.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadRecentTradesParams {
    /// Spread combination symbol.
    pub symbol: String,
    /// Limit (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetSpreadRecentTradesParams {
    /// Create new parameters.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            limit: None,
        }
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Spread public trade.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadRecentTrade {
    /// Execution ID.
    #[serde(default)]
    pub exec_id: Option<String>,
    /// Spread combination symbol.
    pub symbol: String,
    /// Trade price.
    pub price: String,
    /// Trade size.
    pub size: String,
    /// Side of the taker.
    pub side: Side,
    /// Trade time (ms).
    #[serde(default)]
    pub time: Option<String>,
}

/// Spread recent trade list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadRecentTradesResult {
    /// List of trades.
    pub list: Vec<SpreadRecentTrade>,
}

/// Parameters for placing a spread order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PlaceSpreadOrderParams {
    /// Spread combination symbol.
    pub symbol: String,
    /// Order side.
    pub side: Side,
    /// Order type.
    pub order_type: OrderType,
    /// Order quantity.
    pub qty: String,
    /// Order price (required for limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// User custom order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Time in force (GTC, FOK, IOC, PostOnly).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
}

impl PlaceSpreadOrderParams {
    /// Create a limit order.
    pub fn limit(
        symbol: impl Into<String>,
        side: Side,
        qty: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            order_type: OrderType::Limit,
            qty: qty.into(),
            price: Some(price.into()),
            order_link_id: None,
            time_in_force: None,
        }
    }

    /// Create a market order.
    pub fn market(symbol: impl Into<String>, side: Side, qty: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            side,
            order_type: OrderType::Market,
            qty: qty.into(),
            price: None,
            order_link_id: None,
            time_in_force: None,
        }
    }

    /// Set user custom order ID.
    pub fn order_link_id(mut self, id: impl Into<String>) -> Self {
        self.order_link_id = Some(id.into());
        self
    }

    /// Set time in force.
    pub fn time_in_force(mut self, tif: impl Into<String>) -> Self {
        self.time_in_force = Some(tif.into());
        self
    }
}

/// Spread order operation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrderResult {
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
}

/// Parameters for amending a spread order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendSpreadOrderParams {
    /// Spread combination symbol.
    pub symbol: String,
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User custom order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// New quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// New price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
}

impl AmendSpreadOrderParams {
    /// Create new parameters.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            ..Self::default()
        }
    }

    /// Set order ID.
    pub fn order_id(mut self, id: impl Into<String>) -> Self {
        self.order_id = Some(id.into());
        self
    }

    /// Set user custom order ID.
    pub fn order_link_id(mut self, id: impl Into<String>) -> Self {
        self.order_link_id = Some(id.into());
        self
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

/// Parameters for cancelling a spread order.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelSpreadOrderParams {
    /// Order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User custom order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}

impl CancelSpreadOrderParams {
    /// Cancel by order ID.
    pub fn by_order_id(id: impl Into<String>) -> Self {
        Self {
            order_id: Some(id.into()),
            order_link_id: None,
        }
    }

    /// Cancel by user custom order ID.
    pub fn by_order_link_id(id: impl Into<String>) -> Self {
        Self {
            order_id: None,
            order_link_id: Some(id.into()),
        }
    }
}

/// Parameters for cancelling all spread orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllSpreadOrdersParams {
    /// Spread combination symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Cancel all orders regardless of symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_all: Option<bool>,
}

impl CancelAllSpreadOrdersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Cancel every open spread order.
    pub fn cancel_all(mut self) -> Self {
        self.cancel_all = Some(true);
        self
    }
}

/// Result of cancelling all spread orders.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelAllSpreadOrdersResult {
    /// List of cancelled orders.
    #[serde(default)]
    pub list: Vec<SpreadOrderResult>,
}

/// Parameters for getting spread open orders.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadOpenOrdersParams {
    /// Spread combination symbol.
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
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSpreadOpenOrdersParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
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

/// Parameters for getting spread order history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadOrderHistoryParams {
    /// Spread combination symbol.
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

impl GetSpreadOrderHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
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

/// Spread order information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrder {
    /// Spread combination symbol.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Order side.
    #[serde(default)]
    pub side: Option<String>,
    /// Order type.
    #[serde(default)]
    pub order_type: Option<String>,
    /// Order status.
    #[serde(default)]
    pub order_status: Option<String>,
    /// Order price.
    #[serde(default)]
    pub price: Option<String>,
    /// Order quantity.
    #[serde(default)]
    pub qty: Option<String>,
    /// Average fill price.
    #[serde(default)]
    pub avg_price: Option<String>,
    /// Remaining quantity.
    #[serde(default)]
    pub leaves_qty: Option<String>,
    /// Cumulative executed quantity.
    #[serde(default)]
    pub cum_exec_qty: Option<String>,
    /// Time in force.
    #[serde(default)]
    pub time_in_force: Option<String>,
    /// Base coin.
    #[serde(default)]
    pub base_coin: Option<String>,
    /// Contract type.
    #[serde(default)]
    pub contract_type: Option<String>,
    /// Cancel type.
    #[serde(default)]
    pub cancel_type: Option<String>,
    /// Reject reason.
    #[serde(default)]
    pub reject_reason: Option<String>,
    /// Created time (ms).
    #[serde(default)]
    pub created_time: Option<String>,
    /// Updated time (ms).
    #[serde(default)]
    pub updated_time: Option<String>,
}

/// Spread order list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadOrderListResult {
    /// List of orders.
    pub list: Vec<SpreadOrder>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for getting spread executions.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSpreadExecutionListParams {
    /// Spread combination symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Order ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// Order link ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
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

impl GetSpreadExecutionListParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
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

/// Leg fill of a spread execution.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadExecutionLeg {
    /// Leg symbol.
    #[serde(default)]
    pub symbol: Option<String>,
    /// Leg category.
    #[serde(default)]
    pub category: Option<String>,
    /// Leg side.
    #[serde(default)]
    pub side: Option<String>,
    /// Execution price.
    #[serde(default)]
    pub exec_price: Option<String>,
    /// Execution quantity.
    #[serde(default)]
    pub exec_qty: Option<String>,
    /// Execution fee.
    #[serde(default)]
    pub exec_fee: Option<String>,
    /// Execution ID.
    #[serde(default)]
    pub exec_id: Option<String>,
    /// Execution time (ms).
    #[serde(default)]
    pub exec_time: Option<String>,
    /// Execution type.
    #[serde(default)]
    pub exec_type: Option<String>,
}

/// Spread execution record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadExecution {
    /// Spread combination symbol.
    pub symbol: String,
    /// Order ID.
    #[serde(default)]
    pub order_id: Option<String>,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Order side.
    #[serde(default)]
    pub side: Option<String>,
    /// Execution ID.
    #[serde(default)]
    pub exec_id: Option<String>,
    /// Execution price.
    #[serde(default)]
    pub exec_price: Option<String>,
    /// Execution quantity.
    #[serde(default)]
    pub exec_qty: Option<String>,
    /// Execution type.
    #[serde(default)]
    pub exec_type: Option<String>,
    /// Execution time (ms).
    #[serde(default)]
    pub exec_time: Option<String>,
    /// Leg fills.
    #[serde(default)]
    pub legs: Vec<SpreadExecutionLeg>,
}

/// Spread execution list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpreadExecutionListResult {
    /// List of executions.
    pub list: Vec<SpreadExecution>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}
