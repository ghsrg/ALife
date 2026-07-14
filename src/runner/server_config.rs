use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub bind_host: String,
    pub port: u16,
    pub allow_remote_viewer: bool,
    pub target_broadcast_fps: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            port: 8080,
            allow_remote_viewer: false,
            target_broadcast_fps: 30,
        }
    }
}

impl ServerConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.port)
    }
}

#[derive(Debug, Deserialize)]
struct ServerToml {
    server: ServerConfig,
}

pub fn load_server_config(path: &Path) -> Result<ServerConfig, String> {
    if !path.exists() {
        return Ok(ServerConfig::default());
    }

    let content =
        std::fs::read_to_string(path).map_err(|err| format!("Cannot read {:?}: {}", path, err))?;
    let parsed: ServerToml =
        toml::from_str(&content).map_err(|err| format!("Parse error in {:?}: {}", path, err))?;
    Ok(parsed.server)
}
