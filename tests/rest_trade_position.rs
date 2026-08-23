//! Wiremock integration tests for the trade and position services.
//!
//! These tests verify request paths, signing headers, body serialization,
//! and response parsing against a mock HTTP server.

use bybit_client::types::position::{
    GetClosedOptionsPositionsParams, GetPositionInfoParams, MovePositionItem, MovePositionsParams,
    SetLeverageParams, SetTpslModeParams,
};
use bybit_client::types::trade::{
    BatchOrderParams, CancelOrderParams, GetOpenOrdersParams, OrderParams, SetDcpParams,
};
use bybit_client::{BybitClient, Category, ClientConfig, PositionIdx, Side};
use wiremock::matchers::{body_partial_json, header, header_exists, method, path, query_param};
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

async fn signed_client(server: &MockServer) -> BybitClient {
    let config = ClientConfig::new("test_key", "test_secret").base_url(server.uri());
    match BybitClient::with_config(config) {
        Ok(client) => client,
        Err(err) => panic!("Failed to build client: {}", err),
    }
}

fn order_json(order_id: &str) -> serde_json::Value {
    serde_json::json!({
        "orderId": order_id,
        "orderLinkId": "link-1",
        "symbol": "BTCUSDT",
        "price": "50000",
        "qty": "0.001",
        "side": "Buy",
        "orderStatus": "New",
        "cumExecQty": "0",
        "cumExecValue": "0",
        "cumExecFee": "0",
        "orderType": "Limit",
        "timeInForce": "GTC",
        "createdTime": "1755900000000",
        "updatedTime": "1755900000000",
        "category": "linear",
        "parentOrderLinkId": "parent-1",
        "smpGroup": "3"
    })
}

#[tokio::test]
async fn test_submit_order_signed_and_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/order/create"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .and(body_partial_json(serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "side": "Buy",
            "orderType": "Limit",
            "qty": "0.001",
            "price": "50000"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "order-123",
            "orderLinkId": "link-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = OrderParams::limit(Category::Linear, "BTCUSDT", Side::Buy, "0.001", "50000");
    let result = match client.trade().submit_order(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.order_id, "order-123");
}

#[tokio::test]
async fn test_submit_order_position_idx_serializes_as_integer() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/order/create"))
        .and(body_partial_json(serde_json::json!({
            "positionIdx": 1
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "order-124",
            "orderLinkId": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = OrderParams::market(Category::Linear, "BTCUSDT", Side::Buy, "0.001")
        .position_idx(PositionIdx::HedgeBuy);
    let result = match client.trade().submit_order(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.order_id, "order-124");
}

#[tokio::test]
async fn test_cancel_order() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/order/cancel"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "orderId": "order-123"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "order-123",
            "orderLinkId": "link-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = CancelOrderParams::by_order_id(Category::Linear, "BTCUSDT", "order-123");
    let result = match client.trade().cancel_order(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.order_id, "order-123");
}

#[tokio::test]
async fn test_get_open_orders_parses_new_fields() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/order/realtime"))
        .and(query_param("category", "linear"))
        .and(query_param("symbol", "BTCUSDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "linear",
            "list": [order_json("order-1")],
            "nextPageCursor": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetOpenOrdersParams::new(Category::Linear).symbol("BTCUSDT");
    let result = match client.trade().get_open_orders(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].order_id, "order-1");
    assert_eq!(
        result.list[0].parent_order_link_id.as_deref(),
        Some("parent-1")
    );
    assert_eq!(result.list[0].smp_group.as_deref(), Some("3"));
}

#[tokio::test]
async fn test_get_open_orders_smp_group_integer_regression() {
    let server = MockServer::start().await;

    let mut order = order_json("order-2");
    order["smpGroup"] = serde_json::json!(7);

    Mock::given(method("GET"))
        .and(path("/v5/order/realtime"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "linear",
            "list": [order],
            "nextPageCursor": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetOpenOrdersParams::new(Category::Linear);
    let result = match client.trade().get_open_orders(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list[0].smp_group.as_deref(), Some("7"));
}

#[tokio::test]
async fn test_batch_submit_orders() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/order/create-batch"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "category": "linear",
            "request": [
                {"symbol": "BTCUSDT", "side": "Buy", "orderType": "Limit", "qty": "0.001"},
                {"symbol": "ETHUSDT", "side": "Sell", "orderType": "Market", "qty": "0.1"}
            ]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "list": [
                {
                    "category": "linear",
                    "symbol": "BTCUSDT",
                    "orderId": "b-1",
                    "orderLinkId": "",
                    "createAt": "1755900000000"
                },
                {
                    "category": "linear",
                    "symbol": "ETHUSDT",
                    "orderId": "b-2",
                    "orderLinkId": "",
                    "createAt": "1755900000000"
                }
            ]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let orders = vec![
        BatchOrderParams::limit("BTCUSDT", Side::Buy, "0.001", "50000"),
        BatchOrderParams::market("ETHUSDT", Side::Sell, "0.1"),
    ];
    let result = match client
        .trade()
        .batch_submit_orders(Category::Linear, orders)
        .await
    {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 2);
    assert_eq!(result.list[0].order_id, "b-1");
    assert_eq!(result.list[1].symbol, "ETHUSDT");
}

#[tokio::test]
async fn test_pre_check_order() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/order/pre-check"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "pre-1",
            "orderLinkId": "",
            "preImrE4": 100,
            "preMmrE4": 50,
            "postImrE4": 120,
            "postMmrE4": 60
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = OrderParams::limit(Category::Linear, "BTCUSDT", Side::Buy, "0.001", "50000");
    let result = match client.trade().pre_check_order(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.pre_imr_e4, Some(100));
    assert_eq!(result.post_mmr_e4, Some(60));
}

#[tokio::test]
async fn test_set_disconnect_cancel_all() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/order/disconnected-cancel-all"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "timeWindow": 40,
            "product": "DERIVATIVES"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({}))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = SetDcpParams::new(40).product("DERIVATIVES");
    if let Err(err) = client.trade().set_disconnect_cancel_all(&params).await {
        panic!("Request failed: {}", err);
    }
}

