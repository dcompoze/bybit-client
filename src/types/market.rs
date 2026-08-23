//! Market data types for the Bybit V5 API.

use serde::{Deserialize, Serialize};

/// Kline/candlestick data.
///
/// Note: The API returns klines as arrays, so we use a custom deserializer.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kline {
    /// Start time of the candle (ms timestamp as string).
    pub start_time: String,
    /// Open price.
    pub open_price: String,
    /// High price.
    pub high_price: String,
    /// Low price.
    pub low_price: String,
    /// Close price.
    pub close_price: String,
    /// Volume.
    pub volume: String,
    /// Turnover (quote volume).
    pub turnover: String,
}

/// Raw kline data as returned by the API (array format).
#[derive(Debug, Clone)]
pub struct KlineRaw(pub Vec<String>);

impl<'de> Deserialize<'de> for KlineRaw {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        Ok(KlineRaw(arr))
    }
}

impl From<KlineRaw> for Kline {
    fn from(raw: KlineRaw) -> Self {
        let arr = raw.0;
        Kline {
            start_time: arr.first().cloned().unwrap_or_default(),
            open_price: arr.get(1).cloned().unwrap_or_default(),
            high_price: arr.get(2).cloned().unwrap_or_default(),
            low_price: arr.get(3).cloned().unwrap_or_default(),
            close_price: arr.get(4).cloned().unwrap_or_default(),
            volume: arr.get(5).cloned().unwrap_or_default(),
            turnover: arr.get(6).cloned().unwrap_or_default(),
        }
    }
}

/// Kline list result from the API.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct KlineResult {
    /// Trading category.
    pub category: String,
    /// Trading symbol.
    pub symbol: String,
    /// List of klines (as arrays).
    pub list: Vec<KlineRaw>,
}

impl KlineResult {
    /// Convert raw klines to structured Kline objects.
    pub fn klines(&self) -> Vec<Kline> {
        self.list.iter().cloned().map(Kline::from).collect()
    }
}

/// Order book entry (price, size).
#[derive(Debug, Clone, Serialize)]
pub struct OrderbookEntry {
    /// Price level.
    pub price: String,
    /// Size at this price.
    pub size: String,
}

impl<'de> Deserialize<'de> for OrderbookEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let arr: Vec<String> = Vec::deserialize(deserializer)?;
        Ok(OrderbookEntry {
            price: arr.first().cloned().unwrap_or_default(),
            size: arr.get(1).cloned().unwrap_or_default(),
        })
    }
}

/// Order book data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Orderbook {
    /// Trading symbol.
    #[serde(rename = "s")]
    pub symbol: String,
    /// Bid orders (buy side).
    #[serde(rename = "b")]
    pub bids: Vec<OrderbookEntry>,
    /// Ask orders (sell side).
    #[serde(rename = "a")]
    pub asks: Vec<OrderbookEntry>,
    /// Timestamp (milliseconds).
    #[serde(rename = "ts")]
    pub timestamp: u64,
    /// Update ID.
    #[serde(rename = "u")]
    pub update_id: u64,
    /// Sequence number (for linear/inverse).
    #[serde(rename = "seq", default)]
    pub seq: Option<u64>,
}

