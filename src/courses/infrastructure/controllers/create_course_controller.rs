use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

use crate::courses::domain::{CourseId, CourseRepositoryError};
use crate::courses::{
    application::{CreateCourseCommand, CreateCourseCommandHandler},
    domain::course::Difficulty,
};
use crate::pills::domain::PillId;

#[derive(Deserialize)]
pub struct CreateCourseRequest {
    title: String,
    description: String,
    instructor: String,
    pill_ids: Option<Vec<PillId>>,
    difficulty: Difficulty,
    hours: i8,
    tags: Vec<String>,
    price: f32,
}

#[derive(Serialize)]
pub struct CreateCourseResponse {
    id: CourseId,
    message: String,
}

pub async fn create_course_handler(
    State(handler): State<Arc<CreateCourseCommandHandler>>,
    Json(payload): Json<CreateCourseRequest>,
) -> impl IntoResponse {
    let command = CreateCourseCommand::with_pills(
        payload.title.clone(),
        payload.description,
        payload.instructor,
        payload.pill_ids.unwrap_or_default(),
        payload.difficulty,
        payload.hours,
        payload.tags,
        payload.price,
    );

    match handler.handle(command).await {
        Ok(course_id) => {
            let response = CreateCourseResponse {
                id: course_id,
                message: "Course created successfully".to_string(),
            };
            (StatusCode::CREATED, Json(response)).into_response()
        }
        Err(CourseRepositoryError::DuplicateTitle) => (
            StatusCode::CONFLICT,
            Json(serde_json::json!({
                "error": "Course with this title already exists"
            })),
        )
            .into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
            "error": format!("Internal server error: {}", e),
            })),
        )
            .into_response(),
    }
}
