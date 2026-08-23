//! Earn API endpoints for savings products.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::earn::*;

/// Earn service for savings product endpoints.
#[derive(Debug, Clone)]
pub struct EarnService {
    http: HttpClient,
}

impl EarnService {
    /// Create a new earn service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get earn product info.
    ///
    /// This is a public endpoint.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::earn::GetEarnProductParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    ///
    /// let params = GetEarnProductParams::new("FlexibleSaving").coin("USDT");
    /// let result = client.earn().get_product_info(&params).await?;
    /// for product in &result.list {
    ///     println!("{}: {:?}", product.coin, product.estimate_apr);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_product_info(
        &self,
        params: &GetEarnProductParams,
    ) -> Result<EarnProductResult, BybitError> {
        self.http.get("/v5/earn/product", Some(params)).await
    }

    /// Place a stake or redeem order.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::earn::PlaceEarnOrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = PlaceEarnOrderParams::stake(
    ///     "FlexibleSaving", "FUND", "100", "USDT", "3", "my-order-1"
    /// );
    /// let result = client.earn().place_order(&params).await?;
    /// println!("Order ID: {}", result.order_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn place_order(
        &self,
        params: &PlaceEarnOrderParams,
    ) -> Result<EarnOrderResult, BybitError> {
        self.http
            .post_signed("/v5/earn/place-order", Some(params))
            .await
    }

    /// Get stake or redemption order history.
    ///
    /// For the `OnChain` category either `order_id` or `order_link_id` is required.
    pub async fn get_order_history(
        &self,
        params: &GetEarnOrderHistoryParams,
    ) -> Result<EarnOrderHistoryResult, BybitError> {
        self.http.get_signed("/v5/earn/order", Some(params)).await
    }

    /// Get staked positions.
    ///
    /// Fully redeemed positions are also returned.
    pub async fn get_position(
        &self,
        params: &GetEarnPositionParams,
    ) -> Result<EarnPositionResult, BybitError> {
        self.http
            .get_signed("/v5/earn/position", Some(params))
            .await
    }

    /// Modify an OnChain earn position (e.g. auto reinvest).
    pub async fn modify_position(
        &self,
        params: &ModifyEarnPositionParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/earn/position/modify", Some(params))
            .await?;
        Ok(())
    }

    /// Get yield history.
    ///
    /// Do not pass `product_id` for the `OnChain` category.
    pub async fn get_yield(
        &self,
        params: &GetEarnYieldParams,
    ) -> Result<EarnYieldResult, BybitError> {
        self.http.get_signed("/v5/earn/yield", Some(params)).await
    }

    /// Get hourly yield history.
    pub async fn get_hourly_yield(
        &self,
        params: &GetEarnYieldParams,
    ) -> Result<EarnHourlyYieldResult, BybitError> {
        self.http
            .get_signed("/v5/earn/hourly-yield", Some(params))
            .await
    }

    /// Get historical APR.
    ///
    /// This is a public endpoint.
    pub async fn get_apr_history(
        &self,
        params: &GetEarnAprHistoryParams,
    ) -> Result<EarnAprHistoryResult, BybitError> {
        self.http.get("/v5/earn/apr-history", Some(params)).await
    }

    /// Get coupon list.
    pub async fn get_coupons(
        &self,
        params: &GetEarnCouponsParams,
    ) -> Result<EarnCouponsResult, BybitError> {
        self.http.get_signed("/v5/earn/coupons", Some(params)).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_place_order_params_serialization() {
        let params =
            PlaceEarnOrderParams::stake("FlexibleSaving", "FUND", "100", "USDT", "3", "order-1");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize earn order params: {}", err),
        };
        assert!(json.contains("\"category\":\"FlexibleSaving\""));
        assert!(json.contains("\"orderType\":\"Stake\""));
        assert!(json.contains("\"accountType\":\"FUND\""));
        assert!(json.contains("\"amount\":\"100\""));
        assert!(json.contains("\"productId\":\"3\""));
        assert!(json.contains("\"orderLinkId\":\"order-1\""));
        assert!(!json.contains("toAccountType"));
    }

    #[test]
    fn test_redeem_params_serialization() {
        let params =
            PlaceEarnOrderParams::redeem("FlexibleSaving", "UNIFIED", "50", "USDT", "3", "order-2")
                .to_account_type("FUND");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize earn redeem params: {}", err),
        };
        assert!(json.contains("\"orderType\":\"Redeem\""));
        assert!(json.contains("\"toAccountType\":\"FUND\""));
    }

    #[test]
    fn test_order_history_params_serialization() {
        let params = GetEarnOrderHistoryParams::new("FlexibleSaving")
            .product_id("3")
            .limit(20);
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize order history params: {}", err),
        };
        assert!(query.contains("category=FlexibleSaving"));
        assert!(query.contains("productId=3"));
        assert!(query.contains("limit=20"));
    }

    #[test]
    fn test_modify_position_params_serialization() {
        let params = ModifyEarnPositionParams::new(430, 102, true);
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize modify position params: {}", err),
        };
        assert!(json.contains("\"category\":\"OnChain\""));
        assert!(json.contains("\"productId\":430"));
        assert!(json.contains("\"positionId\":102"));
        assert!(json.contains("\"autoReinvest\":1"));
    }

    #[test]
    fn test_yield_result_deserialization() {
        let json = r#"{
            "yield": [{
                "productId": "3",
                "coin": "USDT",
                "id": "1",
                "amount": "0.01",
                "status": "Success"
            }],
            "nextPageCursor": "abc"
        }"#;
        let result: EarnYieldResult = match serde_json::from_str(json) {
            Ok(result) => result,
            Err(err) => panic!("Failed to deserialize yield result: {}", err),
        };
        assert_eq!(result.yields.len(), 1);
        assert_eq!(result.yields[0].coin, "USDT");
        assert_eq!(result.next_page_cursor, Some("abc".to_string()));
    }
}
