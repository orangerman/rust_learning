use super::config;
use sea_orm::{ConnectOptions, DatabaseConnection};

pub async fn init() -> anyhow::Result<DatabaseConnection> {
    let database_config: &config::DataBaseConfig = config::get().database();

    let options = ConnectOptions::new(format!(
        "mysql://{}:{}@{}:{}/{}",
        database_config.username(),
        database_config.password(),
        database_config.host(),
        database_config.port(),
        database_config.database()
    ));

    todo!()
}
