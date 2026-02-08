//! WebSocket message types.

use serde::{Deserialize, Serialize};

use crate::types::Category;


/// WebSocket operation message (subscribe, unsubscribe, ping, auth).
#[derive(Debug, Clone, Serialize)]
pub struct WsOperation {
    /// Operation type.
    pub op: String,
    /// Operation arguments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub args: Option<Vec<String>>,
    /// Request ID (for tracking responses).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub req_id: Option<String>,
}

impl WsOperation {
    /// Create a ping message.
    pub fn ping() -> Self {
        Self {
            op: "ping".to_string(),
            args: None,
            req_id: None,
        }
    }

    /// Create a subscribe message.
    pub fn subscribe(topics: Vec<String>) -> Self {
        Self {
            op: "subscribe".to_string(),
            args: Some(topics),
            req_id: None,
        }
    }

    /// Create an unsubscribe message.
    pub fn unsubscribe(topics: Vec<String>) -> Self {
        Self {
            op: "unsubscribe".to_string(),
            args: Some(topics),
            req_id: None,
        }
    }

    /// Create an auth message for private connections.
    pub fn auth(api_key: &str, expires: u64, signature: &str) -> Self {
        Self {
            op: "auth".to_string(),
            args: Some(vec![
                api_key.to_string(),
                expires.to_string(),
                signature.to_string(),
            ]),
            req_id: None,
        }
    }

    /// Set request ID.
    pub fn with_req_id(mut self, req_id: impl Into<String>) -> Self {
        self.req_id = Some(req_id.into());
        self
    }
}

/// WebSocket response to an operation (subscribe, auth, etc.).
#[derive(Debug, Clone, Deserialize)]
pub struct WsOperationResponse {
    /// Whether the operation was successful.
    pub success: bool,
    /// Response message.
    #[serde(default)]
    pub ret_msg: Option<String>,
    /// Connection ID.
    #[serde(default)]
    pub conn_id: Option<String>,
    /// Request ID (if provided in request).
    #[serde(default)]
    pub req_id: Option<String>,
    /// Operation that was performed.
    #[serde(default)]
    pub op: Option<String>,
}

/// Pong response from server.
#[derive(Debug, Clone, Deserialize)]
pub struct WsPong {
    /// Whether successful.
    pub success: bool,
    /// Response message.
    #[serde(default)]
    pub ret_msg: Option<String>,
    /// Connection ID.
    #[serde(default)]
    pub conn_id: Option<String>,
    /// Request ID.
    #[serde(default)]
    pub req_id: Option<String>,
    /// Operation type ("pong").
    pub op: String,
}


/// Generic WebSocket message wrapper for stream data.
#[derive(Debug, Clone, Deserialize)]
pub struct WsStreamMessage<T> {
    /// Topic name (e.g., "orderbook.50.BTCUSDT").
    pub topic: String,
    /// Message type: "snapshot" or "delta".
    #[serde(rename = "type")]
    pub update_type: String,
    /// Timestamp (milliseconds).
    pub ts: u64,
    /// The actual data.
    pub data: T,
    /// Timestamp of cross sequence.
    #[serde(default)]
    pub cts: Option<u64>,
}

/// Orderbook entry (price level).
#[derive(Debug, Clone, Deserialize)]
pub struct OrderbookEntry {
    /// Price.
    #[serde(rename = "0")]
    pub price: String,
    /// Size/quantity.
    #[serde(rename = "1")]
    pub size: String,
}

/// Orderbook data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderbookData {
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bids (buy orders).
    #[serde(rename = "b")]
    pub bids: Vec<OrderbookEntry>,
    /// Asks (sell orders).
    #[serde(rename = "a")]
    pub asks: Vec<OrderbookEntry>,
    /// Update ID.
    #[serde(rename = "u")]
    pub update_id: u64,
    /// Sequence number.
    #[serde(default)]
    pub seq: Option<u64>,
}

/// Public trade data.
#[derive(Debug, Clone, Deserialize)]
pub struct TradeData {
    /// Timestamp (milliseconds).
    #[serde(rename = "T")]
    pub timestamp: u64,
    /// Symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Side (Buy/Sell).
    #[serde(rename = "S")]
    pub side: String,
    /// Trade size/quantity.
    #[serde(rename = "v")]
    pub size: String,
    /// Trade price.
    #[serde(rename = "p")]
    pub price: String,
    /// Tick direction.
    #[serde(rename = "L")]
    pub tick_direction: String,
    /// Trade ID.
    #[serde(rename = "i")]
    pub trade_id: String,
    /// Block trade flag.
    #[serde(rename = "BT")]
    pub is_block_trade: bool,
}

