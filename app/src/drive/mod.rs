//! Local-only tombstone for Warp Drive.

use std::{cmp::Ordering, fmt};

use serde::{Deserialize, Serialize};
use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

use crate::{
    cloud_object::{
        model::view::{CloudViewModel, UpdateTimestamp},
        CloudObject, GenericStringObjectFormat, ObjectIdType, ObjectType, Space,
    },
    server::ids::{HashedSqliteId, ObjectUid, ServerId, SyncId},
    ui_components::icons::Icon,
};

pub use index::DriveIndexVariant;
pub use panel::{DrivePanel, DrivePanelEvent};

type SortByComparator<'a> = dyn FnMut(&&dyn CloudObject, &&dyn CloudObject) -> Ordering + 'a;

pub mod cloud_object_styling {
    use pathfinder_color::ColorU;

    use crate::{appearance::Appearance, drive::DriveObjectType};
    use warp_core::ui::theme::color::internal_colors;

    pub fn warp_drive_icon_color(appearance: &Appearance, _object_type: DriveObjectType) -> ColorU {
        internal_colors::neutral_7(appearance.theme())
    }
}

pub mod drive_helpers {
    use warpui::AppContext;

    pub fn has_feature_gated_anonymous_user_reached_env_var_limit(_ctx: &mut AppContext) -> bool {
        false
    }

    pub fn has_feature_gated_anonymous_user_reached_notebook_limit(_ctx: &mut AppContext) -> bool {
        false
    }

    pub fn has_feature_gated_anonymous_user_reached_workflow_limit(_ctx: &mut AppContext) -> bool {
        false
    }
}

pub mod export {
    use super::*;

    pub struct ExportManager;

    impl ExportManager {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn export(
            &mut self,
            _window_id: warpui::WindowId,
            _objects: &[CloudObjectTypeAndId],
            _ctx: &mut ModelContext<Self>,
        ) {
        }
    }

    impl Entity for ExportManager {
        type Event = ();
    }

    impl SingletonEntity for ExportManager {}

    pub fn safe_filename(filename: &str) -> String {
        filename
            .chars()
            .map(|c| match c {
                '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' => '_',
                c if c.is_control() => '_',
                c => c,
            })
            .collect()
    }
}

pub mod folders {
    use super::*;

    pub use warp_server_client::ids::FolderId;

    pub type CloudFolder = crate::cloud_object::GenericCloudObject<FolderId, CloudFolderModel>;

    #[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
    pub struct CloudFolderModel {
        pub name: String,
        pub is_open: bool,
        pub is_warp_pack: bool,
    }

    impl CloudFolderModel {
        pub fn new(name: &str, is_warp_pack: bool) -> Self {
            Self {
                name: name.to_string(),
                is_open: true,
                is_warp_pack,
            }
        }
    }

    #[async_trait::async_trait]
    impl crate::cloud_object::CloudModelType for CloudFolderModel {
        type CloudObjectType = CloudFolder;
        type IdType = FolderId;

        fn model_type_name(&self) -> &'static str {
            "folder"
        }

        fn cloud_object_type_and_id(&self, id: SyncId) -> CloudObjectTypeAndId {
            CloudObjectTypeAndId::Folder(id)
        }

        fn object_type(&self) -> ObjectType {
            ObjectType::Folder
        }

        fn renders_in_warp_drive(&self) -> bool {
            false
        }

        fn to_warp_drive_item(
            &self,
            _id: SyncId,
            _appearance: &crate::appearance::Appearance,
            _object: &Self::CloudObjectType,
        ) -> Option<Box<dyn crate::drive::items::WarpDriveItem>> {
            None
        }

        fn display_name(&self) -> String {
            self.name.clone()
        }

        fn set_display_name(&mut self, name: &str) {
            self.name = name.to_string();
        }

        fn upsert_event(&self, object: &Self::CloudObjectType) -> crate::persistence::ModelEvent {
            crate::persistence::ModelEvent::UpsertFolder {
                folder: object.clone(),
            }
        }

        fn bulk_upsert_event(objects: &[Self::CloudObjectType]) -> crate::persistence::ModelEvent {
            crate::persistence::ModelEvent::UpsertFolders(objects.to_vec())
        }

