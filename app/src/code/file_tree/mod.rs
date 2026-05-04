//! Local-only tombstone for the project file tree.

use std::path::PathBuf;

use warp_util::path::LineAndColumnArg;
use warpui::elements::Empty;
use warpui::{AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext};

use crate::code::active_file::ActiveFileModel;
use crate::coding_panel_enablement_state::CodingPanelEnablementState;
use crate::util::openable_file_type::FileTarget;

#[derive(Debug, Clone)]
pub enum FileTreeEvent {
    AttachAsContext {
        path: PathBuf,
    },
    OpenFile {
        path: PathBuf,
        target: FileTarget,
        line_col: Option<LineAndColumnArg>,
    },
    FileRenamed {
        old_path: PathBuf,
        new_path: PathBuf,
    },
    FileDeleted {
        path: PathBuf,
    },
    CDToDirectory {
        path: PathBuf,
    },
    OpenDirectoryInNewTab {
        path: PathBuf,
    },
}

#[derive(Debug, Clone)]
pub enum FileTreeAction {}

pub struct FileTreeView;

impl FileTreeView {
    pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
        Self
    }

    pub fn set_is_active(&mut self, _is_active: bool, _ctx: &mut ViewContext<Self>) {}

    pub fn set_root_directories(&mut self, _paths: Vec<PathBuf>, _ctx: &mut ViewContext<Self>) {}

    pub fn set_remote_root_directories<T, U>(&mut self, _paths: T, _ctx: U) {}

    pub fn set_has_terminal_session(
        &mut self,
        _has_terminal_session: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn set_active_file_model(
        &mut self,
        _active_file_model: ModelHandle<ActiveFileModel>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub(crate) fn set_enablement_state(
        &mut self,
        _enablement: CodingPanelEnablementState,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn auto_expand_to_most_recent_directory(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn on_left_panel_focused(&mut self, _ctx: &mut ViewContext<Self>) {}

    pub fn select_first_item_if_no_selection(&mut self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for FileTreeView {
    type Event = FileTreeEvent;
}

impl View for FileTreeView {
    fn ui_name() -> &'static str {
        "FilePicker"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

impl TypedActionView for FileTreeView {
    type Action = FileTreeAction;

    fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
}

pub fn init(_app: &mut AppContext) {}
