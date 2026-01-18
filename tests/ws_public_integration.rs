//! WebSocket public stream integration tests.
//!
//! These tests connect to Bybit's testnet to verify WebSocket functionality.
//! Run with: `cargo test --test ws_public_integration -- --ignored`
//!
//! Note: These tests require network access and may occasionally fail due to
//! network conditions or testnet availability.

use std::time::Duration;

use bybit_client::ws::{
    IntoWsStream, LocalOrderbook, WsChannel, WsClient, WsMessage, WsStream,
};
use futures::StreamExt;
use tokio::time::timeout;

/// Helper to create a testnet public client.
async fn create_testnet_client(
    channel: WsChannel,
) -> Result<(WsClient, tokio::sync::mpsc::UnboundedReceiver<WsMessage>), bybit_client::BybitError> {
    let config = bybit_client::ClientConfig::public_only().testnet();
    WsClient::connect_with_config(config, channel).await
}

/// Test basic connection to public linear stream.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_connect_public_linear() {
    let result = create_testnet_client(WsChannel::PublicLinear).await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let (client, _rx) = result.unwrap();
    assert!(client.is_connected());

    // Allow some time to ensure connection is stable
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert!(client.is_connected());

    client.disconnect();
}

/// Test basic connection to public spot stream.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_connect_public_spot() {
    let result = create_testnet_client(WsChannel::PublicSpot).await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let (client, _rx) = result.unwrap();
    assert!(client.is_connected());
    client.disconnect();
}

