use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PillId(Uuid);

impl PillId {
    pub fn new() -> Self {
        PillId(Uuid::new_v4())
    }
}

impl FromStr for PillId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(PillId(uuid))
    }
}

impl Default for PillId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for PillId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Pill {
    id: PillId,
    title: String,
    content: String,
}

impl Pill {
    pub fn create(id: PillId, title: String, content: String) -> Self {
        Self { id, title, content }
    }

    pub fn id(&self) -> PillId {
        self.id
    }
}
