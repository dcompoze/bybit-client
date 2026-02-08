//! WebSocket Trade API integration tests.
//!
//! These tests connect to Bybit's testnet to verify WebSocket Trade API functionality.
//! Run with `cargo test --test ws_trade_integration -- --ignored`.
//!
//! Note: These tests require:
//! - `BYBIT_API_KEY` and `BYBIT_API_SECRET` environment variables.
//! - Network access to testnet.
//! - Sufficient testnet balance for order placement.

use std::time::Duration;

use std::time::{SystemTime, UNIX_EPOCH};

use bybit_client::ws::{
    AmendOrderRequest, CancelOrderRequest, CreateOrderRequest, WsTradeClient,
};
use bybit_client::types::{Category, OrderType, Side, TimeInForce};
use bybit_client::ClientConfig;
use tokio::time::sleep;

fn timestamp_ms() -> u128 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis()
}

/// Get API credentials from environment variables.
fn get_credentials() -> Option<(String, String)> {
    let key = std::env::var("BYBIT_API_KEY").ok()?;
    let secret = std::env::var("BYBIT_API_SECRET").ok()?;
    if key.is_empty() || secret.is_empty() {
        return None;
    }
    Some((key, secret))
}

/// Helper to create a testnet trade client.
async fn create_testnet_trade_client() -> Result<WsTradeClient, bybit_client::BybitError> {
    let (api_key, api_secret) = get_credentials()
        .expect("BYBIT_API_KEY and BYBIT_API_SECRET must be set for trade API tests");

    let config = ClientConfig::new(api_key, api_secret).testnet();
    WsTradeClient::connect(config).await
}

/// Test connecting to WebSocket Trade API.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_trade_client_connect() {
    let result = create_testnet_trade_client().await;
    assert!(result.is_ok(), "Failed to connect: {:?}", result.err());

    let client = result.unwrap();
    assert!(client.is_connected().await);

    client.disconnect().await;
    sleep(Duration::from_millis(100)).await;
    assert!(!client.is_connected().await);
}

/// Test creating a limit order.
#[tokio::test]
#[ignore = "requires API credentials, network access, and testnet balance"]
async fn test_create_limit_order() {
    let client = create_testnet_trade_client()
        .await
        .expect("Failed to connect");

    let request = CreateOrderRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        side: Side::Buy,
        order_type: OrderType::Limit,
        qty: "0.001".to_string(),
        price: Some("10000".to_string()), // Very low price to avoid fill
        time_in_force: Some(TimeInForce::GTC),
        order_link_id: Some(format!("test-{}", timestamp_ms())),
        is_leverage: None,
        position_idx: None,
        reduce_only: None,
        close_on_trigger: None,
        take_profit: None,
        stop_loss: None,
        tpsl_mode: None,
        market_unit: None,
    };

    let result = client.create_order(request).await;

    match result {
        Ok(order) => {
            assert!(!order.order_id.is_empty());
            println!("Created order: {}", order.order_id);

            let cancel_request = CancelOrderRequest {
                category: Category::Linear,
                symbol: "BTCUSDT".to_string(),
                order_id: Some(order.order_id.clone()),
                order_link_id: None,
            };

            let cancel_result = client.cancel_order(cancel_request).await;
            assert!(cancel_result.is_ok(), "Failed to cancel: {:?}", cancel_result.err());
        }
        Err(e) => {
            println!("Order creation failed (may be expected on testnet): {:?}", e);
        }
    }

    client.disconnect().await;
}

/// Test canceling a non-existent order (should fail gracefully).
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_cancel_nonexistent_order() {
    let client = create_testnet_trade_client()
        .await
        .expect("Failed to connect");

    let request = CancelOrderRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        order_id: Some("nonexistent-order-id-12345".to_string()),
        order_link_id: None,
    };

    let result = client.cancel_order(request).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    println!("Expected error for non-existent order: {:?}", err);

    client.disconnect().await;
}

