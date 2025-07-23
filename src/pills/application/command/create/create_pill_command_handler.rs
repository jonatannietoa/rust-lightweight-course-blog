use std::sync::Arc;

use super::create_pill_command::CreatePillCommand;
use crate::pills::domain::pills_repository::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};

pub struct CreatePillCommandHandler {
    pills_repository: Arc<dyn PillRepository>,
}

impl CreatePillCommandHandler {
    pub fn new(pills_repository: Arc<dyn PillRepository>) -> Self {
        Self { pills_repository }
    }

    pub async fn handle(&self, command: CreatePillCommand) -> Result<(), RepositoryError> {
        let pill_id = PillId::new();
        let pill = Pill::create(pill_id, command.title, command.content);

        println!("Handler (Create): Saving pill with ID {}", pill.id());

        self.pills_repository.save(&pill).await
    }
}
