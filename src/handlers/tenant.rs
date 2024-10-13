use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Extension, Json,
};
use serde_json::json;

use crate::models::auth::Claims;
use crate::{models::tenant::UpdateTenantDTO, AppState};

pub async fn get_all_tenants(State(app_state): State<AppState>) -> Response {
    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.get_all_tenants().await;

    match result {
        Ok(tenants) => {
            let status = StatusCode::OK;
            let response = Json(json!({
                "data": tenants,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}

pub async fn get_tenant_by_id(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Response {
    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.get_tenant_by_id(claims.sub).await;

    match result {
        Ok(tenant) => {
            let status = StatusCode::OK;
            let response = Json(json!({
                "data": tenant,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}

pub async fn update_tenant(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateTenantDTO>,
) -> Response {
    let input_validation = payload.validate();
    if let Err(errors) = input_validation {
        return errors.into_response();
    }

    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.update_tenant(claims.sub, payload).await;

    match result {
        Ok(tenant) => {
            let status = StatusCode::OK;
            let response = Json(json!({
                "data": tenant,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}

pub async fn delete_tenant(
    State(app_state): State<AppState>,
    Extension(claims): Extension<Claims>,
) -> Response {
    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.delete_tenant(claims.sub).await;

    match result {
        Ok(success) => {
            let status = StatusCode::OK;
            let response = Json(json!({
                "success": success,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}