/// Ticker information (varies by category).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ticker {
    /// Trading symbol.
    pub symbol: String,
    /// Last traded price.
    #[serde(default)]
    pub last_price: Option<String>,
    /// Index price.
    #[serde(default)]
    pub index_price: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// 24h price change percentage.
    #[serde(default)]
    pub price_24h_pcnt: Option<String>,
    /// Price 24 hours ago.
    #[serde(default)]
    pub prev_price_24h: Option<String>,
    /// Highest price in 24h.
    #[serde(default)]
    pub high_price_24h: Option<String>,
    /// Lowest price in 24h.
    #[serde(default)]
    pub low_price_24h: Option<String>,
    /// 24h volume (base currency).
    #[serde(default)]
    pub volume_24h: Option<String>,
    /// 24h turnover (quote currency).
    #[serde(default)]
    pub turnover_24h: Option<String>,
    /// Best bid price.
    #[serde(default)]
    pub bid_1_price: Option<String>,
    /// Best bid size.
    #[serde(default)]
    pub bid_1_size: Option<String>,
    /// Best ask price.
    #[serde(default)]
    pub ask_1_price: Option<String>,
    /// Best ask size.
    #[serde(default)]
    pub ask_1_size: Option<String>,
    /// Open interest (derivatives).
    #[serde(default)]
    pub open_interest: Option<String>,
    /// Open interest value (USD).
    #[serde(default)]
    pub open_interest_value: Option<String>,
    /// Funding rate (perpetuals).
    #[serde(default)]
    pub funding_rate: Option<String>,
    /// Next funding time (ms).
    #[serde(default)]
    pub next_funding_time: Option<String>,
    /// Delivery time (futures).
    #[serde(default)]
    pub delivery_time: Option<String>,
    /// Basis rate (futures).
    #[serde(default)]
    pub basis_rate: Option<String>,
    /// Delivery fee rate.
    #[serde(default)]
    pub delivery_fee_rate: Option<String>,
    /// Predicted delivery price.
    #[serde(default)]
    pub predicted_delivery_price: Option<String>,
    /// Long-side open interest.
    #[serde(default)]
    pub open_interest_long: Option<String>,
    /// Short-side open interest.
    #[serde(default)]
    pub open_interest_short: Option<String>,
}

/// Ticker list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TickerResult {
    /// Trading category.
    pub category: String,
    /// List of tickers.
    pub list: Vec<Ticker>,
}

/// Instrument/symbol information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfo {
    /// Trading symbol.
    pub symbol: String,
    /// Contract type (for derivatives).
    #[serde(default)]
    pub contract_type: Option<String>,
    /// Symbol status.
    pub status: String,
    /// Base coin.
    #[serde(default)]
    pub base_coin: Option<String>,
    /// Quote coin.
    #[serde(default)]
    pub quote_coin: Option<String>,
    /// Settlement coin.
    #[serde(default)]
    pub settle_coin: Option<String>,
    /// Launch time (ms).
    #[serde(default)]
    pub launch_time: Option<String>,
    /// Delivery time (ms) for futures.
    #[serde(default)]
    pub delivery_time: Option<String>,
    /// Delivery fee rate.
    #[serde(default)]
    pub delivery_fee_rate: Option<String>,
    /// Price scale.
    #[serde(default)]
    pub price_scale: Option<String>,
    /// Leverage filter.
    #[serde(default)]
    pub leverage_filter: Option<LeverageFilter>,
    /// Price filter.
    #[serde(default)]
    pub price_filter: Option<PriceFilter>,
    /// Lot size filter.
    #[serde(default)]
    pub lot_size_filter: Option<LotSizeFilter>,
    /// Whether unified margin is supported.
    #[serde(default)]
    pub unified_margin_trade: Option<bool>,
    /// Funding interval (minutes).
    #[serde(default)]
    pub funding_interval: Option<i32>,
    /// Copy trading support.
    #[serde(default)]
    pub copy_trading: Option<String>,
    /// Margin trading support (spot).
    #[serde(default)]
    pub margin_trading: Option<String>,
    /// Symbol type (e.g. `commodity` for commodity perpetuals).
    #[serde(default)]
    pub symbol_type: Option<String>,
    /// Symbol ID.
    #[serde(default)]
    pub symbol_id: Option<String>,
    /// Multiplier for xstocks trading pairs.
    #[serde(default)]
    pub xstock_multiplier: Option<String>,
}

/// Leverage filter for derivatives.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LeverageFilter {
    /// Minimum leverage.
    pub min_leverage: String,
    /// Maximum leverage.
    pub max_leverage: String,
    /// Leverage step.
    pub leverage_step: String,
}

