use std::{
    cmp::Ordering,
    fmt::{Display, Formatter, Result},
};

use uuid::Uuid;

use crate::server::ids::ObjectUid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ServerCardItemId {
    TemplatableMCP(Uuid),
    TemplatableMCPInstallation(Uuid),
    GalleryMCP(Uuid),
    FileBasedMCP(Uuid),
}

impl Ord for ServerCardItemId {
    fn cmp(&self, other: &Self) -> Ordering {
        self.uid().cmp(&other.uid())
    }
}

impl PartialOrd for ServerCardItemId {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for ServerCardItemId {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.write_str(&self.uid())
    }
}

impl ServerCardItemId {
    pub fn uid(&self) -> ObjectUid {
        match self {
            ServerCardItemId::TemplatableMCP(uuid)
            | ServerCardItemId::TemplatableMCPInstallation(uuid)
            | ServerCardItemId::GalleryMCP(uuid)
            | ServerCardItemId::FileBasedMCP(uuid) => uuid.to_string(),
        }
    }
}
