use crate::models::auth::{AuthLoginDTO, AuthRegisterDTO};
use crate::utils::generate_token;
use crate::AppState;
use axum::extract::State;
use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde_json::json;

pub async fn register_user(
    State(app_state): State<AppState>,
    Json(payload): Json<AuthRegisterDTO>,
) -> Response {
    let validation = payload.validate();
    if let Err(errors) = validation {
        return errors.into_response();
    };
    let auth_service = &app_state.service.auth_service;
    let register_result = auth_service.register(payload).await;
    match register_result {
        Ok(user) => {
            let token_result = generate_token(&app_state.config, &user);
            let token = match token_result {
                Ok(token) => token,
                Err(e) => return e.into_response(),
            };
            let status = StatusCode::CREATED;
            let response = Json(json!({
                "token": token,
                "user": user,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}

pub async fn login_user(
    State(app_state): State<AppState>,
    Json(payload): Json<AuthLoginDTO>,
) -> Response {
    let auth_service = &app_state.service.auth_service;
    let login_result = auth_service.login(payload).await;
    match login_result {
        Ok(user) => {
            let token_result = generate_token(&app_state.config, &user);
            let token = match token_result {
                Ok(token) => token,
                Err(e) => return e.into_response(),
            };
            let status = StatusCode::OK;
            let response = Json(json!({
                "token": token,
                "user": user,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}
