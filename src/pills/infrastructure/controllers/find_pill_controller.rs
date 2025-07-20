use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Json},
};
use std::sync::Arc;

use crate::pills::application::{FindPillQuery, FindPillQueryHandler, RepositoryError};
use crate::pills::domain::PillId;

pub async fn find_pill_by_id_handler(
    State(handler): State<Arc<FindPillQueryHandler>>,
    Path(id): Path<PillId>,
) -> impl IntoResponse {
    let query = FindPillQuery { id };

    match handler.handle(query).await {
        Ok(pill) => (StatusCode::OK, Json(pill)).into_response(),
        Err(RepositoryError::NotFound) => StatusCode::NOT_FOUND.into_response(),
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
    }
}
