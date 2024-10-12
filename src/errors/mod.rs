use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum AppErrors {
    #[error("Invalid Input: {0}")]
    ValidationError(String),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
    #[error("Conflict: {0}")]
    Conflict(String),
    #[error("Not Found: {0}")]
    NotFound(String),
}

impl IntoResponse for AppErrors {
    fn into_response(self) -> Response {
        match self {
            AppErrors::ValidationError(message) => {
                let status = StatusCode::BAD_REQUEST;
                let response = Json(json!({
                    "error": message,
                }));
                (status, response).into_response()
            }
            AppErrors::DatabaseError(e) => {
                error!("Database Error: {:?}", e);
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let response = Json(json!({
                    "error": "Internal Server Error",
                }));
                (status, response).into_response()
            }
            AppErrors::Conflict(message) => {
                let status = StatusCode::CONFLICT;
                let response = Json(json!({
                    "error": message,
                }));
                (status, response).into_response()
            }
            AppErrors::NotFound(message) => {
                let status = StatusCode::NOT_FOUND;
                let response = Json(json!({
                    "error": message,
                }));
                (status, response).into_response()
            }
        }
    }
}
