//! Authentication and request signing for Bybit API.

use ring::hmac;

/// Sign a message using HMAC-SHA256.
///
/// Returns the signature as a hex-encoded string.
pub fn sign_hmac_sha256(message: &str, secret: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let signature = hmac::sign(&key, message.as_bytes());
    hex::encode(signature.as_ref())
}

/// Generate a signature for a REST API request.
///
/// The signature format for V5 API is:
/// `HMAC_SHA256(timestamp + api_key + recv_window + payload, api_secret)`
///
/// - For GET requests, `payload` is the query string (without leading `?`)
/// - For POST requests, `payload` is the JSON body
pub fn sign_rest_request(
    timestamp: u64,
    api_key: &str,
    recv_window: u32,
    payload: &str,
    api_secret: &str,
) -> String {
    let message = format!("{}{}{}{}", timestamp, api_key, recv_window, payload);
    sign_hmac_sha256(&message, api_secret)
}

/// Generate a signature for WebSocket authentication.
///
/// The signature format is: `HMAC_SHA256("GET/realtime" + expires, api_secret)`
pub fn sign_ws_auth(expires: u64, api_secret: &str) -> String {
    let message = format!("GET/realtime{}", expires);
    sign_hmac_sha256(&message, api_secret)
}

/// Get the current timestamp in milliseconds.
pub fn current_timestamp_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_else(|err| err.duration())
        .as_millis() as u64
}

/// HTTP headers required for authenticated requests.
pub mod headers {
    /// API key header.
    pub const API_KEY: &str = "X-BAPI-API-KEY";
    /// Timestamp header (milliseconds).
    pub const TIMESTAMP: &str = "X-BAPI-TIMESTAMP";
    /// Signature header.
    pub const SIGN: &str = "X-BAPI-SIGN";
    /// Receive window header (milliseconds).
    pub const RECV_WINDOW: &str = "X-BAPI-RECV-WINDOW";
    /// Sign type header (always "2" for V5).
    pub const SIGN_TYPE: &str = "X-BAPI-SIGN-TYPE";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hmac_signature() {
        let secret = "test_secret";
        let message = "test_message";
        let sig = sign_hmac_sha256(message, secret);

        assert_eq!(sig.len(), 64);
        assert!(sig.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_rest_signature() {
        let timestamp: u64 = 1658384314791;
        let api_key = "XXXXXXXX";
        let recv_window = 5000;
        let payload = r#"{"category":"linear","symbol":"BTCUSDT","side":"Buy","positionIdx":0,"orderType":"Limit","qty":"0.001","price":"18900","timeInForce":"GTC"}"#;
        let api_secret = "YYYYYYYY";

        let signature = sign_rest_request(timestamp, api_key, recv_window, payload, api_secret);

        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_ws_signature() {
        let expires: u64 = 1658384314791;
        let api_secret = "test_secret";

        let signature = sign_ws_auth(expires, api_secret);

        assert_eq!(signature.len(), 64);
        assert!(signature.chars().all(|c| c.is_ascii_hexdigit()));
    }

    #[test]
    fn test_signature_consistency() {
        let sig1 = sign_hmac_sha256("test", "secret");
        let sig2 = sign_hmac_sha256("test", "secret");
        assert_eq!(sig1, sig2);

        let sig3 = sign_hmac_sha256("test2", "secret");
        assert_ne!(sig1, sig3);
    }

    #[test]
    fn test_timestamp() {
        let ts = current_timestamp_ms();
        assert!(ts > 1577836800000); // Jan 1, 2020
    }

    #[test]
    fn test_get_signature_format() {
        let timestamp: u64 = 1658384314791;
        let api_key = "testkey";
        let recv_window = 5000;
        let query = "category=linear&symbol=BTCUSDT";
        let api_secret = "testsecret";

        let sig = sign_rest_request(timestamp, api_key, recv_window, query, api_secret);
        assert_eq!(sig.len(), 64);
    }
}
