use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::pills::application::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};

pub struct InMemoryPillRepository {
    pills: Mutex<HashMap<PillId, Pill>>,
}

impl InMemoryPillRepository {
    pub fn new() -> Self {
        Self {
            pills: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PillRepository for InMemoryPillRepository {
    async fn save(&self, pill: &Pill) -> Result<(), RepositoryError> {
        let mut pills = self.pills.lock().map_err(|_| RepositoryError::Unexpected)?;
        pills.insert(pill.id(), pill.clone());

        println!(
            "Repositorio: Píldora {} guardada. Píldoras totales: {}",
            pill.id(),
            pills.len()
        );

        Ok(())
    }

    async fn find_by_id(&self, id: PillId) -> Result<Option<Pill>, RepositoryError> {
        let pills = self.pills.lock().map_err(|_| RepositoryError::Unexpected)?;
        Ok(pills.get(&id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Pill>, RepositoryError> {
        let pills = self.pills.lock().map_err(|_| RepositoryError::Unexpected)?;
        Ok(pills.values().cloned().collect())
    }
}