        fn create_object_queue_item(
            &self,
            object: &Self::CloudObjectType,
            entrypoint: crate::cloud_object::CloudObjectEventEntrypoint,
            initiated_by: crate::server::cloud_objects::update_manager::InitiatedBy,
        ) -> Option<crate::server::sync_queue::QueueItem> {
            let crate::server::ids::SyncId::ClientId(id) = object.id else {
                return None;
            };
            Some(crate::server::sync_queue::QueueItem::CreateObject {
                object_type: ObjectType::Folder,
                owner: object.permissions.owner,
                id,
                title: Some(std::sync::Arc::new(self.name.clone())),
                serialized_model: None,
                initial_folder_id: object.metadata.folder_id,
                entrypoint,
                initiated_by,
            })
        }

        fn update_object_queue_item(
            &self,
            _revision_ts: Option<crate::cloud_object::Revision>,
            object: &Self::CloudObjectType,
        ) -> crate::server::sync_queue::QueueItem {
            crate::server::sync_queue::QueueItem::UpdateFolder {
                id: object.id,
                model: std::sync::Arc::new(self.clone()),
            }
        }

        fn serialized(&self) -> crate::server::sync_queue::SerializedModel {
            crate::server::sync_queue::SerializedModel::new(self.name.clone())
        }

        async fn send_create_request(
            _object_client: std::sync::Arc<dyn crate::server::server_api::object::ObjectClient>,
            _request: crate::cloud_object::CreateObjectRequest,
        ) -> anyhow::Result<crate::cloud_object::CreateCloudObjectResult> {
            anyhow::bail!("Warp Drive is disabled in warp-lite")
        }

        async fn send_update_request(
            &self,
            _object_client: std::sync::Arc<dyn crate::server::server_api::object::ObjectClient>,
            _server_id: ServerId,
            _revision: Option<crate::cloud_object::Revision>,
        ) -> anyhow::Result<
            crate::cloud_object::UpdateCloudObjectResult<
                crate::cloud_object::GenericServerObject<Self::IdType, Self>,
            >,
        > {
            anyhow::bail!("Warp Drive is disabled in warp-lite")
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
    }
}

pub mod import {
    pub mod modal {
        use warpui::{elements::Empty, AppContext, Entity, TypedActionView, View, ViewContext};

        use crate::{cloud_object::Owner, server::ids::SyncId};

        #[derive(Clone, Debug)]
        pub enum ImportModalAction {
            Close,
        }

        #[derive(Clone, Debug)]
        pub enum ImportModalEvent {
            OpenTargetWithHashedId(String),
            Close,
        }

        pub struct ImportModal;

        impl ImportModal {
            pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
                Self
            }

            pub fn open_with_target(
                &mut self,
                _owner: Owner,
                _initial_folder_id: Option<SyncId>,
                _ctx: &mut ViewContext<Self>,
            ) {
            }
        }

        impl Entity for ImportModal {
            type Event = ImportModalEvent;
        }

        impl TypedActionView for ImportModal {
            type Action = ImportModalAction;

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl View for ImportModal {
            fn ui_name() -> &'static str {
                "ImportModal"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                Box::new(Empty::new())
            }
        }
    }
}

pub mod index {
    use super::*;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash, Serialize, Deserialize)]
    pub enum DriveIndexVariant {
        #[default]
        MainIndex,
        Trash,
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
    pub enum DriveIndexSection {
        Space(Space),
        CreateATeam,
        JoinTeam,
        Trash,
    }

    #[derive(Clone, Debug)]
    pub enum DriveIndexAction {
        OpenObject(CloudObjectTypeAndId),
        RunObject(CloudObjectTypeAndId),
        RenameFolder {
            folder_id: SyncId,
        },
        CloseCloudObjectNamingDialog,
        CreateObject {
            object_type: DriveObjectType,
            folder_id: Option<SyncId>,
        },
    }

    #[derive(Clone, Debug)]
    pub enum DriveIndexEvent {
        FocusWarpDrive,
    }

    pub fn init(_ctx: &mut AppContext) {}

    pub fn warp_drive_section_header_position_id(section: &DriveIndexSection) -> String {
        format!("WarpDriveSectionHeader_{section:?}")
    }
}

pub mod items {
    use super::*;
    use crate::{appearance::Appearance, themes::theme::Fill};
    use warpui::{elements::MouseStateHandle, AppContext, Element};

    pub mod ai_fact {
        #[derive(Clone, Debug)]
        pub struct WarpDriveAIFact;
    }

    pub mod env_var_collection {
        use crate::{drive::items::WarpDriveItem, drive::items::WarpDriveItemId};

        #[derive(Clone, Debug)]
        pub struct WarpDriveEnvVarCollection;

