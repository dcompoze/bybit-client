//! Wiremock integration tests for the account and asset services.
//!
//! These tests verify request paths, query parameters, signing headers,
//! request bodies, and response parsing against a mock HTTP server.

use bybit_client::types::account::{
    DemoApplyMoneyParams, GetFeeRatesParams, GetTransactionLogParams, GetWalletBalanceParams,
    MmpModifyParams,
};
use bybit_client::types::asset::{
    GetCoinInfoParams, GetDepositRecordsParams, GetWithdrawableAmountParams, InterTransferParams,
    RequestConvertQuoteParams, WithdrawParams,
};
use bybit_client::{AccountType, BybitClient, Category, ClientConfig};
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

#[tokio::test]
async fn test_get_wallet_balance_signed_and_parsed() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/account/wallet-balance"))
        .and(query_param("accountType", "UNIFIED"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "list": [{
                "accountType": "UNIFIED",
                "totalEquity": "10000.5",
                "totalWalletBalance": "9000.1",
                "totalAvailableBalance": "8000.2",
                "coin": [{
                    "coin": "USDT",
                    "equity": "10000.5",
                    "usdValue": "10000.5",
                    "walletBalance": "9000.1"
                }]
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetWalletBalanceParams::new(AccountType::Unified);
    let result = match client.account().get_wallet_balance(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list.len(), 1);
    assert_eq!(result.list[0].total_equity, "10000.5");
    assert_eq!(result.list[0].coin[0].coin, "USDT");
}

#[tokio::test]
async fn test_get_account_info() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/account/info"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "unifiedMarginStatus": 4,
            "marginMode": "REGULAR_MARGIN",
            "isMasterTrader": false,
            "spotHedgingStatus": "OFF",
            "updatedTime": "1755900000000"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let info = match client.account().get_account_info().await {
        Ok(info) => info,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(info.margin_mode, "REGULAR_MARGIN");
    assert_eq!(info.unified_margin_status, Some(4));
}

#[tokio::test]
async fn test_get_transaction_log_display_type() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/account/transaction-log"))
        .and(query_param("currency", "USDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "list": [{
                "transactionTime": "1755900000000",
                "type": "DIVIDEND_SETTLEMENT",
                "currency": "USDT",
                "cashFlow": "5.0",
                "change": "5.0",
                "cashBalance": "1005.0",
                "displayType": "Dividend"
            }],
            "nextPageCursor": "cursor-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetTransactionLogParams::new().currency("USDT");
    let result = match client.account().get_transaction_log(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list[0].transaction_type, "DIVIDEND_SETTLEMENT");
    assert_eq!(result.list[0].display_type.as_deref(), Some("Dividend"));
    assert_eq!(result.next_page_cursor.as_deref(), Some("cursor-1"));
}

#[tokio::test]
async fn test_get_mmp_state() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/account/mmp-state"))
        .and(query_param("baseCoin", "BTC"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "result": [{
                "baseCoin": "BTC",
                "mmpEnabled": true,
                "window": "5000",
                "frozenPeriod": "100000",
                "qtyLimit": "10",
                "deltaLimit": "5",
                "vegaLimit": "2",
                "mmpFrozenUntil": "0",
                "mmpFrozen": false
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let result = match client.account().get_mmp_state("BTC").await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.result.len(), 1);
    assert!(result.result[0].mmp_enabled);
    assert_eq!(result.result[0].vega_limit.as_deref(), Some("2"));
}

#[tokio::test]
async fn test_mmp_modify_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/account/mmp-modify"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "baseCoin": "BTC",
            "window": "5000",
            "frozenPeriod": "100000",
            "qtyLimit": "10",
            "deltaLimit": "5",
            "vegaLimit": "2"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({}))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = MmpModifyParams::new("BTC", "5000", "100000", "10", "5").vega_limit("2");
    if let Err(err) = client.account().mmp_modify(&params).await {
        panic!("Request failed: {}", err);
    }
}

#[tokio::test]
async fn test_get_smp_group_string_type() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/account/smp-group"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "smpGroup": "123"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let result = match client.account().get_smp_group().await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.smp_group, "123");
}

#[tokio::test]
async fn test_demo_apply_money_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/account/demo-apply-money"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "adjustType": 0,
            "utaDemoApplyMoney": [{"coin": "USDT", "amountStr": "10000"}]
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({}))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = DemoApplyMoneyParams::new()
        .adjust_type(0)
        .coin("USDT", "10000");
    if let Err(err) = client.account().demo_apply_money(&params).await {
        panic!("Request failed: {}", err);
    }
}

