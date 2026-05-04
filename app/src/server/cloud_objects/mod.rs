//! Local-only tombstone for Warp cloud object sync.

pub mod listener {
    use chrono::{DateTime, Utc};
    use warpui::{AppContext, Entity, ModelContext, SingletonEntity};

    use crate::{
        cloud_object::{CloudObjectMetadata, CloudObjectPermissions},
        server::ids::ServerId,
    };

    #[derive(Clone, Debug)]
    pub enum ObjectUpdateMessage {
        ObjectContentChanged {
            object_uid: ServerId,
        },
        ObjectMetadataChanged {
            metadata: CloudObjectMetadata,
        },
        ObjectPermissionsChanged,
        ObjectPermissionsChangedV2 {
            object_uid: ServerId,
            permissions: CloudObjectPermissions,
        },
        ObjectDeleted {
            object_uid: ServerId,
        },
        ObjectActionOccurred {
            history: crate::cloud_object::model::actions::ObjectActionHistory,
        },
        TeamMembershipsChanged,
        AmbientTaskUpdated {
            task_id: String,
            timestamp: DateTime<Utc>,
        },
    }

    pub struct Listener;

    impl Listener {
        pub fn new<T, U>(_client: T, _ctx: U) -> Self {
            Self
        }

        pub fn mock(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn queued_messages(&self) -> usize {
            0
        }

        pub fn connected(&self) -> bool {
            false
        }
    }

    impl Entity for Listener {
        type Event = ObjectUpdateMessage;
    }

    impl SingletonEntity for Listener {}

    pub fn init(_ctx: &mut AppContext) {}
}

pub mod update_manager {
    use std::{collections::HashMap, sync::Arc};

    use chrono::{DateTime, Utc};
    use serde::{Deserialize, Serialize};
    use warpui::{Entity, ModelContext, SingletonEntity};

