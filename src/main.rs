use crate::config::AppConfig;
use crate::db::DbService;

mod config;
mod db;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app_config = AppConfig::from_env();
    let db_service = DbService::new(&app_config).await;
    Ok(())
}
