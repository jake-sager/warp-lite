//! Local-only tombstone for Warp's code editor, LSP, and file-tree product surfaces.

use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use warp_util::file::FileSaveError;
use warpui::elements::Empty;
use warpui::{
    AppContext, Element, Entity, ModelContext, ModelHandle, SingletonEntity, TypedActionView, View,
    ViewContext,
};

pub mod file_tree;

#[derive(Debug, thiserror::Error)]
pub enum ImmediateSaveError {
    #[error("code editor removed")]
    Removed,
    #[error("failed to save file: {0:#}")]
    FailedToSave(#[from] FileSaveError),
}

#[derive(Debug)]
pub enum SaveStatus {
    SavedImmediately,
    AsyncSaveInProgress,
    Failed(ImmediateSaveError),
}

#[derive(Debug, Eq, PartialEq)]
pub enum SaveOutcome {
    Canceled,
    Failed,
    Succeeded,
}

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct DiffResult {
    pub unified_diff: String,
    pub lines_added: usize,
    pub lines_removed: usize,
}

impl std::ops::AddAssign<&DiffResult> for DiffResult {
    fn add_assign(&mut self, other: &DiffResult) {
        self.lines_added += other.lines_added;
        self.lines_removed += other.lines_removed;
        if !self.unified_diff.is_empty() && !other.unified_diff.is_empty() {
            self.unified_diff.push('\n');
        }
        self.unified_diff.push_str(&other.unified_diff);
    }
}

#[derive(Debug)]
pub struct EditorTabBarDropTargetData {
    pub index: usize,
}

impl warpui::elements::DropTargetData for EditorTabBarDropTargetData {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

pub trait ShowCommentEditorProvider: std::fmt::Debug + 'static {
    fn should_show_comment_editor(
        &self,
        _editor_line_location: pathfinder_geometry::rect::RectF,
        _app: &AppContext,
    ) -> bool {
        false
    }
}

pub trait ShowFindReferencesCardProvider: std::fmt::Debug + 'static {
    fn should_show_find_references_card(
        &self,
        _card_anchor_location: pathfinder_geometry::rect::RectF,
        _app: &AppContext,
    ) -> bool {
        false
    }
}

#[derive(Debug)]
pub struct NoopFindReferencesCardProvider;

impl ShowFindReferencesCardProvider for NoopFindReferencesCardProvider {}

pub mod active_file {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum ActiveFileEvent {}

    #[derive(Default)]
    pub struct ActiveFileModel;

    impl ActiveFileModel {
        pub fn new() -> Self {
            Self
        }

        pub fn active_file_changed<T, U>(&mut self, _path: T, _ctx: U) {}
    }

    impl Entity for ActiveFileModel {
        type Event = ActiveFileEvent;
    }
}

pub mod editor_management {
    use super::*;

    #[derive(Debug, Hash, Eq, PartialEq, Clone, Serialize, Deserialize)]
    pub enum CodeSource {
        Local,
        FileTree {
            path: PathBuf,
        },
        Finder {
            path: PathBuf,
        },
        Link {
            path: PathBuf,
            range_start: Option<warp_util::path::LineAndColumnArg>,
            range_end: Option<warp_util::path::LineAndColumnArg>,
        },
        AIAction {
            id: crate::ai::agent::AIAgentActionId,
        },
        ProjectRules {
            path: PathBuf,
        },
        New {
            default_directory: Option<PathBuf>,
        },
        Skill {
            reference: crate::ai::skills::SkillReference,
            path: PathBuf,
            origin: crate::ai::skills::SkillOpenOrigin,
        },
    }

    impl CodeSource {
        pub fn is_restorable(&self) -> bool {
            false
        }

        pub fn path(&self) -> Option<PathBuf> {
            match self {
                Self::FileTree { path } | Self::Finder { path } | Self::Link { path, .. } => {
                    Some(path.clone())
                }
                Self::ProjectRules { path } | Self::Skill { path, .. } => Some(path.clone()),
                Self::Local | Self::AIAction { .. } | Self::New { .. } => None,
            }
        }

        pub fn telemetry_source_name(&self) -> &'static str {
            "removed"
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct CodeEditorStatus {
        pub unsaved_changes: bool,
    }

    impl CodeEditorStatus {
        pub fn new(unsaved_changes: bool) -> Self {
            Self { unsaved_changes }
        }

