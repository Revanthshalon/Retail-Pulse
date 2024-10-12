use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppErrors {
    #[error("Invalid Input: {0}")]
    ValidationError(String),
    #[error("Database Error: {0}")]
    DatabaseError(#[from] sqlx::Error),
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
            AppErrors::DatabaseError(_e) => {
                // TODO: Log the error
                let status = StatusCode::INTERNAL_SERVER_ERROR;
                let response = Json(json!({
                    "error": "Internal Server Error",
                }));
                (status, response).into_response()
            }
        }
    }
}
