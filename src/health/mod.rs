use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Json},
};
use mongodb::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct HealthStatus {
    pub status: String,
    pub database: DatabaseStatus,
    pub timestamp: String,
    pub version: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseStatus {
    pub connected: bool,
    pub database_name: String,
    pub ping_response_time_ms: Option<u64>,
    pub error: Option<String>,
}

pub struct HealthService {
    database: Arc<Database>,
}

impl HealthService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }

    pub async fn check_health(&self) -> HealthStatus {
        let start_time = std::time::Instant::now();
        let database_status = self.check_database().await;
        let ping_time = if database_status.connected {
            Some(start_time.elapsed().as_millis() as u64)
        } else {
            None
        };

        let overall_status = if database_status.connected {
            "healthy"
        } else {
            "unhealthy"
        };

        HealthStatus {
            status: overall_status.to_string(),
            database: DatabaseStatus {
                connected: database_status.connected,
                database_name: database_status.database_name,
                ping_response_time_ms: ping_time,
                error: database_status.error,
            },
            timestamp: chrono::Utc::now().to_rfc3339(),
            version: env!("CARGO_PKG_VERSION").to_string(),
        }
    }

    async fn check_database(&self) -> DatabaseStatus {
        match self
            .database
            .run_command(mongodb::bson::doc! {"ping": 1}, None)
            .await
        {
            Ok(_) => DatabaseStatus {
                connected: true,
                database_name: self.database.name().to_string(),
                ping_response_time_ms: None, // Will be set by caller
                error: None,
            },
            Err(e) => DatabaseStatus {
                connected: false,
                database_name: self.database.name().to_string(),
                ping_response_time_ms: None,
                error: Some(format!("Database connection error: {}", e)),
            },
        }
    }
}

pub async fn health_check_handler(
    State(health_service): State<Arc<HealthService>>,
) -> impl IntoResponse {
    let health_status = health_service.check_health().await;

    let status_code = if health_status.status == "healthy" {
        StatusCode::OK
    } else {
        StatusCode::SERVICE_UNAVAILABLE
    };

    (status_code, Json(health_status)).into_response()
}

pub async fn readiness_check_handler(
    State(health_service): State<Arc<HealthService>>,
) -> impl IntoResponse {
    let health_status = health_service.check_health().await;

    if health_status.database.connected {
        (
            StatusCode::OK,
            Json(serde_json::json!({
                "status": "ready",
                "timestamp": health_status.timestamp
            })),
        )
            .into_response()
    } else {
        (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(serde_json::json!({
                "status": "not ready",
                "reason": "database not available",
                "timestamp": health_status.timestamp
            })),
        )
            .into_response()
    }
}

pub async fn liveness_check_handler() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "alive",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })),
    )
        .into_response()
}
