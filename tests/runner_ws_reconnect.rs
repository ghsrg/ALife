use alife::viewer_server::{create_app, state::new_app_state};
use futures_util::StreamExt;
use serde_json::Value;
use std::path::PathBuf;
use tokio_tungstenite::{connect_async, tungstenite::Message};

fn scenarios_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios")
}

async fn spawn_test_server() -> String {
    let state = new_app_state(scenarios_dir(), 50, 30);
    let app = create_app(state);
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();

    tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });

    format!("http://{addr}")
}

async fn next_text(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Value {
    let msg = tokio::time::timeout(std::time::Duration::from_secs(1), ws.next())
        .await
        .expect("timed out waiting for WS text")
        .expect("WS stream should yield a message")
        .expect("WS message should be valid");
    let Message::Text(text) = msg else {
        panic!("expected text status message");
    };
    serde_json::from_str(&text).expect("status message should be JSON")
}

async fn next_binary(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
) -> Vec<u8> {
    tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            let msg = ws.next().await.unwrap().unwrap();
            if let Message::Binary(bytes) = msg {
                return bytes.to_vec();
            }
        }
    })
    .await
    .expect("timed out waiting for WS binary frame")
}

#[tokio::test]
async fn reconnect_receives_current_status_and_latest_available_frame() {
    let base_url = spawn_test_server().await;
    let ws_url = base_url.replace("http://", "ws://") + "/stream";
    let client = reqwest::Client::new();

    let (mut first_ws, _) = connect_async(&ws_url).await.unwrap();
    assert_eq!(next_text(&mut first_ws).await["active_run_state"], "idle");

    let start = client
        .post(format!("{base_url}/run/start"))
        .json(&serde_json::json!({
            "scenario_id": "bootstrap_minimal_viable_world",
            "request_id": "ws-reconnect-test"
        }))
        .send()
        .await
        .unwrap();
    assert_eq!(start.status(), 200);
    let first_frame = next_binary(&mut first_ws).await;
    assert_eq!(&first_frame[0..4], b"ALIF");
    drop(first_ws);

    let (mut reconnected_ws, _) = connect_async(&ws_url).await.unwrap();
    let status = next_text(&mut reconnected_ws).await;
    assert_eq!(status["type"], "status");
    assert_eq!(status["run_id"], "ws-reconnect-test");

    let reconnect_frame = next_binary(&mut reconnected_ws).await;
    assert_eq!(&reconnect_frame[0..4], b"ALIF");
}

#[tokio::test]
async fn idle_reconnect_does_not_fabricate_frame() {
    let base_url = spawn_test_server().await;
    let ws_url = base_url.replace("http://", "ws://") + "/stream";

    let (mut ws, _) = connect_async(&ws_url).await.unwrap();
    assert_eq!(next_text(&mut ws).await["active_run_state"], "idle");

    let maybe_frame = tokio::time::timeout(std::time::Duration::from_millis(150), ws.next()).await;
    assert!(
        maybe_frame.is_err(),
        "idle connection should not emit a frame"
    );
}
