pub mod api;
pub mod broadcaster;
pub mod frame_encoder;
pub mod projection_sampler;
pub mod state;

use crate::runner::server_config::ServerConfig;
use axum::{
    Router,
    body::Body,
    extract::State,
    http::{
        HeaderValue, Method, Request, StatusCode,
        header::{
            ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN,
        },
    },
    middleware::{Next, from_fn_with_state},
    response::{IntoResponse, Response},
};
use state::{AppState, new_app_state_with_server_config};

pub fn create_app(app_state: AppState) -> Router {
    api::build_router(app_state.clone()).layer(from_fn_with_state(app_state, viewer_cors))
}

pub fn create_app_with_config(app_state: AppState, server_config: ServerConfig) -> Router {
    let (scenarios_dir, engine_snapshot_buffer_size) = {
        let locked = app_state.lock().unwrap();
        (
            locked.scenarios_dir.clone(),
            locked.engine_snapshot_buffer_size,
        )
    };
    let configured_state =
        new_app_state_with_server_config(scenarios_dir, engine_snapshot_buffer_size, server_config);
    create_app(configured_state)
}

async fn viewer_cors(
    State(app_state): State<AppState>,
    request: Request<Body>,
    next: Next,
) -> Response {
    let origin = request
        .headers()
        .get(ORIGIN)
        .filter(|origin| is_allowed_viewer_origin(origin, &app_state))
        .cloned();

    if request.method() == Method::OPTIONS {
        let mut response = StatusCode::NO_CONTENT.into_response();
        apply_cors_headers(response.headers_mut(), origin.as_ref());
        return response;
    }

    let mut response = next.run(request).await;
    apply_cors_headers(response.headers_mut(), origin.as_ref());
    response
}

fn apply_cors_headers(headers: &mut axum::http::HeaderMap, origin: Option<&HeaderValue>) {
    let Some(origin) = origin else {
        return;
    };

    headers.insert(ACCESS_CONTROL_ALLOW_ORIGIN, origin.clone());
    headers.insert(
        ACCESS_CONTROL_ALLOW_METHODS,
        HeaderValue::from_static("GET,POST,OPTIONS"),
    );
    headers.insert(
        ACCESS_CONTROL_ALLOW_HEADERS,
        HeaderValue::from_static("content-type"),
    );
}

fn is_allowed_local_viewer_origin(origin: &HeaderValue) -> bool {
    let Ok(origin) = origin.to_str() else {
        return false;
    };

    matches!(
        origin,
        "http://127.0.0.1:5173"
            | "http://localhost:5173"
            | "http://127.0.0.1:4173"
            | "http://localhost:4173"
    )
}

fn is_allowed_viewer_origin(origin: &HeaderValue, app_state: &AppState) -> bool {
    if is_allowed_local_viewer_origin(origin) {
        return true;
    }
    let Ok(origin) = origin.to_str() else {
        return false;
    };
    let locked = app_state.lock().unwrap();
    locked.server_config.allow_remote_viewer
        && locked
            .server_config
            .allowed_origins
            .iter()
            .any(|allowed| allowed == origin)
}
