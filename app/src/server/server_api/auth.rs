//! Local-only auth tombstone for Warp Lite.
//!
//! Account creation, Firebase token refresh, cloud settings sync, billing API
//! keys, and ambient workload auth are intentionally disabled.

use std::result::Result as StdResult;

use anyhow::{bail, Result};
use async_trait::async_trait;
use instant::Duration;
#[cfg(test)]
use mockall::automock;
use thiserror::Error;
use warp_graphql::mutations::create_anonymous_user::{
    AnonymousUserType, CreateAnonymousUserResult,
};
use warp_graphql::mutations::expire_api_key::ExpireApiKeyResult;
use warp_graphql::mutations::generate_api_key::GenerateApiKeyResult;
use warp_graphql::mutations::mint_custom_token::MintCustomTokenResult;
use warp_graphql::object_permissions::OwnerType;
use warp_graphql::queries::api_keys::ApiKeyProperties;
use warp_graphql::queries::get_conversation_usage::ConversationUsage;
use warp_graphql::queries::get_user::UserOutput as GqlUserOutput;

use crate::auth::{
    credentials::{AuthToken, Credentials, FirebaseToken, LoginToken},
    user::User,
};
use crate::server::ids::ApiKeyUid;
use crate::settings::PrivacySettingsSnapshot;

pub const AMBIENT_WORKLOAD_TOKEN_HEADER: &str = "X-Warp-Ambient-Workload-Token";
pub const CLOUD_AGENT_ID_HEADER: &str = "X-Warp-Cloud-Agent-ID";
pub type OAuth2Client = ();

#[derive(Copy, Clone, Debug, Default)]
pub struct SyncedUserSettings {
    pub is_cloud_conversation_storage_enabled: bool,
    pub is_crash_reporting_enabled: bool,
    pub is_telemetry_enabled: bool,
}

pub struct FetchUserResult {
    pub user: User,
    pub credentials: Credentials,
    pub server_experiments: Vec<crate::server::experiments::ServerExperiment>,
    pub from_refresh: bool,
    pub llms: crate::ai::llms::ModelsByFeature,
}

#[derive(Debug, Error)]
pub enum UserAuthenticationError {
    #[error("access token denied")]
    DeniedAccessToken(LocalAuthServiceError),
    #[error("user account disabled")]
    UserAccountDisabled(LocalAuthServiceError),
    #[error(transparent)]
    Unexpected(#[from] anyhow::Error),
    #[error("invalid state parameter")]
    InvalidStateParameter,
    #[error("missing state parameter")]
    MissingStateParameter,
}

impl Clone for UserAuthenticationError {
    fn clone(&self) -> Self {
        match self {
            Self::DeniedAccessToken(err) => Self::DeniedAccessToken(err.clone()),
            Self::UserAccountDisabled(err) => Self::UserAccountDisabled(err.clone()),
            Self::Unexpected(err) => Self::Unexpected(anyhow::anyhow!("{err:#}")),
            Self::InvalidStateParameter => Self::InvalidStateParameter,
            Self::MissingStateParameter => Self::MissingStateParameter,
        }
    }
}

#[derive(Clone, Debug, Error)]
#[error("{message}")]
pub struct LocalAuthServiceError {
    pub message: String,
}

#[derive(Debug, Error)]
pub enum MintCustomTokenError {
    #[error("custom tokens were removed in Warp Lite")]
    Removed,
}

#[derive(Debug, Error)]
pub enum AnonymousUserCreationError {
    #[error("anonymous accounts were removed in Warp Lite")]
    CreationFailed,
    #[error("{0}")]
    UserFacingError(String),
    #[error("unknown anonymous account creation error")]
    Unknown,
}

#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait AuthClient: 'static + Send + Sync {
    async fn create_anonymous_user(
        &self,
        referral_code: Option<String>,
        anonymous_user_type: AnonymousUserType,
    ) -> Result<CreateAnonymousUserResult>;

    async fn get_or_refresh_access_token(&self) -> Result<AuthToken>;

    async fn fetch_user(
        &self,
        token: LoginToken,
        for_refresh: bool,
    ) -> StdResult<FetchUserResult, UserAuthenticationError>;

    async fn fetch_new_custom_token(&self) -> Result<MintCustomTokenResult>;

    fn on_custom_token_fetched(
        &self,
        response: Result<MintCustomTokenResult>,
    ) -> Result<String, MintCustomTokenError>;

