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
                    "request_id": "run-ui-debug-projection-test"
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn latest_projections_before_active_run_are_explicitly_unavailable() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/projections/latest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 404);
    let json = response_json(response).await;
    assert_eq!(json["ok"], false);
    assert_eq!(json["category"], "projection_unavailable");
    assert_eq!(json["projection_status"], "unavailable");
}

#[tokio::test]
async fn latest_projections_return_bounded_observer_payload_bundle() {
    let state = make_state();
    start_run(state.clone()).await;

    let response = create_app(state)
        .oneshot(
            Request::builder()
                .uri("/projections/latest")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let json = response_json(response).await;
    assert_eq!(json["schema_version"], "ControlCenterProjectionBundle/v1");
    assert_eq!(json["projection_kind"], "DebugProjectionBundle");
    assert_eq!(json["source"], "live");
    assert_eq!(json["run_id"], "run-ui-debug-projection-test");
    assert_eq!(
        json["visual_world"]["projection_kind"],
        "VisualWorldProjection"
    );
    assert_eq!(json["visual_world"]["completeness"]["state"], "bounded");
    assert_eq!(
        json["visual_world"]["completeness"]["missing_fields"]
            .as_array()
            .unwrap()
            .len(),
        0
    );
    assert_eq!(
        json["visual_world"]["payload"]["source_metrics"][0]["source_owner"],
        "CoreCommittedSnapshot"
    );
    assert!(
        json["visual_world"]["payload"]["cells"]
            .as_array()
            .unwrap()
            .len()
            > 0
    );
    assert!(
        json["visual_world"]["payload"]["resource_layers"][0]["cells"]
            .as_array()
            .unwrap()
            .len()
            > 0
    );
    assert!(
        json["coverage"]["projection_kind"]
            .as_str()
            .unwrap()
            .contains("Coverage")
    );
    assert!(
        json["warnings"]["projection_kind"]
            .as_str()
            .unwrap()
            .contains("Warning")
    );
    assert!(
        json["classifications"]["projection_kind"]
            .as_str()
            .unwrap()
            .contains("Classification")
    );
    assert!(
        json["balance_findings"]["projection_kind"]
            .as_str()
            .unwrap()
            .contains("BalanceFinding")
    );
}
