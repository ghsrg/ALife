use crate::viewer_server::projection_sampler::ViewerProjectionConfig;
use serde::Deserialize;
use std::path::Path;

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct ServerConfig {
    pub bind_host: String,
    pub port: u16,
    pub allow_remote_viewer: bool,
    pub allowed_origins: Vec<String>,
    pub target_broadcast_fps: u32,
    pub runner_pacing: RunnerPacingConfig,
    pub viewer_projection: ViewerProjectionConfig,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            port: 8080,
            allow_remote_viewer: false,
            allowed_origins: Vec::new(),
            target_broadcast_fps: 30,
            runner_pacing: RunnerPacingConfig::default(),
            viewer_projection: ViewerProjectionConfig::default(),
        }
    }
}

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(default)]
pub struct RunnerPacingConfig {
    pub realtime_target_tps: u32,
    pub headless_target_tps: u32,
    pub run_unthrottled: bool,
}

impl Default for RunnerPacingConfig {
    fn default() -> Self {
        Self {
            realtime_target_tps: 10,
            headless_target_tps: 50,
            run_unthrottled: false,
        }
    }
}

impl ServerConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.port)
    }

    pub fn validate(&self) -> Result<(), String> {
        if !self.allow_remote_viewer && !is_local_bind_host(&self.bind_host) {
            return Err(format!(
                "bind_host={} requires allow_remote_viewer=true",
                self.bind_host
            ));
        }
        if self.allow_remote_viewer && self.allowed_origins.is_empty() {
            return Err("allow_remote_viewer=true requires allowed_origins".to_string());
        }
        Ok(())
    }
}

fn is_local_bind_host(bind_host: &str) -> bool {
    matches!(bind_host, "127.0.0.1" | "localhost" | "::1")
}

#[derive(Debug, Deserialize)]
struct ServerToml {
    #[serde(default)]
    server: ServerConfig,
    #[serde(default)]
    runner_pacing: RunnerPacingConfig,
    #[serde(default)]
    viewer_projection: ViewerProjectionConfig,
}

pub fn load_server_config(path: &Path) -> Result<ServerConfig, String> {
    if !path.exists() {
        return Ok(ServerConfig::default());
    }

    let content =
        std::fs::read_to_string(path).map_err(|err| format!("Cannot read {:?}: {}", path, err))?;
    let parsed: ServerToml =
        toml::from_str(&content).map_err(|err| format!("Parse error in {:?}: {}", path, err))?;
    let mut server = parsed.server;
    server.runner_pacing = parsed.runner_pacing;
    server.viewer_projection = parsed.viewer_projection;
    server.validate()?;
    Ok(server)
}
