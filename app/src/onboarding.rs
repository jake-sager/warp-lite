//! Local-only onboarding tombstone for Warp Lite.

use warpui::{elements::Empty, AppContext, Element, Entity, TypedActionView, View, ViewContext};

pub fn init(_ctx: &mut AppContext) {}

#[derive(Clone, Debug)]
pub enum SelectedSettings {
    AgentDrivenDevelopment {
        agent_settings: slides::AgentDevelopmentSettings,
        project_settings: ProjectOnboardingSettings,
        ui_customization: Option<UICustomizationSettings>,
    },
    Terminal {
        ui_customization: Option<UICustomizationSettings>,
        cli_agent_toolbar_enabled: bool,
        show_agent_notifications: bool,
    },
}

impl SelectedSettings {
    pub fn is_ai_enabled(&self) -> bool {
        matches!(self, Self::AgentDrivenDevelopment { agent_settings, .. } if !agent_settings.disable_oz)
    }

    pub fn is_warp_drive_enabled(&self) -> bool {
        match self {
            Self::AgentDrivenDevelopment {
                ui_customization, ..
            }
            | Self::Terminal {
                ui_customization, ..
            } => ui_customization
                .as_ref()
                .is_some_and(|settings| settings.show_warp_drive),
        }
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub enum SessionDefault {
    Agent,
    #[default]
    Terminal,
}

#[derive(Clone, Debug, Default)]
pub struct UICustomizationSettings {
    pub use_vertical_tabs: bool,
    pub show_code_review_button: bool,
    pub show_warp_drive: bool,
    pub show_project_explorer: bool,
    pub show_global_search: bool,
    pub show_conversation_history: bool,
}

#[derive(Clone, Debug, Default)]
pub enum ProjectOnboardingSettings {
    Project {
        selected_local_folder: String,
        initialize_projects_automatically: bool,
    },
    #[default]
    NoProject,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum OnboardingIntention {
    #[default]
    General,
    AgentDrivenDevelopment,
    Terminal,
}

#[derive(Clone, Debug)]
pub enum AgentOnboardingEvent {
    Completed,
    ThemeSelected { theme_name: String },
    SyncWithOsToggled { enabled: bool },
    OnboardingCompleted(SelectedSettings),
    OnboardingSkipped,
    UpgradeRequested,
    UpgradeCopyUrlRequested,
    UpgradePasteTokenFromClipboardRequested,
    PrivacySettingsFromTerminalThemeSlideRequested,
    LoginFromWelcomeRequested,
    AppBecameActive,
}

pub struct AgentOnboardingView;

impl AgentOnboardingView {
    pub fn new<T, U, V, W>(
        _themes: T,
        _unskippable: bool,
        _models: U,
        _default_model_id: V,
        _workspace_enforces_autonomy: bool,
        _agent_view_enabled: bool,
        _free_user_no_ai_experiment_active: bool,
        _agent_price_cents: Option<i32>,
        _auth_state: W,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn set_agent_price_cents(&mut self, _cents: Option<i32>, _ctx: &mut ViewContext<Self>) {}

    pub fn set_onboarding_models<T, U>(
        &mut self,
        _models: T,
        _default_model_id: U,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn start_onboarding(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_workspace_enforces_autonomy(&mut self, _value: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn free_user_no_ai_experiment(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn set_free_user_no_ai_experiment(&mut self, _value: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn advance_to_agent_step(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn set_auth_state<T>(&mut self, _auth_state: T, _ctx: &mut ViewContext<Self>) {}

    pub fn use_vertical_tabs(&self, _ctx: &AppContext) -> bool {
        false
    }
}

impl Entity for AgentOnboardingView {
    type Event = AgentOnboardingEvent;
}

impl View for AgentOnboardingView {
    fn ui_name() -> &'static str {
        "AgentOnboardingView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

impl TypedActionView for AgentOnboardingView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub mod slides {
    #[derive(Clone, Copy, Debug)]
    pub enum AgentAutonomy {
        Full,
        Partial,
        None,
    }

    impl Default for AgentAutonomy {
        fn default() -> Self {
            Self::None
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct AgentDevelopmentSettings {
        pub session_default: super::SessionDefault,
        pub cli_agent_toolbar_enabled: bool,
        pub show_agent_notifications: bool,
        pub selected_model_id: String,
        pub autonomy: Option<AgentAutonomy>,
        pub disable_oz: bool,
    }
}

#[derive(Clone, Debug, Default)]
pub struct OnboardingKeybindings {
    pub toggle_input_mode: String,
    pub submit_to_local_agent: String,
    pub submit_to_cloud_agent: String,
}

pub mod callout {
    #[derive(Clone, Debug)]
    pub enum FinalState {
        Submit,
        Initialize,
        Skip,
        Finish,
        BackToTerminal,
    }

    #[derive(Clone, Debug)]
    pub enum OnboardingQuery {
        None,
        TerminalCommand(String),
        AgentPrompt(String),
    }

    #[derive(Clone, Debug)]
    pub enum OnboardingCalloutViewEvent {
        Completed { final_state: FinalState },
        StateUpdated,
        EnterAgentModality,
        NaturalLanguageDetectionToggled(bool),
    }
}

pub use callout::{FinalState, OnboardingCalloutViewEvent, OnboardingQuery};

pub struct OnboardingCalloutView;

impl OnboardingCalloutView {
    pub fn new_universal_input(
        _has_project: bool,
        _natural_language_detection_enabled: bool,
        _keybindings: OnboardingKeybindings,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn new_agent_modality<T>(
        _has_project: bool,
        _intention: T,
        _natural_language_detection_enabled: bool,
        _keybindings: OnboardingKeybindings,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }

    pub fn start_onboarding(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn prompt_string(&self, _ctx: &AppContext) -> String {
        String::new()
    }

    pub fn prompt(&self, _ctx: &AppContext) -> OnboardingQuery {
        OnboardingQuery::None
    }

    pub fn is_onboarding_active(&self, _ctx: &AppContext) -> bool {
        false
    }

    pub fn should_position_above_zero_state(&self, _ctx: &AppContext) -> bool {
        false
    }
}

impl Entity for OnboardingCalloutView {
    type Event = OnboardingCalloutViewEvent;
}

impl View for OnboardingCalloutView {
    fn ui_name() -> &'static str {
        "OnboardingCalloutView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

impl TypedActionView for OnboardingCalloutView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
