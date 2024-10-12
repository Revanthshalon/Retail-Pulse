use axum::Router;

use crate::AppState;

pub mod tenant;

pub fn create_api_routes(app_state: AppState) -> Router {
    let tenant_routes =
        Router::new().nest("/tenant", tenant::create_tenant_routes(app_state.clone()));

    Router::new().nest("/api", tenant_routes)
}
