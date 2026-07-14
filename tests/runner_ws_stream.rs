use alife::viewer_server::{create_app, state::new_app_state};
use futures_util::StreamExt;
use serde_json::Value;
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

fn scenarios_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios")
}

async fn spawn_test_server() -> (String, alife::viewer_server::state::AppState) {
    let state = new_app_state(scenarios_dir(), 50, 30);
    let app = create_app(state.clone());
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    (format!("http://{addr}"), state)
}

async fn next_text(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    let msg = tokio::time::timeout(std::time::Duration::from_millis(500), ws.next())
        .await
        .expect("timed out waiting for WS message")
        .expect("WS stream should yield a message")
        .expect("WS message should be valid");
    let Message::Text(text) = msg else {
        panic!("expected text JSON status message");
    };
    serde_json::from_str(&text).expect("status message should be JSON")
}

#[tokio::test]
async fn ws_connect_receives_initial_status_idle() {
    let (base_url, _) = spawn_test_server().await;
    let ws_url = base_url.replace("http://", "ws://") + "/stream";

    let (mut ws, _) = connect_async(&ws_url).await.unwrap();
    let status = next_text(&mut ws).await;

    assert_eq!(status["type"], "status");
    assert_eq!(status["process_state"], "ready");
    assert_eq!(status["active_run_state"], "idle");
    assert_eq!(status["committed_tick"], 0);
}

#[tokio::test]
async fn two_ws_clients_both_receive_initial_status() {
    let (base_url, _) = spawn_test_server().await;
    let ws_url = base_url.replace("http://", "ws://") + "/stream";

    let (mut ws1, _) = connect_async(&ws_url).await.unwrap();
    let (mut ws2, _) = connect_async(&ws_url).await.unwrap();

    assert_eq!(next_text(&mut ws1).await["active_run_state"], "idle");
    assert_eq!(next_text(&mut ws2).await["active_run_state"], "idle");
}

#[tokio::test]
async fn ws_receives_binary_alif_frame_after_start() {
    let (base_url, _) = spawn_test_server().await;
    let ws_url = base_url.replace("http://", "ws://") + "/stream";
    let (mut ws, _) = connect_async(&ws_url).await.unwrap();
    let _ = next_text(&mut ws).await;

    let client = reqwest::Client::new();
    let start = client
        .post(format!("{base_url}/run/start"))
        .json(&serde_json::json!({
            "scenario_id": "bootstrap_minimal_viable_world",
            "request_id": "ws-frame-test"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(start.status(), 200);

    let frame = tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            let msg = ws.next().await.unwrap().unwrap();
            if let Message::Binary(bytes) = msg {
                return bytes;
            }
        }
    })
    .await
    .expect("timed out waiting for ALIF frame");

    assert_eq!(&frame[0..4], b"ALIF");
    assert_eq!(frame[4], 1);
    assert!(frame.len() >= 26);
}

#[tokio::test]
async fn slow_ws_client_does_not_block_simulation() {
    let (base_url, state) = spawn_test_server().await;
    let ws_url = base_url.replace("http://", "ws://") + "/stream";
    let (_ws, _) = connect_async(&ws_url).await.unwrap();

    let client = reqwest::Client::new();
    let start = client
        .post(format!("{base_url}/run/start"))
        .json(&serde_json::json!({
            "scenario_id": "bootstrap_minimal_viable_world",
            "request_id": "ws-slow-client-test"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(start.status(), 200);

    tokio::time::sleep(std::time::Duration::from_millis(300)).await;

    assert!(state.lock().unwrap().committed_tick > 0);
}
