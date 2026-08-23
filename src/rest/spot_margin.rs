//! Spot margin trade API endpoints (UTA).

use crate::error::{BybitError, ListResult};
use crate::http::HttpClient;
use crate::types::spot_margin::*;

/// Spot margin service for UTA spot margin trading endpoints.
#[derive(Debug, Clone)]
pub struct SpotMarginService {
    http: HttpClient,
}

impl SpotMarginService {
    /// Create a new spot margin service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Turn spot margin trading on or off.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::spot_margin::SwitchSpotMarginModeParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    /// let result = client.spot_margin().switch_mode(&SwitchSpotMarginModeParams::on()).await?;
    /// println!("Spot margin mode: {}", result.spot_margin_mode);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn switch_mode(
        &self,
        params: &SwitchSpotMarginModeParams,
    ) -> Result<SpotMarginModeResult, BybitError> {
        self.http
            .post_signed("/v5/spot-margin-trade/switch-mode", Some(params))
            .await
    }

    /// Set the maximum leverage for spot margin.
    pub async fn set_leverage(
        &self,
        params: &SetSpotMarginLeverageParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/spot-margin-trade/set-leverage", Some(params))
            .await?;
        Ok(())
    }

    /// Get spot margin status and leverage.
    pub async fn get_state(&self) -> Result<SpotMarginState, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/state", None::<&()>)
            .await
    }

    /// Get VIP margin data (public).
    pub async fn get_vip_margin_data(
        &self,
        params: &GetVipMarginDataParams,
    ) -> Result<VipMarginDataResult, BybitError> {
        self.http
            .get("/v5/spot-margin-trade/data", Some(params))
            .await
    }

    /// Get tiered collateral ratio (public).
    pub async fn get_tiered_collateral_ratio(
        &self,
        params: &GetTieredCollateralRatioParams,
    ) -> Result<ListResult<TieredCollateralRatio>, BybitError> {
        self.http
            .get("/v5/spot-margin-trade/collateral", Some(params))
            .await
    }

    /// Get historical borrow interest rates (up to six months).
    pub async fn get_interest_rate_history(
        &self,
        params: &GetInterestRateHistoryParams,
    ) -> Result<InterestRateHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/interest-rate-history", Some(params))
            .await
    }

    /// Get the maximum borrowable amount for a coin.
    pub async fn get_max_borrowable(
        &self,
        params: &GetMaxBorrowableParams,
    ) -> Result<MaxBorrowableResult, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/max-borrowable", Some(params))
            .await
    }

    /// Get spot margin position tiers.
    pub async fn get_position_tiers(
        &self,
        params: &GetPositionTiersParams,
    ) -> Result<ListResult<CurrencyPositionTiers>, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/position-tiers", Some(params))
            .await
    }

    /// Get coin leverage state.
    pub async fn get_coin_state(
        &self,
        params: &GetCoinStateParams,
    ) -> Result<ListResult<CoinState>, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/coinstate", Some(params))
            .await
    }

    /// Get the amount available for loss-less repayment.
    pub async fn get_repayment_available_amount(
        &self,
        params: &GetRepaymentAvailableAmountParams,
    ) -> Result<RepaymentAvailableAmount, BybitError> {
        self.http
            .get_signed(
                "/v5/spot-margin-trade/repayment-available-amount",
                Some(params),
            )
            .await
    }

    /// Get the automatic repayment mode.
    ///
    /// When no currency is set, modes for all currencies are returned.
    pub async fn get_auto_repay_mode(
        &self,
        params: &GetAutoRepayModeParams,
    ) -> Result<AutoRepayModeResult, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/get-auto-repay-mode", Some(params))
            .await
    }

    /// Set the automatic repayment mode.
    ///
    /// When no currency is set, the mode applies to all currencies.
    pub async fn set_auto_repay_mode(
        &self,
        params: &SetAutoRepayModeParams,
    ) -> Result<AutoRepayModeResult, BybitError> {
        self.http
            .post_signed("/v5/spot-margin-trade/set-auto-repay-mode", Some(params))
            .await
    }

    /// Get liability information.
    pub async fn get_liability(
        &self,
        params: &GetLiabilityParams,
    ) -> Result<LiabilityInfo, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/liability", Some(params))
            .await
    }

    /// Get borrowing currency data.
    pub async fn get_currency_data(
        &self,
        params: &GetCurrencyDataParams,
    ) -> Result<ListResult<SpotMarginCurrencyData>, BybitError> {
        self.http
            .get_signed("/v5/spot-margin-trade/currency-data", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_switch_mode_params_serialization() {
        let params = SwitchSpotMarginModeParams::on();
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize switch mode params: {}", err),
        };
        assert_eq!(json, r#"{"spotMarginMode":"1"}"#);
    }

    #[test]
    fn test_set_auto_repay_mode_params_serialization() {
        let params = SetAutoRepayModeParams::on().currency("BTC");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize auto repay params: {}", err),
        };
        assert!(json.contains("\"autoRepayMode\":\"1\""));
        assert!(json.contains("\"currency\":\"BTC\""));
    }

    #[test]
    fn test_interest_rate_history_params_serialization() {
        let params = GetInterestRateHistoryParams::new("USDT").vip_level("No VIP");
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize interest rate params: {}", err),
        };
        assert!(query.contains("currency=USDT"));
        assert!(query.contains("vipLevel=No+VIP"));
    }
}
