//! Error types for the Bybit client library.

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Main error type for the Bybit client.
#[derive(Error, Debug)]
pub enum BybitError {
    /// API returned an error response.
    #[error("API error (code={code}): {message}")]
    Api {
        code: i32,
        message: String,
        #[source]
        source: Option<Box<dyn std::error::Error + Send + Sync>>,
    },

    /// HTTP request failed.
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// Failed to serialize request parameters.
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    /// Failed to build URL.
    #[error("URL error: {0}")]
    Url(#[from] url::ParseError),

    /// Authentication error.
    #[error("Authentication error: {0}")]
    Auth(String),

    /// Invalid parameter provided.
    #[error("Invalid parameter: {0}")]
    InvalidParameter(String),

    /// WebSocket error.
    #[error("WebSocket error: {0}")]
    WebSocket(String),

    /// Rate limit exceeded.
    #[error("Rate limit exceeded: {message}")]
    RateLimit {
        message: String,
        retry_after_ms: Option<u64>,
    },

    /// Request timeout.
    #[error("Request timeout")]
    Timeout,

    /// Configuration error.
    #[error("Configuration error: {0}")]
    Config(String),
}

impl BybitError {
    /// Create a new API error from response data.
    pub fn api_error(code: i32, message: impl Into<String>) -> Self {
        BybitError::Api {
            code,
            message: message.into(),
            source: None,
        }
    }

    /// Check if this error is retryable.
    pub fn is_retryable(&self) -> bool {
        match self {
            BybitError::Http(e) => e.is_timeout() || e.is_connect(),
            BybitError::RateLimit { .. } => true,
            BybitError::Timeout => true,
            BybitError::Api { code, .. } => {
                // Known retryable API error codes
                matches!(code, 10002 | 10006 | 30034 | 30035 | 130035 | 130150)
            }
            _ => false,
        }
    }

    /// Get the API error code if this is an API error.
    pub fn api_code(&self) -> Option<i32> {
        match self {
            BybitError::Api { code, .. } => Some(*code),
            _ => None,
        }
    }
}

/// API response structure from Bybit V5.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T> {
    /// Return code. 0 means success.
    pub ret_code: i32,
    /// Return message.
    pub ret_msg: String,
    /// Response result data.
    pub result: T,
    /// Extended info (usually empty).
    #[serde(default)]
    pub ret_ext_info: serde_json::Value,
    /// Server timestamp in milliseconds.
    #[serde(default)]
    pub time: u64,
}

impl<T> ApiResponse<T> {
    /// Check if the response indicates success.
    pub fn is_success(&self) -> bool {
        self.ret_code == 0
    }

    /// Convert to a Result, returning an error if ret_code is non-zero.
    pub fn into_result(self) -> Result<T, BybitError> {
        if self.is_success() {
            Ok(self.result)
        } else {
            Err(BybitError::api_error(self.ret_code, self.ret_msg))
        }
    }
}

/// Paginated list response.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListResult<T> {
    /// Category (if applicable).
    #[serde(default)]
    pub category: Option<String>,
    /// List of items.
    pub list: Vec<T>,
    /// Cursor for next page (if paginated).
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

impl<T> ListResult<T> {
    /// Check if there are more pages.
    pub fn has_next_page(&self) -> bool {
        self.next_page_cursor
            .as_ref()
            .map(|c| !c.is_empty())
            .unwrap_or(false)
    }
}

/// Known API error codes.
pub mod error_codes {
    /// Success
    pub const SUCCESS: i32 = 0;

    /// Parameters missing or wrong
    pub const PARAMS_MISSING_OR_WRONG: i32 = 10001;

    /// Request timeout
    pub const REQUEST_TIMEOUT: i32 = 10002;

    /// Invalid API key or permissions
    pub const INVALID_API_KEY: i32 = 10003;

    /// Signature not valid
    pub const INVALID_SIGNATURE: i32 = 10004;

    /// Incorrect API key permissions
    pub const INCORRECT_PERMISSIONS: i32 = 10005;

    /// Too many requests (IP rate limit)
    pub const IP_RATE_LIMIT: i32 = 10006;

    /// Incorrect API request IP
    pub const INCORRECT_IP: i32 = 10010;

    /// Account not unified
    pub const ACCOUNT_NOT_UNIFIED: i32 = 10020;

    /// Order not found or too late to cancel
    pub const ORDER_NOT_FOUND: i32 = 20001;

    /// Insufficient balance for order
    pub const INSUFFICIENT_BALANCE: i32 = 30031;

    /// V5: Order not found
    pub const V5_ORDER_NOT_FOUND: i32 = 110001;

    /// V5: Insufficient balance
    pub const V5_INSUFFICIENT_BALANCE: i32 = 110007;

    /// V5: Position not found
    pub const V5_POSITION_NOT_FOUND: i32 = 110009;

    /// V5: Order quantity exceeds limit
    pub const V5_QTY_EXCEEDS_LIMIT: i32 = 110012;

    /// V5: Order price out of range
    pub const V5_PRICE_OUT_OF_RANGE: i32 = 110013;

    /// V5: Reduce only order would increase position
    pub const V5_REDUCE_ONLY_VIOLATION: i32 = 110017;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_api_response_success() {
        let json = r#"{
            "retCode": 0,
            "retMsg": "OK",
            "result": {"value": 123},
            "time": 1234567890
        }"#;

        let response: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(response.is_success());
        assert_eq!(response.result["value"], 123);
    }

    #[test]
    fn test_api_response_error() {
        let json = r#"{
            "retCode": 10001,
            "retMsg": "Param error",
            "result": {},
            "time": 1234567890
        }"#;

        let response: ApiResponse<serde_json::Value> = serde_json::from_str(json).unwrap();
        assert!(!response.is_success());

        let err = response.into_result().unwrap_err();
        assert_eq!(err.api_code(), Some(10001));
    }

    #[test]
    fn test_error_retryable() {
        let api_err = BybitError::api_error(10002, "timeout");
        assert!(api_err.is_retryable());

        let param_err = BybitError::api_error(10001, "param error");
        assert!(!param_err.is_retryable());
    }
}
