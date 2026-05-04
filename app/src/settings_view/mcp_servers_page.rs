use super::mcp_servers::ServerCardItemId;

#[derive(Debug, Default, Copy, Clone)]
pub enum MCPServersSettingsPage {
    #[default]
    List,
    Edit {
        item_id: Option<ServerCardItemId>,
    },
}