    use crate::{
        cloud_object::{
            model::{
                actions::ObjectActionHistory,
                generic_string_model::{
                    GenericStringModel, GenericStringObjectId, Serializer, StringModel,
                },
            },
            CloudObjectEventEntrypoint, GenericCloudObject, GenericStringObjectFormat, ObjectType,
            ServerCloudObject, ServerObject,
        },
        drive::CloudObjectTypeAndId,
        notebooks::NotebookId,
        persistence::ModelEvent,
        server::{
            ids::{ClientId, ServerId, SyncId},
            server_api::object::ObjectClient,
        },
        workflows::WorkflowId,
    };
    use warp_server_client::ids::FolderId;

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum OperationSuccessType {
        Success,
        Failure,
        Rejection,
        Denied(String),
        FeatureNotAvailable,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub enum ObjectOperation {
        Create { initiated_by: InitiatedBy },
        Update,
        MoveToFolder,
        MoveToDrive,
        Trash,
        TakeEditAccess,
        Untrash,
        Delete { initiated_by: InitiatedBy },
        EmptyTrash,
        UpdatePermissions,
        Leave,
    }

    #[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
    pub struct ObjectOperationResult {
        pub success_type: OperationSuccessType,
        pub operation: ObjectOperation,
        pub client_id: Option<ClientId>,
        pub server_id: Option<ServerId>,
        pub num_objects: Option<i32>,
    }

    #[derive(Debug, Clone)]
    pub enum UpdateManagerEvent {
        ObjectOperationComplete {
            result: ObjectOperationResult,
        },
        CloudPreferencesUpdated {
            updated: Vec<crate::settings::cloud_preferences::Preference>,
        },
        MCPGalleryUpdated {
            templates: Vec<warp_graphql::mcp_gallery_template::MCPGalleryTemplate>,
        },
        AmbientTaskUpdated {
            timestamp: DateTime<Utc>,
        },
    }

    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum FetchSingleObjectOption {
        None,
        ForceOverwrite,
        IgnoreIfExists,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
    pub enum InitiatedBy {
        User,
        System,
    }

    #[derive(Default)]
    pub struct InitialLoadResponse {
        pub updated_notebooks: Vec<crate::cloud_object::ServerNotebook>,
        pub deleted_notebooks: Vec<NotebookId>,
        pub updated_workflows: Vec<crate::cloud_object::ServerWorkflow>,
        pub deleted_workflows: Vec<WorkflowId>,
        pub updated_folders: Vec<crate::cloud_object::ServerFolder>,
        pub deleted_folders: Vec<FolderId>,
        pub updated_generic_string_objects:
            HashMap<GenericStringObjectFormat, Vec<Box<dyn ServerObject>>>,
        pub deleted_generic_string_objects: Vec<GenericStringObjectId>,
        pub user_profiles: Vec<crate::workspaces::user_profiles::UserProfileWithUID>,
        pub action_histories: Vec<ObjectActionHistory>,
        pub mcp_gallery: Vec<warp_graphql::mcp_gallery_template::MCPGalleryTemplate>,
    }

    pub struct GetCloudObjectResponse {
        pub object: ServerCloudObject,
        pub descendants: Vec<ServerCloudObject>,
        pub action_histories: Vec<ObjectActionHistory>,
    }

    #[derive(Debug)]
    pub struct GenericStringObjectInput<T, S>
    where
        T: StringModel<
                CloudObjectType = GenericCloudObject<
                    GenericStringObjectId,
                    GenericStringModel<T, S>,
                >,
            > + 'static,
        S: Serializer<T> + 'static,
    {
        pub id: ClientId,
        pub model: GenericStringModel<T, S>,
        pub initial_folder_id: Option<SyncId>,
        pub entrypoint: CloudObjectEventEntrypoint,
    }

    pub struct UpdateManager;

    impl UpdateManager {
        pub fn new(
            _model_event_sender: Option<std::sync::mpsc::SyncSender<ModelEvent>>,
            _object_client: Arc<dyn ObjectClient>,
            _ctx: &mut ModelContext<Self>,
        ) -> Self {
            Self
        }

        pub fn mock(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn initial_load_complete(&self) -> impl std::future::Future<Output = ()> {
            async {}
        }

        pub fn mock_initial_load(
            &mut self,
            _response: InitialLoadResponse,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn fetch_single_cloud_object<T>(
            &mut self,
            _id: T,
            _option: FetchSingleObjectOption,
            _ctx: &mut ModelContext<Self>,
        ) -> std::future::Ready<()> {
            std::future::ready(())
        }

        pub fn create_workflow<T, U, V, W>(
            &mut self,
            _workflow: T,
            _owner: U,
            _initial_folder_id: V,
            _ctx: W,
        ) {
        }

        pub fn update_workflow<T, U, V, W>(
            &mut self,
            _workflow: T,
            _workflow_id: U,
            _revision_ts: V,
            _ctx: W,
        ) {
        }

        pub fn create_env_var_collection<T, U, V, W, X, Y, Z>(
            &mut self,
            _client_id: T,
            _owner: U,
            _initial_folder_id: V,
            _model: W,
            _entrypoint: X,
            _show_toast: Y,
            _ctx: Z,
        ) {
        }

        pub fn create_notebook<T, U, V, W, X, Y, Z>(
            &mut self,
            _client_id: T,
            _owner: U,
            _initial_folder_id: V,
            _model: W,
            _entrypoint: X,
            _show_toast: Y,
            _ctx: Z,
        ) {
        }

        pub fn update_notebook_data<T, U, V>(&mut self, _content: T, _id: U, _ctx: V) {}

        pub fn update_notebook_title<T, U, V>(&mut self, _title: T, _id: U, _ctx: V) {}

        pub fn give_up_notebook_edit_access<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn grab_notebook_edit_access<T, U, V>(
            &mut self,
            _id: T,
            _optimistically_grant_access: U,
            _ctx: V,
        ) {
        }

        pub fn replace_object_with_conflict<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn resync_object<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn bulk_create_generic_string_objects<T, U, V>(
            &mut self,
            _owner: T,
            _inputs: U,
            _ctx: V,
        ) {
        }

        pub fn update_env_var_collection<T, U, V, W>(
            &mut self,
            _collection: T,
            _id: U,
            _revision_ts: V,
            _ctx: W,
        ) {
        }

        pub fn duplicate_object<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn trash_object<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn untrash_object<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn delete_object<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn delete_object_with_initiated_by<T, U, V>(
            &mut self,
            _id: T,
            _initiated_by: U,
            _ctx: V,
        ) {
        }

        pub fn record_object_action<T, U, W>(
            &mut self,
            _id: T,
            _action_type: U,
            _data: Option<String>,
            _ctx: W,
        ) {
        }

        pub fn update_object<T, U, V, W>(&mut self, _model: T, _id: U, _revision_ts: V, _ctx: W) {}

        pub fn create_generic_string_object<T, U, V>(&mut self, _input: T, _owner: U, _ctx: V) {}

        pub fn move_object_to_folder<T, U, V>(&mut self, _id: T, _folder_id: U, _ctx: V) {}

        pub fn move_object_to_drive<T, U, V>(&mut self, _id: T, _owner: U, _ctx: V) {}

        pub fn take_edit_access<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn update_permissions<T, U, V>(&mut self, _id: T, _permissions: U, _ctx: V) {}

        pub fn empty_trash<T>(&mut self, _ctx: T) {}

        pub fn leave_object<T, U>(&mut self, _id: T, _ctx: U) {}

        pub fn remove_team_objects<T, U: ?Sized>(&mut self, _team_id: T, _ctx: &mut U) {}

        pub fn refresh_updated_objects<T: ?Sized>(&mut self, _ctx: &mut T) {}
    }

    impl Entity for UpdateManager {
        type Event = UpdateManagerEvent;
    }

    impl SingletonEntity for UpdateManager {}

    pub fn get_duplicate_object_name(original_name: &str) -> String {
        format!("{original_name} (1)")
    }

    pub fn success_result(operation: ObjectOperation) -> ObjectOperationResult {
        ObjectOperationResult {
            success_type: OperationSuccessType::Success,
            operation,
            client_id: None,
            server_id: None,
            num_objects: None,
        }
    }

    pub fn disabled_result(operation: ObjectOperation) -> ObjectOperationResult {
        ObjectOperationResult {
            success_type: OperationSuccessType::FeatureNotAvailable,
            operation,
            client_id: None,
            server_id: None,
            num_objects: None,
        }
    }

    pub fn cloud_disabled_operation(_id: CloudObjectTypeAndId) -> ObjectOperation {
        ObjectOperation::Update
    }
}
