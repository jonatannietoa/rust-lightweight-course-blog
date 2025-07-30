use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use crate::pills::application::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};

pub struct MongoDbPillRepository {
    collection: Collection<Pill>,
}

impl MongoDbPillRepository {
    pub fn new(database: &Database) -> Self {
        let collection = database.collection::<Pill>("pills");
        Self { collection }
    }
}

#[async_trait]
impl PillRepository for MongoDbPillRepository {
    async fn save(&self, pill: &Pill) -> Result<(), RepositoryError> {
        let filter = doc! { "_id": pill.id().to_string() };
        let update = doc! {
            "$set": mongodb::bson::to_document(pill)
                .map_err(|e| {
                    eprintln!("Repository: Failed to serialize pill {}: {}", pill.id(), e);
                    RepositoryError::Unexpected
                })?
        };

        let result = self
            .collection
            .update_one(
                filter,
                update,
                mongodb::options::UpdateOptions::builder()
                    .upsert(true)
                    .build(),
            )
            .await
            .map_err(|e| {
                eprintln!("Repository: Failed to save pill {}: {}", pill.id(), e);
                RepositoryError::Unexpected
            })?;

        println!(
            "Repository: Pill {} saved successfully. Modified: {}, Matched: {}",
            pill.id(),
            result.modified_count,
            result.matched_count
        );

        Ok(())
    }

    async fn find_by_id(&self, id: PillId) -> Result<Option<Pill>, RepositoryError> {
        let filter = doc! { "_id": id.to_string() };

        let pill = self.collection.find_one(filter, None).await.map_err(|e| {
            eprintln!("Repository: Failed to find pill {}: {}", id, e);
            RepositoryError::Unexpected
        })?;

        match &pill {
            Some(_) => println!("Repository: Found pill {}", id),
            None => println!("Repository: Pill {} not found", id),
        }

        Ok(pill)
    }

    async fn find_all(&self) -> Result<Vec<Pill>, RepositoryError> {
        let cursor = self.collection.find(doc! {}, None).await.map_err(|e| {
            eprintln!(
                "Repository: Failed to create cursor for find_all pills: {}",
                e
            );
            RepositoryError::Unexpected
        })?;

        let pills: Vec<Pill> = cursor.try_collect().await.map_err(|e| {
            eprintln!("Repository: Failed to collect pills from cursor: {}", e);
            RepositoryError::Unexpected
        })?;

        println!("Repository: Found {} pills", pills.len());
        Ok(pills)
    }
}
