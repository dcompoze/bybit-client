//! Wiremock integration tests for the market service.
//!
//! These tests verify request paths, query parameters, and response parsing
//! against a mock HTTP server.

use bybit_client::rest::market::{
    GetFullOrderbookParams, GetOrderbookParams, GetRpiOrderbookParams, GetTickersParams,
};
use bybit_client::{BybitClient, Category, ClientConfig};
use wiremock::matchers::{method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

fn envelope(result: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "retCode": 0,
        "retMsg": "OK",
        "result": result,
        "retExtInfo": {},
        "time": 1755900000000u64
    })
}

async fn public_client(server: &MockServer) -> BybitClient {
    let config = ClientConfig::public_only().base_url(server.uri());
    match BybitClient::with_config(config) {
        Ok(client) => client,
        Err(err) => panic!("Failed to build client: {}", err),
    }
}

#[tokio::test]
async fn test_get_tickers_path_and_parsing() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/market/tickers"))
        .and(query_param("category", "linear"))
        .and(query_param("symbol", "BTCUSDT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "linear",
            "list": [{
                "symbol": "BTCUSDT",
                "lastPrice": "50000.5",
                "openInterestLong": "100",
                "openInterestShort": "90"
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetTickersParams::new(Category::Linear).symbol("BTCUSDT");
    let result = match client.market().get_tickers(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].symbol, "BTCUSDT");
    assert_eq!(result.list[0].last_price.as_deref(), Some("50000.5"));
    assert_eq!(result.list[0].open_interest_long.as_deref(), Some("100"));
    assert_eq!(result.list[0].open_interest_short.as_deref(), Some("90"));
}

#[tokio::test]
async fn test_get_orderbook_parsing() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/market/orderbook"))
        .and(query_param("category", "spot"))
        .and(query_param("symbol", "BTCUSDT"))
        .and(query_param("limit", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "s": "BTCUSDT",
            "b": [["49999.9", "1.5"]],
            "a": [["50000.1", "2.0"]],
            "ts": 1755900000000u64,
            "u": 12345,
            "seq": 999,
            "cts": 1755899999999u64
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetOrderbookParams::new(Category::Spot, "BTCUSDT").limit(50);
    let orderbook = match client.market().get_orderbook(&params).await {
        Ok(orderbook) => orderbook,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(orderbook.symbol, "BTCUSDT");
    assert_eq!(orderbook.bids[0].price, "49999.9");
    assert_eq!(orderbook.asks[0].size, "2.0");
    assert_eq!(orderbook.cts, Some(1755899999999));
}

#[tokio::test]
async fn test_get_full_orderbook_path() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/market/full_orderbook"))
        .and(query_param("category", "linear"))
        .and(query_param("symbol", "ETHUSDT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "s": "ETHUSDT",
            "b": [["3000.1", "10"]],
            "a": [["3000.2", "12"]],
            "ts": 1755900000000u64,
            "u": 1
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetFullOrderbookParams::new(Category::Linear, "ETHUSDT");
    let orderbook = match client.market().get_full_orderbook(&params).await {
        Ok(orderbook) => orderbook,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(orderbook.symbol, "ETHUSDT");
    assert_eq!(orderbook.bids.len(), 1);
}

#[tokio::test]
async fn test_get_rpi_orderbook_levels() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/market/rpi_orderbook"))
        .and(query_param("category", "spot"))
        .and(query_param("symbol", "BTCUSDT"))
        .and(query_param("limit", "50"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "s": "BTCUSDT",
            "b": [["49999.9", "1.5", "0.5"]],
            "a": [["50000.1", "2.0", "0.7"]],
            "ts": 1755900000000u64,
            "u": 42,
            "seq": 100,
            "cts": 1755899999999u64
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetRpiOrderbookParams::new(Category::Spot, "BTCUSDT", 50);
    let orderbook = match client.market().get_rpi_orderbook(&params).await {
        Ok(orderbook) => orderbook,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(orderbook.symbol, "BTCUSDT");
    assert_eq!(orderbook.bids[0].price, "49999.9");
    assert_eq!(orderbook.bids[0].size, "1.5");
    assert_eq!(orderbook.bids[0].rpi_size, "0.5");
    assert_eq!(orderbook.asks[0].rpi_size, "0.7");
}

#[tokio::test]
async fn test_api_error_is_surfaced() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/market/tickers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "retCode": 10001,
            "retMsg": "params error",
            "result": {},
            "retExtInfo": {},
            "time": 1755900000000u64
        })))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetTickersParams::new(Category::Linear);
    let result = client.market().get_tickers(&params).await;

    assert!(result.is_err());
}
