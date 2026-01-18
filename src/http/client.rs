//! HTTP client for the Bybit REST API.

use std::sync::atomic::{AtomicI64, Ordering};
use std::time::Duration;

use reqwest::{Client, Method, RequestBuilder, Response};
use serde::de::DeserializeOwned;
use serde::Serialize;
use tracing::{debug, trace, warn};

use crate::auth::{current_timestamp_ms, headers, sign_rest_request};
use crate::config::ClientConfig;
use crate::error::{ApiResponse, BybitError};

/// HTTP client for making REST API requests to Bybit.
#[derive(Debug)]
pub struct HttpClient {
    client: Client,
    config: ClientConfig,
    /// Time offset between local time and server time (in milliseconds).
    /// Positive means server is ahead of local.
    time_offset: AtomicI64,
}

impl HttpClient {
    /// Create a new HTTP client with the given configuration.
    pub fn new(config: ClientConfig) -> Result<Self, BybitError> {
        let timeout = Duration::from_millis(config.timeout_ms);

        let client = Client::builder()
            .timeout(timeout)
            .pool_max_idle_per_host(10)
            .build()
            .map_err(BybitError::Http)?;

        Ok(Self {
            client,
            config,
            time_offset: AtomicI64::new(0),
        })
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

        // Serialize query params
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

        // Serialize body
        let body_string = if let Some(b) = body {
            serde_json::to_string(b)?
        } else {
            String::new()
        };

        // Build full URL with query string for GET requests
        let full_url = if !query_string.is_empty() {
            format!("{}?{}", url, query_string)
        } else {
            url.clone()
        };

        let mut request = self.client.request(method.clone(), &full_url);

        // Add body for POST
        if !body_string.is_empty() {
            request = request
                .header("Content-Type", "application/json")
                .body(body_string.clone());
        }

        // Add authentication headers if signed
        if signed {
            request = self.add_auth_headers(request, &query_string, &body_string)?;
        }

        // Add referer if configured
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

        // Send request
        let response = request.send().await?;

        // Handle response
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

        // Payload is query string for GET, body for POST
        let payload = if body_string.is_empty() {
            query_string
        } else {
            body_string
        };

        let signature = sign_rest_request(timestamp, api_key, recv_window, payload, api_secret);

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

        if self.config.debug {
            debug!(status = %status, "Received response");
        }

        // Parse response body
        let body = response.text().await?;

        // Check for HTTP errors
        if !status.is_success() {
            warn!(status = %status, body = %body, "HTTP error");

            // Try to parse as API error
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

        // Parse as ApiResponse
        let api_response: ApiResponse<T> = serde_json::from_str(&body)?;

        // Check API-level errors
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

        // Calculate latency and offset
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

impl Clone for HttpClient {
    fn clone(&self) -> Self {
        Self {
            client: self.client.clone(),
            config: self.config.clone(),
            time_offset: AtomicI64::new(self.time_offset.load(Ordering::Relaxed)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_url() {
        let config = ClientConfig::default();
        let client = HttpClient::new(config).unwrap();
        let url = client.build_url("/v5/market/time");
        assert_eq!(url, "https://api.bybit.com/v5/market/time");
    }

    #[test]
    fn test_testnet_url() {
        let config = ClientConfig::default().testnet();
        let client = HttpClient::new(config).unwrap();
        let url = client.build_url("/v5/market/time");
        assert_eq!(url, "https://api-testnet.bybit.com/v5/market/time");
    }

    #[test]
    fn test_time_offset() {
        let config = ClientConfig::default();
        let client = HttpClient::new(config).unwrap();

        assert_eq!(client.time_offset(), 0);
        client.set_time_offset(1000);
        assert_eq!(client.time_offset(), 1000);
    }
}
