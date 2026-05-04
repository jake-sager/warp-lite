//! Local-only tombstones for removed agent conversation restoration.
//!
//! Warp Lite keeps local terminal session restoration, but account-backed AI
//! conversation downloads, protobuf debug links, cloud transcripts, and CLI
//! agent replay are intentionally removed.

use vec1::Vec1;
use warpui::ViewContext;

use crate::ai::agent::conversation::{AIConversation, AIConversationId};
use crate::ai::blocklist::history_model::{CLIAgentConversation, CloudConversationData};
use crate::terminal::view::TerminalView;

/// Describes restore-context setup state for directory reconciliation and hinting.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) enum RestorationDirState {
    Unchanged,
    MissingOriginalDir,
    NeedsCd { path: String },
}

/// Specifies how removed AI conversations used to be restored when creating a
/// terminal pane. The enum remains only because pane/session plumbing still
/// carries the value while the product surface is being deleted.
#[derive(Clone, Debug)]
pub enum ConversationRestorationInNewPaneType {
    Startup {
        conversation_ids: Vec1<AIConversationId>,
        active_conversation_id: Option<AIConversationId>,
    },
    Historical {
        conversation: AIConversation,
        should_use_live_appearance: bool,
        ambient_agent_task_id: Option<crate::ai::ambient_agents::AmbientAgentTaskId>,
    },
    Forked {
        conversation: AIConversation,
    },
    HistoricalCLIAgent {
        conversation: CLIAgentConversation,
        should_use_live_appearance: bool,
    },
}

impl ConversationRestorationInNewPaneType {
    pub fn is_forked(&self) -> bool {
        matches!(self, Self::Forked { .. })
    }

    pub fn is_startup(&self) -> bool {
        matches!(self, Self::Startup { .. })
    }

    pub fn should_show_restore_context_hint(&self) -> bool {
        false
    }

    pub fn should_use_live_appearance(&self) -> bool {
        match self {
            Self::Historical {
                should_use_live_appearance,
                ..
            }
            | Self::HistoricalCLIAgent {
                should_use_live_appearance,
                ..
            } => *should_use_live_appearance,
            Self::Forked { .. } => true,
            Self::Startup { .. } => false,
        }
    }

    pub fn initial_working_directory(&self) -> Option<String> {
        match self {
            Self::Historical { conversation, .. } | Self::Forked { conversation } => {
                conversation.initial_working_directory()
            }
            Self::HistoricalCLIAgent { conversation, .. } => {
                conversation.metadata.working_directory.clone()
            }
            Self::Startup { .. } => None,
        }
    }
}

/// RestoredAIConversation stores a conversation to restore and any associated
/// data we need for restoration.
pub struct RestoredAIConversation {
    pub ai_conversation: AIConversation,
}

impl RestoredAIConversation {
    pub fn new(conversation: AIConversation) -> Self {
        Self {
            ai_conversation: conversation,
        }
    }
}

impl TerminalView {
    pub(crate) fn restore_conversation_and_directory_context<F>(
        &mut self,
        _cloud_conversation: CloudConversationData,
        _use_live_appearance: bool,
        on_restored: F,
        ctx: &mut ViewContext<Self>,
    ) where
        F: FnOnce(&mut Self, &mut ViewContext<Self>) + 'static,
    {
        on_restored(self, ctx);
    }

    pub(super) fn get_conversations_to_restore(
        _conversation_ids: &[AIConversationId],
        _ctx: &mut ViewContext<Self>,
    ) -> Vec<AIConversation> {
        Vec::new()
    }

    pub fn restore_conversation_after_view_creation(
        &mut self,
        _restored: RestoredAIConversation,
        _use_live_appearance: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(super) fn restore_conversations_on_view_creation(
        &mut self,
        _conversation_restoration: ConversationRestorationInNewPaneType,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn maybe_show_restore_context_hint(
        &mut self,
        _restore_context_state: RestorationDirState,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn load_conversation_from_tasks<T>(&mut self, _task_list: T, _ctx: &mut ViewContext<Self>) {
    }

    pub fn create_and_insert_ai_block<T>(&mut self, _params: T, _ctx: &mut ViewContext<Self>) {}

    pub fn load_agent_mode_conversation(&mut self, _ctx: &mut ViewContext<Self>) {}
}
