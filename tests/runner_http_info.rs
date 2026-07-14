use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use axum::http::Request;
use http_body_util::BodyExt;
use std::path::PathBuf;
use tower::ServiceExt;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        10,
    )
}

#[tokio::test]
async fn get_server_info_returns_200() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/server/info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn get_server_info_returns_json_with_required_fields() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/server/info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("engine_version").is_some());
    assert!(json.get("api_version").is_some());
    assert!(json.get("allow_remote_viewer").is_some());
}

#[tokio::test]
async fn get_server_info_api_version_is_string_1() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/server/info")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["api_version"].as_str().unwrap(), "1");
}
