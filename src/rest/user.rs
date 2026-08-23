//! User API endpoints for sub-account and API key management.

use crate::error::BybitError;
use crate::http::HttpClient;
use crate::types::user::*;

/// User service for sub-account and API key management endpoints.
///
/// Most endpoints require the master account API key.
#[derive(Debug, Clone)]
pub struct UserService {
    http: HttpClient,
}

impl UserService {
    /// Create a new user service.
    pub fn new(http: HttpClient) -> Self {
        Self { http }
    }

    /// Create a new sub member.
    ///
    /// # Example
    ///
    /// ```no_run
    /// # use bybit_client::BybitClient;
    /// # use bybit_client::types::user::CreateSubMemberParams;
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = BybitClient::new("api_key", "api_secret")?;
    ///
    /// let params = CreateSubMemberParams::new("subuser01").note("trading bot");
    /// let result = client.user().create_sub_member(&params).await?;
    /// println!("Created sub member: {}", result.uid);
    /// # Ok(())
    /// # }
    /// ```
    pub async fn create_sub_member(
        &self,
        params: &CreateSubMemberParams,
    ) -> Result<CreateSubMemberResult, BybitError> {
        self.http
            .post_signed("/v5/user/create-sub-member", Some(params))
            .await
    }

    /// Get the sub member list (up to 10k sub accounts).
    pub async fn get_sub_members(&self) -> Result<SubMemberListResult, BybitError> {
        self.http
            .get_signed::<SubMemberListResult, ()>("/v5/user/query-sub-members", None)
            .await
    }

    /// Get the paginated sub member list.
    ///
    /// Use this endpoint for accounts with more than 10k sub accounts.
    pub async fn get_sub_members_page(
        &self,
        params: &GetSubMembersPageParams,
    ) -> Result<SubMemberPageResult, BybitError> {
        self.http
            .get_signed("/v5/user/submembers", Some(params))
            .await
    }

    /// Create an API key for a sub member.
    pub async fn create_sub_api_key(
        &self,
        params: &CreateSubApiKeyParams,
    ) -> Result<CreateSubApiKeyResult, BybitError> {
        self.http
            .post_signed("/v5/user/create-sub-api", Some(params))
            .await
    }

    /// Get all API keys of a sub member.
    pub async fn get_sub_api_keys(
        &self,
        params: &GetSubApiKeysParams,
    ) -> Result<SubApiKeysResult, BybitError> {
        self.http
            .get_signed("/v5/user/sub-apikeys", Some(params))
            .await
    }

    /// Get information about the API key used for this request.
    pub async fn get_api_key_info(&self) -> Result<ApiKeyInfo, BybitError> {
        self.http
            .get_signed::<ApiKeyInfo, ()>("/v5/user/query-api", None)
            .await
    }

    /// Update the master account API key used for this request.
    pub async fn update_master_api_key(
        &self,
        params: &UpdateApiKeyParams,
    ) -> Result<UpdateApiKeyResult, BybitError> {
        self.http
            .post_signed("/v5/user/update-api", Some(params))
            .await
    }

    /// Update the sub account API key used for this request.
    pub async fn update_sub_api_key(
        &self,
        params: &UpdateApiKeyParams,
    ) -> Result<UpdateApiKeyResult, BybitError> {
        self.http
            .post_signed("/v5/user/update-sub-api", Some(params))
            .await
    }

    /// Delete the master account API key used for this request.
    ///
    /// The key becomes invalid immediately.
    pub async fn delete_master_api_key(&self) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed::<serde_json::Value, ()>("/v5/user/delete-api", None)
            .await?;
        Ok(())
    }

    /// Delete the sub account API key used for this request.
    ///
    /// The key becomes invalid immediately.
    pub async fn delete_sub_api_key(&self) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed::<serde_json::Value, ()>("/v5/user/delete-sub-api", None)
            .await?;
        Ok(())
    }

    /// Freeze or unfreeze a sub member.
    pub async fn freeze_sub_member(
        &self,
        params: &FreezeSubMemberParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/user/frozen-sub-member", Some(params))
            .await?;
        Ok(())
    }

    /// Delete a sub member.
    ///
    /// The sub member must not hold any assets.
    pub async fn delete_sub_member(
        &self,
        params: &DeleteSubMemberParams,
    ) -> Result<(), BybitError> {
        let _: serde_json::Value = self
            .http
            .post_signed("/v5/user/del-submember", Some(params))
            .await?;
        Ok(())
    }

    /// Get available wallet types for the current or given accounts.
    pub async fn get_member_type(
        &self,
        params: &GetMemberTypeParams,
    ) -> Result<MemberTypeResult, BybitError> {
        self.http
            .get_signed("/v5/user/get-member-type", Some(params))
            .await
    }

    /// Get affiliate customer information.
    ///
    /// Requires an affiliate account with the Affiliate permission.
    pub async fn get_aff_customer_info(
        &self,
        params: &GetAffCustomerInfoParams,
    ) -> Result<AffCustomerInfo, BybitError> {
        self.http
            .get_signed("/v5/user/aff-customer-info", Some(params))
            .await
    }

    /// Get the affiliate user list.
    pub async fn get_affiliate_user_list(
        &self,
        params: &GetAffiliateUserListParams,
    ) -> Result<AffiliateUserListResult, BybitError> {
        self.http
            .get_signed("/v5/affiliate/aff-user-list", Some(params))
            .await
    }
}
