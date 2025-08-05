use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};
use serde::{Deserialize, Serialize};

use crate::courses::domain::{Course, CourseId, CourseRepository, CourseRepositoryError};
use crate::courses::domain::course::Difficulty;
use crate::pills::domain::PillId;

#[derive(Debug, Clone, Serialize, Deserialize)]
struct CourseDocument {
    #[serde(rename = "_id")]
    id: String,
    title: String,
    description: String,
    instructor: String,
    difficulty: Difficulty,
    hours: i8,
    tags: Vec<String>,
    price: f32,
    pill_ids: Vec<String>,
}

impl From<&Course> for CourseDocument {
    fn from(course: &Course) -> Self {
        Self {
            id: course.id().to_string(),
            title: course.title().to_string(),
            description: course.description().to_string(),
            instructor: course.instructor().to_string(),
            difficulty: course.difficulty(),
            hours: course.hours(),
            tags: course.tags().to_vec(),
            price: course.price(),
            pill_ids: course.pill_ids().iter().map(|id| id.to_string()).collect(),
        }
    }
}

impl TryFrom<CourseDocument> for Course {
    type Error = uuid::Error;

    fn try_from(doc: CourseDocument) -> Result<Self, Self::Error> {
        let id = doc.id.parse::<CourseId>()?;
        let pill_ids: Result<Vec<PillId>, _> = doc.pill_ids
            .iter()
            .map(|s| s.parse::<PillId>())
            .collect();
        let pill_ids = pill_ids?;

        Ok(Course::create(
            id,
            doc.title,
            doc.description,
            doc.instructor,
            doc.difficulty,
            doc.hours,
            doc.tags,
            doc.price,
            pill_ids,
        ))
    }
}

pub struct MongoDbCourseRepository {
    collection: Collection<CourseDocument>,
}

impl MongoDbCourseRepository {
    pub fn new(database: &Database) -> Self {
        let collection = database.collection::<CourseDocument>("courses");
        Self { collection }
    }
}

#[async_trait]
impl CourseRepository for MongoDbCourseRepository {
    async fn save(&self, course: &Course) -> Result<(), CourseRepositoryError> {
        let course_doc = CourseDocument::from(course);
        let filter = doc! { "_id": course.id().to_string() };
        let update = doc! {
            "$set": mongodb::bson::to_document(&course_doc)
                .map_err(|e| {
                    tracing::error!("Repository: Failed to serialize course {}: {}", course.id(), e);
                    CourseRepositoryError::Unexpected
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
                tracing::error!("Repository: Failed to save course {}: {}", course.id(), e);
                CourseRepositoryError::Unexpected
            })?;

        tracing::info!(
            "Repository: Course {} saved successfully. Modified: {}, Matched: {}",
            course.id(),
            result.modified_count,
            result.matched_count
        );

        Ok(())
    }

    async fn find_by_id(&self, id: CourseId) -> Result<Option<Course>, CourseRepositoryError> {
        let filter = doc! { "_id": id.to_string() };

        let course_doc = self.collection.find_one(filter, None).await.map_err(|e| {
            tracing::error!("Repository: Failed to find course {}: {}", id, e);
            CourseRepositoryError::Unexpected
        })?;

        let course = course_doc
            .map(|doc| doc.try_into())
            .transpose()
            .map_err(|e| {
                tracing::error!("Repository: Failed to deserialize course {}: {}", id, e);
                CourseRepositoryError::Unexpected
            })?;

        match &course {
            Some(_) => tracing::debug!("Repository: Found course {}", id),
            None => tracing::debug!("Repository: Course {} not found", id),
        }

        Ok(course)
    }

    async fn find_all(&self) -> Result<Vec<Course>, CourseRepositoryError> {
        let cursor = self.collection.find(doc! {}, None).await.map_err(|e| {
            tracing::error!(
                "Repository: Failed to create cursor for find_all courses: {}",
                e
            );
            CourseRepositoryError::Unexpected
        })?;

        let course_docs: Vec<CourseDocument> = cursor.try_collect().await.map_err(|e| {
            tracing::error!("Repository: Failed to collect courses from cursor: {}", e);
            CourseRepositoryError::Unexpected
        })?;

        let courses: Result<Vec<Course>, _> = course_docs
            .into_iter()
            .map(|doc| doc.try_into())
            .collect();

        let courses = courses.map_err(|e| {
            tracing::error!("Repository: Failed to deserialize courses: {}", e);
            CourseRepositoryError::Unexpected
        })?;

        tracing::info!("Repository: Found {} courses", courses.len());
        Ok(courses)
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<Course>, CourseRepositoryError> {
        let filter = doc! { "title": title };

        let course_doc = self.collection.find_one(filter, None).await.map_err(|e| {
            tracing::error!(
                "Repository: Failed to find course by title '{}': {}",
                title,
                e
            );
            CourseRepositoryError::Unexpected
        })?;

        let course: Option<Course> = course_doc
            .map(|doc| doc.try_into())
            .transpose()
            .map_err(|e| {
                tracing::error!("Repository: Failed to deserialize course by title '{}': {}", title, e);
                CourseRepositoryError::Unexpected
            })?;

        match &course {
            Some(c) => tracing::debug!("Repository: Found course '{}' by title", c.title()),
            None => tracing::debug!("Repository: Course with title '{}' not found", title),
        }

        Ok(course)
    }
}