/// Test subscribing to orderbook stream.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_subscribe_orderbook() {
    let (client, mut rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    // Subscribe to BTCUSDT orderbook
    client
        .subscribe(&["orderbook.50.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    // Wait for first message (should be a snapshot)
    let msg = timeout(Duration::from_secs(10), rx.recv())
        .await
        .expect("Timeout waiting for message")
        .expect("Channel closed");

    // Verify we got an orderbook or operation response
    match msg {
        WsMessage::Orderbook(ob) => {
            assert_eq!(ob.data.symbol, "BTCUSDT");
            assert!(ob.update_type == "snapshot" || ob.update_type == "delta");
            assert!(!ob.data.bids.is_empty() || !ob.data.asks.is_empty());
        }
        WsMessage::OperationResponse(resp) => {
            assert!(resp.success, "Subscription failed: {:?}", resp.ret_msg);
            // After operation response, next should be orderbook data
            let next = timeout(Duration::from_secs(10), rx.recv())
                .await
                .expect("Timeout")
                .expect("Channel closed");
            assert!(matches!(next, WsMessage::Orderbook(_)));
        }
        _ => {
            // First message might be pong or other, keep receiving
        }
    }

    client.disconnect();
}

/// Test subscribing to trade stream.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_subscribe_trades() {
    let (client, rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    client
        .subscribe(&["publicTrade.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    let mut stream = rx.into_stream();

    // Wait for either a subscription response or trade data
    let mut received_trade = false;
    let mut received_sub_response = false;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(30);

    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(5), stream.next()).await;

        match msg {
            Ok(Some(WsMessage::Trade(t))) => {
                assert!(!t.data.is_empty());
                assert_eq!(t.data[0].symbol, "BTCUSDT");
                received_trade = true;
                break;
            }
            Ok(Some(WsMessage::OperationResponse(r))) => {
                assert!(r.success);
                received_sub_response = true;
            }
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(_) => {
                if received_sub_response {
                    // Trades might not happen immediately on testnet
                    break;
                }
            }
        }
    }

    assert!(
        received_trade || received_sub_response,
        "Did not receive trade data or subscription confirmation"
    );

    client.disconnect();
}

/// Test subscribing to ticker stream.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_subscribe_ticker() {
    let (client, mut rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    client
        .subscribe(&["tickers.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    // Wait for ticker data (should come quickly as it updates frequently)
    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    let mut received_ticker = false;

    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(5), rx.recv()).await;

        match msg {
            Ok(Some(WsMessage::Ticker(t))) => {
                assert_eq!(t.data.symbol, "BTCUSDT");
                assert!(t.data.last_price.is_some());
                received_ticker = true;
                break;
            }
            Ok(Some(WsMessage::OperationResponse(r))) => {
                assert!(r.success, "Subscription failed: {:?}", r.ret_msg);
            }
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(_) => continue,
        }
    }

    assert!(received_ticker, "Did not receive ticker data");
    client.disconnect();
}

/// Test subscribing to multiple topics at once.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_subscribe_multiple_topics() {
    let (client, mut rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    client
        .subscribe(&["orderbook.50.BTCUSDT", "tickers.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    let mut received_orderbook = false;
    let mut received_ticker = false;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);

    while tokio::time::Instant::now() < deadline && !(received_orderbook && received_ticker) {
        let msg = timeout(Duration::from_secs(5), rx.recv()).await;

        match msg {
            Ok(Some(WsMessage::Orderbook(_))) => received_orderbook = true,
            Ok(Some(WsMessage::Ticker(_))) => received_ticker = true,
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(_) => continue,
        }
    }

    assert!(
        received_orderbook,
        "Did not receive orderbook data within timeout"
    );
    assert!(received_ticker, "Did not receive ticker data within timeout");

    client.disconnect();
}

/// Test unsubscribing from a topic.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_unsubscribe() {
    let (client, mut rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    // Subscribe first
    client
        .subscribe(&["tickers.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    // Wait for subscription confirmation
    let mut subscribed = false;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        match msg {
            Ok(Some(WsMessage::Ticker(_))) | Ok(Some(WsMessage::OperationResponse(_))) => {
                subscribed = true;
                break;
            }
            _ => continue,
        }
    }
    assert!(subscribed, "Failed to confirm subscription");

    // Unsubscribe
    client
        .unsubscribe(&["tickers.BTCUSDT"])
        .await
        .expect("Failed to unsubscribe");

    // Verify topics list is empty
    let topics = client.subscribed_topics().await;
    assert!(topics.is_empty(), "Topics not cleared after unsubscribe");

    client.disconnect();
}

/// Test LocalOrderbook with real orderbook data.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_local_orderbook_with_real_data() {
    let (client, mut rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    client
        .subscribe(&["orderbook.50.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    let mut orderbook = LocalOrderbook::new("BTCUSDT");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(15);
    let mut updates_applied = 0;

    while tokio::time::Instant::now() < deadline && updates_applied < 5 {
        let msg = timeout(Duration::from_secs(5), rx.recv()).await;

        match msg {
            Ok(Some(WsMessage::Orderbook(ob))) => {
                let result = orderbook.apply_update(&ob);
                assert!(result.is_ok(), "Failed to apply update: {:?}", result.err());
                updates_applied += 1;

                if orderbook.is_initialized() {
                    // Verify orderbook state
                    assert!(orderbook.bid_levels() > 0, "No bid levels after update");
                    assert!(orderbook.ask_levels() > 0, "No ask levels after update");

                    let best_bid = orderbook.best_bid().expect("No best bid");
                    let best_ask = orderbook.best_ask().expect("No best ask");

                    // Best bid should be less than best ask
                    assert!(
                        best_bid.price_f64 < best_ask.price_f64,
                        "Best bid {} >= best ask {}",
                        best_bid.price,
                        best_ask.price
                    );

                    // Spread should be positive
                    let spread = orderbook.spread().expect("No spread");
                    assert!(spread > 0.0, "Spread should be positive");
                }
            }
            Ok(Some(_)) => continue,
            Ok(None) => break,
            Err(_) => continue,
        }
    }

    assert!(updates_applied > 0, "No orderbook updates received");
    assert!(orderbook.is_initialized(), "Orderbook not initialized");

    client.disconnect();
}

/// Test WsStream filtering with real data.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_ws_stream_filtering() {
    use std::pin::pin;

    let (client, rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    client
        .subscribe(&["orderbook.50.BTCUSDT", "tickers.BTCUSDT"])
        .await
        .expect("Failed to subscribe");

    let stream = WsStream::new(rx);

    // Filter only orderbook messages
    let mut orderbooks = pin!(stream.orderbooks());

    // Should receive orderbook data
    let msg = timeout(Duration::from_secs(10), orderbooks.next())
        .await
        .expect("Timeout waiting for orderbook");

    assert!(msg.is_some());
    let ob = msg.unwrap();
    assert_eq!(ob.data.symbol, "BTCUSDT");

    client.disconnect();
}

/// Test ping/pong heartbeat is working.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_heartbeat() {
    let (client, mut rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    // Wait long enough to see heartbeat activity (ping is every 20s)
    // We'll just verify the connection stays alive
    let mut pong_received = false;

    let deadline = tokio::time::Instant::now() + Duration::from_secs(25);

    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(5), rx.recv()).await;

        if let Ok(Some(WsMessage::Pong(_))) = msg {
            pong_received = true;
            break;
        }

        // Connection should remain open
        assert!(client.is_connected(), "Connection lost during heartbeat test");
    }

    assert!(pong_received, "Did not receive pong response");
    client.disconnect();
}

/// Test that the client handles disconnection gracefully.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_graceful_disconnect() {
    let (client, _rx) = create_testnet_client(WsChannel::PublicLinear)
        .await
        .expect("Failed to connect");

    assert!(client.is_connected());

    client.disconnect();

    // Give some time for disconnect to complete
    tokio::time::sleep(Duration::from_millis(500)).await;

    // After disconnect, is_connected should return false
    assert!(!client.is_connected());
}
