//! WebSocket Trade API client for order management.
//!
//! This module provides a client for submitting, amending, and canceling orders
//! via WebSocket connections. This offers lower latency compared to REST API.
//!
//! # Example
//!
//! ```no_run
//! use bybit_client::ws::{WsTradeClient, CreateOrderRequest};
//! use bybit_client::ClientConfig;
//! use bybit_client::types::{Category, Side, OrderType, TimeInForce};
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let config = ClientConfig::new("api_key", "api_secret");
//!     let client = WsTradeClient::connect(config).await?;
//!
//!     // Submit a new order.
//!     let result = client.create_order(CreateOrderRequest {
//!         category: Category::Linear,
//!         symbol: "BTCUSDT".to_string(),
//!         side: Side::Buy,
//!         order_type: OrderType::Limit,
//!         qty: "0.001".to_string(),
//!         price: Some("50000".to_string()),
//!         time_in_force: Some(TimeInForce::GTC),
//!         order_link_id: None,
//!         is_leverage: None,
//!         position_idx: None,
//!         reduce_only: None,
//!         close_on_trigger: None,
//!         take_profit: None,
//!         stop_loss: None,
//!         tpsl_mode: None,
//!         market_unit: None,
//!     }).await?;
//!
//!     println!("Order created: {}", result.order_id);
//!     Ok(())
//! }
//! ```

use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, RwLock};
use tokio::time::timeout;
use tokio_tungstenite::tungstenite::Message;
use tracing::{debug, error, info};

use crate::auth::{current_timestamp_ms, sign_rest_request};
use crate::config::ClientConfig;
use crate::error::BybitError;
use crate::types::{Category, OrderType, Side, TimeInForce};

/// Default timeout for trade operations.
const DEFAULT_TIMEOUT_MS: u64 = 10000;
/// Default receive window.
const DEFAULT_RECV_WINDOW: u32 = 5000;


/// Request for creating a new order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateOrderRequest {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order side (Buy/Sell).
    pub side: Side,
    /// Order type (Limit/Market).
    pub order_type: OrderType,
    /// Order quantity.
    pub qty: String,
    /// Order price (required for Limit orders).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// Time in force.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<TimeInForce>,
    /// User custom order ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// Is leverage order (for spot margin).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_leverage: Option<i32>,
    /// Position index (for hedged mode).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position_idx: Option<i32>,
    /// Reduce only order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    /// Close on trigger.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub close_on_trigger: Option<bool>,
    /// Take profit price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// Stop loss price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// TP/SL mode (Full/Partial).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tpsl_mode: Option<String>,
    /// Market unit (baseCoin/quoteCoin for spot).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub market_unit: Option<String>,
}

/// Request for amending an existing order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AmendOrderRequest {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order ID (required if orderLinkId not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User custom order ID (required if orderId not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
    /// New quantity.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qty: Option<String>,
    /// New price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,
    /// New take profit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub take_profit: Option<String>,
    /// New stop loss.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stop_loss: Option<String>,
    /// New TP trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tp_limit_price: Option<String>,
    /// New SL trigger price.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sl_limit_price: Option<String>,
}

/// Request for canceling an order.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelOrderRequest {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Order ID (required if orderLinkId not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    /// User custom order ID (required if orderId not provided).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_link_id: Option<String>,
}


/// Result of a single order operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OrderResult {
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
}

/// Result of a batch order operation.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatchOrderResult {
    /// Category.
    pub category: String,
    /// Symbol.
    pub symbol: String,
    /// Order ID.
    pub order_id: String,
    /// User custom order ID.
    #[serde(default)]
    pub order_link_id: Option<String>,
    /// Create type.
    #[serde(default)]
    pub create_type: Option<String>,
}

/// WebSocket Trade API response.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTradeResponse {
    /// Request ID (echoed from request).
    pub req_id: String,
    /// Return code (0 = success).
    pub ret_code: i32,
    /// Return message.
    pub ret_msg: String,
    /// Operation type.
    pub op: String,
    /// Response data.
    #[serde(default)]
    pub data: serde_json::Value,
    /// Connection ID.
    #[serde(default)]
    pub conn_id: Option<String>,
}

impl WsTradeResponse {
    /// Check if the response indicates success.
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }

    /// Convert to a typed result.
    pub fn into_result<T: for<'de> Deserialize<'de>>(self) -> Result<T, BybitError> {
        if self.is_success() {
            serde_json::from_value(self.data)
                .map_err(|e| BybitError::Serialization(e))
        } else {
            Err(BybitError::api_error(self.ret_code, self.ret_msg))
        }
    }
}


