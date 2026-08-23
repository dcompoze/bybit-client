//! Types for the asset endpoints.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::enums::*;

/// Parameters for getting coin information.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinInfoParams {
    /// Coin name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetCoinInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Coin chain information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinChain {
    /// Chain name.
    #[serde(default)]
    pub chain: Option<String>,
    /// Chain type.
    #[serde(default)]
    pub chain_type: Option<String>,
    /// Deposit confirmation number.
    #[serde(default)]
    pub confirmation: Option<String>,
    /// Withdraw fee.
    #[serde(default)]
    pub withdraw_fee: Option<String>,
    /// Minimum deposit amount.
    #[serde(default)]
    pub deposit_min: Option<String>,
    /// Minimum withdraw amount.
    #[serde(default)]
    pub withdraw_min: Option<String>,
    /// Minimum accuracy of withdraw.
    #[serde(default)]
    pub min_accuracy: Option<String>,
    /// Deposit status (0: suspended, 1: normal).
    #[serde(default)]
    pub chain_deposit: Option<String>,
    /// Withdraw status (0: suspended, 1: normal).
    #[serde(default)]
    pub chain_withdraw: Option<String>,
    /// Withdraw percentage fee.
    #[serde(default)]
    pub withdraw_percentage_fee: Option<String>,
    /// Contract address.
    #[serde(default)]
    pub contract_address: Option<String>,
}

/// Coin information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfo {
    /// Coin name.
    pub coin: String,
    /// Coin full name.
    #[serde(default)]
    pub name: Option<String>,
    /// Remaining withdraw amount (deprecated, use `withdraw_max`).
    #[serde(default)]
    pub remain_amount: Option<String>,
    /// Maximum withdraw amount.
    #[serde(default)]
    pub withdraw_max: Option<String>,
    /// Chain information.
    #[serde(default)]
    pub chains: Vec<CoinChain>,
}

/// Coin information result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinInfoResult {
    /// List of coins.
    pub rows: Vec<CoinInfo>,
}

/// Parameters for getting asset information.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetInfoParams {
    /// Account type (currently only SPOT).
    pub account_type: AccountType,
    /// Coin name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
}

impl GetAssetInfoParams {
    /// Create new parameters.
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            coin: None,
        }
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Single asset item.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetItem {
    /// Coin name.
    pub coin: String,
    /// Frozen amount.
    #[serde(default)]
    pub frozen: Option<String>,
    /// Free amount.
    #[serde(default)]
    pub free: Option<String>,
    /// Withdraw amount in processing.
    #[serde(default)]
    pub withdraw: Option<String>,
}

/// Spot asset information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpotAssetInfo {
    /// Account status.
    #[serde(default)]
    pub status: Option<String>,
    /// Assets.
    #[serde(default)]
    pub assets: Vec<AssetItem>,
}

/// Asset information result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetInfoResult {
    /// Spot asset information.
    pub spot: SpotAssetInfo,
}

/// Parameters for getting all coins balance.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinsBalanceParams {
    /// Account type.
    pub account_type: AccountType,
    /// User ID (required for sub account queries with master API key).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Coin names, comma separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Whether to include bonus (0 or 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bonus: Option<u8>,
}

impl GetCoinsBalanceParams {
    /// Create new parameters.
    pub fn new(account_type: AccountType) -> Self {
        Self {
            account_type,
            member_id: None,
            coin: None,
            with_bonus: None,
        }
    }

    /// Set member ID.
    pub fn member_id(mut self, id: impl Into<String>) -> Self {
        self.member_id = Some(id.into());
        self
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Include bonus in the response.
    pub fn with_bonus(mut self) -> Self {
        self.with_bonus = Some(1);
        self
    }
}

/// Coin balance item.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalanceItem {
    /// Coin name.
    pub coin: String,
    /// Wallet balance.
    #[serde(default)]
    pub wallet_balance: Option<String>,
    /// Transferable balance.
    #[serde(default)]
    pub transfer_balance: Option<String>,
    /// Bonus.
    #[serde(default)]
    pub bonus: Option<String>,
    /// Transfer safe amount.
    #[serde(default)]
    pub transfer_safe_amount: Option<String>,
    /// Transfer safe amount for LTV.
    #[serde(default)]
    pub ltv_transfer_safe_amount: Option<String>,
}

