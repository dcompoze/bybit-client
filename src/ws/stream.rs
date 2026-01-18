//! Async Stream interface for WebSocket messages.
//!
//! This module provides a Stream-based interface for WebSocket messages,
//! enabling use of stream combinators like `.filter()`, `.map()`, etc.
//!
//! # Example
//!
//! ```no_run
//! use std::pin::pin;
//! use bybit_client::ws::{WsClient, WsChannel, WsStream};
//! use futures::StreamExt;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let (client, receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;
//!     client.subscribe(&["publicTrade.BTCUSDT", "tickers.BTCUSDT"]).await?;
//!
//!     // Create a stream from the receiver
//!     let stream = WsStream::new(receiver);
//!
//!     // Use stream combinators - filter only trade messages
//!     // Note: filtered streams need to be pinned to use .next()
//!     let mut trades = pin!(stream.trades());
//!
//!     while let Some(trade) = trades.next().await {
//!         for t in &trade.data {
//!             println!("Trade: {} {} @ {}", t.symbol, t.size, t.price);
//!         }
//!     }
//!
//!     Ok(())
//! }
//! ```

use std::pin::Pin;
use std::task::{Context, Poll};

use futures::Stream;
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::ws::types::*;

/// A Stream wrapper for WebSocket messages.
///
/// This struct wraps an `mpsc::UnboundedReceiver<WsMessage>` and implements
/// the `Stream` trait, enabling use of stream combinators.
pub struct WsStream {
    inner: UnboundedReceiverStream<WsMessage>,
}

impl WsStream {
    /// Create a new WsStream from an unbounded receiver.
    pub fn new(receiver: mpsc::UnboundedReceiver<WsMessage>) -> Self {
        Self {
            inner: UnboundedReceiverStream::new(receiver),
        }
    }

    /// Create a filtered stream that only yields orderbook messages.
    pub fn orderbooks(self) -> impl Stream<Item = Box<WsStreamMessage<OrderbookData>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Orderbook(ob) => Some(ob),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields trade messages.
    pub fn trades(self) -> impl Stream<Item = Box<WsStreamMessage<Vec<TradeData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Trade(t) => Some(t),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields ticker messages.
    pub fn tickers(self) -> impl Stream<Item = Box<WsStreamMessage<TickerData>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Ticker(t) => Some(t),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields kline messages.
    pub fn klines(self) -> impl Stream<Item = Box<WsStreamMessage<Vec<KlineData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Kline(k) => Some(k),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields liquidation messages.
    pub fn liquidations(self) -> impl Stream<Item = Box<WsStreamMessage<LiquidationData>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Liquidation(l) => Some(l),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields operation responses.
    pub fn operation_responses(self) -> impl Stream<Item = WsOperationResponse> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::OperationResponse(r) => Some(r),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields position updates (private stream).
    pub fn positions(self) -> impl Stream<Item = Box<WsPrivateMessage<Vec<PositionData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Position(p) => Some(p),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields order updates (private stream).
    pub fn orders(self) -> impl Stream<Item = Box<WsPrivateMessage<Vec<OrderData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Order(o) => Some(o),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields execution updates (private stream).
    pub fn executions(self) -> impl Stream<Item = Box<WsPrivateMessage<Vec<ExecutionData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Execution(e) => Some(e),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields fast execution updates (private stream).
    pub fn executions_fast(self) -> impl Stream<Item = Box<WsPrivateMessage<Vec<ExecutionFastData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::ExecutionFast(e) => Some(e),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields wallet updates (private stream).
    pub fn wallets(self) -> impl Stream<Item = Box<WsPrivateMessage<Vec<WalletData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Wallet(w) => Some(w),
                _ => None,
            }
        })
    }

    /// Create a filtered stream that only yields greeks updates (private stream, options).
    pub fn greeks(self) -> impl Stream<Item = Box<WsPrivateMessage<Vec<GreeksData>>>> {
        futures::StreamExt::filter_map(self, |msg| async {
            match msg {
                WsMessage::Greeks(g) => Some(g),
                _ => None,
            }
        })
    }

    /// Filter messages for a specific symbol.
    ///
    /// This filters orderbook, trade, ticker, kline, and liquidation messages
    /// that match the given symbol.
    pub fn for_symbol(self, symbol: impl Into<String>) -> impl Stream<Item = WsMessage> {
        let symbol = symbol.into();
        futures::StreamExt::filter(self, move |msg| {
            let matches = match msg {
                WsMessage::Orderbook(ob) => ob.data.symbol == symbol,
                WsMessage::Trade(t) => t.data.first().map(|d| d.symbol == symbol).unwrap_or(false),
                WsMessage::Ticker(t) => t.data.symbol == symbol,
                WsMessage::Kline(k) => k
                    .topic
                    .split('.')
                    .last()
                    .map(|s| s == symbol)
                    .unwrap_or(false),
                WsMessage::Liquidation(l) => l.data.symbol == symbol,
                _ => true, // Pass through other message types
            };
            std::future::ready(matches)
        })
    }

    /// Filter messages by topic prefix.
    ///
    /// This filters stream messages that have a topic starting with the given prefix.
    pub fn for_topic_prefix(self, prefix: impl Into<String>) -> impl Stream<Item = WsMessage> {
        let prefix = prefix.into();
        futures::StreamExt::filter(self, move |msg| {
            let matches = match msg {
                WsMessage::Orderbook(ob) => ob.topic.starts_with(&prefix),
                WsMessage::Trade(t) => t.topic.starts_with(&prefix),
                WsMessage::Ticker(t) => t.topic.starts_with(&prefix),
                WsMessage::Kline(k) => k.topic.starts_with(&prefix),
                WsMessage::Liquidation(l) => l.topic.starts_with(&prefix),
                _ => true, // Pass through other message types
            };
            std::future::ready(matches)
        })
    }
}