/// Test amending an order.
#[tokio::test]
#[ignore = "requires API credentials, network access, and testnet balance"]
async fn test_amend_order() {
    let client = create_testnet_trade_client()
        .await
        .expect("Failed to connect");

    let order_link_id = format!("test-amend-{}", timestamp_ms());
    let create_request = CreateOrderRequest {
        category: Category::Linear,
        symbol: "BTCUSDT".to_string(),
        side: Side::Buy,
        order_type: OrderType::Limit,
        qty: "0.001".to_string(),
        price: Some("10000".to_string()),
        time_in_force: Some(TimeInForce::GTC),
        order_link_id: Some(order_link_id.clone()),
        is_leverage: None,
        position_idx: None,
        reduce_only: None,
        close_on_trigger: None,
        take_profit: None,
        stop_loss: None,
        tpsl_mode: None,
        market_unit: None,
    };

    let create_result = client.create_order(create_request).await;

    if let Ok(order) = create_result {
        let amend_request = AmendOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some(order.order_id.clone()),
            order_link_id: None,
            qty: Some("0.002".to_string()), // Change quantity
            price: Some("10100".to_string()), // Change price
            take_profit: None,
            stop_loss: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let amend_result = client.amend_order(amend_request).await;

        match amend_result {
            Ok(amended) => {
                assert_eq!(amended.order_id, order.order_id);
                println!("Amended order: {}", amended.order_id);
            }
            Err(e) => {
                println!("Amend failed (may be expected): {:?}", e);
            }
        }

        let cancel_request = CancelOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some(order.order_id),
            order_link_id: None,
        };
        let _ = client.cancel_order(cancel_request).await;
    } else {
        println!("Order creation failed, skipping amend test");
    }

    client.disconnect().await;
}

/// Test batch order operations.
#[tokio::test]
#[ignore = "requires API credentials, network access, and testnet balance"]
async fn test_batch_create_orders() {
    let client = create_testnet_trade_client()
        .await
        .expect("Failed to connect");

    let timestamp = timestamp_ms();

    let orders = vec![
        CreateOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            price: Some("10000".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            order_link_id: Some(format!("batch-1-{}", timestamp)),
            is_leverage: None,
            position_idx: None,
            reduce_only: None,
            close_on_trigger: None,
            take_profit: None,
            stop_loss: None,
            tpsl_mode: None,
            market_unit: None,
        },
        CreateOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            price: Some("10100".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            order_link_id: Some(format!("batch-2-{}", timestamp)),
            is_leverage: None,
            position_idx: None,
            reduce_only: None,
            close_on_trigger: None,
            take_profit: None,
            stop_loss: None,
            tpsl_mode: None,
            market_unit: None,
        },
    ];

    let result = client.batch_create_orders(Category::Linear, orders).await;

    match result {
        Ok(results) => {
            println!("Batch created {} orders", results.len());

            let cancel_requests: Vec<_> = results
                .iter()
                .map(|r| CancelOrderRequest {
                    category: Category::Linear,
                    symbol: r.symbol.clone(),
                    order_id: Some(r.order_id.clone()),
                    order_link_id: None,
                })
                .collect();

            if !cancel_requests.is_empty() {
                let _ = client.batch_cancel_orders(Category::Linear, cancel_requests).await;
            }
        }
        Err(e) => {
            println!("Batch create failed (may be expected on testnet): {:?}", e);
        }
    }

    client.disconnect().await;
}

/// Test that batch operations enforce the 10 order limit.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_batch_order_limit() {
    let client = create_testnet_trade_client()
        .await
        .expect("Failed to connect");

    let orders: Vec<CreateOrderRequest> = (0..11)
        .map(|i| CreateOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            price: Some(format!("{}", 10000 + i * 10)),
            time_in_force: Some(TimeInForce::GTC),
            order_link_id: Some(format!("limit-test-{}", i)),
            is_leverage: None,
            position_idx: None,
            reduce_only: None,
            close_on_trigger: None,
            take_profit: None,
            stop_loss: None,
            tpsl_mode: None,
            market_unit: None,
        })
        .collect();

    let result = client.batch_create_orders(Category::Linear, orders).await;

    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(format!("{:?}", err).contains("limit"));

    client.disconnect().await;
}

/// Test graceful disconnect.
#[tokio::test]
#[ignore = "requires API credentials and network access to testnet"]
async fn test_trade_client_disconnect() {
    let client = create_testnet_trade_client()
        .await
        .expect("Failed to connect");

    assert!(client.is_connected().await);

    client.disconnect().await;
    sleep(Duration::from_millis(200)).await;

    assert!(!client.is_connected().await);
}