/// All coins balance result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinsBalanceResult {
    /// User ID.
    #[serde(default)]
    pub member_id: Option<String>,
    /// Account type.
    #[serde(default)]
    pub account_type: Option<String>,
    /// Balances.
    #[serde(default)]
    pub balance: Vec<CoinBalanceItem>,
}

/// Parameters for getting a single coin balance.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetCoinBalanceParams {
    /// Account type.
    pub account_type: AccountType,
    /// Coin name.
    pub coin: String,
    /// User ID (required for sub account queries with master API key).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Destination UID for transfer safe amount queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_member_id: Option<String>,
    /// Destination account type for transfer safe amount queries.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_account_type: Option<AccountType>,
    /// Whether to include bonus (0 or 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_bonus: Option<u8>,
    /// Whether to include transfer safe amount (0 or 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_transfer_safe_amount: Option<u8>,
    /// Whether to include LTV transfer safe amount (0 or 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub with_ltv_transfer_safe_amount: Option<u8>,
}

impl GetCoinBalanceParams {
    /// Create new parameters.
    pub fn new(account_type: AccountType, coin: impl Into<String>) -> Self {
        Self {
            account_type,
            coin: coin.into(),
            member_id: None,
            to_member_id: None,
            to_account_type: None,
            with_bonus: None,
            with_transfer_safe_amount: None,
            with_ltv_transfer_safe_amount: None,
        }
    }

    /// Set member ID.
    pub fn member_id(mut self, id: impl Into<String>) -> Self {
        self.member_id = Some(id.into());
        self
    }
}

/// Single coin balance result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CoinBalanceResult {
    /// Account type.
    #[serde(default)]
    pub account_type: Option<String>,
    /// Business type.
    #[serde(default)]
    pub biz_type: Option<i32>,
    /// Account ID.
    #[serde(default)]
    pub account_id: Option<String>,
    /// User ID.
    #[serde(default)]
    pub member_id: Option<String>,
    /// Balance.
    pub balance: CoinBalanceItem,
}

/// Parameters for getting transferable coins.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferableCoinParams {
    /// Source account type.
    pub from_account_type: AccountType,
    /// Destination account type.
    pub to_account_type: AccountType,
}

impl GetTransferableCoinParams {
    /// Create new parameters.
    pub fn new(from_account_type: AccountType, to_account_type: AccountType) -> Self {
        Self {
            from_account_type,
            to_account_type,
        }
    }
}

/// Transferable coin list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferableCoinResult {
    /// List of coin names.
    pub list: Vec<String>,
}

/// Parameters for creating an internal transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterTransferParams {
    /// Transfer ID (UUID generated by the caller).
    pub transfer_id: String,
    /// Coin name.
    pub coin: String,
    /// Amount.
    pub amount: String,
    /// Source account type.
    pub from_account_type: AccountType,
    /// Destination account type.
    pub to_account_type: AccountType,
}

impl InterTransferParams {
    /// Create new parameters.
    pub fn new(
        transfer_id: impl Into<String>,
        coin: impl Into<String>,
        amount: impl Into<String>,
        from_account_type: AccountType,
        to_account_type: AccountType,
    ) -> Self {
        Self {
            transfer_id: transfer_id.into(),
            coin: coin.into(),
            amount: amount.into(),
            from_account_type,
            to_account_type,
        }
    }
}

/// Parameters for creating a universal transfer.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UniversalTransferParams {
    /// Transfer ID (UUID generated by the caller).
    pub transfer_id: String,
    /// Coin name.
    pub coin: String,
    /// Amount.
    pub amount: String,
    /// Source UID.
    pub from_member_id: i64,
    /// Destination UID.
    pub to_member_id: i64,
    /// Source account type.
    pub from_account_type: AccountType,
    /// Destination account type.
    pub to_account_type: AccountType,
}

