pub mod course;
pub mod course_repository;

pub use course::Course;
pub use course::CourseId;
pub use course_repository::{CourseRepository, CourseRepositoryError};
