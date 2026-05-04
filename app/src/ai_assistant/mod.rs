//! Warp AI assistant tombstone for Warp Lite.

use std::{collections::HashSet, sync::Arc};

use itertools::Itertools;
use lazy_static::lazy_static;
use pathfinder_color::ColorU;
use serde::{Deserialize, Serialize};
use warp_core::command::ExitCode;

use crate::{
    server::telemetry::OpenedWarpAISource,
    terminal::model::terminal_model::BlockIndex,
    workflows::workflow::{Argument, Workflow},
};

pub mod execution_context;

pub const PROMPT_CHARACTER_LIMIT: usize = 1000;
pub const AI_ASSISTANT_FEATURE_NAME: &str = "Warp AI";
pub const ASK_AI_ASSISTANT_TEXT: &str = "Ask Warp AI";
pub const AI_ASSISTANT_SVG_PATH: &str = "bundled/svg/ai-assistant.svg";

lazy_static! {
    pub static ref AI_ASSISTANT_LOGO_COLOR: ColorU = ColorU::new(243, 185, 17, 255);
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AskAIType {
    FromTextSelection {
        text: Arc<String>,
        populate_input_box: bool,
    },
    FromBlock {
        input: Arc<String>,
        output: Arc<String>,
        exit_code: ExitCode,
        block_index: BlockIndex,
    },
    FromBlocks {
        block_indices: HashSet<BlockIndex>,
    },
    FromAICommandSearch {
        query: Arc<String>,
    },
}

impl From<&AskAIType> for OpenedWarpAISource {
    fn from(value: &AskAIType) -> Self {
        match value {
            AskAIType::FromAICommandSearch { .. } => OpenedWarpAISource::FromAICommandSearch,
            AskAIType::FromBlock { .. } | AskAIType::FromBlocks { .. } => {
                OpenedWarpAISource::HelpWithBlock
            }
            AskAIType::FromTextSelection { .. } => OpenedWarpAISource::HelpWithTextSelection,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct AIGeneratedCommand {
    command: String,
    description: String,
    parameters: Vec<AIGeneratedCommandParameter>,
}

#[derive(Clone, Debug, Default)]
pub struct AIGeneratedCommandParameter {
    id: String,
    description: String,
}

impl From<AIGeneratedCommand> for Workflow {
    fn from(ai_command: AIGeneratedCommand) -> Self {
        Workflow::new(ai_command.description, ai_command.command).with_arguments(
            ai_command
                .parameters
                .into_iter()
                .map(|p| Argument {
                    name: p.id,
                    description: Some(p.description),
                    default_value: None,
                    arg_type: Default::default(),
                })
                .collect_vec(),
        )
    }
}

#[derive(Copy, Clone, Debug, Serialize, Deserialize)]
pub enum GenerateCommandsFromNaturalLanguageError {
    BadPrompt,
    AiProviderError,
    RateLimited,
    Other,
}

pub mod requests {
    #[derive(Clone, Debug, Default)]
    pub struct GenerateDialogueResult;
}

pub mod utils {
    #[derive(Clone, Debug, Default)]
    pub struct TranscriptPart;
}

pub mod panel {
    use warpui::{AppContext, Entity, TypedActionView, View, ViewContext};

    pub fn init(_ctx: &mut AppContext) {}

    #[derive(Clone, Debug)]
    pub enum AIAssistantPanelEvent {
        ClosePanel,
        PasteInTerminalInput(String),
        FocusTerminalInput,
        OpenWorkflowModalWithCommand(String),
    }

    #[derive(Default)]
    pub struct AIAssistantPanelView;

    impl AIAssistantPanelView {
        pub fn new<T, U>(_server_api: T, _ai_client: U, _ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn ask_ai<T, U>(&mut self, _query: T, _ctx: U) {}
    }

    impl Entity for AIAssistantPanelView {
        type Event = AIAssistantPanelEvent;
    }

    impl View for AIAssistantPanelView {
        fn ui_name() -> &'static str {
            "AIAssistantPanelView"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl TypedActionView for AIAssistantPanelView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}