impl UniversalTransferParams {
    /// Create new parameters.
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        transfer_id: impl Into<String>,
        coin: impl Into<String>,
        amount: impl Into<String>,
        from_member_id: i64,
        to_member_id: i64,
        from_account_type: AccountType,
        to_account_type: AccountType,
    ) -> Self {
        Self {
            transfer_id: transfer_id.into(),
            coin: coin.into(),
            amount: amount.into(),
            from_member_id,
            to_member_id,
            from_account_type,
            to_account_type,
        }
    }
}

/// Transfer creation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferResult {
    /// Transfer ID.
    pub transfer_id: String,
    /// Transfer status.
    #[serde(default)]
    pub status: Option<String>,
}

/// Parameters for querying transfer records.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetTransferListParams {
    /// Transfer ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_id: Option<String>,
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Status filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetTransferListParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set start time.
    pub fn start_time(mut self, start: u64) -> Self {
        self.start_time = Some(start);
        self
    }

    /// Set end time.
    pub fn end_time(mut self, end: u64) -> Self {
        self.end_time = Some(end);
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

/// Transfer record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRecord {
    /// Transfer ID.
    pub transfer_id: String,
    /// Coin name.
    pub coin: String,
    /// Amount.
    pub amount: String,
    /// Source account type.
    #[serde(default)]
    pub from_account_type: Option<String>,
    /// Destination account type.
    #[serde(default)]
    pub to_account_type: Option<String>,
    /// Source UID (universal transfers).
    #[serde(default)]
    pub from_member_id: Option<String>,
    /// Destination UID (universal transfers).
    #[serde(default)]
    pub to_member_id: Option<String>,
    /// Timestamp (ms).
    #[serde(default)]
    pub timestamp: Option<String>,
    /// Transfer status.
    #[serde(default)]
    pub status: Option<String>,
}

/// Transfer record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TransferRecordListResult {
    /// List of transfer records.
    pub list: Vec<TransferRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Sub member list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubMemberListResult {
    /// Sub UIDs.
    #[serde(default)]
    pub sub_member_ids: Vec<String>,
    /// Sub UIDs with universal transfer enabled.
    #[serde(default)]
    pub transferable_sub_member_ids: Vec<String>,
}

