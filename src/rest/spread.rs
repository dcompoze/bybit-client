//! Spread trading API endpoints.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::spread::*;

/// Spread trading service.
///
/// Market data endpoints are public.
/// Order and execution endpoints require authentication.
#[derive(Debug, Clone)]
pub struct SpreadService {
    http: HttpClient,
}

impl SpreadService {
    /// Create a new service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get spread instruments info.
    pub async fn get_instruments_info(
        &self,
        params: &GetSpreadInstrumentsInfoParams,
    ) -> Result<SpreadInstrumentsInfoResult, BybitError> {
        self.http.get("/v5/spread/instrument", Some(params)).await
    }

    /// Get the spread orderbook.
    pub async fn get_orderbook(
        &self,
        params: &GetSpreadOrderbookParams,
    ) -> Result<crate::types::market::Orderbook, BybitError> {
        self.http.get("/v5/spread/orderbook", Some(params)).await
    }

    /// Get spread tickers.
    pub async fn get_tickers(
        &self,
        params: &GetSpreadTickersParams,
    ) -> Result<SpreadTickersResult, BybitError> {
        self.http.get("/v5/spread/tickers", Some(params)).await
    }

    /// Get spread recent public trades.
    pub async fn get_recent_trades(
        &self,
        params: &GetSpreadRecentTradesParams,
    ) -> Result<SpreadRecentTradesResult, BybitError> {
        self.http.get("/v5/spread/recent-trade", Some(params)).await
    }

    /// Place a spread order.
    pub async fn place_order(
        &self,
        params: &PlaceSpreadOrderParams,
    ) -> Result<SpreadOrderResult, BybitError> {
        self.http
            .post_signed("/v5/spread/order/create", Some(params))
            .await
    }

    /// Amend a spread order.
    pub async fn amend_order(
        &self,
        params: &AmendSpreadOrderParams,
    ) -> Result<SpreadOrderResult, BybitError> {
        self.http
            .post_signed("/v5/spread/order/amend", Some(params))
            .await
    }

    /// Cancel a spread order.
    pub async fn cancel_order(
        &self,
        params: &CancelSpreadOrderParams,
    ) -> Result<SpreadOrderResult, BybitError> {
        self.http
            .post_signed("/v5/spread/order/cancel", Some(params))
            .await
    }

    /// Cancel all spread orders.
    pub async fn cancel_all_orders(
        &self,
        params: &CancelAllSpreadOrdersParams,
    ) -> Result<CancelAllSpreadOrdersResult, BybitError> {
        self.http
            .post_signed("/v5/spread/order/cancel-all", Some(params))
            .await
    }

    /// Get open spread orders.
    pub async fn get_open_orders(
        &self,
        params: &GetSpreadOpenOrdersParams,
    ) -> Result<SpreadOrderListResult, BybitError> {
        self.http
            .get_signed("/v5/spread/order/realtime", Some(params))
            .await
    }

    /// Get spread order history.
    pub async fn get_order_history(
        &self,
        params: &GetSpreadOrderHistoryParams,
    ) -> Result<SpreadOrderListResult, BybitError> {
        self.http
            .get_signed("/v5/spread/order/history", Some(params))
            .await
    }

    /// Get spread execution list.
    pub async fn get_execution_list(
        &self,
        params: &GetSpreadExecutionListParams,
    ) -> Result<SpreadExecutionListResult, BybitError> {
        self.http
            .get_signed("/v5/spread/execution/list", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::enums::Side;

    #[test]
    fn test_place_spread_order_params_serialization() {
        let params = PlaceSpreadOrderParams::limit("SOLUSDT_SOL/USDT", Side::Buy, "0.1", "21")
            .order_link_id("custom-id")
            .time_in_force("PostOnly");

        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize spread order params: {}", err),
        };
        assert!(json.contains("\"symbol\":\"SOLUSDT_SOL/USDT\""));
        assert!(json.contains("\"side\":\"Buy\""));
        assert!(json.contains("\"orderType\":\"Limit\""));
        assert!(json.contains("\"qty\":\"0.1\""));
        assert!(json.contains("\"price\":\"21\""));
        assert!(json.contains("\"orderLinkId\":\"custom-id\""));
        assert!(json.contains("\"timeInForce\":\"PostOnly\""));
    }

    #[test]
    fn test_market_spread_order_omits_price() {
        let params = PlaceSpreadOrderParams::market("SOLUSDT_SOL/USDT", Side::Sell, "0.1");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize spread order params: {}", err),
        };
        assert!(!json.contains("\"price\""));
        assert!(json.contains("\"orderType\":\"Market\""));
    }

    #[test]
    fn test_get_spread_open_orders_params_serialization() {
        let params = GetSpreadOpenOrdersParams::new()
            .symbol("SOLUSDT_SOL/USDT")
            .limit(10);

        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize open orders params: {}", err),
        };
        assert!(query.contains("symbol=SOLUSDT_SOL%2FUSDT"));
        assert!(query.contains("limit=10"));
    }
}
