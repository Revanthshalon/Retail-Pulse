use crate::middlewares::auth::jwt_auth_middleware;
use crate::{
    handlers::tenant::{delete_tenant, get_all_tenants, get_tenant_by_id, update_tenant},
    AppState,
};
use axum::middleware::from_fn_with_state;
use axum::{
    routing::{delete, get, put},
    Router,
};

pub fn create_tenant_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/all", get(get_all_tenants))
        .route("/", get(get_tenant_by_id))
        .route("/:id", put(update_tenant))
        .route("/:id", delete(delete_tenant))
        .layer(from_fn_with_state(app_state.clone(), jwt_auth_middleware))
        .with_state(app_state)
}
