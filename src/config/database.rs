use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct DataBaseConfig {
    host: Option<String>,
    username: Option<String>,
    password: Option<String>,
    port: Option<u16>,
    database: Option<String>,
}

impl DataBaseConfig {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3306)
    }

    pub fn username(&self) -> &str {
        self.username.as_deref().unwrap_or("root")
    }

    pub fn password(&self) -> &str {
        self.password.as_deref().unwrap_or("root")
    }

    pub fn host(&self) -> &str {
        self.host.as_deref().unwrap_or("url")
    }

    pub fn database(&self) -> &str {
        self.database.as_deref().unwrap_or("demo")
    }
}
