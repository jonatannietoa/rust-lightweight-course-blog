use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;

use crate::courses::application::{CourseRepositoryError, FindCourseQuery, FindCourseQueryHandler};
use crate::courses::domain::CourseId;

pub async fn find_course_by_id_controller(
    State(handler): State<Arc<FindCourseQueryHandler>>,
    Path(id): Path<CourseId>,
) -> impl IntoResponse {
    let query = FindCourseQuery { id };

    match handler.handle(query).await {
        Ok(course) => (StatusCode::OK, Json(course)).into_response(),
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
