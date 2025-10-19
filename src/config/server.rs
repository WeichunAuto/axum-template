use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ServerConfig {
    host: Option<String>,
    port: Option<u16>,
}

impl ServerConfig {
    pub fn get_host(&self) -> String {
        self.host.clone().unwrap_or("127.0.0.1".to_string())
    }
    /// read port from config file
    pub fn get_port(&self) -> u16 {
        self.port.unwrap_or(3000)
    }
}
