use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use axum::http::{Method, Request};
use http_body_util::BodyExt;
use serde_json::json;
use std::path::PathBuf;
use tower::ServiceExt;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        20,
        30,
    )
}

fn json_body(value: serde_json::Value) -> Body {
    Body::from(value.to_string())
}

async fn response_json(response: axum::response::Response) -> serde_json::Value {
    let body = response.into_body().collect().await.unwrap().to_bytes();
    serde_json::from_slice(&body).unwrap()
}

async fn start_run(state: alife::viewer_server::state::AppState) {
    let response = create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "scenario_id": "bootstrap_minimal_viable_world",
                    "request_id": "run-projection-cadence-test"
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn projection_streaming_supports_full_and_sampled_stride() {
    let state = make_state();
    start_run(state.clone()).await;

    // Full resolution (stride=1 or omitted)
    let app = create_app(state.clone());
    let response_full = app
        .oneshot(
            Request::builder()
                .uri("/projections/latest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_full.status(), 200);
    let json_full = response_json(response_full).await;
    assert_eq!(json_full["schema_version"], "ControlCenterProjectionBundle/v1");

    // Sampled resolution (stride=2)
    let app_sampled = create_app(state);
    let response_sampled = app_sampled
        .oneshot(
            Request::builder()
                .uri("/projections/latest?stride=2")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response_sampled.status(), 200);
    let json_sampled = response_json(response_sampled).await;
    assert_eq!(json_sampled["schema_version"], "ControlCenterProjectionBundle/v1");
}
