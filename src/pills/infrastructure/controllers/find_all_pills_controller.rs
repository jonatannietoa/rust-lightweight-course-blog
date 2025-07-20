use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;

use crate::pills::application::query::{FindAllPillsQuery, FindAllPillsQueryHandler};

pub async fn find_all_pills_handler(
    State(handler): State<Arc<FindAllPillsQueryHandler>>,
) -> impl IntoResponse {
    let query = FindAllPillsQuery;

    match handler.handle(query).await {
        Ok(pills) => (StatusCode::OK, Json(pills)).into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
