//! AI and agent functionality has been removed from Warp Lite.
//!
//! The lightweight types below are inert compatibility shims used while the
//! remaining UI and persistence callsites are being deleted from the compile
//! graph. They do not perform network requests, run agents, index code, or
//! persist cloud-backed state.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

#[derive(Clone, Debug, Default)]
pub struct AIRequestUsageModel;

impl Entity for AIRequestUsageModel {
    type Event = request_usage_model::AIRequestUsageModelEvent;
}

impl SingletonEntity for AIRequestUsageModel {}

impl AIRequestUsageModel {
    pub fn new<T, U>(_client: T, _ctx: U) -> Self {
        Self
    }

    pub fn new_for_test<T>(_client: T, _ctx: &mut ModelContext<Self>) -> Self {
        Self
    }

    pub fn refresh_request_usage_async(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn has_any_ai_remaining(&self) -> bool {
        false
    }

    pub fn has_requests_remaining(&self) -> bool {
        false
    }

    pub fn request_limit(&self) -> usize {
        0
    }

    pub fn requests_used(&self) -> usize {
        0
    }

    pub fn is_unlimited(&self) -> bool {
        false
    }

    pub fn next_refresh_time(&self) -> Option<chrono::DateTime<chrono::Utc>> {
        None
    }

    pub fn refresh_duration_to_string(&self) -> String {
        String::new()
    }

    pub fn ambient_only_credits_remaining(&self) -> Option<f32> {
        None
    }

    pub fn total_workspace_bonus_credits_remaining<T>(&self, _workspace: T) -> Option<f32> {
        None
    }

    pub fn total_current_workspace_bonus_credits_remaining<T>(&self, _ctx: T) -> Option<f32> {
        None
    }

    pub fn compute_buy_addon_credits_banner_display_state<T>(
        &self,
        _ctx: T,
    ) -> request_usage_model::BuyCreditsBannerDisplayState {
        request_usage_model::BuyCreditsBannerDisplayState::Hidden
    }

    pub fn dismiss_buy_credits_banner(&mut self, _ctx: &mut ModelContext<Self>) {}

    pub fn enable_buy_credits_banner<T>(&mut self, _ctx: T) {}

    pub fn last_update_time(&self) -> Option<std::time::Instant> {
        None
    }

    pub fn bonus_grants(&self) -> Vec<BonusGrant> {
        Vec::new()
    }

    pub fn codebase_context_limits(&self) -> CodebaseContextLimits {
        CodebaseContextLimits::default()
    }
}

#[derive(Clone, Debug)]
pub struct CodebaseContextLimits {
    pub max_indices_allowed: usize,
    pub max_files_per_repo: usize,
    pub embedding_generation_batch_size: usize,
}

impl Default for CodebaseContextLimits {
    fn default() -> Self {
        Self {
            max_indices_allowed: 0,
            max_files_per_repo: 0,
            embedding_generation_batch_size: 0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct BonusGrant {
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub cost_cents: i64,
    pub expiration: Option<chrono::DateTime<chrono::Utc>>,
    pub grant_type: String,
    pub reason: String,
    pub user_facing_message: Option<String>,
    pub request_credits_granted: i64,
    pub request_credits_remaining: i64,
    pub scope: BonusGrantScope,
}

pub mod request_usage_model {
    pub use crate::ai::{AIRequestUsageModel, BonusGrant};
    use chrono::Utc;
    use warp_graphql::scalars::time::ServerTimestamp;

    pub const AMBIENT_AGENT_TRIAL_CREDIT_THRESHOLD: u64 = 0;

    #[derive(Clone, Debug)]
    pub struct RequestLimitInfo {
        pub next_refresh_time: ServerTimestamp,
        pub is_unlimited: bool,
        pub num_requests_used_since_refresh: usize,
        pub limit: usize,
    }

    impl Default for RequestLimitInfo {
        fn default() -> Self {
            Self {
                next_refresh_time: ServerTimestamp::new(Utc::now()),
                is_unlimited: true,
                num_requests_used_since_refresh: 0,
                limit: 0,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum AIRequestUsageModelEvent {
        RequestUsageUpdated,
        RequestBonusRefunded,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum BuyCreditsBannerDisplayState {
        #[default]
        Hidden,
        Visible,
        OutOfCredits,
        MonthlyLimitReached,
    }

    #[derive(Clone, Debug, Default)]
    pub enum BonusGrantScope {
        #[default]
        Unknown,
        User,
        Workspace(String),
    }
}

pub use request_usage_model::{
    AIRequestUsageModelEvent, BonusGrantScope, BuyCreditsBannerDisplayState, RequestLimitInfo,
};

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct RequestUsageInfo;

pub mod agent {
    use super::*;

    #[derive(
        Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
    )]
    pub struct AIAgentActionId(pub uuid::Uuid);

    impl std::fmt::Display for AIAgentActionId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl From<String> for AIAgentActionId {
        fn from(value: String) -> Self {
            uuid::Uuid::parse_str(&value).map(Self).unwrap_or_default()
        }
    }

    #[derive(
        Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord,
    )]
    pub struct AIAgentExchangeId(pub uuid::Uuid);

    impl AIAgentExchangeId {
        pub fn new() -> Self {
            Self(uuid::Uuid::new_v4())
        }
    }

    impl std::fmt::Display for AIAgentExchangeId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.0.fmt(f)
        }
    }

    impl TryFrom<String> for AIAgentExchangeId {
        type Error = uuid::Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            Ok(Self(uuid::Uuid::parse_str(&value)?))
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AIAgentAttachment {
        DiffSet {
            file_diffs: std::collections::HashMap<String, Vec<DiffSetHunk>>,
            current: Option<CurrentHead>,
            base: DiffBase,
        },
        Removed,
    }

    impl Default for AIAgentAttachment {
        fn default() -> Self {
            Self::Removed
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AIAgentAction {
        pub action: AIAgentActionType,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct MessageId(pub String);

    impl MessageId {
        pub fn new(id: String) -> Self {
            Self(id)
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AIAgentExchange {
        pub id: AIAgentExchangeId,
        pub input: Vec<AIAgentInput>,
        pub start_time: chrono::DateTime<chrono::Local>,
        pub finish_time: Option<chrono::DateTime<chrono::Local>>,
        pub output_status: crate::ai::blocklist::AIBlockOutputStatus,
        pub working_directory: Option<String>,
        pub model_id: String,
        pub coding_model_id: String,
    }

    impl Default for AIAgentExchange {
        fn default() -> Self {
            Self {
                id: AIAgentExchangeId::default(),
                input: Vec::new(),
                start_time: chrono::DateTime::default(),
                finish_time: None,
                output_status: Default::default(),
                working_directory: None,
                model_id: String::new(),
                coding_model_id: String::new(),
            }
        }
    }

    impl AIAgentExchange {
        pub fn has_user_query(&self) -> bool {
            false
        }

        pub fn format_input_for_copy(&self) -> String {
            String::new()
        }

        pub fn has_accepted_file_edit(&self) -> bool {
            false
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum AIAgentInput {
        UserQuery {
            query: String,
            context: Vec<AIAgentContext>,
            static_query_type: Option<StaticQueryType>,
            referenced_attachments: Vec<AIAgentAttachment>,
            user_query_mode: UserQueryMode,
            running_command: Option<String>,
            intended_agent: Option<String>,
        },
        CodeReview {
            context: Vec<AIAgentContext>,
            review_comments: AgentReviewCommentBatch,
        },
        Unknown,
    }

    impl Default for AIAgentInput {
        fn default() -> Self {
            Self::Unknown
        }
    }

    impl AIAgentInput {
        pub fn user_query(&self) -> Option<String> {
            match self {
                Self::UserQuery { query, .. } => Some(query.clone()),
                _ => None,
            }
        }

        pub fn action_result(&self) -> Option<&AIAgentActionResult> {
            None
        }

        pub fn is_user_query(&self) -> bool {
            false
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIAgentOutput {
        pub messages: Vec<AIAgentOutputMessage>,
        pub server_output_id: Option<ServerOutputId>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIAgentOutputMessage {
        pub id: MessageId,
        pub message: AIAgentOutputMessageType,
    }

    impl AIAgentOutputMessage {
        pub fn text(id: MessageId, text: AIAgentText) -> Self {
            Self {
                id,
                message: AIAgentOutputMessageType::Text(text),
            }
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum AIAgentOutputMessageType {
        Text(AIAgentText),
        Unknown,
    }

    impl Default for AIAgentOutputMessageType {
        fn default() -> Self {
            Self::Unknown
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIAgentCitation;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum AIAgentContext {
        Block(Box<crate::ai::block_context::BlockContext>),
        SelectedText(String),
        Image(ImageContext),
        File(crate::ai::blocklist::PendingFile),
        Directory {
            pwd: Option<String>,
            home_dir: Option<std::path::PathBuf>,
            are_file_symbols_indexed: bool,
        },
    }

    impl Default for AIAgentContext {
        fn default() -> Self {
            Self::SelectedText(String::new())
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileLocations {
        pub name: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct FileContext {
        pub file_name: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct UpdatedFileContext {
        pub file_context: FileContext,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PassiveCodeDiffEntry;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum PassiveSuggestionResultType {
        Prompt { prompt: String },
        CodeDiff,
    }

    impl Default for PassiveSuggestionResultType {
        fn default() -> Self {
            Self::CodeDiff
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AgentOutputHandle;

    impl AgentOutputHandle {
        pub fn get(&self) -> &Self {
            self
        }

        pub fn actions(&self) -> &[AIAgentAction] {
            &[]
        }

        pub fn text_from_agent_output(&self) -> Vec<AIAgentText> {
            Vec::new()
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIAgentText {
        pub sections: Vec<AIAgentTextSection>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PlainAgentText(String);

    impl PlainAgentText {
        pub fn text(&self) -> &str {
            self.0.as_str()
        }
    }

    impl From<String> for PlainAgentText {
        fn from(value: String) -> Self {
            Self(value)
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum FinishedAIAgentOutput {
        Success { output: AgentOutputHandle },
        Error { error: RenderableAIError },
    }

    impl Default for FinishedAIAgentOutput {
        fn default() -> Self {
            Self::Success {
                output: AgentOutputHandle,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum StaticQueryType {
        #[default]
        Unknown,
        Install,
        Code,
        Deploy,
        SomethingElse,
        CustomOnboardingRequest,
        EvaluationSuite,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ExecutedShellCommand {
        pub id: crate::terminal::model::block::BlockId,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ShellCommandCompletedTrigger {
        pub executed_shell_command: Box<crate::ai::block_context::BlockContext>,
        pub relevant_files: Vec<String>,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CreateDocumentsRequest;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CreateDocumentsResult;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct EditDocumentsResult;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum AIAgentOutputStatus {
        #[default]
        Unknown,
        Finished {
            finished_output: FinishedAIAgentOutput,
            server_output_id: Option<ServerOutputId>,
        },
    }

    impl AIAgentOutputStatus {
        pub fn server_output_id(&self) -> Option<ServerOutputId> {
            match self {
                Self::Finished {
                    server_output_id, ..
                } => server_output_id.clone(),
                Self::Unknown => None,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum AIAgentPtyWriteMode {
        #[default]
        Raw,
        Block,
    }

    impl AIAgentPtyWriteMode {
        pub fn decorate_bytes<T>(&self, bytes: T, _is_bracketed_paste_enabled: bool) -> T {
            bytes
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ProgrammingLanguage {
        Shell(crate::terminal::shell::ShellType),
        Other(String),
    }

    impl Default for ProgrammingLanguage {
        fn default() -> Self {
            Self::Other(String::new())
        }
    }

    impl From<String> for ProgrammingLanguage {
        fn from(value: String) -> Self {
            if let Some(shell_type) =
                crate::terminal::shell::ShellType::from_markdown_language_spec(value.as_str())
            {
                Self::Shell(shell_type)
            } else {
                Self::Other(value)
            }
        }
    }

    impl std::fmt::Display for ProgrammingLanguage {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Shell(shell_type) => write!(f, "{}", shell_type.name()),
                Self::Other(language) => write!(f, "{}", language.to_lowercase()),
            }
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ImageContext {
        pub data: String,
        pub mime_type: String,
        pub file_name: String,
        pub is_figma: bool,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AgentReviewCommentBatch {
        pub comments: Vec<crate::code_review::comments::AttachedReviewComment>,
        pub diff_set: Option<String>,
        pub base_branch: Option<String>,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DiffSetHunk {
        pub line_range: std::ops::Range<warp_editor::render::model::LineCount>,
        pub diff_content: String,
        pub lines_added: u32,
        pub lines_removed: u32,
    }

    impl Default for DiffSetHunk {
        fn default() -> Self {
            Self {
                line_range: warp_editor::render::model::LineCount::from(0)
                    ..warp_editor::render::model::LineCount::from(0),
                diff_content: String::new(),
                lines_added: 0,
                lines_removed: 0,
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CurrentHead {
        #[default]
        Unknown,
        BranchName(String),
    }

    impl CurrentHead {
        pub fn title(&self) -> String {
            match self {
                Self::Unknown => String::new(),
                Self::BranchName(name) => name.clone(),
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DiffBase {
        #[default]
        Unknown,
        UncommittedChanges,
        BranchName(String),
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIIdentifiers {
        pub server_output_id: Option<ServerOutputId>,
        pub client_exchange_id: Option<AIAgentExchangeId>,
        pub server_conversation_id: Option<conversation::AIConversationId>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum AIAgentTextSection {
        PlainText { text: PlainAgentText },
        Other,
    }

    impl Default for AIAgentTextSection {
        fn default() -> Self {
            Self::Other
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ReadFilesRequest;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum EntrypointType {
        #[default]
        Unknown,
        UserInitiated,
        AgentInitiated,
        Onboarding {
            chip_type: crate::terminal::view::block_onboarding::onboarding_agentic_suggestions_block::OnboardingChipType,
        },
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PassiveSuggestionTrigger {
        #[default]
        Unknown,
        ShellCommandCompleted(ShellCommandCompletedTrigger),
    }

    impl PassiveSuggestionTrigger {
        pub fn block_id(&self) -> Option<crate::terminal::model::block::BlockId> {
            match self {
                Self::ShellCommandCompleted(trigger) => {
                    Some(trigger.executed_shell_command.id.clone())
                }
                Self::Unknown => None,
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct SuggestedLoggingId(pub String);

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum CancellationReason {
        #[default]
        ManuallyCancelled,
        FollowUpSubmitted {
            is_for_same_conversation: bool,
        },
        UserCommandExecuted,
        Reverted,
        Deleted,
        OptimisticCLISubagentCompletion,
    }

    impl CancellationReason {
        pub fn is_manually_cancelled(&self) -> bool {
            matches!(self, Self::ManuallyCancelled)
        }
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum RequestFileEditsResult {
        Success {
            updated_files: Vec<UpdatedFileContext>,
            lines_added: usize,
            lines_removed: usize,
        },
        Error,
    }

    impl Default for RequestFileEditsResult {
        fn default() -> Self {
            Self::Error
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum UserQueryMode {
        #[default]
        Normal,
        Plan,
        Orchestrate,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AIAgentActionType {
        #[default]
        Unknown,
        CreateDocuments(CreateDocumentsRequest),
        EditDocuments(EditDocumentsResult),
        RequestCommandOutput {
            command: String,
        },
        ReadFiles(ReadFilesRequest),
        SearchCodebase(String),
        RequestFileEdits {
            path: String,
        },
        WriteToLongRunningShellCommand {
            input: String,
        },
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AIAgentActionResultType {
        #[default]
        Unknown,
        RequestFileEdits(RequestFileEditsResult),
        RequestCommandOutput,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIAgentActionResult {
        pub id: AIAgentActionId,
        pub task_id: task::TaskId,
        pub result: AIAgentActionResultType,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum RenderableAIError {
        Other { error_message: String },
        QuotaLimit,
    }

    impl Default for RenderableAIError {
        fn default() -> Self {
            Self::Other {
                error_message: String::new(),
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ServerOutputId(pub String);

    impl ServerOutputId {
        pub fn new(id: String) -> Self {
            Self(id)
        }
    }

    impl std::fmt::Display for ServerOutputId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(self.0.as_str())
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct LifecycleEventType;

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum StartAgentExecutionMode {
        Local {
            harness_type: Option<String>,
        },
        Remote {
            environment_id: String,
            skill_references: Vec<String>,
            model_id: Option<String>,
            computer_use_enabled: Option<bool>,
            worker_host: Option<String>,
            harness_type: String,
            title: Option<String>,
        },
    }

    impl Default for StartAgentExecutionMode {
        fn default() -> Self {
            Self::Local { harness_type: None }
        }
    }

    pub mod conversation {
        use super::*;

        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            Hash,
            Serialize,
            Deserialize,
            PartialOrd,
            Ord,
        )]
        pub struct AIConversationId(pub uuid::Uuid);

        impl AIConversationId {
            pub fn new() -> Self {
                Self(uuid::Uuid::new_v4())
            }

            pub fn as_string(&self) -> String {
                self.0.to_string()
            }

            pub fn id(&self) -> Self {
                *self
            }
        }

        impl std::fmt::Display for AIConversationId {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl TryFrom<String> for AIConversationId {
            type Error = uuid::Error;

            fn try_from(value: String) -> Result<Self, Self::Error> {
                Ok(Self(uuid::Uuid::parse_str(&value)?))
            }
        }

        impl TryFrom<&str> for AIConversationId {
            type Error = uuid::Error;

            fn try_from(value: &str) -> Result<Self, Self::Error> {
                Ok(Self(uuid::Uuid::parse_str(value)?))
            }
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct AIConversation {
            #[serde(skip)]
            pub nav_data: crate::ai::conversation_navigation::ConversationNavigationData,
        }

        impl AIConversation {
            pub fn is_empty(&self) -> bool {
                true
            }

            pub fn is_entirely_passive(&self) -> bool {
                false
            }

            pub fn title(&self) -> Option<String> {
                None
            }

            pub fn exchange_count(&self) -> usize {
                0
            }

            pub fn id(&self) -> AIConversationId {
                AIConversationId::default()
            }

            pub fn set_fallback_display_title<T>(&mut self, _title: T) {}

            pub fn latest_exchange(&self) -> Option<&super::AIAgentExchange> {
                None
            }

            pub fn exchange_with_id(
                &self,
                _exchange_id: super::AIAgentExchangeId,
            ) -> Option<&super::AIAgentExchange> {
                None
            }

            pub fn get_task<T>(&self, _task_id: &T) -> Option<&super::task::Task> {
                None
            }

            pub fn get_root_task(&self) -> Option<&super::task::Task> {
                None
            }

            pub fn run_id(&self) -> Option<String> {
                None
            }

            pub fn server_conversation_token(
                &self,
            ) -> Option<&super::api::ServerConversationToken> {
                None
            }

            pub fn forked_from_server_conversation_token(
                &self,
            ) -> Option<&super::api::ServerConversationToken> {
                None
            }

            pub fn is_child_agent_conversation(&self) -> bool {
                false
            }

            pub fn parent_conversation_id(&self) -> Option<AIConversationId> {
                None
            }

            pub fn server_metadata(&self) -> Option<&ServerAIConversationMetadata> {
                None
            }

            pub fn initial_user_query(&self) -> Option<&str> {
                None
            }

            pub fn latest_user_query(&self) -> Option<String> {
                None
            }

            pub fn initial_working_directory(&self) -> Option<String> {
                None
            }

            pub fn credits_spent(&self) -> Option<f32> {
                None
            }

            pub fn credits_spent_for_last_block(&self) -> Option<f32> {
                None
            }

            pub fn tool_usage_metadata(
                &self,
            ) -> &'static crate::persistence::model::ToolUsageMetadata {
                static TOOL_USAGE: std::sync::LazyLock<
                    crate::persistence::model::ToolUsageMetadata,
                > = std::sync::LazyLock::new(Default::default);
                &TOOL_USAGE
            }

            pub fn time_to_first_token_for_last_user_query_ms(&self) -> Option<u64> {
                None
            }

            pub fn total_agent_response_time_since_last_user_query_ms(&self) -> Option<u64> {
                None
            }

            pub fn wall_to_wall_response_time_since_last_query(&self) -> Option<u64> {
                None
            }

            pub fn token_usage(&self) -> &'static [crate::persistence::model::ModelTokenUsage] {
                &[]
            }

            pub fn context_window_usage(&self) -> Option<f32> {
                None
            }

            pub fn has_opened_code_review(&self) -> bool {
                false
            }

            pub fn exchange_id_for_action<T>(
                &self,
                _action_id: T,
            ) -> Option<super::AIAgentExchangeId> {
                None
            }

            pub fn context_for_exchange<T>(
                &self,
                _exchange_id: T,
            ) -> std::vec::IntoIter<super::AIAgentContext> {
                Vec::new().into_iter()
            }

            pub fn exchanges_reversed(&self) -> std::iter::Empty<&'static super::AIAgentExchange> {
                std::iter::empty()
            }

            pub fn artifacts(&self) -> Vec<()> {
                Vec::new()
            }

            pub fn has_active_subagent(&self) -> bool {
                false
            }

            pub fn last_modified_at(&self) -> Option<chrono::DateTime<chrono::Local>> {
                None
            }

            pub fn root_task_exchanges(&self) -> std::slice::Iter<'static, super::AIAgentExchange> {
                static EXCHANGES: std::sync::LazyLock<Vec<super::AIAgentExchange>> =
                    std::sync::LazyLock::new(Vec::new);
                EXCHANGES.iter()
            }

            pub fn export_to_markdown<T>(&self, _action_model: Option<T>) -> String {
                String::new()
            }

            pub fn status(&self) -> ConversationStatus {
                ConversationStatus::default()
            }

            pub fn to_serialized_blocklist_items(
                &self,
            ) -> Vec<crate::terminal::model::block::SerializedBlockListItem> {
                Vec::new()
            }

            pub fn all_tasks(&self) -> std::iter::Empty<&super::task::Task> {
                std::iter::empty()
            }

            pub fn all_exchanges_by_task(
                &self,
            ) -> Vec<(super::task::TaskId, Vec<&super::AIAgentExchange>)> {
                Vec::new()
            }

            pub fn first_exchange(&self) -> Option<&super::AIAgentExchange> {
                None
            }
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum ConversationStatus {
            #[default]
            Unknown,
            InProgress,
            Success,
            Cancelled,
            Error,
            Blocked {
                blocked_action: String,
            },
        }

        impl ConversationStatus {
            pub fn is_error(&self) -> bool {
                matches!(self, Self::Error | Self::Blocked { .. })
            }

            pub fn is_in_progress(&self) -> bool {
                matches!(self, Self::InProgress)
            }

            pub fn is_done(&self) -> bool {
                !self.is_in_progress()
            }

            pub fn is_blocked(&self) -> bool {
                matches!(self, Self::Blocked { .. })
            }

            pub fn render_icon<T>(&self, _appearance: T) -> warpui::elements::Icon {
                warp_core::ui::icons::Icon::Circle.to_warpui_icon(
                    warp_core::ui::theme::Fill::Solid(pathfinder_color::ColorU::black()),
                )
            }

            pub fn status_icon_and_color(
                &self,
                _theme: &warp_core::ui::theme::WarpTheme,
            ) -> (warp_core::ui::icons::Icon, pathfinder_color::ColorU) {
                (
                    warp_core::ui::icons::Icon::Circle,
                    pathfinder_color::ColorU::black(),
                )
            }
        }

        impl std::fmt::Display for ConversationStatus {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{self:?}")
            }
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum AIAgentHarness {
            #[default]
            Unknown,
            Oz,
            ClaudeCode,
            Gemini,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct ServerAIConversationMetadata {
            pub ambient_agent_task_id: Option<crate::ai::ambient_agents::AmbientAgentTaskId>,
        }
    }

    pub mod task {
        use serde::{Deserialize, Serialize};

        #[derive(
            Clone,
            Copy,
            Debug,
            Default,
            PartialEq,
            Eq,
            Hash,
            Serialize,
            Deserialize,
            PartialOrd,
            Ord,
        )]
        pub struct TaskId(pub uuid::Uuid);

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct Task;

        impl Task {
            pub fn all_contexts(&self) -> std::vec::IntoIter<crate::ai::agent::AIAgentContext> {
                Vec::new().into_iter()
            }

            pub fn last_exchange(&self) -> Option<crate::ai::agent::AIAgentExchange> {
                None
            }

            pub fn is_cli_subagent(&self) -> bool {
                false
            }

            pub fn is_warp_documentation_search_subagent(&self) -> bool {
                false
            }

            pub fn is_conversation_search_subagent(&self) -> bool {
                false
            }

            pub fn exchanges_len(&self) -> usize {
                0
            }
        }

        pub mod helper {
            pub trait MessageExt {}
        }
    }

    pub mod api {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct ServerConversationToken(pub String);

        impl ServerConversationToken {
            pub fn new(value: String) -> Self {
                Self(value)
            }

            pub fn as_str(&self) -> &str {
                self.0.as_str()
            }

            pub fn conversation_link(&self) -> String {
                String::new()
            }

            pub fn debug_link(&self) -> String {
                String::new()
            }
        }

        impl From<session_sharing_protocol::common::ServerConversationToken> for ServerConversationToken {
            fn from(token: session_sharing_protocol::common::ServerConversationToken) -> Self {
                Self(token.to_string())
            }
        }

        impl TryFrom<ServerConversationToken>
            for session_sharing_protocol::common::ServerConversationToken
        {
            type Error = uuid::Error;

            fn try_from(token: ServerConversationToken) -> Result<Self, Self::Error> {
                token.as_str().parse()
            }
        }
    }

    pub mod redaction {
        pub fn redact_secrets(input: impl AsRef<str>) -> String {
            input.as_ref().to_owned()
        }
    }

    pub mod action {
        use std::ops::Range;
        use std::path::PathBuf;

        use serde::{Deserialize, Serialize};

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum CommentSide {
            Left,
            #[default]
            Right,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct InsertReviewCommentLine {
            pub diff_hunk_text: String,
            pub comment_line_range: Range<usize>,
            pub side: Option<CommentSide>,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct InsertReviewCommentLocation {
            pub relative_file_path: PathBuf,
            pub line: Option<InsertReviewCommentLine>,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct InsertReviewComment {
            pub author: String,
            pub comment_id: String,
            pub parent_comment_id: Option<String>,
            pub html_url: Option<String>,
            pub comment_body: String,
            pub last_modified_timestamp: String,
            pub comment_location: Option<InsertReviewCommentLocation>,
        }
    }

    pub mod action_result {
        pub use super::RequestFileEditsResult;
    }

    pub mod todos {
        pub mod popup {
            use warpui::{Entity, TypedActionView, View};

            #[derive(Clone, Debug)]
            pub enum AgentTodosPopupEvent {
                Close,
            }

            #[derive(Default)]
            pub struct AgentTodosPopupView;

            impl AgentTodosPopupView {
                pub fn new<A, B, C>(_terminal_view_id: A, _context_model: B, _ctx: C) -> Self {
                    Self
                }
            }

            impl Entity for AgentTodosPopupView {
                type Event = AgentTodosPopupEvent;
            }

            impl View for AgentTodosPopupView {
                fn ui_name() -> &'static str {
                    "AgentTodosPopupView"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl TypedActionView for AgentTodosPopupView {
                type Action = ();

                fn handle_action(
                    &mut self,
                    _action: &Self::Action,
                    _ctx: &mut warpui::ViewContext<Self>,
                ) {
                }
            }
        }
    }

    pub mod icons {
        use warpui::elements::Icon;

        fn inert_icon() -> Icon {
            Icon::new(
                "bundled/svg/ellipse.svg",
                pathfinder_color::ColorU {
                    r: 0,
                    g: 0,
                    b: 0,
                    a: 0,
                },
            )
        }

        pub fn failed_icon<T>(_appearance: T) -> Icon {
            inert_icon()
        }

        pub fn in_progress_icon<T>(_appearance: T) -> Icon {
            inert_icon()
        }

        pub fn yellow_running_icon<T>(_appearance: T) -> Icon {
            inert_icon()
        }

        pub fn yellow_stop_icon<T>(_appearance: T) -> Icon {
            inert_icon()
        }
    }
}

pub mod blocklist {
    use super::*;

    pub const CLAUDE_ORANGE: pathfinder_color::ColorU = pathfinder_color::ColorU {
        r: 0,
        g: 0,
        b: 0,
        a: 0,
    };
    pub const FORK_PREFIX: &str = "";
    pub const NEW_AGENT_PANE_LABEL: &str = "Terminal";
    pub static ATTACH_AS_AGENT_MODE_CONTEXT_TEXT: std::sync::LazyLock<&'static str> =
        std::sync::LazyLock::new(|| "");
    pub const PRE_REWIND_PREFIX: &str = "";
    pub static BLOCK_CONTEXT_ATTACHMENT_REGEX: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new("$^").expect("valid regex"));
    pub static DIFF_HUNK_ATTACHMENT_REGEX: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new("$^").expect("valid regex"));
    pub static DRIVE_OBJECT_ATTACHMENT_REGEX: std::sync::LazyLock<regex::Regex> =
        std::sync::LazyLock::new(|| regex::Regex::new("$^").expect("valid regex"));

    pub fn ai_brand_color<T>(_theme: T) -> pathfinder_color::ColorU {
        CLAUDE_ORANGE
    }

    pub fn format_credits(credits: f32) -> String {
        format!("{credits:.0}")
    }

    pub fn render_ai_agent_mode_icon<T, U>(_app: T, _color: U) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }

    pub fn render_ai_follow_up_icon<T, U>(_color: T, _app: U) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }

    pub fn ai_indicator_height<T>(_ctx: T) -> f32 {
        0.
    }

    pub fn get_ai_block_overflow_menu_element_position_id<T>(_id: T) -> String {
        "removed-ai-block-overflow".to_owned()
    }

    pub fn get_attached_blocks_chip_element_position_id<T>(_id: T) -> String {
        "removed-attached-blocks".to_owned()
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AIQueryHistory {
        pub start_time: chrono::DateTime<chrono::Local>,
        pub query_text: String,
        pub history_order: crate::input_suggestions::HistoryOrder,
        pub output_status: AIQueryHistoryOutputStatus,
        pub working_directory: Option<String>,
    }

    impl Default for AIQueryHistory {
        fn default() -> Self {
            Self {
                start_time: chrono::DateTime::default(),
                query_text: String::new(),
                history_order: crate::input_suggestions::HistoryOrder::DifferentSession,
                output_status: AIQueryHistoryOutputStatus::Unknown,
                working_directory: None,
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AIBlockResponseRating {
        #[default]
        Unknown,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CommandExecutionPermissionAllowedReason {
        #[default]
        Unknown,
    }

    pub use input_classifier::InputType;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum PendingQueryState {
        #[default]
        None,
        New,
        Existing {
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
    }

    #[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
    pub struct InputConfig {
        pub is_locked: bool,
        pub input_type: InputType,
    }

    impl Default for InputConfig {
        fn default() -> Self {
            Self {
                is_locked: false,
                input_type: InputType::Shell,
            }
        }
    }

    impl InputConfig {
        pub fn new(_ctx: &AppContext) -> Self {
            Self::default()
        }

        pub fn is_shell(&self) -> bool {
            matches!(self.input_type, InputType::Shell)
        }

        pub fn is_ai(&self) -> bool {
            matches!(self.input_type, InputType::AI)
        }

        pub fn locked(mut self) -> Self {
            self.is_locked = true;
            self
        }

        pub fn unlocked(mut self) -> Self {
            self.is_locked = false;
            self
        }

        pub fn unlocked_if_autodetection_enabled<T: ?Sized>(
            mut self,
            _enabled: bool,
            _ctx: &T,
        ) -> Self {
            self.is_locked = false;
            self
        }

        pub fn with_toggled_type(mut self) -> Self {
            self.input_type = match self.input_type {
                InputType::Shell => InputType::AI,
                InputType::AI => InputType::Shell,
            };
            self
        }

        pub fn with_input_type(mut self, input_type: InputType) -> Self {
            self.input_type = input_type;
            self
        }
    }

    impl From<InputConfig> for session_sharing_protocol::common::InputMode {
        fn from(value: InputConfig) -> Self {
            let input_type = match value.input_type {
                InputType::Shell => session_sharing_protocol::common::InputType::Shell,
                InputType::AI => session_sharing_protocol::common::InputType::AI,
            };
            Self::new(input_type, value.is_locked)
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PendingFile {
        pub file_name: String,
        pub file_path: std::path::PathBuf,
        pub mime_type: String,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum PendingAttachment {
        File(PendingFile),
    }

    impl Default for PendingAttachment {
        fn default() -> Self {
            Self::File(PendingFile::default())
        }
    }

    impl PendingAttachment {
        pub fn file_name(&self) -> &str {
            match self {
                Self::File(file) => file.file_name.as_str(),
            }
        }

        pub fn attachment_type(&self) -> AttachmentType {
            match self {
                Self::File(_) => AttachmentType::File,
            }
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum AttachmentType {
        #[default]
        Unknown,
        Image,
        File,
    }

    #[derive(Clone, Debug)]
    pub enum AIBlockEvent {}

    #[derive(Clone, Debug)]
    pub enum BlocklistAIActionEvent {
        ActionBlockedOnUserConfirmation {
            action_id: crate::ai::agent::AIAgentActionId,
        },
        ExecutingAction {
            action_id: crate::ai::agent::AIAgentActionId,
        },
        FinishedAction {
            action_id: crate::ai::agent::AIAgentActionId,
        },
        InitProject(()),
        ToggleCodeReview(()),
        InsertCodeReviewComments {
            action_id: crate::ai::agent::AIAgentActionId,
            repo_path: std::path::PathBuf,
            comments: Vec<crate::ai::agent::action::InsertReviewComment>,
            base_branch: Option<String>,
        },
        QueuedAction(()),
    }

    #[derive(Clone, Debug, Default)]
    pub struct BlocklistAIActionModel;

    impl BlocklistAIActionModel {
        pub fn new<A, B, C, D, E, F>(
            _terminal_model: A,
            _active_session: B,
            _model_events_handle: C,
            _get_relevant_files_controller: D,
            _terminal_view_id: E,
            _ctx: F,
        ) -> Self {
            Self
        }

        pub fn shell_command_executor<T>(
            &self,
            ctx: &mut T,
        ) -> warpui::ModelHandle<ShellCommandExecutor>
        where
            T: std::ops::DerefMut<Target = warpui::AppContext>,
        {
            ctx.add_model(|_| ShellCommandExecutor)
        }

        pub fn start_agent_executor<T>(
            &self,
            ctx: &mut T,
        ) -> warpui::ModelHandle<StartAgentExecutor>
        where
            T: std::ops::DerefMut<Target = warpui::AppContext>,
        {
            ctx.add_model(|_| StartAgentExecutor)
        }

        pub fn get_action_result<T>(
            &self,
            _action_id: T,
        ) -> Option<crate::ai::agent::AIAgentActionResult> {
            None
        }

        pub fn get_pending_action(
            &self,
            _ctx: &warpui::AppContext,
        ) -> Option<crate::ai::agent::AIAgentAction> {
            None
        }
    }

    impl Entity for BlocklistAIActionModel {
        type Event = BlocklistAIActionEvent;
    }

    impl SingletonEntity for BlocklistAIActionModel {}

    #[derive(Clone, Debug)]
    pub enum BlocklistAIControllerEvent {
        SentRequest {
            contains_user_query: bool,
            is_queued_prompt: bool,
            model_id: crate::ai::llms::LLMId,
            stream_id: ResponseStreamId,
        },
        FinishedReceivingOutput {
            stream_id: ResponseStreamId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        ExportConversationToFile {
            filename: Option<String>,
        },
        FreeTierLimitCheckTriggered,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ResponseStreamId(pub String);

    #[derive(Clone, Debug, Default)]
    pub struct BlocklistAIController;

    impl BlocklistAIController {
        pub fn new<A, B, C, D, E, F, G, H>(
            _ai_input_model: A,
            _ai_context_model: B,
            _ai_action_model: C,
            _active_session: D,
            _agent_view_controller: E,
            _terminal_model: F,
            _terminal_view_id: G,
            _ctx: H,
        ) -> Self {
            Self
        }

        pub fn send_agent_query_in_conversation<T, U, V>(
            &mut self,
            _prompt: T,
            _conversation_id: U,
            _ctx: V,
        ) {
        }

        pub fn cancel_conversation_progress<T, U, V>(
            &mut self,
            _conversation_id: T,
            _reason: U,
            _ctx: V,
        ) {
        }

        pub fn send_slash_command_request<T, U>(&mut self, _request: T, _ctx: U) {}

        pub fn send_queued_slash_command_request<T, U>(&mut self, _request: T, _ctx: U) {}

        pub fn send_user_query_in_new_conversation<T, X>(
            &mut self,
            _query: T,
            _static_query_type: Option<crate::ai::agent::StaticQueryType>,
            _entrypoint_type: crate::ai::agent::EntrypointType,
            _participant_id: Option<session_sharing_protocol::common::ParticipantId>,
            _ctx: X,
        ) {
        }

        pub fn send_user_query_in_conversation<T, U, W>(
            &mut self,
            _query: T,
            _conversation_id: U,
            _participant_id: Option<session_sharing_protocol::common::ParticipantId>,
            _ctx: W,
        ) {
        }

        pub fn send_queued_user_query_in_conversation<T, U, W>(
            &mut self,
            _query: T,
            _conversation_id: U,
            _participant_id: Option<session_sharing_protocol::common::ParticipantId>,
            _ctx: W,
        ) {
        }

        pub fn send_queued_user_query_in_new_conversation<T, X>(
            &mut self,
            _query: T,
            _static_query_type: Option<crate::ai::agent::StaticQueryType>,
            _entrypoint_type: crate::ai::agent::EntrypointType,
            _participant_id: Option<session_sharing_protocol::common::ParticipantId>,
            _ctx: X,
        ) {
        }

        pub fn send_zero_state_prompt_suggestion<T, U>(&mut self, _suggestion: T, _ctx: U) {}

        pub fn set_sharer_participant_id<T>(&mut self, _participant_id: T) {}

        pub fn handle_shared_session_cancel_action<T, U>(&mut self, _action: T, _ctx: U) {}

        pub fn execute_agent_prompt_for_shared_session<T, U, V, W, X>(
            &mut self,
            _prompt: T,
            _server_conversation_token: U,
            _attachments: V,
            _participant_id: W,
            _ctx: X,
        ) {
        }

        pub fn send_agent_response_for_shared_session<T, U>(&mut self, _response: T, _ctx: U) {}

        pub fn find_existing_conversation_by_server_token<T, U>(
            &mut self,
            _token: T,
            _ctx: U,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn mark_action_as_remotely_executing_in_shared_session<T, U, V>(
            &mut self,
            _action_id: T,
            _conversation_id: U,
            _ctx: V,
        ) {
        }

        pub fn set_current_response_initiator<T>(&mut self, _participant_id: T) {}

        pub fn link_forked_conversation_token<T, U, V: ?Sized>(
            &mut self,
            _conversation_id: T,
            _forked_token: U,
            _ctx: &mut V,
        ) {
        }

        pub fn handle_shared_session_response_event<T, U: ?Sized>(
            &mut self,
            _event: T,
            _ctx: &mut U,
        ) {
        }

        pub fn resume_conversation<A, B, C, E>(
            &mut self,
            _conversation_id: A,
            _can_attempt_resume_on_error: B,
            _is_auto_resume_after_error: C,
            _context: Vec<crate::ai::agent::AIAgentContext>,
            _ctx: E,
        ) {
        }

        pub fn send_passive_suggestion_result<A, B, C, D>(
            &mut self,
            _conversation_id: A,
            _result: B,
            _trigger: C,
            _ctx: D,
        ) {
        }

        pub fn queue_passive_suggestion_result<A, B, C, D>(
            &mut self,
            _conversation_id: A,
            _result: B,
            _trigger: C,
            _ctx: D,
        ) {
        }

        pub fn request_follow_up_after_actions<A, B, C>(&mut self, _id: A, _query: B, _ctx: C) {}

        pub fn send_ai_input_with_context<A, B, C, D>(
            &mut self,
            _query: A,
            _context: B,
            _origin: C,
            _ctx: D,
        ) {
        }

        pub fn send_custom_ai_input_query<A, B>(&mut self, _query: A, _ctx: B) {}

        pub fn clear_finished_action_results<T, U>(&mut self, _conversation_id: T, _ctx: U) {}

        pub fn send_user_query_in_conversation_no_lrc_subagent<T, U, V, W: ?Sized>(
            &mut self,
            _query: T,
            _conversation_id: U,
            _participant_id: V,
            _ctx: &mut W,
        ) {
        }
    }

    impl Entity for BlocklistAIController {
        type Event = BlocklistAIControllerEvent;
    }

    impl SingletonEntity for BlocklistAIController {}

    #[derive(Clone, Debug)]
    pub enum LegacyPassiveSuggestionsEvent {
        PromptSuggestionsGenerated {
            prompt_suggestion: (),
            block_id: crate::terminal::model::block::BlockId,
            command: String,
            request_duration_ms: u64,
        },
        PassiveCodeDiffRequestStarted {
            prompt_suggestion_id: String,
            code_exchange_id: crate::ai::agent::AIAgentExchangeId,
            block_id: crate::terminal::model::block::BlockId,
        },
        PassiveCodeDiffFailed {
            reason: crate::server::telemetry::PromptSuggestionFallbackReason,
        },
    }

    #[derive(Clone, Debug, Default)]
    pub struct LegacyPassiveSuggestionsModel;

    impl LegacyPassiveSuggestionsModel {
        pub fn new<A, B, C, D, E, F>(
            _active_session: A,
            _terminal_model: B,
            _ai_controller: C,
            _model_events_handle: D,
            _terminal_view_id: E,
            _ctx: F,
        ) -> Self {
            Self
        }

        pub fn abort_pending_requests<T>(&mut self, _ctx: T) -> Vec<String> {
            Vec::new()
        }

        pub fn is_passive_code_diff_being_generated(&self) -> bool {
            false
        }
    }

    impl Entity for LegacyPassiveSuggestionsModel {
        type Event = LegacyPassiveSuggestionsEvent;
    }

    #[derive(Clone, Debug)]
    pub enum MaaPassiveSuggestionsEvent {
        NewPromptSuggestion(()),
        NewCodeDiffSuggestion(()),
    }

    #[derive(Clone, Debug, Default)]
    pub struct MaaPassiveSuggestionsModel;

    impl MaaPassiveSuggestionsModel {
        pub fn new<A, B, C, D, E, F, G>(
            _active_session: A,
            _terminal_model: B,
            _ai_controller: C,
            _model_events_handle: D,
            _ambient_agent_view_model: E,
            _terminal_view_id: F,
            _ctx: G,
        ) -> Self {
            Self
        }

        pub fn abort_pending_requests<T>(&mut self, _ctx: T) {}
    }

    impl Entity for MaaPassiveSuggestionsModel {
        type Event = MaaPassiveSuggestionsEvent;
    }

    #[derive(Clone, Debug)]
    pub struct PassiveSuggestionsModels {
        pub maa: warpui::ModelHandle<MaaPassiveSuggestionsModel>,
        pub legacy: warpui::ModelHandle<LegacyPassiveSuggestionsModel>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct RequestFileEditsFormatKind;

    #[derive(Clone, Debug)]
    pub enum ShellCommandExecutorEvent {
        ExecuteCommand {
            command: String,
            action_id: crate::ai::agent::AIAgentActionId,
        },
        WriteToPty {
            input: Vec<u8>,
            mode: crate::ai::agent::AIAgentPtyWriteMode,
        },
        CancelExecution,
        TransferControlToUser {
            reason: String,
        },
    }

    #[derive(Clone, Debug, Default)]
    pub struct ShellCommandExecutor;

    impl Entity for ShellCommandExecutor {
        type Event = ShellCommandExecutorEvent;
    }

    #[derive(Clone, Debug)]
    pub enum StartAgentExecutorEvent {
        CreateAgent(StartAgentRequest),
    }

    #[derive(Clone, Debug, Default)]
    pub struct StartAgentExecutor;

    impl Entity for StartAgentExecutor {
        type Event = StartAgentExecutorEvent;
    }

    #[derive(Clone, Debug, Default)]
    pub struct StartAgentRequest {
        pub name: String,
        pub prompt: String,
        pub execution_mode: crate::ai::agent::StartAgentExecutionMode,
        pub lifecycle_subscription: Option<Vec<crate::ai::agent::LifecycleEventType>>,
        pub parent_conversation_id: crate::ai::agent::conversation::AIConversationId,
        pub parent_run_id: Option<String>,
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum SlashCommandRequest {
        CreateNewProject {
            query: String,
        },
        CloneRepository {
            url: String,
        },
        InitProjectRules,
        CreateEnvironment {
            repos: Vec<String>,
            use_current_dir: bool,
        },
        Summarize {
            prompt: Option<String>,
        },
        FetchReviewComments {
            repo_path: String,
        },
        InvokeSkill {
            skill: crate::ai::skills::ParsedSkill,
            user_query: Option<String>,
        },
    }

    impl Default for SlashCommandRequest {
        fn default() -> Self {
            Self::Summarize { prompt: None }
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ClientIdentifiers {
        pub conversation_id: crate::ai::agent::conversation::AIConversationId,
        pub client_exchange_id: crate::ai::agent::AIAgentExchangeId,
        pub response_stream_id: Option<String>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct PersistedAIInput {
        pub start_ts: chrono::DateTime<chrono::Local>,
        pub inputs: Vec<PersistedAIInputType>,
        pub exchange_id: crate::ai::agent::AIAgentExchangeId,
        pub conversation_id: crate::ai::agent::conversation::AIConversationId,
        pub output_status: AIQueryHistoryOutputStatus,
        pub working_directory: Option<String>,
        pub model_id: String,
        pub coding_model_id: String,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum PersistedAIInputType {
        #[default]
        Unknown,
    }

    impl TryFrom<String> for PersistedAIInputType {
        type Error = ();

        fn try_from(_value: String) -> Result<Self, Self::Error> {
            Ok(Self::Unknown)
        }
    }

    impl TryFrom<&String> for PersistedAIInputType {
        type Error = ();

        fn try_from(_value: &String) -> Result<Self, Self::Error> {
            Ok(Self::Unknown)
        }
    }

    impl TryFrom<&crate::ai::agent::AIAgentInput> for PersistedAIInputType {
        type Error = ();

        fn try_from(_value: &crate::ai::agent::AIAgentInput) -> Result<Self, Self::Error> {
            Ok(Self::Unknown)
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AIQueryHistoryOutputStatus {
        #[default]
        Unknown,
    }

    impl From<&AIBlockOutputStatus> for AIQueryHistoryOutputStatus {
        fn from(_value: &AIBlockOutputStatus) -> Self {
            Self::Unknown
        }
    }

    impl AIQueryHistoryOutputStatus {
        pub fn icon(&self) -> crate::ui_components::icons::Icon {
            crate::ui_components::icons::Icon::Terminal
        }

        pub fn display_text(&self) -> &'static str {
            "Local"
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIBlock;

    impl AIBlock {
        pub fn find_undismissed_code_diff<T: ?Sized>(&self, _ctx: &T) -> Option<()> {
            None
        }

        pub fn pending_unit_test_suggestion<T: ?Sized>(
            &self,
            _ctx: &T,
        ) -> Option<warpui::ModelHandle<PendingUnitTestSuggestion>> {
            None
        }

        pub fn is_passive_conversation<T: ?Sized>(&self, _ctx: &T) -> bool {
            false
        }

        pub fn selected_text<T: ?Sized>(&self, _ctx: &T) -> Option<String> {
            None
        }

        pub fn is_finished(&self) -> bool {
            true
        }

        pub fn handle_passive_code_diff_action<T, U>(&mut self, _action: T, _ctx: U) -> bool {
            false
        }

        pub fn accept_pending_unit_test_suggestion<T, U>(&mut self, _source: T, _ctx: U) -> bool {
            false
        }

        pub fn dismiss_pending_suggested_prompt<T, U>(&mut self, _source: T, _ctx: U) -> bool {
            false
        }

        pub fn conversation_id(&self) -> crate::ai::agent::conversation::AIConversationId {
            crate::ai::agent::conversation::AIConversationId::new()
        }

        pub fn finish_reason(&self) -> crate::ai::blocklist::block::FinishReason {
            crate::ai::blocklist::block::FinishReason::Unknown
        }

        pub fn output_status(&self) -> crate::ai::blocklist::AIBlockOutputStatus {
            crate::ai::blocklist::AIBlockOutputStatus::Unknown
        }

        pub fn has_user_input<T>(&self, _ctx: T) -> bool {
            false
        }

        #[allow(clippy::too_many_arguments)]
        pub fn new<A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R>(
            _model: A,
            _terminal_model: B,
            _client_identifiers: C,
            _ai_controller: D,
            _get_relevant_files_controller: E,
            _pwd: F,
            _shell_launch_data: G,
            _ai_action_model: H,
            _ai_context_model: I,
            _find_model: J,
            _active_session: K,
            _ambient_agent_view_model: L,
            _cli_subagent_controller: M,
            _model_events_handle: N,
            _agent_view_controller: O,
            _terminal_view: P,
            _terminal_view_id: Q,
            _ctx: R,
        ) -> Self {
            Self
        }

        pub fn reset_conversation_id<T, U, V>(&mut self, _id: T, _model: U, _ctx: V) {}

        pub fn update_directory_context<T, U>(&mut self, _context: T, _ctx: U) {}

        pub fn handle_action<T, U>(&mut self, _action: T, _ctx: U) {}

        pub fn contains_action_result<T>(&self, _action_id: T) -> bool {
            false
        }

        pub fn requested_command_copied_from_doc<T, U>(
            &self,
            _action_id: T,
            _ctx: U,
        ) -> Option<()> {
            None
        }

        pub fn on_requested_command_execution_started<T, U>(&mut self, _action_id: T, _ctx: U) {}

        pub fn is_blocked_on_user_confirmation<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn has_expanded_running_commands<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn try_steal_focus<T>(&mut self, _ctx: T) {}

        pub fn dismiss_ai_tooltips<T>(&mut self, _ctx: T) {}

        pub fn response_stream_id(&self) -> Option<&String> {
            None
        }

        pub fn cleanup_block<T>(&mut self, _ctx: T) {}

        pub fn status<T: ?Sized>(&self, _ctx: &T) -> crate::ai::blocklist::AIBlockOutputStatus {
            crate::ai::blocklist::AIBlockOutputStatus::Unknown
        }

        pub fn is_hidden<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn ignore_passive_actions<T>(&mut self, _ctx: T) {}

        pub fn hovered_rich_content_link(&self) -> Option<crate::terminal::view::RichContentLink> {
            None
        }

        pub fn get_preceding_user_query<T: ?Sized>(&self, _ctx: &T) -> String {
            String::new()
        }

        pub fn is_restored(&self) -> bool {
            false
        }

        pub fn num_requested_commands(&self) -> usize {
            0
        }

        pub fn requested_commands_iter(
            &self,
        ) -> std::iter::Empty<(&'static crate::ai::agent::AIAgentActionId, &'static ())> {
            std::iter::empty()
        }

        pub fn server_output_id<T: ?Sized>(
            &self,
            _ctx: &T,
        ) -> Option<crate::ai::agent::ServerOutputId> {
            None
        }

        pub fn set_pending_context_selected_text<T, U>(&mut self, _text: T, _ctx: U) {}

        pub fn set_secret_redaction_state<T, U, V>(&mut self, _location: T, _range: U, _state: V) {}

        pub fn start_selection_at_max_point<T, U>(&self, _selection_type: T, _x_pos: U) {}

        pub fn start_selection_at_min_point<T, U>(&self, _selection_type: T, _x_pos: U) {}

        pub fn clear_all_selections<T>(&mut self, _ctx: T) {}

        pub fn collect_imported_comments(
            &self,
        ) -> Option<crate::ai::agent::AgentReviewCommentBatch> {
            None
        }

        pub fn has_any_imported_comments(&self) -> bool {
            false
        }

        pub fn requested_command_action_id(&self) -> Option<crate::ai::agent::AIAgentActionId> {
            None
        }

        pub fn revert_all_diffs<T>(&mut self, _ctx: T) {}

        pub fn set_shell_launch_data<T, U>(&mut self, _data: T, _ctx: U) {}

        pub fn accept_pending_action<T>(&mut self, _ctx: T) -> bool {
            false
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct PendingUnitTestSuggestion;

    impl PendingUnitTestSuggestion {
        pub fn is_keybindings_hidden(&self) -> bool {
            true
        }
    }

    impl Entity for PendingUnitTestSuggestion {
        type Event = ();
    }

    impl warpui::Entity for AIBlock {
        type Event = AIBlockEvent;
    }

    impl warpui::View for AIBlock {
        fn ui_name() -> &'static str {
            "RemovedAIBlock"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIBlockModelImpl<T = ()>(std::marker::PhantomData<T>);

    impl<T> AIBlockModelImpl<T> {
        pub fn new<A, B, C, D, E>(
            _exchange_id: A,
            _conversation_id: B,
            _is_restored: C,
            _is_hidden: D,
            _ctx: E,
        ) -> anyhow::Result<Self> {
            Ok(Self(std::marker::PhantomData))
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum AIBlockOutputStatus {
        #[default]
        Unknown,
        Streaming,
        Failed {
            error_message: String,
        },
    }

    impl AIBlockOutputStatus {
        pub fn is_streaming(&self) -> bool {
            matches!(self, Self::Streaming)
        }

        pub fn output_to_render(&self) -> Option<crate::ai::agent::FinishedAIAgentOutput> {
            None
        }

        pub fn server_output_id(&self) -> Option<crate::ai::agent::ServerOutputId> {
            None
        }

        pub fn cancel_reason(&self) -> Option<crate::ai::agent::CancellationReason> {
            None
        }
    }

    #[derive(Clone, Debug)]
    pub enum BlocklistAIHistoryEvent {
        StartedNewConversation {
            terminal_view_id: warpui::EntityId,
            new_conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        AppendedExchange {
            terminal_view_id: warpui::EntityId,
            exchange_id: crate::ai::agent::AIAgentExchangeId,
            task_id: crate::ai::agent::task::TaskId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
            is_hidden: bool,
            response_stream_id: Option<String>,
        },
        UpdatedStreamingExchange {
            terminal_view_id: warpui::EntityId,
            exchange_id: crate::ai::agent::AIAgentExchangeId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
            is_hidden: bool,
        },
        ReassignedExchange {
            terminal_view_id: warpui::EntityId,
            exchange_id: crate::ai::agent::AIAgentExchangeId,
            new_conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        SetActiveConversation {
            terminal_view_id: warpui::EntityId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        ClearedActiveConversation {
            terminal_view_id: warpui::EntityId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        ClearedConversationsInTerminalView {
            terminal_view_id: warpui::EntityId,
            active_conversation_id: Option<crate::ai::agent::conversation::AIConversationId>,
        },
        SplitConversation {
            terminal_view_id: warpui::EntityId,
        },
        RestoredConversations {
            terminal_view_id: Option<warpui::EntityId>,
        },
        UpdatedConversationStatus {
            terminal_view_id: warpui::EntityId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
            is_restored: bool,
        },
        UpdatedTodoList {
            terminal_view_id: warpui::EntityId,
        },
        UpdatedAutoexecuteOverride {
            terminal_view_id: warpui::EntityId,
        },
        CreatedSubtask {
            terminal_view_id: warpui::EntityId,
        },
        RemoveConversation {
            terminal_view_id: warpui::EntityId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        DeletedConversation {
            terminal_view_id: warpui::EntityId,
        },
        UpgradedTask {
            terminal_view_id: warpui::EntityId,
        },
        UpdatedConversationMetadata {
            terminal_view_id: Option<warpui::EntityId>,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        UpdatedConversationArtifacts {
            terminal_view_id: warpui::EntityId,
        },
        ConversationServerTokenAssigned {
            terminal_view_id: warpui::EntityId,
        },
    }

    impl BlocklistAIHistoryEvent {
        pub fn terminal_view_id(&self) -> Option<warpui::EntityId> {
            match self {
                Self::StartedNewConversation {
                    terminal_view_id, ..
                }
                | Self::AppendedExchange {
                    terminal_view_id, ..
                }
                | Self::UpdatedStreamingExchange {
                    terminal_view_id, ..
                }
                | Self::ReassignedExchange {
                    terminal_view_id, ..
                }
                | Self::SetActiveConversation {
                    terminal_view_id, ..
                }
                | Self::ClearedActiveConversation {
                    terminal_view_id, ..
                }
                | Self::ClearedConversationsInTerminalView {
                    terminal_view_id, ..
                }
                | Self::SplitConversation {
                    terminal_view_id, ..
                }
                | Self::UpdatedConversationStatus {
                    terminal_view_id, ..
                }
                | Self::UpdatedTodoList {
                    terminal_view_id, ..
                }
                | Self::UpdatedAutoexecuteOverride {
                    terminal_view_id, ..
                }
                | Self::CreatedSubtask {
                    terminal_view_id, ..
                }
                | Self::RemoveConversation {
                    terminal_view_id, ..
                }
                | Self::DeletedConversation {
                    terminal_view_id, ..
                }
                | Self::UpgradedTask {
                    terminal_view_id, ..
                }
                | Self::UpdatedConversationArtifacts {
                    terminal_view_id, ..
                }
                | Self::ConversationServerTokenAssigned {
                    terminal_view_id, ..
                } => Some(*terminal_view_id),
                Self::RestoredConversations { terminal_view_id }
                | Self::UpdatedConversationMetadata {
                    terminal_view_id, ..
                } => *terminal_view_id,
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum BlocklistAIContextEvent {
        UpdatedPendingContext {
            terminal_view_id: Option<warpui::EntityId>,
            previous_block_ids: std::collections::HashSet<crate::terminal::model::block::BlockId>,
            requires_block_resync: bool,
            requires_text_resync: bool,
        },
        PendingQueryStateUpdated,
        QueueNextPromptToggled,
    }

    #[derive(Clone, Debug)]
    pub enum BlocklistAIInputEvent {
        InputTypeChanged { config: InputConfig },
        LockChanged { config: InputConfig },
    }

    impl BlocklistAIInputEvent {
        pub fn updated_config(&self) -> &InputConfig {
            match self {
                Self::InputTypeChanged { config } | Self::LockChanged { config } => config,
            }
        }

        pub fn did_update_input_config(&self) -> bool {
            true
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct BlocklistAIHistoryModel;

    impl BlocklistAIHistoryModel {
        pub fn new<T, U>(_ai_queries: T, _conversations: U) -> Self {
            Self
        }

        pub fn reset(&mut self) {}

        pub fn start_new_child_conversation<T>(
            &mut self,
            _terminal_view_id: warpui::EntityId,
            _name: T,
            _parent_conversation_id: crate::ai::agent::conversation::AIConversationId,
            _ctx: &mut ModelContext<Self>,
        ) -> crate::ai::agent::conversation::AIConversationId {
            crate::ai::agent::conversation::AIConversationId::new()
        }

        pub fn update_conversation_status_with_error_message<T>(
            &mut self,
            _terminal_view_id: warpui::EntityId,
            _conversation_id: crate::ai::agent::conversation::AIConversationId,
            _status: crate::ai::agent::conversation::ConversationStatus,
            _error_message: Option<T>,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn update_conversation_status(
            &mut self,
            _terminal_view_id: warpui::EntityId,
            _conversation_id: crate::ai::agent::conversation::AIConversationId,
            _status: crate::ai::agent::conversation::ConversationStatus,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn assign_run_id_for_conversation<T, U>(
            &mut self,
            _conversation_id: crate::ai::agent::conversation::AIConversationId,
            _run_id: T,
            _task_id: Option<U>,
            _terminal_view_id: warpui::EntityId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn clear_conversations_in_terminal_view(
            &mut self,
            _terminal_view_id: warpui::EntityId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn terminal_view_id_for_conversation(
            &self,
            _conversation_id: &crate::ai::agent::conversation::AIConversationId,
        ) -> Option<warpui::EntityId> {
            None
        }

        pub fn child_conversations_of(
            &self,
            _parent_id: crate::ai::agent::conversation::AIConversationId,
        ) -> std::vec::IntoIter<crate::ai::agent::conversation::AIConversation> {
            Vec::new().into_iter()
        }

        pub fn child_conversation_ids_of<'a>(
            &'a self,
            _parent_id: &crate::ai::agent::conversation::AIConversationId,
        ) -> std::vec::IntoIter<&'a crate::ai::agent::conversation::AIConversationId> {
            Vec::new().into_iter()
        }

        pub fn find_conversation_id_by_server_token<T>(
            &self,
            _token: T,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn conversation_id_for_action<T>(
            &self,
            _action_id: T,
            _terminal_view_id: warpui::EntityId,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn load_conversation_data<T, U: ?Sized>(
            &self,
            _id: T,
            _ctx: &U,
        ) -> std::future::Ready<Option<crate::ai::blocklist::history_model::CloudConversationData>>
        {
            std::future::ready(None)
        }

        pub fn mark_terminal_view_as_conversation_transcript_viewer(
            &mut self,
            _terminal_view_id: warpui::EntityId,
        ) {
        }

        pub fn is_terminal_view_conversation_transcript_viewer(
            &self,
            _terminal_view_id: warpui::EntityId,
        ) -> bool {
            false
        }

        pub fn mark_terminal_view_as_ambient_agent_session_view(
            &mut self,
            _terminal_view_id: warpui::EntityId,
        ) {
        }

        pub fn last_conversation_id(
            &self,
            _terminal_id: warpui::EntityId,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn active_conversation(
            &self,
            _terminal_id: warpui::EntityId,
        ) -> Option<&crate::ai::agent::conversation::AIConversation> {
            None
        }

        pub fn conversation<T>(
            &self,
            _id: T,
        ) -> Option<&crate::ai::agent::conversation::AIConversation> {
            None
        }

        pub fn conversation_mut<T>(
            &mut self,
            _id: T,
        ) -> Option<&mut crate::ai::agent::conversation::AIConversation> {
            None
        }

        pub fn active_conversation_id(
            &self,
            _terminal_id: warpui::EntityId,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn all_live_conversations_for_terminal_view(
            &self,
            _terminal_id: warpui::EntityId,
        ) -> std::iter::Empty<&'static crate::ai::agent::conversation::AIConversation> {
            std::iter::empty()
        }

        pub fn all_live_conversations(
            &self,
        ) -> std::iter::Empty<(
            warpui::EntityId,
            &'static crate::ai::agent::conversation::AIConversation,
        )> {
            std::iter::empty()
        }

        pub fn set_active_conversation_id(
            &mut self,
            _id: crate::ai::agent::conversation::AIConversationId,
            _terminal_id: warpui::EntityId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn mark_conversations_historical_for_terminal_view(
            &mut self,
            _terminal_id: warpui::EntityId,
        ) {
        }

        pub fn can_conversation_be_shared(
            &self,
            _id: &crate::ai::agent::conversation::AIConversationId,
        ) -> bool {
            false
        }

        pub fn all_ai_queries(
            &self,
            _terminal_view_id: Option<warpui::EntityId>,
        ) -> std::vec::IntoIter<AIQueryHistory> {
            Vec::new().into_iter()
        }

        pub fn get_server_conversation_metadata(
            &self,
            _id: &crate::ai::agent::conversation::AIConversationId,
        ) -> Option<crate::ai::conversation_navigation::ConversationNavigationData> {
            None
        }

        pub fn set_server_metadata_for_conversation(
            &mut self,
            _id: crate::ai::agent::conversation::AIConversationId,
            _metadata: crate::ai::conversation_navigation::ConversationNavigationData,
        ) {
        }

        pub fn fork_conversation<T, U, V>(
            &mut self,
            _conversation: T,
            _prefix: U,
            _ctx: V,
        ) -> anyhow::Result<crate::ai::agent::conversation::AIConversation> {
            Ok(crate::ai::agent::conversation::AIConversation::default())
        }

        pub fn fork_conversation_at_exchange<T, U, V, W, X>(
            &mut self,
            _conversation_id: T,
            _exchange_id: U,
            _prefix: V,
            _include_exchange: W,
            _ctx: X,
        ) -> anyhow::Result<crate::ai::agent::conversation::AIConversation> {
            Ok(crate::ai::agent::conversation::AIConversation::default())
        }

        pub fn truncate_conversation_from_exchange<T, U, V>(
            &mut self,
            _conversation_id: T,
            _exchange_id: U,
            _ctx: V,
        ) -> anyhow::Result<std::collections::HashSet<crate::ai::agent::AIAgentExchangeId>>
        {
            Ok(std::collections::HashSet::new())
        }

        pub fn set_has_code_review_opened_to_true<T>(&mut self, _id: T) {}

        pub fn start_new_conversation<T, U, V>(
            &mut self,
            _terminal_view_id: T,
            _origin: U,
            _ctx: V,
        ) -> crate::ai::agent::conversation::AIConversationId {
            crate::ai::agent::conversation::AIConversationId::new()
        }
    }

    impl Entity for BlocklistAIHistoryModel {
        type Event = BlocklistAIHistoryEvent;
    }

    impl SingletonEntity for BlocklistAIHistoryModel {}

    #[derive(Clone, Debug, Default)]
    pub struct BlocklistAIPermissions;

    impl BlocklistAIPermissions {
        pub fn new<T>(_ctx: T) -> Self {
            Self
        }
    }

    impl Entity for BlocklistAIPermissions {
        type Event = ();
    }

    impl SingletonEntity for BlocklistAIPermissions {}

    #[derive(Clone, Debug, Default)]
    pub struct BlocklistAIContextModel;

    impl BlocklistAIContextModel {
        pub fn new<A, B, C, D, E, F>(
            _sessions: A,
            _model_events_handle: B,
            _terminal_model: C,
            _terminal_view_id: D,
            _agent_view_controller: E,
            _ctx: F,
        ) -> Self {
            Self
        }

        pub fn selected_conversation<T: ?Sized>(
            &self,
            _ctx: &T,
        ) -> Option<&crate::ai::agent::conversation::AIConversation> {
            None
        }

        pub fn selected_conversation_id<T: ?Sized>(
            &self,
            _ctx: &T,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn can_start_new_conversation(&self) -> bool {
            true
        }

        pub fn pending_context_block_ids(
            &self,
        ) -> std::collections::HashSet<crate::terminal::model::block::BlockId> {
            std::collections::HashSet::new()
        }

        pub fn pending_context_selected_text(&self) -> Option<&String> {
            None
        }

        pub fn pending_attachments(&self) -> &[PendingAttachment] {
            &[]
        }

        pub fn pending_images(&self) -> Vec<&crate::ai::agent::ImageContext> {
            Vec::new()
        }

        pub fn pending_files(&self) -> Vec<&PendingFile> {
            Vec::new()
        }

        pub fn pending_context<T: ?Sized>(
            &self,
            _ctx: &T,
            _is_user_query: bool,
        ) -> std::vec::IntoIter<crate::ai::agent::AIAgentContext> {
            Vec::new().into_iter()
        }

        pub fn set_pending_context_block_ids<T, U>(
            &mut self,
            _ids: T,
            _replace_existing: bool,
            _ctx: U,
        ) {
        }

        pub fn reset_context_to_default<T>(&mut self, _ctx: T) {}

        pub fn clear_pending_attachments<T>(&mut self, _ctx: T) {}

        pub fn clear_pending_images<T>(&mut self, _ctx: T) {}

        pub fn remove_last_pending_images<T, U>(&mut self, _count: T, _ctx: U) -> usize {
            0
        }

        pub fn set_pending_query_state_for_new_conversation<T, U>(&mut self, _origin: T, _ctx: U) {}

        pub fn pending_query_autoexecute_override<T: ?Sized>(
            &self,
            _ctx: &T,
        ) -> PendingQueryAutoexecuteOverride {
            PendingQueryAutoexecuteOverride
        }

        pub fn toggle_pending_query_autoexecute<T: ?Sized>(&mut self, _ctx: &T) {}

        pub fn set_pending_query_state_for_existing_conversation<T, U, V>(
            &mut self,
            _conversation_id: T,
            _origin: U,
            _ctx: V,
        ) {
        }

        pub fn is_queue_next_prompt_enabled(&self) -> bool {
            false
        }

        pub fn is_targeting_existing_conversation(&self) -> bool {
            false
        }

        pub fn selected_conversation_status_for_hint<T>(
            &self,
            _ctx: T,
        ) -> Option<crate::ai::agent::conversation::ConversationStatus> {
            None
        }

        pub fn register_diff_hunk_attachment(
            &mut self,
            _attachment_key: String,
            _attachment: crate::ai::agent::AIAgentAttachment,
        ) {
        }

        pub fn append_pending_images<T, U>(&mut self, _images: T, _ctx: U) {}

        pub fn append_pending_attachments<T, U>(&mut self, _attachments: T, _ctx: U) {}

        pub fn current_pwd(&self) -> Option<String> {
            None
        }

        pub fn home_directory(&self) -> Option<std::path::PathBuf> {
            None
        }

        pub fn set_pending_context_selected_text<T, U, V>(
            &mut self,
            _text: Option<T>,
            _replace: U,
            _ctx: V,
        ) {
        }

        pub fn remove_pending_attachment<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn toggle_queue_next_prompt<T>(&mut self, _ctx: T) {}
    }

    impl Entity for BlocklistAIContextModel {
        type Event = BlocklistAIContextEvent;
    }

    impl SingletonEntity for BlocklistAIContextModel {}

    #[derive(Clone, Copy, Debug, Default)]
    pub struct PendingQueryAutoexecuteOverride;

    impl PendingQueryAutoexecuteOverride {
        pub fn is_autoexecute_any_action(&self) -> bool {
            false
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct BlocklistAIInputModel;

    impl BlocklistAIInputModel {
        pub fn new<A, B, C, D: ?Sized>(
            _terminal_model: A,
            _agent_view_controller: B,
            _terminal_view_id: C,
            _ctx: &mut D,
        ) -> Self {
            Self
        }

        pub fn input_type(&self) -> InputType {
            InputType::Shell
        }

        pub fn is_ai_input_enabled(&self) -> bool {
            false
        }

        pub fn is_input_type_locked(&self) -> bool {
            false
        }

        pub fn should_run_input_autodetection<T: ?Sized>(&self, _ctx: &T) -> bool {
            false
        }

        pub fn is_autodetection_enabled_for_current_context<T: ?Sized>(&self, _ctx: &T) -> bool {
            false
        }

        pub fn was_lock_set_with_empty_buffer(&self) -> bool {
            false
        }

        pub fn last_ai_autodetection_ts(&self) -> Option<chrono::DateTime<chrono::Utc>> {
            None
        }

        pub fn detect_and_set_input_type<T, U, W>(
            &mut self,
            _parsed_tokens: T,
            _completion_context: U,
            _session_id: Option<warp_core::SessionId>,
            _ctx: W,
        ) {
        }

        pub fn set_input_type<T, U>(&mut self, _input_type: T, _ctx: U) {}

        pub fn input_config(&self) -> InputConfig {
            InputConfig::default()
        }

        pub fn set_input_config<U>(
            &mut self,
            _config: InputConfig,
            _is_buffer_empty: bool,
            _ctx: U,
        ) {
        }

        pub fn set_input_config_for_classic_mode<T>(&mut self, _config: InputConfig, _ctx: T) {}

        pub fn enable_autodetection<T, U>(&mut self, _input_type: T, _ctx: U) {}

        pub fn abort_in_progress_detection(&mut self) {}

        pub fn handle_input_buffer_submitted<T>(&mut self, _ctx: T) {}
    }

    impl Entity for BlocklistAIInputModel {
        type Event = BlocklistAIInputEvent;
    }

    impl SingletonEntity for BlocklistAIInputModel {}

    pub fn block_context_from_terminal_model<T, U, V>(
        _model: &T,
        _block_id: U,
        _include_output: V,
    ) -> Option<crate::ai::block_context::BlockContext> {
        None
    }

    pub mod model {
        pub use super::{AIBlockModelImpl, AIBlockOutputStatus};
    }

    pub use block::TextLocation;

    pub mod usage {
        pub mod conversation_usage_view {
            use warpui::{elements::Empty, Entity, View};

            #[derive(Clone, Debug, Default)]
            pub struct ConversationUsageInfo {
                pub credits_spent: Option<f32>,
                pub credits_spent_for_last_block: Option<f32>,
                pub tool_calls: i32,
                pub models: Vec<crate::persistence::model::ModelTokenUsage>,
                pub context_window_usage: Option<f32>,
                pub files_changed: i32,
                pub lines_added: i32,
                pub lines_removed: i32,
                pub commands_executed: i32,
            }

            impl From<warp_graphql::queries::get_conversation_usage::ConversationUsage>
                for ConversationUsageInfo
            {
                fn from(
                    _value: warp_graphql::queries::get_conversation_usage::ConversationUsage,
                ) -> Self {
                    Self::default()
                }
            }

            #[derive(Clone, Debug, Default)]
            pub struct TimingInfo {
                pub time_to_first_token_ms: Option<u64>,
                pub total_agent_response_time_ms: Option<u64>,
                pub wall_to_wall_response_time_ms: Option<u64>,
            }

            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
            pub enum DisplayMode {
                #[default]
                Compact,
                Full,
                Footer,
                Settings,
            }

            #[derive(Clone, Debug, Default)]
            pub struct ConversationUsageView;

            impl ConversationUsageView {
                pub fn new<A, B, C, D>(
                    _info: A,
                    _display_mode: B,
                    _timing_info: C,
                    _mouse_state: D,
                ) -> Self {
                    Self
                }
            }

            impl Entity for ConversationUsageView {
                type Event = ();
            }

            impl View for ConversationUsageView {
                fn ui_name() -> &'static str {
                    "ConversationUsageView"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(Empty::new())
                }
            }
        }
    }

    pub mod history_model {
        pub use super::BlocklistAIHistoryModel;
        use crate::ai::agent::conversation::AIAgentHarness;
        use crate::ai::agent::conversation::AIConversation;
        use crate::terminal::model::block::SerializedBlock;

        #[derive(Clone, Debug, Default)]
        pub struct CLIAgentConversationMetadata {
            pub working_directory: Option<String>,
            pub ambient_agent_task_id: Option<crate::ai::ambient_agents::AmbientAgentTaskId>,
            pub harness: AIAgentHarness,
        }

        #[derive(Clone, Debug, Default)]
        pub struct CLIAgentConversation {
            pub metadata: CLIAgentConversationMetadata,
            pub block: SerializedBlock,
        }

        #[derive(Clone, Debug)]
        pub enum CloudConversationData {
            Oz(Box<AIConversation>),
            CLIAgent(Box<CLIAgentConversation>),
        }

        pub async fn load_conversation_from_server<T, U, V>(
            _id: T,
            _token: U,
            _client: V,
        ) -> Option<CloudConversationData> {
            None
        }
    }

    pub mod agent_view {
        use super::*;

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum AgentViewDisplayMode {
            #[default]
            Inline,
            FullScreen,
        }

        impl AgentViewDisplayMode {
            pub fn is_fullscreen(&self) -> bool {
                matches!(self, Self::FullScreen)
            }
        }

        #[derive(Clone, Debug, PartialEq, Eq)]
        pub enum AgentViewState {
            Inactive,
            Active {
                conversation_id: crate::ai::agent::conversation::AIConversationId,
                origin: AgentViewEntryOrigin,
                display_mode: AgentViewDisplayMode,
            },
        }

        impl Default for AgentViewState {
            fn default() -> Self {
                Self::Inactive
            }
        }

        impl AgentViewState {
            pub fn is_active(&self) -> bool {
                matches!(self, Self::Active { .. })
            }

            pub fn fullscreen_conversation_id(
                &self,
            ) -> Option<crate::ai::agent::conversation::AIConversationId> {
                match self {
                    Self::Active {
                        conversation_id,
                        display_mode: AgentViewDisplayMode::FullScreen,
                        ..
                    } => Some(*conversation_id),
                    _ => None,
                }
            }
        }

        impl AgentViewState {
            pub fn active_conversation_id(
                &self,
            ) -> Option<crate::ai::agent::conversation::AIConversationId> {
                match self {
                    Self::Active {
                        conversation_id, ..
                    } => Some(*conversation_id),
                    Self::Inactive => None,
                }
            }

            pub fn display_mode(&self) -> Option<AgentViewDisplayMode> {
                match self {
                    Self::Active { display_mode, .. } => Some(*display_mode),
                    Self::Inactive => None,
                }
            }

            pub fn is_inline(&self) -> bool {
                self.display_mode() == Some(AgentViewDisplayMode::Inline)
            }

            pub fn is_fullscreen(&self) -> bool {
                self.display_mode() == Some(AgentViewDisplayMode::FullScreen)
            }

            pub fn is_new(&self) -> bool {
                false
            }

            pub fn zero_state_position_id(&self) -> Option<String> {
                None
            }
        }

        #[derive(Clone, Debug, Default)]
        pub struct AgentViewController;

        impl AgentViewController {
            pub fn new<T, U, V, W>(
                _terminal_model: T,
                _terminal_view_id: warpui::EntityId,
                _ambient_agent_view_model: U,
                _ephemeral_message_model: V,
                _ctx: W,
            ) -> Self {
                Self
            }

            pub fn agent_view_state(&self) -> AgentViewState {
                AgentViewState::Inactive
            }

            pub fn is_inline(&self) -> bool {
                false
            }

            pub fn is_fullscreen(&self) -> bool {
                false
            }

            pub fn is_active(&self) -> bool {
                false
            }

            pub fn exit_agent_view(&mut self, _ctx: &mut ModelContext<Self>) {}

            pub fn set_pane_group_id<T>(&mut self, _pane_group_id: T) {}

            pub fn pane_group_id(&self) -> Option<warpui::EntityId> {
                None
            }

            pub fn clear_pending_exit_confirmation<T>(&mut self, _ctx: T) {}

            pub fn exit_agent_view_with_required_confirmation<T, U>(
                &mut self,
                _trigger: T,
                _ctx: U,
            ) {
            }

            pub fn can_exit_agent_view<T: ?Sized>(
                &self,
                _ctx: &T,
            ) -> Result<(), EnterAgentViewError> {
                Ok(())
            }

            pub fn try_enter_agent_view<U>(
                &mut self,
                _conversation_id: Option<crate::ai::agent::conversation::AIConversationId>,
                _origin: AgentViewEntryOrigin,
                _ctx: U,
            ) -> Result<crate::ai::agent::conversation::AIConversationId, EnterAgentViewError>
            {
                Ok(crate::ai::agent::conversation::AIConversationId::new())
            }

            pub fn try_enter_inline_agent_view<U, V>(
                &mut self,
                _conversation_id: Option<crate::ai::agent::conversation::AIConversationId>,
                _origin: U,
                _ctx: V,
            ) -> Result<crate::ai::agent::conversation::AIConversationId, EnterAgentViewError>
            {
                Ok(crate::ai::agent::conversation::AIConversationId::new())
            }

            pub fn should_start_new_conversation_for_keybinding<T, U>(
                &self,
                _command: T,
                _ctx: U,
            ) -> bool {
                true
            }
        }

        impl Entity for AgentViewController {
            type Event = AgentViewControllerEvent;
        }

        #[derive(Clone, Debug)]
        pub enum AgentViewControllerEvent {
            EnteredAgentView {
                conversation_id: crate::ai::agent::conversation::AIConversationId,
                origin: AgentViewEntryOrigin,
                display_mode: AgentViewDisplayMode,
                is_new: bool,
            },
            ExitedAgentView {
                conversation_id: crate::ai::agent::conversation::AIConversationId,
                origin: AgentViewEntryOrigin,
                display_mode: AgentViewDisplayMode,
                original_exchange_count: usize,
                final_exchange_count: usize,
                was_ambient_agent: bool,
                is_exit_before_new_entrance: bool,
            },
            ExitConfirmed {
                conversation_id: crate::ai::agent::conversation::AIConversationId,
            },
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum AgentViewEntryOrigin {
            #[default]
            Unknown,
            Onboarding,
            OnboardingCallout,
            LongRunningCommand,
            Input {
                was_prompt_autodetected: bool,
            },
            ConversationSelector,
            AgentModeHomepage,
            AgentViewBlock,
            AIDocument,
            AutoFollowUp,
            RestoreExistingConversation,
            SharedSessionSelection,
            AgentRequestedNewConversation,
            AcceptedPromptSuggestion,
            AcceptedUnitTestSuggestion,
            AcceptedPassiveCodeDiff,
            InlineCodeReview,
            CloudAgent,
            ThirdPartyCloudAgent,
            Cli,
            ImageAdded,
            SlashCommand {
                trigger: crate::terminal::input::slash_commands::SlashCommandTrigger,
            },
            CodeReviewContext,
            ContinueConversationButton,
            ViewPassiveCodeDiffDetails,
            ResumeConversationButton,
            CodexModal,
            InlineHistoryMenu,
            InlineConversationMenu,
            PromptChip,
            ConversationListView,
            Keybinding,
            SlashInit,
            CreateEnvironment,
            ProjectEntry,
            ClearBuffer,
            DefaultSessionMode,
            ChildAgent,
            LinearDeepLink,
        }

        impl AgentViewEntryOrigin {
            pub fn is_cloud_agent(&self) -> bool {
                matches!(self, Self::CloudAgent | Self::ThirdPartyCloudAgent)
            }

            pub fn should_autotrigger_request(&self) -> AutoTriggerBehavior {
                AutoTriggerBehavior::Never
            }
        }

        pub const ENTER_OR_EXIT_CONFIRMATION_WINDOW: std::time::Duration =
            std::time::Duration::from_millis(0);

        pub fn agent_view_bg_fill<T>(_appearance: T) -> crate::themes::theme::Fill {
            crate::themes::theme::Fill::Solid(pathfinder_color::ColorU::new(0, 0, 0, 0))
        }

        pub fn agent_view_bg_color<T>(_appearance: T) -> pathfinder_color::ColorU {
            pathfinder_color::ColorU::new(0, 0, 0, 0)
        }

        #[derive(Clone, Debug, Default)]
        pub struct AgentMessageBarMouseStates {
            pub clear_attached_context: warpui::elements::MouseStateHandle,
        }

        #[derive(Clone, Debug, Default)]
        pub struct EphemeralMessage;

        impl EphemeralMessage {
            pub fn new<T, U>(_message: T, _dismissal: U) -> Self {
                Self
            }

            pub fn with_id<T>(self, _id: T) -> Self {
                self
            }

            pub fn id(&self) -> Option<&'static str> {
                None
            }

            pub fn with_duration(self, _duration: std::time::Duration) -> Self {
                self
            }
        }

        #[derive(Clone, Debug, Default)]
        pub struct EphemeralMessageModel;

        impl EphemeralMessageModel {
            pub fn new() -> Self {
                Self
            }

            pub fn show_ephemeral_message<T, U>(&mut self, _message: T, _ctx: U) {}

            pub fn clear_ephemeral_message<T>(&mut self, _ctx: T) {}

            pub fn current_message(&self) -> Option<EphemeralMessage> {
                None
            }

            pub fn clear_message<T>(&mut self, _ctx: T) {}
        }

        impl warpui::Entity for EphemeralMessageModel {
            type Event = ();
        }

        #[derive(Clone, Debug, Default)]
        pub struct DismissalStrategy;

        impl DismissalStrategy {
            #[allow(non_upper_case_globals)]
            pub const Timer: fn(std::time::Duration) -> Self = |_| Self;
        }

        #[derive(Clone, Debug, Default)]
        pub struct ExitConfirmationTrigger;

        impl ExitConfirmationTrigger {
            #[allow(non_upper_case_globals)]
            pub const CtrlC: Self = Self;
            #[allow(non_upper_case_globals)]
            pub const Escape: Self = Self;
        }

        #[derive(Clone, Debug, Default)]
        pub struct AgentViewHeaderTheme;

        #[derive(Clone, Debug, Default)]
        pub struct AgentViewZeroStateBlock;

        impl AgentViewZeroStateBlock {
            #[allow(clippy::too_many_arguments)]
            pub fn new<A, B, C, D, E, F, G, H, I>(
                _conversation_id: A,
                _origin: B,
                _controller: C,
                _sessions: D,
                _ambient_agent_view_model: E,
                _terminal_model: F,
                _model_events_handle: G,
                _should_show_init_callout: H,
                _ctx: I,
            ) -> Self {
                Self
            }
        }

        impl warpui::Entity for AgentViewZeroStateBlock {
            type Event = AgentViewZeroStateEvent;
        }

        impl warpui::View for AgentViewZeroStateBlock {
            fn ui_name() -> &'static str {
                "AgentViewZeroStateBlock"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl warpui::TypedActionView for AgentViewZeroStateBlock {
            type Action = ();

            fn handle_action(
                &mut self,
                _action: &Self::Action,
                _ctx: &mut warpui::ViewContext<Self>,
            ) {
            }
        }

        #[derive(Clone, Debug, Default)]
        pub struct InlineAgentViewHeader;

        impl InlineAgentViewHeader {
            pub fn new<T, U, V, W, X>(
                _conversation_id: T,
                _origin: U,
                _controller: V,
                _terminal_model: W,
                _ctx: X,
            ) -> Self {
                Self
            }
        }

        impl warpui::Entity for InlineAgentViewHeader {
            type Event = ();
        }

        impl warpui::View for InlineAgentViewHeader {
            fn ui_name() -> &'static str {
                "InlineAgentViewHeader"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        #[derive(Clone, Debug, Default)]
        pub struct AgentViewEntryBlock;

        #[derive(Clone, Debug)]
        pub enum AgentViewEntryBlockEvent {
            EnterAgentView {
                conversation_id: crate::ai::agent::conversation::AIConversationId,
            },
        }

        #[derive(Clone, Debug, Default)]
        pub struct AgentViewEntryBlockParams {
            pub conversation_id: crate::ai::agent::conversation::AIConversationId,
            pub is_new: bool,
            pub is_restored: bool,
            pub origin: AgentViewEntryOrigin,
            pub agent_view_controller: Option<warpui::ModelHandle<AgentViewController>>,
        }

        impl AgentViewEntryBlock {
            pub fn new<T>(_params: AgentViewEntryBlockParams, _ctx: T) -> Self {
                Self
            }
        }

        impl warpui::Entity for AgentViewEntryBlock {
            type Event = AgentViewEntryBlockEvent;
        }

        impl warpui::View for AgentViewEntryBlock {
            fn ui_name() -> &'static str {
                "AgentViewEntryBlock"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl warpui::TypedActionView for AgentViewEntryBlock {
            type Action = ();

            fn handle_action(
                &mut self,
                _action: &Self::Action,
                _ctx: &mut warpui::ViewContext<Self>,
            ) {
            }
        }

        #[derive(Clone, Debug, Default)]
        pub enum AutoTriggerBehavior {
            Always,
            InAgentView,
            #[default]
            Never,
        }

        #[derive(Clone, Debug, Default)]
        pub struct EnterAgentViewError;

        impl std::fmt::Display for EnterAgentViewError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                f.write_str("agent view is removed")
            }
        }

        impl std::error::Error for EnterAgentViewError {}

        #[derive(Clone, Debug, Default)]
        pub struct AgentViewHeaderDisabledTheme;

        macro_rules! impl_removed_action_button_theme {
            ($type:ty) => {
                impl crate::view_components::action_button::ActionButtonTheme for $type {
                    fn background(
                        &self,
                        _hovered: bool,
                        _appearance: &crate::appearance::Appearance,
                    ) -> Option<crate::themes::theme::Fill> {
                        None
                    }

                    fn text_color(
                        &self,
                        _hovered: bool,
                        _background: Option<crate::themes::theme::Fill>,
                        _appearance: &crate::appearance::Appearance,
                    ) -> pathfinder_color::ColorU {
                        pathfinder_color::ColorU::black()
                    }
                }
            };
        }

        impl_removed_action_button_theme!(AgentViewHeaderTheme);
        impl_removed_action_button_theme!(AgentViewHeaderDisabledTheme);

        #[derive(Clone, Debug)]
        pub enum AgentViewZeroStateEvent {
            ClickedInitCallout,
            OpenConversation {
                conversation_id: crate::ai::agent::conversation::AIConversationId,
            },
        }

        pub const ENTER_AGENT_VIEW_NEW_CONVERSATION_KEYSTROKE: &str = "enter";
        pub const ENTER_CLOUD_AGENT_VIEW_NEW_CONVERSATION_KEYSTROKE: &str = "enter";

        pub fn render_block_container<T>(_arg: T) -> impl warpui::Element {
            warpui::elements::Empty::new()
        }

        pub mod shortcuts {
            use warpui::{Element, Entity, ModelContext, ModelHandle};

            #[derive(Clone, Debug, Default)]
            pub struct AgentShortcutViewModel {
                is_open: bool,
            }

            #[derive(Clone, Debug, Default)]
            pub struct AgentShortcutsViewContext {
                pub is_cloud_agent: bool,
                pub has_submitted_first_prompt: bool,
            }

            #[derive(Clone, Debug)]
            pub enum AgentShortcutEvent {
                ToggledViewVisibility { is_visible: bool },
            }

            impl AgentShortcutViewModel {
                pub fn new<T, U>(
                    _input_buffer_model: ModelHandle<T>,
                    _agent_view_controller: ModelHandle<U>,
                    _ctx: &mut ModelContext<Self>,
                ) -> Self
                where
                    T: Entity,
                    U: Entity,
                {
                    Self::default()
                }

                pub fn is_shortcut_view_open(&self) -> bool {
                    self.is_open
                }

                pub fn open_shortcut_view(&mut self, ctx: &mut ModelContext<Self>) {
                    self.is_open = true;
                    ctx.emit(AgentShortcutEvent::ToggledViewVisibility { is_visible: true });
                }

                pub fn hide_shortcut_view(&mut self, ctx: &mut ModelContext<Self>) {
                    self.is_open = false;
                    ctx.emit(AgentShortcutEvent::ToggledViewVisibility { is_visible: false });
                }
            }

            impl Entity for AgentShortcutViewModel {
                type Event = AgentShortcutEvent;
            }

            pub fn render_agent_shortcuts_view<T, U>(
                _view_context: T,
                _ctx: U,
            ) -> Box<dyn Element> {
                Box::new(warpui::elements::Empty::new())
            }

            pub fn render_keystroke_with_color_overrides<T, W>(
                _keystroke: T,
                _fg: Option<pathfinder_color::ColorU>,
                _bg: Option<pathfinder_color::ColorU>,
                _ctx: W,
            ) -> Box<dyn Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        pub mod orchestration_conversation_links {
            pub fn parent_conversation_navigation_card<T, U, V>(
                _conversation: T,
                _mouse_state: U,
                _app: V,
            ) -> Option<Box<dyn warpui::Element>> {
                None
            }
        }

        pub fn fork_from_last_known_good_state_exchange_id<T, U>(
            _conversation: T,
            _model: U,
        ) -> Option<crate::ai::agent::AIAgentExchangeId> {
            None
        }

        pub mod toolbar_item {
            #[derive(
                Clone,
                Debug,
                Default,
                PartialEq,
                Eq,
                Hash,
                serde::Serialize,
                serde::Deserialize,
                schemars::JsonSchema,
                settings_value::SettingsValue,
            )]
            pub enum AgentToolbarItemKind {
                #[default]
                Unknown,
                ContextChip(crate::context_chips::ContextChipKind),
                ModelSelector,
                NLDToggle,
                ContextWindowUsage,
                FileExplorer,
                RichInput,
                VoiceInput,
                FileAttach,
                ShareSession,
                Settings,
                FastForwardToggle,
            }

            impl AgentToolbarItemKind {
                pub fn default_left() -> Vec<Self> {
                    Vec::new()
                }

                pub fn default_right() -> Vec<Self> {
                    Vec::new()
                }

                pub fn cli_default_left() -> Vec<Self> {
                    Vec::new()
                }

                pub fn cli_default_right() -> Vec<Self> {
                    Vec::new()
                }

                pub fn display_label(&self) -> &str {
                    match self {
                        Self::Unknown => "Unknown",
                        Self::ContextChip(_) => "Context",
                        _ => "Terminal",
                    }
                }

                pub fn icon(&self) -> Option<crate::ui_components::icons::Icon> {
                    None
                }
            }
        }

        pub mod agent_input_footer {
            use pathfinder_geometry::vector::Vector2F;
            use warpui::{Entity, TypedActionView, View, ViewContext};

            #[derive(Clone, Debug)]
            pub enum AgentInputFooterEvent {
                #[cfg(feature = "voice_input")]
                ToggleVoiceInput(voice_input::VoiceInputToggledFrom),
                SelectFile,
                WriteToPty(String),
                InsertIntoCLIRichInput(String),
                ToggleCodeReviewPane(crate::terminal::CLIAgent),
                ToggleFileExplorer(crate::terminal::CLIAgent),
                StartRemoteControl,
                StopRemoteControl,
                OpenRichInput,
                HideRichInput,
                ToggledChipMenu {
                    open: bool,
                },
                TryExecuteChipCommand(String),
                PromptAlert(crate::ai::blocklist::prompt::prompt_alert::PromptAlertEvent),
                ModelSelectorOpened,
                ModelSelectorClosed,
                ToggleInlineModelSelector {
                    initial_tab: crate::terminal::input::models::InlineModelSelectorTab,
                },
                OpenSettings(crate::settings_view::SettingsSection),
                OpenCodeReview,
                OpenAIDocument {
                    document_id: crate::ai::document::AIDocumentId,
                    document_version: crate::ai::document::AIDocumentVersion,
                },
                ShowContextMenu {
                    position: Vector2F,
                },
                OpenEnvironmentManagementPane,
                PluginInstalled(crate::terminal::CLIAgent),
                #[cfg(not(target_family = "wasm"))]
                OpenPluginInstructionsPane(
                    crate::terminal::CLIAgent,
                    crate::terminal::cli_agent_sessions::plugin_manager::PluginModalKind,
                ),
            }

            #[derive(Default)]
            pub struct AgentInputFooter;

            impl AgentInputFooter {
                pub fn new<T, U, V, W, X, Y>(
                    _menu_positioning_provider: T,
                    _terminal_view_id: warpui::EntityId,
                    _ai_input_model: U,
                    _terminal_model: V,
                    _ambient_agent_view_model: W,
                    _current_prompt: X,
                    _footer_display_chip_config: Y,
                    _ctx: &mut ViewContext<Self>,
                ) -> Self {
                    Self
                }

                pub fn has_open_chip_menu<T: ?Sized>(&self, _ctx: &T) -> bool {
                    false
                }

                pub fn is_model_selector_open<T: ?Sized>(&self, _ctx: &T) -> bool {
                    false
                }

                pub fn is_v2_model_selector_open<T: ?Sized>(&self, _ctx: &T) -> bool {
                    false
                }

                pub fn set_voice_is_active<T>(&mut self, _is_active: bool, _ctx: T) {}

                pub fn update_session_context<T, U>(&mut self, _context: T, _ctx: U) {}

                pub fn set_current_repo_path<T, U>(&mut self, _path: T, _ctx: U) {}
            }

            impl Entity for AgentInputFooter {
                type Event = AgentInputFooterEvent;
            }

            impl View for AgentInputFooter {
                fn ui_name() -> &'static str {
                    "AgentInputFooter"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl TypedActionView for AgentInputFooter {
                type Action = ();

                fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
            }

            #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
            pub struct AgentInputButtonTheme;

            impl crate::view_components::action_button::ActionButtonTheme for AgentInputButtonTheme {
                fn background(
                    &self,
                    _hovered: bool,
                    _appearance: &crate::appearance::Appearance,
                ) -> Option<crate::themes::theme::Fill> {
                    None
                }

                fn text_color(
                    &self,
                    _hovered: bool,
                    _background: Option<crate::themes::theme::Fill>,
                    _appearance: &crate::appearance::Appearance,
                ) -> pathfinder_color::ColorU {
                    pathfinder_color::ColorU::black()
                }
            }

            pub mod editor {
                use warpui::{TypedActionView, ViewContext};

                #[derive(Clone, Debug)]
                pub enum AgentToolbarEditorEvent {
                    Close,
                }

                #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
                pub enum AgentToolbarEditorMode {
                    #[default]
                    Hidden,
                    AgentView,
                    CLIAgent,
                }

                #[derive(Clone, Debug, Default)]
                pub struct AgentToolbarEditorModal;

                impl AgentToolbarEditorModal {
                    pub fn new<T>(_ctx: T) -> Self {
                        Self
                    }

                    pub fn open<T, U>(&mut self, _mode: T, _ctx: U) {}
                }

                impl warpui::Entity for AgentToolbarEditorModal {
                    type Event = AgentToolbarEditorEvent;
                }

                impl warpui::View for AgentToolbarEditorModal {
                    fn ui_name() -> &'static str {
                        "AgentToolbarEditorModal"
                    }

                    fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                        Box::new(warpui::elements::Empty::new())
                    }
                }

                impl TypedActionView for AgentToolbarEditorModal {
                    type Action = ();

                    fn handle_action(
                        &mut self,
                        _action: &Self::Action,
                        _ctx: &mut ViewContext<Self>,
                    ) {
                    }
                }

                #[derive(Clone, Debug, Default)]
                pub struct AgentToolbarInlineEditor;
            }

            pub mod toolbar_item {
                pub use super::super::toolbar_item::AgentToolbarItemKind;
            }
        }

        pub mod editor {
            pub use super::agent_input_footer::editor::{
                AgentToolbarEditorEvent, AgentToolbarEditorModal,
            };
        }

        pub use agent_input_footer::{AgentInputFooter, AgentInputFooterEvent};
    }

    pub mod block {
        pub use super::AIBlock;
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Copy, Debug, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
        pub struct TextLocation;

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum FinishReason {
            #[default]
            Unknown,
            Complete,
            Error,
            Cancelled,
            CancelledDuringRequestedCommandExecution,
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
        pub enum AIBlockAction {
            #[default]
            Unknown,
            ToggleIsUsageFooterExpanded,
            CopyQuery,
            CopyOutput,
            Copy,
            CopyConversation,
            CopyCommand(crate::ai::agent::AIAgentActionId),
        }

        pub mod cli {
            #[derive(Clone, Debug, Default)]
            pub struct CLISubagentView;

            #[derive(Clone, Debug)]
            pub enum CLISubagentViewEvent {
                TextSelected,
                CopiedEmptyText,
                #[cfg(windows)]
                WindowsCtrlC,
            }

            impl CLISubagentView {
                pub fn new<T, U, V, W, X, Y, Z, A, B>(
                    _block_id: T,
                    _ai_action_model: U,
                    _cli_subagent_controller: V,
                    _terminal_model: W,
                    _conversation_id: X,
                    _task_id: Y,
                    _pwd: Z,
                    _shell_launch_data: A,
                    _ctx: B,
                ) -> Self {
                    Self
                }

                pub fn selected_text<T: ?Sized>(&self, _ctx: &T) -> Option<String> {
                    None
                }

                pub fn clear_all_selections<T>(&mut self, _ctx: T) {}

                pub fn start_selection_at_min_point<T>(&mut self, _ctx: T) {}

                pub fn start_selection_at_max_point<T>(&mut self, _ctx: T) {}
            }

            impl warpui::Entity for CLISubagentView {
                type Event = CLISubagentViewEvent;
            }

            impl warpui::View for CLISubagentView {
                fn ui_name() -> &'static str {
                    "CLISubagentView"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl warpui::TypedActionView for CLISubagentView {
                type Action = ();

                fn handle_action(
                    &mut self,
                    _action: &Self::Action,
                    _ctx: &mut warpui::ViewContext<Self>,
                ) {
                }
            }
        }

        pub mod cli_controller {
            use serde::{Deserialize, Serialize};

            #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
            pub enum LongRunningCommandControlState {
                User {
                    reason: UserTakeOverReason,
                },
                Agent {
                    is_blocked: bool,
                    should_hide_responses: bool,
                },
            }

            impl Default for LongRunningCommandControlState {
                fn default() -> Self {
                    Self::User {
                        reason: UserTakeOverReason::Stop,
                    }
                }
            }

            impl LongRunningCommandControlState {
                pub fn is_agent_in_control(&self) -> bool {
                    matches!(
                        self,
                        Self::Agent {
                            is_blocked: false,
                            ..
                        }
                    )
                }

                pub fn is_agent_blocked(&self) -> bool {
                    matches!(
                        self,
                        Self::Agent {
                            is_blocked: true,
                            ..
                        }
                    )
                }

                pub fn is_user_in_control(&self) -> bool {
                    matches!(self, Self::User { .. })
                }

                pub fn should_hide_responses(&self) -> bool {
                    matches!(
                        self,
                        Self::Agent {
                            should_hide_responses: true,
                            ..
                        }
                    )
                }

                pub fn user_take_over_reason(&self) -> Option<&UserTakeOverReason> {
                    match self {
                        Self::User { reason } => Some(reason),
                        Self::Agent { .. } => None,
                    }
                }
            }

            #[derive(Clone, Debug)]
            pub enum CLISubagentEvent {
                SpawnedSubagent {
                    task_id: crate::ai::agent::task::TaskId,
                    block_id: crate::terminal::model::block::BlockId,
                    conversation_id: crate::ai::agent::conversation::AIConversationId,
                    initial_requested_command_action_id: Option<crate::ai::agent::AIAgentActionId>,
                },
                FinishedSubagent {
                    block_id: crate::terminal::model::block::BlockId,
                    conversation_id: Option<crate::ai::agent::conversation::AIConversationId>,
                },
                UpdatedControl {
                    block_id: crate::terminal::model::block::BlockId,
                    requested_command_action_id: Option<crate::ai::agent::AIAgentActionId>,
                    agent_has_control: bool,
                },
                UpdatedLastSnapshot,
                ToggledHideResponses,
                ControlHandedBackAfterTransfer,
            }

            #[derive(Clone, Debug, Default)]
            pub struct CLISubagentController;

            #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
            pub enum UserTakeOverReason {
                #[default]
                Stop,
                Manual,
                TransferFromAgent {
                    reason: String,
                },
            }

            impl UserTakeOverReason {
                pub fn is_stop(&self) -> bool {
                    matches!(self, Self::Stop)
                }
            }

            impl warpui::Entity for CLISubagentController {
                type Event = CLISubagentEvent;
            }

            impl warpui::SingletonEntity for CLISubagentController {}

            impl CLISubagentController {
                #[allow(clippy::too_many_arguments)]
                pub fn new<A, B, C, D, E, F, G>(
                    _ai_controller: A,
                    _ai_action_model: B,
                    _agent_view_controller: C,
                    _terminal_model: D,
                    _model_events_handle: E,
                    _terminal_view_id: F,
                    _ctx: G,
                ) -> Self {
                    Self
                }

                pub fn is_agent_in_control(&self) -> bool {
                    false
                }

                pub(crate) fn is_agent_in_control_or_tagged_in(&self) -> bool {
                    false
                }

                pub fn switch_control_to_user<T, U>(&mut self, _reason: T, _ctx: U) {}

                pub fn handoff_active_command_control_to_agent<T>(&mut self, _ctx: T) {}

                pub fn toggle_hide_responses<T>(&mut self, _ctx: T) {}
            }
        }

        pub mod status_bar {
            #[derive(Clone, Debug)]
            pub enum BlocklistAIStatusBarEvent {
                SummarizationCancelDialogToggled { is_open: bool },
                Stop,
            }

            #[derive(Clone, Debug, Default)]
            pub struct BlocklistAIStatusBar;

            impl BlocklistAIStatusBar {
                pub fn new<T, U, V, W, X, Y, Z, A, B, C, D, E, F, G, H>(
                    _ai_controller: T,
                    _agent_view_controller: U,
                    _cli_subagent_controller: V,
                    _ai_action_model: W,
                    _ai_context_model: X,
                    _ai_input_model: Y,
                    _buffer_model: Z,
                    _model_events: A,
                    _terminal_model: B,
                    _agent_shortcut_view_model: C,
                    _ambient_agent_view_model: D,
                    _suggestions_mode_model: E,
                    _slash_command_model: F,
                    _ephemeral_message_model: G,
                    _terminal_view_id: warpui::EntityId,
                    _ctx: H,
                ) -> Self {
                    Self
                }

                pub fn handle_ctrl_c<T>(&mut self, _ctx: T) -> bool {
                    false
                }

                pub fn should_show_summarization_cancel_dialog<T>(&self, _ctx: T) -> bool {
                    false
                }

                pub fn summarization_cancel_dialog_handle(&self) -> Option<warpui::ViewHandle<crate::ai::blocklist::summarization_cancel_dialog::SummarizationCancelDialog>>{
                    None
                }
            }

            impl warpui::Entity for BlocklistAIStatusBar {
                type Event = BlocklistAIStatusBarEvent;
            }

            impl warpui::View for BlocklistAIStatusBar {
                fn ui_name() -> &'static str {
                    "BlocklistAIStatusBar"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            #[derive(Clone, Debug)]
            pub enum BlocklistAIStatusBarAction {
                ToggleHideResponses,
                SwitchCommandControlToUser,
                Stop,
                ForceRefreshAgentView {
                    block_id: crate::terminal::model::block::BlockId,
                },
            }

            impl warpui::TypedActionView for BlocklistAIStatusBar {
                type Action = BlocklistAIStatusBarAction;

                fn handle_action(
                    &mut self,
                    _action: &Self::Action,
                    _ctx: &mut warpui::ViewContext<Self>,
                ) {
                }
            }
        }

        pub mod keyboard_navigable_buttons {
            use warpui::elements::{
                Container, CrossAxisAlignment, Flex, MouseStateHandle, ParentElement, Text,
            };
            use warpui::{
                AppContext, Element, Entity, SingletonEntity, TypedActionView, View, ViewContext,
            };

            pub enum KeyboardNavigableButtonsEvent {}

            #[derive(Clone, Debug)]
            pub enum KeyboardNavigableButtonsAction {
                Selected(usize),
            }

            pub struct KeyboardNavigableButtonBuilder {
                label: String,
                disabled: bool,
                on_click: Box<dyn Fn(&mut ViewContext<KeyboardNavigableButtons>)>,
            }

            impl KeyboardNavigableButtonBuilder {
                fn new<A: warpui::Action + Clone + 'static>(
                    label: String,
                    action: A,
                    disabled: bool,
                ) -> Self {
                    Self {
                        label,
                        disabled,
                        on_click: Box::new(move |ctx| {
                            if !disabled {
                                ctx.dispatch_typed_action(&action);
                            }
                        }),
                    }
                }
            }

            pub fn simple_navigation_button<A: warpui::Action + Clone + 'static>(
                text_label: String,
                _mouse_state: MouseStateHandle,
                action: A,
                disabled: bool,
            ) -> KeyboardNavigableButtonBuilder {
                KeyboardNavigableButtonBuilder::new(text_label, action, disabled)
            }

            pub fn rich_navigation_button<A: warpui::Action + Clone + 'static>(
                text_label: String,
                _sub_label: Option<String>,
                _recommended: bool,
                _mouse_state: MouseStateHandle,
                action: A,
            ) -> KeyboardNavigableButtonBuilder {
                KeyboardNavigableButtonBuilder::new(text_label, action, false)
            }

            pub struct KeyboardNavigableButtons {
                buttons: Vec<KeyboardNavigableButtonBuilder>,
            }

            impl KeyboardNavigableButtons {
                pub fn new(buttons: Vec<KeyboardNavigableButtonBuilder>) -> Self {
                    Self { buttons }
                }
            }

            impl Entity for KeyboardNavigableButtons {
                type Event = KeyboardNavigableButtonsEvent;
            }

            impl TypedActionView for KeyboardNavigableButtons {
                type Action = KeyboardNavigableButtonsAction;

                fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
                    match action {
                        KeyboardNavigableButtonsAction::Selected(index) => {
                            if let Some(button) = self.buttons.get(*index) {
                                (button.on_click)(ctx);
                            }
                        }
                    }
                }
            }

            impl View for KeyboardNavigableButtons {
                fn ui_name() -> &'static str {
                    "KeyboardNavigableButtons"
                }

                fn render(&self, app: &AppContext) -> Box<dyn Element> {
                    let mut column =
                        Flex::column().with_cross_axis_alignment(CrossAxisAlignment::Stretch);
                    let appearance = crate::appearance::Appearance::as_ref(app);
                    for button in &self.buttons {
                        let text = if button.disabled {
                            format!("{} (disabled)", button.label)
                        } else {
                            button.label.clone()
                        };
                        column.add_child(
                            Container::new(
                                Text::new(text, appearance.ui_font_family(), 13.).finish(),
                            )
                            .with_uniform_padding(4.)
                            .finish(),
                        );
                    }
                    column.finish()
                }
            }
        }

        pub mod toggleable_items {
            use warpui::elements::ParentElement;
            use warpui::ui_components::components::UiComponent;
            use warpui::ui_components::text::Span;
            use warpui::{AppContext, Element, Entity, TypedActionView, View, ViewContext};

            type ItemLabelFn<T> = Box<dyn Fn(&T, &AppContext) -> Span>;

            pub struct ToggleableItemBuilder<T> {
                label_fn: ItemLabelFn<T>,
                is_selected_fn: Box<dyn Fn(&T) -> bool>,
            }

            impl<T> ToggleableItemBuilder<T> {
                pub fn new(
                    label_fn: impl Fn(&T, &AppContext) -> Span + 'static,
                    is_selected_fn: impl Fn(&T) -> bool + 'static,
                ) -> Self {
                    Self {
                        label_fn: Box::new(label_fn),
                        is_selected_fn: Box::new(is_selected_fn),
                    }
                }
            }

            #[derive(Clone, Debug)]
            pub enum ToggleableItemsEvent {
                SelectionChanged,
                SubmitRequested,
            }

            #[derive(Clone, Debug)]
            pub enum ToggleableItemsAction {
                Submit,
            }

            pub struct ToggleableItemsView<T> {
                items: Vec<T>,
                selected: Vec<bool>,
                label_fn: ItemLabelFn<T>,
            }

            impl<T> ToggleableItemsView<T> {
                pub fn new(items: Vec<T>, builder: ToggleableItemBuilder<T>) -> Self {
                    let selected = items
                        .iter()
                        .map(|item| (builder.is_selected_fn)(item))
                        .collect();
                    Self {
                        items,
                        selected,
                        label_fn: builder.label_fn,
                    }
                }

                pub fn get_selected_items(&self) -> impl Iterator<Item = &T> + '_ {
                    self.items
                        .iter()
                        .zip(self.selected.iter())
                        .filter_map(|(item, selected)| selected.then_some(item))
                }
            }

            impl<T: 'static> Entity for ToggleableItemsView<T> {
                type Event = ToggleableItemsEvent;
            }

            impl<T: 'static> TypedActionView for ToggleableItemsView<T> {
                type Action = ToggleableItemsAction;

                fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
                    match action {
                        ToggleableItemsAction::Submit => {
                            ctx.emit(ToggleableItemsEvent::SubmitRequested);
                        }
                    }
                }
            }

            impl<T: 'static> View for ToggleableItemsView<T> {
                fn ui_name() -> &'static str {
                    "ToggleableItemsView"
                }

                fn render(&self, app: &AppContext) -> Box<dyn Element> {
                    let mut column = warpui::elements::Flex::column();
                    for item in &self.items {
                        column.add_child((self.label_fn)(item, app).build().finish());
                    }
                    column.finish()
                }
            }
        }

        pub struct PendingUserQueryBlock;

        #[derive(Clone, Debug)]
        pub enum PendingUserQueryBlockEvent {
            Dismissed,
            SendNow,
        }

        impl PendingUserQueryBlock {
            pub fn new(
                _prompt: String,
                _user_display_name: String,
                _profile_image_path: Option<String>,
                _show_close_button: bool,
                _show_send_now_button: bool,
                _ctx: &mut warpui::ViewContext<Self>,
            ) -> Self {
                Self
            }
        }

        impl warpui::Entity for PendingUserQueryBlock {
            type Event = PendingUserQueryBlockEvent;
        }

        impl warpui::View for PendingUserQueryBlock {
            fn ui_name() -> &'static str {
                "PendingUserQueryBlock"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl warpui::TypedActionView for PendingUserQueryBlock {
            type Action = ();

            fn handle_action(
                &mut self,
                _action: &Self::Action,
                _ctx: &mut warpui::ViewContext<Self>,
            ) {
            }
        }

        pub mod secret_redaction {
            pub fn find_secrets_in_text(_text: &str) -> Vec<String> {
                Vec::new()
            }

            pub fn find_secrets_in_text_with_levels(
                _text: &str,
            ) -> Vec<(
                std::ops::Range<usize>,
                crate::terminal::model::secrets::SecretLevel,
            )> {
                Vec::new()
            }
        }

        pub mod view_impl {
            pub const CONTENT_HORIZONTAL_PADDING: f32 = 0.;
            pub const CONTENT_VERTICAL_PADDING: f32 = 0.;
            pub const CONTENT_ITEM_VERTICAL_MARGIN: f32 = 0.;

            pub trait WithContentItemSpacing {
                fn with_content_item_spacing(self) -> warpui::elements::Container;
                fn with_agent_output_item_spacing<T>(self, _app: T) -> warpui::elements::Container;
            }

            impl WithContentItemSpacing for Box<dyn warpui::Element> {
                fn with_content_item_spacing(self) -> warpui::elements::Container {
                    warpui::elements::Container::new(self)
                }

                fn with_agent_output_item_spacing<T>(self, _app: T) -> warpui::elements::Container {
                    warpui::elements::Container::new(self)
                }
            }

            pub mod output {
                #[derive(Clone, Debug, Default)]
                pub struct LinkActionConstructors<A = ()> {
                    _phantom: std::marker::PhantomData<A>,
                }
            }

            pub mod query {
                pub fn render_query<T>(
                    _query: T,
                    _app: &warpui::AppContext,
                ) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }
        }
    }

    pub mod inline_action {
        pub mod code_diff_view {
            use serde::{Deserialize, Serialize};

            #[derive(Clone, Debug, Default)]
            pub struct CodeDiffView;

            impl CodeDiffView {
                pub fn primary_file_path(&self, _ctx: &warpui::AppContext) -> Option<String> {
                    None
                }

                pub fn display_mode(&self) -> CodeDiffDisplayMode {
                    CodeDiffDisplayMode
                }

                pub fn set_embedded_display_mode<T>(&mut self, _embedded: bool, _ctx: T) {}

                pub fn action_id(&self) -> crate::ai::agent::AIAgentActionId {
                    crate::ai::agent::AIAgentActionId::default()
                }

                pub fn set_original_pane_id<T>(&mut self, _pane_id: T) {}
            }

            #[derive(Clone, Copy, Debug, Default)]
            pub struct CodeDiffDisplayMode;

            impl CodeDiffDisplayMode {
                pub fn is_full_pane(&self) -> bool {
                    false
                }
            }

            #[derive(Clone, Debug)]
            pub enum CodeDiffViewEvent {
                Pane(crate::pane_group::pane::PaneEvent),
                EditorFocused,
            }

            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub enum DiffSessionType {
                #[default]
                Unknown,
                Local,
                Remote(String),
            }

            #[derive(Clone, Debug, Default, Serialize, Deserialize)]
            pub struct FileDiff;
        }

        pub mod inline_action_header {
            use std::borrow::Cow;
            use std::rc::Rc;

            use pathfinder_color::ColorU;
            use warpui::elements::{CornerRadius, Empty, Icon, MouseStateHandle};
            use warpui::fonts::FamilyId;
            use warpui::{AppContext, Element, EventContext, SingletonEntity};

            pub const INLINE_ACTION_HORIZONTAL_PADDING: f32 = 0.;
            pub const INLINE_ACTION_VERTICAL_PADDING: f32 = 0.;
            pub const INLINE_ACTION_HEADER_VERTICAL_PADDING: f32 = 0.;

            pub type OnToggleExpandedCallback = Rc<dyn Fn(&mut EventContext) + 'static>;
            pub type OnRightClickCallback = Rc<dyn Fn(&mut EventContext) + 'static>;

            #[derive(Clone)]
            pub struct ExpandedConfig {
                pub is_expanded: bool,
                pub on_toggle_expanded: Option<OnToggleExpandedCallback>,
                pub on_right_click: Option<OnRightClickCallback>,
                pub toggle_mouse_state: MouseStateHandle,
                pub expands_upwards: bool,
            }

            impl ExpandedConfig {
                pub fn new(is_expanded: bool, toggle_mouse_state: MouseStateHandle) -> Self {
                    Self {
                        is_expanded,
                        on_toggle_expanded: None,
                        on_right_click: None,
                        toggle_mouse_state,
                        expands_upwards: false,
                    }
                }

                pub fn with_expands_upwards(mut self) -> Self {
                    self.expands_upwards = true;
                    self
                }

                pub fn with_toggle_callback<F>(mut self, callback: F) -> Self
                where
                    F: Fn(&mut EventContext) + 'static,
                {
                    self.on_toggle_expanded = Some(Rc::new(callback));
                    self
                }

                pub fn with_right_click_callback<F>(mut self, callback: F) -> Self
                where
                    F: Fn(&mut EventContext) + 'static,
                {
                    self.on_right_click = Some(Rc::new(callback));
                    self
                }
            }

            #[derive(Clone)]
            pub struct RightClickConfig {
                pub on_right_click: OnRightClickCallback,
                pub header_mouse_state: MouseStateHandle,
            }

            impl RightClickConfig {
                pub fn new(
                    on_right_click: OnRightClickCallback,
                    header_mouse_state: MouseStateHandle,
                ) -> Self {
                    Self {
                        on_right_click,
                        header_mouse_state,
                    }
                }
            }

            #[derive(Clone)]
            pub enum InteractionMode {
                ActionButtons {
                    action_buttons: Vec<
                        Rc<
                            dyn crate::view_components::compactible_action_button::RenderCompactibleActionButton,
                        >,
                    >,
                    size_switch_threshold: f32,
                },
                ManuallyExpandable(ExpandedConfig),
                RightClickable(RightClickConfig),
            }

            #[derive(Clone)]
            pub struct HeaderConfig {
                pub title: Cow<'static, str>,
                pub font_family: FamilyId,
                pub use_markdown: bool,
                pub icon: Option<Icon>,
                pub badge: Option<String>,
                pub interaction_mode: Option<InteractionMode>,
                pub is_text_selectable: bool,
                pub font_color_override: Option<ColorU>,
                pub corner_radius_override: Option<CornerRadius>,
                pub soft_wrap_title: bool,
            }

            impl HeaderConfig {
                pub fn new(title: impl Into<Cow<'static, str>>, app: &AppContext) -> Self {
                    Self {
                        title: title.into(),
                        font_family: crate::appearance::Appearance::as_ref(app).ui_font_family(),
                        use_markdown: false,
                        icon: None,
                        badge: None,
                        interaction_mode: None,
                        is_text_selectable: false,
                        font_color_override: None,
                        corner_radius_override: None,
                        soft_wrap_title: false,
                    }
                }

                pub fn with_soft_wrap_title(mut self) -> Self {
                    self.soft_wrap_title = true;
                    self
                }

                pub fn with_font_family(mut self, font: FamilyId) -> Self {
                    self.font_family = font;
                    self
                }

                pub fn with_icon(mut self, icon: Icon) -> Self {
                    self.icon = Some(icon);
                    self
                }

                pub fn with_badge(mut self, badge: String) -> Self {
                    self.badge = Some(badge);
                    self
                }

                pub fn with_interaction_mode(mut self, interaction_mode: InteractionMode) -> Self {
                    self.interaction_mode = Some(interaction_mode);
                    self
                }

                pub fn with_selectable_text(mut self) -> Self {
                    self.is_text_selectable = true;
                    self
                }

                pub fn with_font_color(mut self, font_color: ColorU) -> Self {
                    self.font_color_override = Some(font_color);
                    self
                }

                pub fn with_corner_radius_override(mut self, corner_radius: CornerRadius) -> Self {
                    self.corner_radius_override = Some(corner_radius);
                    self
                }

                pub fn with_markdown(mut self) -> Self {
                    self.use_markdown = true;
                    self
                }

                pub fn render_header(
                    self,
                    _app: &AppContext,
                    _interaction_mode_content: Option<Box<dyn Element>>,
                ) -> Box<dyn Element> {
                    Box::new(Empty::new())
                }

                pub fn render(self, app: &AppContext) -> Box<dyn Element> {
                    self.render_header(app, None)
                }
            }
        }

        pub mod inline_action_icons {
            use crate::ui_components::icons::Icon as UiIcon;
            use warp_core::ui::theme::AnsiColorIdentifier;
            use warpui::elements::Icon;
            use warpui::AppContext;

            pub fn icon_size(_app: &AppContext) -> f32 {
                16.
            }

            pub fn green_check_icon(appearance: &crate::appearance::Appearance) -> Icon {
                Icon::new(
                    UiIcon::Check.into(),
                    AnsiColorIdentifier::Green
                        .to_ansi_color(&appearance.theme().terminal_colors().normal),
                )
            }

            pub fn red_x_icon(appearance: &crate::appearance::Appearance) -> Icon {
                Icon::new(
                    UiIcon::X.into(),
                    AnsiColorIdentifier::Red
                        .to_ansi_color(&appearance.theme().terminal_colors().normal),
                )
            }

            pub fn cancelled_icon(appearance: &crate::appearance::Appearance) -> Icon {
                Icon::new(
                    UiIcon::Cancelled.into(),
                    crate::ui_components::blended_colors::neutral_6(appearance.theme()),
                )
            }
        }

        pub mod requested_action {
            use lazy_static::lazy_static;
            use warpui::elements::{Container, Empty};
            use warpui::keymap::Keystroke;
            use warpui::{AppContext, Element, SingletonEntity};

            lazy_static! {
                pub static ref ENTER_KEYSTROKE: Keystroke = Keystroke {
                    key: "enter".to_owned(),
                    ..Default::default()
                };
                pub static ref ESCAPE_KEYSTROKE: Keystroke = Keystroke {
                    key: "escape".to_owned(),
                    ..Default::default()
                };
            }

            pub struct RenderableAction;

            impl RenderableAction {
                pub fn new(_command: &str, _app: &AppContext) -> Self {
                    Self
                }

                pub fn new_with_element<T>(_element: T, _app: &AppContext) -> Self {
                    Self
                }

                pub fn with_background_color<T>(self, _color: T) -> Self {
                    self
                }

                pub fn with_icon<T>(self, _icon: T) -> Self {
                    self
                }

                pub fn with_header<T>(self, _header: T) -> Self {
                    self
                }

                pub fn with_content_item_spacing(self) -> Self {
                    self
                }

                pub fn with_action_button<T>(self, _button: T) -> Self {
                    self
                }

                pub fn render(&self, _app: &AppContext) -> Container {
                    Container::new(Empty::new().finish())
                }
            }
        }

        pub mod requested_command {
            pub const VIEWING_COMMAND_DETAIL_MESSAGE: &str = "";
        }

        pub mod requested_script {
            use std::rc::Rc;

            use warpui::elements::{Empty, MouseStateHandle};
            use warpui::keymap::Keystroke;
            use warpui::ui_components::toggle_menu::ToggleMenuStateHandle;
            use warpui::{AppContext, Element, EventContext};

            #[derive(Clone, Debug)]
            pub struct TitledScript {
                pub title: String,
                pub content: String,
            }

            #[derive(Clone, Debug, PartialEq, Eq)]
            pub enum RequestedScriptStatus {
                WaitingForUser,
                Running,
                Succeeded,
                Failed,
                Cancelled,
            }

            #[derive(Clone, Default)]
            pub struct RequestedScriptMouseStates {
                pub container: MouseStateHandle,
                pub accept_button: MouseStateHandle,
                pub reject_button: MouseStateHandle,
                pub toggle: MouseStateHandle,
            }

            pub fn render_requested_script<F1, F2, F3>(
                _title: &str,
                _content: &str,
                _status: RequestedScriptStatus,
                _is_collapsed: bool,
                _show_block: bool,
                _on_toggle: F1,
                _on_accept: F2,
                _on_reject: F3,
                _enter_keystroke: &Keystroke,
                _escape_keystroke: &Keystroke,
                _mouse_states: &RequestedScriptMouseStates,
                _is_focused: bool,
                _app: &AppContext,
            ) -> Box<dyn Element>
            where
                F1: Fn(&mut EventContext, f32, f32) + 'static,
                F2: Fn(&mut EventContext) + 'static,
                F3: Fn(&mut EventContext) + 'static,
            {
                Box::new(Empty::new())
            }

            #[allow(clippy::too_many_arguments)]
            pub fn render_requested_scripts<F1, F2, F3>(
                _first_script: TitledScript,
                _second_script: TitledScript,
                _is_first_script_active: bool,
                _status: RequestedScriptStatus,
                _is_collapsed: bool,
                _show_block: bool,
                _on_toggle: F1,
                _on_accept: F2,
                _on_reject: F3,
                _enter_keystroke: &Keystroke,
                _escape_keystroke: &Keystroke,
                _mouse_states: &RequestedScriptMouseStates,
                _toggle_menu_mouse_states: Vec<MouseStateHandle>,
                _toggle_menu_state_handle: ToggleMenuStateHandle,
                _on_toggle_install_script_choice: Rc<dyn Fn(&mut EventContext, f32, f32)>,
                _is_focused: bool,
                _width: f32,
                _app: &AppContext,
            ) -> Box<dyn Element>
            where
                F1: Fn(&mut EventContext, f32, f32) + 'static,
                F2: Fn(&mut EventContext) + 'static,
                F3: Fn(&mut EventContext) + 'static,
            {
                Box::new(Empty::new())
            }
        }
    }

    pub mod codebase_index_speedbump_banner {
        #[derive(Clone, Copy, Debug)]
        pub enum CodebaseIndexSpeedbumpBannerAction {
            ToggleAlwaysAllow,
            AllowIndexing,
            Close,
            ViewStatus,
            DismissForever,
        }

        #[derive(Clone, Debug, Default)]
        pub struct CodebaseIndexSpeedbumpBannerState {
            pub id: usize,
            pub repo_path: std::path::PathBuf,
            pub always_allow_checked: bool,
            pub visibility_state: VisibilityState,
        }

        impl CodebaseIndexSpeedbumpBannerState {
            pub fn new(id: usize, repo_path: std::path::PathBuf) -> Self {
                Self {
                    id,
                    repo_path,
                    always_allow_checked: false,
                    visibility_state: VisibilityState::Speedbump,
                }
            }

            pub fn toggle_always_allow_checked(&mut self) {
                self.always_allow_checked = !self.always_allow_checked;
            }

            pub fn show_indexing_banner(&mut self) {
                self.visibility_state = VisibilityState::Indexing;
            }

            pub fn render_codebase_index_speedbump_banner<T>(
                &self,
                _ctx: T,
            ) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub enum VisibilityState {
            #[default]
            Speedbump,
            Indexing,
        }
    }

    pub mod suggested_agent_mode_workflow_modal {
        use warpui::{Entity, TypedActionView, View, ViewContext};

        #[derive(Clone, Debug, Default)]
        pub struct SuggestedAgentModeWorkflowAndId;

        #[derive(Clone, Debug)]
        pub enum SuggestedAgentModeWorkflowModalEvent {
            Close,
            WorkflowCreated,
            RunWorkflow {
                workflow: std::sync::Arc<crate::workflows::WorkflowType>,
                source: Box<crate::workflows::WorkflowSource>,
                argument_override: Option<std::collections::HashMap<String, String>>,
                workflow_selection_source: crate::workflows::WorkflowSelectionSource,
            },
        }

        #[derive(Default)]
        pub struct SuggestedAgentModeWorkflowModal;

        impl Entity for SuggestedAgentModeWorkflowModal {
            type Event = SuggestedAgentModeWorkflowModalEvent;
        }

        impl SuggestedAgentModeWorkflowModal {
            pub fn open_workflow<T, U>(&mut self, _workflow: T, _ctx: U) {}
        }

        impl View for SuggestedAgentModeWorkflowModal {
            fn ui_name() -> &'static str {
                "SuggestedAgentModeWorkflowModal"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl TypedActionView for SuggestedAgentModeWorkflowModal {
            type Action = ();

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }
    }

    pub mod suggested_rule_modal {
        use warpui::{Entity, TypedActionView, View, ViewContext};

        #[derive(Clone, Debug, Default)]
        pub struct SuggestedRuleAndId {
            pub logging_id: crate::ai::agent::SuggestedLoggingId,
        }

        #[derive(Clone, Debug)]
        pub enum SuggestedRuleModalEvent {
            AddNewRule { rule: SuggestedRuleAndId },
            OpenRuleForEditing { rule: SuggestedRuleAndId },
            Close,
        }

        #[derive(Default)]
        pub struct SuggestedRuleModal;

        impl SuggestedRuleModal {
            pub fn new<T>(_ctx: T) -> Self {
                Self
            }

            pub fn set_rule_and_id<T, U: ?Sized>(&mut self, _rule: T, _ctx: &mut U) {}
        }

        impl Entity for SuggestedRuleModal {
            type Event = SuggestedRuleModalEvent;
        }

        impl View for SuggestedRuleModal {
            fn ui_name() -> &'static str {
                "SuggestedRuleModal"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl TypedActionView for SuggestedRuleModal {
            type Action = ();

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }
    }

    pub mod telemetry_banner {
        #[derive(Clone, Debug, Default)]
        pub struct TelemetryBanner;

        impl TelemetryBanner {
            pub fn new<T, U>(_is_onboarded: T, _ctx: U) -> Self {
                Self
            }
        }

        impl warpui::Entity for TelemetryBanner {
            type Event = ();
        }

        impl warpui::View for TelemetryBanner {
            fn ui_name() -> &'static str {
                "TelemetryBanner"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        pub fn should_collect_ai_ugc_telemetry<T>(_ctx: &warpui::AppContext, _enabled: T) -> bool {
            false
        }
    }

    pub mod secret_redaction {
        pub use super::block::secret_redaction::*;
    }

    pub mod code_block {
        use crate::code::editor_management::CodeSource;
        use warpui::{elements::HighlightedRange, AppContext, Element, EventContext};

        #[derive(Clone, Default)]
        pub struct CodeSnippetButtonHandles;

        pub type HandleCode = Box<dyn FnMut(String, &mut EventContext)>;

        pub struct CodeBlockOptions {
            pub on_open: Option<HandleCode>,
            pub on_execute: Option<HandleCode>,
            pub on_copy: Option<HandleCode>,
            pub on_insert: Option<HandleCode>,
            pub footer_element: Option<Box<dyn Element>>,
            pub mouse_handles: Option<CodeSnippetButtonHandles>,
            pub file_path: Option<String>,
        }

        pub fn render_code_block_plain(
            _code: &str,
            _find_highlight_ranges: impl Iterator<Item = HighlightedRange>,
            _options: CodeBlockOptions,
            _app: &AppContext,
            _source: Option<CodeSource>,
        ) -> Box<dyn Element> {
            Box::new(warpui::elements::Empty::new())
        }

        pub fn render_runnable_code_snippet<T, U, V, W, X>(
            _code: T,
            _language: U,
            _on_execute: V,
            _on_copy: W,
            _handles: X,
            _app: &AppContext,
        ) -> Box<dyn Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    pub mod prompt {
        use pathfinder_color::ColorU;

        use crate::themes::theme::Fill;
        use crate::view_components::action_button::{ActionButtonTheme, NakedTheme};
        use crate::{util::color::coloru_with_opacity, Appearance};

        #[derive(Clone)]
        pub struct PromptIconButtonTheme {
            is_blurred: bool,
        }

        impl PromptIconButtonTheme {
            pub fn new(is_blurred: bool) -> Self {
                Self { is_blurred }
            }
        }

        impl ActionButtonTheme for PromptIconButtonTheme {
            fn background(&self, hovered: bool, appearance: &Appearance) -> Option<Fill> {
                NakedTheme.background(hovered, appearance)
            }

            fn text_color(
                &self,
                _hovered: bool,
                _background: Option<Fill>,
                appearance: &Appearance,
            ) -> ColorU {
                let color = appearance
                    .theme()
                    .sub_text_color(appearance.theme().surface_1())
                    .into_solid();
                if self.is_blurred {
                    coloru_with_opacity(color, 50)
                } else {
                    color
                }
            }
        }

        pub mod prompt_alert {
            use crate::server::ids::ServerId;
            use warpui::{AppContext, Entity, View, ViewContext};

            #[derive(Clone, Debug, Default, PartialEq, Eq)]
            pub enum PromptAlertState {
                #[default]
                NoAlert,
                NoConnection,
                AnonymousUserRequestLimitHardGate,
                AnonymousUserRequestLimitSoftGate,
                DelinquentDueToPaymentIssue,
                OveragesToggleableButNotEnabled,
                MonthlyOveragesSpendLimitReached,
                RequestLimitReached,
            }

            #[derive(Clone, Debug)]
            pub enum PromptAlertEvent {
                SignupAnonymousUser,
                OpenBillingAndUsagePage,
                OpenPrivacyPage,
                OpenBillingPortal { team_uid: ServerId },
            }

            #[derive(Default)]
            pub struct PromptAlertView {
                state: PromptAlertState,
            }

            impl PromptAlertView {
                pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
                    Self::default()
                }

                pub fn state(&self) -> &PromptAlertState {
                    &self.state
                }

                pub fn is_no_alert(&self) -> bool {
                    matches!(self.state, PromptAlertState::NoAlert)
                }

                pub fn does_alert_block_ai_requests(_ctx: &AppContext) -> bool {
                    false
                }
            }

            impl Entity for PromptAlertView {
                type Event = PromptAlertEvent;
            }

            impl View for PromptAlertView {
                fn ui_name() -> &'static str {
                    "PromptAlertView"
                }

                fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl warpui::TypedActionView for PromptAlertView {
                type Action = ();

                fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
            }
        }

        pub mod plan_and_todo_list {
            use warpui::{Entity, TypedActionView, View};

            #[derive(Clone, Debug)]
            pub enum PlanAndTodoListEvent {
                OpenAIDocument {
                    document_id: crate::ai::document::AIDocumentId,
                    document_version: crate::ai::document::AIDocumentVersion,
                },
            }

            #[derive(Default)]
            pub struct PlanAndTodoListView;

            impl Entity for PlanAndTodoListView {
                type Event = PlanAndTodoListEvent;
            }

            impl PlanAndTodoListView {
                pub fn new<T, U>(
                    _data: T,
                    _menu: U,
                    _view_id: warpui::EntityId,
                    _is_in_agent_view: bool,
                    _ctx: &mut warpui::ViewContext<Self>,
                ) -> Self {
                    Self
                }

                pub fn should_render(&self, _app: &warpui::AppContext) -> bool {
                    false
                }
            }

            impl View for PlanAndTodoListView {
                fn ui_name() -> &'static str {
                    "PlanAndTodoListView"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl TypedActionView for PlanAndTodoListView {
                type Action = ();

                fn handle_action(
                    &mut self,
                    _action: &Self::Action,
                    _ctx: &mut warpui::ViewContext<Self>,
                ) {
                }
            }
        }
    }

    pub mod persistence {
        pub use super::PersistedAIInput;
        pub use crate::terminal::model::block::SerializedBlockListItem;
    }

    pub mod orchestration_events {
        use warpui::{Entity, SingletonEntity};

        #[derive(Clone, Debug, Default)]
        pub struct OrchestrationEventService;

        impl Entity for OrchestrationEventService {
            type Event = ();
        }

        impl SingletonEntity for OrchestrationEventService {}

        impl OrchestrationEventService {
            pub fn register_lifecycle_subscription<T, U, V>(
                &mut self,
                _child_conversation_id: T,
                _parent_agent_id: U,
                _lifecycle_subscription: V,
            ) {
            }
        }
    }

    pub mod summarization_cancel_dialog {
        use warpui::{Entity, View};

        #[derive(Default)]
        pub struct SummarizationCancelDialog;

        impl Entity for SummarizationCancelDialog {
            type Event = ();
        }

        impl View for SummarizationCancelDialog {
            fn ui_name() -> &'static str {
                "SummarizationCancelDialog"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }
    }
}

pub mod llms {
    use super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct LLMId {
        pub id: String,
    }

    impl std::fmt::Display for LLMId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            self.id.fmt(f)
        }
    }

    impl From<String> for LLMId {
        fn from(value: String) -> Self {
            Self { id: value }
        }
    }

    impl From<LLMId> for String {
        fn from(value: LLMId) -> Self {
            value.id
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct LLMModelHost(pub String);

    impl LLMModelHost {
        pub const DirectApi: Self = Self(String::new());
        pub const AwsBedrock: Self = Self(String::new());
        pub const Unknown: Self = Self(String::new());
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct ModelsByFeature;

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct LLMHostConfig {
        pub enabled: bool,
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct LLMInfo {
        pub id: LLMId,
        pub display_name: String,
        pub base_model_name: String,
        pub reasoning_level: Option<String>,
        pub description: Option<String>,
        pub disable_reason: Option<DisableReason>,
        pub vision_supported: bool,
        pub spec: Option<LLMSpec>,
        pub provider: LLMProvider,
        pub discount_percentage: Option<f32>,
        pub host_configs: std::collections::HashMap<LLMModelHost, LLMHostConfig>,
    }

    impl LLMInfo {
        pub fn menu_display_name(&self) -> String {
            self.display_name.clone()
        }

        pub fn has_reasoning_level(&self) -> bool {
            self.reasoning_level.is_some()
        }

        pub fn base_model_name(&self) -> String {
            if self.base_model_name.is_empty() {
                self.display_name.clone()
            } else {
                self.base_model_name.clone()
            }
        }

        pub fn reasoning_level(&self) -> Option<String> {
            self.reasoning_level.clone()
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub enum LLMProvider {
        OpenAI,
        Anthropic,
        Google,
        Xai,
        #[default]
        Unknown,
    }

    impl LLMProvider {
        pub fn icon(&self) -> Option<crate::ui_components::icons::Icon> {
            match self {
                Self::OpenAI => Some(crate::ui_components::icons::Icon::OpenAILogo),
                Self::Anthropic => Some(crate::ui_components::icons::Icon::ClaudeLogo),
                Self::Google => Some(crate::ui_components::icons::Icon::GeminiLogo),
                Self::Xai | Self::Unknown => None,
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
    pub struct LLMSpec {
        pub id: LLMId,
        pub cost: f32,
        pub quality: f32,
        pub speed: f32,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DisableReason {
        AdminDisabled,
        OutOfRequests,
        ProviderOutage,
        RequiresUpgrade,
        Unavailable,
    }

    impl Default for DisableReason {
        fn default() -> Self {
            Self::Unavailable
        }
    }

    impl DisableReason {
        pub fn tooltip_text(&self) -> &'static str {
            match self {
                Self::AdminDisabled => "This model has been disabled.",
                Self::OutOfRequests => "No local AI requests are available.",
                Self::ProviderOutage => "This model is unavailable.",
                Self::RequiresUpgrade => "This model is unavailable in Warp Lite.",
                Self::Unavailable => "This model is unavailable.",
            }
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct LLMUsageMetadata;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct RoutingHostConfig;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AvailableLLMs;

    pub fn is_using_api_key_for_provider<T, U>(_provider: T, _ctx: U) -> bool {
        false
    }

    pub fn dedupe_model_display_names<T>(_models: T) {}

    #[derive(Clone, Debug)]
    pub enum LLMPreferencesEvent {
        UpdatedAvailableLLMs,
        UpdatedActiveAgentModeLLM,
        UpdatedActiveCodingLLM,
    }

    #[derive(Clone, Debug, Default)]
    pub struct LLMPreferences;

    impl LLMPreferences {
        pub fn get_base_llm_override(&self, _id: warpui::EntityId) -> Option<String> {
            None
        }

        pub fn update_preferred_agent_mode_llm(
            &mut self,
            _llm_id: &LLMId,
            _terminal_id: warpui::EntityId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn update_feature_model_choices<T, U>(&mut self, _choices: T, _ctx: U) {}

        pub fn refresh_available_models<T>(&mut self, _ctx: T) {}

        pub fn get_active_base_model<T: ?Sized, U>(&self, _ctx: &T, _terminal_id: U) -> LLMInfo {
            LLMInfo::default()
        }

        pub fn get_active_cli_agent_model<T, U>(&self, _ctx: T, _terminal_id: U) -> LLMInfo {
            LLMInfo::default()
        }

        pub fn get_cli_agent_llm_choices(&self) -> std::slice::Iter<'static, LLMInfo> {
            static CHOICES: std::sync::LazyLock<Vec<LLMInfo>> = std::sync::LazyLock::new(Vec::new);
            CHOICES.iter()
        }

        pub fn get_base_llm_choices_for_agent_mode(&self) -> std::slice::Iter<'static, LLMInfo> {
            static CHOICES: std::sync::LazyLock<Vec<LLMInfo>> = std::sync::LazyLock::new(Vec::new);
            CHOICES.iter()
        }

        pub fn vision_supported<T: ?Sized, U>(&self, _ctx: &T, _terminal_id: U) -> bool {
            false
        }

        pub fn remove_llm_override<T, U>(&mut self, _terminal_id: T, _ctx: U) {}

        pub fn new_choices_since_last_update(&self) -> Vec<LLMInfo> {
            Vec::new()
        }

        pub fn hide_llm_popup<T>(&mut self, _ctx: T) {}

        pub fn should_show_new_choices_popup<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn mark_new_choices_popup_as_shown<T>(&self, _ctx: T) {}

        pub fn get_llm_info<T>(&self, _id: T) -> Option<LLMInfo> {
            None
        }

        pub fn get_default_base_model(&self) -> LLMInfo {
            LLMInfo::default()
        }

        pub fn get_preferred_codex_model(&self) -> Option<LLMId> {
            None
        }

        pub fn new<T>(_ctx: T) -> Self {
            Self
        }
    }

    impl Entity for LLMPreferences {
        type Event = LLMPreferencesEvent;
    }

    impl SingletonEntity for LLMPreferences {}
}

pub mod persisted_workspace {
    use super::*;
    use lsp::supported_servers::LSPServerType;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum EnablementState {
        #[default]
        Disabled,
        Enabled,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LspRepoStatus {
        Ready,
        Enabled,
        CheckingForInstallation,
        DisabledAndInstalled { server_type: LSPServerType },
        DisabledAndNotInstalled { server_type: LSPServerType },
        Installing { server_type: LSPServerType },
    }

    impl Default for LspRepoStatus {
        fn default() -> Self {
            Self::CheckingForInstallation
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum LSPInstallationStatus {
        Installed,
        NotInstalled,
        Checking,
        Installing,
    }

    impl LspRepoStatus {
        pub fn from_installation_status(
            status: &LSPInstallationStatus,
            server_type: LSPServerType,
        ) -> Self {
            match status {
                LSPInstallationStatus::Installed => Self::DisabledAndInstalled { server_type },
                LSPInstallationStatus::NotInstalled => {
                    Self::DisabledAndNotInstalled { server_type }
                }
                LSPInstallationStatus::Checking => Self::CheckingForInstallation,
                LSPInstallationStatus::Installing => Self::Installing { server_type },
            }
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum LSPEnablementResultForFile {
        Enabled,
        UnsupportedLanguage,
        LSPNotEnabled { root_name: Option<String> },
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub enum LspTask {
        Spawn {
            file_path: PathBuf,
        },
        Install {
            file_path: PathBuf,
            repo_root: PathBuf,
            server_type: LSPServerType,
        },
    }

    #[derive(Clone, Debug)]
    pub enum PersistedWorkspaceEvent {
        WorkspaceAdded {
            path: PathBuf,
        },
        AvailableServersDetected {
            workspace_path: PathBuf,
            servers: Vec<LSPServerType>,
        },
        InstallStatusUpdate {
            server_type: LSPServerType,
            status: LSPInstallationStatus,
        },
        InstallationSucceeded,
        InstallationFailed,
    }

    #[derive(Clone, Debug, Default)]
    pub struct PersistedWorkspace;

    impl PersistedWorkspace {
        pub fn new<T, U, V>(
            _workspaces: T,
            _servers: U,
            _maybe: V,
            _ctx: &mut ModelContext<Self>,
        ) -> Self {
            Self
        }

        pub fn workspaces(&self) -> std::vec::IntoIter<crate::ai::workspace::WorkspaceMetadata> {
            Vec::new().into_iter()
        }

        pub fn root_for_workspace(&self, _path: impl AsRef<std::path::Path>) -> Option<PathBuf> {
            None
        }

        pub fn total_lsp_server_count(&self, _enabled_only: bool) -> usize {
            0
        }

        pub fn enabled_lsp_servers(
            &self,
            _path: impl AsRef<std::path::Path>,
        ) -> Option<std::iter::Empty<LSPServerType>> {
            None
        }

        pub fn handle_index_metadata_event<T>(&mut self, _event: T, _ctx: &mut ModelContext<Self>) {
        }

        pub fn execute_lsp_task(&mut self, _task: LspTask, _ctx: &mut ModelContext<Self>) {}

        pub fn enable_lsp_server_for_path(
            &mut self,
            _repo_root: &PathBuf,
            _server_type: LSPServerType,
        ) {
        }

        pub fn disable_lsp_server_for_path(
            &mut self,
            _repo_root: &PathBuf,
            _server_type: LSPServerType,
        ) {
        }

        pub fn has_enabled_lsp_server_for_file_path(&self, _path: &PathBuf) -> bool {
            false
        }

        pub fn detect_available_servers_for_workspaces(
            &mut self,
            _paths: Vec<PathBuf>,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn detect_lsp_workspace_status(
            &mut self,
            _workspace_path: PathBuf,
            _server_type: LSPServerType,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn navigated_to_path<T, U: ?Sized>(&mut self, _path: T, _ctx: &mut U) {}

        pub fn user_added_workspace<T, U>(&mut self, _path: T, _ctx: U) {}
    }

    impl Entity for PersistedWorkspace {
        type Event = PersistedWorkspaceEvent;
    }

    impl SingletonEntity for PersistedWorkspace {}
}

pub mod workspace {
    use super::*;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct WorkspaceMetadata {
        pub path: PathBuf,
    }

    impl WorkspaceMetadata {
        pub fn most_recently_navigated(_a: &Self, _b: &Self) -> std::cmp::Ordering {
            std::cmp::Ordering::Equal
        }
    }
}

pub mod skills {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum SkillOpenOrigin {
        #[default]
        Unknown,
        ReadSkill,
        ReadFiles,
        EditFiles,
        OpenSkillCommand,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum SkillReference {
        Path(PathBuf),
        BundledSkillId(String),
    }

    impl Default for SkillReference {
        fn default() -> Self {
            Self::Path(PathBuf::new())
        }
    }

    impl std::fmt::Display for SkillReference {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Self::Path(path) => write!(f, "{}", path.display()),
                Self::BundledSkillId(id) => f.write_str(id),
            }
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ParsedSkill {
        pub path: PathBuf,
        pub name: String,
        pub description: String,
        pub provider: SkillProvider,
        pub scope: SkillScope,
    }

    impl std::fmt::Display for ParsedSkill {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str(&self.name)
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum SkillProvider {
        #[default]
        Warp,
        Agents,
        Claude,
        Codex,
        Cursor,
        Gemini,
        Copilot,
        Droid,
        Github,
        OpenCode,
    }

    impl SkillProvider {
        pub fn icon(&self) -> warp_core::ui::icons::Icon {
            warp_core::ui::icons::Icon::Terminal
        }

        pub fn icon_fill(&self, color: warp_core::ui::theme::Fill) -> warp_core::ui::theme::Fill {
            color
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct SkillDescriptor {
        pub reference: SkillReference,
        pub name: String,
        pub description: String,
        pub scope: SkillScope,
        pub provider: SkillProvider,
        pub icon_override: Option<warp_core::ui::icons::Icon>,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum SkillScope {
        #[default]
        Home,
        Project,
        Bundled,
    }

    #[derive(Clone, Debug)]
    pub enum SkillTelemetryEvent {
        Read {
            reference: SkillReference,
            name: Option<String>,
            scope: Option<SkillScope>,
            provider: Option<SkillProvider>,
            error: bool,
        },
        Opened {
            reference: SkillReference,
            name: Option<String>,
            origin: SkillOpenOrigin,
        },
    }

    warp_core::register_telemetry_event!(SkillTelemetryEvent);

    impl warp_core::telemetry::TelemetryEvent for SkillTelemetryEvent {
        fn name(&self) -> &'static str {
            "Skill Removed"
        }

        fn payload(&self) -> Option<serde_json::Value> {
            None
        }

        fn description(&self) -> &'static str {
            "Skill telemetry removed in Warp Lite"
        }

        fn enablement_state(&self) -> warp_core::telemetry::EnablementState {
            warp_core::telemetry::EnablementState::Always
        }

        fn contains_ugc(&self) -> bool {
            false
        }

        fn event_descs() -> impl Iterator<Item = Box<dyn warp_core::telemetry::TelemetryEventDesc>>
        {
            std::iter::empty()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct SkillManager;

    impl SkillManager {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn get_skills_for_working_directory<T, U>(
            &self,
            _cwd: T,
            _ctx: U,
        ) -> Vec<SkillDescriptor> {
            Vec::new()
        }

        pub fn skill_exists_for_any_provider<T, U>(&self, _skill: T, _providers: U) -> bool {
            false
        }

        pub fn best_supported_provider<U>(
            &self,
            skill: &SkillDescriptor,
            _providers: U,
        ) -> SkillProvider {
            skill.provider
        }

        pub fn skill_by_reference<T>(&self, _reference: T) -> Option<&ParsedSkill> {
            None
        }

        pub fn active_bundled_skill<T, U: ?Sized>(
            &self,
            _name: T,
            _ctx: &U,
        ) -> Option<&ParsedSkill> {
            None
        }
    }

    impl Entity for SkillManager {
        type Event = ();
    }

    impl SingletonEntity for SkillManager {}
}

pub mod document {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct AIDocumentId(pub uuid::Uuid);

    impl std::fmt::Display for AIDocumentId {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    impl TryFrom<String> for AIDocumentId {
        type Error = uuid::Error;

        fn try_from(value: String) -> Result<Self, Self::Error> {
            Self::try_from(value.as_str())
        }
    }

    impl From<AIDocumentId> for String {
        fn from(value: AIDocumentId) -> Self {
            value.to_string()
        }
    }

    impl TryFrom<&str> for AIDocumentId {
        type Error = uuid::Error;

        fn try_from(value: &str) -> Result<Self, Self::Error> {
            Ok(Self(uuid::Uuid::parse_str(value)?))
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AIDocumentVersion(pub usize);

    #[derive(Clone, Debug)]
    pub enum AIDocumentModelEvent {
        DocumentVisibilityChanged(()),
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct AIDocument {
        pub title: String,
        pub version: AIDocumentVersion,
        pub created_at: chrono::DateTime<chrono::Local>,
    }

    #[derive(Clone, Debug, Default)]
    pub struct AIDocumentModel;

    impl AIDocumentModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn apply_persisted_content(
            &mut self,
            _id: AIDocumentId,
            _content: &str,
            _title: Option<&str>,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn is_document_visible_by_conversation_in_pane_group<T>(
            &self,
            _conversation_id: &crate::ai::agent::conversation::AIConversationId,
            _pane_group_id: T,
        ) -> bool {
            false
        }

        pub fn get_all_documents_for_conversation(
            &self,
            _conversation_id: crate::ai::agent::conversation::AIConversationId,
        ) -> Vec<(AIDocumentId, AIDocument)> {
            Vec::new()
        }

        pub fn get_conversation_id_for_document_id(
            &self,
            _id: &AIDocumentId,
        ) -> Option<crate::ai::agent::conversation::AIConversationId> {
            None
        }

        pub fn get_document_content<T>(&self, _id: &AIDocumentId, _app: T) -> Option<String> {
            None
        }

        pub fn get_current_document(&self, _id: &AIDocumentId) -> Option<&AIDocument> {
            None
        }

        pub fn set_document_visible<T, U>(
            &mut self,
            _id: &AIDocumentId,
            _pane_group_id: T,
            _visible: bool,
            _ctx: U,
        ) {
        }
    }

    impl Entity for AIDocumentModel {
        type Event = AIDocumentModelEvent;
    }

    impl SingletonEntity for AIDocumentModel {}

    pub mod ai_document_model {
        pub use super::{
            AIDocument, AIDocumentId, AIDocumentModel, AIDocumentModelEvent, AIDocumentVersion,
        };
    }
}

pub mod ai_document_view {
    use super::document::{AIDocumentId, AIDocumentVersion};
    use warpui::{Entity, ModelHandle, TypedActionView, ViewContext};

    pub const DEFAULT_PLANNING_DOCUMENT_TITLE: &str = "Plan";

    pub struct AIDocumentView {
        id: AIDocumentId,
        version: AIDocumentVersion,
        pane_configuration: ModelHandle<crate::pane_group::pane::PaneConfiguration>,
    }

    #[derive(Clone, Debug)]
    pub enum AIDocumentEvent {
        Pane(crate::pane_group::pane::PaneEvent),
        CloseRequested,
        ViewInWarpDrive(AIDocumentId),
        OpenCodeInWarp {
            source: crate::code::editor_management::CodeSource,
            layout: crate::util::file::external_editor::settings::EditorLayout,
            line_col: Option<warp_util::path::LineAndColumnArg>,
        },
        OpenFileWithTarget {
            path: std::path::PathBuf,
            target: crate::util::openable_file_type::FileTarget,
            line_col: Option<warp_util::path::LineAndColumnArg>,
        },
        AttachPlanAsContext(AIDocumentId),
    }

    impl AIDocumentView {
        pub fn new(
            _id: AIDocumentId,
            _version: AIDocumentVersion,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self {
                id: _id,
                version: _version,
                pane_configuration: _ctx
                    .add_model(|_| crate::pane_group::pane::PaneConfiguration::new("AI Document")),
            }
        }

        pub fn pane_configuration(
            &self,
        ) -> &ModelHandle<crate::pane_group::pane::PaneConfiguration> {
            &self.pane_configuration
        }

        pub fn document_id(&self) -> &AIDocumentId {
            &self.id
        }

        pub fn document_version(&self) -> AIDocumentVersion {
            self.version
        }

        pub fn bind_window<T, U>(&mut self, _window_id: T, _ctx: U) {}

        pub fn focus<T>(&mut self, _ctx: T) {}

        pub fn set_original_terminal_view<T>(&mut self, _terminal_view: T) {}

        pub fn selected_text<T>(&self, _ctx: T) -> Option<String> {
            None
        }
    }

    impl Entity for AIDocumentView {
        type Event = AIDocumentEvent;
    }

    impl TypedActionView for AIDocumentView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }
}

pub mod facts {
    use super::*;
    use warpui::{ModelHandle, TypedActionView, View, ViewContext};

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AIFact {
        Memory(AIMemory),
    }

    impl Default for AIFact {
        fn default() -> Self {
            Self::Memory(AIMemory::default())
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AIMemory {
        pub content: String,
        pub name: Option<String>,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CloudAIFact;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct CloudAIFactModel {
        pub string_model: AIFact,
    }

    #[derive(Clone, Debug, Default)]
    pub struct AIFactManager {
        views: std::collections::HashMap<warpui::WindowId, warpui::ViewHandle<AIFactView>>,
    }

    impl AIFactManager {
        pub fn new() -> Self {
            Self {
                views: std::collections::HashMap::new(),
            }
        }

        pub fn find_pane<T>(&self, _window_id: T) -> Option<crate::workspace::PaneViewLocator> {
            None
        }
    }

    impl Entity for AIFactManager {
        type Event = ();
    }

    impl SingletonEntity for AIFactManager {}

    pub struct AIFactView {
        pane_configuration: ModelHandle<crate::pane_group::pane::PaneConfiguration>,
    }

    impl AIFactView {
        pub fn new(ctx: &mut ViewContext<Self>) -> Self {
            Self {
                pane_configuration: ctx
                    .add_model(|_| crate::pane_group::pane::PaneConfiguration::new("AI Facts")),
            }
        }

        pub fn pane_configuration(
            &self,
        ) -> ModelHandle<crate::pane_group::pane::PaneConfiguration> {
            self.pane_configuration.clone()
        }

        pub fn focus<T>(&mut self, _ctx: T) {}

        pub fn update_page<T, U>(&mut self, _page: T, _ctx: U) {}
    }

    impl Entity for AIFactView {
        type Event = AIFactViewEvent;
    }

    impl TypedActionView for AIFactView {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    #[derive(Clone, Debug)]
    pub enum AIFactViewEvent {
        Pane(crate::pane_group::pane::PaneEvent),
        OpenSettings,
        OpenFile(std::path::PathBuf),
        InitializeProject(std::path::PathBuf),
    }

    impl AIFactManager {
        pub fn ai_fact_view(
            &self,
            window_id: warpui::WindowId,
        ) -> Option<warpui::ViewHandle<AIFactView>> {
            self.views.get(&window_id).cloned()
        }

        pub fn register_view(
            &mut self,
            window_id: warpui::WindowId,
            view: warpui::ViewHandle<AIFactView>,
        ) {
            self.views.insert(window_id, view);
        }

        pub fn register_pane<T, U, V, W>(
            &mut self,
            _pane: T,
            _pane_group_id: U,
            _window_id: V,
            _ctx: W,
        ) {
        }

        pub fn deregister_pane<T, U>(&mut self, _window_id: T, _ctx: U) {}
    }

    pub mod view {
        #[derive(Clone, Debug)]
        pub enum AIFactPage {
            Rules,
            RuleEditor {
                sync_id: Option<crate::server::ids::SyncId>,
            },
        }

        impl Default for AIFactPage {
            fn default() -> Self {
                Self::Rules
            }
        }
    }
}

pub mod execution_profiles {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct ClientProfileId(pub uuid::Uuid);

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AIExecutionProfile {
        pub name: String,
        pub is_default_profile: bool,
    }

    impl AIExecutionProfile {
        pub fn display_name(&self) -> String {
            if self.is_default_profile {
                "Default".to_owned()
            } else if self.name.trim().is_empty() {
                "Terminal".to_owned()
            } else {
                self.name.clone()
            }
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ActionPermission {
        #[default]
        Unknown,
        AgentDecides,
        AlwaysAllow,
        AlwaysAsk,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum WriteToPtyPermission {
        #[default]
        Unknown,
        AlwaysAllow,
        AlwaysAsk,
        AskOnFirstWrite,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AskUserQuestionPermission {
        #[default]
        Unknown,
        AlwaysAsk,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ComputerUsePermission {
        #[default]
        Unknown,
        Never,
        AlwaysAsk,
        AlwaysAllow,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct CloudAgentComputerUseState {
        pub enabled: bool,
        pub permission: ComputerUsePermission,
    }

    impl ComputerUsePermission {
        pub fn cloud_agent_state(self) -> CloudAgentComputerUseState {
            CloudAgentComputerUseState {
                enabled: matches!(self, Self::AlwaysAllow),
                permission: self,
            }
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CloudAIExecutionProfile;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct CloudAIExecutionProfileModel;

    #[derive(Clone, Debug, Default)]
    pub struct ExecutionProfileEditorManager;

    #[derive(Clone, Debug)]
    pub enum AIExecutionProfilesModelEvent {
        ProfileUpdated(ClientProfileId),
        ProfileCreated,
        ProfileDeleted,
        UpdatedActiveProfile { terminal_view_id: warpui::EntityId },
    }

    #[derive(Clone, Debug, Default)]
    pub struct AIExecutionProfileInfo {
        id: ClientProfileId,
        sync_id: Option<crate::server::ids::SyncId>,
        data: AIExecutionProfile,
    }

    impl AIExecutionProfileInfo {
        pub fn id(&self) -> &ClientProfileId {
            &self.id
        }

        pub fn sync_id(&self) -> Option<crate::server::ids::SyncId> {
            self.sync_id
        }

        pub fn data(&self) -> &AIExecutionProfile {
            &self.data
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct AIExecutionProfilesModel;

    impl AIExecutionProfilesModel {
        pub fn new<T, U>(_launch_mode: T, _ctx: U) -> Self {
            Self
        }

        pub fn reset(&mut self) {}

        pub fn active_profile(
            &self,
            _terminal_id: Option<warpui::EntityId>,
            _app: &AppContext,
        ) -> ActiveProfile {
            ActiveProfile::default()
        }

        pub fn get_profile_id_by_sync_id(
            &self,
            _sync_id: &crate::server::ids::SyncId,
        ) -> Option<ClientProfileId> {
            None
        }

        pub fn get_profile_by_id<T>(
            &self,
            profile_id: ClientProfileId,
            _ctx: T,
        ) -> Option<AIExecutionProfileInfo> {
            Some(AIExecutionProfileInfo {
                id: profile_id,
                sync_id: None,
                data: AIExecutionProfile {
                    name: "Terminal".to_owned(),
                    is_default_profile: true,
                },
            })
        }

        pub fn get_all_profile_ids(&self) -> Vec<ClientProfileId> {
            Vec::new()
        }

        pub fn has_multiple_profiles(&self) -> bool {
            false
        }

        pub fn set_active_profile(
            &mut self,
            _terminal_id: warpui::EntityId,
            _profile_id: ClientProfileId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn default_profile(&self, _ctx: &AppContext) -> ActiveProfile {
            ActiveProfile::default()
        }

        pub fn default_profile_id(&self) -> ClientProfileId {
            ClientProfileId::default()
        }

        pub fn set_base_model<T>(
            &mut self,
            _profile_id: ClientProfileId,
            _model_id: Option<T>,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_apply_code_diffs<T>(
            &mut self,
            _profile_id: ClientProfileId,
            _value: &T,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_read_files<T>(
            &mut self,
            _profile_id: ClientProfileId,
            _value: &T,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_execute_commands<T>(
            &mut self,
            _profile_id: ClientProfileId,
            _value: &T,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_mcp_permissions<T>(
            &mut self,
            _profile_id: ClientProfileId,
            _value: &T,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_write_to_pty<T>(
            &mut self,
            _profile_id: ClientProfileId,
            _value: &T,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn set_cli_agent_model<T, U>(
            &mut self,
            _profile_id: ClientProfileId,
            _model_id: Option<T>,
            _ctx: U,
        ) {
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct ActiveProfile {
        id: ClientProfileId,
    }

    impl ActiveProfile {
        pub fn id(&self) -> &ClientProfileId {
            &self.id
        }

        pub fn sync_id(&self) -> Option<crate::server::ids::SyncId> {
            None
        }

        pub fn data(&self) -> AIExecutionProfile {
            AIExecutionProfile {
                name: "Terminal".to_owned(),
                is_default_profile: true,
            }
        }
    }

    impl Entity for AIExecutionProfilesModel {
        type Event = AIExecutionProfilesModelEvent;
    }

    impl SingletonEntity for AIExecutionProfilesModel {}

    pub mod profiles {
        pub use super::{
            AIExecutionProfileInfo, AIExecutionProfilesModel, AIExecutionProfilesModelEvent,
            ClientProfileId,
        };
    }

    pub mod editor {
        use warpui::{Entity, ModelHandle, SingletonEntity, TypedActionView, ViewContext};

        pub struct ExecutionProfileEditorView {
            profile_id: super::ClientProfileId,
            pane_configuration: ModelHandle<crate::pane_group::pane::PaneConfiguration>,
        }

        #[derive(Clone, Debug)]
        pub enum ExecutionProfileEditorViewEvent {
            Pane(crate::pane_group::pane::PaneEvent),
        }

        #[derive(Clone, Debug, Default)]
        pub struct ExecutionProfileEditorManager;

        impl ExecutionProfileEditorView {
            pub fn new(profile_id: super::ClientProfileId, ctx: &mut ViewContext<Self>) -> Self {
                Self {
                    profile_id,
                    pane_configuration: ctx.add_model(|_| {
                        crate::pane_group::pane::PaneConfiguration::new("Execution Profile")
                    }),
                }
            }

            pub fn pane_configuration(
                &self,
            ) -> ModelHandle<crate::pane_group::pane::PaneConfiguration> {
                self.pane_configuration.clone()
            }

            pub fn profile_id(&self) -> super::ClientProfileId {
                self.profile_id
            }

            pub fn focus<T>(&mut self, _ctx: T) {}
        }

        impl Entity for ExecutionProfileEditorView {
            type Event = ExecutionProfileEditorViewEvent;
        }

        impl TypedActionView for ExecutionProfileEditorView {
            type Action = ();

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl ExecutionProfileEditorManager {
            pub fn find_pane<T, U>(
                &self,
                _window_id: T,
                _profile_id: U,
            ) -> Option<crate::workspace::PaneViewLocator> {
                None
            }

            pub fn register_pane<T, U, V, W, X>(
                &mut self,
                _pane: T,
                _pane_group_id: U,
                _window_id: V,
                _profile_id: W,
                _ctx: X,
            ) {
            }

            pub fn deregister_pane<T, U>(&mut self, _window_id: T, _profile_id: U) {}
        }

        impl Entity for ExecutionProfileEditorManager {
            type Event = ();
        }

        impl SingletonEntity for ExecutionProfileEditorManager {}
    }

    pub mod model_menu_items {
        pub fn available_model_menu_items<T>(_ctx: T) -> Vec<()> {
            Vec::new()
        }

        pub fn has_reasoning_variants<T>(_model: T) -> bool {
            false
        }

        pub fn is_auto<T>(_model: T) -> bool {
            false
        }
    }
}

pub mod cloud_environments {
    use super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct GithubRepo {
        pub repo: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AmbientAgentEnvironment {
        pub base_image: String,
        pub github_repos: Vec<GithubRepo>,
        pub name: String,
    }

    impl AmbientAgentEnvironment {
        pub fn display_name(&self) -> String {
            self.name.clone()
        }
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct CloudAmbientAgentEnvironment {
        pub id: crate::server::ids::SyncId,
        pub model: CloudAmbientAgentEnvironmentModel,
    }

    impl Default for CloudAmbientAgentEnvironment {
        fn default() -> Self {
            Self {
                id: crate::server::ids::SyncId::ClientId(crate::server::ids::ClientId::new()),
                model: CloudAmbientAgentEnvironmentModel::default(),
            }
        }
    }

    impl CloudAmbientAgentEnvironment {
        pub fn get_all(_ctx: &AppContext) -> Vec<Self> {
            Vec::new()
        }

        pub fn get_by_id<T>(_id: T, _ctx: &AppContext) -> Option<Self> {
            None
        }

        pub fn model(&self) -> &CloudAmbientAgentEnvironmentModel {
            &self.model
        }
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CloudAmbientAgentEnvironmentModel {
        pub string_model: AmbientAgentEnvironment,
    }

    pub fn owner_for_new_environment(_ctx: &AppContext) -> Option<crate::cloud_object::Owner> {
        None
    }

    pub fn owner_for_new_personal_environment(
        _ctx: &AppContext,
    ) -> Option<crate::cloud_object::Owner> {
        None
    }
}

pub mod ambient_agents {
    use super::*;
    use std::time::Duration;

    pub type AmbientAgentTaskId = uuid::Uuid;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AgentConfigSnapshot {
        pub environment_id: Option<String>,
        pub model_id: Option<String>,
        pub worker_host: Option<String>,
        pub computer_use_enabled: Option<bool>,
        pub harness: Option<task::HarnessConfig>,
        pub name: Option<String>,
    }

    pub mod task {
        use serde::{Deserialize, Serialize};
        use warp_cli::agent::Harness;

        #[derive(Clone, Copy, Debug)]
        pub struct HarnessConfig {
            pub harness_type: Harness,
        }

        impl Serialize for HarnessConfig {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
            where
                S: serde::Serializer,
            {
                serializer.serialize_str(&self.harness_type.to_string())
            }
        }

        impl<'de> Deserialize<'de> for HarnessConfig {
            fn deserialize<D>(_deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                Ok(Self::default())
            }
        }

        impl Default for HarnessConfig {
            fn default() -> Self {
                Self {
                    harness_type: Harness::Oz,
                }
            }
        }

        impl HarnessConfig {
            pub fn from_harness_type(harness_type: Harness) -> Self {
                Self { harness_type }
            }
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct AttachmentInput {
            pub file_name: String,
            pub mime_type: String,
            pub data: String,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct TaskAttachment {
            pub attachment_id: String,
            pub filename: String,
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AmbientAgentTaskState {
        #[default]
        Unknown,
        Pending,
        Queued,
        Claimed,
        InProgress,
        Succeeded,
        Failed,
        Error,
        Blocked,
        Cancelled,
    }

    impl AmbientAgentTaskState {
        pub fn is_failure_like(&self) -> bool {
            matches!(
                self,
                Self::Failed | Self::Error | Self::Blocked | Self::Unknown
            )
        }

        pub fn as_query_param(&self) -> Option<&'static str> {
            Some(match self {
                Self::Unknown => "UNKNOWN",
                Self::Pending => "PENDING",
                Self::Queued => "QUEUED",
                Self::Claimed => "CLAIMED",
                Self::InProgress => "IN_PROGRESS",
                Self::Succeeded => "SUCCEEDED",
                Self::Failed => "FAILED",
                Self::Error => "ERROR",
                Self::Blocked => "BLOCKED",
                Self::Cancelled => "CANCELLED",
            })
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AgentSource {
        pub name: Option<String>,
    }

    impl AgentSource {
        pub const GitHubAction: Self = Self { name: None };
        pub const Cli: Self = Self { name: None };
        pub const CloudMode: Self = Self { name: None };

        pub fn display_name(&self) -> &str {
            self.name.as_deref().unwrap_or("Agent")
        }

        pub fn as_str(&self) -> &str {
            self.name.as_deref().unwrap_or("agent")
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TaskStatusMessage {
        pub message: String,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct AmbientAgentTask {
        pub task_id: AmbientAgentTaskId,
        pub title: String,
        pub conversation_id: Option<String>,
        pub updated_at: chrono::DateTime<chrono::Utc>,
        pub source: Option<AgentSource>,
        pub agent_config_snapshot: Option<AgentConfigSnapshot>,
        pub state: AmbientAgentTaskState,
        pub status_message: Option<TaskStatusMessage>,
        pub artifacts: Vec<crate::ai::artifacts::Artifact>,
    }

    impl AmbientAgentTask {
        pub fn run_time(&self) -> Option<Duration> {
            None
        }

        pub fn credits_used(&self) -> Option<f32> {
            None
        }
    }

    #[derive(Clone, Debug)]
    pub enum AmbientConversationStatus {
        Unknown,
        Error { error: String },
    }

    pub fn conversation_output_status_from_conversation<T>(
        _conversation: T,
    ) -> Option<AmbientConversationStatus> {
        None
    }

    pub const OUT_OF_CREDITS_TASK_FAILURE_MESSAGE: &str = "Out of credits";
    pub const SERVER_OVERLOADED_TASK_FAILURE_MESSAGE: &str = "Server overloaded";

    pub mod scheduled {
        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct ScheduledAmbientAgent;

        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct CloudScheduledAmbientAgent;

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct CloudScheduledAmbientAgentModel;

        impl CloudScheduledAmbientAgentModel {
            pub fn new<T>(_agent: T) -> Self {
                Self
            }
        }
    }

    pub mod telemetry {
        #[derive(Clone, Debug)]
        pub enum CloudAgentTelemetryEvent {
            EnteredCloudMode { entry_point: CloudModeEntryPoint },
        }

        warp_core::register_telemetry_event!(CloudAgentTelemetryEvent);

        impl warp_core::telemetry::TelemetryEvent for CloudAgentTelemetryEvent {
            fn name(&self) -> &'static str {
                "CloudAgent.Removed"
            }

            fn payload(&self) -> Option<serde_json::Value> {
                None
            }

            fn description(&self) -> &'static str {
                "Cloud agent telemetry removed in Warp Lite"
            }

            fn enablement_state(&self) -> warp_core::telemetry::EnablementState {
                warp_core::telemetry::EnablementState::Always
            }

            fn contains_ugc(&self) -> bool {
                false
            }

            fn event_descs(
            ) -> impl Iterator<Item = Box<dyn warp_core::telemetry::TelemetryEventDesc>>
            {
                std::iter::empty()
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub enum CloudModeEntryPoint {
            #[default]
            Unknown,
            OzLaunchModal,
            NewTab,
        }
    }

    pub mod github_auth_notifier {
        use warpui::{Entity, SingletonEntity};

        #[derive(Clone, Debug, Default)]
        pub struct GitHubAuthNotifier;

        #[derive(Clone, Debug)]
        pub enum GitHubAuthEvent {}

        impl GitHubAuthNotifier {
            pub fn notify_auth_completed(&mut self, _ctx: &mut warpui::ModelContext<Self>) {}
        }

        impl Entity for GitHubAuthNotifier {
            type Event = GitHubAuthEvent;
        }

        impl SingletonEntity for GitHubAuthNotifier {}
    }

    pub mod spawn {
        use futures::stream::{self, BoxStream};

        use crate::server::server_api::ai::{AIClient, SpawnAgentRequest};

        use super::{AmbientAgentTaskId, AmbientAgentTaskState, TaskStatusMessage};

        #[derive(Clone, Debug, Default)]
        pub struct SessionJoinInfo {
            pub session_id: Option<session_sharing_protocol::common::SessionId>,
        }

        #[derive(Clone, Debug)]
        pub enum AmbientAgentEvent {
            TaskSpawned {
                task_id: AmbientAgentTaskId,
                run_id: String,
            },
            StateChanged {
                state: AmbientAgentTaskState,
                status_message: Option<TaskStatusMessage>,
            },
            SessionStarted {
                session_join_info: SessionJoinInfo,
            },
            AtCapacity,
            Error(String),
        }

        pub fn spawn_task(
            _request: SpawnAgentRequest,
            _ai_client: std::sync::Arc<dyn AIClient>,
            _token: Option<String>,
        ) -> BoxStream<'static, anyhow::Result<AmbientAgentEvent>> {
            Box::pin(stream::empty())
        }
    }
}

pub mod cloud_agent_config {
    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct CloudAgentConfigModel;
}

pub mod cloud_agent_settings {
    #[derive(Clone, Debug, Default)]
    pub struct CloudAgentSettings;

    impl CloudAgentSettings {
        pub fn register<T>(_ctx: T) {}
    }
}

pub mod attachment_utils {
    pub const MAX_ATTACHMENT_SIZE_BYTES: usize = 10 * 1024 * 1024;
}

pub mod block_context {
    #[derive(Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
    pub struct BlockContext {
        pub id: crate::terminal::model::block::BlockId,
        pub index: crate::terminal::model::terminal_model::BlockIndex,
        pub command: String,
    }

    impl BlockContext {
        pub fn from_completed_block<T>(_block: T) -> Box<Self> {
            Box::default()
        }
    }
}

pub mod onboarding {
    #[derive(Clone, Debug, Default)]
    pub struct OnboardingModels;

    #[derive(Clone, Debug, Default)]
    pub struct OnboardingAuthState;

    pub fn apply_free_tier_default_model_override<T, U, V>(
        _models: T,
        default_model_id: U,
        _ctx: V,
    ) -> U {
        default_model_id
    }

    pub fn build_onboarding_models<T>(
        _ctx: T,
    ) -> (OnboardingModels, Option<crate::ai::llms::LLMId>) {
        (OnboardingModels, None)
    }

    pub fn current_onboarding_auth_state<T>(_ctx: T) -> OnboardingAuthState {
        OnboardingAuthState
    }
}

pub mod mcp {
    use super::*;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Author;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct CloudMCPServer;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct MCPServer;

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub struct CloudMCPServerModel;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TemplatableMCPServer;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TemplatableMCPServerInstallation {
        uuid: uuid::Uuid,
        server: TemplatableMCPServer,
        variables: Vec<TemplateVariable>,
    }

    impl TemplatableMCPServerInstallation {
        pub fn new<T, U, V>(_uuid: T, _server: U, _variable_values: V) -> Self {
            Self::default()
        }

        pub fn uuid(&self) -> uuid::Uuid {
            self.uuid
        }

        pub fn templatable_mcp_server(&self) -> &TemplatableMCPServer {
            &self.server
        }

        pub fn variable_values(&self) -> &[TemplateVariable] {
            &self.variables
        }
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct TemplateVariable;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct MCPServerUpdate;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct FileBasedMCPManager;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct MCPGalleryManager;

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct MCPProvider;

    #[derive(Clone, Debug, Default)]
    pub struct FileMCPWatcher;

    #[derive(Clone, Debug)]
    pub enum FileMCPWatcherEvent {}

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub enum TransportType {
        #[default]
        Stdio,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq)]
    pub enum MCPServerState {
        #[default]
        Stopped,
    }

    #[derive(Clone, Debug)]
    pub enum TemplatableMCPServerManagerEvent {
        TemplatableMCPServersUpdated,
        LegacyServerConverted,
        ServerInstallationAdded(uuid::Uuid),
        ServerInstallationDeleted(uuid::Uuid),
        StateChanged {
            uuid: uuid::Uuid,
            state: MCPServerState,
        },
    }

    #[derive(Clone, Debug, Default)]
    pub struct TemplatableMCPServerManager;

    impl TemplatableMCPServerManager {
        pub fn handle_oauth_callback<T>(&mut self, _url: T) -> anyhow::Result<()> {
            Ok(())
        }

        pub fn get_all_cloud_synced_mcp_servers<T>(_ctx: T) -> Vec<uuid::Uuid> {
            Vec::new()
        }

        pub fn get_all_runnable_mcp_servers<T>(_ctx: T) -> Vec<uuid::Uuid> {
            Vec::new()
        }

        pub fn get_first_team_space_id<T>(_ctx: T) -> Option<crate::server::ids::ServerId> {
            None
        }

        pub fn get_mcp_name<T>(_uuid: &uuid::Uuid, _ctx: T) -> Option<String> {
            None
        }

        pub fn get_template_uuid<T>(&self, _uuid: T) -> Option<uuid::Uuid> {
            None
        }

        pub fn get_templatable_mcp_server<T>(&self, _uuid: T) -> Option<TemplatableMCPServer> {
            None
        }

        pub fn get_installed_server<T>(
            &self,
            _uuid: T,
        ) -> Option<TemplatableMCPServerInstallation> {
            None
        }

        pub fn get_server_state<T>(&self, _uuid: T) -> MCPServerState {
            MCPServerState::Stopped
        }

        pub fn is_author<T, U>(&self, _uuid: T, _ctx: U) -> bool {
            false
        }

        pub fn is_shared<T, U>(&self, _uuid: T, _ctx: U) -> bool {
            false
        }

        pub fn install_figma_from_gallery<T>(&mut self, _ctx: T) {}

        pub fn enable_figma_mcp<T>(&mut self, _ctx: T) {}
    }

    impl Entity for TemplatableMCPServerManager {
        type Event = TemplatableMCPServerManagerEvent;
    }

    impl SingletonEntity for TemplatableMCPServerManager {}

    pub mod templatable_manager {
        pub use super::{TemplatableMCPServerManager, TemplatableMCPServerManagerEvent};
    }

    pub mod templatable {
        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct CloudTemplatableMCPServer;

        #[derive(Clone, Debug, Default, PartialEq, Eq)]
        pub struct CloudTemplatableMCPServerModel;

        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct TemplatableMCPServer;

        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct GalleryData;
    }

    pub mod templatable_installation {
        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub enum VariableType {
            #[default]
            String,
        }

        #[derive(Clone, Debug, Default, serde::Serialize, serde::Deserialize)]
        pub struct VariableValue;
    }

    pub mod parsing {
        #[derive(Clone, Debug, Default)]
        pub struct ParsedTemplatableMCPServerResult;

        pub fn prettify_json(input: &str) -> String {
            input.to_owned()
        }

        pub fn resolve_json(input: &str) -> anyhow::Result<String> {
            Ok(input.to_owned())
        }
    }

    pub mod file_based_manager {
        #[derive(Clone, Debug)]
        pub enum FileBasedMCPManagerEvent {}
    }

    pub mod gallery {
        #[derive(Clone, Debug)]
        pub enum MCPGalleryManagerEvent {}

        pub use super::MCPGalleryManager;
    }

    pub mod logs {
        use std::path::PathBuf;

        pub fn log_file_path_from_uuid(_uuid: &uuid::Uuid) -> PathBuf {
            PathBuf::new()
        }
    }
}

pub mod harness_display {
    use crate::ui_components::icons::Icon;
    use warp_cli::agent::Harness;

    pub fn display_name(harness: Harness) -> &'static str {
        match harness {
            Harness::Claude => "Claude",
            Harness::Gemini => "Gemini",
            Harness::Oz => "Oz",
            Harness::OpenCode | Harness::Unknown => "Terminal",
        }
    }

    pub fn icon_for(_harness: Harness) -> Icon {
        Icon::Terminal
    }
}

pub mod agent_tips {
    use markdown_parser::FormattedTextFragment;
    use warpui::keymap::Keystroke;
    use warpui::{AppContext, Entity, ModelContext};

    pub trait AITip: Clone {
        fn keystroke(&self, _app: &AppContext) -> Option<Keystroke>;
        fn link(&self) -> Option<String>;
        fn description(&self) -> &str;

        fn to_formatted_text(&self, _app: &AppContext) -> Vec<FormattedTextFragment> {
            vec![FormattedTextFragment::plain_text(
                self.description().to_owned(),
            )]
        }
    }

    pub struct AITipModel<T: AITip> {
        tips: Vec<T>,
        current_tip: Option<T>,
    }

    impl<T: AITip + 'static> AITipModel<T> {
        pub fn new(tips: Vec<T>) -> Self {
            let current_tip = tips.first().cloned();
            Self { tips, current_tip }
        }

        pub fn current_tip(&self) -> Option<&T> {
            self.current_tip.as_ref()
        }

        pub fn maybe_refresh_tip(&mut self, _ctx: &mut ModelContext<Self>) {
            self.current_tip = self.tips.first().cloned();
        }

        pub fn reset_cooldown(&mut self, _ctx: &mut ModelContext<Self>) {}
    }

    impl<T: AITip + 'static> Entity for AITipModel<T> {
        type Event = ();
    }
}

pub mod index {
    #[derive(Clone, Debug, Default)]
    pub struct Symbol {
        pub type_prefix: Option<String>,
        pub name: String,
        pub line_number: usize,
    }

    pub mod full_source_code_embedding {
        #[derive(Clone, Debug, Default)]
        pub struct SyncProgress;

        pub mod manager {
            use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

            #[derive(Clone, Debug, Default)]
            pub struct CodebaseIndexManager;

            impl CodebaseIndexManager {
                pub fn new<A, B, C, D, E, F>(
                    _indices_to_restore: A,
                    _max_indices_allowed: B,
                    _max_files_per_repo: C,
                    _embedding_generation_batch_size: D,
                    _server_api: E,
                    _ctx: F,
                ) -> Self {
                    Self
                }

                pub fn new_for_test<T>(_server_api: T, _ctx: &mut ModelContext<Self>) -> Self {
                    Self
                }

                pub fn can_create_new_indices(&self) -> bool {
                    false
                }

                pub fn get_codebase_paths(&self) -> std::iter::Empty<&'static std::path::PathBuf> {
                    std::iter::empty()
                }

                pub fn get_codebase_index_status_for_path<T, U>(
                    &self,
                    _path: T,
                    _ctx: &U,
                ) -> Option<()> {
                    None
                }

                pub fn index_directory<T, U>(&mut self, _path: T, _ctx: U) {}

                pub fn handle_session_bootstrapped<T>(&mut self, _path: T) {}

                pub fn build_and_sync_codebase_index<T, U>(&mut self, _source: T, _ctx: U) {}

                pub fn write_snapshot<T, U>(&mut self, _path: T, _ctx: U) {}

                pub fn handle_active_session_changed<T>(&mut self, _session: T) {}

                pub fn update<T>(&mut self, _ctx: &mut ModelContext<Self>, _f: T)
                where
                    T: FnOnce(&mut Self, &mut ModelContext<Self>),
                {
                }
            }

            impl Entity for CodebaseIndexManager {
                type Event = CodebaseIndexManagerEvent;
            }

            impl SingletonEntity for CodebaseIndexManager {}

            #[derive(Clone, Debug)]
            pub enum CodebaseIndexManagerEvent {
                SyncStateUpdated,
                NewIndexCreated,
                RemoveExpiredIndexMetadata {},
                IndexMetadataUpdated {},
                RetrievalRequestCompleted {},
                RetrievalRequestFailed {},
            }

            #[derive(Clone, Debug, Default)]
            pub struct BuildSource;

            impl BuildSource {
                pub fn FromPath<T>(_path: T) -> Self {
                    Self
                }
            }

            #[derive(Clone, Debug, Default)]
            pub struct CodebaseIndexStatus;

            #[derive(Clone, Debug, Default)]
            pub struct CodebaseIndexFinishedStatus;

            #[derive(Clone, Debug, Default)]
            pub struct CodebaseIndexingError;
        }
    }
}

pub mod project_context {
    pub mod model {
        use warpui::{Entity, SingletonEntity};

        #[derive(Clone, Debug, Default)]
        pub struct ProjectRulePath {
            pub path: std::path::PathBuf,
            pub project_root: std::path::PathBuf,
        }

        #[derive(Clone, Debug, Default)]
        pub struct ProjectContextModel;

        impl ProjectContextModel {
            pub fn new_from_persisted<T, U>(_rules: T, _ctx: U) -> Self {
                Self
            }

            pub fn rules_for_workspace<T>(&self, _workspace: T) -> Vec<String> {
                Vec::new()
            }

            pub fn find_applicable_rules<T>(&self, _path: T) -> Option<Vec<String>> {
                None
            }
        }

        impl Entity for ProjectContextModel {
            type Event = ProjectContextModelEvent;
        }

        impl SingletonEntity for ProjectContextModel {}

        #[derive(Clone, Debug)]
        pub enum ProjectContextModelEvent {
            KnownRulesChanged(()),
        }
    }
}

pub mod active_agent_views_model {
    use warpui::{Entity, SingletonEntity};

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum ConversationOrTaskId {
        Conversation(crate::ai::agent::conversation::AIConversationId),
        ConversationId(crate::ai::agent::conversation::AIConversationId),
        Task(crate::ai::ambient_agents::AmbientAgentTaskId),
        TaskId(crate::ai::ambient_agents::AmbientAgentTaskId),
    }

    #[derive(Clone, Debug, Default)]
    pub struct ActiveAgentViewsModel;

    impl ActiveAgentViewsModel {
        pub fn get_all_active_conversation_ids(
            &self,
            _ctx: &warpui::AppContext,
        ) -> std::collections::HashSet<ConversationOrTaskId> {
            std::collections::HashSet::new()
        }

        pub fn get_all_open_conversation_ids(
            &self,
            _ctx: &warpui::AppContext,
        ) -> std::collections::HashSet<ConversationOrTaskId> {
            std::collections::HashSet::new()
        }

        pub fn register_ambient_session<T, U>(
            &mut self,
            _terminal_id: T,
            _task_id: U,
            _ctx: &mut warpui::ModelContext<Self>,
        ) {
        }

        pub fn unregister_ambient_session<T, U: ?Sized>(&mut self, _terminal_id: T, _ctx: &mut U) {}

        pub fn remove_focused_state_for_window<T, U>(&mut self, _window_id: T, _ctx: U) {}

        pub fn get_focused_conversation<T>(&self, _window_id: T) -> Option<ConversationOrTaskId> {
            None
        }

        pub fn get_terminal_view_id_for_ambient_task<T>(
            &self,
            _task_id: T,
        ) -> Option<warpui::EntityId> {
            None
        }

        pub fn get_terminal_view_id_for_conversation<T, U: ?Sized>(
            &self,
            _conversation_id: T,
            _ctx: &U,
        ) -> Option<warpui::EntityId> {
            None
        }

        pub fn get_last_focused_terminal_id(&self) -> Option<warpui::EntityId> {
            None
        }

        pub fn get_active_session_for_conversation<T, U>(&self, _id: T, _ctx: U) -> Option<()> {
            None
        }

        pub fn maybe_get_focused_new_conversation<T>(&self, _window_id: T) -> Option<()> {
            None
        }

        pub fn get_last_opened_time<T>(&self, _id: T) -> Option<chrono::DateTime<chrono::Utc>> {
            None
        }

        pub fn get_controller_for_conversation<T, U: ?Sized>(
            &self,
            _id: T,
            _ctx: &U,
        ) -> Option<warpui::ModelHandle<crate::ai::blocklist::agent_view::AgentViewController>>
        {
            None
        }

        pub fn is_conversation_open<T, U: ?Sized>(&self, _id: T, _ctx: &U) -> bool {
            false
        }

        pub fn handle_pane_focus_change<T, U, V, W>(
            &mut self,
            _pane_group_id: T,
            _terminal_view_id: U,
            _ambient_agent_task_id: V,
            _ctx: W,
        ) {
        }

        pub fn register_agent_view_controller<T, U>(
            &mut self,
            _controller: T,
            _active_session: U,
            _terminal_view_id: warpui::EntityId,
            _ctx: &mut warpui::ModelContext<Self>,
        ) {
        }

        pub fn unregister_agent_view_controller<T>(
            &mut self,
            _terminal_view_id: warpui::EntityId,
            _ctx: &mut T,
        ) {
        }
    }

    impl Entity for ActiveAgentViewsModel {
        type Event = ();
    }

    impl SingletonEntity for ActiveAgentViewsModel {}
}

pub mod agent_conversations_model {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum AgentConversationsModelEvent {
        TasksUpdated,
        ConversationsLoaded,
        NewTasksReceived,
        ConversationUpdated,
        ConversationArtifactsUpdated,
        TaskManuallyOpened,
    }

    #[derive(Clone, Debug, Default)]
    pub struct AgentManagementFilters {
        pub owners: OwnerFilter,
        pub status: StatusFilter,
        pub source: SourceFilter,
        pub created_on: CreatedOnFilter,
        pub creator: CreatorFilter,
        pub artifact: ArtifactFilter,
        pub environment: (),
        pub harness: (),
    }

    #[derive(Clone, Debug, Default)]
    pub struct ArtifactFilter;

    impl ArtifactFilter {
        pub const All: Self = Self;
    }

    #[derive(Clone, Debug, Default)]
    pub struct CreatedOnFilter;

    impl CreatedOnFilter {
        pub const All: Self = Self;
    }

    #[derive(Clone, Debug, Default)]
    pub struct CreatorFilter;

    impl CreatorFilter {
        pub const All: Self = Self;
    }

    #[derive(Clone, Debug, Default)]
    pub struct OwnerFilter;

    impl OwnerFilter {
        pub const PersonalOnly: Self = Self;
    }

    #[derive(Clone, Debug, Default)]
    pub struct SessionStatus;

    impl SessionStatus {
        pub const Available: Self = Self;
    }

    #[derive(Clone, Debug, Default)]
    pub struct SourceFilter;

    impl SourceFilter {
        pub const All: Self = Self;
    }

    #[derive(Clone, Debug, Default)]
    pub struct StatusFilter;

    impl StatusFilter {
        pub const All: Self = Self;
    }

    #[derive(Clone, Debug)]
    pub enum ConversationOrTask<'a> {
        Conversation(&'a crate::ai::agent::conversation::AIConversation),
        Task(&'a crate::ai::ambient_agents::AmbientAgentTask),
    }

    impl ConversationOrTask<'_> {
        pub fn title(&self) -> String {
            String::new()
        }

        pub fn is_ambient_agent_conversation(&self) -> bool {
            false
        }

        pub fn status(&self) -> crate::ai::agent::conversation::ConversationStatus {
            crate::ai::agent::conversation::ConversationStatus::Unknown
        }

        pub fn last_updated(&self) -> chrono::DateTime<chrono::Utc> {
            chrono::Utc::now()
        }

        pub fn navigation_data(
            &self,
        ) -> crate::ai::conversation_navigation::ConversationNavigationData {
            crate::ai::conversation_navigation::ConversationNavigationData::default()
        }

        pub fn get_open_action(
            &self,
            _restore_layout: Option<crate::workspace::RestoreConversationLayout>,
            _app: &AppContext,
        ) -> Option<crate::workspace::WorkspaceAction> {
            None
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct AgentConversationsModel;

    impl AgentConversationsModel {
        pub fn new<T>(_ctx: T) -> Self {
            Self
        }

        pub fn reset(&mut self) {}

        pub fn get_or_async_fetch_task_data<T>(
            &mut self,
            _task_id: &T,
            _ctx: &mut ModelContext<Self>,
        ) -> Option<crate::ai::ambient_agents::AmbientAgentTask> {
            None
        }

        pub fn get_task_data<T>(
            &self,
            _task_id: &T,
        ) -> Option<crate::ai::ambient_agents::AmbientAgentTask> {
            None
        }

        pub fn mark_task_as_manually_opened<T>(
            &mut self,
            _task_id: T,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn tasks_iter(
            &self,
        ) -> std::vec::IntoIter<crate::ai::ambient_agents::AmbientAgentTask> {
            Vec::new().into_iter()
        }

        pub fn register_view_open<T, U, V>(&mut self, _window_id: T, _view_id: U, _ctx: V) {}

        pub fn register_view_closed<T, U, V>(&mut self, _window_id: T, _view_id: U, _ctx: V) {}

        pub fn get_tasks_and_conversations<T>(
            &self,
            _filters: T,
        ) -> std::vec::IntoIter<ConversationOrTask<'static>> {
            Vec::new().into_iter()
        }

        pub fn get_task<T>(&self, _id: T) -> Option<crate::ai::ambient_agents::AmbientAgentTask> {
            None
        }

        pub fn get_conversation<T>(&self, _id: T) -> Option<ConversationOrTask<'static>> {
            None
        }

        pub fn is_task_manually_opened<T>(&self, _id: T) -> bool {
            false
        }
    }

    impl Entity for AgentConversationsModel {
        type Event = AgentConversationsModelEvent;
    }

    impl SingletonEntity for AgentConversationsModel {}
}

pub mod restored_conversations {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct RestoredAgentConversations;

    impl RestoredAgentConversations {
        pub fn get_conversation<T>(&self, _id: T) -> Option<RestoredConversation> {
            None
        }

        pub fn take_conversation<T>(
            &mut self,
            _id: T,
        ) -> Option<crate::ai::agent::conversation::AIConversation> {
            None
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct RestoredConversation;

    impl RestoredConversation {
        pub fn all_tasks(&self) -> std::vec::IntoIter<()> {
            Vec::new().into_iter()
        }

        pub fn is_entirely_passive(&self) -> bool {
            true
        }
    }

    impl Entity for RestoredAgentConversations {
        type Event = ();
    }

    impl SingletonEntity for RestoredAgentConversations {}
}

pub mod conversation_navigation {
    use chrono::{DateTime, Local};
    use warpui::{EntityId, WindowId};

    use crate::{
        ai::agent::api::ServerConversationToken, ai::agent::conversation::AIConversationId,
        workspace::PaneViewLocator,
    };

    #[derive(Clone, Debug)]
    pub struct ConversationNavigationData {
        pub id: AIConversationId,
        pub metadata: ConversationMetadata,
        pub permissions: ConversationPermissions,
        pub title: String,
        pub initial_query: Option<String>,
        pub last_updated: DateTime<Local>,
        pub terminal_view_id: Option<EntityId>,
        pub window_id: Option<WindowId>,
        pub pane_view_locator: Option<PaneViewLocator>,
        pub initial_working_directory: Option<String>,
        pub latest_working_directory: Option<String>,
        pub is_selected: bool,
        pub is_closed: bool,
        pub server_conversation_token: Option<ServerConversationToken>,
        pub is_in_active_pane: bool,
    }

    impl Default for ConversationNavigationData {
        fn default() -> Self {
            Self {
                id: AIConversationId::default(),
                metadata: ConversationMetadata::default(),
                permissions: ConversationPermissions::default(),
                title: String::new(),
                initial_query: None,
                last_updated: Local::now(),
                terminal_view_id: None,
                window_id: None,
                pane_view_locator: None,
                initial_working_directory: None,
                latest_working_directory: None,
                is_selected: false,
                is_closed: false,
                server_conversation_token: None,
                is_in_active_pane: false,
            }
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct ConversationMetadata {
        pub creator_uid: Option<String>,
    }

    #[derive(Clone, Debug)]
    pub struct ConversationPermissions {
        pub space: crate::cloud_object::Owner,
    }

    impl Default for ConversationPermissions {
        fn default() -> Self {
            Self {
                space: crate::cloud_object::Owner::User {
                    user_uid: crate::auth::UserUid::new(""),
                },
            }
        }
    }

    impl PartialEq for ConversationNavigationData {
        fn eq(&self, other: &Self) -> bool {
            self.id == other.id
        }
    }

    impl Eq for ConversationNavigationData {}

    impl PartialOrd for ConversationNavigationData {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for ConversationNavigationData {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.id.cmp(&other.id)
        }
    }

    impl ConversationNavigationData {
        pub fn id(&self) -> AIConversationId {
            self.id
        }

        pub fn title(&self) -> &str {
            &self.title
        }

        pub fn last_updated(&self) -> DateTime<Local> {
            self.last_updated
        }

        pub fn pane_view_locator(&self) -> Option<PaneViewLocator> {
            self.pane_view_locator.clone()
        }

        pub fn window_id(&self) -> Option<WindowId> {
            self.window_id
        }

        pub fn initial_working_directory(&self) -> Option<String> {
            self.initial_working_directory.clone()
        }

        pub fn server_conversation_token(&self) -> Option<&ServerConversationToken> {
            self.server_conversation_token.as_ref()
        }

        pub fn is_historical(&self) -> bool {
            self.is_closed
        }

        pub fn all_conversations(_app: &warpui::AppContext) -> Vec<Self> {
            Vec::new()
        }

        pub fn historical_conversations(_app: &warpui::AppContext) -> Vec<Self> {
            Vec::new()
        }
    }
}

pub mod conversation_status_ui {
    pub const STATUS_ELEMENT_PADDING: f32 = 0.;

    pub fn render_status_element<T, U, V>(
        _status: T,
        _icon_size: U,
        _appearance: V,
    ) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }
}

pub mod paths {
    pub fn host_native_absolute_path<T>(path: T) -> T {
        path
    }
}

pub mod agent_sdk {
    use std::{collections::HashMap, ffi::OsString, path::Path};

    use warp_cli::agent::Harness;
    use warp_managed_secrets::ManagedSecretValue;

    use crate::ai::ambient_agents::AmbientAgentTaskId;

    #[derive(Debug)]
    pub struct AgentDriverError;

    impl std::fmt::Display for AgentDriverError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("agent drivers were removed in Warp Lite")
        }
    }

    impl std::error::Error for AgentDriverError {}

    pub mod driver {
        pub use super::AgentDriverError;

        pub const WARP_DRIVE_SYNC_TIMEOUT: std::time::Duration = std::time::Duration::from_secs(0);

        pub mod environment {
            pub fn prepare_environment<A, B, C, D, E>(
                _environment: A,
                _home_dir: B,
                _is_sandbox: C,
                _harness: D,
                _ctx: E,
            ) -> std::future::Ready<anyhow::Result<()>> {
                std::future::ready(Ok(()))
            }
        }

        pub mod terminal {
            use warpui::{AppContext, Entity, ModelHandle, ViewHandle};

            use crate::terminal::TerminalView;

            #[derive(Clone, Debug, Default)]
            pub struct TerminalDriver;

            impl TerminalDriver {
                pub fn create_from_existing_view(
                    _terminal_view: ViewHandle<TerminalView>,
                    ctx: &mut AppContext,
                ) -> ModelHandle<Self> {
                    ctx.add_model(|_| Self)
                }

                pub fn wait_for_session_bootstrapped(
                    &mut self,
                ) -> impl std::future::Future<Output = anyhow::Result<()>> {
                    async { Ok(()) }
                }
            }

            impl Entity for TerminalDriver {
                type Event = ();
            }
        }
    }

    pub mod retry {
        pub async fn with_bounded_retry<L, F, Fut, T, E>(_label: L, _f: F) -> Result<T, E>
        where
            F: FnMut() -> Fut,
            Fut: std::future::Future<Output = Result<T, E>>,
            E: From<anyhow::Error>,
        {
            Err(anyhow::anyhow!("agent retries were removed in Warp Lite").into())
        }
    }

    pub trait ThirdPartyHarness {
        fn validate(&self) -> Result<(), AgentDriverError> {
            Err(AgentDriverError)
        }

        fn prepare_environment_config(
            &self,
            _working_dir: &Path,
            _token: Option<&str>,
            _managed_secrets: &HashMap<String, ManagedSecretValue>,
        ) -> Result<(), AgentDriverError> {
            Ok(())
        }

        fn cli_agent(&self) -> crate::terminal::cli_agent::CLIAgent {
            crate::terminal::cli_agent::CLIAgent::Claude
        }
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub struct ClaudeHarness;

    impl ThirdPartyHarness for ClaudeHarness {}

    pub fn validate_cli_installed(
        _name: &str,
        _docs_url: Option<&str>,
    ) -> Result<(), AgentDriverError> {
        Err(AgentDriverError)
    }

    pub fn task_env_vars(
        _task_id: Option<&AmbientAgentTaskId>,
        _parent_run_id: Option<&str>,
        _harness: Harness,
    ) -> HashMap<OsString, OsString> {
        HashMap::new()
    }
}

pub mod agent_management {
    use warpui::{Entity, SingletonEntity, TypedActionView, View, ViewContext};

    #[derive(Clone, Debug)]
    pub enum AgentManagementEvent {
        ConversationNeedsAttention {
            window_id: warpui::WindowId,
            tab_index: usize,
            terminal_view_id: warpui::EntityId,
            conversation_id: crate::ai::agent::conversation::AIConversationId,
        },
        NotificationAdded {},
        NotificationUpdated,
        AllNotificationsMarkedRead,
    }

    #[derive(Clone, Debug, Default)]
    pub struct AgentNotificationsModel;

    impl AgentNotificationsModel {
        pub fn notifications(&self) -> NotificationItems {
            NotificationItems
        }

        pub fn mark_items_from_terminal_view_read<T, U>(&mut self, _terminal_view_id: T, _ctx: U) {}

        pub fn mark_item_read<T, U>(&mut self, _id: T, _ctx: U) {}
    }

    impl Entity for AgentNotificationsModel {
        type Event = AgentManagementEvent;
    }

    impl SingletonEntity for AgentNotificationsModel {}

    #[derive(Clone, Copy, Debug)]
    pub struct NotificationItem {
        pub id: uuid::Uuid,
        pub terminal_view_id: warpui::EntityId,
    }

    #[derive(Clone, Debug, Default)]
    pub struct NotificationItems;

    impl NotificationItems {
        pub fn filtered_count<T>(&self, _filter: T) -> usize {
            0
        }

        pub fn items_filtered<T>(&self, _filter: T) -> std::vec::IntoIter<NotificationItem> {
            Vec::new().into_iter()
        }

        pub fn has_unread_for_terminal_view<T>(&self, _terminal_view_id: T) -> bool {
            false
        }
    }

    pub mod notifications {
        use super::*;

        #[derive(Clone, Copy, Debug, Default)]
        pub enum NotificationSourceAgent {
            #[default]
            Oz,
            CLI(crate::server::telemetry::CLIAgentType),
        }

        #[derive(Clone, Debug, Default)]
        pub struct NotificationFilter;

        impl NotificationFilter {
            pub const Unread: Self = Self;
        }

        pub mod toast_stack {
            use warpui::{Entity, TypedActionView, View, ViewContext};

            #[derive(Default)]
            pub struct AgentNotificationToastStack;

            impl AgentNotificationToastStack {
                pub fn new<T>(_ctx: T) -> Self {
                    Self
                }

                pub fn set_mailbox_open<T, U>(&mut self, _is_open: T, _ctx: U) {}
            }

            impl Entity for AgentNotificationToastStack {
                type Event = ();
            }

            impl View for AgentNotificationToastStack {
                fn ui_name() -> &'static str {
                    "AgentNotificationToastStack"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl TypedActionView for AgentNotificationToastStack {
                type Action = ();

                fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
            }
        }

        pub mod view {
            use warpui::{Entity, TypedActionView, View, ViewContext};

            #[derive(Clone, Debug)]
            pub enum AgentNotificationsViewEvent {}

            #[derive(Default)]
            pub struct AgentNotificationsView;

            impl Entity for AgentNotificationsView {
                type Event = AgentNotificationsViewEvent;
            }

            impl View for AgentNotificationsView {
                fn ui_name() -> &'static str {
                    "AgentNotificationsView"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            #[derive(Clone, Debug)]
            pub enum NotificationMailboxViewEvent {
                NavigateToTerminal { terminal_view_id: warpui::EntityId },
                Dismissed,
            }

            #[derive(Default)]
            pub struct NotificationMailboxView;

            impl NotificationMailboxView {
                pub fn new<T>(_ctx: T) -> Self {
                    Self
                }

                pub fn reset_for_open<T, U>(&mut self, _select_first: T, _ctx: U) {}
            }

            impl Entity for NotificationMailboxView {
                type Event = NotificationMailboxViewEvent;
            }

            impl View for NotificationMailboxView {
                fn ui_name() -> &'static str {
                    "NotificationMailboxView"
                }

                fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                    Box::new(warpui::elements::Empty::new())
                }
            }

            impl TypedActionView for NotificationMailboxView {
                type Action = ();

                fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
            }
        }
    }

    pub mod telemetry {
        #[derive(Clone, Debug)]
        pub enum AgentManagementTelemetryEvent {
            TombstoneArtifactClicked { artifact_type: ArtifactType },
            TombstoneContinueLocally,
            ConversationOpened { opened_from: OpenedFrom },
            CloudRunOpened { opened_from: OpenedFrom },
            ViewToggled { is_open: bool },
        }

        warp_core::register_telemetry_event!(AgentManagementTelemetryEvent);

        impl warp_core::telemetry::TelemetryEvent for AgentManagementTelemetryEvent {
            fn name(&self) -> &'static str {
                "Agent Management Removed"
            }

            fn payload(&self) -> Option<serde_json::Value> {
                None
            }

            fn description(&self) -> &'static str {
                "Agent management telemetry removed in Warp Lite"
            }

            fn enablement_state(&self) -> warp_core::telemetry::EnablementState {
                warp_core::telemetry::EnablementState::Always
            }

            fn contains_ugc(&self) -> bool {
                false
            }

            fn event_descs(
            ) -> impl Iterator<Item = Box<dyn warp_core::telemetry::TelemetryEventDesc>>
            {
                std::iter::empty()
            }
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub enum ArtifactType {
            Plan,
            Branch,
            PullRequest,
            #[default]
            File,
        }

        #[derive(Clone, Copy, Debug, Default)]
        pub enum OpenedFrom {
            ConversationList,
            #[default]
            Unknown,
        }
    }

    pub mod view {
        use warpui::{Entity, TypedActionView, View, ViewContext};

        #[derive(Clone, Debug)]
        pub enum AgentManagementViewEvent {
            OpenNewTabAndRunWorkflow(std::sync::Arc<crate::workflows::WorkflowType>),
            OpenPlanNotebook {
                notebook_uid: crate::notebooks::NotebookId,
            },
        }

        #[derive(Default)]
        pub struct AgentManagementView;

        impl AgentManagementView {
            pub fn new<T, U>(_initial_filter: T, _ctx: U) -> Self {
                Self
            }

            pub fn apply_environment_filter_from_link<T, U>(&mut self, _env: T, _ctx: U) {}

            pub fn show_setup_guide_from_link<T>(&mut self, _ctx: T) {}
        }

        impl Entity for AgentManagementView {
            type Event = AgentManagementViewEvent;
        }

        impl View for AgentManagementView {
            fn ui_name() -> &'static str {
                "AgentManagementView"
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl TypedActionView for AgentManagementView {
            type Action = ();

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }
    }
}

pub mod api_keys {
    use warpui::{Entity, ModelContext, SingletonEntity};

    #[derive(Clone, Debug, Default)]
    pub enum AwsCredentialsState {
        Loaded {},
        #[default]
        Missing,
    }

    #[derive(Clone, Debug, Default)]
    pub struct ApiKeys;

    #[derive(Clone, Debug)]
    pub enum ApiKeyManagerEvent {
        KeysUpdated,
    }

    #[derive(Clone, Debug, Default)]
    pub struct ApiKeyManager;

    impl ApiKeyManager {
        pub fn new<T>(_ctx: T) -> Self {
            Self
        }

        pub fn subscribe_to_settings_changes<T: ?Sized>(&mut self, _ctx: &mut T) {}

        pub fn refresh_aws_credentials_if_needed(&mut self, _ctx: &mut ModelContext<Self>) {}

        pub fn register_model_event_dispatcher<T, U>(&mut self, _dispatcher: T, _ctx: U) {}

        pub fn aws_credentials_state(&self) -> AwsCredentialsState {
            AwsCredentialsState::Missing
        }
    }

    impl Entity for ApiKeyManager {
        type Event = ApiKeyManagerEvent;
    }

    impl SingletonEntity for ApiKeyManager {}
}

pub mod aws_credentials {
    pub trait AwsCredentialRefresher {}

    pub fn refresh_aws_credentials<T, U>(_manager: T, _ctx: U) -> anyhow::Result<()> {
        Ok(())
    }
}

pub mod outline {
    use std::{
        collections::HashMap,
        path::{Path, PathBuf},
    };

    use crate::ai::index::Symbol;
    use warpui::{Entity, SingletonEntity};

    #[derive(Clone, Debug, Default)]
    pub struct RepoOutlines;

    #[derive(Clone, Debug)]
    pub enum OutlineStatus {
        Pending,
        Complete(Outline),
    }

    impl Default for OutlineStatus {
        fn default() -> Self {
            Self::Pending
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct Outline;

    impl Outline {
        pub fn to_symbols_by_file(&self, _filter: Option<()>) -> HashMap<PathBuf, FileOutline> {
            HashMap::new()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct FileOutline;

    impl FileOutline {
        pub fn symbols(&self) -> Vec<Vec<Symbol>> {
            Vec::new()
        }
    }

    #[derive(Clone, Debug)]
    pub enum RepoOutlinesEvent {
        Changed,
        OutlinesUpdated(PathBuf),
    }

    impl RepoOutlines {
        pub fn new<T>(_ctx: T) -> Self {
            Self
        }

        pub fn get_outline(&self, _repo_path: &Path) -> Option<(OutlineStatus, ())> {
            None
        }
    }

    impl Entity for RepoOutlines {
        type Event = RepoOutlinesEvent;
    }

    impl SingletonEntity for RepoOutlines {}
}

pub mod facts_manager {
    pub use crate::ai::facts::AIFactManager;
}

pub mod conversation_utils {
    pub fn remove_conversation<T, U, V, W: ?Sized>(
        _conversation_id: T,
        _terminal_view_id: U,
        _delete: V,
        _ctx: &mut W,
    ) {
    }

    pub fn delete_conversation<T, U, V: ?Sized>(
        _conversation_id: T,
        _terminal_view_id: U,
        _ctx: &mut V,
    ) {
    }
}

pub mod loading {
    pub fn shimmering_warp_loading_text<T, U>(
        _message: &str,
        _font_size: f32,
        _shimmer_handle: T,
        _app: U,
    ) -> Box<dyn warpui::Element> {
        Box::new(warpui::elements::Empty::new())
    }
}

pub mod predict {
    pub mod generate_ai_input_suggestions {
        #[derive(Clone, Debug, Default, serde::Serialize)]
        pub struct GenerateAIInputSuggestionsRequest;

        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        pub struct GenerateAIInputSuggestionsResponseV2 {
            pub most_likely_action: String,
        }

        #[derive(Clone, Debug)]
        pub struct HistoryContext {
            pub previous_commands: Vec<crate::terminal::HistoryEntry>,
            pub next_command: crate::terminal::HistoryEntry,
        }
    }

    pub mod generate_am_query_suggestions {
        #[derive(Clone, Debug, Default, serde::Serialize)]
        pub struct GenerateAMQuerySuggestionsRequest;

        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        pub struct GenerateAMQuerySuggestionsResponse;
    }

    pub mod predict_am_queries {
        #[derive(Clone, Debug, Default, serde::Serialize)]
        pub struct PredictAMQueriesRequest {
            pub context_messages: Vec<String>,
            pub partial_query: String,
            pub system_context: Option<String>,
        }

        #[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
        pub struct PredictAMQueriesResponse {
            pub suggestion: String,
        }
    }

    pub mod next_command_model {
        use warpui::{Entity, SingletonEntity};

        use super::generate_ai_input_suggestions::{
            GenerateAIInputSuggestionsRequest, GenerateAIInputSuggestionsResponseV2, HistoryContext,
        };

        #[derive(Clone, Debug, Default)]
        pub struct NextCommandModel;

        #[derive(Clone, Debug)]
        pub enum NextCommandModelEvent {
            NextCommandSuggestionReady,
        }

        #[derive(Clone, Debug)]
        pub struct ZeroStateSuggestionInfo {
            pub request: Box<GenerateAIInputSuggestionsRequest>,
            pub response: GenerateAIInputSuggestionsResponseV2,
            pub is_from_ai: bool,
            pub history_based_autosuggestion_state: HistoryBasedAutosuggestionState,
            pub request_duration_ms: i64,
        }

        impl Default for ZeroStateSuggestionInfo {
            fn default() -> Self {
                Self {
                    request: Box::default(),
                    response: GenerateAIInputSuggestionsResponseV2::default(),
                    is_from_ai: false,
                    history_based_autosuggestion_state: HistoryBasedAutosuggestionState::default(),
                    request_duration_ms: 0,
                }
            }
        }

        pub async fn is_command_valid<T, U, V>(
            _command: T,
            _ctx: Option<U>,
            _session_env_vars: Option<V>,
        ) -> bool {
            false
        }

        pub fn is_next_command_enabled<T: ?Sized>(_ctx: &T) -> bool {
            false
        }

        impl NextCommandModel {
            pub fn new<T, U, V>(_sessions: T, _terminal_model: U, _server_api: V) -> Self {
                Self
            }

            pub fn get_state(&self) -> &NextCommandSuggestionState {
                static STATE: NextCommandSuggestionState = NextCommandSuggestionState::None;
                &STATE
            }

            pub fn clear_state(&mut self) {}

            pub fn cycle_next_command_suggestion<T>(&mut self, _ctx: T) {}

            pub fn generate_next_command_suggestion<Y>(
                &mut self,
                _block_completed: crate::terminal::event::UserBlockCompleted,
                _context: crate::ai_assistant::execution_context::WarpAiExecutionContext,
                _completer_data: crate::terminal::input::CompleterData,
                _block_context: Option<Box<crate::ai::block_context::BlockContext>>,
                _previous_result: Option<crate::terminal::input::IntelligentAutosuggestionResult>,
                _ctx: Y,
            ) {
            }

            pub fn generate_next_command_suggestion_with_prefix<Z>(
                &mut self,
                _prefix: Option<String>,
                _block_completed: crate::terminal::event::UserBlockCompleted,
                _context: crate::ai_assistant::execution_context::WarpAiExecutionContext,
                _completer_data: crate::terminal::input::CompleterData,
                _block_context: Option<Box<crate::ai::block_context::BlockContext>>,
                _previous_result: Option<crate::terminal::input::IntelligentAutosuggestionResult>,
                _ctx: Z,
            ) {
            }

            pub fn abort_inflight_request(&mut self) {}

            pub fn get_zero_state_suggestion_info(&self) -> Option<&ZeroStateSuggestionInfo> {
                None
            }

            pub fn get_reverse_chronological_potential_autosuggestions<T, U, V: ?Sized>(
                _prefix: T,
                _completer_data: U,
                _ctx: &mut V,
            ) -> Option<Vec<crate::terminal::HistoryEntry>> {
                Some(Vec::new())
            }

            pub fn get_similar_history_context<T, U, V>(
                _conn: T,
                _completed_block: U,
                _num_additional_preceding_commands: V,
            ) -> Vec<HistoryContext> {
                Vec::new()
            }
        }

        #[derive(Clone, Debug, Default, serde::Serialize)]
        pub struct HistoryBasedAutosuggestionState {
            pub history_command_prediction: String,
            pub history_command_prediction_likelihood: f64,
            pub total_history_count: usize,
        }

        #[derive(Clone, Debug, Default)]
        pub enum NextCommandSuggestionState {
            #[default]
            None,
            Cycling,
            Ready {
                request: Box<GenerateAIInputSuggestionsRequest>,
                response: GenerateAIInputSuggestionsResponseV2,
                request_duration_ms: i64,
                is_from_ai: bool,
                is_from_cycle: bool,
                history_based_autosuggestion_state: HistoryBasedAutosuggestionState,
            },
        }

        impl NextCommandSuggestionState {
            pub fn command_suggestion(&self) -> Option<&str> {
                match self {
                    Self::Ready { response, .. } => Some(response.most_likely_action.as_str()),
                    _ => None,
                }
            }
        }

        impl NextCommandSuggestionState {
            pub fn is_cycling(&self) -> bool {
                matches!(self, Self::Cycling)
            }
        }

        impl Entity for NextCommandModel {
            type Event = NextCommandModelEvent;
        }

        impl SingletonEntity for NextCommandModel {}
    }

    pub mod prompt_suggestions {
        pub const ACCEPT_PROMPT_SUGGESTION_KEYBINDING: &str = "ctrl-enter";

        pub fn has_pending_code_or_unit_test_prompt_suggestion<T, U>(_model: T, _ctx: U) -> bool {
            false
        }

        pub fn is_accept_prompt_suggestion_bound_to_ctrl_enter<T: ?Sized>(_ctx: &T) -> bool {
            false
        }

        pub fn is_accept_prompt_suggestion_bound_to_cmd_enter<T: ?Sized>(_ctx: &T) -> bool {
            false
        }
    }
}

pub mod conversation_details_panel {
    use crate::ai::agent::conversation::AIConversation;
    use crate::ai::ambient_agents::{AmbientAgentTask, AmbientAgentTaskId};
    use crate::notebooks::NotebookId;
    use warpui::{AppContext, Entity, View, ViewContext};

    #[derive(Clone, Debug, Default)]
    pub struct ConversationDetailsData;

    impl ConversationDetailsData {
        pub fn from_task(
            _task: &AmbientAgentTask,
            _open_action: Option<crate::workspace::WorkspaceAction>,
            _copy_link_url: Option<String>,
            _ctx: &AppContext,
        ) -> Self {
            Self
        }

        pub fn from_task_id(_task_id: AmbientAgentTaskId) -> Self {
            Self
        }

        pub fn from_conversation(_conversation: &AIConversation, _ctx: &AppContext) -> Self {
            Self
        }
    }

    pub struct ConversationDetailsPanel;

    impl ConversationDetailsPanel {
        pub fn new<T, U>(_show_open_button: T, _width: U, _ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn set_conversation_details(
            &mut self,
            _data: ConversationDetailsData,
            _ctx: &mut ViewContext<Self>,
        ) {
        }
    }

    impl Entity for ConversationDetailsPanel {
        type Event = ConversationDetailsPanelEvent;
    }

    impl View for ConversationDetailsPanel {
        fn ui_name() -> &'static str {
            "ConversationDetailsPanel"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for ConversationDetailsPanel {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    #[derive(Clone, Debug)]
    pub enum ConversationDetailsPanelEvent {
        Close,
        OpenPlanNotebook { notebook_uid: NotebookId },
    }
}

pub mod artifacts {
    use serde::{Deserialize, Serialize};

    pub fn open_screenshot_lightbox<T>(_artifact_uids: T, _ctx: &mut warpui::AppContext) {}

    pub fn download_file_artifact<T>(_artifact_uid: T, _ctx: &mut warpui::AppContext) {}

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct Artifact;

    #[derive(Clone, Debug, Default)]
    pub struct ArtifactButtonsRow;

    #[derive(Clone, Debug)]
    pub enum ArtifactButtonsRowEvent {
        OpenPlan { notebook_uid: String },
        CopyBranch { branch: String },
        OpenPullRequest { url: String },
        ViewScreenshots { artifact_uids: Vec<String> },
        DownloadFile { artifact_uid: String },
    }

    impl ArtifactButtonsRow {
        pub fn new<T, U>(_artifact: T, _ctx: U) -> Self {
            Self
        }

        pub fn update_artifacts<T, U>(&mut self, _artifacts: T, _ctx: U) {}
    }

    impl warpui::Entity for ArtifactButtonsRow {
        type Event = ArtifactButtonsRowEvent;
    }

    impl warpui::View for ArtifactButtonsRow {
        fn ui_name() -> &'static str {
            "ArtifactButtonsRow"
        }

        fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
            Box::new(warpui::elements::Empty::new())
        }
    }

    impl warpui::TypedActionView for ArtifactButtonsRow {
        type Action = ();

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut warpui::ViewContext<Self>) {}
    }
}

pub mod generate_code_review_content {
    pub mod api {
        #[derive(Clone, Debug, Default)]
        pub struct GenerateCodeReviewContentRequest {
            pub output_type: OutputType,
            pub diff: String,
            pub branch_name: String,
            pub commit_messages: Vec<String>,
        }

        #[derive(Clone, Debug, Default)]
        pub struct GenerateCodeReviewContentResponse {
            pub content: String,
        }

        #[derive(Clone, Debug, Default)]
        pub enum OutputType {
            #[default]
            Unknown,
            CommitMessage,
            PrTitle,
            PrDescription,
        }
    }
}

pub mod diff_validation {
    use std::ops::Range;

    use serde::{Deserialize, Serialize};

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DiffDelta {
        pub replacement_line_range: Range<usize>,
        pub insertion: String,
    }

    #[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DiffType {
        Create {
            delta: DiffDelta,
        },
        Update {
            deltas: Vec<DiffDelta>,
            rename: Option<std::path::PathBuf>,
        },
        Delete {
            delta: DiffDelta,
        },
    }
}

pub mod get_relevant_files {
    pub mod controller {
        use warpui::{Entity, SingletonEntity};

        #[derive(Clone, Debug, Default)]
        pub struct GetRelevantFilesController;

        impl GetRelevantFilesController {
            pub fn new<T>(_ctx: T) -> Self {
                Self
            }
        }

        impl Entity for GetRelevantFilesController {
            type Event = ();
        }

        impl SingletonEntity for GetRelevantFilesController {}
    }

    pub mod api {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct GetRelevantFiles;

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct GetRelevantFilesResponse;
    }
}

pub mod voice {
    pub mod transcribe {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct TranscribeRequest {
            pub wav_base64: String,
            pub audio: Option<String>,
            pub provider: Provider,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub struct TranscribeResponse {
            pub text: String,
        }

        #[derive(Clone, Debug, Default, Serialize, Deserialize)]
        pub enum Provider {
            #[default]
            Default,
            Wispr,
        }
    }
}

pub mod generate_block_title {
    pub mod api {
        #[derive(Clone, Debug, Default, serde::Serialize)]
        pub struct GenerateBlockTitleRequest {
            pub command: String,
            pub output: String,
        }

        #[derive(Clone, Debug, Default, serde::Deserialize)]
        pub struct GenerateBlockTitleResponse {
            pub title: String,
        }
    }
}

macro_rules! impl_removed_backing_view {
    ($ty:path, $name:literal) => {
        impl warpui::View for $ty {
            fn ui_name() -> &'static str {
                $name
            }

            fn render(&self, _app: &warpui::AppContext) -> Box<dyn warpui::Element> {
                Box::new(warpui::elements::Empty::new())
            }
        }

        impl crate::pane_group::pane::BackingView for $ty {
            type PaneHeaderOverflowMenuAction = ();
            type CustomAction = ();
            type AssociatedData = ();

            fn handle_pane_header_overflow_menu_action(
                &mut self,
                _action: &Self::PaneHeaderOverflowMenuAction,
                _ctx: &mut warpui::ViewContext<Self>,
            ) {
            }

            fn close(&mut self, _ctx: &mut warpui::ViewContext<Self>) {}

            fn focus_contents(&mut self, _ctx: &mut warpui::ViewContext<Self>) {}

            fn render_header_content(
                &self,
                _ctx: &crate::pane_group::pane::view::HeaderRenderContext<'_>,
                _app: &warpui::AppContext,
            ) -> crate::pane_group::pane::view::HeaderContent {
                crate::pane_group::pane::view::HeaderContent::simple($name)
            }

            fn set_focus_handle(
                &mut self,
                _focus_handle: crate::pane_group::focus_state::PaneFocusHandle,
                _ctx: &mut warpui::ViewContext<Self>,
            ) {
            }
        }
    };
}

impl Entity for blocklist::inline_action::code_diff_view::CodeDiffView {
    type Event = blocklist::inline_action::code_diff_view::CodeDiffViewEvent;
}

impl_removed_backing_view!(ai_document_view::AIDocumentView, "AI Document");
impl_removed_backing_view!(facts::AIFactView, "AI Fact");
impl_removed_backing_view!(
    blocklist::inline_action::code_diff_view::CodeDiffView,
    "Code Diff"
);
impl_removed_backing_view!(
    execution_profiles::editor::ExecutionProfileEditorView,
    "Execution Profile"
);

macro_rules! impl_removed_cloud_model {
    ($model:path, $json_variant:ident, $queue_variant:ident, $name:literal) => {
        #[async_trait::async_trait]
        impl crate::cloud_object::CloudModelType for $model {
            type CloudObjectType = crate::cloud_object::GenericCloudObject<
                crate::cloud_object::model::generic_string_model::GenericStringObjectId,
                Self,
            >;
            type IdType = crate::cloud_object::model::generic_string_model::GenericStringObjectId;

            fn model_type_name(&self) -> &'static str {
                $name
            }

            fn cloud_object_type_and_id(
                &self,
                id: crate::server::ids::SyncId,
            ) -> crate::drive::CloudObjectTypeAndId {
                crate::drive::CloudObjectTypeAndId::GenericStringObject {
                    object_type: crate::cloud_object::GenericStringObjectFormat::Json(
                        crate::cloud_object::JsonObjectType::$json_variant,
                    ),
                    id,
                }
            }

            fn object_type(&self) -> crate::cloud_object::ObjectType {
                crate::cloud_object::ObjectType::GenericStringObject(
                    crate::cloud_object::GenericStringObjectFormat::Json(
                        crate::cloud_object::JsonObjectType::$json_variant,
                    ),
                )
            }

            fn renders_in_warp_drive(&self) -> bool {
                false
            }

            fn should_show_activity_toasts(&self) -> bool {
                false
            }

            fn warn_if_unsaved_at_quit(&self) -> bool {
                false
            }

            fn to_warp_drive_item(
                &self,
                _id: crate::server::ids::SyncId,
                _appearance: &crate::appearance::Appearance,
                _object: &Self::CloudObjectType,
            ) -> Option<Box<dyn crate::drive::items::WarpDriveItem>> {
                None
            }

            fn display_name(&self) -> String {
                $name.to_owned()
            }

            fn upsert_event(
                &self,
                _object: &Self::CloudObjectType,
            ) -> crate::persistence::ModelEvent {
                crate::persistence::ModelEvent::UpsertGenericStringObjects(Vec::new())
            }

            fn bulk_upsert_event(
                _objects: &[Self::CloudObjectType],
            ) -> crate::persistence::ModelEvent {
                crate::persistence::ModelEvent::UpsertGenericStringObjects(Vec::new())
            }

            fn create_object_queue_item(
                &self,
                object: &Self::CloudObjectType,
                entrypoint: crate::cloud_object::CloudObjectEventEntrypoint,
                initiated_by: crate::server::cloud_objects::update_manager::InitiatedBy,
            ) -> Option<crate::server::sync_queue::QueueItem> {
                if let crate::server::ids::SyncId::ClientId(id) = object.id {
                    Some(crate::server::sync_queue::QueueItem::CreateObject {
                        object_type: self.object_type(),
                        owner: object.permissions.owner,
                        id,
                        title: Some(std::sync::Arc::new(self.display_name())),
                        serialized_model: Some(std::sync::Arc::new(self.serialized())),
                        initial_folder_id: object.metadata.folder_id,
                        entrypoint,
                        initiated_by,
                    })
                } else {
                    None
                }
            }

            fn update_object_queue_item(
                &self,
                revision: Option<crate::cloud_object::Revision>,
                object: &Self::CloudObjectType,
            ) -> crate::server::sync_queue::QueueItem {
                crate::server::sync_queue::QueueItem::$queue_variant {
                    model: std::sync::Arc::new(self.clone()),
                    id: object.id,
                    revision,
                }
            }

            fn serialized(&self) -> crate::server::sync_queue::SerializedModel {
                crate::server::sync_queue::SerializedModel::new("{}".to_owned())
            }

            async fn send_create_request(
                _object_client: std::sync::Arc<dyn crate::server::server_api::object::ObjectClient>,
                _request: crate::cloud_object::CreateObjectRequest,
            ) -> anyhow::Result<crate::cloud_object::CreateCloudObjectResult> {
                anyhow::bail!("cloud object creation was removed in Warp Lite")
            }

            async fn send_update_request(
                &self,
                _object_client: std::sync::Arc<dyn crate::server::server_api::object::ObjectClient>,
                _server_id: crate::server::ids::ServerId,
                _revision: Option<crate::cloud_object::Revision>,
            ) -> anyhow::Result<
                crate::cloud_object::UpdateCloudObjectResult<
                    crate::cloud_object::GenericServerObject<Self::IdType, Self>,
                >,
            > {
                anyhow::bail!("cloud object updates were removed in Warp Lite")
            }

            fn should_update_after_server_conflict(&self) -> bool {
                false
            }

            fn new_from_server_update(
                &self,
                _server_cloud_object: &crate::cloud_object::ServerCloudObject,
            ) -> Option<Self> {
                None
            }

            fn supports_linking(&self) -> bool {
                false
            }
        }
    };
}

impl_removed_cloud_model!(facts::CloudAIFactModel, AIFact, UpdateAIFact, "AI Fact");
impl_removed_cloud_model!(
    mcp::CloudMCPServerModel,
    MCPServer,
    UpdateMCPServer,
    "MCP Server"
);
impl_removed_cloud_model!(
    execution_profiles::CloudAIExecutionProfileModel,
    AIExecutionProfile,
    UpdateAIExecutionProfile,
    "AI Execution Profile"
);
impl_removed_cloud_model!(
    mcp::templatable::CloudTemplatableMCPServerModel,
    TemplatableMCPServer,
    UpdateTemplatableMCPServer,
    "Templatable MCP Server"
);
impl_removed_cloud_model!(
    cloud_environments::CloudAmbientAgentEnvironmentModel,
    CloudEnvironment,
    UpdateCloudEnvironment,
    "Cloud Environment"
);
impl_removed_cloud_model!(
    ambient_agents::scheduled::CloudScheduledAmbientAgentModel,
    ScheduledAmbientAgent,
    UpdateScheduledAmbientAgent,
    "Scheduled Agent"
);
impl_removed_cloud_model!(
    cloud_agent_config::CloudAgentConfigModel,
    CloudAgentConfig,
    UpdateCloudAgentConfig,
    "Cloud Agent Config"
);
pub const CLAUDE_ORANGE: pathfinder_color::ColorU = pathfinder_color::ColorU {
    r: 0,
    g: 0,
    b: 0,
    a: 0,
};
pub const FORK_PREFIX: &str = "";
pub const NEW_AGENT_PANE_LABEL: &str = "Terminal";

pub fn ai_brand_color<T>(_theme: T) -> pathfinder_color::ColorU {
    CLAUDE_ORANGE
}