/// Ticker data for linear/inverse.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerData {
    /// Symbol.
    pub symbol: String,
    /// Tick direction.
    #[serde(default)]
    pub tick_direction: Option<String>,
    /// 24h price change percentage.
    #[serde(default)]
    pub price24h_pcnt: Option<String>,
    /// Last price.
    #[serde(default)]
    pub last_price: Option<String>,
    /// Previous 24h price.
    #[serde(default)]
    pub prev_price24h: Option<String>,
    /// Highest price in 24h.
    #[serde(default)]
    pub high_price24h: Option<String>,
    /// Lowest price in 24h.
    #[serde(default)]
    pub low_price24h: Option<String>,
    /// Previous 1h price.
    #[serde(default)]
    pub prev_price1h: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Index price.
    #[serde(default)]
    pub index_price: Option<String>,
    /// Open interest.
    #[serde(default)]
    pub open_interest: Option<String>,
    /// Open interest value.
    #[serde(default)]
    pub open_interest_value: Option<String>,
    /// Turnover in 24h.
    #[serde(default)]
    pub turnover24h: Option<String>,
    /// Volume in 24h.
    #[serde(default)]
    pub volume24h: Option<String>,
    /// Next funding timestamp.
    #[serde(default)]
    pub next_funding_time: Option<String>,
    /// Funding rate.
    #[serde(default)]
    pub funding_rate: Option<String>,
    /// Best bid price.
    #[serde(default)]
    pub bid1_price: Option<String>,
    /// Best bid size.
    #[serde(default)]
    pub bid1_size: Option<String>,
    /// Best ask price.
    #[serde(default)]
    pub ask1_price: Option<String>,
    /// Best ask size.
    #[serde(default)]
    pub ask1_size: Option<String>,
}

/// Kline (candlestick) data.
#[derive(Debug, Clone, Deserialize)]
pub struct KlineData {
    /// Start time (milliseconds).
    pub start: u64,
    /// End time (milliseconds).
    pub end: u64,
    /// Interval.
    pub interval: String,
    /// Open price.
    pub open: String,
    /// Close price.
    pub close: String,
    /// High price.
    pub high: String,
    /// Low price.
    pub low: String,
    /// Volume.
    pub volume: String,
    /// Turnover.
    pub turnover: String,
    /// Whether this kline is confirmed.
    pub confirm: bool,
    /// Timestamp.
    pub timestamp: u64,
}

/// Liquidation data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiquidationData {
    /// Symbol.
    pub symbol: String,
    /// Side (Buy/Sell).
    pub side: String,
    /// Liquidation price.
    pub price: String,
    /// Liquidation size.
    pub size: String,
    /// Update time (milliseconds).
    pub updated_time: u64,
}


/// Generic private WebSocket message wrapper.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsPrivateMessage<T> {
    /// Message ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Topic name.
    pub topic: String,
    /// Creation timestamp (milliseconds).
    pub creation_time: u64,
    /// The actual data.
    pub data: T,
}

