use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConifg {
    port: Option<u16>,
}

impl ServerConifg {
    pub fn port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
