use crate::courses::domain::CourseId;
use crate::pills::domain::PillId;

#[derive(Debug, Clone)]
pub struct AddPillToCourseCommand {
    pub course_id: CourseId,
    pub pill_id: PillId,
}

impl AddPillToCourseCommand {
    pub fn new(course_id: CourseId, pill_id: PillId) -> Self {
        Self { course_id, pill_id }
    }
}
