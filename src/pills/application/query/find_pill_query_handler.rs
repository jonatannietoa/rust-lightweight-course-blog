use crate::pills::domain::{Pill, PillId, PillRepository, RepositoryError};
use std::sync::Arc;

pub struct FindPillQuery {
    pub id: PillId,
}

pub struct FindPillQueryHandler {
    repository: Arc<dyn PillRepository>,
}

impl FindPillQueryHandler {
    pub fn new(repository: Arc<dyn PillRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: FindPillQuery) -> Result<Pill, RepositoryError> {
        println!("Handler (Find): Searching pill with ID {}", query.id);
        let pill = self
            .repository
            .find_by_id(query.id)
            .await?
            .ok_or(RepositoryError::NotFound)?;

        println!(
            "Found pill '{}' with content: {}",
            pill.title(),
            pill.content()
        );

        Ok(pill)
    }
}
