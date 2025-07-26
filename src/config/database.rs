use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    pub url: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    pub port: Option<u16>,
}
