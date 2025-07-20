use std::sync::Arc;

use super::command::CreatePillCommand;
use super::repository::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};

pub struct CreatePillCommandHandler {
    repository: Arc<dyn PillRepository>,
}

impl CreatePillCommandHandler {
    pub fn new(repository: Arc<dyn PillRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, command: CreatePillCommand) -> Result<(), RepositoryError> {
        let pill_id = PillId::new();
        let pill = Pill::new(pill_id, command.title, command.content);

        println!("Handler (Create): Guardando píldora con ID {}", pill.id());

        self.repository.save(&pill).await
    }
}
