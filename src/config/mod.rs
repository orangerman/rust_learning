mod database;
mod server;

use std::sync::LazyLock;

use anyhow::Context;
use config::Config;
pub use database::DataBaseConfig;
use serde::Deserialize;
pub use server::ServerConifg;

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(|| AppConfig::load().expect("failed to init config"));

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    server: ServerConifg,
    database: DataBaseConfig,
}
impl AppConfig {
    pub fn load() -> anyhow::Result<Self> {
        tracing::info!("AppConfig init");
        Config::builder()
            .add_source(
                config::File::with_name("application")
                    .format(config::FileFormat::Yaml)
                    .required(true),
            )
            .add_source(
                config::Environment::with_prefix("APP")
                    .try_parsing(true)
                    .separator("_")
                    .list_separator(","),
            )
            .build()
            .with_context(|| anyhow::anyhow!("Failed to load config"))?
            .try_deserialize()
            .with_context(|| anyhow::anyhow!("Failed to deserialize config"))
    }

    pub fn server(&self) -> &ServerConifg {
        &self.server
    }

    pub fn database(&self) -> &DataBaseConfig {
        &self.database
    }
}

pub fn get() -> &'static AppConfig {
    &CONFIG
}
