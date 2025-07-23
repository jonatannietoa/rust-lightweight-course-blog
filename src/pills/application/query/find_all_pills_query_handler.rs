use crate::pills::domain::{Pill, PillRepository, RepositoryError};
use std::sync::Arc;

pub struct FindAllPillsQuery;

pub struct FindAllPillsQueryHandler {
    repository: Arc<dyn PillRepository>,
}

impl FindAllPillsQueryHandler {
    pub fn new(repository: Arc<dyn PillRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: FindAllPillsQuery) -> Result<Vec<Pill>, RepositoryError> {
        let pills = self.repository.find_all().await?;

        Ok(pills)
    }
}
