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
    jwt_secret: String,
    jwt_expiry: i64,
    jwt_refresh_secret: String,
    jwt_refresh_expiry: i64,
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
        let jwt_secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
        let jwt_expiry = env::var("JWT_EXPIRY")
            .expect("JWT_EXPIRY must be set")
            .parse::<i64>()
            .expect("JWT_EXPIRY must be a number");
        let jwt_refresh_secret =
            env::var("JWT_REFRESH_SECRET").expect("JWT_REFRESH_SECRET must be set");
        let jwt_refresh_expiry = env::var("JWT_REFRESH_EXPIRY")
            .expect("JWT_REFRESH_EXPIRY must be set")
            .parse::<i64>()
            .expect("JWT_REFRESH_EXPIRY must be a number");

        Self {
            db_url,
            db_max_connections,
            db_connection_timeout,
            jwt_secret,
            jwt_expiry,
            jwt_refresh_secret,
            jwt_refresh_expiry,
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
    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
    pub fn jwt_expiry(&self) -> i64 {
        self.jwt_expiry
    }
    pub fn jwt_refresh_secret(&self) -> &str {
        &self.jwt_refresh_secret
    }
    pub fn jwt_refresh_expiry(&self) -> i64 {
        self.jwt_refresh_expiry
    }
}
