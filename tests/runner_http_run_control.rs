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

async fn start_run(state: alife::viewer_server::state::AppState) -> axum::response::Response {
    create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({
                    "scenario_id": "bootstrap_minimal_viable_world"
                })))
                .unwrap(),
        )
        .await
        .unwrap()
}

#[tokio::test]
async fn get_run_status_initially_idle() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/run/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let json = response_json(response).await;
    assert_eq!(json["process_state"].as_str().unwrap(), "ready");
    assert_eq!(json["active_run_state"].as_str().unwrap(), "idle");
    assert_eq!(json["committed_tick"].as_u64().unwrap(), 0);
}

#[tokio::test]
async fn post_run_start_with_valid_scenario_returns_canon_fields() {
    let state = make_state();
    let response = start_run(state).await;

    assert_eq!(response.status(), 200);
    let json = response_json(response).await;
    assert_eq!(json["ok"].as_bool().unwrap(), true);
    assert_eq!(json["active_run_state"].as_str().unwrap(), "running");
    assert!(json["run_id"].as_str().unwrap().starts_with("run-"));
    assert!(
        json["scenario_hash"]
            .as_str()
            .unwrap()
            .starts_with("scenario_hash_v1:")
    );
    assert_eq!(json["effective_seed"].as_u64().unwrap(), 42);
    assert!(json.get("bootstrap_manifest").is_some());
}

#[tokio::test]
async fn post_run_start_sets_status_to_running() {
    let state = make_state();
    start_run(state.clone()).await;

    let response = create_app(state)
        .oneshot(
            Request::builder()
                .uri("/run/status")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let json = response_json(response).await;
    assert_eq!(json["active_run_state"].as_str().unwrap(), "running");
    assert_eq!(
        json["scenario_id"].as_str().unwrap(),
        "bootstrap_minimal_viable_world"
    );
}

#[tokio::test]
async fn post_run_start_with_unknown_scenario_returns_400() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "not_a_real_scenario" })))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn post_run_start_when_active_returns_409() {
    let state = make_state();
    start_run(state.clone()).await;

    let response = start_run(state).await;
    assert_eq!(response.status(), 409);
    let json = response_json(response).await;
    assert_eq!(json["category"].as_str().unwrap(), "state_conflict");
}

#[tokio::test]
async fn pause_and_resume_change_state_through_http() {
    let state = make_state();
    start_run(state.clone()).await;

    let pause_response = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/pause")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pause_response.status(), 200);
    let pause_json = response_json(pause_response).await;
    assert_eq!(pause_json["active_run_state"].as_str().unwrap(), "paused");

    let resume_response = create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/resume")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resume_response.status(), 200);
    let resume_json = response_json(resume_response).await;
    assert_eq!(resume_json["active_run_state"].as_str().unwrap(), "running");
}

#[tokio::test]
async fn post_run_step_from_running_returns_409_state_conflict() {
    let state = make_state();
    start_run(state.clone()).await;

    let response = create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/step")
                .header("content-type", "application/json")
                .body(json_body(json!({})))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 409);
    let json = response_json(response).await;
    assert_eq!(json["category"].as_str().unwrap(), "state_conflict");
}

#[tokio::test]
async fn post_run_step_from_paused_commits_exactly_one_tick_and_remains_paused() {
    let state = make_state();
    start_run(state.clone()).await;

    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/pause")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let before = {
        let response = create_app(state.clone())
            .oneshot(
                Request::builder()
                    .uri("/run/status")
                    .body(Body::empty())
                    .unwrap(),
            )
            .await
            .unwrap();
        response_json(response).await["committed_tick"]
            .as_u64()
            .unwrap()
    };

    let response = create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/step")
                .header("content-type", "application/json")
                .body(json_body(json!({})))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let json = response_json(response).await;
    assert_eq!(json["active_run_state"].as_str().unwrap(), "paused");
    assert_eq!(json["committed_tick"].as_u64().unwrap(), before + 1);
}
