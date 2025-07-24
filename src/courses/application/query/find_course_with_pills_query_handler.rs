use crate::courses::domain::{Course, CourseId, CourseRepository, CourseRepositoryError};
use crate::pills::domain::{Pill, PillRepository, RepositoryError};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CourseWithPills {
    pub course: Course,
    pub pills: Vec<Pill>,
}

pub struct FindCourseWithPillsQuery {
    pub id: CourseId,
}

pub struct FindCourseWithPillsQueryHandler {
    course_repository: Arc<dyn CourseRepository>,
    pill_repository: Arc<dyn PillRepository>,
}

impl FindCourseWithPillsQueryHandler {
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
        query: FindCourseWithPillsQuery,
    ) -> Result<CourseWithPills, CourseRepositoryError> {
        let course = self
            .course_repository
            .find_by_id(query.id)
            .await?
            .ok_or(CourseRepositoryError::NotFound)?;

        let mut pills = Vec::new();
        for pill_id in course.pill_ids() {
            match self.pill_repository.find_by_id(*pill_id).await {
                Ok(Some(pill)) => {
                    pills.push(pill);
                }
                Ok(None) => {
                    println!(
                        "Warning: Pill {} referenced by course but not found",
                        pill_id
                    );
                    // Continue processing other pills instead of failing
                }
                Err(RepositoryError::NotFound) => {
                    println!(
                        "Warning: Pill {} referenced by course but not found",
                        pill_id
                    );
                    // Continue processing other pills instead of failing
                }
                Err(_) => {
                    println!("Error fetching pill {}", pill_id);
                    return Err(CourseRepositoryError::Unexpected);
                }
            }
        }

        Ok(CourseWithPills { course, pills })
    }
}
