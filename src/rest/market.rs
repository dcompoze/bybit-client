//! Market data API endpoints.

use serde::Serialize;

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::market::*;
use crate::types::{Category, KlineInterval, ServerTime};

/// Market data service for public endpoints.
#[derive(Debug, Clone)]
pub struct MarketService {
    http: HttpClient,
}

impl MarketService {
    /// Create a new market service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get server time.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let time = client.market().get_server_time().await?;
    /// println!("Server time: {} ms", time.as_millis());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_server_time(&self) -> Result<ServerTime, BybitError> {
        self.http.get("/v5/market/time", None::<&()>).await
    }

    /// Get kline/candlestick data.
    ///
    /// # Arguments
    ///
    /// * `category` - Product category (spot, linear, inverse)
    /// * `symbol` - Trading symbol (e.g., "BTCUSDT")
    /// * `interval` - Kline interval
    /// * `params` - Optional parameters (start, end, limit)
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category, KlineInterval};
    /// # use bybit_client::api::market::GetKlineParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetKlineParams::new(Category::Linear, "BTCUSDT", KlineInterval::Hour1)
    ///     .limit(100);
    /// let result = client.market().get_kline(&params).await?;
    /// for kline in result.klines() {
    ///     println!("{}: O={} H={} L={} C={}",
    ///         kline.start_time, kline.open_price, kline.high_price,
    ///         kline.low_price, kline.close_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_kline(&self, params: &GetKlineParams) -> Result<KlineResult, BybitError> {
        self.http.get("/v5/market/kline", Some(params)).await
    }

    /// Get mark price kline data.
    pub async fn get_mark_price_kline(
        &self,
        params: &GetKlineParams,
    ) -> Result<KlineResult, BybitError> {
        self.http
            .get("/v5/market/mark-price-kline", Some(params))
            .await
    }

    /// Get index price kline data.
    pub async fn get_index_price_kline(
        &self,
        params: &GetKlineParams,
    ) -> Result<KlineResult, BybitError> {
        self.http
            .get("/v5/market/index-price-kline", Some(params))
            .await
    }

    /// Get premium index price kline data.
    pub async fn get_premium_index_price_kline(
        &self,
        params: &GetKlineParams,
    ) -> Result<KlineResult, BybitError> {
        self.http
            .get("/v5/market/premium-index-price-kline", Some(params))
            .await
    }

    /// Get instrument/symbol information.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::api::market::GetInstrumentsInfoParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetInstrumentsInfoParams::new(Category::Linear)
    ///     .symbol("BTCUSDT");
    /// let result = client.market().get_instruments_info(&params).await?;
    /// for inst in &result.list {
    ///     println!("{}: status={}", inst.symbol, inst.status);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_instruments_info(
        &self,
        params: &GetInstrumentsInfoParams,
    ) -> Result<InstrumentInfoResult, BybitError> {
        self.http
            .get("/v5/market/instruments-info", Some(params))
            .await
    }

    /// Get order book depth.
    ///
    /// # Arguments
    ///
    /// * `params` - Order book parameters
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::api::market::GetOrderbookParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetOrderbookParams::new(Category::Linear, "BTCUSDT")
    ///     .limit(25);
    /// let orderbook = client.market().get_orderbook(&params).await?;
    /// println!("Best bid: {} @ {}", orderbook.bids[0].size, orderbook.bids[0].price);
    /// println!("Best ask: {} @ {}", orderbook.asks[0].size, orderbook.asks[0].price);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_orderbook(&self, params: &GetOrderbookParams) -> Result<Orderbook, BybitError> {
        self.http.get("/v5/market/orderbook", Some(params)).await
    }

    /// Get tickers for one or all symbols.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::api::market::GetTickersParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetTickersParams::new(Category::Linear)
    ///     .symbol("BTCUSDT");
    /// let result = client.market().get_tickers(&params).await?;
    /// if let Some(ticker) = result.list.first() {
    ///     println!("{}: last={:?}", ticker.symbol, ticker.last_price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_tickers(&self, params: &GetTickersParams) -> Result<TickerResult, BybitError> {
        self.http.get("/v5/market/tickers", Some(params)).await
    }

    /// Get funding rate history.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::api::market::GetFundingRateHistoryParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetFundingRateHistoryParams::new(Category::Linear, "BTCUSDT")
    ///     .limit(10);
    /// let result = client.market().get_funding_rate_history(&params).await?;
    /// for rate in &result.list {
    ///     println!("{}: {}", rate.funding_rate_timestamp, rate.funding_rate);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_funding_rate_history(
        &self,
        params: &GetFundingRateHistoryParams,
    ) -> Result<FundingRateHistoryResult, BybitError> {
        self.http
            .get("/v5/market/funding/history", Some(params))
            .await
    }

    /// Get public trading history (recent trades).
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, Category};
    /// # use bybit_client::api::market::GetPublicTradingHistoryParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::public_only()?;
    /// let params = GetPublicTradingHistoryParams::new(Category::Linear, "BTCUSDT")
    ///     .limit(50);
    /// let result = client.market().get_public_trading_history(&params).await?;
    /// for trade in &result.list {
    ///     println!("{}: {} {} @ {}", trade.time, trade.side, trade.size, trade.price);
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_public_trading_history(
        &self,
        params: &GetPublicTradingHistoryParams,
    ) -> Result<PublicTradeResult, BybitError> {
        self.http
            .get("/v5/market/recent-trade", Some(params))
            .await
    }

    /// Get open interest.
    pub async fn get_open_interest(
        &self,
        params: &GetOpenInterestParams,
    ) -> Result<OpenInterestResult, BybitError> {
        self.http
            .get("/v5/market/open-interest", Some(params))
            .await
    }

    /// Get historical volatility (options only).
    pub async fn get_historical_volatility(
        &self,
        params: &GetHistoricalVolatilityParams,
    ) -> Result<Vec<HistoricalVolatility>, BybitError> {
        self.http
            .get("/v5/market/historical-volatility", Some(params))
            .await
    }

    /// Get insurance fund data.
    pub async fn get_insurance(
        &self,
        coin: Option<&str>,
    ) -> Result<InsuranceResult, BybitError> {
        #[derive(Serialize)]
        struct Params<'a> {
            #[serde(skip_serializing_if = "Option::is_none")]
            coin: Option<&'a str>,
        }
        self.http
            .get("/v5/market/insurance", Some(&Params { coin }))
            .await
    }

    /// Get risk limit information.
    pub async fn get_risk_limit(
        &self,
        params: &GetRiskLimitParams,
    ) -> Result<RiskLimitResult, BybitError> {
        self.http.get("/v5/market/risk-limit", Some(params)).await
    }

    /// Get delivery price (futures/options).
    pub async fn get_delivery_price(
        &self,
        params: &GetDeliveryPriceParams,
    ) -> Result<DeliveryPriceResult, BybitError> {
        self.http
            .get("/v5/market/delivery-price", Some(params))
            .await
    }

    /// Get long/short ratio.
    pub async fn get_long_short_ratio(
        &self,
        params: &GetLongShortRatioParams,
    ) -> Result<LongShortRatioResult, BybitError> {
        self.http
            .get("/v5/market/account-ratio", Some(params))
            .await
    }
}


