//! Asset API endpoints for transfers, deposits, withdrawals, and convert.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::asset::*;

/// Asset service for asset management endpoints.
#[derive(Debug, Clone)]
pub struct AssetService {
    http: HttpClient,
}

impl AssetService {
    /// Create a new asset service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Get coin information, including chain status and withdraw limits.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::asset::GetCoinInfoParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = GetCoinInfoParams::new().coin("USDT");
    /// let result = client.asset().get_coin_info(&params).await?;
    /// for coin in &result.rows {
    ///     println!("{}: {} chains", coin.coin, coin.chains.len());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_coin_info(
        &self,
        params: &GetCoinInfoParams,
    ) -> Result<CoinInfoResult, BybitError> {
        self.http
            .get_signed("/v5/asset/coin/query-info", Some(params))
            .await
    }

    /// Get asset information (SPOT account only).
    pub async fn get_asset_info(
        &self,
        params: &GetAssetInfoParams,
    ) -> Result<AssetInfoResult, BybitError> {
        self.http
            .get_signed("/v5/asset/transfer/query-asset-info", Some(params))
            .await
    }

    /// Get all coin balances for an account type.
    pub async fn get_coins_balance(
        &self,
        params: &GetCoinsBalanceParams,
    ) -> Result<CoinsBalanceResult, BybitError> {
        self.http
            .get_signed("/v5/asset/transfer/query-account-coins-balance", Some(params))
            .await
    }

    /// Get the balance of a single coin for an account type.
    pub async fn get_coin_balance(
        &self,
        params: &GetCoinBalanceParams,
    ) -> Result<CoinBalanceResult, BybitError> {
        self.http
            .get_signed("/v5/asset/transfer/query-account-coin-balance", Some(params))
            .await
    }

    /// Get the transferable coin list between two account types.
    pub async fn get_transferable_coins(
        &self,
        params: &GetTransferableCoinParams,
    ) -> Result<TransferableCoinResult, BybitError> {
        self.http
            .get_signed("/v5/asset/transfer/query-transfer-coin-list", Some(params))
            .await
    }

    /// Create an internal transfer between account types under the same UID.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::{BybitClient, AccountType};
    /// # use bybit_client::types::asset::InterTransferParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = InterTransferParams::new(
    ///     "generated-uuid", "USDT", "100",
    ///     AccountType::Fund, AccountType::Unified,
    /// );
    /// let result = client.asset().create_inter_transfer(&params).await?;
    /// println!("Transfer ID: {}", result.transfer_id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_inter_transfer(
        &self,
        params: &InterTransferParams,
    ) -> Result<TransferResult, BybitError> {
        self.http
            .post_signed("/v5/asset/transfer/inter-transfer", Some(params))
            .await
    }

    /// Get internal transfer records.
    pub async fn get_inter_transfer_list(
        &self,
        params: &GetTransferListParams,
    ) -> Result<TransferRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/transfer/query-inter-transfer-list", Some(params))
            .await
    }

    /// Create a universal transfer between different UIDs.
    pub async fn create_universal_transfer(
        &self,
        params: &UniversalTransferParams,
    ) -> Result<TransferResult, BybitError> {
        self.http
            .post_signed("/v5/asset/transfer/universal-transfer", Some(params))
            .await
    }

    /// Get universal transfer records.
    pub async fn get_universal_transfer_list(
        &self,
        params: &GetTransferListParams,
    ) -> Result<TransferRecordListResult, BybitError> {
        self.http
            .get_signed(
                "/v5/asset/transfer/query-universal-transfer-list",
                Some(params),
            )
            .await
    }

    /// Get the sub UIDs under the master UID.
    pub async fn get_sub_member_list(&self) -> Result<SubMemberListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/transfer/query-sub-member-list", None::<&()>)
            .await
    }

    /// Get on-chain deposit records.
    pub async fn get_deposit_records(
        &self,
        params: &GetDepositRecordsParams,
    ) -> Result<DepositRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/deposit/query-record", Some(params))
            .await
    }

    /// Get sub account deposit records using the master API key.
    ///
    /// Requires `sub_member_id` to be set on the parameters.
    pub async fn get_sub_deposit_records(
        &self,
        params: &GetDepositRecordsParams,
    ) -> Result<DepositRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/deposit/query-sub-member-record", Some(params))
            .await
    }

    /// Get internal (off-chain) deposit records.
    pub async fn get_internal_deposit_records(
        &self,
        params: &GetDepositRecordsParams,
    ) -> Result<InternalDepositRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/deposit/query-internal-record", Some(params))
            .await
    }

    /// Get the master account deposit address for a coin.
    pub async fn get_deposit_address(
        &self,
        params: &GetDepositAddressParams,
    ) -> Result<DepositAddressResult, BybitError> {
        self.http
            .get_signed("/v5/asset/deposit/query-address", Some(params))
            .await
    }

    /// Get a sub account deposit address for a coin.
    ///
    /// Requires `sub_member_id` and `chain_type` to be set on the parameters.
    pub async fn get_sub_deposit_address(
        &self,
        params: &GetDepositAddressParams,
    ) -> Result<DepositAddressResult, BybitError> {
        self.http
            .get_signed("/v5/asset/deposit/query-sub-member-address", Some(params))
            .await
    }

    /// Get allowed deposit coin information.
    pub async fn get_allowed_deposit_coin_info(
        &self,
        params: &GetAllowedDepositCoinInfoParams,
    ) -> Result<AllowedDepositCoinResult, BybitError> {
        self.http
            .get_signed("/v5/asset/deposit/query-allowed-list", Some(params))
            .await
    }

    /// Set the auto transfer account after deposit.
    pub async fn set_deposit_account(
        &self,
        params: &SetDepositAccountParams,
    ) -> Result<OperationStatusResult, BybitError> {
        self.http
            .post_signed("/v5/asset/deposit/deposit-to-account", Some(params))
            .await
    }

    /// Create a withdrawal.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::asset::WithdrawParams;
    /// # use bybit_client::auth::current_timestamp_ms;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = WithdrawParams::new("USDT", "0xabc...", "100", current_timestamp_ms())
    ///     .chain("ETH");
    /// let result = client.asset().withdraw(&params).await?;
    /// println!("Withdrawal ID: {}", result.id);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn withdraw(&self, params: &WithdrawParams) -> Result<WithdrawResult, BybitError> {
        self.http
            .post_signed("/v5/asset/withdraw/create", Some(params))
            .await
    }

    /// Cancel a withdrawal.
    pub async fn cancel_withdrawal(
        &self,
        params: &CancelWithdrawalParams,
    ) -> Result<OperationStatusResult, BybitError> {
        self.http
            .post_signed("/v5/asset/withdraw/cancel", Some(params))
            .await
    }

    /// Get withdrawal records.
    pub async fn get_withdrawal_records(
        &self,
        params: &GetWithdrawalRecordsParams,
    ) -> Result<WithdrawalRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/withdraw/query-record", Some(params))
            .await
    }

    /// Get the withdrawal address list.
    pub async fn get_withdrawal_addresses(
        &self,
        params: &GetWithdrawalAddressParams,
    ) -> Result<WithdrawalAddressListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/withdraw/query-address", Some(params))
            .await
    }

    /// Get the withdrawable amount for a coin.
    pub async fn get_withdrawable_amount(
        &self,
        params: &GetWithdrawableAmountParams,
    ) -> Result<WithdrawableAmountResult, BybitError> {
        self.http
            .get_signed("/v5/asset/withdraw/withdrawable-amount", Some(params))
            .await
    }

    /// Get the exchange entity (VASP) list for Travel Rule withdrawals.
    pub async fn get_vasp_list(&self) -> Result<VaspListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/withdraw/vasp/list", None::<&()>)
            .await
    }

    /// Get delivery records (options and futures).
    pub async fn get_delivery_records(
        &self,
        params: &GetDeliveryRecordParams,
    ) -> Result<DeliveryRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/delivery-record", Some(params))
            .await
    }

    /// Get USDC session settlement records.
    pub async fn get_settlement_records(
        &self,
        params: &GetSettlementRecordParams,
    ) -> Result<SettlementRecordListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/settlement-record", Some(params))
            .await
    }

    /// Get the list of coins available for convert.
    pub async fn get_convert_coin_list(
        &self,
        params: &GetConvertCoinListParams,
    ) -> Result<ConvertCoinListResult, BybitError> {
        self.http
            .get_signed("/v5/asset/exchange/query-coin-list", Some(params))
            .await
    }

    /// Request a convert quote.
    pub async fn request_convert_quote(
        &self,
        params: &RequestConvertQuoteParams,
    ) -> Result<ConvertQuoteResult, BybitError> {
        self.http
            .post_signed("/v5/asset/exchange/quote-apply", Some(params))
            .await
    }

    /// Confirm a convert quote and execute the conversion.
    pub async fn confirm_convert_quote(
        &self,
        params: &ConfirmConvertQuoteParams,
    ) -> Result<ConvertExecuteResult, BybitError> {
        self.http
            .post_signed("/v5/asset/exchange/convert-execute", Some(params))
            .await
    }

    /// Get the status of a conversion.
    pub async fn get_convert_status(
        &self,
        params: &GetConvertStatusParams,
    ) -> Result<ConvertStatusResult, BybitError> {
        self.http
            .get_signed("/v5/asset/exchange/convert-result-query", Some(params))
            .await
    }

    /// Get convert history.
    pub async fn get_convert_history(
        &self,
        params: &GetConvertHistoryParams,
    ) -> Result<ConvertHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/asset/exchange/query-convert-history", Some(params))
            .await
    }

    /// Get the asset overview across accounts and product categories.
    pub async fn get_asset_overview(
        &self,
        params: &GetAssetOverviewParams,
    ) -> Result<AssetOverviewResult, BybitError> {
        self.http
            .get_signed("/v5/asset/asset-overview", Some(params))
            .await
    }

    /// Get the funding account transaction history.
    pub async fn get_fund_history(
        &self,
        params: &GetFundHistoryParams,
    ) -> Result<FundHistoryResult, BybitError> {
        self.http
            .get_signed("/v5/asset/fundinghistory", Some(params))
            .await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::AccountType;

    #[test]
    fn test_inter_transfer_params_serialization() {
        let params = InterTransferParams::new(
            "uuid-1234",
            "USDT",
            "100",
            AccountType::Fund,
            AccountType::Unified,
        );
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize transfer params: {}", err),
        };
        assert!(json.contains("\"transferId\":\"uuid-1234\""));
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"amount\":\"100\""));
        assert!(json.contains("\"fromAccountType\":\"FUND\""));
        assert!(json.contains("\"toAccountType\":\"UNIFIED\""));
    }

    #[test]
    fn test_withdraw_params_serialization() {
        let params = WithdrawParams::new("USDT", "0xabc", "100", 1658384314791).chain("ETH");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize withdraw params: {}", err),
        };
        assert!(json.contains("\"coin\":\"USDT\""));
        assert!(json.contains("\"chain\":\"ETH\""));
        assert!(json.contains("\"address\":\"0xabc\""));
        assert!(json.contains("\"timestamp\":1658384314791"));
        assert!(!json.contains("\"tag\""));
    }

    #[test]
    fn test_deposit_records_params_serialization() {
        let params = GetDepositRecordsParams::new().coin("BTC").limit(20);
        let query = match serde_urlencoded::to_string(&params) {
            Ok(query) => query,
            Err(err) => panic!("Failed to serialize deposit params: {}", err),
        };
        assert!(query.contains("coin=BTC"));
        assert!(query.contains("limit=20"));
        assert!(!query.contains("cursor"));
    }

    #[test]
    fn test_request_convert_quote_params_serialization() {
        let params = RequestConvertQuoteParams::new("BTC", "USDT", "0.1", "eb_convert_uta");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize quote params: {}", err),
        };
        assert!(json.contains("\"fromCoin\":\"BTC\""));
        assert!(json.contains("\"requestCoin\":\"BTC\""));
        assert!(json.contains("\"toCoin\":\"USDT\""));
        assert!(json.contains("\"requestAmount\":\"0.1\""));
        assert!(json.contains("\"accountType\":\"eb_convert_uta\""));
    }
}