/// Parameters for querying deposit records.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositRecordsParams {
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Deposit ID filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    /// Transaction ID filter.
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Sub UID (sub member deposit queries only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_member_id: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDepositRecordsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set sub member ID.
    pub fn sub_member_id(mut self, id: impl Into<String>) -> Self {
        self.sub_member_id = Some(id.into());
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

/// Deposit record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecord {
    /// Coin name.
    pub coin: String,
    /// Chain name.
    #[serde(default)]
    pub chain: Option<String>,
    /// Amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// Transaction ID.
    #[serde(rename = "txID", default)]
    pub tx_id: Option<String>,
    /// Deposit status.
    #[serde(default)]
    pub status: Option<i32>,
    /// Deposit address.
    #[serde(default)]
    pub to_address: Option<String>,
    /// Tag.
    #[serde(default)]
    pub tag: Option<String>,
    /// Deposit fee.
    #[serde(default)]
    pub deposit_fee: Option<String>,
    /// Success time (ms).
    #[serde(default)]
    pub success_at: Option<String>,
    /// Number of confirmations.
    #[serde(default)]
    pub confirmations: Option<String>,
    /// Transaction index.
    #[serde(default)]
    pub tx_index: Option<String>,
    /// Block hash.
    #[serde(default)]
    pub block_hash: Option<String>,
    /// Batch release limit.
    #[serde(default)]
    pub batch_release_limit: Option<String>,
    /// Deposit type.
    #[serde(default)]
    pub deposit_type: Option<String>,
}

/// Deposit record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositRecordListResult {
    /// List of deposit records.
    pub rows: Vec<DepositRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Internal deposit record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalDepositRecord {
    /// Record ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Deposit type.
    #[serde(rename = "type", default)]
    pub deposit_type: Option<i32>,
    /// Coin name.
    pub coin: String,
    /// Amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// Deposit status.
    #[serde(default)]
    pub status: Option<i32>,
    /// Deposit address.
    #[serde(default)]
    pub address: Option<String>,
    /// Created time (ms).
    #[serde(default)]
    pub created_time: Option<String>,
    /// Transaction ID.
    #[serde(rename = "txID", default)]
    pub tx_id: Option<String>,
}

/// Internal deposit record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InternalDepositRecordListResult {
    /// List of internal deposit records.
    pub rows: Vec<InternalDepositRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for querying a deposit address.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDepositAddressParams {
    /// Coin name.
    pub coin: String,
    /// Chain type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain_type: Option<String>,
    /// Sub UID (sub member address queries only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sub_member_id: Option<String>,
}

impl GetDepositAddressParams {
    /// Create new parameters.
    pub fn new(coin: impl Into<String>) -> Self {
        Self {
            coin: coin.into(),
            chain_type: None,
            sub_member_id: None,
        }
    }

    /// Set chain type.
    pub fn chain_type(mut self, chain_type: impl Into<String>) -> Self {
        self.chain_type = Some(chain_type.into());
        self
    }

    /// Set sub member ID.
    pub fn sub_member_id(mut self, id: impl Into<String>) -> Self {
        self.sub_member_id = Some(id.into());
        self
    }
}

/// Deposit address for a chain.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositChainAddress {
    /// Chain type.
    #[serde(default)]
    pub chain_type: Option<String>,
    /// Deposit address.
    #[serde(default)]
    pub address_deposit: Option<String>,
    /// Deposit tag.
    #[serde(default)]
    pub tag_deposit: Option<String>,
    /// Chain name.
    #[serde(default)]
    pub chain: Option<String>,
    /// Batch release limit.
    #[serde(default)]
    pub batch_release_limit: Option<String>,
    /// Contract address.
    #[serde(default)]
    pub contract_address: Option<String>,
}

/// Deposit address result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DepositAddressResult {
    /// Coin name.
    pub coin: String,
    /// Chain addresses.
    #[serde(default)]
    pub chains: Vec<DepositChainAddress>,
}

/// Parameters for querying allowed deposit coins.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAllowedDepositCoinInfoParams {
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Chain filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Limit (max 35).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetAllowedDepositCoinInfoParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Allowed deposit coin entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedDepositCoin {
    /// Coin name.
    pub coin: String,
    /// Chain name.
    #[serde(default)]
    pub chain: Option<String>,
    /// Coin display name.
    #[serde(default)]
    pub coin_show_name: Option<String>,
    /// Chain type.
    #[serde(default)]
    pub chain_type: Option<String>,
    /// Block confirmation number.
    #[serde(default)]
    pub block_confirm_number: Option<i32>,
    /// Minimum deposit amount.
    #[serde(default)]
    pub min_deposit_amount: Option<String>,
}

/// Allowed deposit coin list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AllowedDepositCoinResult {
    /// Allowed deposit coins.
    #[serde(default)]
    pub config_list: Vec<AllowedDepositCoin>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for setting the deposit account.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SetDepositAccountParams {
    /// Account type.
    pub account_type: AccountType,
}

impl SetDepositAccountParams {
    /// Create new parameters.
    pub fn new(account_type: AccountType) -> Self {
        Self { account_type }
    }
}

/// Operation status result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OperationStatusResult {
    /// Status (1: success, 0: failure).
    pub status: i32,
}

