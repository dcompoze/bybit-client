//! Position API endpoints for position management.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::position::*;

/// Position service for position management endpoints.
#[derive(Debug, Clone)]
pub struct PositionService {
    http: HttpClient,
}

impl PositionService {
    /// Create a new position service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get position information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::GetPositionInfoParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetPositionInfoParams::new(Category::Linear)
    ///     .symbol("BTCUSDT");
    /// let result = client.position().get_position_info(&params).await?;
    /// for pos in &result.list {
    ///     println!("{}: {} @ {} (unrealised PnL: {})",
    ///         pos.symbol, pos.size, pos.avg_price, pos.unrealised_pnl);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_position_info(
        &self,
        params: &GetPositionInfoParams,
    ) -> Result<PositionListResult, BybitError> {
        self.http.get_signed("/v5/position/list", Some(params)).await
    }

    /// Set leverage for a symbol.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::SetLeverageParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Set 10x leverage (same for buy and sell)
    /// let params = SetLeverageParams::uniform(Category::Linear, "BTCUSDT", "10");
    /// client.position().set_leverage(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_leverage(&self, params: &SetLeverageParams) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/position/set-leverage", Some(params))
            .await?;
        Ok(())
    }

    /// Switch between cross margin and isolated margin.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::SwitchMarginModeParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Switch to isolated margin with 10x leverage
    /// let params = SwitchMarginModeParams::isolated_margin(
    ///     Category::Linear, "BTCUSDT", "10", "10"
    /// );
    /// client.position().switch_margin_mode(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn switch_margin_mode(
        &self,
        params: &SwitchMarginModeParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/position/switch-isolated", Some(params))
            .await?;
        Ok(())
    }

    /// Switch position mode between one-way and hedge mode.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::SwitchPositionModeParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Switch to hedge mode
    /// let params = SwitchPositionModeParams::hedge_by_symbol(Category::Linear, "BTCUSDT");
    /// client.position().switch_position_mode(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn switch_position_mode(
        &self,
        params: &SwitchPositionModeParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/position/switch-mode", Some(params))
            .await?;
        Ok(())
    }

    /// Set trading stop (take profit, stop loss, trailing stop).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category, PositionIdx};
    /// # use bybit_client::types::position::SetTradingStopParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = SetTradingStopParams::new(Category::Linear, "BTCUSDT", PositionIdx::OneWay)
    ///     .take_profit("55000")
    ///     .stop_loss("45000");
    /// client.position().set_trading_stop(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_trading_stop(&self, params: &SetTradingStopParams) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/position/trading-stop", Some(params))
            .await?;
        Ok(())
    }

    /// Set auto add margin.
    ///
    /// When enabled, the system will automatically add margin to prevent liquidation.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::SetAutoAddMarginParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = SetAutoAddMarginParams::enable(Category::Linear, "BTCUSDT");
    /// client.position().set_auto_add_margin(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn set_auto_add_margin(
        &self,
        params: &SetAutoAddMarginParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/position/set-auto-add-margin", Some(params))
            .await?;
        Ok(())
    }

    /// Add or reduce margin for a position.
    ///
    /// Use positive value to add margin, negative to reduce.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::AddReduceMarginParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// // Add 100 USDT margin
    /// let params = AddReduceMarginParams::new(Category::Linear, "BTCUSDT", "100");
    /// let result = client.position().add_or_reduce_margin(&params).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn add_or_reduce_margin(
        &self,
        params: &AddReduceMarginParams,
    ) -> Result<MarginOperationResult, BybitError> {
        self.http
            .post_signed("/v5/position/add-margin", Some(params))
            .await
    }

    /// Get closed profit and loss records.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::types::position::GetClosedPnlParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetClosedPnlParams::new(Category::Linear)
    ///     .symbol("BTCUSDT")
    ///     .limit(20);
    /// let result = client.position().get_closed_pnl(&params).await?;
    /// for pnl in &result.list {
    ///     println!("{}: {} (closed PnL: {})", pnl.symbol, pnl.side, pnl.closed_pnl);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_closed_pnl(
        &self,
        params: &GetClosedPnlParams,
    ) -> Result<ClosedPnlListResult, BybitError> {
        self.http
            .get_signed("/v5/position/closed-pnl", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{Category, PositionIdx};

    #[test]
    fn test_set_leverage_params_serialization() {
        let params = SetLeverageParams::uniform(Category::Linear, "BTCUSDT", "10");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize leverage params: {}", err),
        };
        assert!(json.contains("\"category\":\"linear\""));
        assert!(json.contains("\"symbol\":\"BTCUSDT\""));
        assert!(json.contains("\"buyLeverage\":\"10\""));
        assert!(json.contains("\"sellLeverage\":\"10\""));
    }

    #[test]
    fn test_trading_stop_params_serialization() {
        let params = SetTradingStopParams::new(Category::Linear, "BTCUSDT", PositionIdx::OneWay)
            .take_profit("55000")
            .stop_loss("45000");

        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize trading stop params: {}", err),
        };
        assert!(json.contains("\"takeProfit\":\"55000\""));
        assert!(json.contains("\"stopLoss\":\"45000\""));
    }

    #[test]
    fn test_get_closed_pnl_params_serialization() {
        let params = GetClosedPnlParams::new(Category::Linear)
            .symbol("BTCUSDT")
            .limit(20);

        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize closed PnL params: {}", err),
        };
        assert!(query.contains("category=linear"));
        assert!(query.contains("symbol=BTCUSDT"));
        assert!(query.contains("limit=20"));
    }
}
