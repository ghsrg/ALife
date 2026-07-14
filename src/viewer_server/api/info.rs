use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct ServerInfo {
    engine_version: &'static str,
    api_version: &'static str,
    allow_remote_viewer: bool,
}

async fn handle_server_info() -> Json<ServerInfo> {
    Json(ServerInfo {
        engine_version: env!("CARGO_PKG_VERSION"),
        api_version: "1",
        allow_remote_viewer: false,
    })
}

pub fn router() -> Router {
    Router::new().route("/server/info", get(handle_server_info))
}
