//! Local-only AI/server-agent tombstone for Warp Lite.
//!
//! This module intentionally keeps a small compatibility surface for code that
//! is still being removed from the app, but every method is inert and performs
//! no network I/O.

use anyhow::{bail, Result};
use async_trait::async_trait;
use chrono::{DateTime, Utc};
#[cfg(test)]
use mockall::automock;
use serde::{Deserialize, Serialize};

use super::ServerApi;
use crate::ai::agent::api::ServerConversationToken;
use crate::ai::agent::conversation::ServerAIConversationMetadata;
use crate::ai::ambient_agents::AmbientAgentTaskId;
use crate::ai::generate_code_review_content::api::{
    GenerateCodeReviewContentRequest, GenerateCodeReviewContentResponse,
};
use crate::ai_assistant::{
    execution_context::WarpAiExecutionContext, AIGeneratedCommand,
    GenerateCommandsFromNaturalLanguageError,
};
use crate::drive::workflows::ai_assist::{GeneratedCommandMetadata, GeneratedCommandMetadataError};
use crate::terminal::model::block::SerializedBlock;

pub use crate::ai::ambient_agents::{
    task::{AttachmentInput, TaskAttachment},
    AgentConfigSnapshot, AgentSource, AmbientAgentTask, AmbientAgentTaskState, TaskStatusMessage,
};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum AgentTaskState {
    #[default]
    Unknown,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub enum PlatformErrorCode {
    #[default]
    Unknown,
}

#[derive(Debug, Clone)]
pub struct TaskStatusUpdate {
    pub message: String,
    pub error_code: Option<PlatformErrorCode>,
}

impl TaskStatusUpdate {
    pub fn message(message: impl Into<String>) -> Self {
        Self {
            message: message.into(),
            error_code: None,
        }
    }

    pub fn with_error_code(message: impl Into<String>, error_code: PlatformErrorCode) -> Self {
        Self {
            message: message.into(),
            error_code: Some(error_code),
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SpawnAgentRequest {
    pub prompt: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub config: Option<AgentConfigSnapshot>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub team: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skill: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub attachments: Vec<AttachmentInput>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interactive: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent_run_id: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub runtime_skills: Vec<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub referenced_attachments: Vec<String>,
}

#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct SpawnAgentResponse {
    pub task_id: AmbientAgentTaskId,
    pub run_id: String,
    #[serde(default)]
    pub at_capacity: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SendAgentMessageRequest {
    pub to: Vec<String>,
    pub subject: String,
    pub body: String,
    pub sender_run_id: String,
}

#[derive(Debug, Clone, Default)]
pub struct ListAgentMessagesRequest {
    pub unread_only: bool,
    pub since: Option<String>,
    pub limit: i32,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct SendAgentMessageResponse {
    pub message_ids: Vec<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentMessageHeader {
    pub message_id: String,
    pub sender_run_id: String,
    pub subject: String,
    pub sent_at: String,
    pub delivered_at: Option<String>,
    pub read_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct AgentRunEvent {
    pub event_type: String,
    pub run_id: String,
    pub ref_id: Option<String>,
    pub execution_id: Option<String>,
    pub occurred_at: String,
    pub sequence: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportAgentEventRequest {
    pub event_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub execution_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ref_id: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReportAgentEventResponse {
    pub sequence: i64,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ReadAgentMessageResponse {
    pub message_id: String,
    pub sender_run_id: String,
    pub subject: String,
    pub body: String,
    pub sent_at: String,
    pub delivered_at: Option<String>,
    pub read_at: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize)]
pub struct AttachmentFileInfo {
    pub filename: String,
    pub mime_type: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AttachmentUploadInfo {
    pub attachment_id: String,
    pub upload_url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct PrepareAttachmentUploadsResponse {
    pub attachments: Vec<AttachmentUploadInfo>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct AttachmentDownloadInfo {
    pub attachment_id: String,
    pub download_url: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct DownloadAttachmentsResponse {
    pub attachments: Vec<AttachmentDownloadInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct CreateFileArtifactUploadRequest {
    pub conversation_id: Option<String>,
    pub run_id: Option<String>,
    pub filepath: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub size_bytes: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct FileArtifactRecord {
    pub artifact_uid: String,
    pub filepath: String,
    pub description: Option<String>,
    pub mime_type: String,
    pub size_bytes: Option<i32>,
}

#[derive(Debug, Clone, Default)]
pub struct FileArtifactUploadHeaderInfo {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Default)]
pub struct FileArtifactUploadTargetInfo {
    pub url: String,
    pub method: String,
    pub headers: Vec<FileArtifactUploadHeaderInfo>,
}

#[derive(Debug, Clone, Default)]
pub struct CreateFileArtifactUploadResponse {
    pub artifact: FileArtifactRecord,
    pub upload_target: FileArtifactUploadTargetInfo,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(tag = "artifact_type")]
pub enum ArtifactDownloadResponse {
    #[default]
    #[serde(rename = "FILE")]
    File,
}

impl ArtifactDownloadResponse {
    pub fn artifact_uid(&self) -> &str {
        ""
    }

    pub fn artifact_type(&self) -> &'static str {
        "FILE"
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::UNIX_EPOCH
    }

    pub fn download_url(&self) -> &str {
        ""
    }

    pub fn expires_at(&self) -> DateTime<Utc> {
        DateTime::<Utc>::UNIX_EPOCH
    }

    pub fn content_type(&self) -> &str {
        ""
    }

    pub fn filepath(&self) -> Option<&str> {
        None
    }

    pub fn filename(&self) -> Option<&str> {
        None
    }

    pub fn description(&self) -> Option<&str> {
        None
    }

    pub fn size_bytes(&self) -> Option<i64> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct TaskListFilter {
    pub creator_uid: Option<String>,
    pub updated_after: Option<DateTime<Utc>>,
    pub created_after: Option<DateTime<Utc>>,
    pub created_before: Option<DateTime<Utc>>,
    pub states: Option<Vec<AmbientAgentTaskState>>,
    pub source: Option<AgentSource>,
    pub execution_location: Option<ExecutionLocation>,
    pub environment_id: Option<String>,
    pub skill_spec: Option<String>,
    pub schedule_id: Option<String>,
    pub ancestor_run_id: Option<String>,
    pub config_name: Option<String>,
    pub model_id: Option<String>,
    pub artifact_type: Option<ArtifactType>,
    pub search_query: Option<String>,
    pub sort_by: Option<RunSortBy>,
    pub sort_order: Option<RunSortOrder>,
    pub cursor: Option<String>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ExecutionLocation {
    Local,
    Remote,
}

impl ExecutionLocation {
    pub fn as_query_param(&self) -> &'static str {
        match self {
            ExecutionLocation::Local => "LOCAL",
            ExecutionLocation::Remote => "REMOTE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArtifactType {
    Plan,
    PullRequest,
    Screenshot,
    File,
}

impl ArtifactType {
    pub fn as_query_param(&self) -> &'static str {
        match self {
            ArtifactType::Plan => "PLAN",
            ArtifactType::PullRequest => "PULL_REQUEST",
            ArtifactType::Screenshot => "SCREENSHOT",
            ArtifactType::File => "FILE",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunSortBy {
    UpdatedAt,
    CreatedAt,
    Title,
    Agent,
}

impl RunSortBy {
    pub fn as_query_param(&self) -> &'static str {
        match self {
            RunSortBy::UpdatedAt => "updated_at",
            RunSortBy::CreatedAt => "created_at",
            RunSortBy::Title => "title",
            RunSortBy::Agent => "agent",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RunSortOrder {
    Asc,
    Desc,
}

impl RunSortOrder {
    pub fn as_query_param(&self) -> &'static str {
        match self {
            RunSortOrder::Asc => "asc",
            RunSortOrder::Desc => "desc",
        }
    }
}

pub(crate) fn build_list_agent_runs_url(limit: i32, _filter: &TaskListFilter) -> String {
    format!("agent/runs?limit={limit}")
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AgentListSource {
    pub owner: String,
    pub name: String,
    pub skill_path: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AgentListEnvironment {
    pub uid: String,
    pub name: String,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AgentListVariant {
    pub id: String,
    pub description: String,
    pub base_prompt: String,
    pub source: AgentListSource,
    pub environments: Vec<AgentListEnvironment>,
}

#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct AgentListItem {
    pub name: String,
    pub variants: Vec<AgentListVariant>,
}

#[derive(Clone, Debug, Default, Deserialize)]
pub struct ScheduledAgentHistory;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ConversationData;

#[cfg_attr(test, automock)]
#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
pub trait AIClient: 'static + Send + Sync {
    async fn generate_commands_from_natural_language(
        &self,
        prompt: String,
        ai_execution_context: Option<WarpAiExecutionContext>,
    ) -> Result<Vec<AIGeneratedCommand>, GenerateCommandsFromNaturalLanguageError>;

    async fn generate_metadata_for_command(
        &self,
        command: String,
    ) -> Result<GeneratedCommandMetadata, GeneratedCommandMetadataError>;

    async fn create_agent_task(
        &self,
        prompt: String,
        environment_uid: Option<String>,
        parent_run_id: Option<String>,
        config: Option<AgentConfigSnapshot>,
    ) -> Result<AmbientAgentTaskId>;

    async fn update_agent_task(
        &self,
        task_id: AmbientAgentTaskId,
        task_state: Option<AgentTaskState>,
        session_id: Option<session_sharing_protocol::common::SessionId>,
        conversation_id: Option<String>,
        status_message: Option<TaskStatusUpdate>,
    ) -> Result<()>;

    async fn spawn_agent(&self, request: SpawnAgentRequest) -> Result<SpawnAgentResponse>;

    async fn list_ambient_agent_tasks(
        &self,
        limit: i32,
        filter: TaskListFilter,
    ) -> Result<Vec<AmbientAgentTask>>;

    async fn list_agent_runs_raw(
        &self,
        limit: i32,
        filter: TaskListFilter,
    ) -> Result<serde_json::Value>;

    async fn get_ambient_agent_task(
        &self,
        task_id: &AmbientAgentTaskId,
    ) -> Result<AmbientAgentTask>;

    async fn get_agent_run_raw(&self, task_id: &AmbientAgentTaskId) -> Result<serde_json::Value>;

    async fn get_scheduled_agent_history(&self, schedule_id: &str)
        -> Result<ScheduledAgentHistory>;

    async fn get_ai_conversation(
        &self,
        server_conversation_token: ServerConversationToken,
    ) -> Result<(ConversationData, ServerAIConversationMetadata)>;

    async fn list_ai_conversation_metadata(
        &self,
        conversation_ids: Option<Vec<String>>,
    ) -> Result<Vec<ServerAIConversationMetadata>>;

    async fn get_block_snapshot(
        &self,
        server_conversation_token: ServerConversationToken,
    ) -> Result<SerializedBlock>;

    async fn delete_ai_conversation(&self, server_conversation_token: String) -> Result<()>;

    async fn list_agents(&self, repo: Option<String>) -> Result<Vec<AgentListItem>>;

    async fn cancel_ambient_agent_task(&self, task_id: &AmbientAgentTaskId) -> Result<()>;

    async fn get_task_attachments(&self, task_id: String) -> Result<Vec<TaskAttachment>>;

    async fn create_file_artifact_upload_target(
        &self,
        request: CreateFileArtifactUploadRequest,
    ) -> Result<CreateFileArtifactUploadResponse>;

    async fn confirm_file_artifact_upload(
        &self,
        artifact_uid: String,
        checksum: String,
    ) -> Result<FileArtifactRecord>;

    async fn get_artifact_download(&self, artifact_uid: &str) -> Result<ArtifactDownloadResponse>;

    async fn prepare_attachments_for_upload(
        &self,
        task_id: &AmbientAgentTaskId,
        files: &[AttachmentFileInfo],
    ) -> Result<PrepareAttachmentUploadsResponse>;

    async fn download_task_attachments(
        &self,
        task_id: &AmbientAgentTaskId,
        attachment_ids: &[String],
    ) -> Result<DownloadAttachmentsResponse>;

    async fn get_handoff_snapshot_attachments(
        &self,
        task_id: &AmbientAgentTaskId,
    ) -> Result<Vec<TaskAttachment>>;

    async fn send_agent_message(
        &self,
        request: SendAgentMessageRequest,
    ) -> Result<SendAgentMessageResponse>;

    async fn list_agent_messages(
        &self,
        run_id: &str,
        request: ListAgentMessagesRequest,
    ) -> Result<Vec<AgentMessageHeader>>;

    async fn poll_agent_events(
        &self,
        run_ids: &[String],
        since_sequence: i64,
        limit: i32,
    ) -> Result<Vec<AgentRunEvent>>;

    async fn update_event_sequence_on_server(&self, run_id: &str, sequence: i64) -> Result<()>;

    async fn report_agent_event(
        &self,
        run_id: &str,
        request: ReportAgentEventRequest,
    ) -> Result<ReportAgentEventResponse>;

    async fn mark_message_delivered(&self, message_id: &str) -> Result<()>;

    async fn read_agent_message(&self, message_id: &str) -> Result<ReadAgentMessageResponse>;

    async fn get_public_conversation(&self, conversation_id: &str) -> Result<serde_json::Value>;

    async fn get_run_conversation(&self, run_id: &str) -> Result<serde_json::Value>;

    async fn generate_code_review_content(
        &self,
        request: GenerateCodeReviewContentRequest,
    ) -> Result<GenerateCodeReviewContentResponse>;
}

#[cfg_attr(not(target_family = "wasm"), async_trait)]
#[cfg_attr(target_family = "wasm", async_trait(?Send))]
impl AIClient for ServerApi {
    async fn generate_commands_from_natural_language(
        &self,
        _prompt: String,
        _ai_execution_context: Option<WarpAiExecutionContext>,
    ) -> Result<Vec<AIGeneratedCommand>, GenerateCommandsFromNaturalLanguageError> {
        Ok(Vec::new())
    }

    async fn generate_metadata_for_command(
        &self,
        _command: String,
    ) -> Result<GeneratedCommandMetadata, GeneratedCommandMetadataError> {
        Err(GeneratedCommandMetadataError::Other)
    }

    async fn create_agent_task(
        &self,
        _prompt: String,
        _environment_uid: Option<String>,
        _parent_run_id: Option<String>,
        _config: Option<AgentConfigSnapshot>,
    ) -> Result<AmbientAgentTaskId> {
        bail!("agents were removed in Warp Lite")
    }

    async fn update_agent_task(
        &self,
        _task_id: AmbientAgentTaskId,
        _task_state: Option<AgentTaskState>,
        _session_id: Option<session_sharing_protocol::common::SessionId>,
        _conversation_id: Option<String>,
        _status_message: Option<TaskStatusUpdate>,
    ) -> Result<()> {
        Ok(())
    }

    async fn spawn_agent(&self, _request: SpawnAgentRequest) -> Result<SpawnAgentResponse> {
        bail!("agents were removed in Warp Lite")
    }

    async fn list_ambient_agent_tasks(
        &self,
        _limit: i32,
        _filter: TaskListFilter,
    ) -> Result<Vec<AmbientAgentTask>> {
        Ok(Vec::new())
    }

    async fn list_agent_runs_raw(
        &self,
        _limit: i32,
        _filter: TaskListFilter,
    ) -> Result<serde_json::Value> {
        Ok(serde_json::json!({ "runs": [] }))
    }

    async fn get_ambient_agent_task(
        &self,
        _task_id: &AmbientAgentTaskId,
    ) -> Result<AmbientAgentTask> {
        bail!("agents were removed in Warp Lite")
    }

    async fn get_agent_run_raw(&self, _task_id: &AmbientAgentTaskId) -> Result<serde_json::Value> {
        bail!("agents were removed in Warp Lite")
    }

    async fn get_scheduled_agent_history(
        &self,
        _schedule_id: &str,
    ) -> Result<ScheduledAgentHistory> {
        Ok(ScheduledAgentHistory)
    }

    async fn get_ai_conversation(
        &self,
        _server_conversation_token: ServerConversationToken,
    ) -> Result<(ConversationData, ServerAIConversationMetadata)> {
        bail!("AI conversations were removed in Warp Lite")
    }

    async fn list_ai_conversation_metadata(
        &self,
        _conversation_ids: Option<Vec<String>>,
    ) -> Result<Vec<ServerAIConversationMetadata>> {
        Ok(Vec::new())
    }

    async fn get_block_snapshot(
        &self,
        _server_conversation_token: ServerConversationToken,
    ) -> Result<SerializedBlock> {
        bail!("AI conversations were removed in Warp Lite")
    }

    async fn delete_ai_conversation(&self, _server_conversation_token: String) -> Result<()> {
        Ok(())
    }

    async fn list_agents(&self, _repo: Option<String>) -> Result<Vec<AgentListItem>> {
        Ok(Vec::new())
    }

    async fn cancel_ambient_agent_task(&self, _task_id: &AmbientAgentTaskId) -> Result<()> {
        Ok(())
    }

    async fn get_task_attachments(&self, _task_id: String) -> Result<Vec<TaskAttachment>> {
        Ok(Vec::new())
    }

    async fn create_file_artifact_upload_target(
        &self,
        _request: CreateFileArtifactUploadRequest,
    ) -> Result<CreateFileArtifactUploadResponse> {
        bail!("agent artifacts were removed in Warp Lite")
    }

    async fn confirm_file_artifact_upload(
        &self,
        _artifact_uid: String,
        _checksum: String,
    ) -> Result<FileArtifactRecord> {
        bail!("agent artifacts were removed in Warp Lite")
    }

    async fn get_artifact_download(&self, _artifact_uid: &str) -> Result<ArtifactDownloadResponse> {
        bail!("agent artifacts were removed in Warp Lite")
    }

    async fn prepare_attachments_for_upload(
        &self,
        _task_id: &AmbientAgentTaskId,
        _files: &[AttachmentFileInfo],
    ) -> Result<PrepareAttachmentUploadsResponse> {
        Ok(PrepareAttachmentUploadsResponse::default())
    }

    async fn download_task_attachments(
        &self,
        _task_id: &AmbientAgentTaskId,
        _attachment_ids: &[String],
    ) -> Result<DownloadAttachmentsResponse> {
        Ok(DownloadAttachmentsResponse::default())
    }

    async fn get_handoff_snapshot_attachments(
        &self,
        _task_id: &AmbientAgentTaskId,
    ) -> Result<Vec<TaskAttachment>> {
        Ok(Vec::new())
    }

    async fn send_agent_message(
        &self,
        _request: SendAgentMessageRequest,
    ) -> Result<SendAgentMessageResponse> {
        bail!("agent messaging was removed in Warp Lite")
    }

    async fn list_agent_messages(
        &self,
        _run_id: &str,
        _request: ListAgentMessagesRequest,
    ) -> Result<Vec<AgentMessageHeader>> {
        Ok(Vec::new())
    }

    async fn poll_agent_events(
        &self,
        _run_ids: &[String],
        _since_sequence: i64,
        _limit: i32,
    ) -> Result<Vec<AgentRunEvent>> {
        Ok(Vec::new())
    }

    async fn update_event_sequence_on_server(&self, _run_id: &str, _sequence: i64) -> Result<()> {
        Ok(())
    }

    async fn report_agent_event(
        &self,
        _run_id: &str,
        _request: ReportAgentEventRequest,
    ) -> Result<ReportAgentEventResponse> {
        Ok(ReportAgentEventResponse::default())
    }

    async fn mark_message_delivered(&self, _message_id: &str) -> Result<()> {
        Ok(())
    }

    async fn read_agent_message(&self, _message_id: &str) -> Result<ReadAgentMessageResponse> {
        bail!("agent messaging was removed in Warp Lite")
    }

    async fn get_public_conversation(&self, _conversation_id: &str) -> Result<serde_json::Value> {
        bail!("AI conversations were removed in Warp Lite")
    }

    async fn get_run_conversation(&self, _run_id: &str) -> Result<serde_json::Value> {
        bail!("AI conversations were removed in Warp Lite")
    }

    async fn generate_code_review_content(
        &self,
        _request: GenerateCodeReviewContentRequest,
    ) -> Result<GenerateCodeReviewContentResponse> {
        bail!("AI code-review generation was removed in Warp Lite")
    }
}
