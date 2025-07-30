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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    Beginner,
    Intermediate,
    Advanced,
    Expert,
}

impl fmt::Display for Difficulty {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Difficulty::Beginner => write!(f, "Beginner"),
            Difficulty::Intermediate => write!(f, "Intermediate"),
            Difficulty::Advanced => write!(f, "Advanced"),
            Difficulty::Expert => write!(f, "Expert"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Course {
    id: CourseId,
    title: String,
    description: String,
    instructor: String,
    pill_ids: Vec<PillId>,
    difficulty: Difficulty,
    hours: i8,
    tags: Vec<String>,
    price: f32,
}

impl Course {
    pub fn create(
        id: CourseId,
        title: String,
        description: String,
        instructor: String,
        difficulty: Difficulty,
        hours: i8,
        tags: Vec<String>,
        price: f32,
    ) -> Self {
        Self {
            id,
            title,
            description,
            instructor,
            pill_ids: Vec::new(),
            difficulty,
            hours,
            tags,
            price,
        }
    }

    pub fn with_pills(
        id: CourseId,
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
            id,
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

    pub fn id(&self) -> CourseId {
        self.id
    }

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn description(&self) -> &str {
        &self.description
    }

    pub fn instructor(&self) -> &str {
        &self.instructor
    }

    pub fn pill_ids(&self) -> &[PillId] {
        &self.pill_ids
    }

    pub fn difficulty(&self) -> Difficulty {
        self.difficulty
    }

    pub fn hours(&self) -> i8 {
        self.hours
    }

    pub fn tags(&self) -> &[String] {
        &self.tags
    }

    pub fn price(&self) -> f32 {
        self.price
    }

    pub fn add_pill(&mut self, pill_id: PillId) {
        if !self.pill_ids.contains(&pill_id) {
            self.pill_ids.push(pill_id);
        }
    }

    pub fn pill_count(&self) -> usize {
        self.pill_ids.len()
    }

    pub fn add_tag(&mut self, tag: String) {
        if !self.tags.contains(&tag) {
            self.tags.push(tag);
        }
    }

    pub fn remove_tag(&mut self, tag: &str) {
        self.tags.retain(|t| t != tag);
    }

    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.contains(&tag.to_string())
    }

    pub fn update_difficulty(&mut self, difficulty: Difficulty) {
        self.difficulty = difficulty;
    }

    pub fn update_hours(&mut self, hours: i8) {
        self.hours = hours;
    }

    pub fn update_price(&mut self, price: f32) {
        self.price = price;
    }
}
