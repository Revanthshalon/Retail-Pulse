use std::sync::Arc;

use repositories::RepositoryContainer;
use routes::create_api_routes;
use services::ServiceContainer;
use tokio::net::TcpListener;
use tracing::info;

use crate::config::AppConfig;
use crate::db::DbService;

mod config;
mod db;
mod entities;
mod errors;
mod handlers;
mod middlewares;
mod models;
mod repositories;
mod routes;
mod services;
mod utils;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().pretty().json().init();

    let app_config = AppConfig::from_env();
    let db_service = DbService::new(&app_config).await;

    let repository_container = RepositoryContainer::new(db_service.pool());
    let service_container = ServiceContainer::new(repository_container);

    let app_state = AppState::new(service_container, app_config);

    let app_routes = create_api_routes(app_state);
    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("Failed to bind port 8080");

    info!("Server is running on port 8080");

    axum::serve(listener, app_routes.into_make_service())
        .await
        .map_err(|err| err.into())
}

#[derive(Clone)]
pub struct AppState {
    pub service: Arc<ServiceContainer>,
    pub config: Arc<AppConfig>,
}

impl AppState {
    pub fn new(service: ServiceContainer, app_config: AppConfig) -> Self {
        Self {
            service: Arc::new(service),
            config: Arc::new(app_config),
        }
    }
}
