//! HTTP client for the Bybit REST API.

use std::sync::atomic::{AtomicI64, Ordering};
use std::sync::{Arc, RwLock};
use std::time::Duration;

use reqwest::header::HeaderMap;
use reqwest::{Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, trace, warn};

use crate::auth::{current_timestamp_ms, headers, sign_message};
use crate::config::ClientConfig;
use crate::error::{ApiResponse, BybitError};
use crate::types::common::RateLimitInfo;

/// HTTP client for making REST API requests to Bybit.
#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    config: ClientConfig,
    /// Time offset between local time and server time (in milliseconds).
    /// Positive means server is ahead of local.
    time_offset: AtomicI64,
    /// Rate limit information from the most recent response.
    rate_limit: Arc<RwLock<Option<RateLimitInfo>>>,
}

impl HttpClient {
    /// Create a new HTTP client with the given configuration.
    pub fn new(config: ClientConfig) -> Result<Self, BybitError> {
        let timeout = Duration::from_millis(config.timeout_ms);

        let builder = Client::builder()
            .timeout(timeout)
            .pool_max_idle_per_host(10);

        #[cfg(feature = "native-tls")]
        let builder = builder.tls_backend_native();

        #[cfg(feature = "rustls-tls")]
        let builder = builder.tls_backend_rustls();

        let client = builder.build().map_err(BybitError::Http)?;

        Ok(Self {
            client,
            config,
            time_offset: AtomicI64::new(0),
            rate_limit: Arc::new(RwLock::new(None)),
        })
    }

    /// Get the rate limit information from the most recent response.
    pub fn last_rate_limit(&self) -> Option<RateLimitInfo> {
        self.rate_limit.read().ok().and_then(|guard| guard.clone())
    }

    /// Get the client configuration.
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Get the current time offset.
    pub fn time_offset(&self) -> i64 {
        self.time_offset.load(Ordering::Relaxed)
    }

    /// Set the time offset manually.
    pub fn set_time_offset(&self, offset_ms: i64) {
        self.time_offset.store(offset_ms, Ordering::Relaxed);
    }

    /// Get the adjusted timestamp (local time + offset).
    fn get_timestamp(&self) -> u64 {
        let local = current_timestamp_ms() as i64;
        let offset = self.time_offset.load(Ordering::Relaxed);
        (local + offset) as u64
    }

    /// Build the full URL for an endpoint.
    fn build_url(&self, endpoint: &str) -> String {
        format!("{}{}", self.config.get_rest_url(), endpoint)
    }

    /// Make a public (unauthenticated) GET request.
    pub async fn get<T, P>(&self, endpoint: &str, params: Option<&P>) -> Result<T, BybitError>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        self.request(Method::GET, endpoint, params, None::<&()>, false)
            .await
    }

    /// Make a private (authenticated) GET request.
    pub async fn get_signed<T, P>(
        &self,
        endpoint: &str,
        params: Option<&P>,
    ) -> Result<T, BybitError>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
    {
        self.request(Method::GET, endpoint, params, None::<&()>, true)
            .await
    }

    /// Make a private (authenticated) POST request.
    pub async fn post_signed<T, B>(
        &self,
        endpoint: &str,
        body: Option<&B>,
    ) -> Result<T, BybitError>
    where
        T: DeserializeOwned,
        B: Serialize + ?Sized,
    {
        self.request(Method::POST, endpoint, None::<&()>, body, true)
            .await
    }

    /// Internal method to make HTTP requests.
    async fn request<T, P, B>(
        &self,
        method: Method,
        endpoint: &str,
        params: Option<&P>,
        body: Option<&B>,
        signed: bool,
    ) -> Result<T, BybitError>
    where
        T: DeserializeOwned,
        P: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let url = self.build_url(endpoint);

        let query_string = if let Some(p) = params {
            serde_urlencoded::to_string(p).map_err(|e| {
                BybitError::Serialization(serde_json::Error::io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    e.to_string(),
                )))
            })?
        } else {
            String::new()
        };

        let body_string = if let Some(b) = body {
            serde_json::to_string(b)?
        } else {
            String::new()
        };

        let full_url = if !query_string.is_empty() {
            format!("{}?{}", url, query_string)
        } else {
            url.clone()
        };

        let mut request = self.client.request(method.clone(), &full_url);

        if !body_string.is_empty() {
            request = request
                .header("Content-Type", "application/json")
                .body(body_string.clone());
        }

        if signed {
            request = self.add_auth_headers(request, &query_string, &body_string)?;
        }

        if let Some(ref referer) = self.config.referer {
            request = request.header("Referer", referer);
        }

        if self.config.debug {
            debug!(
                method = %method,
                url = %full_url,
                signed = signed,
                "Sending request"
            );
            if !body_string.is_empty() {
                trace!(body = %body_string, "Request body");
            }
        }

        let response = request.send().await?;

        self.handle_response(response).await
    }

    /// Add authentication headers to a request.
    fn add_auth_headers(
        &self,
        request: RequestBuilder,
        query_string: &str,
        body_string: &str,
    ) -> Result<RequestBuilder, BybitError> {
        let api_key = self
            .config
            .api_key
            .as_ref()
            .ok_or_else(|| BybitError::Auth("API key not configured".to_string()))?;

        let api_secret = self
            .config
            .get_secret()
            .ok_or_else(|| BybitError::Auth("API secret not configured".to_string()))?;

        let timestamp = self.get_timestamp();
        let recv_window = self.config.recv_window;

        let payload = if body_string.is_empty() {
            query_string
        } else {
            body_string
        };

        let message = format!("{}{}{}{}", timestamp, api_key, recv_window, payload);
        let signature = sign_message(&message, api_secret)?;

        Ok(request
            .header(headers::API_KEY, api_key)
            .header(headers::TIMESTAMP, timestamp.to_string())
            .header(headers::SIGN, signature)
            .header(headers::RECV_WINDOW, recv_window.to_string())
            .header(headers::SIGN_TYPE, "2"))
    }

    /// Handle the HTTP response.
    async fn handle_response<T: DeserializeOwned>(
        &self,
        response: Response,
    ) -> Result<T, BybitError> {
        let status = response.status();

        if let Some(info) = parse_rate_limit_headers(response.headers()) {
            if let Ok(mut guard) = self.rate_limit.write() {
                *guard = Some(info);
            }
        }

        if self.config.debug {
            debug!(status = %status, "Received response");
        }

        let body = response.text().await?;

        if !status.is_success() {
            warn!(status = %status, body = %body, "HTTP error");

            if let Ok(api_response) = serde_json::from_str::<ApiResponse<serde_json::Value>>(&body)
            {
                return Err(BybitError::api_error(
                    api_response.ret_code,
                    api_response.ret_msg,
                ));
            }

            return Err(BybitError::api_error(
                status.as_u16() as i32,
                format!("HTTP error {}: {}", status, body),
            ));
        }

        if self.config.debug {
            trace!(body = %body, "Response body");
        }

        let api_response: ApiResponse<T> = serde_json::from_str(&body)?;

        api_response.into_result()
    }

    /// Synchronize time with the server.
    ///
    /// This fetches the server time and calculates the offset between
    /// local time and server time. The offset is used to adjust
    /// timestamps in authenticated requests.
    pub async fn sync_time(&self) -> Result<i64, BybitError> {
        use crate::types::ServerTime;

        let start = current_timestamp_ms();
        let server_time: ServerTime = self.get("/v5/market/time", None::<&()>).await?;
        let end = current_timestamp_ms();

        let latency = (end - start) / 2;
        let server_ms = server_time.as_millis();
        let offset = server_ms as i64 - (end as i64) + (latency as i64);

        self.time_offset.store(offset, Ordering::Relaxed);

        debug!(
            server_time = server_ms,
            local_time = end,
            latency = latency,
            offset = offset,
            "Time synchronized"
        );

        Ok(offset)
    }
}

