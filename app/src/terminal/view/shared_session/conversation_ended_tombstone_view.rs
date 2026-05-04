use crate::ai::{agent::conversation::AIConversationId, ambient_agents::AmbientAgentTaskId};
use warpui::{
    elements::Empty, AppContext, Element, Entity, EntityId, TypedActionView, View, ViewContext,
};

pub struct ConversationEndedTombstoneView;

#[derive(Clone, Debug)]
pub enum ConversationEndedTombstoneAction {
    #[cfg(not(target_family = "wasm"))]
    ContinueLocally(AIConversationId),
    #[cfg(target_family = "wasm")]
    OpenInWarp(AIConversationId),
}

impl ConversationEndedTombstoneView {
    pub fn new(
        _ctx: &mut ViewContext<Self>,
        _terminal_view_id: EntityId,
        _task_id: Option<AmbientAgentTaskId>,
    ) -> Self {
        Self
    }
}

impl Entity for ConversationEndedTombstoneView {
    type Event = ();
}

impl View for ConversationEndedTombstoneView {
    fn ui_name() -> &'static str {
        "ConversationEndedTombstoneView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

impl TypedActionView for ConversationEndedTombstoneView {
    type Action = ConversationEndedTombstoneAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}
