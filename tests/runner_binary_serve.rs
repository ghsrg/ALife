use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};

use futures_util::StreamExt;
use serde_json::Value;
use tokio_tungstenite::tungstenite::Message;

struct ChildGuard(Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}

#[tokio::test]
async fn serve_flag_starts_http_server() {
    let child = Command::new(env!("CARGO_BIN_EXE_runner"))
        .arg("--serve")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("runner --serve should spawn");
    let mut child = ChildGuard(child);

    let client = reqwest::Client::new();
    let deadline = Instant::now() + Duration::from_secs(5);
    loop {
        if let Some(status) = child.0.try_wait().expect("child status should be readable") {
            panic!("runner --serve exited before accepting HTTP requests: {status}");
        }

        let last_error = match client.get("http://127.0.0.1:8080/server/info").send().await {
            Ok(response) if response.status().is_success() => {
                let body: Value = response.json().await.expect("server info should be JSON");
                assert_eq!(body["api_version"], "1");
                assert_eq!(body["allow_remote_viewer"], false);

                let (mut ws, _) = tokio_tungstenite::connect_async("ws://127.0.0.1:8080/stream")
                    .await
                    .expect("runner --serve should accept WebSocket stream connections");
                let message = ws
                    .next()
                    .await
                    .expect("WebSocket should emit initial status")
                    .expect("initial WebSocket status should be readable");
                let Message::Text(text) = message else {
                    panic!("initial WebSocket message should be text status");
                };
                let status: Value =
                    serde_json::from_str(&text).expect("WebSocket status should be JSON");
                assert_eq!(status["type"], "status");
                assert_eq!(status["process_state"], "ready");
                assert_eq!(status["active_run_state"], "idle");
                return;
            }
            Ok(response) => format!("HTTP {}", response.status()),
            Err(error) => error.to_string(),
        };

        if Instant::now() >= deadline {
            panic!("runner --serve did not accept HTTP requests: {last_error}");
        }

        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
