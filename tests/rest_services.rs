//! Wiremock integration tests for the newer REST services.
//!
//! These tests verify request paths, auth headers, request bodies,
//! and response parsing against a mock HTTP server.

use bybit_client::types::crypto_loan::{FlexibleBorrowParams, GetCollateralDataParams};
use bybit_client::types::earn::{GetEarnProductParams, PlaceEarnOrderParams};
use bybit_client::types::pre_upgrade::PreUpgradeOrderHistoryParams;
use bybit_client::types::spot_leverage_token::{
    GetLeverageTokenInfoParams, PurchaseLeverageTokenParams,
};
use bybit_client::types::spot_margin::SwitchSpotMarginModeParams;
use bybit_client::types::spread::{GetSpreadInstrumentsInfoParams, PlaceSpreadOrderParams};
use bybit_client::types::user::CreateSubMemberParams;
use bybit_client::{BybitClient, Category, ClientConfig, Side};
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

async fn public_client(server: &MockServer) -> BybitClient {
    let config = ClientConfig::public_only().base_url(server.uri());
    match BybitClient::with_config(config) {
        Ok(client) => client,
        Err(err) => panic!("Failed to build client: {}", err),
    }
}

#[tokio::test]
async fn test_user_create_sub_member() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/user/create-sub-member"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "username": "subuser01",
            "memberType": 1
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "uid": "12345678",
            "username": "subuser01",
            "memberType": 1,
            "status": 1
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = CreateSubMemberParams::new("subuser01");
    let result = match client.user().create_sub_member(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.uid, "12345678");
    assert_eq!(result.username, "subuser01");
}

#[tokio::test]
async fn test_user_get_api_key_info() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/user/query-api"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "id": "13770661",
            "note": "bot key",
            "apiKey": "test_key",
            "readOnly": 0,
            "ips": ["*"],
            "type": 1
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let info = match client.user().get_api_key_info().await {
        Ok(info) => info,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(info.id, "13770661");
    assert_eq!(info.api_key, "test_key");
    assert_eq!(info.note.as_deref(), Some("bot key"));
}

#[tokio::test]
async fn test_pre_upgrade_order_history() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/pre-upgrade/order/history"))
        .and(query_param("category", "linear"))
        .and(query_param("symbol", "BTCUSDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "linear",
            "list": [{
                "orderId": "order-1",
                "orderLinkId": "link-1",
                "symbol": "BTCUSDT",
                "price": "50000",
                "qty": "0.1",
                "side": "Buy",
                "orderStatus": "Filled",
                "cumExecQty": "0.1",
                "cumExecValue": "5000",
                "cumExecFee": "2.75",
                "orderType": "Limit",
                "createdTime": "1672281744610",
                "updatedTime": "1672281744612"
            }],
            "nextPageCursor": "cursor-abc"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = PreUpgradeOrderHistoryParams::new(Category::Linear).symbol("BTCUSDT");
    let result = match client.pre_upgrade().get_order_history(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].order_id, "order-1");
    assert_eq!(result.next_page_cursor.as_deref(), Some("cursor-abc"));
}

#[tokio::test]
async fn test_spot_margin_switch_mode() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/spot-margin-trade/switch-mode"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "spotMarginMode": "1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "spotMarginMode": "1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let result = match client
        .spot_margin()
        .switch_mode(&SwitchSpotMarginModeParams::on())
        .await
    {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.spot_margin_mode, "1");
}

#[tokio::test]
async fn test_spot_margin_get_state() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/spot-margin-trade/state"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "spotLeverage": "10",
            "spotMarginMode": "1",
            "effectiveLeverage": "1.5"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let state = match client.spot_margin().get_state().await {
        Ok(state) => state,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(state.spot_leverage, "10");
    assert_eq!(state.spot_margin_mode, "1");
    assert_eq!(state.effective_leverage.as_deref(), Some("1.5"));
}