    async fn fetch_user_properties<'a>(&self, auth_token: Option<&'a str>)
        -> Result<GqlUserOutput>;

    async fn get_user_settings(&self) -> Result<Option<SyncedUserSettings>>;

    async fn get_conversation_usage_history(
        &self,
        days: Option<i32>,
        limit: Option<i32>,
        last_updated_end_timestamp: Option<warp_graphql::scalars::Time>,
    ) -> Result<Vec<ConversationUsage>>;

    async fn set_is_telemetry_enabled(&self, value: bool) -> Result<()>;

    async fn set_is_crash_reporting_enabled(&self, value: bool) -> Result<()>;

    async fn set_is_cloud_conversation_storage_enabled(&self, value: bool) -> Result<()>;

    async fn update_user_settings(&self, settings_snapshot: PrivacySettingsSnapshot) -> Result<()>;

    async fn set_user_is_onboarded(&self) -> Result<bool>;

    async fn request_device_code(
        &self,
    ) -> StdResult<oauth2::StandardDeviceAuthorizationResponse, UserAuthenticationError>;

    async fn exchange_device_access_token(
        &self,
        details: &oauth2::StandardDeviceAuthorizationResponse,
        timeout: Duration,
    ) -> StdResult<FirebaseToken, UserAuthenticationError>;

    async fn list_api_keys(&self) -> Result<Vec<ApiKeyProperties>>;

    async fn create_api_key(
        &self,
        name: String,
        team_id: Option<cynic::Id>,
        expires_at: Option<warp_graphql::scalars::Time>,
    ) -> Result<GenerateApiKeyResult>;

    async fn expire_api_key(&self, key_uid: &ApiKeyUid) -> Result<ExpireApiKeyResult>;

    async fn get_or_create_ambient_workload_token(&self) -> Result<Option<String>>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl AuthClient for super::ServerApi {
    async fn create_anonymous_user(
        &self,
        _referral_code: Option<String>,
        _anonymous_user_type: AnonymousUserType,
    ) -> Result<CreateAnonymousUserResult> {
        bail!("Warp accounts were removed in Warp Lite")
    }

    async fn get_or_refresh_access_token(&self) -> Result<AuthToken> {
        Ok(AuthToken::local())
    }

    async fn fetch_user(
        &self,
        _token: LoginToken,
        _for_refresh: bool,
    ) -> StdResult<FetchUserResult, UserAuthenticationError> {
        Err(UserAuthenticationError::Unexpected(anyhow::anyhow!(
            "Warp accounts were removed in Warp Lite"
        )))
    }

    async fn fetch_new_custom_token(&self) -> Result<MintCustomTokenResult> {
        bail!("Warp accounts were removed in Warp Lite")
    }

    fn on_custom_token_fetched(
        &self,
        _response: Result<MintCustomTokenResult>,
    ) -> Result<String, MintCustomTokenError> {
        Err(MintCustomTokenError::Removed)
    }

    async fn fetch_user_properties<'a>(
        &self,
        _auth_token: Option<&'a str>,
    ) -> Result<GqlUserOutput> {
        bail!("Warp accounts were removed in Warp Lite")
    }

    async fn get_user_settings(&self) -> Result<Option<SyncedUserSettings>> {
        Ok(None)
    }

    async fn get_conversation_usage_history(
        &self,
        _days: Option<i32>,
        _limit: Option<i32>,
        _last_updated_end_timestamp: Option<warp_graphql::scalars::Time>,
    ) -> Result<Vec<ConversationUsage>> {
        Ok(Vec::new())
    }

    async fn set_is_telemetry_enabled(&self, _value: bool) -> Result<()> {
        Ok(())
    }

    async fn set_is_crash_reporting_enabled(&self, _value: bool) -> Result<()> {
        Ok(())
    }

    async fn set_is_cloud_conversation_storage_enabled(&self, _value: bool) -> Result<()> {
        Ok(())
    }

    async fn update_user_settings(
        &self,
        _settings_snapshot: PrivacySettingsSnapshot,
    ) -> Result<()> {
        Ok(())
    }

    async fn set_user_is_onboarded(&self) -> Result<bool> {
        Ok(true)
    }

    async fn request_device_code(
        &self,
    ) -> StdResult<oauth2::StandardDeviceAuthorizationResponse, UserAuthenticationError> {
        Err(UserAuthenticationError::Unexpected(anyhow::anyhow!(
            "device auth was removed in Warp Lite"
        )))
    }

    async fn exchange_device_access_token(
        &self,
        _details: &oauth2::StandardDeviceAuthorizationResponse,
        _timeout: Duration,
    ) -> StdResult<FirebaseToken, UserAuthenticationError> {
        Err(UserAuthenticationError::Unexpected(anyhow::anyhow!(
            "device auth was removed in Warp Lite"
        )))
    }

    async fn list_api_keys(&self) -> Result<Vec<ApiKeyProperties>> {
        Ok(Vec::new())
    }

    async fn create_api_key(
        &self,
        _name: String,
        _team_id: Option<cynic::Id>,
        _expires_at: Option<warp_graphql::scalars::Time>,
    ) -> Result<GenerateApiKeyResult> {
        bail!("API keys were removed in Warp Lite")
    }

    async fn expire_api_key(&self, _key_uid: &ApiKeyUid) -> Result<ExpireApiKeyResult> {
        bail!("API keys were removed in Warp Lite")
    }

    async fn get_or_create_ambient_workload_token(&self) -> Result<Option<String>> {
        Ok(None)
    }
}
