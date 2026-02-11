# bybit-client

A Rust client library for the [Bybit V5 API](https://bybit-exchange.github.io/docs/):

- Full REST API V5 coverage for market data, trading, positions, and account management
- WebSocket support for real-time public and private streams
- WebSocket Trade API for low-latency order management
- Local orderbook management with delta updates
- HMAC-SHA256 authentication
- Async/await support with Tokio
- Strongly typed request/response structures
- Automatic retry with exponential backoff
- Testnet and demo trading support

## Library

Public REST API client:

```rust
use bybit_client::{BybitClient, Category};
use bybit_client::api::market::GetTickersParams;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BybitClient::public_only()?;

    let params = GetTickersParams::new(Category::Linear).symbol("BTCUSDT");
    let tickers = client.market().get_tickers(&params).await?;

    for ticker in &tickers.list {
        println!("{}: ${}", ticker.symbol, ticker.last_price);
    }

    Ok(())
}
```

Authenticated client:

```rust
use bybit_client::{BybitClient, ClientConfig, Category, Side};
use bybit_client::types::trade::OrderParams;

#[tokio::main]
async fn main() -> Result<()> {
    let client = BybitClient::new("your_api_key", "your_api_secret")?;

    let client = BybitClient::with_config(
        ClientConfig::new("api_key", "api_secret").testnet()
    )?;

    let params = OrderParams::market(Category::Linear, "BTCUSDT", Side::Buy, "0.001");
    let result = client.trade().submit_order(&params).await?;
    println!("Order ID: {}", result.order_id);

    Ok(())
}
```

WebSocket streams:

```rust
use bybit_client::ws::{WsClient, WsChannel, WsMessage};

#[tokio::main]
async fn main() -> Result<()> {
    let (client, mut receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;

    client.subscribe(&["orderbook.50.BTCUSDT", "publicTrade.BTCUSDT"]).await?;

    while let Some(msg) = receiver.recv().await {
        match msg {
            WsMessage::Orderbook(ob) => {
                println!("Orderbook: {} bids, {} asks",
                    ob.data.bids.len(), ob.data.asks.len());
            }
            WsMessage::Trade(trades) => {
                for trade in &trades.data {
                    println!("Trade: {} {} @ {}",
                        trade.symbol, trade.size, trade.price);
                }
            }
            _ => {}
        }
    }

    Ok(())
}
```

Local orderbook:

```rust
use bybit_client::ws::{WsClient, WsChannel, WsMessage, LocalOrderbook};

#[tokio::main]
async fn main() -> Result<()> {
    let (client, mut receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;
    client.subscribe(&["orderbook.50.BTCUSDT"]).await?;

    let mut orderbook = LocalOrderbook::new("BTCUSDT");

    while let Some(msg) = receiver.recv().await {
        if let WsMessage::Orderbook(update) = msg {
            orderbook.apply_update(&update)?;

            if let (Some(bid), Some(ask)) = (orderbook.best_bid(), orderbook.best_ask()) {
                println!("Best bid: {} @ {}, Best ask: {} @ {}",
                    bid.size, bid.price, ask.size, ask.price);
                println!("Spread: {:.4}", orderbook.spread().unwrap_or(0.0));
            }
        }
    }

    Ok(())
}
```

Private WebSocket streams:

```rust
use bybit_client::ws::{WsClient, WsMessage};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, mut receiver) = WsClient::connect_private(
        "your_api_key",
        "your_api_secret",
    ).await?;

    client.subscribe(&["position", "order", "execution"]).await?;

    while let Some(msg) = receiver.recv().await {
        match msg {
            WsMessage::Position(pos) => println!("Position update: {:?}", pos),
            WsMessage::Order(order) => println!("Order update: {:?}", order),
            WsMessage::Execution(exec) => println!("Execution: {:?}", exec),
            _ => {}
        }
    }

    Ok(())
}
```

## API coverage

REST API services:

| Service | Description | Authentication |
|---------|-------------|----------------|
| `market()` | Market data (tickers, orderbook, klines, trades) | Public |
| `trade()` | Order placement and management | Required |
| `position()` | Position information and management | Required |
| `account()` | Wallet balance and account info | Required |

Public WebSocket topics:

- `orderbook.{depth}.{symbol}` - Orderbook (depth: 1, 50, 200, 500)
- `publicTrade.{symbol}` - Public trades
- `tickers.{symbol}` - Ticker updates
- `kline.{interval}.{symbol}` - Kline/candlestick data
- `liquidation.{symbol}` - Liquidation events

Private WebSocket topics:

- `position` - Position updates
- `execution` - Execution updates
- `order` - Order updates
- `wallet` - Wallet balance updates

## Configuration

```rust
use bybit_client::{ClientConfig, Environment, ApiRegion};

let config = ClientConfig::new("api_key", "api_secret")
    .testnet()                    // Use testnet environment
    .recv_window(10000)           // Set recv_window to 10 seconds
    .debug(true)                  // Enable debug logging
    .timeout_ms(30000);           // Set request timeout to 30 seconds
```

## Environment variables

You can set credentials via environment variables:

```bash
export BYBIT_API_KEY=""
export BYBIT_API_SECRET=""
```
