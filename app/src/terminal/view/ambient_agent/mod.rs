use crate::{
    ai::{agent_tips::AITip, llms::LLMId},
    pane_group::TerminalViewResources,
    terminal::{TerminalManager, TerminalView},
};
use warp_core::{ui::icons::Icon, SessionId};
use warpui::{
    elements::Empty, geometry::vector::Vector2F, AppContext, Entity, ModelHandle, TypedActionView,
    View, ViewContext, ViewHandle, WindowId,
};

pub fn create_cloud_mode_view(
    _resources: TerminalViewResources,
    _view_bounds_size: Vector2F,
    _window_id: WindowId,
    _ctx: &mut AppContext,
) -> (
    ViewHandle<TerminalView>,
    ModelHandle<Box<dyn TerminalManager>>,
) {
    unreachable!("cloud environments are removed in Warp Lite")
}

pub fn is_cloud_agent_pre_first_exchange(
    _ambient_agent_view_model: &ModelHandle<AmbientAgentViewModel>,
    _agent_view_controller: &ModelHandle<crate::ai::blocklist::agent_view::AgentViewController>,
    _app: &AppContext,
) -> bool {
    false
}

#[derive(Clone, Debug, Default)]
pub struct AmbientAgentViewModel {
    pub ui_state: AmbientAgentProgressUIState,
}

#[derive(Clone, Debug)]
pub enum AmbientAgentViewModelEvent {
    SessionReady { session_id: SessionId },
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Status {
    #[default]
    Idle,
    AgentRunning,
    Error,
}

impl AmbientAgentViewModel {
    pub fn new<T, U, V>(_terminal_view_id: T, _has_parent: U, _ctx: V) -> Self {
        Self::default()
    }

    pub fn status(&self) -> Status {
        Status::Idle
    }

    pub fn is_ambient_agent(&self) -> bool {
        false
    }

    pub fn is_configuring_ambient_agent(&self) -> bool {
        false
    }

    pub fn is_waiting_for_session(&self) -> bool {
        false
    }

    pub fn is_agent_running(&self) -> bool {
        false
    }

    pub fn should_show_status_footer(&self) -> bool {
        false
    }

    pub fn error_message(&self) -> Option<&str> {
        None
    }

    pub fn task_id(&self) -> Option<crate::ai::ambient_agents::AmbientAgentTaskId> {
        None
    }

    pub fn has_parent_terminal(&self) -> bool {
        false
    }

    pub fn is_in_setup(&self) -> bool {
        false
    }

    pub fn harness_command_started(&self) -> bool {
        false
    }

    pub fn progress(&self) -> AgentProgress {
        AgentProgress::default()
    }

    pub fn set_model_id(&mut self, _model_id: LLMId, _ctx: &mut warpui::ModelContext<Self>) {}

    pub fn set_harness<T>(&mut self, _harness: T, _ctx: &mut warpui::ModelContext<Self>) {}

    pub fn spawn_agent<T, U>(
        &mut self,
        _prompt: T,
        _attachments: U,
        _ctx: &mut warpui::ModelContext<Self>,
    ) {
    }

    pub fn enter_viewing_existing_session<T>(
        &mut self,
        _task_id: T,
        _ctx: &mut warpui::ModelContext<Self>,
    ) {
    }

    pub fn cancel_task<T>(&mut self, _ctx: T) {}