/// Parameters for creating a withdrawal.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawParams {
    /// Coin name.
    pub coin: String,
    /// Chain name (required unless `force_chain` is 2).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chain: Option<String>,
    /// Wallet address or Bybit UID for internal transfers.
    pub address: String,
    /// Tag (required if the address has a tag).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tag: Option<String>,
    /// Withdraw amount.
    pub amount: String,
    /// Current timestamp (ms) used for replay protection.
    pub timestamp: u64,
    /// Force chain flag (0: address book, 1: on-chain, 2: internal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_chain: Option<i32>,
    /// Account type to withdraw from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
    /// Fee type (0: netto, 1: brutto).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fee_type: Option<i32>,
    /// Custom request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl WithdrawParams {
    /// Create new parameters.
    pub fn new(
        coin: impl Into<String>,
        address: impl Into<String>,
        amount: impl Into<String>,
        timestamp: u64,
    ) -> Self {
        Self {
            coin: coin.into(),
            chain: None,
            address: address.into(),
            tag: None,
            amount: amount.into(),
            timestamp,
            force_chain: None,
            account_type: None,
            fee_type: None,
            request_id: None,
        }
    }

    /// Set chain.
    pub fn chain(mut self, chain: impl Into<String>) -> Self {
        self.chain = Some(chain.into());
        self
    }

    /// Set tag.
    pub fn tag(mut self, tag: impl Into<String>) -> Self {
        self.tag = Some(tag.into());
        self
    }

    /// Set account type.
    pub fn account_type(mut self, account_type: AccountType) -> Self {
        self.account_type = Some(account_type);
        self
    }
}

/// Withdrawal creation result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawResult {
    /// Withdrawal ID.
    pub id: String,
}

/// Parameters for cancelling a withdrawal.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelWithdrawalParams {
    /// Withdrawal ID.
    pub id: String,
}

impl CancelWithdrawalParams {
    /// Create new parameters.
    pub fn new(id: impl Into<String>) -> Self {
        Self { id: id.into() }
    }
}

/// Parameters for querying withdrawal records.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalRecordsParams {
    /// Withdrawal ID filter.
    #[serde(rename = "withdrawID", skip_serializing_if = "Option::is_none")]
    pub withdraw_id: Option<String>,
    /// Transaction ID filter.
    #[serde(rename = "txID", skip_serializing_if = "Option::is_none")]
    pub tx_id: Option<String>,
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Withdraw type (0: on-chain, 1: off-chain, 2: all).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub withdraw_type: Option<i32>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWithdrawalRecordsParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
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

/// Withdrawal record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecord {
    /// Withdrawal ID.
    #[serde(default)]
    pub withdraw_id: Option<String>,
    /// Transaction ID.
    #[serde(rename = "txID", default)]
    pub tx_id: Option<String>,
    /// Withdraw type (0: on-chain, 1: off-chain).
    #[serde(default)]
    pub withdraw_type: Option<i32>,
    /// Coin name.
    pub coin: String,
    /// Chain name.
    #[serde(default)]
    pub chain: Option<String>,
    /// Amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// Withdraw fee.
    #[serde(default)]
    pub withdraw_fee: Option<String>,
    /// Withdrawal status.
    #[serde(default)]
    pub status: Option<String>,
    /// Destination address.
    #[serde(default)]
    pub to_address: Option<String>,
    /// Tag.
    #[serde(default)]
    pub tag: Option<String>,
    /// Created time (ms).
    #[serde(default)]
    pub create_time: Option<String>,
    /// Updated time (ms).
    #[serde(default)]
    pub update_time: Option<String>,
}

/// Withdrawal record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalRecordListResult {
    /// List of withdrawal records.
    pub rows: Vec<WithdrawalRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for querying the withdrawal address list.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawalAddressParams {
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWithdrawalAddressParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }
}

/// Withdrawal address entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalAddress {
    /// Coin name.
    #[serde(default)]
    pub coin: Option<String>,
    /// Chain name.
    #[serde(default)]
    pub chain: Option<String>,
    /// Address.
    #[serde(default)]
    pub address: Option<String>,
    /// Tag.
    #[serde(default)]
    pub tag: Option<String>,
    /// Address remark.
    #[serde(default)]
    pub remark: Option<String>,
    /// Whether the address is verified.
    #[serde(default)]
    pub verified: Option<i32>,
}

/// Withdrawal address list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawalAddressListResult {
    /// List of addresses.
    #[serde(default)]
    pub rows: Vec<WithdrawalAddress>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for querying the withdrawable amount.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetWithdrawableAmountParams {
    /// Coin name.
    pub coin: String,
}