/// Parse rate limit headers from a response.
///
/// Bybit returns `X-Bapi-Limit` (max per window), `X-Bapi-Limit-Status`
/// (remaining), and `X-Bapi-Limit-Reset-Timestamp` (reset time in ms).
fn parse_rate_limit_headers(headers: &HeaderMap) -> Option<RateLimitInfo> {
    let get = |name: &str| headers.get(name).and_then(|v| v.to_str().ok());

    let limit = get("x-bapi-limit").and_then(|v| v.parse().ok());
    let remaining = get("x-bapi-limit-status").and_then(|v| v.parse().ok());
    let reset_at = get("x-bapi-limit-reset-timestamp").and_then(|v| v.parse().ok());

    if limit.is_none() && remaining.is_none() && reset_at.is_none() {
        return None;
    }

    Some(RateLimitInfo {
        remaining,
        limit,
        reset_at,
    })
}

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
            time_offset: AtomicI64::new(self.time_offset.load(Ordering::Relaxed)),
            rate_limit: Arc::clone(&self.rate_limit),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let config = ClientConfig::default();
        let client = match HttpClient::new(config) {
            Ok(client) => client,
            Err(err) => panic!("Failed to build HTTP client: {}", err),
        };
        let url = client.build_url("/v5/market/time");
        assert_eq!(url, "https://api.bybit.com/v5/market/time");
    }

    #[test]
    fn test_testnet_url() {
        let config = ClientConfig::default().testnet();
        let client = match HttpClient::new(config) {
            Ok(client) => client,
            Err(err) => panic!("Failed to build HTTP client: {}", err),
        };
        let url = client.build_url("/v5/market/time");
        assert_eq!(url, "https://api-testnet.bybit.com/v5/market/time");
    }

    #[test]
    fn test_parse_rate_limit_headers() {
        let mut headers = HeaderMap::new();
        headers.insert("X-Bapi-Limit", "120".parse().unwrap());
        headers.insert("X-Bapi-Limit-Status", "118".parse().unwrap());
        headers.insert(
            "X-Bapi-Limit-Reset-Timestamp",
            "1699123456789".parse().unwrap(),
        );

        let info = match parse_rate_limit_headers(&headers) {
            Some(info) => info,
            None => panic!("Expected rate limit info"),
        };
        assert_eq!(info.limit, Some(120));
        assert_eq!(info.remaining, Some(118));
        assert_eq!(info.reset_at, Some(1699123456789));

        assert!(parse_rate_limit_headers(&HeaderMap::new()).is_none());
    }

    #[test]
    fn test_time_offset() {
        let config = ClientConfig::default();
        let client = match HttpClient::new(config) {
            Ok(client) => client,
            Err(err) => panic!("Failed to build HTTP client: {}", err),
        };

        assert_eq!(client.time_offset(), 0);
        client.set_time_offset(1000);
        assert_eq!(client.time_offset(), 1000);
    }
}
