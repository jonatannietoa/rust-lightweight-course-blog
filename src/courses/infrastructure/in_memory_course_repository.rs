use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Mutex;

use crate::courses::domain::{Course, CourseId, CourseRepository, CourseRepositoryError};

pub struct InMemoryCourseRepository {
    courses: Mutex<HashMap<CourseId, Course>>,
}

impl InMemoryCourseRepository {
    pub fn new() -> Self {
        Self {
            courses: Mutex::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl CourseRepository for InMemoryCourseRepository {
    async fn save(&self, course: &Course) -> Result<(), CourseRepositoryError> {
        let mut courses = self
            .courses
            .lock()
            .map_err(|_| CourseRepositoryError::Unexpected)?;

        courses.insert(course.id(), course.clone());

        Ok(())
    }

    async fn find_by_id(&self, id: CourseId) -> Result<Option<Course>, CourseRepositoryError> {
        let courses = self
            .courses
            .lock()
            .map_err(|_| CourseRepositoryError::Unexpected)?;

        Ok(courses.get(&id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Course>, CourseRepositoryError> {
        let courses = self
            .courses
            .lock()
            .map_err(|_| CourseRepositoryError::Unexpected)?;

        Ok(courses.values().cloned().collect())
    }

    async fn find_by_title(&self, title: &str) -> Result<Option<Course>, CourseRepositoryError> {
        let courses = self
            .courses
            .lock()
            .map_err(|_| CourseRepositoryError::Unexpected)?;

        Ok(courses
            .values()
            .find(|course| course.title() == title)
            .cloned())
    }
}
