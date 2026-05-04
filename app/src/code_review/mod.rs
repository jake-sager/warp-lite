//! Local-only tombstone for Warp's code review product surface.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use warpui::{
    AppContext, Entity, EntityId, ModelContext, SingletonEntity, WeakViewHandle, WindowId,
};

use crate::terminal::{view::TerminalView, CLIAgent};

pub mod telemetry_event {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CodeReviewPaneEntrypoint {
        GitDiffChip,
        AgentModeCompleted,
        AgentModeRunning,
        SlashCommand,
        InvokedByAgent,
        ForceOpened,
        CodeDiffHeader,
        PaneHeader,
        RightPanel,
        CLIAgentView,
        #[default]
        Other,
    }

    impl std::fmt::Display for CodeReviewPaneEntrypoint {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{self:?}")
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum CodeReviewContextDestination {
        #[default]
        AgentInput,
        Pty,
        AgentAttachment,
        ActiveCommandBuffer,
        AgentReview,
        RichInput,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DiffSetContextScope {
        #[default]
        All,
        File,
    }

    #[derive(Clone, Debug, Serialize)]
    pub enum CodeReviewTelemetryEvent {
        Removed,
        PaneOpened {
            entrypoint: CodeReviewPaneEntrypoint,
            is_code_mode_v2: bool,
            cli_agent: Option<crate::terminal::CLIAgent>,
        },
    }

    warp_core::register_telemetry_event!(CodeReviewTelemetryEvent);

    impl warp_core::telemetry::TelemetryEvent for CodeReviewTelemetryEvent {
        fn name(&self) -> &'static str {
            "CodeReview.Removed"
        }

        fn payload(&self) -> Option<serde_json::Value> {
            None
        }

        fn description(&self) -> &'static str {
            "Code review telemetry removed in Warp Lite"
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
}

pub use telemetry_event::CodeReviewTelemetryEvent;

pub mod diff_state {
    use super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum DiffMode {
        #[default]
        Head,
        MainBranch,
        OtherBranch(String),
    }