/// Parameters for getting kline data.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetKlineParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Kline interval.
    pub interval: String,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end: Option<u64>,
    /// Limit (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetKlineParams {
    /// Create new kline parameters.
    pub fn new(category: Category, symbol: impl Into<String>, interval: KlineInterval) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            interval: interval.as_str().to_string(),
            start: None,
            end: None,
            limit: None,
        }
    }

    /// Set start time.
    pub fn start(mut self, start: u64) -> Self {
        self.start = Some(start);
        self
    }

    /// Set end time.
    pub fn end(mut self, end: u64) -> Self {
        self.end = Some(end);
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for getting instruments info.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetInstrumentsInfoParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Symbol status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetInstrumentsInfoParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            status: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set status filter.
    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    /// Set base coin filter.
    pub fn base_coin(mut self, base_coin: impl Into<String>) -> Self {
        self.base_coin = Some(base_coin.into());
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

/// Parameters for getting order book.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOrderbookParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Depth limit (1, 25, 50, 100, 200, 500).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetOrderbookParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            limit: None,
        }
    }

    /// Set depth limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for getting tickers.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTickersParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol (optional - if not set, returns all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Expiry date filter (options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
}

impl GetTickersParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            exp_date: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set base coin filter.
    pub fn base_coin(mut self, base_coin: impl Into<String>) -> Self {
        self.base_coin = Some(base_coin.into());
        self
    }

    /// Set expiry date filter.
    pub fn exp_date(mut self, exp_date: impl Into<String>) -> Self {
        self.exp_date = Some(exp_date.into());
        self
    }
}

