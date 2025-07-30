use std::sync::Arc;

use super::create_course_command::CreateCourseCommand;
use crate::courses::domain::course_repository::{CourseRepository, CourseRepositoryError};
use crate::courses::domain::{Course, CourseId};

pub struct CreateCourseCommandHandler {
    course_repository: Arc<dyn CourseRepository>,
}

impl CreateCourseCommandHandler {
    pub fn new(course_repository: Arc<dyn CourseRepository>) -> Self {
        Self { course_repository }
    }

    pub async fn handle(
        &self,
        command: CreateCourseCommand,
    ) -> Result<CourseId, CourseRepositoryError> {
        if let Ok(Some(_)) = self.course_repository.find_by_title(&command.title).await {
            return Err(CourseRepositoryError::DuplicateTitle);
        }

        let course_id = CourseId::new();
        let course = Course::create(
            course_id,
            command.title.clone(),
            command.description,
            command.instructor,
            command.difficulty,
            command.hours,
            command.tags,
            command.price,
        );

        self.course_repository.save(&course).await?;

        Ok(course_id)
    }
}
