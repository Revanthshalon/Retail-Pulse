use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

use crate::{
    models::tenant::{CreateTenantDTO, UpdateTenantDTO},
    AppState,
};

pub async fn create_tenant(
    State(app_state): State<AppState>,
    Json(payload): Json<CreateTenantDTO>,
) -> Response {
    let input_validation = payload.validate();
    if let Err(errors) = input_validation {
        return errors.into_response();
    }

    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.create_tenant(payload).await;

    match result {
        Ok(tenant) => {
            let status = StatusCode::CREATED;
            let response = Json(json!({
                "data": tenant,
            }));
            (status, response).into_response()
        }
        Err(errors) => errors.into_response(),
    }
}

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

pub async fn get_tenant_by_id(State(app_state): State<AppState>, Path(id): Path<i32>) -> Response {
    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.get_tenant_by_id(id).await;

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
    Path(id): Path<i32>,
    Json(payload): Json<UpdateTenantDTO>,
) -> Response {
    let input_validation = payload.validate();
    if let Err(errors) = input_validation {
        return errors.into_response();
    }

    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.update_tenant(id, payload).await;

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

pub async fn delete_tenant(State(app_state): State<AppState>, Path(id): Path<i32>) -> Response {
    let tenant_service = &app_state.service.tenant_service;

    let result = tenant_service.delete_tenant(id).await;

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
