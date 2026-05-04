use crate::ai::agent::conversation::AIConversationId;
use warpui::elements::{Element, Empty};
use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

pub fn register_conversation_list_view_bindings(_app: &mut AppContext) {}

pub enum Event {
    NewConversationInNewTab,
    ShowDeleteConfirmationDialog {
        conversation_id: AIConversationId,
        conversation_title: String,
        terminal_view_id: Option<warpui::EntityId>,
    },
}

pub struct ConversationListView;

impl ConversationListView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn on_left_panel_focused(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for ConversationListView {
    type Event = Event;
}

impl View for ConversationListView {
    fn ui_name() -> &'static str {
        "ConversationListView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Empty::new().finish()
    }
}

impl TypedActionView for ConversationListView {
    type Action = ();

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
