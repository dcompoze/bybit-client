//! Common types shared across the API.

use serde::{Deserialize, Serialize};

/// Server time response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerTime {
    /// Server time in seconds (string).
    pub time_second: String,
    /// Server time in nanoseconds (string).
    pub time_nano: String,
}

impl ServerTime {
    /// Get the server time in milliseconds.
    pub fn as_millis(&self) -> u64 {
        self.time_nano
            .parse::<u64>()
            .map(|ns| ns / 1_000_000)
            .unwrap_or(0)
    }
}

/// Rate limit information parsed from response headers.
#[derive(Debug, Clone, Default)]
pub struct RateLimitInfo {
    /// Remaining requests in the current window.
    pub remaining: Option<u32>,
    /// Maximum requests per window.
    pub limit: Option<u32>,
    /// Reset timestamp in milliseconds.
    pub reset_at: Option<u64>,
}

/// Generic cursor-based pagination parameters.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginationParams {
    /// Maximum number of items to return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl PaginationParams {
    /// Create new pagination parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

/// Empty result type for endpoints that return no data.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Empty {}

/// Deserialize an optional field that the API may return as a string or an integer.
/// Bybit changed some fields (e.g. `smpGroup`) from integer to string.
pub fn string_or_int<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrInt {
        String(String),
        Int(i64),
    }

    Ok(Option::<StringOrInt>::deserialize(deserializer)?.map(|v| match v {
        StringOrInt::String(s) => s,
        StringOrInt::Int(i) => i.to_string(),
    }))
}

/// Symbol information base fields (common across categories).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SymbolInfo {
    /// Trading symbol.
    pub symbol: String,
    /// Base currency.
    #[serde(default)]
    pub base_coin: Option<String>,
    /// Quote currency.
    #[serde(default)]
    pub quote_coin: Option<String>,
    /// Symbol status.
    #[serde(default)]
    pub status: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_server_time() {
        let st = ServerTime {
            time_second: "1699123456".to_string(),
            time_nano: "1699123456789000000".to_string(),
        };
        assert_eq!(st.as_millis(), 1699123456789);
    }

    #[test]
    fn test_pagination() {
        let params = PaginationParams::new().limit(50).cursor("abc123");
        assert_eq!(params.limit, Some(50));
        assert_eq!(params.cursor, Some("abc123".to_string()));
    }
}
