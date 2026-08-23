//! Types for the user management endpoints.

use serde::{Deserialize, Serialize};

/// API key permissions grouped by product area.
/// Field values are permission strings such as `Order` or `Position`.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ApiKeyPermissions {
    /// Contract trading permissions.
    #[serde(rename = "ContractTrade", default, skip_serializing_if = "Option::is_none")]
    pub contract_trade: Option<Vec<String>>,
    /// Spot trading permissions.
    #[serde(rename = "Spot", default, skip_serializing_if = "Option::is_none")]
    pub spot: Option<Vec<String>>,
    /// Wallet permissions.
    #[serde(rename = "Wallet", default, skip_serializing_if = "Option::is_none")]
    pub wallet: Option<Vec<String>>,
    /// Options trading permissions.
    #[serde(rename = "Options", default, skip_serializing_if = "Option::is_none")]
    pub options: Option<Vec<String>>,
    /// Derivatives permissions.
    #[serde(rename = "Derivatives", default, skip_serializing_if = "Option::is_none")]
    pub derivatives: Option<Vec<String>>,
    /// Copy trading permissions.
    #[serde(rename = "CopyTrading", default, skip_serializing_if = "Option::is_none")]
    pub copy_trading: Option<Vec<String>>,
    /// Block trade permissions.
    #[serde(rename = "BlockTrade", default, skip_serializing_if = "Option::is_none")]
    pub block_trade: Option<Vec<String>>,
    /// Exchange (convert) permissions.
    #[serde(rename = "Exchange", default, skip_serializing_if = "Option::is_none")]
    pub exchange: Option<Vec<String>>,
    /// NFT permissions (deprecated).
    #[serde(rename = "NFT", default, skip_serializing_if = "Option::is_none")]
    pub nft: Option<Vec<String>>,
    /// Earn permissions.
    #[serde(rename = "Earn", default, skip_serializing_if = "Option::is_none")]
    pub earn: Option<Vec<String>>,
    /// Affiliate permissions.
    #[serde(rename = "Affiliate", default, skip_serializing_if = "Option::is_none")]
    pub affiliate: Option<Vec<String>>,
    /// Fiat P2P permissions.
    #[serde(rename = "FiatP2P", default, skip_serializing_if = "Option::is_none")]
    pub fiat_p2p: Option<Vec<String>>,
    /// Bybit Pay permissions.
    #[serde(rename = "FiatBitPay", default, skip_serializing_if = "Option::is_none")]
    pub fiat_bit_pay: Option<Vec<String>>,
}

/// Parameters for creating a sub member.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubMemberParams {
    /// Username (6-16 characters, letters and numbers).
    pub username: String,
    /// Login password.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Member type (1: normal, 6: custodial).
    pub member_type: i32,
    /// Quick login switch (0: disabled, 1: enabled).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub switch: Option<i32>,
    /// Create as UTA account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_uta: Option<bool>,
    /// Note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
}

impl CreateSubMemberParams {
    /// Create parameters for a normal sub account.
    pub fn new(username: impl Into<String>) -> Self {
        Self {
            username: username.into(),
            password: None,
            member_type: 1,
            switch: None,
            is_uta: None,
            note: None,
        }
    }

    /// Set the password.
    pub fn password(mut self, password: impl Into<String>) -> Self {
        self.password = Some(password.into());
        self
    }

    /// Set the member type.
    pub fn member_type(mut self, member_type: i32) -> Self {
        self.member_type = member_type;
        self
    }

    /// Set a note.
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Result of creating a sub member.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubMemberResult {
    /// Sub member UID.
    pub uid: String,
    /// Username.
    pub username: String,
    /// Member type.
    #[serde(default)]
    pub member_type: Option<i32>,
    /// Status.
    #[serde(default)]
    pub status: Option<i32>,
    /// Remark.
    #[serde(default)]
    pub remark: Option<String>,
}

/// Sub member information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubMember {
    /// Sub member UID.
    pub uid: String,
    /// Username.
    pub username: String,
    /// Member type.
    #[serde(default)]
    pub member_type: Option<i32>,
    /// Status.
    #[serde(default)]
    pub status: Option<i32>,
    /// Account mode.
    #[serde(default)]
    pub account_mode: Option<i32>,
    /// Remark.
    #[serde(default)]
    pub remark: Option<String>,
}

