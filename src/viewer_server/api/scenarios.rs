use crate::viewer_server::state::AppState;
use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
};
use serde::Serialize;

#[derive(Serialize)]
struct ScenarioListItem {
    id: String,
    path: String,
}

#[derive(Serialize)]
struct ScenarioDocumentResponse {
    id: String,
    config_toml: String,
}

async fn handle_list_scenarios(
    State(state): State<AppState>,
) -> Result<Json<Vec<ScenarioListItem>>, StatusCode> {
    let scenarios_dir = state.lock().unwrap().scenarios_dir.clone();
    let metas = crate::runner::scenario::scan_scenarios(&scenarios_dir)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let items = metas
        .into_iter()
        .map(|meta| {
            let path = meta
                .path
                .strip_prefix(&scenarios_dir)
                .unwrap_or(&meta.path)
                .to_string_lossy()
                .replace('\\', "/");
            ScenarioListItem { id: meta.id, path }
        })
        .collect();

    Ok(Json(items))
}

async fn handle_get_scenario(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ScenarioDocumentResponse>, StatusCode> {
    let scenarios_dir = state.lock().unwrap().scenarios_dir.clone();
    let metas = crate::runner::scenario::scan_scenarios(&scenarios_dir)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    let meta = metas
        .into_iter()
        .find(|scenario| scenario.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;
    let config_toml =
        std::fs::read_to_string(&meta.path).map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ScenarioDocumentResponse { id, config_toml }))
}

pub fn router(_state: AppState) -> Router {
    Router::new()
        .route("/scenarios", get(handle_list_scenarios))
        .route("/scenarios/{id}", get(handle_get_scenario))
        .with_state(_state)
}