        impl WarpDriveEnvVarCollection {
            pub fn new<T, U>(_id: T, _collection: U) -> Self {
                Self
            }
        }

        impl WarpDriveItem for WarpDriveEnvVarCollection {
            fn warp_drive_id(&self) -> WarpDriveItemId {
                WarpDriveItemId::AIFactCollection
            }

            fn clone_box(&self) -> Box<dyn WarpDriveItem> {
                Box::new(self.clone())
            }
        }
    }

    pub mod notebook {
        use crate::{drive::items::WarpDriveItem, drive::items::WarpDriveItemId};

        #[derive(Clone, Debug)]
        pub struct WarpDriveNotebook;

        impl WarpDriveNotebook {
            pub fn new<T, U, V>(_id: T, _notebook: U, _is_ai_document: V) -> Self {
                Self
            }
        }

        impl WarpDriveItem for WarpDriveNotebook {
            fn warp_drive_id(&self) -> WarpDriveItemId {
                WarpDriveItemId::AIFactCollection
            }

            fn clone_box(&self) -> Box<dyn WarpDriveItem> {
                Box::new(self.clone())
            }
        }
    }

    pub mod workflow {
        #[derive(Clone, Debug)]
        pub struct WarpDriveWorkflow;

        impl WarpDriveWorkflow {
            pub fn new<T, U>(_workflow: T, _id: U) -> Self {
                Self
            }
        }

        impl super::WarpDriveItem for WarpDriveWorkflow {
            fn warp_drive_id(&self) -> super::WarpDriveItemId {
                super::WarpDriveItemId::Workflow
            }

            fn clone_box(&self) -> Box<dyn super::WarpDriveItem> {
                Box::new(self.clone())
            }
        }
    }

    #[derive(Debug, Clone, PartialEq, Eq, Copy)]
    pub enum WarpDriveItemId {
        Workflow,
        AIFactCollection,
        MCPServerCollection,
        Object(CloudObjectTypeAndId),
        Space(Space),
        Trash,
    }

    impl WarpDriveItemId {
        pub fn drive_row_position_id(&self) -> String {
            match self {
                Self::Workflow => "Workflow".to_string(),
                Self::AIFactCollection => "AI_fact_collection".to_string(),
                Self::MCPServerCollection => "MCP_server_collection".to_string(),
                Self::Object(object_id) => object_id.drive_row_position_id(),
                Self::Space(space) => format!("WarpDriveSpace_{space:?}"),
                Self::Trash => "Trash".to_string(),
            }
        }
    }

    pub trait WarpDriveItem {
        fn display_name(&self) -> Option<String> {
            None
        }

        fn metadata(&self) -> Option<&crate::cloud_object::CloudObjectMetadata> {
            None
        }

        fn object_type(&self) -> Option<DriveObjectType> {
            None
        }

        fn secondary_icon(&self, _color: Option<Fill>) -> Option<Box<dyn Element>> {
            None
        }

        fn click_action(&self) -> Option<crate::drive::index::DriveIndexAction> {
            None
        }

        fn preview(&self, _appearance: &Appearance) -> Option<Box<dyn Element>> {
            None
        }

        fn warp_drive_id(&self) -> WarpDriveItemId;

        fn sync_status_icon(
            &self,
            _sync_queue_is_dequeueing: bool,
            _hover_state: MouseStateHandle,
            _appearance: &Appearance,
        ) -> Option<Box<dyn Element>> {
            None
        }

        fn action_summary(&self, _app: &AppContext) -> Option<String> {
            None
        }

        fn is_folder_open(&self) -> Option<bool> {
            None
        }

        fn clone_box(&self) -> Box<dyn WarpDriveItem>;
    }

    impl Clone for Box<dyn WarpDriveItem> {
        fn clone(&self) -> Self {
            self.clone_box()
        }
    }
}

pub mod panel {
    use super::*;
    use warpui::{elements::Empty, AppContext, Entity, TypedActionView, View, ViewContext};

    pub const MIN_SIDEBAR_WIDTH: f32 = 250.;
    pub const MAX_SIDEBAR_WIDTH_RATIO: f32 = 0.75;

    pub struct DrivePanel;

    #[derive(Clone, Debug)]
    pub enum DrivePanelAction {
        OpenSearch,
        FocusDriveIndex,
    }

