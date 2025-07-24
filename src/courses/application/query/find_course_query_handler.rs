use crate::courses::domain::{Course, CourseId, CourseRepository, CourseRepositoryError};
use std::sync::Arc;

pub struct FindCourseQuery {
    pub id: CourseId,
}

pub struct FindCourseQueryHandler {
    repository: Arc<dyn CourseRepository>,
}

impl FindCourseQueryHandler {
    pub fn new(repository: Arc<dyn CourseRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(&self, query: FindCourseQuery) -> Result<Course, CourseRepositoryError> {
        let course = self
            .repository
            .find_by_id(query.id)
            .await?
            .ok_or(CourseRepositoryError::NotFound)?;

        Ok(course)
    }
}