impl GetWithdrawableAmountParams {
    /// Create new parameters.
    pub fn new(coin: impl Into<String>) -> Self {
        Self { coin: coin.into() }
    }
}

/// Withdrawable amount entry per account.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountEntry {
    /// Coin name.
    #[serde(default)]
    pub coin: Option<String>,
    /// Withdrawable amount.
    #[serde(default)]
    pub withdrawable_amount: Option<String>,
    /// Available balance.
    #[serde(default)]
    pub available_balance: Option<String>,
}

/// Withdrawable amount result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WithdrawableAmountResult {
    /// Delayed withdraw amount in USD.
    #[serde(default)]
    pub limit_amount_usd: Option<String>,
    /// Withdrawable amount per account type.
    #[serde(default)]
    pub withdrawable_amount: HashMap<String, WithdrawableAmountEntry>,
}

/// VASP (exchange entity) information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaspEntity {
    /// VASP entity ID.
    pub vasp_entity_id: String,
    /// VASP name.
    #[serde(default)]
    pub vasp_name: Option<String>,
}

/// VASP list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaspListResult {
    /// List of VASP entities.
    #[serde(default)]
    pub vasp: Vec<VaspEntity>,
}

/// Parameters for querying delivery records.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetDeliveryRecordParams {
    /// Product category (linear, option).
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Expiry date filter (e.g. `25MAR22`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_date: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDeliveryRecordParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            exp_date: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Delivery record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRecord {
    /// Trading symbol.
    pub symbol: String,
    /// Order side.
    #[serde(default)]
    pub side: Option<String>,
    /// Position size.
    #[serde(default)]
    pub position: Option<String>,
    /// Delivery price.
    #[serde(default)]
    pub delivery_price: Option<String>,
    /// Strike price (options).
    #[serde(default)]
    pub strike: Option<String>,
    /// Delivery fee.
    #[serde(default)]
    pub fee: Option<String>,
    /// Realized PnL from delivery.
    #[serde(default)]
    pub delivery_rpl: Option<String>,
    /// Delivery time (ms).
    #[serde(default)]
    pub delivery_time: Option<i64>,
}

/// Delivery record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeliveryRecordListResult {
    /// Product category.
    #[serde(default)]
    pub category: Option<String>,
    /// List of delivery records.
    pub list: Vec<DeliveryRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for querying settlement records.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSettlementRecordParams {
    /// Product category (linear).
    pub category: Category,
    /// Trading symbol.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub symbol: Option<String>,
    /// Start time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_time: Option<u64>,
    /// End time (ms).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_time: Option<u64>,
    /// Limit (max 50).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSettlementRecordParams {
    /// Create new parameters.
    pub fn new(category: Category) -> Self {
        Self {
            category,
            symbol: None,
            start_time: None,
            end_time: None,
            limit: None,
            cursor: None,
        }
    }

    /// Set symbol filter.
    pub fn symbol(mut self, symbol: impl Into<String>) -> Self {
        self.symbol = Some(symbol.into());
        self
    }
}

/// Settlement record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementRecord {
    /// Trading symbol.
    pub symbol: String,
    /// Position side.
    #[serde(default)]
    pub side: Option<String>,
    /// Position size.
    #[serde(default)]
    pub size: Option<String>,
    /// Session average price.
    #[serde(default)]
    pub session_avg_price: Option<String>,
    /// Mark price.
    #[serde(default)]
    pub mark_price: Option<String>,
    /// Realized PnL.
    #[serde(default)]
    pub realised_pnl: Option<String>,
    /// Created time (ms).
    #[serde(default)]
    pub created_time: Option<String>,
}

/// Settlement record list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettlementRecordListResult {
    /// Product category.
    #[serde(default)]
    pub category: Option<String>,
    /// List of settlement records.
    pub list: Vec<SettlementRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Parameters for querying convert coins.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertCoinListParams {
    /// Wallet type (e.g. `eb_convert_uta`, `eb_convert_funding`).
    pub account_type: String,
    /// Coin filter (only valid for `side` = 1).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Side (0: from coin list, 1: to coin list).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub side: Option<i32>,
}