        pub fn editor_status<T, U>(_view: T, _app: U) -> Self {
            Self::default()
        }

        pub fn editors_in_tab<T, U>(_pane_group: T, _ctx: U) -> std::vec::IntoIter<Self> {
            Vec::new().into_iter()
        }

        pub fn editors_in_window<T, U>(_window_id: T, _ctx: U) -> std::vec::IntoIter<Self> {
            Vec::new().into_iter()
        }

        pub fn all_editors<T>(_ctx: T) -> std::vec::IntoIter<Self> {
            Vec::new().into_iter()
        }

        pub fn code_review_views_in_window<T, U>(
            _window_id: T,
            _ctx: U,
        ) -> std::vec::IntoIter<Self> {
            Vec::new().into_iter()
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct CodeEditorSummary<'a> {
        pub unsaved_changes: Vec<&'a CodeEditorStatus>,
    }

    impl<'a> CodeEditorSummary<'a> {
        pub fn new(editors: &'a [CodeEditorStatus]) -> Self {
            Self {
                unsaved_changes: editors
                    .iter()
                    .filter(|editor| editor.unsaved_changes)
                    .collect(),
            }
        }
    }

    #[derive(Default)]
    pub struct CodeManager;

    impl CodeManager {
        pub fn new() -> Self {
            Self
        }

        pub fn register_pane<T, U, V, W>(
            &mut self,
            _pane_group_id: T,
            _window_id: U,
            _pane_id: V,
            _source: W,
        ) {
        }

        pub fn deregister_pane<T>(&mut self, _source: T) {}

        pub fn get_locator_for_path_in_tab<T, U>(
            &self,
            _pane_group_id: T,
            _path: U,
        ) -> Option<crate::workspace::PaneViewLocator> {
            None
        }
    }

    impl Entity for CodeManager {
        type Event = ();
    }

    impl SingletonEntity for CodeManager {}
}

pub mod opened_files {
    use super::*;
    use instant::Instant;
    use std::collections::HashMap;

    #[derive(Default, Clone)]
    pub struct OpenedFilesInRepo(HashMap<PathBuf, Instant>);

    impl OpenedFilesInRepo {
        pub fn get(&self, file_path: &PathBuf) -> Option<&Instant> {
            self.0.get(file_path)
        }

        pub fn iter(&self) -> impl Iterator<Item = (&PathBuf, &Instant)> {
            self.0.iter()
        }
    }

    #[derive(Default)]
    pub struct OpenedFilesModel {
        opened_files: HashMap<PathBuf, OpenedFilesInRepo>,
    }

    impl OpenedFilesModel {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn file_opened<T, U, V>(&mut self, _repo_path: T, _path: U, _ctx: V) {}

        pub fn opened_files_for_repo(&self, repo: &PathBuf) -> Option<&OpenedFilesInRepo> {
            self.opened_files.get(repo)
        }
    }

    impl Entity for OpenedFilesModel {
        type Event = ();
    }

    impl SingletonEntity for OpenedFilesModel {}
}

pub mod global_buffer_model {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum GlobalBufferModelEvent {}

    #[derive(Default)]
    pub struct GlobalBufferModel;

    impl GlobalBufferModel {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }
    }

    impl Entity for GlobalBufferModel {
        type Event = GlobalBufferModelEvent;
    }

    impl SingletonEntity for GlobalBufferModel {}
}

pub mod editor {
    use super::*;

    pub fn add_color(appearance: &crate::appearance::Appearance) -> pathfinder_color::ColorU {
        warp_core::ui::theme::AnsiColorIdentifier::Green
            .to_ansi_color(&appearance.theme().terminal_colors().normal)
            .into()
    }

    pub fn remove_color(appearance: &crate::appearance::Appearance) -> pathfinder_color::ColorU {
        warp_core::ui::theme::AnsiColorIdentifier::Red
            .to_ansi_color(&appearance.theme().terminal_colors().normal)
            .into()
    }

    pub mod scroll {
        #[derive(Clone, Copy, Debug, PartialEq)]
        pub enum ScrollPosition {
            LineAndColumn(warp_util::path::LineAndColumnArg),
        }

        impl Default for ScrollPosition {
            fn default() -> Self {
                Self::LineAndColumn(warp_util::path::LineAndColumnArg {
                    line_num: 1,
                    column_num: Some(0),
                })
            }
        }

        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum ScrollTrigger {
            #[default]
            Unknown,
        }

