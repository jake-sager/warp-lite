//! Local-only authentication tombstone for Warp Lite.

use std::sync::Arc;

use serde::{Deserialize, Serialize};
use url::Url;
use warpui::{AppContext, Entity, ModelContext, SingletonEntity, ViewContext};

pub const API_KEY_PREFIX: &str = "wk-";

pub use warp_server_client::UserUid;

pub mod user_uid {
    pub use super::UserUid;
}

pub mod user {
    use super::UserUid;

    pub const TEST_USER_UID: &str = "test_user_uid";

    #[derive(Clone, Debug, Default)]
    pub struct User {
        pub uid: UserUid,
        pub email: Option<String>,
        pub is_anonymous: bool,
        pub is_onboarded: bool,
    }

    #[derive(Clone, Debug, Default)]
    pub struct FirebaseAuthTokens;

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum PrincipalType {
        User,
        Anonymous,
    }
}

pub mod anonymous_id {
    pub fn get_or_create_anonymous_id() -> String {
        "warp-lite-local".to_owned()
    }
}

pub mod credentials {
    #[derive(Clone, Debug, Default)]
    pub struct FirebaseToken(pub String);

    #[derive(Clone, Debug, Default)]
    pub struct RefreshToken(pub String);

    #[derive(Clone, Debug, Default)]
    pub struct AuthToken {
        bearer: Option<String>,
    }

    impl AuthToken {
        pub fn local() -> Self {
            Self { bearer: None }
        }

        pub fn bearer_token(&self) -> Option<String> {
            self.bearer.clone()
        }

        pub fn as_bearer_token(&self) -> Option<&str> {
            self.bearer.as_deref()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Credentials {
        pub auth_token: AuthToken,
        pub refresh_token: Option<RefreshToken>,
    }

    #[derive(Clone, Debug)]
    pub enum LoginToken {
        Firebase(FirebaseToken),
        SessionCookie,
        Custom(String),
    }
}

#[derive(Clone, Debug)]
pub struct AuthState {
    anonymous_id: String,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct PersonalObjectLimits {
    pub workflow_limit: usize,
    pub notebook_limit: usize,
    pub env_var_limit: usize,
}

impl AuthState {
    pub fn initialize(_ctx: &mut AppContext, _api_key: Option<String>) -> Self {
        Self {
            anonymous_id: anonymous_id::get_or_create_anonymous_id(),
        }
    }

    pub fn is_logged_in(&self) -> bool {
        false
    }

    pub fn is_anonymous_or_logged_out(&self) -> bool {
        true
    }

    pub fn is_user_anonymous(&self) -> Option<bool> {
        Some(true)
    }

    pub fn is_onboarded(&self) -> Option<bool> {
        Some(true)
    }

    pub fn needs_sso_link(&self) -> Option<bool> {
        Some(false)
    }

    pub fn needs_reauth(&self) -> bool {
        false
    }

    pub fn user_id(&self) -> Option<UserUid> {
        None
    }

    pub fn anonymous_id(&self) -> String {
        self.anonymous_id.clone()
    }

    pub fn user_email(&self) -> Option<String> {
        None
    }

    pub fn username_for_display(&self) -> Option<String> {
        None
    }

    pub fn user_photo_url(&self) -> Option<String> {
        None
    }

    pub fn is_user_web_anonymous_user(&self) -> Option<bool> {
        Some(false)
    }

    pub fn get_access_token_ignoring_validity(&self) -> Option<String> {
        None
    }

    pub fn is_anonymous_user_past_object_limit<T>(
        &self,
        _object_type: T,
        _object_count: usize,
    ) -> Option<bool> {
        Some(false)
    }

    pub fn personal_object_limits(&self) -> Option<PersonalObjectLimits> {
        None
    }
}

impl warp_managed_secrets::ActorProvider for AuthState {
    fn actor_uid(&self) -> Option<String> {
        None
    }
}

pub mod auth_state {
    use super::{Arc, Entity, SingletonEntity};

    pub use super::AuthState;

    pub struct AuthStateProvider {
        state: Arc<AuthState>,
    }

    impl AuthStateProvider {
        pub fn new(state: Arc<AuthState>) -> Self {
            Self { state }
        }

        pub fn get(&self) -> Arc<AuthState> {
            self.state.clone()
        }
    }

    impl Entity for AuthStateProvider {
        type Event = ();
    }

