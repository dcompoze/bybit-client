//! Orderbook management with delta update support.
//!
//! This module provides a local orderbook that can be maintained from
//! WebSocket snapshot and delta messages.
//!
//! # Usage
//!
//! ```no_run
//! use bybit_client::ws::{LocalOrderbook, WsClient, WsChannel, WsMessage};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (client, mut receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;
//!     client.subscribe(&["orderbook.50.BTCUSDT"]).await?;
//!
//!     let mut orderbook = LocalOrderbook::new("BTCUSDT");
//!
//!     while let Some(msg) = receiver.recv().await {
//!         if let WsMessage::Orderbook(update) = msg {
//!             orderbook.apply_update(&update)?;
//!             println!("Best bid: {:?}, Best ask: {:?}",
//!                 orderbook.best_bid(), orderbook.best_ask());
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::collections::BTreeMap;
use std::cmp::Ordering;

use crate::error::BybitError;
use crate::ws::types::{OrderbookData, OrderbookEntry, WsStreamMessage};

/// A price level in the orderbook.
#[derive(Debug, Clone, PartialEq)]
pub struct PriceLevel {
    /// Price as a string (preserves decimal precision).
    pub price: String,
    /// Price as f64 for comparison and sorting.
    pub price_f64: f64,
    /// Size/quantity at this price level.
    pub size: String,
    /// Size as f64.
    pub size_f64: f64,
}

impl PriceLevel {
    /// Create a new price level from strings.
    pub fn new(price: String, size: String) -> Result<Self, BybitError> {
        let price_f64 = price
            .parse::<f64>()
            .map_err(|e| BybitError::InvalidParameter(format!("Invalid price '{}': {}", price, e)))?;
        let size_f64 = size
            .parse::<f64>()
            .map_err(|e| BybitError::InvalidParameter(format!("Invalid size '{}': {}", size, e)))?;

        Ok(Self {
            price,
            price_f64,
            size,
            size_f64,
        })
    }

    /// Create from an orderbook entry.
    pub fn from_entry(entry: &OrderbookEntry) -> Result<Self, BybitError> {
        Self::new(entry.price.clone(), entry.size.clone())
    }
}

/// Wrapper for price that implements Ord for use in BTreeMap.
/// Bids are sorted descending (highest first), asks ascending (lowest first).
#[derive(Debug, Clone, Copy)]
struct OrderedPrice {
    price: f64,
    /// If true, sort descending (for bids). If false, sort ascending (for asks).
    descending: bool,
}

impl PartialEq for OrderedPrice {
    fn eq(&self, other: &Self) -> bool {
        self.price == other.price
    }
}

impl Eq for OrderedPrice {}

impl PartialOrd for OrderedPrice {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for OrderedPrice {
    fn cmp(&self, other: &Self) -> Ordering {
        let cmp = self
            .price
            .partial_cmp(&other.price)
            .unwrap_or(Ordering::Equal);
        if self.descending {
            cmp.reverse()
        } else {
            cmp
        }
    }
}

/// Local orderbook maintained from WebSocket updates.
///
/// This structure maintains a sorted orderbook from snapshot and delta updates.
/// Bids are sorted by price descending (highest first), asks ascending (lowest first).
#[derive(Debug)]
pub struct LocalOrderbook {
    /// Symbol this orderbook is for.
    symbol: String,
    /// Bid price levels (sorted by price descending).
    bids: BTreeMap<OrderedPrice, PriceLevel>,
    /// Ask price levels (sorted by price ascending).
    asks: BTreeMap<OrderedPrice, PriceLevel>,
    /// Current update ID.
    update_id: u64,
    /// Sequence number for cross-depth comparison.
    sequence: Option<u64>,
    /// Last update timestamp.
    last_update_ts: u64,
    /// Whether the orderbook has received a snapshot.
    initialized: bool,
}

impl LocalOrderbook {
    /// Create a new empty orderbook for a symbol.
    pub fn new(symbol: impl Into<String>) -> Self {
        Self {
            symbol: symbol.into(),
            bids: BTreeMap::new(),
            asks: BTreeMap::new(),
            update_id: 0,
            sequence: None,
            last_update_ts: 0,
            initialized: false,
        }
    }

    /// Get the symbol this orderbook is for.
    pub fn symbol(&self) -> &str {
        &self.symbol
    }

