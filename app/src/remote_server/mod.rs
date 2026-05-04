pub mod setup {
    #[derive(Clone, Debug)]
    pub enum RemoteServerSetupState {
        Checking,
        Installing { progress_percent: Option<u8> },
        Initializing,
        Ready,
        Failed { error: String },
    }

    impl RemoteServerSetupState {
        pub fn is_in_progress(&self) -> bool {
            matches!(
                self,
                Self::Checking | Self::Installing { .. } | Self::Initializing
            )
        }

        pub fn is_connecting(&self) -> bool {
            matches!(self, Self::Checking | Self::Initializing)
        }
    }
}

pub mod client {
    #[derive(Clone, Debug)]
    pub struct RemoteServerClient;
}

pub mod manager {
    use serde::Serialize;
    use std::collections::HashSet;
    use std::sync::Arc;
    use warp_core::{HostId, SessionId};
    use warp_util::standardized_path::StandardizedPath;
    use warpui::{Entity, ModelContext, SingletonEntity};

    use super::client::RemoteServerClient;
    use super::setup::RemoteServerSetupState;

    #[derive(Clone, Copy, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RemoteServerInitPhase {
        Connect,
        Initialize,
    }

    #[derive(Clone, Copy, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RemoteServerOperation {
        NavigateToDirectory,
        LoadRepoMetadataDirectory,
    }

    #[derive(Clone, Copy, Debug, Serialize)]
    #[serde(rename_all = "snake_case")]
    pub enum RemoteServerErrorKind {
        Timeout,
        Disconnected,
        ServerError,
        Other,
    }

    #[derive(Clone, Debug, Serialize)]
    pub struct RemoteServerExitStatus {
        pub code: Option<i32>,
        pub signal_killed: bool,
    }

    #[derive(Clone, Debug)]
    pub enum RemoteOs {
        Linux,
        MacOs,
    }

    impl RemoteOs {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::Linux => "linux",
                Self::MacOs => "macos",
            }
        }
    }

    #[derive(Clone, Debug)]
    pub enum RemoteArch {
        X86_64,
        Aarch64,
    }

    impl RemoteArch {
        pub fn as_str(&self) -> &'static str {
            match self {
                Self::X86_64 => "x86_64",
                Self::Aarch64 => "aarch64",
            }
        }
    }

    #[derive(Clone, Debug)]
    pub struct RemotePlatform {
        pub os: RemoteOs,
        pub arch: RemoteArch,
    }

    #[derive(Clone, Debug)]
    pub enum RemoteServerManagerEvent {
        SessionConnecting {
            session_id: SessionId,
        },
        SessionConnected {
            session_id: SessionId,
            host_id: HostId,
        },
        SessionConnectionFailed {
            session_id: SessionId,
            phase: RemoteServerInitPhase,
            error: String,
        },
        SessionDisconnected {
            session_id: SessionId,
            host_id: HostId,
            exit_status: Option<RemoteServerExitStatus>,
        },
        SessionReconnected {
            session_id: SessionId,
            host_id: HostId,
            attempt: u32,
            client: Arc<RemoteServerClient>,
        },
        SessionDeregistered {
            session_id: SessionId,
        },
        HostConnected {
            host_id: HostId,
        },
        HostDisconnected {
            host_id: HostId,
        },
        NavigatedToDirectory {
            session_id: SessionId,
            host_id: HostId,
            indexed_path: StandardizedPath,
            is_git: bool,
        },
        SetupStateChanged {
            session_id: SessionId,
            state: RemoteServerSetupState,
        },
        BinaryCheckComplete {
            session_id: SessionId,
            result: Result<bool, String>,
            remote_platform: Option<RemotePlatform>,
        },
        BinaryInstallComplete {
            session_id: SessionId,
            result: Result<(), String>,
        },
        ClientRequestFailed {
            session_id: SessionId,
            operation: RemoteServerOperation,
            error_kind: RemoteServerErrorKind,
        },
        ServerMessageDecodingError {
            session_id: SessionId,
        },
        RepoMetadataSnapshot {
            host_id: HostId,
            update: repo_metadata::RepoMetadataUpdate,
        },
        RepoMetadataUpdated {
            host_id: HostId,
            update: repo_metadata::RepoMetadataUpdate,
        },
        RepoMetadataDirectoryLoaded {
            host_id: HostId,
            update: repo_metadata::RepoMetadataUpdate,
        },
    }

    pub struct RemoteServerManager;

    impl Entity for RemoteServerManager {
        type Event = RemoteServerManagerEvent;
    }

    impl SingletonEntity for RemoteServerManager {}

    impl RemoteServerManager {
        pub fn new(_ctx: &mut ModelContext<Self>) -> Self {
            Self
        }

        pub fn session(&self, _session_id: SessionId) -> Option<()> {
            None
        }

        pub fn sessions_for_host(&self, _host_id: &HostId) -> Option<&HashSet<SessionId>> {
            None
        }

        pub fn client_for_host(&self, _host_id: &HostId) -> Option<&Arc<RemoteServerClient>> {
            None
        }

        pub fn client_for_session(
            &self,
            _session_id: SessionId,
        ) -> Option<&Arc<RemoteServerClient>> {
            None
        }

        pub fn host_id_for_session(&self, _session_id: SessionId) -> Option<&HostId> {
            None
        }

        pub fn platform_for_session(&self, _session_id: SessionId) -> Option<&RemotePlatform> {
            None
        }

        pub fn rotate_auth_token(&mut self, _token: String) {}

        pub fn deregister_session(
            &mut self,
            _session_id: SessionId,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn navigate_to_directory(
            &mut self,
            _session_id: SessionId,
            _path: String,
            _ctx: &mut ModelContext<Self>,
        ) {
        }

        pub fn notify_session_bootstrapped(
            &mut self,
            _session_id: SessionId,
            _shell_type_name: &str,
            _shell_path: Option<&str>,
        ) {
        }

        pub fn load_remote_repo_metadata_directory(
            &mut self,
            _session_id: SessionId,
            _repo_root: String,
            _dir_path: String,
            _ctx: &mut ModelContext<Self>,
        ) {
        }
    }
}

pub fn run_proxy(_identity_key: String) -> anyhow::Result<()> {
    anyhow::bail!("remote server proxy was removed in Warp Lite")
}

pub fn run_daemon(_identity_key: String) -> anyhow::Result<()> {
    anyhow::bail!("remote server daemon was removed in Warp Lite")
}