impl Stream for WsStream {
    type Item = WsMessage;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        Pin::new(&mut self.inner).poll_next(cx)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

/// Extension trait for converting receivers to streams.
pub trait IntoWsStream {
    /// Convert into a WsStream.
    fn into_stream(self) -> WsStream;
}

impl IntoWsStream for mpsc::UnboundedReceiver<WsMessage> {
    fn into_stream(self) -> WsStream {
        WsStream::new(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    fn make_trade_message(symbol: &str) -> WsMessage {
        WsMessage::Trade(Box::new(WsStreamMessage {
            topic: format!("publicTrade.{}", symbol),
            update_type: "snapshot".to_string(),
            ts: 1234567890000,
            data: vec![TradeData {
                timestamp: 1234567890000,
                symbol: symbol.to_string(),
                side: "Buy".to_string(),
                size: "0.1".to_string(),
                price: "50000".to_string(),
                tick_direction: "ZeroPlusTick".to_string(),
                trade_id: "test-123".to_string(),
                is_block_trade: false,
            }],
            cts: None,
        }))
    }

    fn make_orderbook_message(symbol: &str) -> WsMessage {
        WsMessage::Orderbook(Box::new(WsStreamMessage {
            topic: format!("orderbook.50.{}", symbol),
            update_type: "snapshot".to_string(),
            ts: 1234567890000,
            data: OrderbookData {
                symbol: symbol.to_string(),
                bids: vec![],
                asks: vec![],
                update_id: 1,
                seq: None,
            },
            cts: None,
        }))
    }

    #[tokio::test]
    async fn test_ws_stream_basic() {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut stream = WsStream::new(rx);

        tx.send(make_trade_message("BTCUSDT")).unwrap();
        tx.send(make_orderbook_message("BTCUSDT")).unwrap();
        drop(tx);

        let mut count = 0;
        while let Some(_msg) = stream.next().await {
            count += 1;
        }
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn test_ws_stream_trades_filter() {
        let (tx, rx) = mpsc::unbounded_channel();
        let stream = WsStream::new(rx);

        tx.send(make_trade_message("BTCUSDT")).unwrap();
        tx.send(make_orderbook_message("BTCUSDT")).unwrap();
        tx.send(make_trade_message("ETHUSDT")).unwrap();
        drop(tx);

        let trades: Vec<_> = stream.trades().collect().await;
        assert_eq!(trades.len(), 2);
        assert_eq!(trades[0].data[0].symbol, "BTCUSDT");
        assert_eq!(trades[1].data[0].symbol, "ETHUSDT");
    }

    #[tokio::test]
    async fn test_ws_stream_orderbooks_filter() {
        let (tx, rx) = mpsc::unbounded_channel();
        let stream = WsStream::new(rx);

        tx.send(make_trade_message("BTCUSDT")).unwrap();
        tx.send(make_orderbook_message("BTCUSDT")).unwrap();
        tx.send(make_orderbook_message("ETHUSDT")).unwrap();
        drop(tx);

        let orderbooks: Vec<_> = stream.orderbooks().collect().await;
        assert_eq!(orderbooks.len(), 2);
    }

    #[tokio::test]
    async fn test_ws_stream_for_symbol() {
        let (tx, rx) = mpsc::unbounded_channel();
        let stream = WsStream::new(rx);

        tx.send(make_trade_message("BTCUSDT")).unwrap();
        tx.send(make_orderbook_message("BTCUSDT")).unwrap();
        tx.send(make_trade_message("ETHUSDT")).unwrap();
        tx.send(make_orderbook_message("ETHUSDT")).unwrap();
        drop(tx);

        let btc_messages: Vec<_> = stream.for_symbol("BTCUSDT").collect().await;
        assert_eq!(btc_messages.len(), 2);
    }

    #[tokio::test]
    async fn test_into_ws_stream() {
        let (tx, rx) = mpsc::unbounded_channel();
        let mut stream = rx.into_stream();

        tx.send(make_trade_message("BTCUSDT")).unwrap();
        drop(tx);

        let msg = stream.next().await;
        assert!(matches!(msg, Some(WsMessage::Trade(_))));
    }
}