        impl ScrollTrigger {
            pub fn new<T>(_value: T) -> Self {
                Self::Unknown
            }
        }
    }

    pub mod find {
        pub mod view {
            pub fn init(_app: &mut warpui::AppContext) {}
        }
    }

    pub mod line {
        #[derive(
            Clone, Copy, Debug, Default, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize,
        )]
        pub struct EditorLineLocation {
            pub line: usize,
        }
    }

    pub mod diff {
        #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
        pub enum ChangeType {
            #[default]
            Unknown,
        }
    }

    #[derive(Clone, Debug, Default)]
    pub struct EditorReviewComment {
        pub uuid: crate::code_review::comments::CommentId,
    }

    #[derive(Clone, Debug)]
    pub enum CommentEditorEvent {}

    #[derive(Default)]
    pub struct CommentEditor;

    impl Entity for CommentEditor {
        type Event = CommentEditorEvent;
    }

    impl View for CommentEditor {
        fn ui_name() -> &'static str {
            "CommentEditor"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Box::new(Empty::new())
        }
    }

    #[derive(Default)]
    pub struct EditorCommentsModel;

    impl Entity for EditorCommentsModel {
        type Event = ();
    }

    pub mod comment_editor {
        pub const DEFAULT_COMMENT_MAX_WIDTH: f32 = 0.;
        pub use super::{CommentEditor, CommentEditorEvent};

        pub fn create_readonly_comment_markdown_editor<T>(_comment: T) -> super::CommentEditor {
            super::CommentEditor
        }
    }

    pub mod comments {
        pub use super::EditorCommentsModel;
        pub use crate::code_review::comments::*;
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum NavBarBehavior {
        #[default]
        Hidden,
    }

    pub mod view {
        use super::*;

        #[derive(Clone, Debug, Default)]
        pub struct CodeEditorRenderOptions;

        impl CodeEditorRenderOptions {
            pub fn new<T>(_arg: T) -> Self {
                Self
            }
        }

        #[derive(Clone, Debug)]
        pub enum CodeEditorEvent {
            DiffUpdated,
            UnifiedDiffComputed(std::rc::Rc<crate::code::DiffResult>),
            ContentChanged {
                origin: crate::editor::InteractionState,
            },
            DeleteComment {
                id: crate::code_review::comments::CommentId,
            },
        }

        #[derive(Clone, Debug)]
        pub enum CodeEditorViewAction {}

        #[derive(Default)]
        pub struct CodeEditorView;

        impl CodeEditorView {
            pub fn new<T>(_options: T, _ctx: &mut ViewContext<Self>) -> Self {
                Self
            }

            pub fn init(_app: &mut AppContext) {}

            pub fn version(&self, _ctx: &AppContext) -> usize {
                0
            }

            pub fn reset<T>(&mut self, _content: T, _ctx: &mut ViewContext<Self>) {}

            pub fn buffer_version(&self) -> usize {
                0
            }

            pub fn set_interaction_state(
                &mut self,
                _interaction_state: crate::editor::InteractionState,
                _ctx: &mut ViewContext<Self>,
            ) {
            }

            pub fn set_pending_scroll(
                &mut self,
                _position: crate::code::editor::scroll::ScrollPosition,
            ) {
            }
        }

        impl Entity for CodeEditorView {
            type Event = CodeEditorEvent;
        }

        impl View for CodeEditorView {
            fn ui_name() -> &'static str {
                "CodeEditorView"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn Element> {
                Box::new(Empty::new())
            }
        }

        impl TypedActionView for CodeEditorView {
            type Action = CodeEditorViewAction;

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }
    }
}

pub mod lsp_telemetry {
    #[derive(Clone, Copy, Debug, Default)]
    pub enum LspControlActionType {
        #[default]
        Unknown,
    }

    #[derive(Clone, Copy, Debug, Default)]
    pub enum LspEnablementSource {
        #[default]
        Unknown,
        InitFlow,
    }

    #[derive(Clone, Debug)]
    pub enum LspTelemetryEvent {
        ServerInstallCompleted {
            server_type: String,
            success: bool,
        },
        ServerEnabled {
            server_type: String,
            source: LspEnablementSource,
            needed_install: bool,
        },
        ServerEnablementSkipped,
    }

    warp_core::register_telemetry_event!(LspTelemetryEvent);

    impl warp_core::telemetry::TelemetryEvent for LspTelemetryEvent {
        fn name(&self) -> &'static str {
            "LSP Removed"
        }

