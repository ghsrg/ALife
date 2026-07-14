use alife::runner::server_config::{ServerConfig, load_server_config};
use std::path::PathBuf;

fn server_toml_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/server.toml")
}

#[test]
fn server_config_loads_from_file() {
    let cfg = load_server_config(&server_toml_path()).expect("should parse");
    assert!(!cfg.bind_host.is_empty());
    assert!(cfg.port > 0);
}

#[test]
fn server_config_defaults_are_local() {
    let cfg = ServerConfig::default();
    assert_eq!(cfg.bind_host, "127.0.0.1");
    assert_eq!(cfg.port, 8080);
    assert_eq!(cfg.target_broadcast_fps, 30);
    assert!(!cfg.allow_remote_viewer);
}

#[test]
fn server_config_bind_addr_formats_correctly() {
    let cfg = ServerConfig {
        bind_host: "127.0.0.1".to_string(),
        port: 9090,
        ..ServerConfig::default()
    };
    assert_eq!(cfg.bind_addr(), "127.0.0.1:9090");
}