#[tokio::test]
async fn test_get_fee_rates() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/account/fee-rate"))
        .and(query_param("category", "linear"))
        .and(query_param("symbol", "BTCUSDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "category": "linear",
            "list": [{
                "symbol": "BTCUSDT",
                "takerFeeRate": "0.00055",
                "makerFeeRate": "0.0002"
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetFeeRatesParams::new(Category::Linear).symbol("BTCUSDT");
    let result = match client.account().get_fee_rates(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.list[0].maker_fee_rate, "0.0002");
    assert_eq!(result.list[0].taker_fee_rate, "0.00055");
}

#[tokio::test]
async fn test_get_coin_info_withdraw_max() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/asset/coin/query-info"))
        .and(query_param("coin", "USDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "rows": [{
                "coin": "USDT",
                "name": "Tether",
                "remainAmount": "1000000",
                "withdrawMax": "2000000",
                "chains": [{"chain": "ETH"}]
            }]
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetCoinInfoParams::new().coin("USDT");
    let result = match client.asset().get_coin_info(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.rows[0].coin, "USDT");
    assert_eq!(result.rows[0].withdraw_max.as_deref(), Some("2000000"));
}

#[tokio::test]
async fn test_create_inter_transfer_body() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/asset/transfer/inter-transfer"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(header("X-BAPI-API-KEY", "test_key"))
        .and(body_partial_json(serde_json::json!({
            "transferId": "uuid-1",
            "coin": "USDT",
            "amount": "100",
            "fromAccountType": "UNIFIED",
            "toAccountType": "FUND"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "transferId": "uuid-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = InterTransferParams::new(
        "uuid-1",
        "USDT",
        "100",
        AccountType::Unified,
        AccountType::Fund,
    );
    let result = match client.asset().create_inter_transfer(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.transfer_id, "uuid-1");
}

#[tokio::test]
async fn test_get_deposit_records() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/asset/deposit/query-record"))
        .and(query_param("coin", "BTC"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "rows": [{
                "coin": "BTC",
                "chain": "BTC",
                "amount": "0.5",
                "txID": "tx-abc",
                "status": 3
            }],
            "nextPageCursor": "next-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetDepositRecordsParams::default().coin("BTC");
    let result = match client.asset().get_deposit_records(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.rows[0].tx_id.as_deref(), Some("tx-abc"));
    assert_eq!(result.rows[0].status, Some(3));
}

#[tokio::test]
async fn test_withdraw_body_and_result() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/asset/withdraw/create"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "coin": "USDT",
            "chain": "ETH",
            "address": "0xabc",
            "amount": "100",
            "timestamp": 1755900000000u64
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "id": "wd-1"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = WithdrawParams::new("USDT", "0xabc", "100", 1755900000000).chain("ETH");
    let result = match client.asset().withdraw(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.id, "wd-1");
}

#[tokio::test]
async fn test_request_convert_quote() {
    let server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/v5/asset/exchange/quote-apply"))
        .and(header_exists("X-BAPI-SIGN"))
        .and(body_partial_json(serde_json::json!({
            "fromCoin": "BTC",
            "toCoin": "USDT",
            "requestCoin": "BTC",
            "requestAmount": "0.1",
            "accountType": "eb_convert_uta"
        })))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "quoteTxId": "quote-1",
            "exchangeRate": "50000",
            "fromCoin": "BTC",
            "toCoin": "USDT",
            "toAmount": "5000"
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = RequestConvertQuoteParams::new("BTC", "USDT", "0.1", "eb_convert_uta");
    let result = match client.asset().request_convert_quote(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    assert_eq!(result.quote_tx_id, "quote-1");
    assert_eq!(result.to_amount.as_deref(), Some("5000"));
}

#[tokio::test]
async fn test_get_withdrawable_amount() {
    let server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/v5/asset/withdraw/withdrawable-amount"))
        .and(query_param("coin", "USDT"))
        .and(header_exists("X-BAPI-SIGN"))
        .respond_with(ResponseTemplate::new(200).set_body_json(envelope(serde_json::json!({
            "limitAmountUsd": "0",
            "withdrawableAmount": {
                "FUND": {
                    "coin": "USDT",
                    "withdrawableAmount": "500",
                    "availableBalance": "500"
                }
            }
        }))))
        .expect(1)
        .mount(&server)
        .await;

    let client = signed_client(&server).await;
    let params = GetWithdrawableAmountParams::new("USDT");
    let result = match client.asset().get_withdrawable_amount(&params).await {
        Ok(result) => result,
        Err(err) => panic!("Request failed: {}", err),
    };

    let fund = match result.withdrawable_amount.get("FUND") {
        Some(entry) => entry,
        None => panic!("Missing FUND entry"),
    };
    assert_eq!(fund.withdrawable_amount.as_deref(), Some("500"));
}
