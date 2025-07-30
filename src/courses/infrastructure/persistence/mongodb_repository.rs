use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::{bson::doc, Collection, Database};

use crate::courses::domain::{Course, CourseId, CourseRepository, CourseRepositoryError};

pub struct MongoDbCourseRepository {
    collection: Collection<Course>,
}

impl MongoDbCourseRepository {
    pub fn new(database: &Database) -> Self {
        let collection = database.collection::<Course>("courses");
        Self { collection }
    }
}

#[async_trait]
impl CourseRepository for MongoDbCourseRepository {
    async fn save(&self, course: &Course) -> Result<(), CourseRepositoryError> {
        let filter = doc! { "_id": course.id().to_string() };
        let update = doc! {
            "$set": mongodb::bson::to_document(course)
                .map_err(|e| {
                    eprintln!("Repository: Failed to serialize course {}: {}", course.id(), e);
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
                eprintln!("Repository: Failed to save course {}: {}", course.id(), e);
                CourseRepositoryError::Unexpected
            })?;

        println!(
            "Repository: Course {} saved successfully. Modified: {}, Matched: {}",
            course.id(),
            result.modified_count,
            result.matched_count
        );

        Ok(())
    }

    async fn find_by_id(&self, id: CourseId) -> Result<Option<Course>, CourseRepositoryError> {
        let filter = doc! { "_id": id.to_string() };

        let course = self.collection.find_one(filter, None).await.map_err(|e| {
            eprintln!("Repository: Failed to find course {}: {}", id, e);
            CourseRepositoryError::Unexpected
        })?;

        match &course {
            Some(_) => println!("Repository: Found course {}", id),
            None => println!("Repository: Course {} not found", id),
        }

        Ok(course)
    }

    async fn find_all(&self) -> Result<Vec<Course>, CourseRepositoryError> {
        let cursor = self.collection.find(doc! {}, None).await.map_err(|e| {
            eprintln!(
                "Repository: Failed to create cursor for find_all courses: {}",
                e
            );
            CourseRepositoryError::Unexpected
        })?;

        let courses: Vec<Course> = cursor.try_collect().await.map_err(|e| {
            eprintln!("Repository: Failed to collect courses from cursor: {}", e);
            CourseRepositoryError::Unexpected
        })?;

        println!("Repository: Found {} courses", courses.len());
        Ok(courses)
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<Course>, CourseRepositoryError> {
        let filter = doc! { "title": title };

        let course = self.collection.find_one(filter, None).await.map_err(|e| {
            eprintln!(
                "Repository: Failed to find course by title '{}': {}",
                title, e
            );
            CourseRepositoryError::Unexpected
        })?;

        match &course {
            Some(c) => println!("Repository: Found course '{}' by title", c.title()),
            None => println!("Repository: Course with title '{}' not found", title),
        }

        Ok(course)
    }
}
