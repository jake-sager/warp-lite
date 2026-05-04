use std::sync::Arc;

use parking_lot::FairMutex;
use warpui::{
    elements::Empty, AppContext, Entity, EntityId, ModelHandle, TypedActionView, View, ViewContext,
};

use crate::{
    ai::{
        blocklist::{BlocklistAIController, BlocklistAIInputModel},
        execution_profiles::profiles::ClientProfileId,
        llms::LLMId,
    },
    settings_view::SettingsSection,
    terminal::{
        input::MenuPositioningProvider, view::ambient_agent::AmbientAgentViewModel, TerminalModel,
    },
};

pub fn calculate_scaled_font_size(appearance: &warp_core::ui::appearance::Appearance) -> f32 {
    appearance.monospace_font_size()
}

pub fn calculate_max_profile_name_width(
    _appearance: &warp_core::ui::appearance::Appearance,
) -> f32 {
    0.0
}

pub struct ProfileModelSelector {
    is_open: bool,
    _terminal_view_id: EntityId,
}

pub enum ProfileModelSelectorEvent {
    OpenSettings(SettingsSection),
    MenuVisibilityChanged { open: bool },
    ToggleInlineModelSelector,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProfileModelSelectorAction {
    SelectProfile(ClientProfileId),
    SelectModel(LLMId),
    SelectAutoModel,
    SelectReasoningModel(String),
    ManageProfiles,
    ToggleProfileMenu,
    ToggleModelMenu,
}

impl ProfileModelSelectorAction {
    pub fn selected_model_id(&self) -> Option<LLMId> {
        match self {
            Self::SelectModel(id) => Some(id.clone()),
            _ => None,
        }
    }
}

impl ProfileModelSelector {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        _menu_positioning_provider: Arc<dyn MenuPositioningProvider>,
        terminal_view_id: EntityId,
        _input_model: ModelHandle<BlocklistAIInputModel>,
        _ambient_agent_view_model: ModelHandle<AmbientAgentViewModel>,
        _terminal_model: Arc<FairMutex<TerminalModel>>,
        _controller: Option<ModelHandle<BlocklistAIController>>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            is_open: false,
            _terminal_view_id: terminal_view_id,
        }
    }

    pub fn set_profile_menu_visibility(&mut self, is_open: bool, ctx: &mut ViewContext<Self>) {
        self.is_open = is_open;
        ctx.emit(ProfileModelSelectorEvent::MenuVisibilityChanged { open: is_open });
        ctx.notify();
    }

    pub fn set_model_menu_visibility(&mut self, is_open: bool, ctx: &mut ViewContext<Self>) {
        self.is_open = is_open;
        ctx.emit(ProfileModelSelectorEvent::MenuVisibilityChanged { open: is_open });
        ctx.notify();
    }

    pub fn is_open(&self) -> bool {
        self.is_open
    }

    pub fn model_menu_item_position_id(&self, llm_id: &LLMId) -> String {
        format!("profile-model-selector-{llm_id}")
    }

    pub fn set_blurred(&mut self, _is_blurred: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn set_render_compact(&mut self, _render_compact: bool, _ctx: &mut ViewContext<Self>) {}
}

impl TypedActionView for ProfileModelSelector {
    type Action = ProfileModelSelectorAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            ProfileModelSelectorAction::ToggleProfileMenu
            | ProfileModelSelectorAction::ToggleModelMenu => {
                let is_open = !self.is_open;
                self.set_model_menu_visibility(is_open, ctx);
            }
            ProfileModelSelectorAction::ManageProfiles => {
                ctx.emit(ProfileModelSelectorEvent::OpenSettings(SettingsSection::AI));
            }
            _ => {}
        }
    }
}

impl View for ProfileModelSelector {
    fn ui_name() -> &'static str {
        "ProfileModelSelector"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
        Box::new(Empty::new())
    }
}

impl Entity for ProfileModelSelector {
    type Event = ProfileModelSelectorEvent;
}
