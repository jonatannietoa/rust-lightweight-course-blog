use crate::pills::domain::PillId;

#[derive(Debug, Clone)]
pub struct CreateCourseCommand {
    pub title: String,
    pub description: String,
    pub instructor: String,
    pub pill_ids: Vec<PillId>,
}

impl CreateCourseCommand {
    pub fn with_pills(
        title: String,
        description: String,
        instructor: String,
        pill_ids: Vec<PillId>,
    ) -> Self {
        Self {
            title,
            description,
            instructor,
            pill_ids,
        }
    }
}