/// Sub member list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubMemberListResult {
    /// List of sub members.
    #[serde(default)]
    pub sub_members: Vec<SubMember>,
}

/// Parameters for the paginated sub member list.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubMembersPageParams {
    /// Page size (max 100).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub page_size: Option<String>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
}

/// Paginated sub member list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubMemberPageResult {
    /// List of sub members.
    #[serde(default)]
    pub sub_members: Vec<SubMember>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_cursor: Option<String>,
}

/// Parameters for creating a sub account API key.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubApiKeyParams {
    /// Sub user ID.
    pub subuid: i64,
    /// Note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    /// Read only flag (0: read and write, 1: read only).
    pub read_only: i32,
    /// Comma separated IP whitelist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    /// Permissions.
    pub permissions: ApiKeyPermissions,
}

impl CreateSubApiKeyParams {
    /// Create new parameters.
    pub fn new(subuid: i64, read_only: i32, permissions: ApiKeyPermissions) -> Self {
        Self {
            subuid,
            note: None,
            read_only,
            ips: None,
            permissions,
        }
    }

    /// Set the IP whitelist.
    pub fn ips(mut self, ips: impl Into<String>) -> Self {
        self.ips = Some(ips.into());
        self
    }

    /// Set a note.
    pub fn note(mut self, note: impl Into<String>) -> Self {
        self.note = Some(note.into());
        self
    }
}

/// Result of creating an API key.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubApiKeyResult {
    /// Key ID.
    pub id: String,
    /// Note.
    #[serde(default)]
    pub note: Option<String>,
    /// API key.
    pub api_key: String,
    /// Read only flag.
    #[serde(default)]
    pub read_only: Option<i32>,
    /// API secret.
    pub secret: String,
    /// Permissions.
    #[serde(default)]
    pub permissions: ApiKeyPermissions,
}

/// API key information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyInfo {
    /// Key ID.
    pub id: String,
    /// Note.
    #[serde(default)]
    pub note: Option<String>,
    /// API key.
    pub api_key: String,
    /// Read only flag.
    #[serde(default)]
    pub read_only: Option<i32>,
    /// Permissions.
    #[serde(default)]
    pub permissions: ApiKeyPermissions,
    /// IP whitelist.
    #[serde(default)]
    pub ips: Vec<String>,
    /// Key type (1: personal, 2: third-party app).
    #[serde(rename = "type", default)]
    pub key_type: Option<i32>,
    /// Days until expiration.
    #[serde(default)]
    pub deadline_day: Option<i64>,
    /// Expiration time.
    #[serde(default)]
    pub expired_at: Option<String>,
    /// Creation time.
    #[serde(default)]
    pub created_at: Option<String>,
    /// UTA flag (0: classic, 1: unified).
    #[serde(default)]
    pub uta: Option<i32>,
    /// User ID.
    #[serde(rename = "userID", default)]
    pub user_id: Option<i64>,
    /// Inviter ID.
    #[serde(rename = "inviterID", default)]
    pub inviter_id: Option<i64>,
    /// VIP level.
    #[serde(default)]
    pub vip_level: Option<String>,
    /// Market maker level.
    #[serde(default)]
    pub mkt_maker_level: Option<String>,
    /// Affiliate ID.
    #[serde(rename = "affiliateID", default)]
    pub affiliate_id: Option<i64>,
    /// RSA public key.
    #[serde(default)]
    pub rsa_public_key: Option<String>,
    /// Is master account key.
    #[serde(default)]
    pub is_master: Option<bool>,
    /// Parent UID.
    #[serde(default)]
    pub parent_uid: Option<String>,
    /// KYC level.
    #[serde(default)]
    pub kyc_level: Option<String>,
    /// KYC region.
    #[serde(default)]
    pub kyc_region: Option<String>,
}