#[tokio::test]
async fn test_get_position_info_parses_new_fields() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/position/list"))
        .and(query_param("category", "linear"))
        .and(query_param("symbol", "BTCUSDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "linear",
            "list": [{
                "positionIdx": 0,
                "symbol": "BTCUSDT",
                "side": "Buy",
                "size": "0.5",
                "avgPrice": "48000",
                "positionValue": "24000",
                "tradeMode": 0,
                "leverage": "10",
                "markPrice": "50000",
                "unrealisedPnl": "1000",
                "cumRealisedPnl": "250",
                "createdTime": "1755900000000",
                "updatedTime": "1755900000000",
                "breakEvenPrice": "48100.5",
                "openTime": "1755890000000"
            }],
            "nextPageCursor": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetPositionInfoParams::new(Category::Linear).symbol("BTCUSDT");
    let result = match client.position().get_position_info(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].break_even_price.as_deref(), Some("48100.5"));
    assert_eq!(result.list[0].open_time.as_deref(), Some("1755890000000"));
}

#[tokio::test]
async fn test_set_leverage() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/position/set-leverage"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "buyLeverage": "10",
            "sellLeverage": "10"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({}))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = SetLeverageParams::uniform(Category::Linear, "BTCUSDT", "10");
    if let Err(err) = client.position().set_leverage(&params).await {
        panic!("Request failed: {}", err);
    }
}

#[tokio::test]
async fn test_set_tpsl_mode() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/position/set-tpsl-mode"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "category": "linear",
            "symbol": "BTCUSDT",
            "tpSlMode": "Partial"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "tpSlMode": "Partial"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = SetTpslModeParams::partial(Category::Linear, "BTCUSDT");
    let result = match client.position().set_tpsl_mode(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.tp_sl_mode, "Partial");
}

#[tokio::test]
async fn test_move_positions() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/position/move-positions"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "fromUid": "100001",
            "toUid": "100002",
            "list": [{
                "category": "linear",
                "symbol": "BTCUSDT",
                "price": "50000",
                "side": "Buy",
                "qty": "0.1"
            }]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "blockTradeId": "bt-1",
            "status": "Processing",
            "rejectParty": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let item = MovePositionItem::new(Category::Linear, "BTCUSDT", "50000", Side::Buy, "0.1");
    let params = MovePositionsParams::new("100001", "100002", vec![item]);
    let result = match client.position().move_positions(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.block_trade_id, "bt-1");
    assert_eq!(result.status, "Processing");
}

#[tokio::test]
async fn test_get_closed_options_positions() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/position/get-closed-positions"))
        .and(query_param("category", "option"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "option",
            "list": [{
                "symbol": "BTC-27JUN26-50000-C",
                "side": "Buy",
                "qty": "1",
                "closeTime": 1755900000000i64,
                "openTime": 1755800000000i64,
                "avgExitPrice": "1200",
                "avgEntryPrice": "1000",
                "totalPnl": "200"
            }],
            "nextPageCursor": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetClosedOptionsPositionsParams::new();
    let result = match client.position().get_closed_options_positions(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].symbol, "BTC-27JUN26-50000-C");
    assert_eq!(result.list[0].total_pnl.as_deref(), Some("200"));
}