/// Position update data from private WebSocket stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PositionData {
    /// Category (linear, inverse, option).
    pub category: String,
    /// Symbol.
    pub symbol: String,
    /// Position side (Buy/Sell for one-way, None for two-way).
    pub side: String,
    /// Position size.
    pub size: String,
    /// Position index (0: one-way, 1: buy hedge, 2: sell hedge).
    #[serde(default)]
    pub position_idx: i32,
    /// Trade mode (0: cross, 1: isolated).
    #[serde(default)]
    pub trade_mode: i32,
    /// Position value.
    #[serde(default)]
    pub position_value: Option<String>,
    /// Risk ID.
    #[serde(default)]
    pub risk_id: Option<i32>,
    /// Risk limit value.
    #[serde(default)]
    pub risk_limit_value: Option<String>,
    /// Entry price.
    #[serde(default)]
    pub entry_price: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Leverage.
    #[serde(default)]
    pub leverage: Option<String>,
    /// Position balance.
    #[serde(default)]
    pub position_balance: Option<String>,
    /// Auto add margin (0: no, 1: yes).
    #[serde(default)]
    pub auto_add_margin: Option<i32>,
    /// Position maintenance margin.
    #[serde(default)]
    pub position_m_m: Option<String>,
    /// Position initial margin.
    #[serde(default)]
    pub position_i_m: Option<String>,
    /// Liquidation price.
    #[serde(default)]
    pub liq_price: Option<String>,
    /// Bankruptcy price.
    #[serde(default)]
    pub bust_price: Option<String>,
    /// TP/SL mode (Full/Partial).
    #[serde(default)]
    pub tpsl_mode: Option<String>,
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
    /// Current realised PnL.
    #[serde(default)]
    pub cur_realised_pnl: Option<String>,
    /// Cumulative realised PnL.
    #[serde(default)]
    pub cum_realised_pnl: Option<String>,
    /// Session average price.
    #[serde(default)]
    pub session_avg_price: Option<String>,
    /// Position status (Normal, Liq, Adl).
    #[serde(default)]
    pub position_status: Option<String>,
    /// ADL rank indicator.
    #[serde(default)]
    pub adl_rank_indicator: Option<i32>,
    /// Is reduce only.
    #[serde(default)]
    pub is_reduce_only: Option<bool>,
    /// Created time.
    #[serde(default)]
    pub created_time: Option<String>,
    /// Updated time.
    #[serde(default)]
    pub updated_time: Option<String>,
    /// Sequence number.
    #[serde(default)]
    pub seq: Option<i64>,
}

/// Order update data from private WebSocket stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderData {
    /// Category (spot, linear, inverse, option).
    pub category: String,
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Is leverage order.
    #[serde(default)]
    pub is_leverage: Option<String>,
    /// Block trade ID.
    #[serde(default)]
    pub block_trade_id: Option<String>,
    /// Symbol.
    pub symbol: String,
    /// Order price.
    pub price: String,
    /// Order quantity.
    pub qty: String,
    /// Side (Buy/Sell).
    pub side: String,
    /// Position index.
    #[serde(default)]
    pub position_idx: Option<i32>,
    /// Order status.
    pub order_status: String,
    /// Create type.
    #[serde(default)]
    pub create_type: Option<String>,
    /// Cancel type.
    #[serde(default)]
    pub cancel_type: Option<String>,
    /// Reject reason.
    #[serde(default)]
    pub reject_reason: Option<String>,
    /// Average fill price.
    #[serde(default)]
    pub avg_price: Option<String>,
    /// Remaining quantity.
    #[serde(default)]
    pub leaves_qty: Option<String>,
    /// Remaining value.
    #[serde(default)]
    pub leaves_value: Option<String>,
    /// Cumulative executed quantity.
    #[serde(default)]
    pub cum_exec_qty: Option<String>,
    /// Cumulative executed value.
    #[serde(default)]
    pub cum_exec_value: Option<String>,
    /// Cumulative executed fee.
    #[serde(default)]
    pub cum_exec_fee: Option<String>,
    /// Closed PnL.
    #[serde(default)]
    pub closed_pnl: Option<String>,
    /// Fee currency.
    #[serde(default)]
    pub fee_currency: Option<String>,
    /// Time in force.
    #[serde(default)]
    pub time_in_force: Option<String>,
    /// Order type (Limit/Market).
    pub order_type: String,
    /// Stop order type.
    #[serde(default)]
    pub stop_order_type: Option<String>,
    /// Trigger price.
    #[serde(default)]
    pub trigger_price: Option<String>,
    /// Take profit.
    #[serde(default)]
    pub take_profit: Option<String>,
    /// Stop loss.
    #[serde(default)]
    pub stop_loss: Option<String>,
    /// TP/SL mode.
    #[serde(default)]
    pub tpsl_mode: Option<String>,
    /// Reduce only.
    #[serde(default)]
    pub reduce_only: Option<bool>,
    /// Close on trigger.
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
    /// Created time.
    #[serde(default)]
    pub created_time: Option<String>,
    /// Updated time.
    #[serde(default)]
    pub updated_time: Option<String>,
}