        fn payload(&self) -> Option<serde_json::Value> {
            None
        }

        fn description(&self) -> &'static str {
            "LSP telemetry removed in Warp Lite"
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

#[cfg(feature = "local_fs")]
pub mod language_server_shutdown_manager {
    use super::*;

    #[derive(Default)]
    pub struct LanguageServerShutdownManager;

    impl LanguageServerShutdownManager {
        pub fn new() -> Self {
            Self
        }
    }

    impl Entity for LanguageServerShutdownManager {
        type Event = ();
    }

    impl SingletonEntity for LanguageServerShutdownManager {}
}

pub mod local_code_editor {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum ShowFindReferencesCard {
        #[default]
        No,
        Yes,
    }

    #[derive(Clone, Debug)]
    pub enum LocalCodeEditorEvent {}

    #[derive(Default)]
    pub struct LocalCodeEditorView;

    impl LocalCodeEditorView {
        pub fn init(_app: &mut AppContext) {}
    }

    impl Entity for LocalCodeEditorView {
        type Event = LocalCodeEditorEvent;
    }

    impl View for LocalCodeEditorView {
        fn ui_name() -> &'static str {
            "LocalCodeEditorView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Box::new(Empty::new())
        }
    }

    pub fn init(_app: &mut AppContext) {}
}

pub mod diff_viewer {
    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub enum DisplayMode {
        #[default]
        View,
    }

    #[derive(Default)]
    pub struct DiffViewer;
}

pub mod inline_diff {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum InlineDiffViewEvent {}

    #[derive(Default)]
    pub struct InlineDiffView;

    impl Entity for InlineDiffView {
        type Event = InlineDiffViewEvent;
    }

    impl View for InlineDiffView {
        fn ui_name() -> &'static str {
            "InlineDiffView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Box::new(Empty::new())
        }
    }
}

pub mod footer {
    use super::*;

    #[derive(Clone, Debug)]
    pub enum CodeFooterViewEvent {}

    #[derive(Default)]
    pub struct CodeFooterView;

    impl Entity for CodeFooterView {
        type Event = CodeFooterViewEvent;
    }

    impl View for CodeFooterView {
        fn ui_name() -> &'static str {
            "CodeFooterView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Box::new(Empty::new())
        }
    }
}

pub mod view {
    use super::*;
    use warp_util::path::LineAndColumnArg;

    pub const SAVE_FILE_BINDING_NAME: &str = "code_view:save";
    pub const SAVE_FILE_BINDING_DESCRIPTION: &str = "Save file";

    #[derive(Debug, Clone)]
    pub enum CodeViewAction {
        SaveFile,
        SaveFileAs,
        AcceptPendingDiffsAndSave,
        RejectPendingDiffs,
        CloseAll,
        CloseSaved,
        ToggleMaximized,
        RemoveTabAtIndex { index: usize },
    }

    #[derive(Debug, Clone)]
    pub enum CodeViewEvent {
        Pane(crate::pane_group::pane::PaneEvent),
        TabChanged {
            file_path: Option<PathBuf>,
            tab_index: usize,
        },
        FileOpened {
            file_path: PathBuf,
            tab_index: usize,
        },
        RunTabConfigSkill {
            path: PathBuf,
        },
        OpenLspLogs {
            log_path: PathBuf,
        },
    }

    pub struct CodeView {
        source: crate::code::editor_management::CodeSource,
        pane_configuration: ModelHandle<crate::pane_group::pane::PaneConfiguration>,
    }

    impl CodeView {
        pub fn restore<T, U, V>(
            _tabs: T,
            _active_tab_index: U,
            _source: V,
            _ctx: &mut ViewContext<Self>,
        ) -> Self {
            Self {
                source: crate::code::editor_management::CodeSource::Local,
                pane_configuration: _ctx
                    .add_model(|_| crate::pane_group::pane::PaneConfiguration::new("Code")),
            }
        }

        pub fn new<T, U>(
            _source: crate::code::editor_management::CodeSource,
            _line_col: T,
            _ctx: &mut ViewContext<U>,
        ) -> Self {
            Self {
                source: _source,
                pane_configuration: _ctx
                    .add_model(|_| crate::pane_group::pane::PaneConfiguration::new("Code")),
            }
        }

