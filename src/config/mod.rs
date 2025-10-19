use crate::config::server::ServerConfig;
use anyhow::{Context, Result};
use config::{Config, FileFormat};
use serde::Deserialize;
use std::fmt::Debug;
use std::sync::LazyLock;

pub mod server;

/// Lazily initialized global application configuration.
///
/// This static instance will be initialized only once on first access.
///
/// # Example
/// ```rust
/// let config = AppConfig::get_config();
/// println!("Server host: {}", config.server.host);
/// ```
static APP_CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("Failed to load config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server: ServerConfig,
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
    pub fn get_config() -> &'static Self {
        &APP_CONFIG
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config() {
        let config = AppConfig::get_config();
        println!("{:?}", config);
    }
}
