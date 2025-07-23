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
        println!("Handler (FindAll): Searching all pills");
        let pills = self.repository.find_all().await?;

        for pill in &pills {
            println!(
                "Pill found: ID={}, Title='{}', Content='{}'",
                pill.id(),
                pill.title(),
                pill.content()
            );
        }

        Ok(pills)
    }
}