    /// Check if the orderbook has been initialized with a snapshot.
    pub fn is_initialized(&self) -> bool {
        self.initialized
    }

    /// Get the current update ID.
    pub fn update_id(&self) -> u64 {
        self.update_id
    }

    /// Get the sequence number.
    pub fn sequence(&self) -> Option<u64> {
        self.sequence
    }

    /// Get the last update timestamp.
    pub fn last_update_ts(&self) -> u64 {
        self.last_update_ts
    }

    /// Apply an orderbook update (snapshot or delta).
    ///
    /// # Update Rules
    ///
    /// - Snapshots replace the entire orderbook.
    /// - Deltas are applied incrementally:
    ///   - If size is "0", the price level is removed.
    ///   - If the price level does not exist, it is inserted.
    ///   - If the price level exists, the size is updated.
    /// - If `update_id` is 1, treat it as a snapshot.
    pub fn apply_update(
        &mut self,
        update: &WsStreamMessage<OrderbookData>,
    ) -> Result<(), BybitError> {
        if update.data.symbol != self.symbol {
            return Err(BybitError::InvalidParameter(format!(
                "Symbol mismatch: expected {}, got {}",
                self.symbol, update.data.symbol
            )));
        }

        let is_snapshot = update.update_type == "snapshot" || update.data.update_id == 1;

        if is_snapshot {
            self.apply_snapshot(&update.data)?;
        } else {
            if !self.initialized {
                return Err(BybitError::InvalidParameter(
                    "Received delta before snapshot".to_string(),
                ));
            }
            self.apply_delta(&update.data)?;
        }

        self.update_id = update.data.update_id;
        self.sequence = update.data.seq;
        self.last_update_ts = update.ts;

        Ok(())
    }

    /// Apply a snapshot update (replaces entire orderbook).
    fn apply_snapshot(&mut self, data: &OrderbookData) -> Result<(), BybitError> {
        self.bids.clear();
        self.asks.clear();

        for entry in &data.bids {
            let level = PriceLevel::from_entry(entry)?;
            let key = OrderedPrice {
                price: level.price_f64,
                descending: true,
            };
            self.bids.insert(key, level);
        }

        for entry in &data.asks {
            let level = PriceLevel::from_entry(entry)?;
            let key = OrderedPrice {
                price: level.price_f64,
                descending: false,
            };
            self.asks.insert(key, level);
        }

        self.initialized = true;
        Ok(())
    }

    /// Apply a delta update (incremental changes).
    fn apply_delta(&mut self, data: &OrderbookData) -> Result<(), BybitError> {
        for entry in &data.bids {
            let level = PriceLevel::from_entry(entry)?;
            let key = OrderedPrice {
                price: level.price_f64,
                descending: true,
            };

            if level.size_f64 == 0.0 {
                self.bids.remove(&key);
            } else {
                self.bids.insert(key, level);
            }
        }

        for entry in &data.asks {
            let level = PriceLevel::from_entry(entry)?;
            let key = OrderedPrice {
                price: level.price_f64,
                descending: false,
            };

            if level.size_f64 == 0.0 {
                self.asks.remove(&key);
            } else {
                self.asks.insert(key, level);
            }
        }

        Ok(())
    }

    /// Get the best bid (highest buy price).
    pub fn best_bid(&self) -> Option<&PriceLevel> {
        self.bids.values().next()
    }

    /// Get the best ask (lowest sell price).
    pub fn best_ask(&self) -> Option<&PriceLevel> {
        self.asks.values().next()
    }

    /// Get the bid-ask spread.
    pub fn spread(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some(ask.price_f64 - bid.price_f64),
            _ => None,
        }
    }

    /// Get the mid price (average of best bid and best ask).
    pub fn mid_price(&self) -> Option<f64> {
        match (self.best_ask(), self.best_bid()) {
            (Some(ask), Some(bid)) => Some((ask.price_f64 + bid.price_f64) / 2.0),
            _ => None,
        }
    }

    /// Get the top N bids (sorted by price descending).
    pub fn top_bids(&self, n: usize) -> Vec<&PriceLevel> {
        self.bids.values().take(n).collect()
    }