/// Parameters for getting funding rate history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundingRateHistoryParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetFundingRateHistoryParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            start_time: None,
            end_time: None,
            limit: None,
        }
    }

    /// Set start time.
    pub fn start_time(mut self, start_time: u64) -> Self {
        self.start_time = Some(start_time);
        self
    }

    /// Set end time.
    pub fn end_time(mut self, end_time: u64) -> Self {
        self.end_time = Some(end_time);
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for getting public trading history.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetPublicTradingHistoryParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Base coin filter (option only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Option type (option only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub option_type: Option<String>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetPublicTradingHistoryParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            base_coin: None,
            option_type: None,
            limit: None,
        }
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for getting open interest.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetOpenInterestParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Interval (5min, 15min, 30min, 1h, 4h, 1d).
    pub interval_time: String,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetOpenInterestParams {
    /// Create new parameters.
    pub fn new(
        category: Category,
        symbol: impl Into<String>,
        interval_time: impl Into<String>,
    ) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            interval_time: interval_time.into(),
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Parameters for getting historical volatility.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetHistoricalVolatilityParams {
    /// Product category (must be option).
    pub category: Category,
    /// Base coin (e.g., "BTC").
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Period (days).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<i32>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
}

impl GetHistoricalVolatilityParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            base_coin: None,
            period: None,
            start_time: None,
            end_time: None,
        }
    }
}

/// Parameters for getting risk limit.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetRiskLimitParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
}

impl GetRiskLimitParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
        }
    }

    /// Set symbol.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
}

/// Parameters for getting delivery price.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryPriceParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol (optional).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Base coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub base_coin: Option<String>,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDeliveryPriceParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            base_coin: None,
            limit: None,
            cursor: None,
        }
    }
}

/// Parameters for getting long/short ratio.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetLongShortRatioParams {
    /// Product category.
    pub category: Category,
    /// Trading symbol.
    pub symbol: String,
    /// Data recording period (5min, 15min, 30min, 1h, 4h, 1d).
    pub period: String,
    /// Limit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetLongShortRatioParams {
    /// Create new parameters.
    pub fn new(category: Category, symbol: impl Into<String>, period: impl Into<String>) -> Self {
        Self {
            category,
            symbol: symbol.into(),
            period: period.into(),
            limit: None,
        }
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_params_serialization() {
        let params =
            GetKlineParams::new(Category::Linear, "BTCUSDT", KlineInterval::Hour1).limit(100);

        let serialized = match serde_urlencoded::to_string(&params) {
            Ok(serialized) => serialized,
            Err(err) => panic!("Failed to serialize kline params: {}", err),
        };
        assert!(serialized.contains("category=linear"));
        assert!(serialized.contains("symbol=BTCUSDT"));
        assert!(serialized.contains("interval=60"));
        assert!(serialized.contains("limit=100"));
    }

    #[test]
    fn test_orderbook_params() {
        let params = GetOrderbookParams::new(Category::Spot, "BTCUSDT").limit(25);
        assert_eq!(params.symbol, "BTCUSDT");
        assert_eq!(params.limit, Some(25));
    }
}
