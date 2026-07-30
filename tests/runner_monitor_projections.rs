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
                    "request_id": "run-monitor-projection-test"
                })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn latest_projections_include_monitor_data_panel_contract() {
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
    assert_eq!(json["monitor"]["schema_version"], "MonitorDataPanelProjection/v1");
    assert_eq!(json["monitor"]["projection_kind"], "MonitorDataPanelProjection");
    assert_eq!(json["monitor"]["source"], "live");
    assert_eq!(json["monitor"]["run_id"], "run-monitor-projection-test");
    assert_eq!(json["monitor"]["completeness"]["state"], "partial");

    for key in ["world", "cells", "organisms", "lineages", "evolution", "analytics"] {
        assert!(
            json["monitor"]["payload"].get(key).is_some(),
            "monitor payload must include {key}"
        );
    }

    assert_eq!(
        json["monitor"]["payload"]["world"]["population_lifecycle"]["source"],
        "VisualWorldProjection.cells.lifecycleState"
    );
    assert_eq!(
        json["monitor"]["payload"]["world"]["energy_flow"]["state"],
        "unavailable"
    );
    assert_eq!(
        json["monitor"]["payload"]["cells"]["observed_primary_roles"]["state"],
        "unavailable"
    );
}