/// WebSocket Trade API request wrapper.
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct WsTradeRequest<T> {
    req_id: String,
    op: String,
    header: WsTradeHeader,
    args: Vec<T>,
}

/// Request header with authentication.
#[derive(Debug, Serialize)]
struct WsTradeHeader {
    #[serde(rename = "X-BAPI-TIMESTAMP")]
    timestamp: String,
    #[serde(rename = "X-BAPI-RECV-WINDOW")]
    recv_window: String,
    #[serde(rename = "X-BAPI-API-KEY")]
    api_key: String,
    #[serde(rename = "X-BAPI-SIGN")]
    sign: String,
}

/// Pending request waiting for response.
struct PendingRequest {
    sender: oneshot::Sender<Result<WsTradeResponse, BybitError>>,
}


/// WebSocket Trade API client for low-latency order management.
///
/// This client connects to the WebSocket Trade endpoint and provides
/// methods for creating, amending, and canceling orders with lower
/// latency compared to REST API.
pub struct WsTradeClient {
    /// Sender for outgoing WebSocket messages.
    tx: mpsc::UnboundedSender<Message>,
    /// Pending requests waiting for responses.
    pending: Arc<RwLock<HashMap<String, PendingRequest>>>,
    /// Request ID counter.
    req_counter: AtomicU64,
    /// API key.
    api_key: String,
    /// API secret.
    api_secret: String,
    /// Receive window for requests.
    recv_window: u32,
    /// Connected flag.
    connected: Arc<RwLock<bool>>,
}

impl WsTradeClient {
    /// Connect to the WebSocket Trade API.
    pub async fn connect(config: ClientConfig) -> Result<Self, BybitError> {
        let api_key = config
            .api_key
            .as_ref()
            .ok_or_else(|| BybitError::Auth("API key required for trade API".to_string()))?
            .clone();
        let api_secret = config
            .get_secret()
            .ok_or_else(|| BybitError::Auth("API secret required for trade API".to_string()))?
            .to_string();

        let url = config.get_ws_trade_url();

        info!("Connecting to WebSocket Trade API: {}", url);
        let url = url.to_string();

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url)
            .await
            .map_err(|e| BybitError::WebSocket(format!("Connection failed: {}", e)))?;

        let (mut write, mut read) = ws_stream.split();

        let (tx, mut rx) = mpsc::unbounded_channel::<Message>();
        let pending: Arc<RwLock<HashMap<String, PendingRequest>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let connected = Arc::new(RwLock::new(true));

        let pending_clone = pending.clone();
        let connected_clone = connected.clone();

