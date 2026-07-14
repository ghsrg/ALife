use alife::viewer_server::{
    broadcaster::WsMessage,
    create_app,
    state::{AppState, new_app_state},
};
use axum::body::Body;
use axum::http::{Method, Request};
use http_body_util::BodyExt;
use std::path::PathBuf;
use tower::ServiceExt;

fn make_state() -> AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        20,
        30,
    )
}

async fn start_bootstrap_run(state: AppState) {
    let response = create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "scenario_id": "bootstrap_minimal_viable_world",
                        "request_id": "broadcast-test"
                    })
                    .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(response.status(), 200);
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["ok"], true);
}

#[test]
fn app_state_exposes_broadcaster_and_target_fps() {
    let state = make_state();
    let locked = state.lock().unwrap();

    assert_eq!(locked.target_broadcast_fps, 30);
    let _rx = locked.broadcaster.subscribe();
}

#[tokio::test]
async fn tick_loop_broadcasts_alif_frame_without_blocking_run_state() {
    let state = make_state();
    let mut rx = state.lock().unwrap().broadcaster.subscribe();

    start_bootstrap_run(state.clone()).await;

    let frame = tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            match rx.recv().await.unwrap() {
                WsMessage::Frame(bytes) => return bytes,
                WsMessage::Status(_) => {}
            }
        }
    })
    .await
    .expect("tick loop should broadcast a frame");

    assert_eq!(&frame[0..4], b"ALIF");
    assert!(state.lock().unwrap().committed_tick > 0);
}