/// Price filter.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceFilter {
    /// Minimum price.
    #[serde(default)]
    pub min_price: Option<String>,
    /// Maximum price.
    #[serde(default)]
    pub max_price: Option<String>,
    /// Price tick size.
    pub tick_size: String,
}

/// Lot size filter.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LotSizeFilter {
    /// Minimum order quantity.
    #[serde(alias = "minOrderQty")]
    pub min_order_qty: String,
    /// Maximum order quantity.
    #[serde(alias = "maxOrderQty")]
    pub max_order_qty: String,
    /// Quantity step.
    pub qty_step: String,
    /// Base precision (spot).
    #[serde(default)]
    pub base_precision: Option<String>,
    /// Quote precision (spot).
    #[serde(default)]
    pub quote_precision: Option<String>,
    /// Minimum notional value (spot).
    #[serde(default)]
    pub min_notional_value: Option<String>,
    /// Post-only max order quantity.
    #[serde(default)]
    pub post_only_max_order_qty: Option<String>,
    /// Maximum market order quantity.
    #[serde(default)]
    pub max_mkt_order_qty: Option<String>,
    /// Minimum notional value (derivatives).
    #[serde(default)]
    pub min_order_amt: Option<String>,
    /// Maximum order amount.
    #[serde(default)]
    pub max_order_amt: Option<String>,
}

/// Instrument list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstrumentInfoResult {
    /// Trading category.
    pub category: String,
    /// List of instruments.
    pub list: Vec<InstrumentInfo>,
    /// Next page cursor.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Public trade data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTrade {
    /// Execution ID.
    #[serde(alias = "execId")]
    pub exec_id: String,
    /// Trading symbol.
    pub symbol: String,
    /// Trade price.
    pub price: String,
    /// Trade size.
    pub size: String,
    /// Trade side (Buy/Sell).
    pub side: String,
    /// Trade time (ms).
    pub time: String,
    /// Is block trade.
    #[serde(default)]
    pub is_block_trade: Option<bool>,
}

/// Public trade list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublicTradeResult {
    /// Trading category.
    pub category: String,
    /// List of trades.
    pub list: Vec<PublicTrade>,
}

/// Funding rate history entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistory {
    /// Trading symbol.
    pub symbol: String,
    /// Funding rate.
    pub funding_rate: String,
    /// Funding rate timestamp (ms).
    pub funding_rate_timestamp: String,
}

/// Funding rate history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundingRateHistoryResult {
    /// Trading category.
    pub category: String,
    /// List of funding rates.
    pub list: Vec<FundingRateHistory>,
}

/// Open interest data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterest {
    /// Open interest value.
    pub open_interest: String,
    /// Timestamp (ms).
    pub timestamp: String,
}

/// Open interest result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenInterestResult {
    /// Trading category.
    pub category: String,
    /// Trading symbol.
    pub symbol: String,
    /// List of open interest entries.
    pub list: Vec<OpenInterest>,
    /// Next page cursor.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Historical volatility data (options).
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HistoricalVolatility {
    /// Period (days).
    pub period: i32,
    /// Volatility value.
    pub value: String,
    /// Timestamp (ms).
    pub time: String,
}

/// Insurance fund data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Insurance {
    /// Coin name.
    pub coin: String,
    /// Insurance fund balance.
    pub balance: String,
    /// Value (USD).
    #[serde(default)]
    pub value: Option<String>,
}

/// Insurance fund result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InsuranceResult {
    /// Updated time (ms).
    #[serde(default)]
    pub updated_time: Option<String>,
    /// List of insurance entries.
    pub list: Vec<Insurance>,
}

/// Risk limit tier.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimit {
    /// Risk limit ID.
    pub id: i32,
    /// Trading symbol.
    pub symbol: String,
    /// Risk limit value.
    pub risk_limit_value: String,
    /// Maintenance margin rate.
    pub maintenance_margin: String,
    /// Initial margin rate.
    pub initial_margin: String,
    /// Is lowest risk.
    #[serde(default)]
    pub is_lowest_risk: Option<i32>,
    /// Maximum leverage.
    pub max_leverage: String,
}

