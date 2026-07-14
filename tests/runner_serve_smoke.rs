//! Integration smoke tests: spawn a real HTTP server on an OS-assigned port.

use alife::viewer_server::{create_app, state::new_app_state};
use serde_json::Value;
use std::path::PathBuf;

async fn spawn_test_server() -> (String, tokio::task::JoinHandle<()>) {
    let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios");
    let state = new_app_state(scenarios_dir, 20, 30);
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0")
        .await
        .expect("test server should bind to a random local port");
    let addr = listener
        .local_addr()
        .expect("test server local address should be available");
    let base_url = format!("http://{addr}");

    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    (base_url, handle)
}

#[tokio::test]
async fn smoke_server_info_roundtrip() {
    let (base_url, handle) = spawn_test_server().await;

    let response = reqwest::get(format!("{base_url}/server/info"))
        .await
        .expect("server info request should succeed");
    assert_eq!(response.status(), 200);

    let body: Value = response.json().await.expect("server info should be JSON");
    assert_eq!(body["api_version"], "1");
    assert_eq!(body["allow_remote_viewer"], false);

    handle.abort();
}

#[tokio::test]
async fn smoke_start_and_status_roundtrip() {
    let (base_url, handle) = spawn_test_server().await;
    let client = reqwest::Client::new();

    let scenarios_response = client
        .get(format!("{base_url}/scenarios"))
        .send()
        .await
        .expect("scenarios request should succeed");
    assert_eq!(scenarios_response.status(), 200);
    let scenarios: Value = scenarios_response
        .json()
        .await
        .expect("scenarios response should be JSON");
    assert!(
        scenarios
            .as_array()
            .expect("scenarios response should be an array")
            .iter()
            .any(|item| item["id"] == "bootstrap_minimal_viable_world")
    );

    let start_response = client
        .post(format!("{base_url}/run/start"))
        .json(&serde_json::json!({
            "scenario_id": "bootstrap_minimal_viable_world",
            "request_id": "smoke-run"
        }))
        .send()
        .await
        .expect("run start request should succeed");
    assert_eq!(start_response.status(), 200);
    let start: Value = start_response
        .json()
        .await
        .expect("run start response should be JSON");
    assert_eq!(start["ok"], true);
    assert_eq!(start["run_id"], "smoke-run");
    assert!(
        start["scenario_hash"]
            .as_str()
            .expect("scenario_hash should be a string")
            .starts_with("scenario_hash_v1:")
    );

    let status_response = client
        .get(format!("{base_url}/run/status"))
        .send()
        .await
        .expect("run status request should succeed");
    assert_eq!(status_response.status(), 200);
    let status: Value = status_response
        .json()
        .await
        .expect("run status response should be JSON");
    assert_eq!(status["process_state"], "ready");
    assert_eq!(status["run_id"], "smoke-run");
    assert_eq!(status["scenario_id"], "bootstrap_minimal_viable_world");
    assert!(matches!(
        status["active_run_state"].as_str(),
        Some("running") | Some("completed")
    ));

    handle.abort();
}
