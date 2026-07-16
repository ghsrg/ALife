pub mod api;
pub mod broadcaster;
pub mod frame_encoder;
pub mod projection_sampler;
pub mod state;

use axum::{
    Router,
    body::Body,
    http::{
        HeaderValue, Method, Request, StatusCode,
        header::{
            ACCESS_CONTROL_ALLOW_HEADERS, ACCESS_CONTROL_ALLOW_METHODS,
            ACCESS_CONTROL_ALLOW_ORIGIN, ORIGIN,
        },
    },
    middleware::{Next, from_fn},
    response::{IntoResponse, Response},
};
use state::AppState;

pub fn create_app(app_state: AppState) -> Router {
    api::build_router(app_state).layer(from_fn(local_viewer_cors))
}

async fn local_viewer_cors(request: Request<Body>, next: Next) -> Response {
    let origin = request
        .headers()
        .get(ORIGIN)
        .filter(|origin| is_allowed_local_viewer_origin(origin))
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
