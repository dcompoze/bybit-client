//! Trade API endpoints for order management.

use serde::Serialize;

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::trade::*;
use crate::types::Category;

/// Trade service for order management endpoints.
#[derive(Debug, Clone)]
pub struct TradeService {
    http: HttpClient,
}

impl TradeService {
    /// Create a new trade service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Submit a new order.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category, Side, OrderType};
    /// # use bybit_client::types::trade::OrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Place a market order.
    /// let params = OrderParams::market(Category::Linear, "BTCUSDT", Side::Buy, "0.001");
    /// let result = client.trade().submit_order(&params).await?;
    /// println!("Order ID: {}", result.order_id);
    ///
    /// // Place a limit order.
    /// let params = OrderParams::limit(Category::Spot, "BTCUSDT", Side::Buy, "0.001", "50000");
    /// let result = client.trade().submit_order(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn submit_order(&self, params: &OrderParams) -> Result<OrderResult, BybitError> {
        self.http.post_signed("/v5/order/create", Some(params)).await
    }

    /// Amend an existing order.
    ///
    /// You can modify the order quantity, price, or TP/SL settings.
    /// Either `order_id` or `order_link_id` must be provided.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::AmendOrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = AmendOrderParams::by_order_id(Category::Linear, "BTCUSDT", "order123")
    ///     .price("51000")
    ///     .qty("0.002");
    /// let result = client.trade().amend_order(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn amend_order(&self, params: &AmendOrderParams) -> Result<OrderResult, BybitError> {
        self.http.post_signed("/v5/order/amend", Some(params)).await
    }

    /// Cancel an existing order.
    ///
    /// Either `order_id` or `order_link_id` must be provided.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::CancelOrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = CancelOrderParams::by_order_id(Category::Linear, "BTCUSDT", "order123");
    /// let result = client.trade().cancel_order(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_order(&self, params: &CancelOrderParams) -> Result<OrderResult, BybitError> {
        self.http
            .post_signed("/v5/order/cancel", Some(params))
            .await
    }

    /// Cancel all active orders.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::CancelAllOrdersParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Cancel all linear orders.
    /// let params = CancelAllOrdersParams::new(Category::Linear);
    /// let result = client.trade().cancel_all_orders(&params).await?;
    ///
    /// // Cancel all orders for a specific symbol.
    /// let params = CancelAllOrdersParams::new(Category::Spot)
    ///     .symbol("BTCUSDT");
    /// let result = client.trade().cancel_all_orders(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn cancel_all_orders(
        &self,
        params: &CancelAllOrdersParams,
    ) -> Result<CancelAllResult, BybitError> {
        self.http
            .post_signed("/v5/order/cancel-all", Some(params))
            .await
    }

    /// Get open/active orders.
    ///
    /// Returns real-time order data. For conditional orders, use `order_filter`.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::GetOpenOrdersParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetOpenOrdersParams::new(Category::Linear)
    ///     .symbol("BTCUSDT")
    ///     .limit(20);
    /// let result = client.trade().get_open_orders(&params).await?;
    /// for order in &result.list {
    ///     println!("{}: {} {} @ {}", order.order_id, order.side, order.qty, order.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_open_orders(
        &self,
        params: &GetOpenOrdersParams,
    ) -> Result<OrderListResult, BybitError> {
        self.http
            .get_signed("/v5/order/realtime", Some(params))
            .await
    }

    /// Get order history.
    ///
    /// Returns historical orders including filled, cancelled, and rejected orders.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::GetOrderHistoryParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetOrderHistoryParams::new(Category::Linear)
    ///     .symbol("BTCUSDT")
    ///     .limit(50);
    /// let result = client.trade().get_order_history(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_order_history(
        &self,
        params: &GetOrderHistoryParams,
    ) -> Result<OrderListResult, BybitError> {
        self.http
            .get_signed("/v5/order/history", Some(params))
            .await
    }

    /// Get trade execution list.
    ///
    /// Returns the trade history (fills) for your orders.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::GetExecutionListParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetExecutionListParams::new(Category::Linear)
    ///     .symbol("BTCUSDT")
    ///     .limit(50);
    /// let result = client.trade().get_execution_list(&params).await?;
    /// for exec in &result.list {
    ///     println!("{}: {} {} @ {}", exec.exec_id, exec.side, exec.exec_qty, exec.exec_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_execution_list(
        &self,
        params: &GetExecutionListParams,
    ) -> Result<ExecutionListResult, BybitError> {
        self.http
            .get_signed("/v5/execution/list", Some(params))
            .await
    }

    /// Get spot borrow quota.
    ///
    /// Check the maximum borrow amount for spot margin trading.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Side};
    /// # use bybit_client::types::trade::GetBorrowQuotaParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetBorrowQuotaParams::new("BTCUSDT", Side::Buy);
    /// let result = client.trade().get_borrow_quota(&params).await?;
    /// println!("Max trade qty: {}", result.max_trade_qty);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_borrow_quota(
        &self,
        params: &GetBorrowQuotaParams,
    ) -> Result<BorrowQuotaResult, BybitError> {
        self.http
            .get_signed("/v5/order/spot-borrow-check", Some(params))
            .await
    }

    /// Submit multiple orders in a single request.
    ///
    /// Supports up to 10 orders for options, 20 for USDT perpetual/USDC contracts.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category, Side};
    /// # use bybit_client::types::trade::BatchOrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let orders = vec![
    ///     BatchOrderParams::limit("BTCUSDT", Side::Buy, "0.001", "50000")
    ///         .order_link_id("batch_1"),
    ///     BatchOrderParams::limit("BTCUSDT", Side::Buy, "0.001", "49000")
    ///         .order_link_id("batch_2"),
    /// ];
    /// let result = client.trade().batch_submit_orders(Category::Linear, orders).await?;
    /// for order in &result.list {
    ///     println!("Created: {} - {}", order.order_id, order.order_link_id);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn batch_submit_orders(
        &self,
        category: Category,
        orders: Vec<BatchOrderParams>,
    ) -> Result<BatchOperationResult, BybitError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchRequest {
            category: Category,
            request: Vec<BatchOrderParams>,
        }

        let request = BatchRequest {
            category,
            request: orders,
        };

        self.http
            .post_signed("/v5/order/create-batch", Some(&request))
            .await
    }

    /// Amend multiple orders in a single request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::BatchAmendOrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let orders = vec![
    ///     BatchAmendOrderParams::by_order_link_id("BTCUSDT", "batch_1")
    ///         .price("51000"),
    ///     BatchAmendOrderParams::by_order_link_id("BTCUSDT", "batch_2")
    ///         .price("49500"),
    /// ];
    /// let result = client.trade().batch_amend_orders(Category::Linear, orders).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn batch_amend_orders(
        &self,
        category: Category,
        orders: Vec<BatchAmendOrderParams>,
    ) -> Result<BatchOperationResult, BybitError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchRequest {
            category: Category,
            request: Vec<BatchAmendOrderParams>,
        }

        let request = BatchRequest {
            category,
            request: orders,
        };

        self.http
            .post_signed("/v5/order/amend-batch", Some(&request))
            .await
    }

    /// Cancel multiple orders in a single request.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::trade::BatchCancelOrderParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let orders = vec![
    ///     BatchCancelOrderParams::by_order_link_id("BTCUSDT", "batch_1"),
    ///     BatchCancelOrderParams::by_order_link_id("BTCUSDT", "batch_2"),
    /// ];
    /// let result = client.trade().batch_cancel_orders(Category::Linear, orders).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn batch_cancel_orders(
        &self,
        category: Category,
        orders: Vec<BatchCancelOrderParams>,
    ) -> Result<BatchOperationResult, BybitError> {
        #[derive(Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchRequest {
            category: Category,
            request: Vec<BatchCancelOrderParams>,
        }

        let request = BatchRequest {
            category,
            request: orders,
        };

        self.http
            .post_signed("/v5/order/cancel-batch", Some(&request))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Side;

    #[test]
    fn test_order_params_serialization() {
        let params = OrderParams::limit(Category::Linear, "BTCUSDT", Side::Buy, "0.001", "50000")
            .time_in_force(crate::types::TimeInForce::GTC)
            .order_link_id("test_order");

        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize order params: {}", err),
        };
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"side\":\"Buy\""));
        assert!(json.contains("\"orderType\":\"Limit\""));
        assert!(json.contains("\"qty\":\"0.001\""));
        assert!(json.contains("\"price\":\"50000\""));
        assert!(json.contains("\"timeInForce\":\"GTC\""));
        assert!(json.contains("\"orderLinkId\":\"test_order\""));
    }

    #[test]
    fn test_amend_order_params_serialization() {
        let params = AmendOrderParams::by_order_id(Category::Spot, "ETHUSDT", "order456")
            .price("3000")
            .qty("1.5");

        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize amend params: {}", err),
        };
        assert!(json.contains("\"category\":\"spot\""));
        assert!(json.contains("\"symbol\":\"ETHUSDT\""));
        assert!(json.contains("\"orderId\":\"order456\""));
        assert!(json.contains("\"price\":\"3000\""));
        assert!(json.contains("\"qty\":\"1.5\""));
        assert!(!json.contains("orderLinkId"));
    }

    #[test]
    fn test_cancel_order_params_serialization() {
        let params =
            CancelOrderParams::by_order_link_id(Category::Linear, "BTCUSDT", "my_order_789");

        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize cancel params: {}", err),
        };
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"orderLinkId\":\"my_order_789\""));
        assert!(!json.contains("orderId"));
    }

    #[test]
    fn test_get_open_orders_params_serialization() {
        let params = GetOpenOrdersParams::new(Category::Spot)
            .symbol("BTCUSDT")
            .limit(25);

        let json = match serde_urlencoded::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize open orders params: {}", err),
        };
        assert!(json.contains("category=spot"));
        assert!(json.contains("symbol=BTCUSDT"));
        assert!(json.contains("limit=25"));
    }

    #[test]
    fn test_batch_request_serialization() {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct BatchRequest {
            category: Category,
            request: Vec<BatchOrderParams>,
        }

        let orders = vec![
            BatchOrderParams::limit("BTCUSDT", Side::Buy, "0.001", "50000"),
            BatchOrderParams::market("ETHUSDT", Side::Sell, "0.1"),
        ];

        let request = BatchRequest {
            category: Category::Linear,
            request: orders,
        };

        let json = match serde_json::to_string(&request) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize batch request: {}", err),
        };
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"request\":["));
        assert!(json.contains("BTCUSDT"));
        assert!(json.contains("ETHUSDT"));
    }
}