    /// Get the top N asks (sorted by price ascending).
    pub fn top_asks(&self, n: usize) -> Vec<&PriceLevel> {
        self.asks.values().take(n).collect()
    }

    /// Get all bids (sorted by price descending).
    pub fn bids(&self) -> impl Iterator<Item = &PriceLevel> {
        self.bids.values()
    }

    /// Get all asks (sorted by price ascending).
    pub fn asks(&self) -> impl Iterator<Item = &PriceLevel> {
        self.asks.values()
    }

    /// Get the total bid depth (sum of all bid sizes).
    pub fn bid_depth(&self) -> f64 {
        self.bids.values().map(|l| l.size_f64).sum()
    }

    /// Get the total ask depth (sum of all ask sizes).
    pub fn ask_depth(&self) -> f64 {
        self.asks.values().map(|l| l.size_f64).sum()
    }

    /// Get the number of bid levels.
    pub fn bid_levels(&self) -> usize {
        self.bids.len()
    }

    /// Get the number of ask levels.
    pub fn ask_levels(&self) -> usize {
        self.asks.len()
    }

    /// Clear the orderbook and reset to uninitialized state.
    pub fn clear(&mut self) {
        self.bids.clear();
        self.asks.clear();
        self.update_id = 0;
        self.sequence = None;
        self.last_update_ts = 0;
        self.initialized = false;
    }

    /// Get orderbook imbalance ratio.
    ///
    /// Returns a value between -1 and 1.
    /// - Positive values indicate more bid depth (buying pressure).
    /// - Negative values indicate more ask depth (selling pressure).
    /// - Zero indicates balanced depth.
    pub fn imbalance(&self) -> f64 {
        let bid_depth = self.bid_depth();
        let ask_depth = self.ask_depth();
        let total = bid_depth + ask_depth;

        if total == 0.0 {
            0.0
        } else {
            (bid_depth - ask_depth) / total
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_entry(price: &str, size: &str) -> OrderbookEntry {
        OrderbookEntry {
            price: price.to_string(),
            size: size.to_string(),
        }
    }

    fn make_snapshot(
        symbol: &str,
        bids: Vec<(&str, &str)>,
        asks: Vec<(&str, &str)>,
    ) -> WsStreamMessage<OrderbookData> {
        WsStreamMessage {
            topic: format!("orderbook.50.{}", symbol),
            update_type: "snapshot".to_string(),
            ts: 1234567890000,
            data: OrderbookData {
                symbol: symbol.to_string(),
                bids: bids.iter().map(|(p, s)| make_entry(p, s)).collect(),
                asks: asks.iter().map(|(p, s)| make_entry(p, s)).collect(),
                update_id: 1,
                seq: Some(100),
            },
            cts: None,
        }
    }

    fn make_delta(
        symbol: &str,
        bids: Vec<(&str, &str)>,
        asks: Vec<(&str, &str)>,
        update_id: u64,
    ) -> WsStreamMessage<OrderbookData> {
        WsStreamMessage {
            topic: format!("orderbook.50.{}", symbol),
            update_type: "delta".to_string(),
            ts: 1234567890001,
            data: OrderbookData {
                symbol: symbol.to_string(),
                bids: bids.iter().map(|(p, s)| make_entry(p, s)).collect(),
                asks: asks.iter().map(|(p, s)| make_entry(p, s)).collect(),
                update_id,
                seq: Some(101),
            },
            cts: None,
        }
    }

    #[test]
    fn test_new_orderbook() {
        let ob = LocalOrderbook::new("BTCUSDT");
        assert_eq!(ob.symbol(), "BTCUSDT");
        assert!(!ob.is_initialized());
        assert_eq!(ob.bid_levels(), 0);
        assert_eq!(ob.ask_levels(), 0);
    }

    #[test]
    fn test_apply_snapshot() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.5"), ("49999", "2.0"), ("49998", "3.0")],
            vec![("50001", "0.8"), ("50002", "1.2")],
        );

        ob.apply_update(&snapshot).unwrap();

        assert!(ob.is_initialized());
        assert_eq!(ob.bid_levels(), 3);
        assert_eq!(ob.ask_levels(), 2);

        let best_bid = ob.best_bid().unwrap();
        assert_eq!(best_bid.price, "50000");
        assert_eq!(best_bid.size, "1.5");

        let best_ask = ob.best_ask().unwrap();
        assert_eq!(best_ask.price, "50001");
        assert_eq!(best_ask.size, "0.8");
    }

