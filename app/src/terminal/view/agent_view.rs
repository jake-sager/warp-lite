use warpui::ViewContext;

use crate::{
    ai::{
        agent::conversation::AIConversationId,
        blocklist::agent_view::{
            AgentViewEntryBlockParams, AgentViewEntryOrigin, EnterAgentViewError,
        },
    },
    terminal::{view::RichContentInsertionPosition, TerminalView},
};

pub const ENTER_AGAIN_TO_SEND_MESSAGE_ID: &str = "enter_again_to_send";

impl TerminalView {
    pub fn enter_agent_view(
        &mut self,
        _initial_prompt: Option<String>,
        _conversation_id: Option<AIConversationId>,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn enter_agent_view_for_new_conversation(
        &mut self,
        _initial_prompt: Option<String>,
        _origin: AgentViewEntryOrigin,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn enter_agent_view_for_conversation(
        &mut self,
        _initial_prompt: Option<String>,
        _origin: AgentViewEntryOrigin,
        _conversation_id: AIConversationId,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn enter_cloud_agent_view<T>(&mut self, _initial_prompt: T, _ctx: &mut ViewContext<Self>) {}

    pub(super) fn try_enter_agent_view(
        &mut self,
        _initial_prompt: Option<String>,
        _origin: AgentViewEntryOrigin,
        conversation_id: Option<AIConversationId>,
        _ctx: &mut ViewContext<Self>,
    ) -> Result<AIConversationId, EnterAgentViewError> {
        Ok(conversation_id.unwrap_or_else(AIConversationId::new))
    }

    pub(super) fn insert_agent_view_entry_block(
        &mut self,
        _params: AgentViewEntryBlockParams,
        _position: RichContentInsertionPosition,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn set_rich_content_agent_view_conversation_id(
        &mut self,
        _rich_content_view_id: warpui::EntityId,
        _conversation_id: AIConversationId,
    ) {
    }

    pub(super) fn maybe_auto_open_cloud_mode_details_panel(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn fetch_and_update_cloud_mode_details_panel(
        &mut self,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn handle_ambient_agent_event<T>(
        &mut self,
        _event: T,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn handle_first_time_cloud_agent_setup_event<T>(
        &mut self,
        _event: T,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn maybe_insert_setup_command_blocks<T>(
        &mut self,
        _block_id: T,
        _ctx: &mut ViewContext<Self>,
    ) {
    }
}