    impl SingletonEntity for AuthStateProvider {}
}

pub use auth_state::AuthStateProvider;

pub mod auth_view_modal {
    use serde::{Deserialize, Serialize};
    use url::Url;
    use warpui::{AppContext, Entity, ViewContext};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AuthViewVariant {
        Initial,
        RequireLoginCloseable,
        ShareRequirementCloseable,
    }

    #[derive(Clone, Debug)]
    pub struct AuthRedirectPayload;

    impl AuthRedirectPayload {
        pub fn from_url(_url: Url) -> Result<Self, anyhow::Error> {
            Err(anyhow::anyhow!("auth redirects are disabled in Warp Lite"))
        }
    }

    #[derive(Clone, Debug)]
    pub enum AuthViewEvent {
        Close,
    }

    pub struct AuthView {
        variant: AuthViewVariant,
        pub last_login_failure_reason: Option<super::LoginFailureReason>,
    }

    impl AuthView {
        pub fn new(variant: AuthViewVariant, _ctx: &mut ViewContext<Self>) -> Self {
            Self {
                variant,
                last_login_failure_reason: None,
            }
        }

        pub fn set_variant(&mut self, _ctx: &mut ViewContext<Self>, variant: AuthViewVariant) {
            self.variant = variant;
        }

        pub fn skip_to_browser_open_step(&mut self, _ctx: &mut ViewContext<Self>) {}
    }

    impl Entity for AuthView {
        type Event = AuthViewEvent;
    }

    impl warpui::View for AuthView {
        fn ui_name() -> &'static str {
            "AuthView"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for AuthView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    pub fn init(_app: &mut AppContext) {}
}

pub use auth_view_modal::{AuthRedirectPayload, AuthViewVariant};

pub mod auth_manager {
    use super::{AuthRedirectPayload, AuthViewVariant, Entity, ModelContext, SingletonEntity, Url};
    use crate::server::server_api::auth::UserAuthenticationError;

    #[derive(Clone, Debug)]
    pub enum AuthManagerEvent {
        AuthComplete,
        SkippedLogin,
        AuthFailed(UserAuthenticationError),
        LoginOverrideDetected(AuthRedirectPayload),
        AttemptedLoginGatedFeature { auth_view_variant: AuthViewVariant },
    }

    pub type LoginGatedFeature = &'static str;

    #[derive(Clone, Debug, Default)]
    pub struct PersistedCurrentUserInformation {
        pub email: String,
    }

    pub struct AuthManager;

    impl AuthManager {
        pub fn new<T, U>(_server_api: T, _auth_client: U, _ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn refresh_user(&mut self, _ctx: &mut ModelContext<Self>) {}

        pub fn log_out(&mut self, ctx: &mut ModelContext<Self>) {
            ctx.emit(AuthManagerEvent::SkippedLogin);
        }

        pub fn set_user_onboarded(&mut self, _ctx: &mut ModelContext<Self>) {}

        pub fn set_needs_reauth(&mut self, _needs_reauth: bool, _ctx: &mut ModelContext<Self>) {}

        pub fn anonymous_user_hit_drive_object_limit(&mut self, _ctx: &mut ModelContext<Self>) {}

        pub fn initiate_anonymous_user_linking(
            &mut self,
            _entrypoint: impl std::fmt::Debug,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn attempt_login_gated_feature(
            &mut self,
            _feature: impl std::fmt::Debug,
            auth_view_variant: AuthViewVariant,
            ctx: &mut ModelContext<Self>,
        ) {
            ctx.emit(AuthManagerEvent::AttemptedLoginGatedFeature { auth_view_variant });
        }

        pub fn initialize_user_from_auth_payload(
            &mut self,
            _payload: AuthRedirectPayload,
            _from_redirect: bool,
            ctx: &mut ModelContext<Self>,
        ) {
            ctx.emit(AuthManagerEvent::SkippedLogin);
        }

        pub fn sign_in_url(&self) -> String {
            "http://127.0.0.1/warp-lite/sign-in-disabled".to_string()
        }

        pub fn sign_up_url(&self) -> String {
            "http://127.0.0.1/warp-lite/sign-up-disabled".to_string()
        }

        pub fn upgrade_url(&self) -> String {
            "http://127.0.0.1/warp-lite/upgrade-disabled".to_string()
        }

        pub fn open_url_maybe_with_anonymous_token<T, U>(&mut self, _url: T, _ctx: U) {}
    }

    impl Entity for AuthManager {
        type Event = AuthManagerEvent;
    }

    impl SingletonEntity for AuthManager {}
}

pub use auth_manager::AuthManager;

#[derive(Clone, Debug)]
pub enum LoginFailureReason {
    InvalidRedirectUrl { was_pasted: bool },
    Other,
}

pub mod login_failure_notification {
    pub use super::LoginFailureReason;
}

pub mod auth_override_warning_modal {
    use super::{AuthRedirectPayload, Entity, ViewContext};

