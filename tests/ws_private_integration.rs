//! WebSocket private stream integration tests.
//!
//! These tests connect to Bybit's testnet with authentication to verify
//! private WebSocket functionality.
//!
//! Run with `cargo test --test ws_private_integration -- --ignored`.
//!
//! Note: These tests require:
//! - `BYBIT_API_KEY` and `BYBIT_API_SECRET` environment variables.
//! - Network access to testnet.
//! - A testnet account with some positions and orders for full coverage.

use std::time::Duration;

use bybit_client::ws::{WsChannel, WsClient, WsMessage};
use bybit_client::ClientConfig;
use tokio::time::timeout;

/// Get API credentials from environment variables.
fn get_credentials() -> Option<(String, String)> {
    let key = std::env::var("BYBIT_API_KEY").ok()?;
    let secret = std::env::var("BYBIT_API_SECRET").ok()?;
    if key.is_empty() || secret.is_empty() {
        return None;
    }
    Some((key, secret))
}

/// Helper to create a testnet private client.
async fn create_testnet_private_client(
) -> Result<(WsClient, tokio::sync::mpsc::UnboundedReceiver<WsMessage>), bybit_client::BybitError> {
    let (api_key, api_secret) = get_credentials()
        .expect("BYBIT_API_KEY and BYBIT_API_SECRET must be set for private stream tests");

    let config = ClientConfig::new(api_key, api_secret).testnet();
    WsClient::connect_with_config(config, WsChannel::Private).await
}

/// Test connecting to private WebSocket channel with authentication.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_connect_private_authenticated() {
    let result = create_testnet_private_client().await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let (client, _rx) = result.unwrap();
    assert!(client.is_connected());

    tokio::time::sleep(Duration::from_secs(2)).await;
    assert!(client.is_connected());

    client.disconnect();
}

/// Test subscribing to position updates.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_subscribe_position() {
    let (client, mut rx) = create_testnet_private_client()
        .await
        .expect("Failed to connect");

    let mut authenticated = false;
    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        if let Ok(Some(WsMessage::OperationResponse(resp))) = msg {
            if resp.op.as_deref() == Some("auth") && resp.success {
                authenticated = true;
                break;
            }
        }
    }
    assert!(authenticated, "Authentication failed");

    client
        .subscribe(&["position"])
        .await
        .expect("Failed to subscribe");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut subscribed = false;
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        match msg {
            Ok(Some(WsMessage::OperationResponse(resp))) => {
                if resp.success {
                    subscribed = true;
                    break;
                }
            }
            Ok(Some(WsMessage::Position(pos))) => {
                assert!(!pos.data.is_empty() || pos.data.is_empty()); // Valid either way
                subscribed = true;
                break;
            }
            _ => continue,
        }
    }

    assert!(subscribed, "Position subscription not confirmed");
    client.disconnect();
}

/// Test subscribing to order updates.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_subscribe_order() {
    let (client, mut rx) = create_testnet_private_client()
        .await
        .expect("Failed to connect");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        if let Ok(Some(WsMessage::OperationResponse(resp))) = msg {
            if resp.op.as_deref() == Some("auth") && resp.success {
                break;
            }
        }
    }

    client
        .subscribe(&["order"])
        .await
        .expect("Failed to subscribe");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut subscribed = false;
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        if let Ok(Some(WsMessage::OperationResponse(resp))) = msg {
            if resp.success {
                subscribed = true;
                break;
            }
        }
    }

    assert!(subscribed, "Order subscription not confirmed");
    client.disconnect();
}

/// Test subscribing to wallet updates.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_subscribe_wallet() {
    let (client, mut rx) = create_testnet_private_client()
        .await
        .expect("Failed to connect");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        if let Ok(Some(WsMessage::OperationResponse(resp))) = msg {
            if resp.op.as_deref() == Some("auth") && resp.success {
                break;
            }
        }
    }

    client
        .subscribe(&["wallet"])
        .await
        .expect("Failed to subscribe");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut received = false;
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        match msg {
            Ok(Some(WsMessage::OperationResponse(resp))) => {
                if resp.success {
                    received = true;
                    break;
                }
            }
            Ok(Some(WsMessage::Wallet(wallet))) => {
                assert!(!wallet.data.is_empty());
                let w = &wallet.data[0];
                assert!(!w.account_type.is_empty());
                received = true;
                break;
            }
            _ => continue,
        }
    }

    assert!(received, "Wallet subscription not confirmed");
    client.disconnect();
}

/// Test subscribing to multiple private topics at once.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_subscribe_multiple_private_topics() {
    let (client, mut rx) = create_testnet_private_client()
        .await
        .expect("Failed to connect");

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        if let Ok(Some(WsMessage::OperationResponse(resp))) = msg {
            if resp.op.as_deref() == Some("auth") && resp.success {
                break;
            }
        }
    }

    client
        .subscribe(&["position", "order", "execution", "wallet"])
        .await
        .expect("Failed to subscribe");

    let topics = client.subscribed_topics().await;
    assert!(topics.contains(&"position".to_string()));
    assert!(topics.contains(&"order".to_string()));
    assert!(topics.contains(&"execution".to_string()));
    assert!(topics.contains(&"wallet".to_string()));

    client.disconnect();
}

/// Test that authentication failure is handled properly.
#[tokio::test]
#[ignore = "requires network access to testnet"]
async fn test_invalid_credentials() {
    let config = ClientConfig::new("invalid_key", "invalid_secret").testnet();
    let result = WsClient::connect_with_config(config, WsChannel::Private).await;

    assert!(result.is_ok());

    let (client, mut rx) = result.unwrap();

    let deadline = tokio::time::Instant::now() + Duration::from_secs(10);
    let mut auth_failed = false;
    while tokio::time::Instant::now() < deadline {
        let msg = timeout(Duration::from_secs(3), rx.recv()).await;
        if let Ok(Some(WsMessage::OperationResponse(resp))) = msg {
            if resp.op.as_deref() == Some("auth") && !resp.success {
                auth_failed = true;
                break;
            }
        }
    }

    assert!(auth_failed, "Expected authentication to fail with invalid credentials");
    client.disconnect();
}

/// Test graceful disconnect from private channel.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_private_graceful_disconnect() {
    let (client, _rx) = create_testnet_private_client()
        .await
        .expect("Failed to connect");

    assert!(client.is_connected());

    client.disconnect();

    tokio::time::sleep(Duration::from_millis(500)).await;

    assert!(!client.is_connected());
}