/// Execution (trade fill) data from private WebSocket stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionData {
    /// Category.
    pub category: String,
    /// Symbol.
    pub symbol: String,
    /// Is leverage.
    #[serde(default)]
    pub is_leverage: Option<String>,
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Side.
    pub side: String,
    /// Order price.
    #[serde(default)]
    pub order_price: Option<String>,
    /// Order quantity.
    #[serde(default)]
    pub order_qty: Option<String>,
    /// Remaining quantity.
    #[serde(default)]
    pub leaves_qty: Option<String>,
    /// Create type.
    #[serde(default)]
    pub create_type: Option<String>,
    /// Order type.
    #[serde(default)]
    pub order_type: Option<String>,
    /// Stop order type.
    #[serde(default)]
    pub stop_order_type: Option<String>,
    /// Execution fee.
    #[serde(default)]
    pub exec_fee: Option<String>,
    /// Fee currency.
    #[serde(default)]
    pub fee_currency: Option<String>,
    /// Execution ID.
    pub exec_id: String,
    /// Execution price.
    pub exec_price: String,
    /// Execution quantity.
    pub exec_qty: String,
    /// Execution PnL.
    #[serde(default)]
    pub exec_pnl: Option<String>,
    /// Execution type (Trade, Funding, AdlTrade, BustTrade).
    #[serde(default)]
    pub exec_type: Option<String>,
    /// Execution value.
    #[serde(default)]
    pub exec_value: Option<String>,
    /// Execution time.
    pub exec_time: String,
    /// Is maker.
    #[serde(default)]
    pub is_maker: Option<bool>,
    /// Fee rate.
    #[serde(default)]
    pub fee_rate: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Index price.
    #[serde(default)]
    pub index_price: Option<String>,
    /// Closed size.
    #[serde(default)]
    pub closed_size: Option<String>,
    /// Sequence number.
    #[serde(default)]
    pub seq: Option<i64>,
}

/// Fast execution data (low-latency) from private WebSocket stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExecutionFastData {
    /// Category.
    pub category: String,
    /// Symbol.
    pub symbol: String,
    /// Execution ID.
    pub exec_id: String,
    /// Execution price.
    pub exec_price: String,
    /// Execution quantity.
    pub exec_qty: String,
    /// Order ID.
    pub order_id: String,
    /// Is maker.
    #[serde(default)]
    pub is_maker: Option<bool>,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Side.
    pub side: String,
    /// Execution time.
    pub exec_time: String,
    /// Sequence number.
    #[serde(default)]
    pub seq: Option<i64>,
}

/// Coin balance data within wallet update.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinData {
    /// Coin name.
    pub coin: String,
    /// Equity.
    #[serde(default)]
    pub equity: Option<String>,
    /// USD value.
    #[serde(default)]
    pub usd_value: Option<String>,
    /// Wallet balance.
    #[serde(default)]
    pub wallet_balance: Option<String>,
    /// Free balance.
    #[serde(default)]
    pub free: Option<String>,
    /// Locked balance.
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
}

/// Wallet update data from private WebSocket stream.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WalletData {
    /// Account type (UNIFIED, CONTRACT, SPOT).
    pub account_type: String,
    /// Account LTV (loan-to-value).
    #[serde(default)]
    pub account_l_t_v: Option<String>,
    /// Account initial margin rate.
    #[serde(default)]
    pub account_i_m_rate: Option<String>,
    /// Account maintenance margin rate.
    #[serde(default)]
    pub account_m_m_rate: Option<String>,
    /// Total equity.
    #[serde(default)]
    pub total_equity: Option<String>,
    /// Total wallet balance.
    #[serde(default)]
    pub total_wallet_balance: Option<String>,
    /// Total margin balance.
    #[serde(default)]
    pub total_margin_balance: Option<String>,
    /// Total available balance.
    #[serde(default)]
    pub total_available_balance: Option<String>,
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
    #[serde(default)]
    pub coin: Vec<CoinData>,
}

/// Greeks update data from private WebSocket stream (options).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GreeksData {
    /// Base coin.
    pub base_coin: String,
    /// Total delta.
    #[serde(default)]
    pub total_delta: Option<String>,
    /// Total gamma.
    #[serde(default)]
    pub total_gamma: Option<String>,
    /// Total vega.
    #[serde(default)]
    pub total_vega: Option<String>,
    /// Total theta.
    #[serde(default)]
    pub total_theta: Option<String>,
}