impl GetConvertCoinListParams {
    /// Create new parameters.
    pub fn new(account_type: impl Into<String>) -> Self {
        Self {
            account_type: account_type.into(),
            coin: None,
            side: None,
        }
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set side.
    pub fn side(mut self, side: i32) -> Self {
        self.side = Some(side);
        self
    }
}

/// Convertible coin information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertCoin {
    /// Coin name.
    pub coin: String,
    /// Coin full name.
    #[serde(default)]
    pub full_name: Option<String>,
    /// Coin type (crypto or fiat).
    #[serde(default)]
    pub coin_type: Option<String>,
    /// Balance.
    #[serde(default)]
    pub balance: Option<String>,
    /// Balance in USDT.
    #[serde(default)]
    pub u_balance: Option<String>,
    /// Precision.
    #[serde(default)]
    pub accuracy_length: Option<i32>,
    /// Minimum convert amount.
    #[serde(default)]
    pub single_from_min_limit: Option<String>,
    /// Maximum convert amount.
    #[serde(default)]
    pub single_from_max_limit: Option<String>,
}

/// Convert coin list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertCoinListResult {
    /// List of convertible coins.
    pub coins: Vec<ConvertCoin>,
}

/// Parameters for requesting a convert quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestConvertQuoteParams {
    /// Coin to sell.
    pub from_coin: String,
    /// Coin to buy.
    pub to_coin: String,
    /// Request coin, same as `from_coin`.
    pub request_coin: String,
    /// Amount of the request coin to sell.
    pub request_amount: String,
    /// Wallet type.
    pub account_type: String,
    /// From coin type (crypto or fiat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_coin_type: Option<String>,
    /// To coin type (crypto or fiat).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub to_coin_type: Option<String>,
    /// Custom request ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
}

impl RequestConvertQuoteParams {
    /// Create new parameters.
    pub fn new(
        from_coin: impl Into<String>,
        to_coin: impl Into<String>,
        request_amount: impl Into<String>,
        account_type: impl Into<String>,
    ) -> Self {
        let from_coin = from_coin.into();
        Self {
            request_coin: from_coin.clone(),
            from_coin,
            to_coin: to_coin.into(),
            request_amount: request_amount.into(),
            account_type: account_type.into(),
            from_coin_type: None,
            to_coin_type: None,
            request_id: None,
        }
    }
}

/// Convert quote result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertQuoteResult {
    /// Quote transaction ID.
    pub quote_tx_id: String,
    /// Exchange rate.
    #[serde(default)]
    pub exchange_rate: Option<String>,
    /// From coin.
    #[serde(default)]
    pub from_coin: Option<String>,
    /// From amount.
    #[serde(default)]
    pub from_amount: Option<String>,
    /// To coin.
    #[serde(default)]
    pub to_coin: Option<String>,
    /// To amount.
    #[serde(default)]
    pub to_amount: Option<String>,
    /// Quote expiry time (ms).
    #[serde(default)]
    pub expired_time: Option<String>,
    /// Request ID.
    #[serde(default)]
    pub request_id: Option<String>,
}

/// Parameters for confirming a convert quote.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConfirmConvertQuoteParams {
    /// Quote transaction ID.
    pub quote_tx_id: String,
}

impl ConfirmConvertQuoteParams {
    /// Create new parameters.
    pub fn new(quote_tx_id: impl Into<String>) -> Self {
        Self {
            quote_tx_id: quote_tx_id.into(),
        }
    }
}

/// Convert execution result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertExecuteResult {
    /// Quote transaction ID.
    pub quote_tx_id: String,
    /// Exchange status.
    #[serde(default)]
    pub exchange_status: Option<String>,
}

/// Parameters for querying convert status.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertStatusParams {
    /// Quote transaction ID.
    pub quote_tx_id: String,
    /// Wallet type.
    pub account_type: String,
}

