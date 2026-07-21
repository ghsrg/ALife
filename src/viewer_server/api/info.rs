use crate::viewer_server::state::AppState;
use axum::{Json, Router, extract::State, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct ServerInfo {
    engine_version: &'static str,
    api_version: &'static str,
    allow_remote_viewer: bool,
    bind_host: String,
    allowed_origins: Vec<String>,
}

async fn handle_server_info(State(state): State<AppState>) -> Json<ServerInfo> {
    let locked = state.lock().unwrap();
    Json(ServerInfo {
        engine_version: env!("CARGO_PKG_VERSION"),
        api_version: "1",
        allow_remote_viewer: locked.server_config.allow_remote_viewer,
        bind_host: locked.server_config.bind_host.clone(),
        allowed_origins: locked.server_config.allowed_origins.clone(),
    })
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/server/info", get(handle_server_info))
        .with_state(state)
}
