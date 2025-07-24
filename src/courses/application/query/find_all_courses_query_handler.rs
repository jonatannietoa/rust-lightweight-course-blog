use crate::courses::domain::{Course, CourseRepository, CourseRepositoryError};
use std::sync::Arc;

pub struct FindAllCoursesQuery;

pub struct FindAllCoursesQueryHandler {
    repository: Arc<dyn CourseRepository>,
}

impl FindAllCoursesQueryHandler {
    pub fn new(repository: Arc<dyn CourseRepository>) -> Self {
        Self { repository }
    }

    pub async fn handle(
        &self,
        _query: FindAllCoursesQuery,
    ) -> Result<Vec<Course>, CourseRepositoryError> {
        let courses = self.repository.find_all().await?;

        Ok(courses)
    }
}
