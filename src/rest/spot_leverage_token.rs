//! Spot leverage token API endpoints.

use crate::error::{BybitError, ListResult};
use crate::http::HttpClient;
use crate::types::spot_leverage_token::*;

/// Spot leverage token service.
#[derive(Debug, Clone)]
pub struct SpotLeverageTokenService {
    http: HttpClient,
}

impl SpotLeverageTokenService {
    /// Create a new spot leverage token service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get leverage token information (public).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::spot_leverage_token::GetLeverageTokenInfoParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetLeverageTokenInfoParams::new().lt_coin("BTC3L");
    /// let result = client.spot_leverage_token().get_info(&params).await?;
    /// for token in &result.list {
    ///     println!("{}: status {}", token.lt_coin, token.lt_status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_info(
        &self,
        params: &GetLeverageTokenInfoParams,
    ) -> Result<ListResult<LeverageTokenInfo>, BybitError> {
        self.http.get("/v5/spot-lever-token/info", Some(params)).await
    }

    /// Get leverage token market reference (public).
    pub async fn get_reference(
        &self,
        params: &GetLeverageTokenReferenceParams,
    ) -> Result<LeverageTokenReference, BybitError> {
        self.http
            .get("/v5/spot-lever-token/reference", Some(params))
            .await
    }

    /// Purchase a leverage token.
    pub async fn purchase(
        &self,
        params: &PurchaseLeverageTokenParams,
    ) -> Result<PurchaseLeverageTokenResult, BybitError> {
        self.http
            .post_signed("/v5/spot-lever-token/purchase", Some(params))
            .await
    }

    /// Redeem a leverage token.
    pub async fn redeem(
        &self,
        params: &RedeemLeverageTokenParams,
    ) -> Result<RedeemLeverageTokenResult, BybitError> {
        self.http
            .post_signed("/v5/spot-lever-token/redeem", Some(params))
            .await
    }

    /// Get purchase and redemption records.
    pub async fn get_order_record(
        &self,
        params: &GetLeverageTokenOrderRecordParams,
    ) -> Result<ListResult<LeverageTokenOrderRecord>, BybitError> {
        self.http
            .get_signed("/v5/spot-lever-token/order-record", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_purchase_params_serialization() {
        let params = PurchaseLeverageTokenParams::new("BTC3L", "100").serial_no("abc-1");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize purchase params: {}", err),
        };
        assert!(json.contains("\"ltCoin\":\"BTC3L\""));
        assert!(json.contains("\"ltAmount\":\"100\""));
        assert!(json.contains("\"serialNo\":\"abc-1\""));
    }

    #[test]
    fn test_order_record_params_serialization() {
        let params = GetLeverageTokenOrderRecordParams::new()
            .lt_coin("BTC3L")
            .lt_order_type(1)
            .limit(20);
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize order record params: {}", err),
        };
        assert!(query.contains("ltCoin=BTC3L"));
        assert!(query.contains("ltOrderType=1"));
        assert!(query.contains("limit=20"));
    }
}
