//! Core enums for the Bybit V5 API.

use serde::{Deserialize, Serialize};

/// Trading category for the V5 API.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Category {
    /// Spot trading
    Spot,
    /// Linear perpetuals (USDT/USDC settled)
    Linear,
    /// Inverse perpetuals and futures
    Inverse,
    /// Options trading
    Option,
}

impl Category {
    /// Returns the string representation used in API requests.
    pub fn as_str(&self) -> &'static str {
        match self {
            Category::Spot => "spot",
            Category::Linear => "linear",
            Category::Inverse => "inverse",
            Category::Option => "option",
        }
    }
}

impl std::fmt::Display for Category {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Order side (buy or sell).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Side {
    Buy,
    Sell,
}

impl Side {
    pub fn as_str(&self) -> &'static str {
        match self {
            Side::Buy => "Buy",
            Side::Sell => "Sell",
        }
    }
}

impl std::fmt::Display for Side {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderType {
    Market,
    Limit,
}

impl OrderType {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderType::Market => "Market",
            OrderType::Limit => "Limit",
        }
    }
}

impl std::fmt::Display for OrderType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Time in force for orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TimeInForce {
    /// Good Till Cancel
    GTC,
    /// Immediate Or Cancel
    IOC,
    /// Fill Or Kill
    FOK,
    /// Post Only (maker only)
    PostOnly,
}

impl TimeInForce {
    pub fn as_str(&self) -> &'static str {
        match self {
            TimeInForce::GTC => "GTC",
            TimeInForce::IOC => "IOC",
            TimeInForce::FOK => "FOK",
            TimeInForce::PostOnly => "PostOnly",
        }
    }
}

impl std::fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum OrderStatus {
    /// Order has been created but not yet submitted
    Created,
    /// Order is active and waiting to be filled
    New,
    /// Order was rejected
    Rejected,
    /// Order is partially filled
    PartiallyFilled,
    /// Order was partially filled then cancelled
    PartiallyFilledCanceled,
    /// Order is completely filled
    Filled,
    /// Order was cancelled
    Cancelled,
    /// Conditional order waiting for trigger
    Untriggered,
    /// Conditional order has been triggered
    Triggered,
    /// Order has been deactivated
    Deactivated,
}

impl OrderStatus {
    pub fn as_str(&self) -> &'static str {
        match self {
            OrderStatus::Created => "Created",
            OrderStatus::New => "New",
            OrderStatus::Rejected => "Rejected",
            OrderStatus::PartiallyFilled => "PartiallyFilled",
            OrderStatus::PartiallyFilledCanceled => "PartiallyFilledCanceled",
            OrderStatus::Filled => "Filled",
            OrderStatus::Cancelled => "Cancelled",
            OrderStatus::Untriggered => "Untriggered",
            OrderStatus::Triggered => "Triggered",
            OrderStatus::Deactivated => "Deactivated",
        }
    }

    /// Returns true if the order is in a final state (no more updates expected).
    pub fn is_final(&self) -> bool {
        matches!(
            self,
            OrderStatus::Rejected
                | OrderStatus::PartiallyFilledCanceled
                | OrderStatus::Filled
                | OrderStatus::Cancelled
                | OrderStatus::Deactivated
        )
    }

    /// Returns true if the order is still active.
    pub fn is_active(&self) -> bool {
        matches!(
            self,
            OrderStatus::Created
                | OrderStatus::New
                | OrderStatus::PartiallyFilled
                | OrderStatus::Untriggered
                | OrderStatus::Triggered
        )
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Position index for position mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum PositionIdx {
    /// One-way mode (default)
    OneWay = 0,
    /// Hedge mode - Buy side
    HedgeBuy = 1,
    /// Hedge mode - Sell side
    HedgeSell = 2,
}

impl Serialize for PositionIdx {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_i32(*self as i32)
    }
}

impl<'de> Deserialize<'de> for PositionIdx {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        match i32::deserialize(deserializer)? {
            0 => Ok(PositionIdx::OneWay),
            1 => Ok(PositionIdx::HedgeBuy),
            2 => Ok(PositionIdx::HedgeSell),
            other => Err(serde::de::Error::custom(format!(
                "invalid positionIdx: {other}"
            ))),
        }
    }
}

impl From<PositionIdx> for i32 {
    fn from(idx: PositionIdx) -> Self {
        idx as i32
    }
}