    pub fn agent_progress(&self) -> Option<AgentProgress> {
        None
    }
}

impl Entity for AmbientAgentViewModel {
    type Event = AmbientAgentViewModelEvent;
}

#[derive(Clone, Debug, Default)]
pub struct AgentProgress {
    pub harness_started_at: Option<std::time::Instant>,
    pub claimed_at: Option<std::time::Instant>,
}

#[derive(Clone, Debug, Default)]
pub struct AmbientAgentProgressUIState {
    pub error_selected_text: std::sync::Arc<std::sync::RwLock<Option<String>>>,
}

#[derive(Clone, Debug, Default)]
pub struct ProgressProps;

#[derive(Clone, Debug, Default)]
pub struct ProgressStep;

#[derive(Clone, Debug, Default)]
pub enum ProgressStepState {
    #[default]
    Pending,
    Active,
    Complete,
}

pub fn render_progress<T>(_props: T, _app: &AppContext) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

pub fn render_error_footer<T, U>(_message: T, _app: U) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

pub fn render_loading_footer<T>(_app: T) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

pub fn render_cloud_mode_error_screen<T, U, V, W, X>(
    _message: T,
    _appearance: U,
    _selection_handle: V,
    _selected_text: W,
    _app: X,
) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

impl TerminalView {
    pub fn render_ambient_agent_progress<T>(
        &self,
        _progress: T,
        _app: &AppContext,
    ) -> Box<dyn warpui::Element> {
        Box::new(Empty::new())
    }
}

pub fn render_cloud_mode_loading_screen<T, U, V, W, X>(
    _message: T,
    _appearance: U,
    _shimmer_handle: V,
    _tip_model: W,
    _app: X,
) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

pub fn render_cloud_mode_cancelled_screen<T, U, V, W>(
    _appearance: T,
    _selection_handle: U,
    _selected_text: V,
    _app: W,
) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

pub fn render_cloud_mode_github_auth_required_screen<T, U, V>(
    _appearance: T,
    _mouse_state: U,
    _app: V,
) -> Box<dyn warpui::Element> {
    Box::new(Empty::new())
}

#[derive(Clone, Debug, Default)]
pub struct FirstTimeCloudAgentSetupView;

#[derive(Clone, Debug)]
pub enum FirstTimeCloudAgentSetupViewEvent {}

impl FirstTimeCloudAgentSetupView {
    pub fn new<T>(_ctx: T) -> Self {
        Self
    }
}

impl View for FirstTimeCloudAgentSetupView {
    fn ui_name() -> &'static str {
        "FirstTimeCloudAgentSetupView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
        Box::new(Empty::new())
    }
}

impl Entity for FirstTimeCloudAgentSetupView {
    type Event = FirstTimeCloudAgentSetupViewEvent;
}

impl TypedActionView for FirstTimeCloudAgentSetupView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

macro_rules! selector_view {
    ($name:ident, $action:ident, $event:ident) => {
        #[derive(Clone, Debug, Default)]
        pub struct $name;

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum $action {
            ToggleMenu,
        }

        #[derive(Clone, Debug)]
        pub enum $event {}

        impl $name {
            pub fn new<T, U>(_arg: T, _maybe_model: U, _ctx: &mut ViewContext<Self>) -> Self {
                Self
            }

            pub fn set_button_theme<T>(&mut self, _theme: T, _ctx: &mut ViewContext<Self>) {}
        }

        impl TypedActionView for $name {
            type Action = $action;

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl View for $name {
            fn ui_name() -> &'static str {
                stringify!($name)
            }

            fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                Box::new(Empty::new())
            }
        }

        impl Entity for $name {
            type Event = $event;
        }
    };
}

selector_view!(HarnessSelector, HarnessSelectorAction, HarnessSelectorEvent);
selector_view!(HostSelector, HostSelectorAction, HostSelectorEvent);
selector_view!(ModelSelector, ModelSelectorAction, ModelSelectorEvent);

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Host {
    pub id: String,
    pub name: String,
}

#[derive(Clone, Debug, Default)]
pub struct NakedHeaderButtonTheme;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CloudModeTip {
    title: String,
    body: String,
    icon: Icon,
}

impl Default for CloudModeTip {
    fn default() -> Self {
        Self {
            title: String::new(),
            body: String::new(),
            icon: Icon::Info,
        }
    }
}

impl CloudModeTip {
    pub fn new(title: impl Into<String>, body: impl Into<String>, icon: Icon) -> Self {
        Self {
            title: title.into(),
            body: body.into(),
            icon,
        }
    }
}

impl AITip for CloudModeTip {
    fn keystroke(&self, _app: &AppContext) -> Option<warpui::keymap::Keystroke> {
        None
    }

    fn link(&self) -> Option<String> {
        None
    }

    fn description(&self) -> &str {
        &self.body
    }
}

pub fn get_cloud_mode_tips() -> Vec<CloudModeTip> {
    Vec::new()
}

#[derive(Clone, Debug, Default)]
pub struct AmbientAgentEntryBlock;

#[derive(Clone, Debug)]
pub enum AmbientAgentEntryBlockAction {
    OpenAmbientAgent,
}

impl AmbientAgentEntryBlock {
    pub fn new<T, U, V>(
        _terminal_view: T,
        _terminal_manager: U,
        _pane_stack: V,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self
    }
}

impl TypedActionView for AmbientAgentEntryBlock {
    type Action = AmbientAgentEntryBlockAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for AmbientAgentEntryBlock {
    fn ui_name() -> &'static str {
        "AmbientAgentEntryBlock"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
        Box::new(Empty::new())
    }
}

impl Entity for AmbientAgentEntryBlock {
    type Event = ();
}
