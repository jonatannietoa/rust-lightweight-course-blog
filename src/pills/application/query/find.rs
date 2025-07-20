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
        println!("Handler (Find): Buscando píldora con ID {}", query.id);
        self.repository
            .find_by_id(query.id)
            .await?
            .ok_or(RepositoryError::NotFound)
    }
}

pub struct FindAllPillsQuery;

pub struct FindAllPillsQueryHandler {
    repository: Arc<dyn PillRepository>,
}

impl FindAllPillsQueryHandler {
    pub fn new(repository: Arc<dyn PillRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, _query: FindAllPillsQuery) -> Result<Vec<Pill>, RepositoryError> {
        println!("Handler (FindAll): Buscando todas las píldoras");
        self.repository.find_all().await
    }
}
