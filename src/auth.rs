//! Authentication and request signing for Bybit API.

use base64::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;
use ring::hmac;

use crate::error::BybitError;

/// Sign a message using HMAC-SHA256.
///
/// Returns the signature as a hex-encoded string.
pub fn sign_hmac_sha256(message: &str, secret: &str) -> String {
    let key = hmac::Key::new(hmac::HMAC_SHA256, secret.as_bytes());
    let signature = hmac::sign(&key, message.as_bytes());
    hex::encode(signature.as_ref())
}

/// Check if a secret is an RSA private key in PEM format.
///
/// Bybit RSA API keys use a PEM encoded private key as the secret.
pub fn is_rsa_secret(secret: &str) -> bool {
    secret.contains("PRIVATE KEY")
}

/// Decode a PEM private key into DER bytes.
fn pem_to_der(pem: &str) -> Result<Vec<u8>, BybitError> {
    let body: String = pem
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with("-----"))
        .collect();
    BASE64
        .decode(body.as_bytes())
        .map_err(|err| BybitError::Auth(format!("Invalid PEM private key: {}", err)))
}

/// Sign a message using RSASSA-PKCS1-v1_5 with SHA-256.
///
/// The secret must be a PEM encoded PKCS8 or PKCS1 RSA private key.
/// Returns the signature as a base64-encoded string.
pub fn sign_rsa_sha256(message: &str, private_key_pem: &str) -> Result<String, BybitError> {
    let der = pem_to_der(private_key_pem)?;
    let key_pair = ring::signature::RsaKeyPair::from_pkcs8(&der)
        .or_else(|_| ring::signature::RsaKeyPair::from_der(&der))
        .map_err(|err| BybitError::Auth(format!("Invalid RSA private key: {}", err)))?;

    let rng = ring::rand::SystemRandom::new();
    let mut signature = vec![0u8; key_pair.public().modulus_len()];
    key_pair
        .sign(
            &ring::signature::RSA_PKCS1_SHA256,
            &rng,
            message.as_bytes(),
            &mut signature,
        )
        .map_err(|err| BybitError::Auth(format!("RSA signing failed: {}", err)))?;

    Ok(BASE64.encode(signature))
}