    #[test]
    fn test_apply_delta_insert() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.5")],
            vec![("50001", "0.8")],
        );
        ob.apply_update(&snapshot).unwrap();

        let delta = make_delta("BTCUSDT", vec![("49999", "2.0")], vec![], 2);
        ob.apply_update(&delta).unwrap();

        assert_eq!(ob.bid_levels(), 2);
        let top_bids = ob.top_bids(2);
        assert_eq!(top_bids[0].price, "50000");
        assert_eq!(top_bids[1].price, "49999");
    }

    #[test]
    fn test_apply_delta_update() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.5")],
            vec![("50001", "0.8")],
        );
        ob.apply_update(&snapshot).unwrap();

        let delta = make_delta("BTCUSDT", vec![("50000", "3.0")], vec![], 2);
        ob.apply_update(&delta).unwrap();

        assert_eq!(ob.bid_levels(), 1);
        let best_bid = ob.best_bid().unwrap();
        assert_eq!(best_bid.price, "50000");
        assert_eq!(best_bid.size, "3.0");
    }

    #[test]
    fn test_apply_delta_delete() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.5"), ("49999", "2.0")],
            vec![("50001", "0.8")],
        );
        ob.apply_update(&snapshot).unwrap();

        let delta = make_delta("BTCUSDT", vec![("50000", "0")], vec![], 2);
        ob.apply_update(&delta).unwrap();

        assert_eq!(ob.bid_levels(), 1);
        let best_bid = ob.best_bid().unwrap();
        assert_eq!(best_bid.price, "49999");
    }

    #[test]
    fn test_spread_and_mid_price() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.0")],
            vec![("50010", "1.0")],
        );
        ob.apply_update(&snapshot).unwrap();

        assert_eq!(ob.spread(), Some(10.0));
        assert_eq!(ob.mid_price(), Some(50005.0));
    }

    #[test]
    fn test_depth_calculation() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.0"), ("49999", "2.0")],
            vec![("50001", "0.5"), ("50002", "1.5")],
        );
        ob.apply_update(&snapshot).unwrap();

        assert_eq!(ob.bid_depth(), 3.0);
        assert_eq!(ob.ask_depth(), 2.0);
    }

    #[test]
    fn test_imbalance() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "3.0")],
            vec![("50001", "1.0")],
        );
        ob.apply_update(&snapshot).unwrap();

        assert_eq!(ob.imbalance(), 0.5);
    }

    #[test]
    fn test_symbol_mismatch() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot("ETHUSDT", vec![], vec![]);

        let result = ob.apply_update(&snapshot);
        assert!(result.is_err());
    }

    #[test]
    fn test_delta_before_snapshot() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let delta = make_delta("BTCUSDT", vec![("50000", "1.0")], vec![], 2);

        let result = ob.apply_update(&delta);
        assert!(result.is_err());
    }

    #[test]
    fn test_reset_on_update_id_1() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.5")],
            vec![("50001", "0.8")],
        );
        ob.apply_update(&snapshot).unwrap();

        let reset = WsStreamMessage {
            topic: "orderbook.50.BTCUSDT".to_string(),
            update_type: "delta".to_string(),
            ts: 1234567890002,
            data: OrderbookData {
                symbol: "BTCUSDT".to_string(),
                bids: vec![make_entry("49000", "5.0")],
                asks: vec![make_entry("49001", "4.0")],
                update_id: 1,
                seq: Some(1),
            },
            cts: None,
        };
        ob.apply_update(&reset).unwrap();

        assert_eq!(ob.bid_levels(), 1);
        assert_eq!(ob.best_bid().unwrap().price, "49000");
    }

    #[test]
    fn test_clear() {
        let mut ob = LocalOrderbook::new("BTCUSDT");
        let snapshot = make_snapshot(
            "BTCUSDT",
            vec![("50000", "1.5")],
            vec![("50001", "0.8")],
        );
        ob.apply_update(&snapshot).unwrap();

        ob.clear();

        assert!(!ob.is_initialized());
        assert_eq!(ob.bid_levels(), 0);
        assert_eq!(ob.ask_levels(), 0);
    }
}
