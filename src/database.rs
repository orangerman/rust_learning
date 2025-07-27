use std::time::Duration;

use super::config;
use anyhow::Ok;
use sea_orm::{
    ConnectOptions, ConnectionTrait, Database, DatabaseConnection, DbBackend, Statement,
};

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config: &config::DataBaseConfig = config::get().database();

    let mut options: ConnectOptions = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        database_config.username(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database()
    ));

    options
        .min_connections(2)
        .max_connections(50)
        .connect_timeout(Duration::from_secs(10))
        .acquire_timeout(Duration::from_secs(10))
        .idle_timeout(Duration::from_secs(60))
        .max_lifetime(Duration::from_secs(1800))
        .sqlx_logging(false);
    let db: DatabaseConnection = Database::connect(options).await?;
    db.ping().await?;
    tracing::info!("Databse connected successfully");

    log_database_version(&db).await?;

    Ok(db)
}

async fn log_database_version(db: &DatabaseConnection) -> anyhow::Result<()> {
    let version_result = db
        .query_one(Statement::from_string(DbBackend::MySql, "select version()"))
        .await?
        .ok_or_else(|| anyhow::anyhow!("Database init failed"))?;

    tracing::info!(
        "Database version is {}",
        version_result.try_get_by_index::<String>(0)?
    );

    Ok(())
}
