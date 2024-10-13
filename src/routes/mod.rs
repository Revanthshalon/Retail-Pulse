use axum::Router;

use crate::routes::auth::create_auth_routes;
use crate::routes::tenant::create_tenant_routes;
use crate::AppState;

mod auth;
pub mod tenant;

pub fn create_api_routes(app_state: AppState) -> Router {
    let tenant_routes = Router::new().nest("/tenant", create_tenant_routes(app_state.clone()));
    let auth_routes = Router::new().nest("/auth", create_auth_routes(app_state.clone()));

    let merged_routes = Router::new().merge(auth_routes).merge(tenant_routes);

    Router::new().nest("/api", merged_routes)
}
