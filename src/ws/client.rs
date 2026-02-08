//! WebSocket client implementation.

use std::collections::HashSet;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use tokio::net::TcpStream;
use tokio::sync::{mpsc, RwLock};
use tokio::time::interval;
use tokio_tungstenite::{
    connect_async,
    tungstenite::Message,
    MaybeTlsStream, WebSocketStream,
};
use tracing::{debug, error, info, warn};

use crate::auth;
use crate::config::{ClientConfig, Environment};
use crate::error::BybitError;
use crate::ws::types::*;

/// Default ping interval in seconds.
const DEFAULT_PING_INTERVAL_SECS: u64 = 20;

/// Default reconnect delay in seconds.
const DEFAULT_RECONNECT_DELAY_SECS: u64 = 5;

/// Maximum reconnect attempts before giving up.
const MAX_RECONNECT_ATTEMPTS: u32 = 10;

/// WebSocket client for public and private streams.
pub struct WsClient {
    /// Configuration.
    #[allow(dead_code)]
    config: ClientConfig,
    /// Channel type.
    channel: WsChannel,
    /// Subscribed topics.
    subscribed_topics: Arc<RwLock<HashSet<String>>>,
    /// Message sender.
    #[allow(dead_code)]
    message_tx: mpsc::UnboundedSender<WsMessage>,
    /// Command sender for internal commands.
    command_tx: mpsc::UnboundedSender<WsCommand>,
    /// Whether the client is connected.
    connected: Arc<AtomicBool>,
    /// Whether the client is running.
    running: Arc<AtomicBool>,
}

/// Internal commands for the WebSocket task.
enum WsCommand {
    Subscribe(Vec<String>),
    Unsubscribe(Vec<String>),
    #[allow(dead_code)]
    SendRaw(String),
    Disconnect,
}