        pub fn new_preview<U>(
            _source: crate::code::editor_management::CodeSource,
            _ctx: &mut ViewContext<U>,
        ) -> Self {
            Self {
                source: _source,
                pane_configuration: _ctx
                    .add_model(|_| crate::pane_group::pane::PaneConfiguration::new("Code")),
            }
        }

        pub fn pane_configuration(
            &self,
        ) -> ModelHandle<crate::pane_group::pane::PaneConfiguration> {
            self.pane_configuration.clone()
        }

        pub fn source(&self) -> &crate::code::editor_management::CodeSource {
            &self.source
        }

        pub fn open_in_preview_or_promote_and_jump(
            &mut self,
            _path: PathBuf,
            _line_col: Option<LineAndColumnArg>,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn open_or_focus_existing(
            &mut self,
            _path: Option<PathBuf>,
            _line_col: Option<LineAndColumnArg>,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn set_original_pane_id(&mut self, _pane_id: Option<crate::pane_group::pane::PaneId>) {}

        pub fn local_path(&self, _ctx: &AppContext) -> Option<PathBuf> {
            None
        }

        pub fn focus<T>(&mut self, _ctx: T) {}

        pub fn active_tab_has_unsaved_changes<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn contains_unsaved_changes<T>(&self, _ctx: T) -> bool {
            false
        }

        pub fn selected_text<T>(&self, _ctx: T) -> Option<String> {
            None
        }

        pub fn active_tab_index(&self) -> usize {
            0
        }

        pub fn tab_count(&self) -> usize {
            0
        }

        pub fn tab_at(&self, _index: usize) -> Option<CodeViewTab> {
            None
        }

        pub fn cleanup_all_tabs<T>(&mut self, _ctx: T) {}

        pub fn close_overlays<T>(&mut self, _ctx: T) {}

        pub fn remove_tab_for_move<T>(
            &mut self,
            _index: usize,
            _ctx: T,
        ) -> Option<crate::pane_group::CodePane> {
            None
        }

        pub fn close_tabs_with_path<T, U>(&mut self, _path: T, _ctx: U) {}

        pub fn rename_tabs_with_path<T, U, V>(&mut self, _old_path: T, _new_path: U, _ctx: V) {}

        pub fn set_active_tab_index<T, U>(&mut self, _index: T, _ctx: U) {}

        pub fn merge_tabs<T, U>(&mut self, _source_code_view: T, _ctx: U) {}
    }

    #[derive(Clone, Debug)]
    pub struct CodeViewTab {
        path: PathBuf,
    }

    impl CodeViewTab {
        pub fn path(&self) -> Option<PathBuf> {
            Some(self.path.clone())
        }
    }

    impl Entity for CodeView {
        type Event = CodeViewEvent;
    }

    impl View for CodeView {
        fn ui_name() -> &'static str {
            "CodeView"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn Element> {
            Box::new(Empty::new())
        }
    }

    impl TypedActionView for CodeView {
        type Action = CodeViewAction;

        fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
    }

    impl crate::pane_group::pane::BackingView for CodeView {
        type PaneHeaderOverflowMenuAction = ();
        type CustomAction = ();
        type AssociatedData = ();

        fn handle_pane_header_overflow_menu_action(
            &mut self,
            _action: &Self::PaneHeaderOverflowMenuAction,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        fn close(&mut self, ctx: &mut ViewContext<Self>) {
            ctx.emit(CodeViewEvent::Pane(
                crate::pane_group::pane::PaneEvent::Close,
            ));
        }

        fn focus_contents(&mut self, _ctx: &mut ViewContext<Self>) {}

        fn render_header_content(
            &self,
            _ctx: &crate::pane_group::pane::view::HeaderRenderContext<'_>,
            _app: &AppContext,
        ) -> crate::pane_group::pane::view::HeaderContent {
            crate::pane_group::pane::view::HeaderContent::simple("Code")
        }

        fn set_focus_handle(
            &mut self,
            _focus_handle: crate::pane_group::focus_state::PaneFocusHandle,
            _ctx: &mut ViewContext<Self>,
        ) {
        }
    }

    pub fn init(_app: &mut AppContext) {}

    pub use crate::util::openable_file_type::is_binary_file;
}

pub fn icon_from_file_path(
    _path: &str,
    _appearance: &crate::appearance::Appearance,
) -> Option<Box<dyn Element>> {
    None
}

pub fn init(app: &mut AppContext) {
    self::view::init(app);
    self::file_tree::init(app);
}
