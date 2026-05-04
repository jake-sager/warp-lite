use std::collections::HashMap;
use std::sync::Arc;

use crate::drive::items::WarpDriveItemId;
use crate::drive::{CloudObjectTypeAndId, OpenWarpDriveObjectSettings};
use crate::pane_group::focus_state::PaneFocusHandle;
use crate::pane_group::pane::PaneEvent;
use crate::pane_group::PaneConfiguration;
use crate::server::ids::{ClientId, SyncId};
use crate::server::telemetry::SharingDialogSource;
use crate::workflows::{WorkflowSelectionSource, WorkflowSource, WorkflowType, WorkflowViewMode};
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, ModelHandle, TypedActionView, View, ViewContext};

pub mod env_var_selector {
    use warpui::elements::{Element, Empty};
    use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

    #[derive(Clone, Debug)]
    pub enum EnvVarSelectorEvent {
        Changed,
        SelectionChanged(uuid::Uuid),
        Refreshed,
    }

    #[derive(Default)]
    pub struct EnvVarSelector;

    impl EnvVarSelector {
        pub fn new<T>(_ctx: T) -> Self {
            Self
        }

        pub fn set_selected_env_vars<T, U>(&mut self, _vars: T, _ctx: U) {}

        pub fn has_env_vars<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn set_orientation<T, U>(&mut self, _orientation: T, _ctx: U) {}

        pub fn set_width<T, U>(&mut self, _width: T, _ctx: U) {}
    }

    impl Entity for EnvVarSelector {
        type Event = EnvVarSelectorEvent;
    }

    impl View for EnvVarSelector {
        fn ui_name() -> &'static str {
            "EnvVarSelector"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Empty::new().finish()
        }
    }

    impl TypedActionView for EnvVarSelector {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub enum WorkflowViewEvent {
    Pane(PaneEvent),
    CreatedWorkflow(SyncId),
    UpdatedWorkflow(SyncId),
    ViewInWarpDrive(WarpDriveItemId),
    OpenDriveObjectShareDialog {
        cloud_object_type_and_id: CloudObjectTypeAndId,
        invitee_email: Option<String>,
        source: SharingDialogSource,
    },
    RunWorkflow {
        workflow: Arc<WorkflowType>,
        source: WorkflowSource,
        argument_override: Option<HashMap<String, String>>,
    },
}

pub fn init(_app: &mut AppContext) {}

pub struct WorkflowView {
    workflow_id: SyncId,
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
}

impl WorkflowView {
    pub fn new_in_pane(ctx: &mut ViewContext<Self>) -> Self {
        Self {
            workflow_id: SyncId::ClientId(ClientId::default()),
            pane_configuration: ctx.add_model(|_| PaneConfiguration::new("Workflow")),
            focus_handle: None,
        }
    }

    pub fn new_in_suggestion_dialog(ctx: &mut ViewContext<Self>) -> Self {
        Self::new_in_pane(ctx)
    }

    pub fn load<T>(
        &mut self,
        _workflow: T,
        _settings: &OpenWarpDriveObjectSettings,
        _mode: WorkflowViewMode,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn wait_for_initial_load_then_load(
        &mut self,
        workflow_id: SyncId,
        _settings: &OpenWarpDriveObjectSettings,
        _mode: WorkflowViewMode,
        _window_id: warpui::WindowId,
        _ctx: &mut ViewContext<Self>,
    ) {
        self.workflow_id = workflow_id;
    }

    #[allow(clippy::too_many_arguments)]
    pub fn open_new_workflow<T, U>(
        &mut self,
        _title: Option<String>,
        _content: Option<String>,
        _owner: T,
        _initial_folder_id: Option<SyncId>,
        _is_for_agent_mode: bool,
        workflow_id: SyncId,
        _ctx: U,
    ) {
        self.workflow_id = workflow_id;
    }

    pub fn workflow_id(&self) -> SyncId {
        self.workflow_id
    }

    pub fn workflow_link(&self, _ctx: &AppContext) -> Option<String> {
        None
    }

    pub fn pane_configuration(&self) -> &ModelHandle<PaneConfiguration> {
        &self.pane_configuration
    }

    pub fn set_focus_handle(
        &mut self,
        focus_handle: PaneFocusHandle,
        _ctx: &mut ViewContext<Self>,
    ) {
        self.focus_handle = Some(focus_handle);
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    pub fn is_agent_mode_workflow(&self) -> bool {
        false
    }
}

impl Entity for WorkflowView {
    type Event = WorkflowViewEvent;
}

impl View for WorkflowView {
    fn ui_name() -> &'static str {
        "WorkflowView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for WorkflowView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl crate::pane_group::pane::BackingView for WorkflowView {
    type PaneHeaderOverflowMenuAction = ();
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        _action: &Self::PaneHeaderOverflowMenuAction,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    fn close(&mut self, _ctx: &mut ViewContext<Self>) {}

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &crate::pane_group::pane::view::HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> crate::pane_group::pane::view::HeaderContent {
        crate::pane_group::pane::view::HeaderContent::simple("Workflow")
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, ctx: &mut ViewContext<Self>) {
        self.set_focus_handle(focus_handle, ctx);
    }
}
