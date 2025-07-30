use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::courses::application::{AddPillToCourseCommand, AddPillToCourseCommandHandler};
use crate::courses::domain::{CourseId, CourseRepositoryError};
use crate::pills::domain::PillId;

#[derive(Deserialize)]
pub struct AddPillToCourseRequest {
    pill_id: PillId,
}

pub async fn add_pill_to_course_controller(
    State(handler): State<Arc<AddPillToCourseCommandHandler>>,
    Path(course_id): Path<CourseId>,
    Json(payload): Json<AddPillToCourseRequest>,
) -> impl IntoResponse {
    let command = AddPillToCourseCommand::new(course_id, payload.pill_id);

    match handler.handle(command).await {
        Ok(_) => (
            StatusCode::OK,
            Json(serde_json::json!({
                "message": "Pill added to course successfully"
            })),
        )
            .into_response(),
        Err(CourseRepositoryError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Course or pill not found"
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