/// High-level WebSocket message types.
#[derive(Debug, Clone)]
pub enum WsMessage {
    /// Pong response.
    Pong(WsPong),
    /// Operation response (subscribe, unsubscribe, auth).
    OperationResponse(WsOperationResponse),
    /// Orderbook update.
    Orderbook(Box<WsStreamMessage<OrderbookData>>),
    /// Trade update.
    Trade(Box<WsStreamMessage<Vec<TradeData>>>),
    /// Ticker update.
    Ticker(Box<WsStreamMessage<TickerData>>),
    /// Kline update.
    Kline(Box<WsStreamMessage<Vec<KlineData>>>),
    /// Liquidation update.
    Liquidation(Box<WsStreamMessage<LiquidationData>>),
    /// Position update (private stream).
    Position(Box<WsPrivateMessage<Vec<PositionData>>>),
    /// Order update (private stream).
    Order(Box<WsPrivateMessage<Vec<OrderData>>>),
    /// Execution update (private stream).
    Execution(Box<WsPrivateMessage<Vec<ExecutionData>>>),
    /// Fast execution update (private stream).
    ExecutionFast(Box<WsPrivateMessage<Vec<ExecutionFastData>>>),
    /// Wallet update (private stream).
    Wallet(Box<WsPrivateMessage<Vec<WalletData>>>),
    /// Greeks update (private stream, options).
    Greeks(Box<WsPrivateMessage<Vec<GreeksData>>>),
    /// Unknown/raw message.
    Raw(String),
}


/// WebSocket channel type (determines which URL to connect to).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum WsChannel {
    /// Public spot channel.
    PublicSpot,
    /// Public linear perpetual channel.
    PublicLinear,
    /// Public inverse perpetual channel.
    PublicInverse,
    /// Public options channel.
    PublicOption,
    /// Private channel (requires authentication).
    Private,
    /// Trade channel (WebSocket API for orders).
    Trade,
}

impl WsChannel {
    /// Get the URL path for this channel.
    pub fn path(&self) -> &'static str {
        match self {
            WsChannel::PublicSpot => "/v5/public/spot",
            WsChannel::PublicLinear => "/v5/public/linear",
            WsChannel::PublicInverse => "/v5/public/inverse",
            WsChannel::PublicOption => "/v5/public/option",
            WsChannel::Private => "/v5/private",
            WsChannel::Trade => "/v5/trade",
        }
    }

    /// Create a public channel from a Category.
    pub fn from_category(category: Category) -> Self {
        match category {
            Category::Spot => WsChannel::PublicSpot,
            Category::Linear => WsChannel::PublicLinear,
            Category::Inverse => WsChannel::PublicInverse,
            Category::Option => WsChannel::PublicOption,
        }
    }

    /// Check if this channel requires authentication.
    pub fn requires_auth(&self) -> bool {
        matches!(self, WsChannel::Private | WsChannel::Trade)
    }
}

