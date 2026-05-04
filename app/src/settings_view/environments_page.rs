use crate::pane_group::{
    focus_state::PaneFocusHandle,
    pane::{
        view::{HeaderContent, HeaderRenderContext},
        BackingView, PaneConfiguration,
    },
};
use crate::settings_view::{
    settings_page::{PaneEventWrapper, SettingsPageEvent},
    update_environment_form::GithubAuthRedirectTarget,
};
use crate::terminal::view::init_environment::mode_selector::EnvironmentSetupModeSelector;
use warpui::{
    elements::{Element, Text},
    AppContext, Entity, ModelHandle, SingletonEntity, TypedActionView, View, ViewContext,
    ViewHandle,
};

pub struct RemovedAgentAssistedEnvironmentModal;

impl Entity for RemovedAgentAssistedEnvironmentModal {
    type Event = ();
}

impl View for RemovedAgentAssistedEnvironmentModal {
    fn ui_name() -> &'static str {
        "RemovedAgentAssistedEnvironmentModal"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(warpui::elements::Empty::new())
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum EnvironmentsPage {
    #[default]
    List,
    Create,
    Edit {
        env_id: crate::server::ids::SyncId,
    },
}

#[derive(Clone, Debug)]
pub enum EnvironmentsPageAction {
    NoOp,
}

pub struct EnvironmentsPageView {
    current_page: EnvironmentsPage,
    pane_configuration: ModelHandle<PaneConfiguration>,
    focus_handle: Option<PaneFocusHandle>,
}

impl EnvironmentsPageView {
    pub fn new(ctx: &mut ViewContext<Self>) -> Self {
        Self {
            current_page: EnvironmentsPage::List,
            pane_configuration: ctx.add_model(|_| PaneConfiguration::new("Environments")),
            focus_handle: None,
        }
    }

    pub fn update_page(&mut self, page: EnvironmentsPage, ctx: &mut ViewContext<Self>) {
        self.current_page = page;
        ctx.notify();
    }

    pub fn current_page(&self) -> &EnvironmentsPage {
        &self.current_page
    }

    pub fn pane_configuration(&self) -> ModelHandle<PaneConfiguration> {
        self.pane_configuration.clone()
    }

    pub fn focus(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.focus_self();
    }

    pub fn set_github_auth_redirect_target(
        &mut self,
        _target: GithubAuthRedirectTarget,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn environment_setup_mode_selector_handle(
        &self,
    ) -> Option<&ViewHandle<EnvironmentSetupModeSelector>> {
        None
    }

    pub fn agent_assisted_environment_modal_handle(
        &self,
        _app: &AppContext,
    ) -> Option<&ViewHandle<RemovedAgentAssistedEnvironmentModal>> {
        None
    }
}

impl Entity for EnvironmentsPageView {
    type Event = SettingsPageEvent;
}

impl TypedActionView for EnvironmentsPageView {
    type Action = EnvironmentsPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for EnvironmentsPageView {
    fn ui_name() -> &'static str {
        "EnvironmentsPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        let appearance = crate::appearance::Appearance::as_ref(app);
        Text::new(
            "Cloud environments are not available in Warp Lite.",
            appearance.ui_font_family(),
            appearance.ui_font_size(),
        )
        .with_color(appearance.theme().nonactive_ui_text_color().into())
        .finish()
    }
}

impl BackingView for EnvironmentsPageView {
    type PaneHeaderOverflowMenuAction = EnvironmentsPageAction;
    type CustomAction = ();
    type AssociatedData = ();

    fn handle_pane_header_overflow_menu_action(
        &mut self,
        action: &Self::PaneHeaderOverflowMenuAction,
        ctx: &mut ViewContext<Self>,
    ) {
        self.handle_action(action, ctx);
    }

    fn close(&mut self, ctx: &mut ViewContext<Self>) {
        ctx.emit(SettingsPageEvent::Pane(PaneEventWrapper::Close));
    }

    fn focus_contents(&mut self, ctx: &mut ViewContext<Self>) {
        self.focus(ctx);
    }

    fn render_header_content(
        &self,
        _ctx: &HeaderRenderContext<'_>,
        _app: &AppContext,
    ) -> HeaderContent {
        HeaderContent::simple("Environments")
    }

    fn set_focus_handle(&mut self, focus_handle: PaneFocusHandle, _ctx: &mut ViewContext<Self>) {
        self.focus_handle = Some(focus_handle);
    }
}
