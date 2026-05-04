use crate::{ai::cloud_environments::AmbientAgentEnvironment, server::ids::SyncId};
use warpui::{
    elements::{Element, Empty},
    AppContext, Entity, TypedActionView, View, ViewContext,
};

pub fn init(_ctx: &mut AppContext) {}

#[derive(Clone, Debug, Default)]
pub struct EnvironmentFormValues;

#[derive(Clone, Debug)]
pub enum EnvironmentFormInitArgs {
    Create,
    Edit {
        env_id: SyncId,
        initial_values: Box<EnvironmentFormValues>,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GithubAuthRedirectTarget {
    SettingsEnvironments,
    FocusCloudMode,
}

#[derive(Debug, Clone)]
pub enum UpdateEnvironmentFormEvent {
    Created {
        environment: AmbientAgentEnvironment,
        share_with_team: bool,
    },
    Updated {
        env_id: SyncId,
        environment: AmbientAgentEnvironment,
    },
    DeleteRequested {
        env_id: SyncId,
    },
    Cancelled,
}

#[derive(Clone, Debug)]
pub enum UpdateEnvironmentFormAction {
    NoOp,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum AuthSource {
    #[default]
    Settings,
    CloudSetup,
}

pub struct UpdateEnvironmentForm {
    mode: EnvironmentFormInitArgs,
}

impl UpdateEnvironmentForm {
    pub fn new(init_args: EnvironmentFormInitArgs, _ctx: &mut ViewContext<Self>) -> Self {
        Self { mode: init_args }
    }

    pub fn set_mode(&mut self, init_args: EnvironmentFormInitArgs, ctx: &mut ViewContext<Self>) {
        self.mode = init_args;
        ctx.notify();
    }

    pub fn set_github_auth_redirect_target(&mut self, _target: GithubAuthRedirectTarget) {}

    pub fn set_show_header(&mut self, _show_header: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn set_should_handle_escape_from_editor(&mut self, _should_handle: bool) {}

    pub fn set_auth_source(&mut self, _source: AuthSource) {}

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }
}

impl Entity for UpdateEnvironmentForm {
    type Event = UpdateEnvironmentFormEvent;
}

impl TypedActionView for UpdateEnvironmentForm {
    type Action = UpdateEnvironmentFormAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for UpdateEnvironmentForm {
    fn ui_name() -> &'static str {
        "UpdateEnvironmentForm"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}
