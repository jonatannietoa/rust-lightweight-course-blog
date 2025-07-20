use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use serde::Deserialize;
use std::sync::Arc;

use crate::pills::application::{CreatePillCommand, CreatePillCommandHandler};

#[derive(Deserialize)]
pub struct CreatePillRequest {
    title: String,
    content: String,
}

pub async fn create_pill_handler(
    State(handler): State<Arc<CreatePillCommandHandler>>,
    Json(payload): Json<CreatePillRequest>,
) -> impl IntoResponse {
    let command = CreatePillCommand {
        title: payload.title,
        content: payload.content,
    };

    match handler.handle(command).await {
        Ok(_) => {
            println!("Controlador: Píldora creada con éxito.");
            StatusCode::CREATED
        }
        Err(e) => {
            eprintln!("Controlador: Error al crear la píldora: {:?}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}