    #[derive(Clone, Debug)]
    pub enum DrivePanelEvent {
        RunWorkflow(Box<crate::workflows::CloudWorkflow>),
        InvokeEnvironmentVariables {
            env_var_collection: Box<crate::env_vars::CloudEnvVarCollection>,
            in_subshell: bool,
        },
        OpenSearch,
        OpenSharedObjectsCreationDeniedModal(DriveObjectType, ServerId),
        OpenTeamSettingsPage,
        OpenAIFactCollection,
        OpenMCPServerCollection,
        OpenImportModal {
            owner: crate::cloud_object::Owner,
            initial_folder_id: Option<SyncId>,
        },
        OpenWorkflowModalWithNew {
            space: Space,
            initial_folder_id: Option<SyncId>,
        },
        OpenWorkflowModalWithCloudWorkflow(SyncId),
        OpenNotebook(crate::notebooks::manager::NotebookSource),
        OpenEnvVarCollection(crate::env_vars::manager::EnvVarCollectionSource),
        OpenWorkflowInPane(
            crate::workflows::manager::WorkflowOpenSource,
            crate::workflows::WorkflowViewMode,
        ),
        FocusWarpDrive,
        AttachPlanAsContext(crate::ai::document::ai_document_model::AIDocumentId),
    }

    impl DrivePanel {
        pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
            Self
        }

        pub fn has_warp_drive_initialized_sections(
            &self,
            _app: &AppContext,
        ) -> impl std::future::Future<Output = ()> {
            async {}
        }

