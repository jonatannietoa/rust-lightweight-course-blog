use async_trait::async_trait;
use thiserror::Error;

use crate::pills::domain::{Pill, PillId};

#[derive(Error, Debug, Clone)]
pub enum RepositoryError {
    #[error("An unexpected error occurred")]
    Unexpected,
    #[error("Pill not found")]
    NotFound,
}

#[async_trait]
pub trait PillRepository: Send + Sync {
    async fn save(&self, pill: &Pill) -> Result<(), RepositoryError>;
    async fn find_by_id(&self, id: PillId) -> Result<Option<Pill>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Pill>, RepositoryError>;
}