/// Parameters for updating a master or sub API key.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiKeyParams {
    /// Read only flag (0: read and write, 1: read only).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub read_only: Option<i32>,
    /// Comma separated IP whitelist.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ips: Option<String>,
    /// Permissions. Required by the API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permissions: Option<ApiKeyPermissions>,
}

impl UpdateApiKeyParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the read only flag.
    pub fn read_only(mut self, read_only: i32) -> Self {
        self.read_only = Some(read_only);
        self
    }

    /// Set the IP whitelist.
    pub fn ips(mut self, ips: impl Into<String>) -> Self {
        self.ips = Some(ips.into());
        self
    }

    /// Set the permissions.
    pub fn permissions(mut self, permissions: ApiKeyPermissions) -> Self {
        self.permissions = Some(permissions);
        self
    }
}

/// Result of updating an API key.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiKeyResult {
    /// Key ID.
    #[serde(default)]
    pub id: Option<String>,
    /// Note.
    #[serde(default)]
    pub note: Option<String>,
    /// API key.
    #[serde(default)]
    pub api_key: Option<String>,
    /// Read only flag.
    #[serde(default)]
    pub read_only: Option<i32>,
    /// Permissions.
    #[serde(default)]
    pub permissions: ApiKeyPermissions,
    /// IP whitelist.
    #[serde(default)]
    pub ips: Vec<String>,
}

/// Parameters for freezing a sub member.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FreezeSubMemberParams {
    /// Sub user ID.
    pub subuid: i64,
    /// Frozen flag (0: unfreeze, 1: freeze).
    pub frozen: i32,
}

impl FreezeSubMemberParams {
    /// Freeze the given sub member.
    pub fn freeze(subuid: i64) -> Self {
        Self { subuid, frozen: 1 }
    }

    /// Unfreeze the given sub member.
    pub fn unfreeze(subuid: i64) -> Self {
        Self { subuid, frozen: 0 }
    }
}

/// Parameters for deleting a sub member.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DeleteSubMemberParams {
    /// Sub member ID.
    pub sub_member_id: String,
}

impl DeleteSubMemberParams {
    /// Create new parameters.
    pub fn new(sub_member_id: impl Into<String>) -> Self {
        Self {
            sub_member_id: sub_member_id.into(),
        }
    }
}

/// Parameters for listing all API keys of a sub member.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetSubApiKeysParams {
    /// Sub member ID.
    pub sub_member_id: String,
    /// Page size (max 20).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSubApiKeysParams {
    /// Create new parameters.
    pub fn new(sub_member_id: impl Into<String>) -> Self {
        Self {
            sub_member_id: sub_member_id.into(),
            limit: None,
            cursor: None,
        }
    }

    /// Set the limit.
    pub fn limit(mut self, limit: u32) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set the cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

/// Sub member API key list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SubApiKeysResult {
    /// List of API keys.
    #[serde(default)]
    pub result: Vec<ApiKeyInfo>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

/// Wallet type entry for a UID.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberWalletType {
    /// User ID.
    pub uid: String,
    /// Available wallet types.
    #[serde(default)]
    pub wallet_type: Vec<String>,
}

/// Result of the member type query.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MemberTypeResult {
    /// Accounts and their wallet types.
    #[serde(default)]
    pub accounts: Vec<MemberWalletType>,
}

/// Parameters for the member type query.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetMemberTypeParams {
    /// Comma separated UIDs. Queries the current account when omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_ids: Option<String>,
}

impl GetMemberTypeParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the member IDs.
    pub fn member_ids(mut self, ids: impl Into<String>) -> Self {
        self.member_ids = Some(ids.into());
        self
    }
}

/// Parameters for the affiliate customer info query.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffCustomerInfoParams {
    /// The master account UID of the affiliate client.
    pub uid: String,
}

impl GetAffCustomerInfoParams {
    /// Create new parameters.
    pub fn new(uid: impl Into<String>) -> Self {
        Self { uid: uid.into() }
    }
}

