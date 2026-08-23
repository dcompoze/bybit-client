//! Pre-upgrade API endpoints for pre-UTA historical data.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::pre_upgrade::*;
use crate::types::trade::{ExecutionListResult, OrderListResult};
use crate::types::position::ClosedPnlListResult;
use crate::types::account::TransactionLogResult;

/// Pre-upgrade service for historical data generated before the UTA upgrade.
#[derive(Debug, Clone)]
pub struct PreUpgradeService {
    http: HttpClient,
}

impl PreUpgradeService {
    /// Create a new pre-upgrade service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get pre-upgrade order history.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::pre_upgrade::PreUpgradeOrderHistoryParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = PreUpgradeOrderHistoryParams::new(Category::Linear).symbol("BTCUSDT");
    /// let result = client.pre_upgrade().get_order_history(&params).await?;
    /// for order in &result.list {
    ///     println!("{}: {}", order.order_id, order.order_status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_order_history(
        &self,
        params: &PreUpgradeOrderHistoryParams,
    ) -> Result<OrderListResult, BybitError> {
        self.http
            .get_signed("/v5/pre-upgrade/order/history", Some(params))
            .await
    }

    /// Get pre-upgrade execution list.
    pub async fn get_execution_list(
        &self,
        params: &PreUpgradeExecutionParams,
    ) -> Result<ExecutionListResult, BybitError> {
        self.http
            .get_signed("/v5/pre-upgrade/execution/list", Some(params))
            .await
    }

    /// Get pre-upgrade closed PnL records.
    pub async fn get_closed_pnl(
        &self,
        params: &PreUpgradeClosedPnlParams,
    ) -> Result<ClosedPnlListResult, BybitError> {
        self.http
            .get_signed("/v5/pre-upgrade/position/closed-pnl", Some(params))
            .await
    }

    /// Get pre-upgrade transaction log.
    pub async fn get_transaction_log(
        &self,
        params: &PreUpgradeTransactionLogParams,
    ) -> Result<TransactionLogResult, BybitError> {
        self.http
            .get_signed("/v5/pre-upgrade/account/transaction-log", Some(params))
            .await
    }

    /// Get pre-upgrade option delivery records.
    pub async fn get_delivery_record(
        &self,
        params: &PreUpgradeDeliveryRecordParams,
    ) -> Result<PreUpgradeDeliveryRecordResult, BybitError> {
        self.http
            .get_signed("/v5/pre-upgrade/asset/delivery-record", Some(params))
            .await
    }

    /// Get pre-upgrade USDC settlement records.
    pub async fn get_settlement_record(
        &self,
        params: &PreUpgradeSettlementParams,
    ) -> Result<PreUpgradeSettlementResult, BybitError> {
        self.http
            .get_signed("/v5/pre-upgrade/asset/settlement-record", Some(params))
            .await
    }
}
