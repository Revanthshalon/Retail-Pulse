use crate::handlers::auth::{login_user, register_user};
use crate::AppState;
use axum::routing::post;
use axum::Router;

pub fn create_auth_routes(app_state: AppState) -> Router {
    Router::new()
        .route("/register", post(register_user))
        .route("/login", post(login_user))
        .with_state(app_state)
}