#[tokio::test]
async fn test_leverage_token_info_is_public() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/spot-lever-token/info"))
        .and(query_param("ltCoin", "BTC3L"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "list": [{
                "ltCoin": "BTC3L",
                "ltName": "3X Long",
                "maxPurchase": "5000",
                "minPurchase": "100",
                "maxPurchaseDaily": "20000",
                "maxRedeem": "1000",
                "minRedeem": "0.1",
                "maxRedeemDaily": "10000",
                "purchaseFeeRate": "0.0005",
                "redeemFeeRate": "0.0005",
                "ltStatus": "1"
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetLeverageTokenInfoParams::new().lt_coin("BTC3L");
    let result = match client.spot_leverage_token().get_info(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].lt_coin, "BTC3L");
    assert_eq!(result.list[0].lt_status, "1");
}

#[tokio::test]
async fn test_leverage_token_purchase_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/spot-lever-token/purchase"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "ltCoin": "BTC3L",
            "ltAmount": "500"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "ltCoin": "BTC3L",
            "ltOrderStatus": "1",
            "execQty": "1.1",
            "execAmt": "500",
            "amount": "500",
            "purchaseId": "purchase-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = PurchaseLeverageTokenParams::new("BTC3L", "500");
    let result = match client.spot_leverage_token().purchase(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.lt_coin, "BTC3L");
    assert_eq!(result.lt_order_status, "1");
}

#[tokio::test]
async fn test_crypto_loan_collateral_data() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/crypto-loan-common/collateral-data"))
        .and(query_param("currency", "BTC"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "vipCoinList": [{
                "vipLevel": "VIP0",
                "list": [{
                    "currency": "BTC",
                    "collateralAccuracy": 8,
                    "initialLtv": "0.65",
                    "marginCallLtv": "0.75",
                    "liquidationLtv": "0.85"
                }]
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetCollateralDataParams::new().currency("BTC");
    let result = match client.crypto_loan().get_collateral_data(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.vip_coin_list.len(), 1);
    assert_eq!(result.vip_coin_list[0].vip_level.as_deref(), Some("VIP0"));
    assert_eq!(
        result.vip_coin_list[0].list[0].currency.as_deref(),
        Some("BTC")
    );
}

#[tokio::test]
async fn test_crypto_loan_flexible_borrow() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/crypto-loan-flexible/borrow"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "loanCurrency": "USDT",
            "loanAmount": "1000"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "loan-order-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = FlexibleBorrowParams::new("USDT", "1000");
    let result = match client.crypto_loan().flexible_borrow(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.order_id.as_deref(), Some("loan-order-1"));
}

#[tokio::test]
async fn test_spread_instruments_is_public() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/spread/instrument"))
        .and(query_param("symbol", "SOLUSDT_SOL/USDT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "list": [{
                "symbol": "SOLUSDT_SOL/USDT",
                "contractType": "CarryTrade",
                "status": "Trading",
                "baseCoin": "SOL",
                "quoteCoin": "USDT"
            }],
            "nextPageCursor": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetSpreadInstrumentsInfoParams::new().symbol("SOLUSDT_SOL/USDT");
    let result = match client.spread().get_instruments_info(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].contract_type.as_deref(), Some("CarryTrade"));
}

#[tokio::test]
async fn test_spread_place_order() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/spread/order/create"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "symbol": "SOLUSDT_SOL/USDT",
            "side": "Buy",
            "orderType": "Limit",
            "qty": "10",
            "price": "1.5"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "spread-order-1",
            "orderLinkId": ""
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = PlaceSpreadOrderParams::limit("SOLUSDT_SOL/USDT", Side::Buy, "10", "1.5");
    let result = match client.spread().place_order(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.order_id.as_deref(), Some("spread-order-1"));
}

#[tokio::test]
async fn test_earn_product_info() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/earn/product"))
        .and(query_param("category", "FlexibleSaving"))
        .and(query_param("coin", "USDT"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "list": [{
                "category": "FlexibleSaving",
                "estimateApr": "3%",
                "coin": "USDT",
                "minStakeAmount": "10",
                "maxStakeAmount": "10000000",
                "precision": "8",
                "productId": "sp-usdt-1",
                "status": "Available"
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = public_client(&server).await;
    let params = GetEarnProductParams::new("FlexibleSaving").coin("USDT");
    let result = match client.earn().get_product_info(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].product_id, "sp-usdt-1");
    assert_eq!(result.list[0].estimate_apr.as_deref(), Some("3%"));
}

#[tokio::test]
async fn test_earn_place_order() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/earn/place-order"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "category": "FlexibleSaving",
            "orderType": "Stake",
            "accountType": "FUND",
            "amount": "100",
            "coin": "USDT",
            "productId": "sp-usdt-1"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "orderId": "earn-order-1",
            "orderLinkId": "my-link-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = PlaceEarnOrderParams::stake(
        "FlexibleSaving",
        "FUND",
        "100",
        "USDT",
        "sp-usdt-1",
        "my-link-1",
    );
    let result = match client.earn().place_order(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.order_id, "earn-order-1");
    assert_eq!(result.order_link_id, "my-link-1");
}