impl GetConvertStatusParams {
    /// Create new parameters.
    pub fn new(quote_tx_id: impl Into<String>, account_type: impl Into<String>) -> Self {
        Self {
            quote_tx_id: quote_tx_id.into(),
            account_type: account_type.into(),
        }
    }
}

/// Convert record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertRecord {
    /// Account type.
    #[serde(default)]
    pub account_type: Option<String>,
    /// Exchange transaction ID.
    #[serde(default)]
    pub exchange_tx_id: Option<String>,
    /// User ID.
    #[serde(default)]
    pub user_id: Option<String>,
    /// From coin.
    #[serde(default)]
    pub from_coin: Option<String>,
    /// From coin type.
    #[serde(default)]
    pub from_coin_type: Option<String>,
    /// From amount.
    #[serde(default)]
    pub from_amount: Option<String>,
    /// To coin.
    #[serde(default)]
    pub to_coin: Option<String>,
    /// To coin type.
    #[serde(default)]
    pub to_coin_type: Option<String>,
    /// To amount.
    #[serde(default)]
    pub to_amount: Option<String>,
    /// Exchange status.
    #[serde(default)]
    pub exchange_status: Option<String>,
    /// Convert rate.
    #[serde(default)]
    pub convert_rate: Option<String>,
    /// Created time.
    #[serde(default)]
    pub created_at: Option<String>,
}

/// Convert status result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertStatusResult {
    /// Convert record.
    pub result: ConvertRecord,
}

/// Parameters for querying convert history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetConvertHistoryParams {
    /// Wallet types, comma separated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<String>,
    /// Page number, starting from 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub index: Option<u32>,
    /// Limit (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
}

impl GetConvertHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Convert history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConvertHistoryResult {
    /// List of convert records.
    pub list: Vec<ConvertRecord>,
}

/// Parameters for querying the asset overview.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAssetOverviewParams {
    /// User ID (required for sub account queries with master API key).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    /// Valuation currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub valuation_currency: Option<String>,
    /// Account type filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<AccountType>,
}

impl GetAssetOverviewParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set member ID.
    pub fn member_id(mut self, id: impl Into<String>) -> Self {
        self.member_id = Some(id.into());
        self
    }
}

/// Asset overview result.
///
/// The category breakdown format changes frequently, so it is kept as raw JSON.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssetOverviewResult {
    /// Total asset value.
    #[serde(default)]
    pub total_value: Option<String>,
    /// Valuation currency.
    #[serde(default)]
    pub valuation_currency: Option<String>,
    /// Per-category asset breakdown.
    #[serde(default)]
    pub categories: Option<serde_json::Value>,
    /// Flat coin detail breakdown (older response format).
    #[serde(default)]
    pub coin_detail: Option<serde_json::Value>,
}

/// Parameters for querying funding account history.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetFundHistoryParams {
    /// Start timestamp (seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_from: Option<String>,
    /// End timestamp (seconds).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub create_time_to: Option<String>,
    /// Coin filter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coin: Option<String>,
    /// Limit (1 to 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFundHistoryParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set coin filter.
    pub fn coin(mut self, coin: impl Into<String>) -> Self {
        self.coin = Some(coin.into());
        self
    }

    /// Set limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }
}

/// Funding account history record.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundHistoryRecord {
    /// Coin name.
    #[serde(default)]
    pub coin: Option<String>,
    /// Transaction type.
    #[serde(rename = "type", default)]
    pub transaction_type: Option<String>,
    /// Amount.
    #[serde(default)]
    pub amount: Option<String>,
    /// Fee.
    #[serde(default)]
    pub fee: Option<String>,
    /// Status.
    #[serde(default)]
    pub status: Option<String>,
    /// Created time.
    #[serde(default)]
    pub create_time: Option<String>,
}

/// Funding account history result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FundHistoryResult {
    /// List of records.
    #[serde(default)]
    pub list: Vec<FundHistoryRecord>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}
