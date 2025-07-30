use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;

use crate::courses::application::{
    CourseRepositoryError, FindCourseWithPillsQuery, FindCourseWithPillsQueryHandler,
};
use crate::courses::domain::CourseId;

pub async fn find_course_with_pills_controller(
    State(handler): State<Arc<FindCourseWithPillsQueryHandler>>,
    Path(id): Path<CourseId>,
) -> impl IntoResponse {
    let query = FindCourseWithPillsQuery { id };

    match handler.handle(query).await {
        Ok(course_with_pills) => (StatusCode::OK, Json(course_with_pills)).into_response(),
        Err(CourseRepositoryError::NotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({
                "error": "Course not found"
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