/// Affiliate customer information.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffCustomerInfo {
    /// User ID.
    #[serde(default)]
    pub uid: Option<String>,
    /// VIP level.
    #[serde(default)]
    pub vip_level: Option<String>,
    /// Registration date.
    #[serde(default)]
    pub take_profit_time: Option<String>,
    /// Total wallet balance range.
    #[serde(default)]
    pub total_wallet_balance: Option<String>,
    /// Deposit updated time.
    #[serde(default)]
    pub deposit_update_time: Option<String>,
    /// Volume updated time.
    #[serde(default)]
    pub vol_update_time: Option<String>,
    /// KYC level.
    #[serde(default)]
    pub kyc_level: Option<String>,
}

/// Parameters for the affiliate user list.
#[derive(Debug, Clone, Default, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GetAffiliateUserListParams {
    /// Page size (max 1000).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u32>,
    /// Cursor for pagination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Include deposit information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub need_deposit: Option<bool>,
    /// Include 30-day metrics.
    #[serde(rename = "need30", skip_serializing_if = "Option::is_none")]
    pub need_30: Option<bool>,
    /// Include 365-day metrics.
    #[serde(rename = "need365", skip_serializing_if = "Option::is_none")]
    pub need_365: Option<bool>,
}

impl GetAffiliateUserListParams {
    /// Create new parameters.
    pub fn new() -> Self {
        Self::default()
    }

    /// Set the page size.
    pub fn size(mut self, size: u32) -> Self {
        self.size = Some(size);
        self
    }

    /// Set the cursor.
    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

/// Affiliate user entry.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateUser {
    /// User ID.
    #[serde(default)]
    pub user_id: Option<String>,
    /// Registration time.
    #[serde(default)]
    pub register_time: Option<String>,
    /// Source of the referral.
    #[serde(default)]
    pub source: Option<String>,
    /// Remarks.
    #[serde(default)]
    pub remarks: Option<String>,
    /// Is KYC approved.
    #[serde(default)]
    pub is_kyc: Option<bool>,
}

/// Affiliate user list result.
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliateUserListResult {
    /// List of affiliate users.
    #[serde(default)]
    pub list: Vec<AffiliateUser>,
    /// Cursor for next page.
    #[serde(default)]
    pub next_page_cursor: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_sub_member_params_serialization() {
        let params = CreateSubMemberParams::new("testuser01").note("bot account");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize params: {}", err),
        };
        assert!(json.contains("\"username\":\"testuser01\""));
        assert!(json.contains("\"memberType\":1"));
        assert!(json.contains("\"note\":\"bot account\""));
        assert!(!json.contains("password"));
    }

    #[test]
    fn test_create_sub_api_key_params_serialization() {
        let permissions = ApiKeyPermissions {
            contract_trade: Some(vec!["Order".to_string(), "Position".to_string()]),
            ..Default::default()
        };
        let params = CreateSubApiKeyParams::new(123456, 0, permissions).ips("10.0.0.1");
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize params: {}", err),
        };
        assert!(json.contains("\"subuid\":123456"));
        assert!(json.contains("\"readOnly\":0"));
        assert!(json.contains("\"ContractTrade\":[\"Order\",\"Position\"]"));
        assert!(!json.contains("\"Spot\""));
    }

    #[test]
    fn test_freeze_params_serialization() {
        let params = FreezeSubMemberParams::freeze(42);
        let json = match serde_json::to_string(&params) {
            Ok(json) => json,
            Err(err) => panic!("Failed to serialize params: {}", err),
        };
        assert!(json.contains("\"subuid\":42"));
        assert!(json.contains("\"frozen\":1"));
    }

    #[test]
    fn test_api_key_info_deserialization() {
        let json = r#"{
            "id": "1",
            "note": "test",
            "apiKey": "abc",
            "readOnly": 0,
            "permissions": {"ContractTrade": ["Order"]},
            "ips": ["*"],
            "type": 1,
            "uta": 1,
            "userID": 100,
            "isMaster": true
        }"#;
        let info: ApiKeyInfo = match serde_json::from_str(json) {
            Ok(info) => info,
            Err(err) => panic!("Failed to deserialize API key info: {}", err),
        };
        assert_eq!(info.api_key, "abc");
        assert_eq!(info.uta, Some(1));
        assert_eq!(info.user_id, Some(100));
        assert_eq!(
            info.permissions.contract_trade,
            Some(vec!["Order".to_string()])
        );
    }
}