    #[derive(Clone, Copy, Debug)]
    pub enum AuthOverrideWarningModalVariant {
        OnboardingView,
        WorkspaceModal,
    }

    #[derive(Clone, Debug)]
    pub enum AuthOverrideWarningModalEvent {
        Close,
        BulkExport,
    }

    pub struct AuthOverrideWarningModal {
        _variant: AuthOverrideWarningModalVariant,
        _payload: Option<AuthRedirectPayload>,
    }

    impl AuthOverrideWarningModal {
        pub fn new(_ctx: &mut ViewContext<Self>, variant: AuthOverrideWarningModalVariant) -> Self {
            Self {
                _variant: variant,
                _payload: None,
            }
        }

        pub fn set_interrupted_auth_payload(&mut self, payload: AuthRedirectPayload) {
            self._payload = Some(payload);
        }
    }

    impl Entity for AuthOverrideWarningModal {
        type Event = AuthOverrideWarningModalEvent;
    }

    impl warpui::View for AuthOverrideWarningModal {
        fn ui_name() -> &'static str {
            "AuthOverrideWarningModal"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for AuthOverrideWarningModal {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub mod login_slide {
    use super::{Entity, LoginFailureReason, ViewContext};

    #[derive(Clone, Copy, Debug)]
    pub enum LoginSlideSource {
        OnboardingFlow,
        PrivacySettingsFromTerminalIntentionTheme,
        LoginExistingUserFromWelcome,
    }

    #[derive(Clone, Debug)]
    pub enum LoginSlideEvent {
        BackToOnboarding,
        LoginLaterConfirmed,
    }

    pub struct LoginSlideView {
        _source: LoginSlideSource,
    }

    impl LoginSlideView {
        pub fn new<T>(
            _ai_enabled: bool,
            _theme_name: T,
            _use_vertical_tabs: bool,
            _intention: crate::onboarding::OnboardingIntention,
            source: LoginSlideSource,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self { _source: source }
        }

        pub fn is_auth_token_input_visible(&self) -> bool {
            false
        }
    }

    impl Entity for LoginSlideView {
        type Event = LoginSlideEvent;
    }

    impl warpui::View for LoginSlideView {
        fn ui_name() -> &'static str {
            "LoginSlideView"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for LoginSlideView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub mod needs_sso_link_view {
    use super::{Entity, ViewContext};

    pub struct NeedsSsoLinkView;

    impl NeedsSsoLinkView {
        pub fn new() -> Self {
            Self
        }

        pub fn set_email<T>(&mut self, _email: T) {}
    }

    impl Entity for NeedsSsoLinkView {
        type Event = ();
    }

    impl warpui::View for NeedsSsoLinkView {
        fn ui_name() -> &'static str {
            "NeedsSsoLinkView"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for NeedsSsoLinkView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub mod paste_auth_token_modal {
    use super::{Entity, ViewContext};

    #[derive(Clone, Debug)]
    pub enum PasteAuthTokenModalEvent {
        Cancelled,
    }

    pub struct PasteAuthTokenModalView;

    impl PasteAuthTokenModalView {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for PasteAuthTokenModalView {
        type Event = PasteAuthTokenModalEvent;
    }

    impl warpui::View for PasteAuthTokenModalView {
        fn ui_name() -> &'static str {
            "PasteAuthTokenModalView"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for PasteAuthTokenModalView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub mod web_handoff {
    use super::{Entity, ViewContext};

    #[derive(Clone, Debug)]
    pub enum WebHandoffEvent {
        Unsupported,
    }

    pub struct WebHandoffView;

    impl WebHandoffView {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for WebHandoffView {
        type Event = WebHandoffEvent;
    }
}

pub fn init(_app: &mut AppContext) {}

pub fn maybe_log_out(app: &mut AppContext) {
    log_out(app);
}

pub fn log_out(_app: &mut AppContext) {}