        pub fn reset_focused_index_in_warp_drive(
            &mut self,
            _should_scroll: bool,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn scroll_item_into_view(
            &mut self,
            _item_id: crate::drive::items::WarpDriveItemId,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn expand_section_for_drive_item_id(
            &mut self,
            _item_id: crate::drive::items::WarpDriveItemId,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn initialize_drive_section_states(&mut self, _ctx: &mut ViewContext<Self>) {}

        pub fn reset_and_open_to_main_index(&mut self, _ctx: &mut ViewContext<Self>) {}

        pub fn set_focused_item(
            &mut self,
            _item_id: crate::drive::items::WarpDriveItemId,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn open_object_sharing_settings<T>(
            &mut self,
            _object_id: CloudObjectTypeAndId,
            _invitee_email: Option<String>,
            _source: T,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn move_object_to_team_owner(
            &mut self,
            _object_id: CloudObjectTypeAndId,
            _space: Space,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn set_focused_index(&mut self, _index: Option<usize>, _ctx: &mut ViewContext<Self>) {}

        pub fn set_selected_object<T>(&mut self, _id: T, _ctx: &mut ViewContext<Self>) {}

        pub fn create_workflow_with_content<T, U, V, W, X>(
            &mut self,
            _name: T,
            _command: U,
            _content: V,
            _is_for_agent_mode: W,
            _ctx: X,
        ) {
        }

        pub fn open_cloud_object_dialog(
            &mut self,
            _object_type: DriveObjectType,
            _space: Space,
            _folder_id: Option<SyncId>,
            _ctx: &mut ViewContext<Self>,
        ) {
        }

        pub fn undo_trash(
            &mut self,
            _object_id: &CloudObjectTypeAndId,
            _ctx: &mut ViewContext<Self>,
        ) {
        }
    }

    impl Entity for DrivePanel {
        type Event = DrivePanelEvent;
    }

    impl TypedActionView for DrivePanel {
        type Action = DrivePanelAction;

        fn handle_action(&mut self, action: &Self::Action, ctx: &mut ViewContext<Self>) {
            if matches!(action, DrivePanelAction::OpenSearch) {
                ctx.emit(DrivePanelEvent::OpenSearch);
            }
        }
    }

    impl View for DrivePanel {
        fn ui_name() -> &'static str {
            "WarpDrivePanel"
        }

        fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
            Box::new(Empty::new())
        }
    }
}

pub mod settings {
    use settings::{
        macros::define_settings_group, RespectUserSyncSetting, SupportedPlatforms, SyncToCloud,
    };

    use super::DriveSortOrder;

    pub const HAS_AUTO_OPENED_WELCOME_FOLDER: &str = "HasAutoOpenedWelcomeFolder";

    define_settings_group!(WarpDriveSettings, settings: [
        sorting_choice: WarpDriveSortingChoice {
            type: DriveSortOrder,
            default: DriveSortOrder::ByObjectType,
            supported_platforms: SupportedPlatforms::ALL,
            sync_to_cloud: SyncToCloud::Never,
            private: false,
            toml_path: "warp_drive.sorting_choice",
            description: "The sort order for local Warp Drive compatibility.",
        },
        enable_warp_drive: EnableWarpDrive {
            type: bool,
            default: false,
            supported_platforms: SupportedPlatforms::ALL,
            sync_to_cloud: SyncToCloud::Never,
            private: false,
            toml_path: "warp_drive.enabled",
            description: "Whether Warp Drive is enabled.",
        },
        sharing_onboarding_block_shown: WarpDriveSharingOnboardingBlockShown {
            type: bool,
            default: false,
            supported_platforms: SupportedPlatforms::ALL,
            sync_to_cloud: SyncToCloud::Never,
            private: true,
        },
    ]);

    impl WarpDriveSettings {
        pub fn is_warp_drive_enabled(_app: &warpui::AppContext) -> bool {
            false
        }
    }
}

pub mod sharing {
    use super::*;

    pub use warp_server_client::drive::sharing::{
        LinkSharingSubjectType, SharingAccessLevel, Subject, TeamKind, UserKind,
    };

    #[derive(Debug, Clone, Copy)]
    pub enum ContentEditability {
        ReadOnly,
        RequiresLogin,
        Editable,
    }

    impl ContentEditability {
        pub fn can_edit(self) -> bool {
            matches!(self, ContentEditability::Editable)
        }
    }

    #[derive(Debug, Clone)]
    pub enum ShareableObject {
        WarpDriveObject(ServerId),
        Session {
            handle: warpui::WeakViewHandle<crate::terminal::TerminalView>,
            session_id: session_sharing_protocol::common::SessionId,
            started_at: chrono::DateTime<chrono::Local>,
        },
        AIConversation(crate::ai::agent::conversation::AIConversationId),
    }

    impl ShareableObject {
        pub fn link(&self, _app: &AppContext) -> Option<String> {
            None
        }
    }

    pub mod dialog {
        use warpui::{elements::Empty, AppContext, Entity, TypedActionView, View, ViewContext};

        #[derive(Clone, Debug)]
        pub enum SharingDialogAction {
            Close,
        }

        #[derive(Clone, Debug)]
        pub enum SharingDialogEvent {
            Close,
        }

        pub struct SharingDialog {
            target: Option<super::ShareableObject>,
        }

        impl SharingDialog {
            pub fn new<T>(
                _target: Option<super::ShareableObject>,
                _ctx: &mut ViewContext<T>,
            ) -> Self {
                Self { target: _target }
            }

            pub fn set_target<T>(&mut self, target: Option<super::ShareableObject>, _ctx: T) {
                self.target = target;
            }

            pub fn has_target(&self) -> bool {
                self.target.is_some()
            }

            pub fn has_shared_session_target(&self) -> bool {
                matches!(self.target, Some(super::ShareableObject::Session { .. }))
            }

            pub fn editability<T: ?Sized>(&self, _ctx: &T) -> super::ContentEditability {
                super::ContentEditability::ReadOnly
            }

            pub fn copy_link<T>(&mut self, _ctx: T) {}

            pub fn report_open<T, U>(&mut self, _source: T, _ctx: U) {}

            pub fn is_unsharable_conversation<T>(&self, _ctx: T) -> bool {
                false
            }
        }

        impl Entity for SharingDialog {
            type Event = SharingDialogEvent;
        }

        impl TypedActionView for SharingDialog {
            type Action = SharingDialogAction;

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl View for SharingDialog {
            fn ui_name() -> &'static str {
                "SharingDialog"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                Box::new(Empty::new())
            }
        }

        pub fn init(_ctx: &mut AppContext) {}
    }
}

pub mod workflows {
    pub mod ai_assist {
        use serde::{Deserialize, Serialize};

        #[derive(Clone, Debug, Default, Serialize)]
        pub struct GeneratedCommandMetadata;

        #[derive(Clone, Debug, Default, Deserialize)]
        pub enum GeneratedCommandMetadataError {
            #[default]
            Other,
        }

        #[derive(Clone, Debug, Default, Serialize)]
        pub struct GenerateBlockTitleRequest {
            pub command: String,
        }

        #[derive(Clone, Debug, Default, Deserialize)]
        pub struct GenerateBlockTitleResponse {
            pub title: String,
        }
    }

    pub mod arguments {
        #[derive(Clone, Debug, Default)]
        pub struct ArgumentsState {
            pub arguments: Vec<crate::workflows::workflow::Argument>,
        }

        impl ArgumentsState {
            pub fn for_command_workflow(
                _prev_state: &ArgumentsState,
                _input_string: String,
            ) -> Self {
                Self::default()
            }

            pub fn for_saved_prompt(_prev_state: &ArgumentsState, _input_string: String) -> Self {
                Self::default()
            }
        }
    }

    pub mod enum_creation_dialog {
        use warpui::{elements::Empty, AppContext, Entity, TypedActionView, View, ViewContext};

        #[derive(Clone, Debug, Default)]
        pub struct WorkflowEnumData;

        pub struct EnumCreationDialog;

        #[derive(Clone, Debug)]
        pub enum EnumCreationDialogEvent {
            Close,
        }

        impl EnumCreationDialog {
            pub fn new(_ctx: &mut ViewContext<Self>) -> Self {
                Self
            }
        }

        impl Entity for EnumCreationDialog {
            type Event = EnumCreationDialogEvent;
        }

        impl TypedActionView for EnumCreationDialog {
            type Action = ();

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl View for EnumCreationDialog {
            fn ui_name() -> &'static str {
                "EnumCreationDialog"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                Box::new(Empty::new())
            }
        }
    }

    pub mod workflow_arg_selector {
        use warpui::{elements::Empty, AppContext, Entity, TypedActionView, View, ViewContext};

        #[derive(Clone, Debug)]
        pub enum WorkflowArgSelectorEvent {
            NewEnum,
            LoadEnum(usize),
            Edited,
            Close,
            ToggleExpanded,
            InputTab,
            InputShiftTab,
        }

        #[derive(Clone, Debug, Default)]
        pub struct WorkflowArgSelectorStyles;

        pub struct WorkflowArgSelector;

        impl WorkflowArgSelector {
            pub fn new<T>(
                _styles: WorkflowArgSelectorStyles,
                _value: T,
                _ctx: &mut ViewContext<Self>,
            ) -> Self {
                Self
            }
        }

        impl Entity for WorkflowArgSelector {
            type Event = WorkflowArgSelectorEvent;
        }

        impl TypedActionView for WorkflowArgSelector {
            type Action = ();

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl View for WorkflowArgSelector {
            fn ui_name() -> &'static str {
                "WorkflowArgSelector"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                Box::new(Empty::new())
            }
        }
    }

    pub mod workflow_arg_type_helpers {
        use std::collections::HashMap;

        use warpui::AppContext;

        use crate::{
            cloud_object::Owner, drive::workflows::enum_creation_dialog::WorkflowEnumData,
        };

        #[derive(Clone, Copy, Debug, PartialEq, Eq)]
        pub struct ArgumentEditorRowIndex(pub usize);

        pub trait ArgumentTypeEditor {
            fn arg_type_editor(
                &self,
            ) -> &warpui::ViewHandle<
                crate::drive::workflows::workflow_arg_selector::WorkflowArgSelector,
            >;
        }

        pub fn load_workflow_enums_with_owner(
            _owner: Owner,
            _ctx: &mut AppContext,
        ) -> HashMap<crate::server::ids::SyncId, WorkflowEnumData> {
            HashMap::new()
        }

        pub fn create_enum<T>(_row: T, _owner: Owner, _ctx: &mut AppContext) {}

        pub fn edit_enum<T, U>(_row: T, _enum_data: U, _owner: Owner, _ctx: &mut AppContext) {}

        pub fn load_enum<T, U>(
            _row: T,
            _enum_data: U,
            _owner: Owner,
            _ctx: &mut AppContext,
        ) -> bool {
            false
        }

        pub fn extract_typed_argument_from_selector<T>(_selector: T) -> Option<()> {
            None
        }

        pub fn save_enum<T>(_enum_data: T, _owner: Owner, _ctx: &mut AppContext) {}

        pub fn load_argument_into_selector<T, U>(
            _argument: T,
            _selector: U,
            _ctx: &mut AppContext,
        ) {
        }
    }

    pub mod modal {
        use warpui::{elements::Empty, AppContext, Entity, TypedActionView, View, ViewContext};

        use crate::{cloud_object::Owner, drive::items::WarpDriveItemId, server::ids::SyncId};

        #[derive(Clone, Debug)]
        pub enum WorkflowModalAction {
            Close,
        }

        #[derive(Clone, Debug)]
        pub enum WorkflowModalEvent {
            Close,
            AiAssistError(String),
            UpdatedWorkflow(SyncId),
            AiAssistUpgradeError(Option<String>, Option<String>),
            ViewInWarpDrive(WarpDriveItemId),
        }

        pub struct WorkflowModal;

        impl WorkflowModal {
            pub fn new<T, U>(_client: T, _ctx: U) -> Self {
                Self
            }

            pub fn open_with_new(
                &mut self,
                _owner: Owner,
                _initial_folder_id: Option<SyncId>,
                _ctx: &mut ViewContext<Self>,
            ) {
            }

            pub fn open_with_new_workflow(
                &mut self,
                _owner: Owner,
                _initial_folder_id: Option<SyncId>,
                _ctx: &mut ViewContext<Self>,
            ) {
            }

            pub fn open_with_cloud_workflow(
                &mut self,
                _workflow_id: SyncId,
                _ctx: &mut ViewContext<Self>,
            ) {
            }

            pub fn is_open(&self) -> bool {
                false
            }
        }

        impl Entity for WorkflowModal {
            type Event = WorkflowModalEvent;
        }

        impl TypedActionView for WorkflowModal {
            type Action = WorkflowModalAction;

            fn handle_action(&mut self, _action: &Self::Action, _ctx: &mut ViewContext<Self>) {}
        }

        impl View for WorkflowModal {
            fn ui_name() -> &'static str {
                "WorkflowModal"
            }

            fn render(&self, _app: &AppContext) -> Box<dyn warpui::Element> {
                Box::new(Empty::new())
            }
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum DriveObjectType {
    Workflow,
    AgentModeWorkflow,
    AIFact,
    AIFactCollection,
    Notebook { is_ai_document: bool },
    Folder,
    EnvVarCollection,
    MCPServer,
    MCPServerCollection,
}

impl From<DriveObjectType> for Icon {
    fn from(cloud_object_type: DriveObjectType) -> Icon {
        match cloud_object_type {
            DriveObjectType::Workflow => Icon::Workflow,
            DriveObjectType::AgentModeWorkflow => Icon::Prompt,
            DriveObjectType::AIFact | DriveObjectType::AIFactCollection => Icon::BookOpen,
            DriveObjectType::Notebook { is_ai_document } => {
                if is_ai_document {
                    Icon::Compass
                } else {
                    Icon::Notebook
                }
            }
            DriveObjectType::Folder => Icon::Folder,
            DriveObjectType::EnvVarCollection => Icon::EnvVarCollection,
            DriveObjectType::MCPServer | DriveObjectType::MCPServerCollection => Icon::Dataflow,
        }
    }
}

impl fmt::Display for DriveObjectType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriveObjectType::Notebook { .. } => write!(f, "notebook"),
            DriveObjectType::Workflow => write!(f, "workflow"),
            DriveObjectType::Folder => write!(f, "folder"),
            DriveObjectType::EnvVarCollection => write!(f, "env var collection"),
            DriveObjectType::AgentModeWorkflow => write!(f, "prompt"),
            DriveObjectType::AIFact => write!(f, "ai fact"),
            DriveObjectType::AIFactCollection => write!(f, "ai fact collection"),
            DriveObjectType::MCPServer => write!(f, "mcp server"),
            DriveObjectType::MCPServerCollection => write!(f, "mcp server collection"),
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Default, Serialize, Deserialize)]
pub struct OpenWarpDriveObjectSettings {
    pub focused_folder_id: Option<ServerId>,
    pub invitee_email: Option<String>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct OpenWarpDriveObjectArgs {
    pub object_type: ObjectType,
    pub server_id: ServerId,
    pub settings: OpenWarpDriveObjectSettings,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub enum CloudObjectTypeAndId {
    Notebook(SyncId),
    Workflow(SyncId),
    Folder(SyncId),
    GenericStringObject {
        object_type: GenericStringObjectFormat,
        id: SyncId,
    },
}

impl CloudObjectTypeAndId {
    pub fn from_id_and_type(id: SyncId, object_type: ObjectType) -> Self {
        match object_type {
            ObjectType::Notebook => Self::Notebook(id),
            ObjectType::Workflow => Self::Workflow(id),
            ObjectType::Folder => Self::Folder(id),
            ObjectType::GenericStringObject(format) => Self::GenericStringObject {
                object_type: format,
                id,
            },
        }
    }

    pub fn uid(self) -> ObjectUid {
        match self {
            Self::Notebook(id)
            | Self::Workflow(id)
            | Self::Folder(id)
            | Self::GenericStringObject { id, .. } => id.uid(),
        }
    }

    pub fn sync_id(self) -> SyncId {
        match self {
            Self::Notebook(id)
            | Self::Workflow(id)
            | Self::Folder(id)
            | Self::GenericStringObject { id, .. } => id,
        }
    }

    pub fn sqlite_uid_hash(self) -> HashedSqliteId {
        match self {
            CloudObjectTypeAndId::Notebook(id) => id.sqlite_uid_hash(ObjectIdType::Notebook),
            CloudObjectTypeAndId::Workflow(id) => id.sqlite_uid_hash(ObjectIdType::Workflow),
            CloudObjectTypeAndId::Folder(id) => id.sqlite_uid_hash(ObjectIdType::Folder),
            CloudObjectTypeAndId::GenericStringObject { id, .. } => {
                id.sqlite_uid_hash(ObjectIdType::GenericStringObject)
            }
        }
    }

    pub fn object_id_type(&self) -> ObjectIdType {
        match self {
            CloudObjectTypeAndId::Notebook(_) => ObjectIdType::Notebook,
            CloudObjectTypeAndId::Workflow(_) => ObjectIdType::Workflow,
            CloudObjectTypeAndId::GenericStringObject { .. } => ObjectIdType::GenericStringObject,
            CloudObjectTypeAndId::Folder(_) => ObjectIdType::Folder,
        }
    }

    pub fn object_type(&self) -> ObjectType {
        match self {
            CloudObjectTypeAndId::Notebook(_) => ObjectType::Notebook,
            CloudObjectTypeAndId::Workflow(_) => ObjectType::Workflow,
            CloudObjectTypeAndId::Folder(_) => ObjectType::Folder,
            CloudObjectTypeAndId::GenericStringObject { object_type, .. } => {
                ObjectType::GenericStringObject(*object_type)
            }
        }
    }

    pub fn as_folder_id(self) -> Option<SyncId> {
        match self {
            CloudObjectTypeAndId::Folder(id) => Some(id),
            _ => None,
        }
    }

    pub fn as_notebook_id(self) -> Option<SyncId> {
        match self {
            CloudObjectTypeAndId::Notebook(id) => Some(id),
            _ => None,
        }
    }

    pub fn as_generic_string_object_id(self) -> Option<SyncId> {
        match self {
            CloudObjectTypeAndId::GenericStringObject { id, .. } => Some(id),
            _ => None,
        }
    }

    pub fn has_server_id(self) -> bool {
        self.server_id().is_some()
    }

    pub fn server_id(self) -> Option<ServerId> {
        match self {
            CloudObjectTypeAndId::Notebook(SyncId::ServerId(id))
            | CloudObjectTypeAndId::Workflow(SyncId::ServerId(id))
            | CloudObjectTypeAndId::Folder(SyncId::ServerId(id))
            | CloudObjectTypeAndId::GenericStringObject {
                id: SyncId::ServerId(id),
                ..
            } => Some(id),
            _ => None,
        }
    }

    pub fn drive_row_position_id(self) -> String {
        format!("WarpDriveRow_{}", self.uid())
    }

    pub fn from_generic_string_object(object_type: GenericStringObjectFormat, id: SyncId) -> Self {
        Self::GenericStringObject { object_type, id }
    }
}

pub fn should_auto_open_welcome_folder(_app: &mut AppContext) -> bool {
    false
}

pub fn write_has_auto_opened_welcome_folder_to_user_defaults(_app: &mut AppContext) {}

#[derive(
    Default,
    PartialEq,
    Eq,
    Hash,
    Clone,
    Copy,
    Debug,
    Serialize,
    Deserialize,
    schemars::JsonSchema,
    settings_value::SettingsValue,
)]
#[schemars(
    description = "Sort order for local Warp Drive compatibility.",
    rename_all = "snake_case"
)]
pub enum DriveSortOrder {
    #[default]
    ByObjectType,
    ByTimestamp,
    AlphabeticalDescending,
    AlphabeticalAscending,
}

impl DriveSortOrder {
    pub fn sort_by<'a>(
        &self,
        _cloud_model: &'a CloudViewModel,
        _update_timestamp: UpdateTimestamp,
        _app: &'a AppContext,
    ) -> Box<SortByComparator<'a>> {
        Box::new(|a, b| a.display_name().cmp(&b.display_name()))
    }

    pub fn menu_text(&self, _index_variant: index::DriveIndexVariant) -> &str {
        match self {
            DriveSortOrder::ByTimestamp => "Last updated",
            DriveSortOrder::AlphabeticalDescending => "A to Z",
            DriveSortOrder::AlphabeticalAscending => "Z to A",
            DriveSortOrder::ByObjectType => "Type",
        }
    }
}
