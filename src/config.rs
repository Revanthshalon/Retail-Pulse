/// `AppConfig` is a struct that holds the configuration for the application.
///
/// # Fields
///
/// * `db_url` - The URL of the database.
/// * `db_max_connections` - The maximum number of connections to the database.
/// * `db_connection_timeout` - The timeout duration for database connections.
pub struct AppConfig {
    db_url: String,
    db_max_connections: u32,
    db_connection_timeout: u64,
}

impl AppConfig {
    /// Creates a new instance of `AppConfig` from environment variables.
    ///
    /// # Panics
    ///
    /// This function will panic if any of the required environment variables are not set or cannot be parsed.
    pub fn from_env() -> Self {
        use dotenvy::dotenv;
        use std::env;

        dotenv().ok();

        let db_url = env::var("DATABASE_URL").expect("DB_URL must be set");
        let db_max_connections = env::var("DATABASE_MAX_CONNECTIONS")
            .expect("DB_MAX_CONNECTIONS must be set")
            .parse::<u32>()
            .expect("DB_MAX_CONNECTIONS must be a number");
        let db_connection_timeout = env::var("DATABASE_CONNECTION_TIMEOUT")
            .expect("DB_CONNECTION_TIMEOUT must be set")
            .parse::<u64>()
            .expect("DB_CONNECTION_TIMEOUT must be a number");

        Self {
            db_url,
            db_max_connections,
            db_connection_timeout,
        }
    }

    /// Returns the database URL.
    ///
    /// # Returns
    ///
    /// A string slice that holds the database URL.
    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    /// Returns the maximum number of database connections.
    ///
    /// # Returns
    ///
    /// A `u32` value that represents the maximum number of database connections.
    pub fn db_max_connections(&self) -> u32 {
        self.db_max_connections
    }

    /// Returns the database connection timeout duration.
    ///
    /// # Returns
    ///
    /// A `u64` value that represents the timeout duration for database connections.
    pub fn db_connection_timeout(&self) -> u64 {
        self.db_connection_timeout
    }
}
