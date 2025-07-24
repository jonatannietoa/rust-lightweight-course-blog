use crate::pills::domain::PillId;
use serde::{Deserialize, Serialize};
use std::fmt;
use std::str::FromStr;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct CourseId(Uuid);

impl CourseId {
    pub fn new() -> Self {
        CourseId(Uuid::new_v4())
    }
}

impl FromStr for CourseId {
    type Err = uuid::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let uuid = Uuid::parse_str(s)?;
        Ok(CourseId(uuid))
    }
}

impl Default for CourseId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CourseId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    id: CourseId,
    title: String,
    description: String,
    instructor: String,
    pill_ids: Vec<PillId>,
}

impl Course {
    pub fn create(id: CourseId, title: String, description: String, instructor: String) -> Self {
        let course = Self {
            id,
            title: title.clone(),
            description: description.clone(),
            instructor: instructor.clone(),
            pill_ids: Vec::new(),
        };

        course
    }

    pub fn with_pills(
        id: CourseId,
        title: String,
        description: String,
        instructor: String,
        pill_ids: Vec<PillId>,
    ) -> Self {
        let course = Self {
            id,
            title: title.clone(),
            description: description.clone(),
            instructor: instructor.clone(),
            pill_ids: pill_ids.clone(),
        };

        course
    }

    pub fn id(&self) -> CourseId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn instructor(&self) -> &str {
        &self.instructor
    }

    pub fn pill_ids(&self) -> &[PillId] {
        &self.pill_ids
    }

    pub fn add_pill(&mut self, pill_id: PillId) {
        if !self.pill_ids.contains(&pill_id) {
            self.pill_ids.push(pill_id);
        }
    }

    pub fn pill_count(&self) -> usize {
        self.pill_ids.len()
    }
}
