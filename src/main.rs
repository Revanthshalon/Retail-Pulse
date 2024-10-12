use std::sync::Arc;

use tokio::net::TcpListener;

use repositories::RepositoryContainer;
use routes::create_api_routes;
use services::ServiceContainer;

use crate::config::AppConfig;
use crate::db::DbService;

mod config;
mod db;
mod entities;
mod errors;
mod handlers;
mod models;
mod repositories;
mod routes;
mod services;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::from_env();
    let db_service = DbService::new(&app_config).await;

    let repository_container = RepositoryContainer::new(db_service.pool());
    let service_container = ServiceContainer::new(repository_container);

    let app_state = AppState::new(service_container);

    let app_routes = create_api_routes(app_state);
    let listener = TcpListener::bind("localhost:3000")
        .await
        .expect("Failed to bind port 3000");

    axum::serve(listener, app_routes.into_make_service())
        .await
        .map_err(|err| err.into())
}

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<ServiceContainer>,
}

impl AppState {
    pub fn new(service: ServiceContainer) -> Self {
        Self {
            service: Arc::new(service),
        }
    }
}
