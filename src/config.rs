//! Configuration for the Bybit client.

use secrecy::{ExposeSecret, SecretString};

/// API base URLs for REST endpoints.
pub mod rest_urls {
    /// Default mainnet API URL.
    pub const MAINNET: &str = "https://api.bybit.com";
    /// Alternative mainnet (bytick).
    pub const MAINNET_BYTICK: &str = "https://api.bytick.com";
    /// Testnet API URL.
    pub const TESTNET: &str = "https://api-testnet.bybit.com";
    /// Demo trading API URL.
    pub const DEMO: &str = "https://api-demo.bybit.com";

    // Regional endpoints
    /// Netherlands
    pub const NL: &str = "https://api.bybit.nl";
    /// Turkey
    pub const TK: &str = "https://api.bybit-tr.com";
    /// Kazakhstan
    pub const KZ: &str = "https://api.bybit.kz";
    /// Hong Kong
    pub const HK: &str = "https://api.byhkbit.com";
    /// Georgia
    pub const GE: &str = "https://api.bybitgeorgia.ge";
    /// UAE
    pub const UAE: &str = "https://api.bybit.ae";
    /// EU
    pub const EU: &str = "https://api.bybit.eu";
}

/// WebSocket URLs.
pub mod ws_urls {
    use super::super::types::Category;

    /// Get public WebSocket URL for mainnet.
    pub fn public_mainnet(category: Category) -> String {
        format!("wss://stream.bybit.com/v5/public/{}", category.as_str())
    }

    /// Get public WebSocket URL for testnet.
    pub fn public_testnet(category: Category) -> String {
        format!(
            "wss://stream-testnet.bybit.com/v5/public/{}",
            category.as_str()
        )
    }

    /// Get public WebSocket URL for demo.
    pub fn public_demo(category: Category) -> String {
        format!("wss://stream-demo.bybit.com/v5/public/{}", category.as_str())
    }

    /// Private WebSocket URL for mainnet.
    pub const PRIVATE_MAINNET: &str = "wss://stream.bybit.com/v5/private";
    /// Private WebSocket URL for testnet.
    pub const PRIVATE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/private";
    /// Private WebSocket URL for demo.
    pub const PRIVATE_DEMO: &str = "wss://stream-demo.bybit.com/v5/private";

    /// Trade WebSocket URL for mainnet.
    pub const TRADE_MAINNET: &str = "wss://stream.bybit.com/v5/trade";
    /// Trade WebSocket URL for testnet.
    pub const TRADE_TESTNET: &str = "wss://stream-testnet.bybit.com/v5/trade";
    /// Trade WebSocket URL for demo.
    pub const TRADE_DEMO: &str = "wss://stream-demo.bybit.com/v5/trade";
}

/// API region selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ApiRegion {
    /// Default (api.bybit.com)
    #[default]
    Default,
    /// Bytick (api.bytick.com)
    Bytick,
    /// Netherlands
    NL,
    /// Turkey
    TK,
    /// Kazakhstan
    KZ,
    /// Hong Kong
    HK,
    /// Georgia
    GE,
    /// UAE
    UAE,
    /// EU
    EU,
}

impl ApiRegion {
    /// Get the base URL for this region.
    pub fn base_url(&self) -> &'static str {
        match self {
            ApiRegion::Default => rest_urls::MAINNET,
            ApiRegion::Bytick => rest_urls::MAINNET_BYTICK,
            ApiRegion::NL => rest_urls::NL,
            ApiRegion::TK => rest_urls::TK,
            ApiRegion::KZ => rest_urls::KZ,
            ApiRegion::HK => rest_urls::HK,
            ApiRegion::GE => rest_urls::GE,
            ApiRegion::UAE => rest_urls::UAE,
            ApiRegion::EU => rest_urls::EU,
        }
    }
}

/// Environment selection (production, testnet, demo).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Environment {
    /// Production environment.
    #[default]
    Production,
    /// Testnet environment.
    Testnet,
    /// Demo trading environment.
    Demo,
}

/// Client configuration.
#[derive(Clone)]
pub struct ClientConfig {
    /// API key (required for private endpoints).
    pub api_key: Option<String>,
    /// API secret (required for private endpoints).
    api_secret: Option<SecretString>,
    /// Environment (production, testnet, demo).
    pub environment: Environment,
    /// API region for production.
    pub region: ApiRegion,
    /// Custom base URL (overrides environment/region).
    pub base_url: Option<String>,
    /// Request validity window in milliseconds.
    pub recv_window: u32,
    /// Enable request/response logging.
    pub debug: bool,
    /// Request timeout in milliseconds.
    pub timeout_ms: u64,
    /// Referer header value.
    pub referer: Option<String>,
}

