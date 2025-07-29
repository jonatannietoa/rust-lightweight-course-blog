use std::sync::Arc;

use super::add_pill_to_course_command::AddPillToCourseCommand;
use crate::courses::domain::course_repository::{CourseRepository, CourseRepositoryError};
use crate::pills::domain::pills_repository::PillRepository;

pub struct AddPillToCourseCommandHandler {
    course_repository: Arc<dyn CourseRepository>,
    pill_repository: Arc<dyn PillRepository>,
}

impl AddPillToCourseCommandHandler {
    pub fn new(
        course_repository: Arc<dyn CourseRepository>,
        pill_repository: Arc<dyn PillRepository>,
    ) -> Self {
        Self {
            course_repository,
            pill_repository,
        }
    }

    pub async fn handle(
        &self,
        command: AddPillToCourseCommand,
    ) -> Result<(), CourseRepositoryError> {
        match self.pill_repository.find_by_id(command.pill_id).await {
            Ok(Some(_)) => {
                println!(
                    "Handler (AddPillToCourse): Pill {} exists, adding to course {}",
                    command.pill_id, command.course_id
                );
            }
            Ok(None) => {
                return Err(CourseRepositoryError::NotFound);
            }
            Err(_) => {
                return Err(CourseRepositoryError::Unexpected);
            }
        }

        let mut course = self
            .course_repository
            .find_by_id(command.course_id)
            .await?
            .ok_or(CourseRepositoryError::NotFound)?;

        course.add_pill(command.pill_id);

        self.course_repository.save(&course).await?;

        Ok(())
    }
}