/// Risk limit result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiskLimitResult {
    /// Trading category.
    pub category: String,
    /// List of risk limits.
    pub list: Vec<RiskLimit>,
}

/// Delivery price data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPrice {
    /// Trading symbol.
    pub symbol: String,
    /// Delivery price.
    pub delivery_price: String,
    /// Delivery time (ms).
    pub delivery_time: String,
}

/// Delivery price result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryPriceResult {
    /// Trading category.
    pub category: String,
    /// List of delivery prices.
    pub list: Vec<DeliveryPrice>,
    /// Next page cursor.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Long/short ratio data.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatio {
    /// Trading symbol.
    pub symbol: String,
    /// Buy ratio.
    pub buy_ratio: String,
    /// Sell ratio.
    pub sell_ratio: String,
    /// Timestamp (ms).
    pub timestamp: String,
}

/// Long/short ratio result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LongShortRatioResult {
    /// List of ratios.
    pub list: Vec<LongShortRatio>,
}

/// Order price limit for a symbol.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PriceLimitResult {
    /// Trading symbol.
    pub symbol: String,
    /// Highest allowed buy price.
    pub buy_lmt: String,
    /// Lowest allowed sell price.
    pub sell_lmt: String,
    /// Timestamp (ms).
    pub ts: String,
}

/// Component of an index price.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceComponent {
    /// Exchange name.
    pub exchange: String,
    /// Spot pair on the exchange.
    pub spot_pair: String,
    /// Equivalent price.
    pub equivalent_price: String,
    /// Multiplier used for the component price.
    pub multiplier: String,
    /// Actual price.
    pub price: String,
    /// Weight in the index calculation.
    pub weight: String,
}

/// Index price components result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IndexPriceComponentsResult {
    /// Index name.
    pub index_name: String,
    /// Last index price.
    pub last_price: String,
    /// Last update time (ms).
    pub update_time: String,
    /// Components contributing to the index price.
    pub components: Vec<IndexPriceComponent>,
}

/// Fee rate level within a fee group.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroupLevel {
    /// Level name.
    pub level: String,
    /// Taker fee rate.
    pub taker_fee_rate: String,
    /// Maker fee rate.
    pub maker_fee_rate: String,
    /// Maker rebate rate.
    #[serde(default)]
    pub maker_rebate: Option<String>,
}

/// Fee rates for a fee group.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroupRates {
    /// Pro-level fee structures.
    #[serde(default)]
    pub pro: Vec<FeeGroupLevel>,
    /// Market maker-level fee structures.
    #[serde(default)]
    pub market_maker: Vec<FeeGroupLevel>,
}

/// Fee group entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroupItem {
    /// Fee group name.
    pub group_name: String,
    /// Group weighting factor.
    #[serde(default)]
    pub weighting_factor: Option<f64>,
    /// Number of symbols in the group.
    #[serde(default)]
    pub symbols_numbers: Option<i64>,
    /// Symbol names.
    #[serde(default)]
    pub symbols: Vec<String>,
    /// Fee rate details.
    pub fee_rates: FeeGroupRates,
    /// Last update time (ms).
    #[serde(default)]
    pub update_time: Option<String>,
}

/// Fee group info result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeeGroupInfoResult {
    /// List of fee groups.
    pub list: Vec<FeeGroupItem>,
}

/// ADL alert entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdlAlertItem {
    /// Insurance pool coin.
    pub coin: String,
    /// Trading symbol.
    pub symbol: String,
    /// Insurance fund balance.
    pub balance: String,
    /// Maximum balance in the last 8 hours.
    #[serde(default)]
    pub max_balance: Option<String>,
    /// PnL ratio threshold for triggering ADL.
    #[serde(default)]
    pub insurance_pnl_ratio: Option<String>,
    /// PnL drawdown ratio in the last 8 hours.
    #[serde(default)]
    pub pnl_ratio: Option<String>,
    /// Trigger threshold for PnL drawdown ADL.
    #[serde(default)]
    pub adl_trigger_threshold: Option<String>,
    /// Stop ratio threshold for PnL drawdown ADL.
    #[serde(default)]
    pub adl_stop_ratio: Option<String>,
}

