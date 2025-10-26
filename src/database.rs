use crate::config;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};
use std::cmp::max;
use std::time::Duration;

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let db_config = config::AppConfig::get().database();
    let mut options = ConnectOptions::new(format!(
        "postgres://{}:{}@{}:{}/{}",
        db_config.user(),
        db_config.password(),
        db_config.host(),
        db_config.port(),
        db_config.db_name(),
    ));

    let num_cpus = num_cpus::get() as u32;
    options
        .min_connections(max(num_cpus * 4, 10))
        .max_connections(max(num_cpus * 8, 20))
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(20)) // read timeout
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(3600 * 24))
        .sqlx_logging(false)
        .set_schema_search_path(db_config.schema());

    let db_connection = Database::connect(options).await?;
    db_connection.ping().await?;
    tracing::info!("Database connection established");
    print_db_version(&db_connection).await?;

    Ok(db_connection)
}

async fn print_db_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version = db
        .query_one(Statement::from_string(
            DbBackend::Postgres,
            "SELECT version()",
        ))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Failed to get database version!"))?;
    tracing::info!(
        "Database version is: {}",
        version.try_get_by_index::<String>(0)?
    );

    Ok(())
}
