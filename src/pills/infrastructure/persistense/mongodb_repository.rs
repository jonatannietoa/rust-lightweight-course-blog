use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::pills::application::{PillRepository, RepositoryError};
use crate::pills::domain::{Pill, PillId};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PillDocument {
    #[serde(rename = "_id")]
    id: String,
    title: String,
    content: String,
}

impl From<&Pill> for PillDocument {
    fn from(pill: &Pill) -> Self {
        Self {
            id: pill.id().to_string(),
            title: pill.title().to_string(),
            content: pill.content().to_string(),
        }
    }
}

impl TryFrom<PillDocument> for Pill {
    type Error = uuid::Error;

    fn try_from(doc: PillDocument) -> Result<Self, Self::Error> {
        let id = doc.id.parse::<PillId>()?;
        Ok(Pill::create(id, doc.title, doc.content))
    }
}

pub struct MongoDbPillRepository {
    collection: Collection<PillDocument>,
}

impl MongoDbPillRepository {
    pub fn new(database: &Database) -> Self {
        let collection = database.collection::<PillDocument>("pills");
        Self { collection }
    }
}

#[async_trait]
impl PillRepository for MongoDbPillRepository {
    async fn save(&self, pill: &Pill) -> Result<(), RepositoryError> {
        let pill_doc = PillDocument::from(pill);
        let filter = doc! { "_id": pill.id().to_string() };
        let update = doc! {
            "$set": mongodb::bson::to_document(&pill_doc)
                .map_err(|e| {
                    tracing::error!("Repository: Failed to serialize pill {}: {}", pill.id(), e);
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
                tracing::error!("Repository: Failed to save pill {}: {}", pill.id(), e);
                RepositoryError::Unexpected
            })?;

        tracing::info!(
            "Repository: Pill {} saved successfully. Modified: {}, Matched: {}",
            pill.id(),
            result.modified_count,
            result.matched_count
        );

        Ok(())
    }

    async fn find_by_id(&self, id: PillId) -> Result<Option<Pill>, RepositoryError> {
        let filter = doc! { "_id": id.to_string() };

        let pill_doc = self.collection.find_one(filter, None).await.map_err(|e| {
            tracing::error!("Repository: Failed to find pill {}: {}", id, e);
            RepositoryError::Unexpected
        })?;

        let pill = pill_doc
            .map(|doc| doc.try_into())
            .transpose()
            .map_err(|e| {
                tracing::error!("Repository: Failed to deserialize pill {}: {}", id, e);
                RepositoryError::Unexpected
            })?;

        match &pill {
            Some(_) => tracing::debug!("Repository: Found pill {}", id),
            None => tracing::debug!("Repository: Pill {} not found", id),
        }

        Ok(pill)
    }

    async fn find_all(&self) -> Result<Vec<Pill>, RepositoryError> {
        let cursor = self.collection.find(doc! {}, None).await.map_err(|e| {
            tracing::error!(
                "Repository: Failed to create cursor for find_all pills: {}",
                e
            );
            RepositoryError::Unexpected
        })?;

        let pill_docs: Vec<PillDocument> = cursor.try_collect().await.map_err(|e| {
            tracing::error!("Repository: Failed to collect pills from cursor: {}", e);
            RepositoryError::Unexpected
        })?;

        let pills: Result<Vec<Pill>, _> = pill_docs
            .into_iter()
            .map(|doc| doc.try_into())
            .collect();

        let pills = pills.map_err(|e| {
            tracing::error!("Repository: Failed to deserialize pills: {}", e);
            RepositoryError::Unexpected
        })?;

        tracing::info!("Repository: Found {} pills", pills.len());
        Ok(pills)
    }
}
