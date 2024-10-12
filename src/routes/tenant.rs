use axum::{
    routing::{delete, get, post, put},
    Router,
};

use crate::{
    handlers::tenant::{
        create_tenant, delete_tenant, get_all_tenants, get_tenant_by_id, update_tenant,
    },
    AppState,
};

pub fn create_tenant_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/:id", get(get_tenant_by_id))
        .route("/all", get(get_all_tenants))
        .route("/", post(create_tenant))
        .route("/tenant/:id", put(update_tenant))
        .route("/tenant/:id", delete(delete_tenant))
        .with_state(app_state)
}
