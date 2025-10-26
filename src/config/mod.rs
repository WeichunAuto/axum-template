use crate::config::database::DbConfig;
use crate::config::server::ServerConfig;
use anyhow::{Context, Result};
use config::{Config, FileFormat};
use serde::Deserialize;
use std::fmt::Debug;
use std::sync::LazyLock;

pub mod server;

pub mod database;

/// Lazily initialized global application configuration.
///
/// This static instance will be initialized only once on first access.
static APP_CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConfig,
    database: DbConfig,
}
impl AppConfig {
    /// Loads configuration from multiple sources:
    ///
    /// 1. **TOML file:** `config/{RUN_ENV}.toml`
    ///    - Determined by the `RUN_ENV` environment variable (default: `"dev"`).
    /// 2. **Environment variables:** prefixed with `APP_`
    ///    - Example: `APP_SERVER_PORT=9090`
    ///
    /// Environment variables have higher priority and override file values.
    ///
    /// # Returns
    /// - `Ok(AppConfig)` on success
    /// - `Err(AppError)` with context if loading or deserialization fails
    pub fn load() -> Result<Self> {
        // Determine the runtime environment, defaulting to "dev"
        let run_env = std::env::var("RUN_ENV").unwrap_or_else(|_| "dev".into());

        // Build configuration from multiple sources
        Config::builder()
            .add_source(
                config::File::with_name(format!("config/{}.yaml", run_env).as_str())
                    .format(FileFormat::Yaml)
                    .required(false),
            )
            // Override with environment variables prefixed by `APP_`
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_"),
            )
            .build()
            .with_context(|| "Failed to load config file".to_string())?
            .try_deserialize()
            .with_context(|| "Failed to deserialize config file".to_string())
    }

    /// Returns a global, lazily initialized reference to the application configuration.
    pub fn get() -> &'static Self {
        &APP_CONFIG
    }

    pub fn server(&self) -> &ServerConfig {
        &self.server
    }

    pub fn database(&self) -> &DbConfig {
        &self.database
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = AppConfig::get();
        println!("{:?}", config);
    }
}
