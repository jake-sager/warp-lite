//! Local-only tombstone for the code review / agent right panel.

use std::path::PathBuf;

use warp_util::path::LineAndColumnArg;
use warpui::elements::Empty;
use warpui::{
    AppContext, Element, Entity, ModelHandle, TypedActionView, View, ViewContext, ViewHandle,
};

use crate::code_review::diff_state::DiffStateModel;
use crate::pane_group::{PaneGroup, WorkingDirectoriesModel};
use crate::terminal::resizable_data::ModalType;
use crate::terminal::view::TerminalView;
use crate::util::openable_file_type::FileTarget;

pub const HEADER_BUTTON_PADDING: f32 = 0.;

#[derive(Clone, Debug)]
pub enum RightPanelAction {
    ToggleMaximize,
    OpenRepository,
    ToggleFileSidebar,
    SelectRepo { repo_path: PathBuf },
}

#[derive(Clone, Debug)]
pub enum RightPanelEvent {
    ToggleMaximize,
    OpenFileWithTarget {
        path: PathBuf,
        target: FileTarget,
        line_col: Option<LineAndColumnArg>,
    },
    OpenFileInNewTab {
        path: PathBuf,
        line_and_column: Option<LineAndColumnArg>,
    },
    OpenLspLogs {
        log_path: PathBuf,
    },
}

pub struct RightPanelView {
    selected_repo_path: Option<PathBuf>,
    is_maximized: bool,
}

impl RightPanelView {
    pub fn init(_app: &mut AppContext) {}

    pub fn new(
        _working_directories_model: ModelHandle<WorkingDirectoriesModel>,
        _ctx: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            selected_repo_path: None,
            is_maximized: false,
        }
    }

    pub fn set_agent_management_view_open(&mut self, _is_open: bool, _ctx: &mut ViewContext<Self>) {
    }

    pub fn set_panel_position<T>(&mut self, _position: T, _ctx: &mut ViewContext<Self>) {}

    pub fn update_session_env(
        &mut self,
        _is_remote: bool,
        _is_wsl_session: bool,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn selected_repo_path(&self) -> Option<&PathBuf> {
        self.selected_repo_path.as_ref()
    }

    pub fn update_selected_repo(&mut self, repo_path: PathBuf, _ctx: &mut ViewContext<Self>) {
        self.selected_repo_path = Some(repo_path);
    }

    pub fn set_active_pane_group(
        &mut self,
        _pane_group: ViewHandle<PaneGroup>,
        _working_directories_model: &ModelHandle<WorkingDirectoriesModel>,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn open_code_review<T, U>(
        &mut self,
        _repo_path: T,
        _diff_state_model: ModelHandle<DiffStateModel>,
        _terminal_view: U,
        _ctx: &mut ViewContext<Self>,
    ) {
    }

    pub fn close_code_review(&mut self, _ctx: &mut ViewContext<Self>) {
        self.selected_repo_path = None;
    }

    pub fn set_maximized(&mut self, is_maximized: bool, _ctx: &mut ViewContext<Self>) {
        self.is_maximized = is_maximized;
    }

    pub fn focus_active_code_review_view(&self, _ctx: &mut ViewContext<Self>) {}

    pub fn log_review_comment_send_status_for_active_tab(&self, _ctx: &AppContext) {}

    pub fn recompute_terminal_availability(&self, _ctx: &mut ViewContext<Self>) {}
}

impl Entity for RightPanelView {
    type Event = RightPanelEvent;
}

impl View for RightPanelView {
    fn ui_name() -> &'static str {
        "RightPanelView"
    }

    fn render(&self, _app: &AppContext) -> Box<dyn Element> {
        Box::new(Empty::new())
    }
}

impl TypedActionView for RightPanelView {
    type Action = RightPanelAction;

    fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
        match action {
            RightPanelAction::ToggleMaximize => ctx.emit(RightPanelEvent::ToggleMaximize),
            RightPanelAction::SelectRepo { repo_path } => {
                self.selected_repo_path = Some(repo_path.clone());
            }
            RightPanelAction::OpenRepository | RightPanelAction::ToggleFileSidebar => {}
        }
    }
}
