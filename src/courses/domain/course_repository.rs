use async_trait::async_trait;
use thiserror::Error;

use crate::courses::domain::{Course, CourseId};

#[derive(Error, Debug, Clone)]
pub enum CourseRepositoryError {
    #[error("An unexpected error occurred")]
    Unexpected,
    #[error("Course not found")]
    NotFound,
    #[error("Course with this title already exists")]
    DuplicateTitle,
}

#[async_trait]
pub trait CourseRepository: Send + Sync {
    async fn save(&self, course: &Course) -> Result<(), CourseRepositoryError>;
    async fn find_by_id(&self, id: CourseId) -> Result<Option<Course>, CourseRepositoryError>;
    async fn find_all(&self) -> Result<Vec<Course>, CourseRepositoryError>;
    async fn find_by_title(&self, title: &str) -> Result<Option<Course>, CourseRepositoryError>;
}
