use serde::Deserialize;

/// Database configuration for PostgreSQL connection.
///
/// All fields are optional with sensible defaults for development environment.
/// Configuration can be loaded from YAML files or overridden by environment variables.
#[derive(Debug, Deserialize)]
pub struct DbConfig {
    /// Database server hostname or IP address
    host: Option<String>,
    /// Database server port
    port: Option<u16>,
    /// Database authentication username
    user: Option<String>,
    /// Database authentication password
    password: Option<String>,
    /// Name of the database to connect to
    db_name: Option<String>,
    /// PostgreSQL schema name (namespace)
    schema: Option<String>,
}

impl DbConfig {
    /// Returns the database host with fallback to localhost.
    ///
    /// Default: `127.0.0.1`
    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("127.0.0.1")
    }

    /// Returns the database port with fallback to PostgreSQL default.
    ///
    /// Default: `5432`
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(5432)
    }

    /// Returns the database username with fallback to 'postgres'.
    ///
    /// Default: `postgres`
    pub fn user(&self) -> &str {
        self.user.as_deref().unwrap_or("postgres")
    }

    /// Returns the database password with fallback to 'postgres'.
    ///
    /// Default: `postgres`
    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("postgres")
    }

    /// Returns the database name with fallback to 'axum_template'.
    ///
    /// Default: `axum_template`
    pub fn db_name(&self) -> &str {
        self.db_name.as_deref().unwrap_or("axum_template")
    }

    /// Returns the PostgreSQL schema name with fallback to 'public'.
    ///
    /// Default: `public`
    pub fn schema(&self) -> &str {
        self.schema.as_deref().unwrap_or("public")
    }

    /// Constructs a PostgreSQL connection URL from the configuration.
    ///
    /// Format: `postgres://user:password@host:port/database`
    pub fn database_url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user(),
            self.password(),
            self.host(),
            self.port(),
            self.db_name()
        )
    }
}
