//! Crypto loan API endpoints.
//!
//! Covers the new crypto loan API: the common, flexible, and fixed groups.
//! The legacy `/v5/crypto-loan/*` endpoints are deprecated and not included.
//! All endpoints require authentication.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::crypto_loan::*;

/// Crypto loan service.
#[derive(Debug, Clone)]
pub struct CryptoLoanService {
    http: HttpClient,
}

impl CryptoLoanService {
    /// Create a new service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get collateral coin data.
    pub async fn get_collateral_data(
        &self,
        params: &GetCollateralDataParams,
    ) -> Result<CollateralDataResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-common/collateral-data", Some(params))
            .await
    }

    /// Get loanable coin data.
    pub async fn get_loanable_data(
        &self,
        params: &GetLoanableDataParams,
    ) -> Result<LoanableDataResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-common/loanable-data", Some(params))
            .await
    }

    /// Get the crypto loan position.
    pub async fn get_position(&self) -> Result<CryptoLoanPositionResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-common/position", None::<&()>)
            .await
    }

    /// Get the maximum collateral reduction amount.
    pub async fn get_max_collateral_amount(
        &self,
        params: &GetMaxCollateralAmountParams,
    ) -> Result<MaxCollateralAmountResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-common/max-collateral-amount", Some(params))
            .await
    }

    /// Get the maximum loan amount and remaining platform limit.
    pub async fn get_max_loan_amount(
        &self,
        params: &GetMaxLoanAmountParams,
    ) -> Result<MaxLoanAmountResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-common/max-loan", Some(params))
            .await
    }

    /// Adjust collateral amount (LTV).
    pub async fn adjust_ltv(&self, params: &AdjustLtvParams) -> Result<AdjustLtvResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-common/adjust-ltv", Some(params))
            .await
    }

    /// Get LTV adjustment history.
    pub async fn get_adjustment_history(
        &self,
        params: &GetAdjustmentHistoryParams,
    ) -> Result<AdjustmentHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-common/adjustment-history", Some(params))
            .await
    }

    /// Borrow a flexible loan.
    pub async fn flexible_borrow(
        &self,
        params: &FlexibleBorrowParams,
    ) -> Result<FlexibleBorrowResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-flexible/borrow", Some(params))
            .await
    }

    /// Repay a flexible loan.
    pub async fn flexible_repay(
        &self,
        params: &FlexibleRepayParams,
    ) -> Result<FlexibleRepayResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-flexible/repay", Some(params))
            .await
    }

    /// Repay a flexible loan with collateral.
    pub async fn flexible_repay_collateral(
        &self,
        params: &RepayCollateralParams,
    ) -> Result<FlexibleRepayResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-flexible/repay-collateral", Some(params))
            .await
    }

    /// Get ongoing flexible loans.
    pub async fn get_flexible_ongoing_coin(
        &self,
        params: &GetFlexibleOngoingCoinParams,
    ) -> Result<FlexibleOngoingCoinResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-flexible/ongoing-coin", Some(params))
            .await
    }

    /// Get flexible borrow history.
    pub async fn get_flexible_borrow_history(
        &self,
        params: &GetFlexibleBorrowHistoryParams,
    ) -> Result<FlexibleBorrowHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-flexible/borrow-history", Some(params))
            .await
    }

    /// Get flexible repayment history.
    pub async fn get_flexible_repayment_history(
        &self,
        params: &GetFlexibleRepaymentHistoryParams,
    ) -> Result<FlexibleRepaymentHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-flexible/repayment-history", Some(params))
            .await
    }

    /// Get the fixed borrow order quote market.
    pub async fn get_fixed_borrow_order_quote(
        &self,
        params: &GetFixedOrderQuoteParams,
    ) -> Result<FixedOrderQuoteResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/borrow-order-quote", Some(params))
            .await
    }

    /// Get the fixed supply order quote market.
    pub async fn get_fixed_supply_order_quote(
        &self,
        params: &GetFixedOrderQuoteParams,
    ) -> Result<FixedOrderQuoteResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/supply-order-quote", Some(params))
            .await
    }

    /// Create a fixed borrow order.
    pub async fn fixed_borrow(
        &self,
        params: &FixedBorrowParams,
    ) -> Result<FixedOrderResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-fixed/borrow", Some(params))
            .await
    }

    /// Create a fixed supply (lending) order.
    pub async fn fixed_supply(
        &self,
        params: &FixedSupplyParams,
    ) -> Result<FixedOrderResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-fixed/supply", Some(params))
            .await
    }

    /// Cancel a fixed borrow order.
    pub async fn cancel_fixed_borrow_order(
        &self,
        params: &CancelFixedOrderParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/crypto-loan-fixed/borrow-order-cancel", Some(params))
            .await?;
        Ok(())
    }

    /// Cancel a fixed supply order.
    pub async fn cancel_fixed_supply_order(
        &self,
        params: &CancelFixedOrderParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/crypto-loan-fixed/supply-order-cancel", Some(params))
            .await?;
        Ok(())
    }

    /// Get fixed borrow order info.
    pub async fn get_fixed_borrow_order_info(
        &self,
        params: &GetFixedOrderInfoParams,
    ) -> Result<FixedOrderInfoResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/borrow-order-info", Some(params))
            .await
    }

    /// Get fixed supply order info.
    pub async fn get_fixed_supply_order_info(
        &self,
        params: &GetFixedOrderInfoParams,
    ) -> Result<FixedOrderInfoResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/supply-order-info", Some(params))
            .await
    }

    /// Get fixed borrow contract info.
    pub async fn get_fixed_borrow_contract_info(
        &self,
        params: &GetFixedContractInfoParams,
    ) -> Result<FixedContractInfoResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/borrow-contract-info", Some(params))
            .await
    }

    /// Get fixed supply contract info.
    pub async fn get_fixed_supply_contract_info(
        &self,
        params: &GetFixedContractInfoParams,
    ) -> Result<FixedContractInfoResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/supply-contract-info", Some(params))
            .await
    }

    /// Fully repay fixed loans.
    pub async fn fixed_fully_repay(
        &self,
        params: &FixedFullyRepayParams,
    ) -> Result<FixedRepayResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-fixed/fully-repay", Some(params))
            .await
    }

    /// Repay fixed loans with collateral.
    pub async fn fixed_repay_collateral(
        &self,
        params: &RepayCollateralParams,
    ) -> Result<FixedRepayResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-fixed/repay-collateral", Some(params))
            .await
    }

    /// Renew a fixed loan.
    pub async fn fixed_renew(
        &self,
        params: &FixedRenewParams,
    ) -> Result<FixedOrderResult, BybitError> {
        self.http
            .post_signed("/v5/crypto-loan-fixed/renew", Some(params))
            .await
    }

    /// Get fixed loan renewal orders.
    pub async fn get_fixed_renew_info(
        &self,
        params: &GetFixedRenewInfoParams,
    ) -> Result<FixedRenewInfoResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/renew-info", Some(params))
            .await
    }

    /// Get fixed repayment history.
    pub async fn get_fixed_repayment_history(
        &self,
        params: &GetFixedRepaymentHistoryParams,
    ) -> Result<FixedRepaymentHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/crypto-loan-fixed/repayment-history", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_adjust_ltv_params_serialization() {
        let params = AdjustLtvParams::add("BTC", "0.1");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize adjust LTV params: {}", err),
        };
        assert!(json.contains("\"currency\":\"BTC\""));
        assert!(json.contains("\"amount\":\"0.1\""));
        assert!(json.contains("\"direction\":\"0\""));
    }

    #[test]
    fn test_flexible_borrow_params_serialization() {
        let params = FlexibleBorrowParams::new("USDT", "1000")
            .collateral_list(vec![CollateralCoinItem::new("BTC", "0.05")]);

        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize flexible borrow params: {}", err),
        };
        assert!(json.contains("\"loanCurrency\":\"USDT\""));
        assert!(json.contains("\"loanAmount\":\"1000\""));
        assert!(json.contains("\"collateralList\":[{\"currency\":\"BTC\",\"amount\":\"0.05\"}]"));
    }

    #[test]
    fn test_fixed_borrow_params_serialization() {
        let params = FixedBorrowParams::new("USDT", "1000", "0.05", "30").auto_repay(true);
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize fixed borrow params: {}", err),
        };
        assert!(json.contains("\"orderCurrency\":\"USDT\""));
        assert!(json.contains("\"orderAmount\":\"1000\""));
        assert!(json.contains("\"annualRate\":\"0.05\""));
        assert!(json.contains("\"term\":\"30\""));
        assert!(json.contains("\"autoRepay\":\"true\""));
    }

    #[test]
    fn test_adjustment_history_params_serialization() {
        let params = GetAdjustmentHistoryParams::new()
            .collateral_currency("BTC")
            .limit(50);

        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize adjustment history params: {}", err),
        };
        assert!(query.contains("collateralCurrency=BTC"));
        assert!(query.contains("limit=50"));
    }
}
