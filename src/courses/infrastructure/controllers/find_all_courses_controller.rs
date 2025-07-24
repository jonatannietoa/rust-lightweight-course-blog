use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;

use crate::courses::application::{FindAllCoursesQuery, FindAllCoursesQueryHandler};

pub async fn find_all_courses_handler(
    State(handler): State<Arc<FindAllCoursesQueryHandler>>,
) -> impl IntoResponse {
    let query = FindAllCoursesQuery;

    match handler.handle(query).await {
        Ok(courses) => (StatusCode::OK, Json(courses)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Internal server error: {}", e),
            })),
        )
            .into_response(),
    }
}
