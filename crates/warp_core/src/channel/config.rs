use std::borrow::Cow;

use serde::{Deserialize, Serialize};

use crate::AppId;

#[derive(Debug, Deserialize, Serialize)]
pub struct ChannelConfig {
    /// The application ID for this channel.
    pub app_id: AppId,

    /// The name of the file to which logs should be written.
    pub logfile_name: Cow<'static, str>,

    /// Local-only URL configuration. These addresses are intentionally inert.
    pub server_config: WarpServerConfig,
    /// Local-only placeholder for legacy channel shape.
    pub oz_config: OzConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WarpServerConfig {
    /// Inert local root URL.
    pub server_root_url: Cow<'static, str>,
    /// Inert local websocket URL.
    pub rtc_server_url: Cow<'static, str>,
    /// Always [`None`] for Warp Lite.
    pub session_sharing_server_url: Option<Cow<'static, str>>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct RudderStackDestination {
    pub write_key: Cow<'static, str>,
    pub root_url: Cow<'static, str>,
}

impl RudderStackDestination {
    pub fn local_only() -> Self {
        Self {
            write_key: "".into(),
            root_url: "http://127.0.0.1:0".into(),
        }
    }
}

impl WarpServerConfig {
    pub fn production() -> Self {
        Self::local_only()
    }

    pub fn local_only() -> Self {
        Self {
            server_root_url: "http://127.0.0.1:0".into(),
            rtc_server_url: "ws://127.0.0.1:0/graphql/v2".into(),
            session_sharing_server_url: None,
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct OzConfig {
    /// Inert local root URL.
    pub oz_root_url: Cow<'static, str>,

    /// Always [`None`] for Warp Lite.
    pub workload_audience_url: Option<Cow<'static, str>>,
}

impl OzConfig {
    pub fn production() -> Self {
        Self::local_only()
    }

    pub fn local_only() -> Self {
        Self {
            oz_root_url: "http://127.0.0.1:0".into(),
            workload_audience_url: None,
        }
    }
}