/// Connection state for a WebSocket connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConnectionState {
    /// Not connected.
    Disconnected,
    /// Connection in progress.
    Connecting,
    /// Connected and ready.
    Connected,
    /// Reconnecting after disconnect.
    Reconnecting,
    /// Connection closed.
    Closed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ws_operation_serialize() {
        let op = WsOperation::subscribe(vec!["orderbook.50.BTCUSDT".to_string()]);
        let json = serde_json::to_string(&op).unwrap();
        assert!(json.contains("\"op\":\"subscribe\""));
        assert!(json.contains("orderbook.50.BTCUSDT"));
    }

    #[test]
    fn test_ws_operation_ping() {
        let op = WsOperation::ping();
        let json = serde_json::to_string(&op).unwrap();
        assert_eq!(json, r#"{"op":"ping"}"#);
    }

    #[test]
    fn test_ws_channel_path() {
        assert_eq!(WsChannel::PublicLinear.path(), "/v5/public/linear");
        assert_eq!(WsChannel::Private.path(), "/v5/private");
    }

    #[test]
    fn test_orderbook_data_deserialize() {
        let json = r#"{
            "s": "BTCUSDT",
            "b": [["50000", "1.5"], ["49999", "2.0"]],
            "a": [["50001", "0.8"]],
            "u": 12345
        }"#;
        let data: OrderbookData = serde_json::from_str(json).unwrap();
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.bids.len(), 2);
        assert_eq!(data.bids[0].price, "50000");
        assert_eq!(data.bids[0].size, "1.5");
    }

    #[test]
    fn test_position_data_deserialize() {
        let json = r#"{
            "category": "linear",
            "symbol": "BTCUSDT",
            "side": "Buy",
            "size": "0.01",
            "positionIdx": 0,
            "tradeMode": 0,
            "positionValue": "500.0",
            "entryPrice": "50000",
            "markPrice": "50100",
            "leverage": "10",
            "unrealisedPnl": "1.0",
            "cumRealisedPnl": "100.0",
            "positionStatus": "Normal",
            "createdTime": "1658384314791",
            "updatedTime": "1658384314792"
        }"#;
        let data: PositionData = serde_json::from_str(json).unwrap();
        assert_eq!(data.category, "linear");
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.side, "Buy");
        assert_eq!(data.size, "0.01");
        assert_eq!(data.entry_price, Some("50000".to_string()));
    }

    #[test]
    fn test_order_data_deserialize() {
        let json = r#"{
            "category": "linear",
            "orderId": "order-123",
            "orderLinkId": "my-order-1",
            "symbol": "BTCUSDT",
            "price": "50000",
            "qty": "0.01",
            "side": "Buy",
            "orderStatus": "New",
            "orderType": "Limit",
            "timeInForce": "GTC",
            "createdTime": "1658384314791",
            "updatedTime": "1658384314792"
        }"#;
        let data: OrderData = serde_json::from_str(json).unwrap();
        assert_eq!(data.category, "linear");
        assert_eq!(data.order_id, "order-123");
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.side, "Buy");
        assert_eq!(data.order_status, "New");
        assert_eq!(data.order_type, "Limit");
    }

    #[test]
    fn test_execution_data_deserialize() {
        let json = r#"{
            "category": "linear",
            "symbol": "BTCUSDT",
            "orderId": "order-123",
            "side": "Buy",
            "execId": "exec-456",
            "execPrice": "50000",
            "execQty": "0.01",
            "execTime": "1658384314791",
            "execType": "Trade",
            "isMaker": false,
            "feeRate": "0.0006"
        }"#;
        let data: ExecutionData = serde_json::from_str(json).unwrap();
        assert_eq!(data.category, "linear");
        assert_eq!(data.symbol, "BTCUSDT");
        assert_eq!(data.exec_id, "exec-456");
        assert_eq!(data.exec_price, "50000");
        assert_eq!(data.exec_qty, "0.01");
        assert_eq!(data.is_maker, Some(false));
    }

    #[test]
    fn test_wallet_data_deserialize() {
        let json = r#"{
            "accountType": "UNIFIED",
            "totalEquity": "10000.0",
            "totalWalletBalance": "9500.0",
            "totalAvailableBalance": "8000.0",
            "coin": [
                {
                    "coin": "USDT",
                    "equity": "5000.0",
                    "walletBalance": "5000.0",
                    "availableToWithdraw": "4500.0"
                },
                {
                    "coin": "BTC",
                    "equity": "0.1",
                    "walletBalance": "0.1"
                }
            ]
        }"#;
        let data: WalletData = serde_json::from_str(json).unwrap();
        assert_eq!(data.account_type, "UNIFIED");
        assert_eq!(data.total_equity, Some("10000.0".to_string()));
        assert_eq!(data.coin.len(), 2);
        assert_eq!(data.coin[0].coin, "USDT");
        assert_eq!(data.coin[1].coin, "BTC");
    }

    #[test]
    fn test_greeks_data_deserialize() {
        let json = r#"{
            "baseCoin": "BTC",
            "totalDelta": "0.5",
            "totalGamma": "0.001",
            "totalVega": "100.0",
            "totalTheta": "-50.0"
        }"#;
        let data: GreeksData = serde_json::from_str(json).unwrap();
        assert_eq!(data.base_coin, "BTC");
        assert_eq!(data.total_delta, Some("0.5".to_string()));
        assert_eq!(data.total_gamma, Some("0.001".to_string()));
    }

    #[test]
    fn test_private_message_deserialize() {
        let json = r#"{
            "topic": "position",
            "creationTime": 1658384314791,
            "data": [{
                "category": "linear",
                "symbol": "BTCUSDT",
                "side": "Buy",
                "size": "0.01"
            }]
        }"#;
        let msg: WsPrivateMessage<Vec<PositionData>> = serde_json::from_str(json).unwrap();
        assert_eq!(msg.topic, "position");
        assert_eq!(msg.creation_time, 1658384314791);
        assert_eq!(msg.data.len(), 1);
        assert_eq!(msg.data[0].symbol, "BTCUSDT");
    }
}