    impl DiffMode {
        pub fn from_branch(branch: &str, _main_branch: Option<&str>) -> Self {
            Self::OtherBranch(branch.to_string())
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GitDeltaPreference {
        #[default]
        Current,
        OnlyDirty,
        Always,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct DiffStats {
        pub files_changed: usize,
        pub lines_added: usize,
        pub lines_removed: usize,
        pub total_additions: usize,
        pub total_deletions: usize,
    }

    impl DiffStats {
        pub fn has_no_changes(&self) -> bool {
            self.files_changed == 0
                && self.lines_added == 0
                && self.lines_removed == 0
                && self.total_additions == 0
                && self.total_deletions == 0
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum DiffLineType {
        #[default]
        Add,
        Delete,
        Context,
        HunkHeader,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct DiffLine {
        pub line_type: DiffLineType,
        pub text: String,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct DiffHunk {
        pub lines: Vec<DiffLine>,
        pub new_start_line: usize,
        pub new_line_count: usize,
    }

    #[derive(Clone, Debug, Default, Serialize, Deserialize)]
    pub struct FileDiff {
        pub file_path: PathBuf,
        pub hunks: Vec<DiffHunk>,
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum GitFileStatus {
        #[default]
        Modified,
    }

    #[derive(Default)]
    pub struct DiffStateModel;

    impl DiffStateModel {
        pub fn new(_repo_name: Option<String>, _ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub async fn load_diff_data_for_mode(
            _diff_mode: DiffMode,
            _repo_path: PathBuf,
        ) -> anyhow::Result<()> {
            Ok(())
        }

        pub async fn get_all_branches(
            _cwd: &std::path::Path,
            _main: Option<String>,
            _include_remote: bool,
        ) -> anyhow::Result<Vec<(String, bool)>> {
            Ok(Vec::new())
        }

        pub async fn get_all_branches_with_known_main(
            _cwd: &std::path::Path,
            _main: &String,
            _other: Option<String>,
            _include_remote: bool,
        ) -> anyhow::Result<Vec<(String, bool)>> {
            Ok(Vec::new())
        }

        pub fn sort_branches_main_first(
            branches: &[(String, bool)],
        ) -> std::vec::IntoIter<(String, bool)> {
            let mut branches = branches.to_vec();
            branches.sort_by_key(|(_, is_main)| !*is_main);
            branches.into_iter()
        }

        pub fn stop_active_watcher<T>(&mut self, _ctx: T) {}
    }

    impl Entity for DiffStateModel {
        type Event = ();
    }
}

pub mod comments {
    use super::*;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub struct CommentId(pub String);

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum CommentOrigin {
        #[default]
        Local,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct LineDiffContent {
        pub content: String,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub enum AttachedReviewCommentTarget {
        #[default]
        General,
        File {
            path: PathBuf,
        },
        Line {
            path: PathBuf,
            line: usize,
        },
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct AttachedReviewComment {
        pub id: CommentId,
        pub body: String,
        pub target: AttachedReviewCommentTarget,
    }

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct PendingImportedReviewComment;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ReviewComment {
        pub id: CommentId,
        pub body: String,
    }

    #[derive(Clone, Debug, Default)]
    pub struct ReviewCommentBatch {
        pub comments: Vec<AttachedReviewComment>,
    }

    #[derive(Clone, Debug)]
    pub enum ReviewCommentBatchEvent {
        Updated,
    }

    impl Entity for ReviewCommentBatch {
        type Event = ReviewCommentBatchEvent;
    }

    impl ReviewCommentBatch {
        pub fn add_pending_imported_comments<T, U, V>(
            &mut self,
            _comments: T,
            _diff_mode: U,
            _ctx: V,
        ) {
        }

        pub fn upsert_imported_comments<T, U>(&mut self, _comments: T, _ctx: U) {}
    }

    pub fn convert_insert_review_comments(
        comments: Vec<AttachedReviewComment>,
    ) -> Vec<AttachedReviewComment> {
        comments
    }
}

pub mod context {
    use std::collections::HashMap;
    use std::path::Path;

    use warp_editor::render::model::LineCount;
    use warpui::{AppContext, ModelHandle};

    use crate::ai::agent::{AIAgentAttachment, CurrentHead, DiffBase, DiffSetHunk};
    use crate::ai::blocklist::BlocklistAIContextModel;
    use crate::code_review::diff_state::{DiffMode, FileDiff};
    use crate::code_review::DiffSetScope;

    pub fn convert_file_diffs_to_diffset_hunks<'a, I>(
        _files: I,
    ) -> HashMap<String, Vec<DiffSetHunk>>
    where
        I: Iterator<Item = &'a FileDiff>,
    {
        let _ = LineCount::from(0);
        HashMap::new()
    }

    pub fn create_attachment_reference_and_key(
        scope: &DiffSetScope,
        _diff_mode: &DiffMode,
        _main_branch_name: Option<&str>,
        _repo_path: &Path,
    ) -> (String, String) {
        let key = match scope {
            DiffSetScope::All => "changes".to_owned(),
            DiffSetScope::File(path) => path.display().to_string(),
        };
        (format!("<change:{key}>"), key)
    }

    pub fn register_diffset_attachment(
        ai_context_model: &ModelHandle<BlocklistAIContextModel>,
        attachment_key: String,
        file_diffs: HashMap<String, Vec<DiffSetHunk>>,
        current: Option<CurrentHead>,
        base: DiffBase,
        ctx: &mut AppContext,
    ) {
        let attachment = AIAgentAttachment::DiffSet {
            file_diffs,
            current,
            base,
        };
        ai_context_model.update(ctx, |context_model, _| {
            context_model.register_diff_hunk_attachment(attachment_key, attachment);
        });
    }
}

pub mod git_status_update {
    use super::*;

    #[derive(Clone, Debug, Default)]
    pub struct GitStatusMetadata {
        pub repo_path: Option<PathBuf>,
        pub current_branch_name: String,
        pub main_branch_name: String,
        pub stats_against_head: crate::code_review::diff_state::DiffStats,
    }

    #[derive(Clone, Debug)]
    pub enum GitRepoStatusEvent {
        Updated,
        MetadataChanged,
    }

    #[derive(Default)]
    pub struct GitRepoStatusModel;

    impl GitRepoStatusModel {
        pub fn new_for_test<T>(_repo_handle: T, _metadata: Option<GitStatusMetadata>) -> Self {
            Self
        }

        pub fn metadata(&self) -> Option<&GitStatusMetadata> {
            None
        }

        pub fn refresh_metadata<T>(&mut self, _ctx: T) {}
    }

    impl Entity for GitRepoStatusModel {
        type Event = GitRepoStatusEvent;
    }

    #[derive(Default)]
    pub struct GitStatusUpdateModel;

    impl GitStatusUpdateModel {
        pub fn new() -> Self {
            Self
        }

        pub fn subscribe<T, U>(
            &mut self,
            _repo_path: T,
            ctx: &mut U,
        ) -> anyhow::Result<warpui::ModelHandle<GitRepoStatusModel>>
        where
            U: std::ops::DerefMut<Target = warpui::AppContext>,
        {
            Ok(ctx.add_model(|_| GitRepoStatusModel))
        }
    }

    impl Entity for GitStatusUpdateModel {
        type Event = ();
    }

    impl SingletonEntity for GitStatusUpdateModel {}
}

pub mod code_review_header {
    pub const HEADER_BUTTON_PADDING: f32 = 0.;
}

pub mod code_review_view {
    use super::*;
    use crate::code::view::CodeViewAction;
    use crate::util::openable_file_type::FileTarget;
    use warp_util::path::LineAndColumnArg;
    use warpui::elements::Empty;
    use warpui::{Element, ModelHandle, View, ViewContext};

    pub const CODE_REVIEW_TOOLTIP_TEXT: &str = "";
    pub const CONTENT_LEFT_MARGIN: f32 = 0.;
    pub const CONTENT_RIGHT_MARGIN: f32 = 0.;

    #[derive(Clone, Debug)]
    pub enum CodeReviewAction {
        SaveAllUnsavedFiles,
        ShowFindBar,
        UndoRevert,
        ToggleFileSidebar,
    }

    #[derive(Clone, Debug)]
    pub enum CodeReviewViewEvent {
        ReviewSubmitted,
        SubmitReviewComments {
            comments: crate::ai::agent::AgentReviewCommentBatch,
        },
        OpenFileWithTarget {
            path: PathBuf,
            target: FileTarget,
            line_col: Option<LineAndColumnArg>,
        },
        OpenFileInNewTab {
            path: PathBuf,
        },
        OpenLspLogs {
            log_path: PathBuf,
        },
    }

    #[derive(Clone, Debug, Default)]
    pub struct CodeReviewCommentDebugState;

    #[derive(Clone, Debug, Default)]
    pub struct CodeReviewVisibleAnchorForTest;

    pub struct CodeReviewView;

    impl CodeReviewView {
        pub fn new<T, U, V>(
            _repo_path: PathBuf,
            _diff_state_model: ModelHandle<crate::code_review::diff_state::DiffStateModel>,
            _terminal_view: T,
            _working_directories_model: U,
            _cli_agent_sessions_model: V,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self
        }

        pub fn render_loading_state(
            _appearance: &crate::appearance::Appearance,
        ) -> Box<dyn Element> {
            Box::new(Empty::new())
        }

        pub fn render_remote_state(
            _appearance: &crate::appearance::Appearance,
            _button: Option<Box<dyn Element>>,
        ) -> Box<dyn Element> {
            Box::new(Empty::new())
        }

        pub fn render_wsl_state(
            _appearance: &crate::appearance::Appearance,
            _button: Option<Box<dyn Element>>,
        ) -> Box<dyn Element> {
            Box::new(Empty::new())
        }

        pub fn render_not_repo_state(
            _appearance: &crate::appearance::Appearance,
            _button: Option<Box<dyn Element>>,
        ) -> Box<dyn Element> {
            Box::new(Empty::new())
        }

        pub fn render_diff_stats(
            _stats: &crate::code_review::diff_state::DiffStats,
            _appearance: &crate::appearance::Appearance,
        ) -> Box<dyn Element> {
            Box::new(Empty::new())
        }

        pub fn handle_action(&mut self, _action: &CodeReviewAction, _ctx: &mut ViewContext<Self>) {}

        pub fn code_view_action(&mut self, _action: CodeViewAction, _ctx: &mut ViewContext<Self>) {}

        pub fn set_diff_base<T, U: ?Sized>(&mut self, _diff_mode: T, _ctx: &mut U) {}

        pub fn expand_comment_list<T: ?Sized>(&mut self, _ctx: &mut T) {}

        pub fn navigate_to_imported_comment<T, U, V>(
            &mut self,
            _comment_id: T,
            _diff_mode: U,
            _ctx: V,
        ) {
        }
    }

    impl Entity for CodeReviewView {
        type Event = CodeReviewViewEvent;
    }

    impl View for CodeReviewView {
        fn ui_name() -> &'static str {
            "CodeReviewView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Box::new(Empty::new())
        }
    }

    pub fn render_file_navigation_button<T>(_arg: T) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

pub use diff_state::DiffMode;
pub use telemetry_event::CodeReviewPaneEntrypoint;

#[derive(Clone)]
pub struct CodeReviewPanelArg {
    pub repo_path: Option<PathBuf>,
    pub terminal_view: WeakViewHandle<TerminalView>,
    pub entrypoint: CodeReviewPaneEntrypoint,
    pub focus_new_pane: bool,
    pub cli_agent: Option<CLIAgent>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum DiffSetScope {
    All,
    File(PathBuf),
}

pub fn init(_app: &mut AppContext) {}

pub struct GlobalCodeReviewModel;

impl GlobalCodeReviewModel {
    pub fn undo_revert_in_code_review_pane(
        &mut self,
        window_id: WindowId,
        view_id: EntityId,
        ctx: &mut ModelContext<Self>,
    ) {
        ctx.emit(GlobalCodeReviewEvent::DiffReverted { window_id, view_id });
    }
}

pub enum GlobalCodeReviewEvent {
    DiffReverted {
        window_id: WindowId,
        view_id: EntityId,
    },
}

impl SingletonEntity for GlobalCodeReviewModel {}

impl Entity for GlobalCodeReviewModel {
    type Event = GlobalCodeReviewEvent;
}
