use crate::config::AppConfig;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;

/// `DbService` is a struct that holds a connection pool to the PostgreSQL database.
pub struct DbService {
    pool: PgPool,
}

impl DbService {
    /// Creates a new instance of `DbService`.
    ///
    /// # Arguments
    ///
    /// * `app_config` - A reference to the application configuration.
    ///
    /// # Returns
    ///
    /// A new instance of `DbService`.
    ///
    /// # Panics
    ///
    /// This function will panic if it fails to create the connection pool.
    pub async fn new(app_config: &AppConfig) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(app_config.db_max_connections())
            .idle_timeout(Duration::from_secs(app_config.db_connection_timeout()))
            .connect(&app_config.db_url())
            .await
            .expect("Failed to create connection pool");

        Self { pool }
    }

    /// Returns a clone of the connection pool.
    ///
    /// # Returns
    ///
    /// A `PgPool` instance.
    pub fn pool(&self) -> PgPool {
        self.pool.clone()
    }
}