/// Account type for wallet operations.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountType {
    /// Unified trading account
    Unified,
    /// Contract account (derivatives)
    Contract,
    /// Spot account
    Spot,
    /// Investment/Earn account
    Investment,
    /// Options account (USDC)
    Option,
    /// Funding account
    Fund,
}

impl AccountType {
    pub fn as_str(&self) -> &'static str {
        match self {
            AccountType::Unified => "UNIFIED",
            AccountType::Contract => "CONTRACT",
            AccountType::Spot => "SPOT",
            AccountType::Investment => "INVESTMENT",
            AccountType::Option => "OPTION",
            AccountType::Fund => "FUND",
        }
    }
}

impl std::fmt::Display for AccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Trigger price type for conditional orders.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TriggerBy {
    LastPrice,
    IndexPrice,
    MarkPrice,
}

impl TriggerBy {
    pub fn as_str(&self) -> &'static str {
        match self {
            TriggerBy::LastPrice => "LastPrice",
            TriggerBy::IndexPrice => "IndexPrice",
            TriggerBy::MarkPrice => "MarkPrice",
        }
    }
}

impl std::fmt::Display for TriggerBy {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Kline/candlestick interval.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum KlineInterval {
    /// 1 minute
    #[serde(rename = "1")]
    Min1,
    /// 3 minutes
    #[serde(rename = "3")]
    Min3,
    /// 5 minutes
    #[serde(rename = "5")]
    Min5,
    /// 15 minutes
    #[serde(rename = "15")]
    Min15,
    /// 30 minutes
    #[serde(rename = "30")]
    Min30,
    /// 1 hour
    #[serde(rename = "60")]
    Hour1,
    /// 2 hours
    #[serde(rename = "120")]
    Hour2,
    /// 4 hours
    #[serde(rename = "240")]
    Hour4,
    /// 6 hours
    #[serde(rename = "360")]
    Hour6,
    /// 12 hours
    #[serde(rename = "720")]
    Hour12,
    /// 1 day
    #[serde(rename = "D")]
    Day1,
    /// 1 week
    #[serde(rename = "W")]
    Week1,
    /// 1 month
    #[serde(rename = "M")]
    Month1,
}

impl KlineInterval {
    pub fn as_str(&self) -> &'static str {
        match self {
            KlineInterval::Min1 => "1",
            KlineInterval::Min3 => "3",
            KlineInterval::Min5 => "5",
            KlineInterval::Min15 => "15",
            KlineInterval::Min30 => "30",
            KlineInterval::Hour1 => "60",
            KlineInterval::Hour2 => "120",
            KlineInterval::Hour4 => "240",
            KlineInterval::Hour6 => "360",
            KlineInterval::Hour12 => "720",
            KlineInterval::Day1 => "D",
            KlineInterval::Week1 => "W",
            KlineInterval::Month1 => "M",
        }
    }
}

impl std::fmt::Display for KlineInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

/// Stop order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum StopOrderType {
    TakeProfit,
    StopLoss,
    TrailingStop,
    Stop,
    PartialTakeProfit,
    PartialStopLoss,
}

/// Trade mode (cross or isolated margin).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum TradeMode {
    /// Cross margin mode
    CrossMargin = 0,
    /// Isolated margin mode
    IsolatedMargin = 1,
}

/// Position mode.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(i32)]
pub enum PositionMode {
    /// Merged single position (one-way mode)
    MergedSingle = 0,
    /// Both side position (hedge mode)
    BothSide = 3,
}

/// Margin mode for the account.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum MarginMode {
    RegularMargin,
    PortfolioMargin,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_category_serialization() {
        let serialized = match serde_json::to_string(&Category::Linear) {
            Ok(serialized) => serialized,
            Err(err) => panic!("Failed to serialize category: {}", err),
        };
        assert_eq!(serialized, "\"linear\"");

        let parsed = match serde_json::from_str::<Category>("\"spot\"") {
            Ok(parsed) => parsed,
            Err(err) => panic!("Failed to deserialize category: {}", err),
        };
        assert_eq!(parsed, Category::Spot);
    }

    #[test]
    fn test_order_status_helpers() {
        assert!(OrderStatus::Filled.is_final());
        assert!(!OrderStatus::New.is_final());
        assert!(OrderStatus::New.is_active());
        assert!(!OrderStatus::Cancelled.is_active());
    }

    #[test]
    fn test_kline_interval() {
        assert_eq!(KlineInterval::Hour1.as_str(), "60");
        assert_eq!(KlineInterval::Day1.as_str(), "D");
    }
}