impl WsClient {
    /// Create a new WebSocket client for public streams.
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bybit_client::ws::{WsClient, WsChannel};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let (client, mut receiver) = WsClient::connect_public(WsChannel::PublicLinear).await?;
    ///
    ///     client.subscribe(&["orderbook.50.BTCUSDT"]).await?;
    ///
    ///     while let Some(msg) = receiver.recv().await {
    ///         println!("Received: {:?}", msg);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect_public(
        channel: WsChannel,
    ) -> Result<(Self, mpsc::UnboundedReceiver<WsMessage>), BybitError> {
        Self::connect_with_config(ClientConfig::public_only(), channel).await
    }

    /// Create a new WebSocket client for private streams (requires authentication).
    ///
    /// # Example
    ///
    /// ```no_run
    /// use bybit_client::ws::{WsClient, WsChannel};
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     let (client, mut receiver) = WsClient::connect_private(
    ///         "api_key",
    ///         "api_secret",
    ///     ).await?;
    ///
    ///     client.subscribe(&["position", "order", "execution"]).await?;
    ///
    ///     while let Some(msg) = receiver.recv().await {
    ///         println!("Received: {:?}", msg);
    ///     }
    ///
    ///     Ok(())
    /// }
    /// ```
    pub async fn connect_private(
        api_key: impl Into<String>,
        api_secret: impl Into<String>,
    ) -> Result<(Self, mpsc::UnboundedReceiver<WsMessage>), BybitError> {
        let config = ClientConfig::new(api_key, api_secret);
        Self::connect_with_config(config, WsChannel::Private).await
    }

    /// Create a new WebSocket client with custom configuration.
    pub async fn connect_with_config(
        config: ClientConfig,
        channel: WsChannel,
    ) -> Result<(Self, mpsc::UnboundedReceiver<WsMessage>), BybitError> {
        if channel.requires_auth() && !config.has_credentials() {
            return Err(BybitError::Config(
                "Authentication required for private WebSocket channels".to_string(),
            ));
        }

        let (message_tx, message_rx) = mpsc::unbounded_channel();
        let (command_tx, command_rx) = mpsc::unbounded_channel();
        let subscribed_topics = Arc::new(RwLock::new(HashSet::new()));
        let connected = Arc::new(AtomicBool::new(false));
        let running = Arc::new(AtomicBool::new(true));

        let client = WsClient {
            config: config.clone(),
            channel,
            subscribed_topics: subscribed_topics.clone(),
            message_tx: message_tx.clone(),
            command_tx,
            connected: connected.clone(),
            running: running.clone(),
        };

        tokio::spawn(Self::run_ws_loop(
            config,
            channel,
            subscribed_topics,
            message_tx,
            command_rx,
            connected,
            running,
        ));

        Ok((client, message_rx))
    }

    /// Subscribe to topics.
    pub async fn subscribe(&self, topics: &[&str]) -> Result<(), BybitError> {
        if topics.is_empty() {
            return Ok(());
        }

        let topics: Vec<String> = topics.iter().map(|t| t.to_string()).collect();

        {
            let mut subscribed = self.subscribed_topics.write().await;
            for topic in &topics {
                subscribed.insert(topic.clone());
            }
        }

        self.command_tx
            .send(WsCommand::Subscribe(topics))
            .map_err(|_| BybitError::WebSocket("Failed to send subscribe command".to_string()))?;

        Ok(())
    }

    /// Unsubscribe from topics.
    pub async fn unsubscribe(&self, topics: &[&str]) -> Result<(), BybitError> {
        if topics.is_empty() {
            return Ok(());
        }

        let topics: Vec<String> = topics.iter().map(|t| t.to_string()).collect();

        {
            let mut subscribed = self.subscribed_topics.write().await;
            for topic in &topics {
                subscribed.remove(topic);
            }
        }

        self.command_tx
            .send(WsCommand::Unsubscribe(topics))
            .map_err(|_| {
                BybitError::WebSocket("Failed to send unsubscribe command".to_string())
            })?;

        Ok(())
    }

    /// Check if the client is connected.
    pub fn is_connected(&self) -> bool {
        self.connected.load(Ordering::SeqCst)
    }

    /// Disconnect and stop the client.
    pub fn disconnect(&self) {
        self.running.store(false, Ordering::SeqCst);
        let _ = self.command_tx.send(WsCommand::Disconnect);
    }

    /// Get the list of subscribed topics.
    pub async fn subscribed_topics(&self) -> Vec<String> {
        self.subscribed_topics
            .read()
            .await
            .iter()
            .cloned()
            .collect()
    }

    /// Get the WebSocket channel.
    pub fn channel(&self) -> WsChannel {
        self.channel
    }

    /// Build the WebSocket URL.
    fn build_ws_url(config: &ClientConfig, channel: WsChannel) -> String {
        let base = match config.environment {
            Environment::Production => "wss://stream.bybit.com",
            Environment::Testnet => "wss://stream-testnet.bybit.com",
            Environment::Demo => "wss://stream-demo.bybit.com",
        };
        format!("{}{}", base, channel.path())
    }

    /// Main WebSocket loop.
    async fn run_ws_loop(
        config: ClientConfig,
        channel: WsChannel,
        subscribed_topics: Arc<RwLock<HashSet<String>>>,
        message_tx: mpsc::UnboundedSender<WsMessage>,
        mut command_rx: mpsc::UnboundedReceiver<WsCommand>,
        connected: Arc<AtomicBool>,
        running: Arc<AtomicBool>,
    ) {
        let mut reconnect_attempts = 0;

        while running.load(Ordering::SeqCst) {
            let url = Self::build_ws_url(&config, channel);
            info!("Connecting to WebSocket: {}", url);

            match Self::connect_and_run(
                &url,
                &config,
                channel,
                &subscribed_topics,
                &message_tx,
                &mut command_rx,
                &connected,
                &running,
            )
            .await
            {
                Ok(()) => {
                    info!("WebSocket connection closed normally");
                    break;
                }
                Err(e) => {
                    error!("WebSocket error: {}", e);
                    connected.store(false, Ordering::SeqCst);

                    if !running.load(Ordering::SeqCst) {
                        break;
                    }

                    reconnect_attempts += 1;
                    if reconnect_attempts >= MAX_RECONNECT_ATTEMPTS {
                        error!(
                            "Max reconnect attempts ({}) reached, giving up",
                            MAX_RECONNECT_ATTEMPTS
                        );
                        break;
                    }

                    let delay = Duration::from_secs(DEFAULT_RECONNECT_DELAY_SECS);
                    warn!(
                        "Reconnecting in {} seconds (attempt {}/{})",
                        delay.as_secs(),
                        reconnect_attempts,
                        MAX_RECONNECT_ATTEMPTS
                    );
                    tokio::time::sleep(delay).await;
                }
            }
        }

        connected.store(false, Ordering::SeqCst);
        info!("WebSocket task ended");
    }

    /// Connect and run the WebSocket.
    #[allow(clippy::too_many_arguments)]
    async fn connect_and_run(
        url: &str,
        config: &ClientConfig,
        channel: WsChannel,
        subscribed_topics: &Arc<RwLock<HashSet<String>>>,
        message_tx: &mpsc::UnboundedSender<WsMessage>,
        command_rx: &mut mpsc::UnboundedReceiver<WsCommand>,
        connected: &Arc<AtomicBool>,
        running: &Arc<AtomicBool>,
    ) -> Result<(), BybitError> {
        let (ws_stream, _) = tokio::time::timeout(Duration::from_secs(30), connect_async(url))
            .await
            .map_err(|_| BybitError::WebSocket("Connection timeout".to_string()))?
            .map_err(|e| BybitError::WebSocket(format!("Connection failed: {}", e)))?;

        info!("WebSocket connected");
        connected.store(true, Ordering::SeqCst);

        let (mut write, mut read) = ws_stream.split();

        if channel.requires_auth() {
            Self::authenticate(&mut write, config).await?;
        }

        {
            let topics: Vec<String> = subscribed_topics.read().await.iter().cloned().collect();
            if !topics.is_empty() {
                info!("Re-subscribing to {} topics", topics.len());
                let op = WsOperation::subscribe(topics);
                let msg = serde_json::to_string(&op)
                    .map_err(|e| BybitError::WebSocket(format!("Serialize error: {}", e)))?;
                write
                    .send(Message::Text(msg.into()))
                    .await
                    .map_err(|e| BybitError::WebSocket(format!("Send error: {}", e)))?;
            }
        }

        let mut ping_interval = interval(Duration::from_secs(DEFAULT_PING_INTERVAL_SECS));

        loop {
            tokio::select! {
                msg = read.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            if let Some(ws_msg) = Self::parse_message(text.as_str()) {
                                if message_tx.send(ws_msg).is_err() {
                                    debug!("Message receiver dropped");
                                    break;
                                }
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            debug!("Received ping");
                            write.send(Message::Pong(data)).await
                                .map_err(|e| BybitError::WebSocket(format!("Pong error: {}", e)))?;
                        }
                        Some(Ok(Message::Pong(_))) => {
                            debug!("Received pong");
                        }
                        Some(Ok(Message::Close(frame))) => {
                            info!("Received close frame: {:?}", frame);
                            break;
                        }
                        Some(Err(e)) => {
                            return Err(BybitError::WebSocket(format!("Read error: {}", e)));
                        }
                        None => {
                            info!("WebSocket stream ended");
                            break;
                        }
                        _ => {}
                    }
                }

                cmd = command_rx.recv() => {
                    match cmd {
                        Some(WsCommand::Subscribe(topics)) => {
                            let op = WsOperation::subscribe(topics);
                            let msg = serde_json::to_string(&op)
                                .map_err(|e| BybitError::WebSocket(format!("Serialize error: {}", e)))?;
                            write.send(Message::Text(msg.into())).await
                                .map_err(|e| BybitError::WebSocket(format!("Send error: {}", e)))?;
                        }
                        Some(WsCommand::Unsubscribe(topics)) => {
                            let op = WsOperation::unsubscribe(topics);
                            let msg = serde_json::to_string(&op)
                                .map_err(|e| BybitError::WebSocket(format!("Serialize error: {}", e)))?;
                            write.send(Message::Text(msg.into())).await
                                .map_err(|e| BybitError::WebSocket(format!("Send error: {}", e)))?;
                        }
                        Some(WsCommand::SendRaw(text)) => {
                            write.send(Message::Text(text.into())).await
                                .map_err(|e| BybitError::WebSocket(format!("Send error: {}", e)))?;
                        }
                        Some(WsCommand::Disconnect) | None => {
                            info!("Disconnect requested");
                            let _ = write.send(Message::Close(None)).await;
                            break;
                        }
                    }
                }

                _ = ping_interval.tick() => {
                    let op = WsOperation::ping();
                    let msg = serde_json::to_string(&op)
                        .map_err(|e| BybitError::WebSocket(format!("Serialize error: {}", e)))?;
                    write.send(Message::Text(msg.into())).await
                        .map_err(|e| BybitError::WebSocket(format!("Ping error: {}", e)))?;
                    debug!("Sent ping");
                }

                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    if !running.load(Ordering::SeqCst) {
                        info!("Stop requested");
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Authenticate the private WebSocket connection.
    async fn authenticate(
        write: &mut futures_util::stream::SplitSink<
            WebSocketStream<MaybeTlsStream<TcpStream>>,
            Message,
        >,
        config: &ClientConfig,
    ) -> Result<(), BybitError> {
        let api_key = config.api_key.as_ref().ok_or_else(|| {
            BybitError::Config("API key required for authentication".to_string())
        })?;

        let api_secret = config.get_secret().ok_or_else(|| {
            BybitError::Config("API secret required for authentication".to_string())
        })?;

        let expires = auth::current_timestamp_ms() + 10_000;
        let signature = auth::sign_ws_auth(expires, api_secret);

        let op = WsOperation::auth(api_key, expires, &signature);

        let msg = serde_json::to_string(&op)
            .map_err(|e| BybitError::WebSocket(format!("Serialize error: {}", e)))?;

        write
            .send(Message::Text(msg.into()))
            .await
            .map_err(|e| BybitError::WebSocket(format!("Auth send error: {}", e)))?;

        info!("Sent authentication request");
        Ok(())
    }

    /// Parse an incoming WebSocket message.
    fn parse_message(text: &str) -> Option<WsMessage> {
        let value: serde_json::Value = match serde_json::from_str(text) {
            Ok(v) => v,
            Err(e) => {
                warn!("Failed to parse WebSocket message: {}", e);
                return Some(WsMessage::Raw(text.to_string()));
            }
        };

        if value.get("op").and_then(|v| v.as_str()) == Some("pong") {
            if let Ok(pong) = serde_json::from_value(value.clone()) {
                return Some(WsMessage::Pong(pong));
            }
        }

        if value.get("success").is_some() && value.get("topic").is_none() {
            if let Ok(response) = serde_json::from_value(value.clone()) {
                return Some(WsMessage::OperationResponse(response));
            }
        }

        if let Some(topic) = value.get("topic").and_then(|v| v.as_str()) {
            if topic.starts_with("orderbook.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Orderbook(Box::new(msg)));
                }
            } else if topic.starts_with("publicTrade.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Trade(Box::new(msg)));
                }
            } else if topic.starts_with("tickers.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Ticker(Box::new(msg)));
                }
            } else if topic.starts_with("kline.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Kline(Box::new(msg)));
                }
            } else if topic.starts_with("liquidation.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Liquidation(Box::new(msg)));
                }
            }
            else if topic == "position" || topic.starts_with("position.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Position(Box::new(msg)));
                }
            } else if topic == "order" || topic.starts_with("order.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Order(Box::new(msg)));
                }
            } else if topic == "execution.fast" {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::ExecutionFast(Box::new(msg)));
                }
            } else if topic == "execution" || topic.starts_with("execution.") {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Execution(Box::new(msg)));
                }
            } else if topic == "wallet" {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Wallet(Box::new(msg)));
                }
            } else if topic == "greeks" {
                if let Ok(msg) = serde_json::from_value(value) {
                    return Some(WsMessage::Greeks(Box::new(msg)));
                }
            }
        }

        Some(WsMessage::Raw(text.to_string()))
    }
}

impl Drop for WsClient {
    fn drop(&mut self) {
        self.disconnect();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_ws_url() {
        let config = ClientConfig::public_only();
        let url = WsClient::build_ws_url(&config, WsChannel::PublicLinear);
        assert_eq!(url, "wss://stream.bybit.com/v5/public/linear");

        let testnet = config.testnet();
        let url = WsClient::build_ws_url(&testnet, WsChannel::PublicLinear);
        assert_eq!(url, "wss://stream-testnet.bybit.com/v5/public/linear");
    }

    #[test]
    fn test_parse_pong() {
        let text = r#"{"success":true,"ret_msg":"pong","conn_id":"abc123","op":"pong"}"#;
        let msg = WsClient::parse_message(text);
        assert!(matches!(msg, Some(WsMessage::Pong(_))));
    }

    #[test]
    fn test_parse_operation_response() {
        let text = r#"{"success":true,"ret_msg":"subscribe","conn_id":"abc123"}"#;
        let msg = WsClient::parse_message(text);
        assert!(matches!(msg, Some(WsMessage::OperationResponse(_))));
    }
}