        tokio::spawn(async move {
            while let Some(msg) = read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        debug!("Trade API received: {}", text);

                        if let Ok(response) = serde_json::from_str::<WsTradeResponse>(&text) {
                            let mut pending = pending_clone.write().await;
                            if let Some(request) = pending.remove(&response.req_id) {
                                let _ = request.sender.send(Ok(response));
                            }
                        }
                    }
                    Ok(Message::Ping(data)) => {
                        debug!("Trade API ping received");
                        let _ = data;
                    }
                    Ok(Message::Close(_)) => {
                        info!("Trade API connection closed");
                        *connected_clone.write().await = false;
                        break;
                    }
                    Err(e) => {
                        error!("Trade API read error: {}", e);
                        *connected_clone.write().await = false;
                        break;
                    }
                    _ => {}
                }
            }

            let mut pending = pending_clone.write().await;
            for (_, request) in pending.drain() {
                let _ = request
                    .sender
                    .send(Err(BybitError::WebSocket("Connection closed".to_string())));
            }
        });

        tokio::spawn(async move {
            while let Some(msg) = rx.recv().await {
                if let Err(e) = write.send(msg).await {
                    error!("Trade API write error: {}", e);
                    break;
                }
            }
        });

        Ok(Self {
            tx,
            pending,
            req_counter: AtomicU64::new(1),
            api_key,
            api_secret,
            recv_window: DEFAULT_RECV_WINDOW,
            connected,
        })
    }

    /// Check if the client is connected.
    pub async fn is_connected(&self) -> bool {
        *self.connected.read().await
    }

    /// Generate a unique request ID.
    fn generate_req_id(&self) -> String {
        let counter = self.req_counter.fetch_add(1, Ordering::SeqCst);
        format!("req-{}", counter)
    }

    /// Create authentication header for a request.
    fn create_header(&self, args_json: &str) -> WsTradeHeader {
        let timestamp = current_timestamp_ms();
        let recv_window = self.recv_window;

        let signature = sign_rest_request(
            timestamp,
            &self.api_key,
            recv_window,
            args_json,
            &self.api_secret,
        );

        WsTradeHeader {
            timestamp: timestamp.to_string(),
            recv_window: recv_window.to_string(),
            api_key: self.api_key.clone(),
            sign: signature,
        }
    }

    /// Send a trade request and wait for response.
    async fn send_request<T: Serialize>(
        &self,
        op: &str,
        args: Vec<T>,
    ) -> Result<WsTradeResponse, BybitError> {
        if !self.is_connected().await {
            return Err(BybitError::WebSocket("Not connected".to_string()));
        }

        let req_id = self.generate_req_id();

        let args_json = serde_json::to_string(&args)
            .map_err(|e| BybitError::Serialization(e))?;

        let header = self.create_header(&args_json);

        let request = WsTradeRequest {
            req_id: req_id.clone(),
            op: op.to_string(),
            header,
            args,
        };

        let json = serde_json::to_string(&request)
            .map_err(|e| BybitError::Serialization(e))?;

        debug!("Trade API sending: {}", json);

        let (tx, rx) = oneshot::channel();
        {
            let mut pending = self.pending.write().await;
            pending.insert(req_id.clone(), PendingRequest { sender: tx });
        }

        self.tx
            .send(Message::Text(json.into()))
            .map_err(|e| BybitError::WebSocket(format!("Send failed: {}", e)))?;

        let result = timeout(Duration::from_millis(DEFAULT_TIMEOUT_MS), rx).await;

        match result {
            Ok(Ok(response)) => response,
            Ok(Err(_)) => Err(BybitError::WebSocket("Response channel closed".to_string())),
            Err(_) => {
                let mut pending = self.pending.write().await;
                pending.remove(&req_id);
                Err(BybitError::Timeout)
            }
        }
    }


    /// Create a new order.
    pub async fn create_order(
        &self,
        request: CreateOrderRequest,
    ) -> Result<OrderResult, BybitError> {
        let response = self.send_request("order.create", vec![request]).await?;
        response.into_result()
    }

    /// Amend an existing order.
    pub async fn amend_order(
        &self,
        request: AmendOrderRequest,
    ) -> Result<OrderResult, BybitError> {
        let response = self.send_request("order.amend", vec![request]).await?;
        response.into_result()
    }

    /// Cancel an order.
    pub async fn cancel_order(
        &self,
        request: CancelOrderRequest,
    ) -> Result<OrderResult, BybitError> {
        let response = self.send_request("order.cancel", vec![request]).await?;
        response.into_result()
    }


    /// Create multiple orders in a single request (max 10).
    pub async fn batch_create_orders(
        &self,
        category: Category,
        orders: Vec<CreateOrderRequest>,
    ) -> Result<Vec<BatchOrderResult>, BybitError> {
        if orders.is_empty() {
            return Ok(Vec::new());
        }
        if orders.len() > 10 {
            return Err(BybitError::InvalidParameter(
                "Batch order limit is 10".to_string(),
            ));
        }

        let orders: Vec<_> = orders
            .into_iter()
            .map(|mut o| {
                o.category = category.clone();
                o
            })
            .collect();

        let response = self.send_request("order.create-batch", orders).await?;

        if response.is_success() {
            let list = response
                .data
                .get("result")
                .and_then(|r| r.get("list"))
                .cloned()
                .unwrap_or(serde_json::Value::Array(vec![]));
            serde_json::from_value(list).map_err(|e| BybitError::Serialization(e))
        } else {
            Err(BybitError::api_error(response.ret_code, response.ret_msg))
        }
    }

    /// Amend multiple orders in a single request (max 10).
    pub async fn batch_amend_orders(
        &self,
        category: Category,
        orders: Vec<AmendOrderRequest>,
    ) -> Result<Vec<BatchOrderResult>, BybitError> {
        if orders.is_empty() {
            return Ok(Vec::new());
        }
        if orders.len() > 10 {
            return Err(BybitError::InvalidParameter(
                "Batch order limit is 10".to_string(),
            ));
        }

        let orders: Vec<_> = orders
            .into_iter()
            .map(|mut o| {
                o.category = category.clone();
                o
            })
            .collect();

        let response = self.send_request("order.amend-batch", orders).await?;

        if response.is_success() {
            let list = response
                .data
                .get("result")
                .and_then(|r| r.get("list"))
                .cloned()
                .unwrap_or(serde_json::Value::Array(vec![]));
            serde_json::from_value(list).map_err(|e| BybitError::Serialization(e))
        } else {
            Err(BybitError::api_error(response.ret_code, response.ret_msg))
        }
    }

    /// Cancel multiple orders in a single request (max 10).
    pub async fn batch_cancel_orders(
        &self,
        category: Category,
        orders: Vec<CancelOrderRequest>,
    ) -> Result<Vec<BatchOrderResult>, BybitError> {
        if orders.is_empty() {
            return Ok(Vec::new());
        }
        if orders.len() > 10 {
            return Err(BybitError::InvalidParameter(
                "Batch order limit is 10".to_string(),
            ));
        }

        let orders: Vec<_> = orders
            .into_iter()
            .map(|mut o| {
                o.category = category.clone();
                o
            })
            .collect();

        let response = self.send_request("order.cancel-batch", orders).await?;

        if response.is_success() {
            let list = response
                .data
                .get("result")
                .and_then(|r| r.get("list"))
                .cloned()
                .unwrap_or(serde_json::Value::Array(vec![]));
            serde_json::from_value(list).map_err(|e| BybitError::Serialization(e))
        } else {
            Err(BybitError::api_error(response.ret_code, response.ret_msg))
        }
    }

    /// Disconnect from the WebSocket Trade API.
    pub async fn disconnect(&self) {
        *self.connected.write().await = false;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_order_request_serialize() {
        let request = CreateOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            side: Side::Buy,
            order_type: OrderType::Limit,
            qty: "0.001".to_string(),
            price: Some("50000".to_string()),
            time_in_force: Some(TimeInForce::GTC),
            order_link_id: None,
            is_leverage: None,
            position_idx: None,
            reduce_only: None,
            close_on_trigger: None,
            take_profit: None,
            stop_loss: None,
            tpsl_mode: None,
            market_unit: None,
        };

        let json = match serde_json::to_string(&request) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize create request: {}", err),
        };
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"Buy\""));
        assert!(json.contains("\"orderType\":\"Limit\""));
        assert!(json.contains("\"qty\":\"0.001\""));
        assert!(json.contains("\"price\":\"50000\""));
    }

    #[test]
    fn test_amend_order_request_serialize() {
        let request = AmendOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order-123".to_string()),
            order_link_id: None,
            qty: Some("0.002".to_string()),
            price: Some("51000".to_string()),
            take_profit: None,
            stop_loss: None,
            tp_limit_price: None,
            sl_limit_price: None,
        };

        let json = match serde_json::to_string(&request) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize amend request: {}", err),
        };
        assert!(json.contains("\"orderId\":\"order-123\""));
        assert!(json.contains("\"qty\":\"0.002\""));
        assert!(json.contains("\"price\":\"51000\""));
    }

    #[test]
    fn test_cancel_order_request_serialize() {
        let request = CancelOrderRequest {
            category: Category::Linear,
            symbol: "BTCUSDT".to_string(),
            order_id: Some("order-123".to_string()),
            order_link_id: None,
        };

        let json = match serde_json::to_string(&request) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize cancel request: {}", err),
        };
        assert!(json.contains("\"orderId\":\"order-123\""));
        assert!(!json.contains("orderLinkId"));
    }

    #[test]
    fn test_trade_response_deserialize() {
        let json = r#"{
            "reqId": "req-1",
            "retCode": 0,
            "retMsg": "OK",
            "op": "order.create",
            "data": {
                "orderId": "order-456",
                "orderLinkId": "my-order-1"
            },
            "connId": "conn-123"
        }"#;

        let response: WsTradeResponse = match serde_json::from_str(json) {
            Ok(response) => response,
            Err(err) => panic!("Failed to parse trade response: {}", err),
        };
        assert_eq!(response.req_id, "req-1");
        assert!(response.is_success());
        assert_eq!(response.op, "order.create");

        let result: OrderResult = match response.into_result() {
            Ok(result) => result,
            Err(err) => panic!("Expected successful result: {}", err),
        };
        assert_eq!(result.order_id, "order-456");
        assert_eq!(result.order_link_id, Some("my-order-1".to_string()));
    }

    #[test]
    fn test_trade_response_error() {
        let json = r#"{
            "reqId": "req-1",
            "retCode": 10001,
            "retMsg": "Param error",
            "op": "order.create",
            "data": {}
        }"#;

        let response: WsTradeResponse = match serde_json::from_str(json) {
            Ok(response) => response,
            Err(err) => panic!("Failed to parse trade response: {}", err),
        };
        assert!(!response.is_success());

        let result: Result<OrderResult, _> = response.into_result();
        assert!(result.is_err());
    }
}