/// Sign a message, selecting the algorithm from the secret format.
///
/// A PEM private key secret selects RSA, any other secret selects HMAC.
pub fn sign_message(message: &str, secret: &str) -> Result<String, BybitError> {
    if is_rsa_secret(secret) {
        sign_rsa_sha256(message, secret)
    } else {
        Ok(sign_hmac_sha256(message, secret))
    }
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

    // Throwaway key generated for tests only, not used anywhere.
    const TEST_RSA_KEY: &str = "-----BEGIN PRIVATE KEY-----
MIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDLsfXuIY4e6bpB
Z5QD69ho27NEAVz6RwwfeOWj6etTFFD9TfjZ00fohfza7MAlwlzO1FgeUb0gHgi5
sxBhgGhCf0p6/hrezVkzs2RI/pJoZce2C3gnYvrVtKOb6GVUMI6sCd65YUCO4UuS
9HqSeHPedNIBbn0ck5x1eQiml8dfX/87Ck2YDrxawdFKe88mdydwzI4t0Fckos65
4FlsTB3rRPRUxMglEBZzCXZQREfFHXemVM5zxoJPrn9CIDJ7RT4FpHjUc0ostbYh
w1DVCiKWOqh7O6qHa7ntOoph6AVbxP6VqBl4qQBAuChIJovP21UJxBNwFBhUSgBG
hyMynLQBAgMBAAECggEAAalz1kPBvSIUTf65DrqiAddYDDbJd3UfWyg2pxqW8U8L
QEkpHdLbMCFxZZJfWMzatRPMXRahjnARoL0+UG4BJm4Thshzt5a76B/KB81EQck3
IswPaLKGVXpzXXJnwD2jWQc9a6xd1S9Eo/VuGhY/lhnZ0Z9BMqZdf+OzH2kshuGg
g890xAg+78VZPKY7Jhgeeuls55bgNoJvdWfTGnXa8su2mryynG5hrmGe6ZUmrVWC
u04erY2Fg0LtHr65r4FyJDqOoEWPZj9l4eKLrfxToRhiTU+cW0lOme2s6v3VJfQI
IijhcPR+uiUKAmWeJBFrKhF4axob87PzPdlvHXMmaQKBgQD3RnvDYiLv9dTpt84R
Kupv86B3fIZdEQEhdAhk+x4BITNV16GYTQu9SaJ9ZAweTyFWtoeVMtoSitGZ8H/2
w7O4mnHjNAT7xAoZPZVgviklUtudDr9Y7P14iAdRXihqokwnVxy44g4tI6vHXzy5
6nJoMO9pL/0gDzd4rhEj39yXCQKBgQDS4dbG8I3qWVag1hmpKQEAU4t9o9lXlzN/
B0TZjfxYNCNC8tlUhwJmPeeKNeYg+Ex9bTrE81kLjTIXznVRNfWh5sibfmOm0C5G
ChikSi7+N9rGY175VxX0/88Dyqzis6QCBaoR2aR+BpkzmOsohgwSshscHz2+cvYH
nyjLIjA7OQKBgQCGg3YiBEmjJaTL+ywDFMhxXCOHvFrPV8e4rgk+/YgF70ygvutu
EUN4lgzpjzo0ZrFpHWRGGmKcwZ1h032Y/D7RBls70HmBh9RChMi01t1FK3zCRFp9
AnakUcNBVSpBT8aDhBTg0kAJ+CSjEUDUlh/kY/Jo0y0senqfkHlMfaPfGQKBgAK1
mBbSPGCIaU6Dsay0tCiW6jHhCwfPD4BQTZgl9NoFQXa4vAYT/fnmTbU+4AgIaKMr
5hdlsP0vATfBFUzHzrIXzvgAdbNM9Ws88fgaY+QsP4lQ+YVmkwekyboYvoPjHvZP
sFGuZCiiWRGKo2rP3hPXafXAiQCLZZHPf6ysccYxAoGACKZnWgkhHNtADp/991hU
lZ+nzfaFyHO2DoESDN69/MtUD/enjcCOiMQlsZQVy06yF6UPlfcFdmRFWghNZshz
J5D/4jjNOidOnvtdox8E2/SVlUGjhtxi0RTmp1AJTEcdpgAVx3lXxnt6vGXTMYyr
chkBmX6vWB0orVZpFnmB1iU=
-----END PRIVATE KEY-----";

    #[test]
    fn test_rsa_secret_detection() {
        assert!(is_rsa_secret(TEST_RSA_KEY));
        assert!(!is_rsa_secret("plain_hmac_secret"));
    }

    #[test]
    fn test_rsa_signature() {
        let sig = match sign_rsa_sha256("test_message", TEST_RSA_KEY) {
            Ok(sig) => sig,
            Err(err) => panic!("RSA signing failed: {}", err),
        };

        // 2048-bit RSA signature is 256 bytes, 344 base64 characters.
        assert_eq!(sig.len(), 344);
    }

    #[test]
    fn test_sign_message_dispatch() {
        let hmac_sig = match sign_message("test", "secret") {
            Ok(sig) => sig,
            Err(err) => panic!("HMAC signing failed: {}", err),
        };
        assert_eq!(hmac_sig, sign_hmac_sha256("test", "secret"));

        let rsa_sig = match sign_message("test", TEST_RSA_KEY) {
            Ok(sig) => sig,
            Err(err) => panic!("RSA signing failed: {}", err),
        };
        assert_ne!(rsa_sig, hmac_sig);
    }

    #[test]
    fn test_invalid_rsa_key() {
        let result = sign_rsa_sha256("test", "-----BEGIN PRIVATE KEY-----\nnot base64!!\n-----END PRIVATE KEY-----");
        assert!(result.is_err());
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