impl std::fmt::Debug for ClientConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientConfig")
            .field("api_key", &self.api_key.as_ref().map(|_| "[REDACTED]"))
            .field("api_secret", &self.api_secret.as_ref().map(|_| "[REDACTED]"))
            .field("environment", &self.environment)
            .field("region", &self.region)
            .field("base_url", &self.base_url)
            .field("recv_window", &self.recv_window)
            .field("debug", &self.debug)
            .field("timeout_ms", &self.timeout_ms)
            .field("referer", &self.referer)
            .finish()
    }
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            api_key: None,
            api_secret: None,
            environment: Environment::Production,
            region: ApiRegion::Default,
            base_url: None,
            recv_window: 5000,
            debug: false,
            timeout_ms: 10000,
            referer: None,
        }
    }
}

impl ClientConfig {
    /// Create a new configuration with API credentials.
    pub fn new(api_key: impl Into<String>, api_secret: impl Into<String>) -> Self {
        Self {
            api_key: Some(api_key.into()),
            api_secret: Some(SecretString::from(api_secret.into())),
            ..Default::default()
        }
    }

    /// Create a configuration for public endpoints only (no authentication).
    pub fn public_only() -> Self {
        Self::default()
    }

    /// Set the environment.
    pub fn environment(mut self, env: Environment) -> Self {
        self.environment = env;
        self
    }

    /// Use testnet environment.
    pub fn testnet(mut self) -> Self {
        self.environment = Environment::Testnet;
        self
    }

    /// Use demo trading environment.
    pub fn demo(mut self) -> Self {
        self.environment = Environment::Demo;
        self
    }

    /// Set the API region (for production only).
    pub fn region(mut self, region: ApiRegion) -> Self {
        self.region = region;
        self
    }

    /// Set a custom base URL.
    pub fn base_url(mut self, url: impl Into<String>) -> Self {
        self.base_url = Some(url.into());
        self
    }

    /// Set the recv_window value.
    pub fn recv_window(mut self, ms: u32) -> Self {
        self.recv_window = ms;
        self
    }

    /// Enable debug mode.
    pub fn debug(mut self, enabled: bool) -> Self {
        self.debug = enabled;
        self
    }

    /// Set request timeout.
    pub fn timeout_ms(mut self, ms: u64) -> Self {
        self.timeout_ms = ms;
        self
    }

    /// Set referer header.
    pub fn referer(mut self, referer: impl Into<String>) -> Self {
        self.referer = Some(referer.into());
        self
    }

    /// Get the effective REST API base URL.
    pub fn get_rest_url(&self) -> &str {
        if let Some(ref url) = self.base_url {
            return url;
        }

        match self.environment {
            Environment::Production => self.region.base_url(),
            Environment::Testnet => rest_urls::TESTNET,
            Environment::Demo => rest_urls::DEMO,
        }
    }

    /// Get the WebSocket URL for public streams.
    pub fn get_ws_public_url(&self, category: crate::types::Category) -> String {
        match self.environment {
            Environment::Production => ws_urls::public_mainnet(category),
            Environment::Testnet => ws_urls::public_testnet(category),
            Environment::Demo => ws_urls::public_demo(category),
        }
    }

    /// Get the WebSocket URL for private streams.
    pub fn get_ws_private_url(&self) -> &'static str {
        match self.environment {
            Environment::Production => ws_urls::PRIVATE_MAINNET,
            Environment::Testnet => ws_urls::PRIVATE_TESTNET,
            Environment::Demo => ws_urls::PRIVATE_DEMO,
        }
    }

    /// Get the WebSocket URL for trade API.
    pub fn get_ws_trade_url(&self) -> &'static str {
        match self.environment {
            Environment::Production => ws_urls::TRADE_MAINNET,
            Environment::Testnet => ws_urls::TRADE_TESTNET,
            Environment::Demo => ws_urls::TRADE_DEMO,
        }
    }

    /// Check if authentication is configured.
    pub fn has_credentials(&self) -> bool {
        self.api_key.is_some() && self.api_secret.is_some()
    }

    /// Get the API secret (for signing).
    pub(crate) fn get_secret(&self) -> Option<&str> {
        self.api_secret.as_ref().map(|s| s.expose_secret())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = ClientConfig::default();
        assert_eq!(config.get_rest_url(), rest_urls::MAINNET);
        assert!(!config.has_credentials());
    }

    #[test]
    fn test_testnet_config() {
        let config = ClientConfig::new("key", "secret").testnet();
        assert_eq!(config.get_rest_url(), rest_urls::TESTNET);
        assert!(config.has_credentials());
    }

    #[test]
    fn test_regional_config() {
        let config = ClientConfig::public_only().region(ApiRegion::HK);
        assert_eq!(config.get_rest_url(), rest_urls::HK);
    }

    #[test]
    fn test_custom_url() {
        let config = ClientConfig::public_only().base_url("https://custom.api.com");
        assert_eq!(config.get_rest_url(), "https://custom.api.com");
    }

    #[test]
    fn test_ws_urls() {
        use crate::types::Category;

        let config = ClientConfig::default();
        assert!(config
            .get_ws_public_url(Category::Linear)
            .contains("linear"));

        let testnet = ClientConfig::default().testnet();
        assert!(testnet.get_ws_private_url().contains("testnet"));
    }
}
