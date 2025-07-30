use crate::{courses::domain::course::Difficulty, pills::domain::PillId};

#[derive(Debug, Clone)]
pub struct CreateCourseCommand {
    pub title: String,
    pub description: String,
    pub instructor: String,
    pub pill_ids: Vec<PillId>,
    pub difficulty: Difficulty,
    pub hours: i8,
    pub tags: Vec<String>,
    pub price: f32,
}

impl CreateCourseCommand {
    pub fn with_pills(
        title: String,
        description: String,
        instructor: String,
        pill_ids: Vec<PillId>,
        difficulty: Difficulty,
        hours: i8,
        tags: Vec<String>,
        price: f32,
    ) -> Self {
        Self {
            title,
            description,
            instructor,
            pill_ids,
            difficulty,
            hours,
            tags,
            price,
        }
    }
}