/// ADL alert result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AdlAlertResult {
    /// Last update time (ms).
    pub update_time: String,
    /// List of ADL alerts.
    pub list: Vec<AdlAlertItem>,
}

/// Announcement type descriptor.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementType {
    /// Type title.
    pub title: String,
    /// Type key.
    pub key: String,
}

/// Announcement entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Announcement {
    /// Title.
    pub title: String,
    /// Description.
    #[serde(default)]
    pub description: Option<String>,
    /// Announcement type.
    #[serde(rename = "type", default)]
    pub announcement_type: Option<AnnouncementType>,
    /// Tags.
    #[serde(default)]
    pub tags: Vec<String>,
    /// URL.
    #[serde(default)]
    pub url: Option<String>,
    /// Publication timestamp (ms).
    #[serde(default)]
    pub date_timestamp: Option<u64>,
    /// Start timestamp (ms).
    #[serde(default)]
    pub start_date_timestamp: Option<u64>,
    /// End timestamp (ms).
    #[serde(default)]
    pub end_date_timestamp: Option<u64>,
}

/// Announcement list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AnnouncementResult {
    /// Total number of announcements.
    pub total: i64,
    /// List of announcements.
    pub list: Vec<Announcement>,
}

/// System status entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusItem {
    /// Maintenance ID.
    pub id: String,
    /// Title.
    pub title: String,
    /// State.
    pub state: String,
    /// Begin time (ms).
    #[serde(default)]
    pub begin: Option<String>,
    /// End time (ms).
    #[serde(default)]
    pub end: Option<String>,
    /// Announcement link.
    #[serde(default)]
    pub href: Option<String>,
    /// Affected service types.
    #[serde(default)]
    pub service_types: Vec<i32>,
    /// Affected products.
    #[serde(default)]
    pub product: Vec<i32>,
    /// Affected UID suffixes.
    #[serde(default)]
    pub uid_suffix: Vec<i32>,
    /// Maintenance type.
    #[serde(default)]
    pub maintain_type: Option<String>,
    /// Environment.
    #[serde(default)]
    pub env: Option<String>,
}

/// System status result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemStatusResult {
    /// List of maintenance entries.
    pub list: Vec<SystemStatusItem>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kline_from_raw() {
        let raw = KlineRaw(vec![
            "1699000000000".to_string(),
            "35000.5".to_string(),
            "35100.0".to_string(),
            "34900.0".to_string(),
            "35050.0".to_string(),
            "100.5".to_string(),
            "3520000.0".to_string(),
        ]);
        let kline = Kline::from(raw);
        assert_eq!(kline.start_time, "1699000000000");
        assert_eq!(kline.open_price, "35000.5");
        assert_eq!(kline.high_price, "35100.0");
    }

    #[test]
    fn test_orderbook_entry_deserialize() {
        let json = r#"["35000.5", "10.5"]"#;
        let entry: OrderbookEntry = match serde_json::from_str(json) {
            Ok(entry) => entry,
            Err(err) => panic!("Failed to parse orderbook entry: {}", err),
        };
        assert_eq!(entry.price, "35000.5");
        assert_eq!(entry.size, "10.5");
    }

    #[test]
    fn test_ticker_deserialize() {
        let json = r#"{
            "symbol": "BTCUSDT",
            "lastPrice": "35000.5",
            "price24hPcnt": "0.05",
            "volume24h": "10000"
        }"#;
        let ticker: Ticker = match serde_json::from_str(json) {
            Ok(ticker) => ticker,
            Err(err) => panic!("Failed to parse ticker: {}", err),
        };
        assert_eq!(ticker.symbol, "BTCUSDT");
        assert_eq!(ticker.last_price, Some("35000.5".to_string()));
    }
}
