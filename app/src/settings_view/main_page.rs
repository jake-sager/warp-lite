use super::{
    settings_page::{
        MatchData, PageType, SettingsPageMeta, SettingsPageViewHandle, SettingsWidget,
    },
    SettingsSection,
};
use crate::appearance::Appearance;
use warpui::{
    elements::{Element, Text},
    AppContext, Entity, TypedActionView, View, ViewContext, ViewHandle,
};

pub fn init_actions_from_parent_view<T>(
    _app: &mut AppContext,
    _context: &warpui::keymap::ContextPredicate,
    _builder: fn(super::SettingsAction) -> T,
) {
}

pub fn handle_experiment_change(_app: &mut AppContext) {}

#[derive(Debug, Clone)]
pub enum MainPageAction {
    NoOp,
}

#[derive(Clone, Copy)]
pub enum MainSettingsPageEvent {}

pub struct MainSettingsPageView {
    page: PageType<Self>,
}

impl Entity for MainSettingsPageView {
    type Event = MainSettingsPageEvent;
}

impl TypedActionView for MainSettingsPageView {
    type Action = MainPageAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

impl View for MainSettingsPageView {
    fn ui_name() -> &'static str {
        "MainSettingsPage"
    }

    fn render(&self, app: &AppContext) -> Box<dyn Element> {
        self.page.render(self, app)
    }
}

impl MainSettingsPageView {
    pub fn new(_ctx: &mut ViewContext<MainSettingsPageView>) -> Self {
        let page = PageType::new_monolith(LocalSettingsWidget, Some("General"), false);
        Self { page }
    }
}

struct LocalSettingsWidget;

impl SettingsWidget for LocalSettingsWidget {
    type View = MainSettingsPageView;

    fn search_terms(&self) -> &str {
        "general local settings terminal configuration settings.toml"
    }

    fn render(
        &self,
        _view: &Self::View,
        appearance: &Appearance,
        _app: &AppContext,
    ) -> Box<dyn Element> {
        Text::new(
            "Warp Lite uses local configuration and terminal history only.",
            appearance.ui_font_family(),
            appearance.ui_font_size(),
        )
        .with_color(appearance.theme().nonactive_ui_text_color().into())
        .finish()
    }
}

impl SettingsPageMeta for MainSettingsPageView {
    fn section() -> SettingsSection {
        SettingsSection::Account
    }

    fn should_render(&self, _ctx: &AppContext) -> bool {
        true
    }

    fn update_filter(&mut self, query: &str, ctx: &mut ViewContext<Self>) -> MatchData {
        self.page.update_filter(query, ctx)
    }

    fn scroll_to_widget(&mut self, widget_id: &'static str) {
        self.page.scroll_to_widget(widget_id)
    }

    fn clear_highlighted_widget(&mut self) {
        self.page.clear_highlighted_widget();
    }
}

impl From<ViewHandle<MainSettingsPageView>> for SettingsPageViewHandle {
    fn from(view_handle: ViewHandle<MainSettingsPageView>) -> Self {
        SettingsPageViewHandle::Main(view_handle)
    }
}
