# Runner Phase 2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Додати `--serve` прапорець до runner binary, який стартує HTTP сервер (axum + tokio) з повним набором command API ендпоінтів: `/server/info`, `/scenarios`, `/scenarios/{id}`, `/run/status`, `/run/start`, `/run/pause`, `/run/resume`, `/run/step`, `/run/stop`.

**Architecture:** `src/viewer_server/` — новий модуль (crate в межах workspace — пізніше, зараз internal module). `SharedState` в `Arc<Mutex<>>` зберігає поточний `RunEngine` та run state. Tick loop запускається в окремому `std::thread` щоб не блокувати tokio runtime. HTTP handlers отримують стан через `axum::extract::State<Arc<Mutex<SharedState>>>`. Тести використовують `tower::ServiceExt::oneshot()` без реального порту.

**Tech Stack:** `axum 0.8`, `tokio 1 (full)`, `tower` (dev-dep для тестування), `serde_json` (вже є). Всі endpoint-тести — `#[tokio::test]` async.

**Передумови:** Runner-1 завершений: `RunEngine`, `RunState`, `RingBuffer<CommittedSnapshot>`, `ScenarioMeta`, `scan_scenarios`, `load_scenario`, `src/bin/runner.rs` — все існує.

---

## File Structure

```
src/
  viewer_server/
    mod.rs           [NEW] — pub mod state, api; pub fn create_app
    state.rs         [NEW] — SharedState, TickLoopSignal, RunEngineHandle
    api/
      mod.rs         [NEW] — route registration
      info.rs        [NEW] — GET /server/info
      scenarios.rs   [NEW] — GET /scenarios, GET /scenarios/{id}
      run.rs         [NEW] — GET /run/status, POST /run/start|pause|resume|step|stop
  runner/
    server_config.rs [NEW] — ServerConfig, load_server_config, ServerConfigBuilder
    mod.rs           [MODIFY] — pub mod server_config
  bin/
    runner.rs        [MODIFY] — parse --serve, start tokio runtime, call serve()
  lib.rs             [MODIFY] — pub mod viewer_server
Cargo.toml           [MODIFY] — axum, tokio deps; tower, http-body-util dev-deps
config/
  server.toml        [NEW] — bind_host, port, snapshot_buffer_size, stream_frame_interval
tests/
  runner_server_config.rs      [NEW] — ServerConfig parse tests
  runner_http_info.rs          [NEW] — GET /server/info tests
  runner_http_scenarios.rs     [NEW] — GET /scenarios, GET /scenarios/{id}
  runner_http_run_control.rs   [NEW] — run state machine via HTTP
```

---

## Task 1: Dependencies and ServerConfig

**Files:**
- Modify: `Cargo.toml`
- Create: `src/runner/server_config.rs`
- Modify: `src/runner/mod.rs`
- Create: `config/server.toml`
- Test: `tests/runner_server_config.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_server_config.rs
use alife::runner::server_config::{ServerConfig, load_server_config};
use std::path::PathBuf;

fn server_toml_path() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/server.toml")
}

#[test]
fn server_config_loads_from_file() {
    let cfg = load_server_config(&server_toml_path()).expect("should parse");
    assert!(!cfg.bind_host.is_empty());
    assert!(cfg.port > 0);
}

#[test]
fn server_config_defaults_are_local() {
    let cfg = ServerConfig::default();
    assert_eq!(cfg.bind_host, "127.0.0.1");
    assert_eq!(cfg.port, 8080);
    assert_eq!(cfg.snapshot_buffer_size, 300);
    assert_eq!(cfg.stream_frame_interval, 3);
    assert!(!cfg.allow_remote_viewer);
}

#[test]
fn server_config_bind_addr_formats_correctly() {
    let cfg = ServerConfig {
        bind_host: "127.0.0.1".to_string(),
        port: 9090,
        ..ServerConfig::default()
    };
    assert_eq!(cfg.bind_addr(), "127.0.0.1:9090");
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_server_config
```

Expected: `FAIL` — `alife::runner::server_config` does not exist.

- [ ] **Step 3: Update `Cargo.toml`**

```toml
[package]
name = "alife"
version = "0.1.0"
edition = "2024"

[[bin]]
name = "runner"
path = "src/bin/runner.rs"

[[bin]]
name = "sweep_analyzer"
path = "src/bin/sweep_analyzer.rs"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
toml = "0.8"
serde_json = "1.0"
axum = "0.8"
tokio = { version = "1", features = ["full"] }

[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
```

- [ ] **Step 4: Create `config/server.toml`**

```toml
[server]
bind_host = "127.0.0.1"
port = 8080
allow_remote_viewer = false
snapshot_buffer_size = 300
stream_frame_interval = 3
```

- [ ] **Step 5: Create `src/runner/server_config.rs`**

```rust
use serde::Deserialize;
use std::path::Path;

/// HTTP server configuration loaded from config/server.toml.
#[derive(Debug, Clone, Deserialize)]
pub struct ServerConfig {
    pub bind_host: String,
    pub port: u16,
    pub allow_remote_viewer: bool,
    /// Number of committed snapshots to keep in ring buffer (scroll-back window).
    pub snapshot_buffer_size: usize,
    /// Push a frame to WS clients every N ticks (Runner-3, ignored here).
    pub stream_frame_interval: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            port: 8080,
            allow_remote_viewer: false,
            snapshot_buffer_size: 300,
            stream_frame_interval: 3,
        }
    }
}

impl ServerConfig {
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.bind_host, self.port)
    }
}

/// Wrapper for TOML top-level [server] section.
#[derive(Debug, Deserialize)]
struct ServerToml {
    server: ServerConfig,
}

/// Load server config from a TOML file.
/// Returns `ServerConfig::default()` if file not found, error on parse failure.
pub fn load_server_config(path: &Path) -> Result<ServerConfig, String> {
    if !path.exists() {
        return Ok(ServerConfig::default());
    }
    let content = std::fs::read_to_string(path)
        .map_err(|e| format!("Cannot read {:?}: {}", path, e))?;
    let parsed: ServerToml = toml::from_str(&content)
        .map_err(|e| format!("Parse error in {:?}: {}", path, e))?;
    Ok(parsed.server)
}
```

- [ ] **Step 6: Додати `pub mod server_config` у `src/runner/mod.rs`**

```rust
pub mod config_parser;
pub mod engine;
pub mod ring_buffer;
pub mod scenario;
pub mod server_config;
```

- [ ] **Step 7: Run tests**

```bash
cargo test --test runner_server_config
```

Expected: всі 3 тести `PASS`.

- [ ] **Step 8: Commit**

```bash
git add Cargo.toml config/server.toml \
        src/runner/server_config.rs src/runner/mod.rs \
        tests/runner_server_config.rs
git commit -m "feat(runner): add ServerConfig with bind_host/port/buffer_size"
```

---

## Task 2: SharedState

**Files:**
- Create: `src/viewer_server/state.rs`
- Create: `src/viewer_server/mod.rs` (skeleton)
- Modify: `src/lib.rs`

> Тут немає окремого тестового файлу для SharedState — його поведінка перевіряється через HTTP endpoint тести у Tasks 3-5. Але ми тестуємо побудову SharedState у тих самих тестах.

- [ ] **Step 1: Create `src/viewer_server/state.rs`**

```rust
use crate::runner::engine::{RunEngine, RunEngineConfig, RunState};
use crate::runner::scenario::{ScenarioMeta, scan_scenarios, load_scenario};
use crate::core::snapshot::CommittedSnapshot;
use crate::runner::ring_buffer::RingBuffer;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU32, Ordering};
use std::sync::{Arc, Mutex};

/// Signals sent to the background tick loop.
pub struct TickLoopSignal {
    pub stop: Arc<AtomicBool>,
    pub pause: Arc<AtomicBool>,
}

impl TickLoopSignal {
    pub fn new() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            pause: Arc::new(AtomicBool::new(false)),
        }
    }

    pub fn request_stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }

    pub fn request_pause(&self) {
        self.pause.store(true, Ordering::Relaxed);
    }

    pub fn request_resume(&self) {
        self.pause.store(false, Ordering::Relaxed);
    }

    pub fn is_stop_requested(&self) -> bool {
        self.stop.load(Ordering::Relaxed)
    }

    pub fn is_pause_requested(&self) -> bool {
        self.pause.load(Ordering::Relaxed)
    }
}

/// Full server state shared between HTTP handlers and the tick loop.
pub struct SharedState {
    /// Active run engine. None when Idle.
    pub engine: Option<RunEngine>,
    /// Current run lifecycle state.
    pub run_state: RunState,
    /// Scenario ID of the active run.
    pub scenario_id: Option<String>,
    /// Tick count from the last completed tick.
    pub current_tick: u32,
    /// Ring buffer of recent snapshots (scroll-back).
    pub snapshots: RingBuffer<CommittedSnapshot>,
    /// Reason for the last collapse/stop.
    pub collapse_reason: Option<String>,
    /// Path to config/scenarios/ directory.
    pub scenarios_dir: PathBuf,
    /// Loop signal — Some when a tick loop thread is running.
    pub tick_signal: Option<Arc<TickLoopSignal>>,
    /// Snapshot buffer capacity (from ServerConfig).
    pub snapshot_buffer_size: usize,
}

impl SharedState {
    pub fn new(scenarios_dir: PathBuf, snapshot_buffer_size: usize) -> Self {
        Self {
            engine: None,
            run_state: RunState::Idle,
            scenario_id: None,
            current_tick: 0,
            snapshots: RingBuffer::new(snapshot_buffer_size),
            collapse_reason: None,
            scenarios_dir,
            tick_signal: None,
            snapshot_buffer_size,
        }
    }

    /// Check whether a run is already active (Running or Paused).
    pub fn is_active(&self) -> bool {
        matches!(self.run_state, RunState::Running | RunState::Paused)
    }
}

/// Type alias used by axum handlers.
pub type AppState = Arc<Mutex<SharedState>>;

/// Build a new AppState.
pub fn new_app_state(scenarios_dir: PathBuf, snapshot_buffer_size: usize) -> AppState {
    Arc::new(Mutex::new(SharedState::new(scenarios_dir, snapshot_buffer_size)))
}

/// Spawn the background tick loop. Stores the signal in `state.tick_signal`.
/// Assumes engine is already initialised and state.run_state == Running.
pub fn spawn_tick_loop(state: AppState) {
    let signal = Arc::new(TickLoopSignal::new());
    {
        let mut locked = state.lock().unwrap();
        locked.tick_signal = Some(Arc::clone(&signal));
    }

    std::thread::spawn(move || {
        loop {
            if signal.is_stop_requested() {
                break;
            }
            if signal.is_pause_requested() {
                std::thread::sleep(std::time::Duration::from_millis(10));
                continue;
            }

            let result = {
                let mut locked = state.lock().unwrap();
                if let Some(engine) = locked.engine.as_mut() {
                    let r = engine.step(1);
                    if r.is_ok() {
                        locked.current_tick = engine.current_tick();
                        if let Some(snap) = engine.snapshots().newest() {
                            locked.snapshots.push(snap.clone());
                        }
                    }
                    r
                } else {
                    break;
                }
            };

            if result.is_err() {
                let mut locked = state.lock().unwrap();
                locked.run_state = RunState::Idle;
                locked.collapse_reason = Some("simulation_error".to_string());
                break;
            }
        }

        // Ensure state is cleaned up
        let mut locked = state.lock().unwrap();
        if matches!(locked.run_state, RunState::Running) {
            locked.run_state = RunState::Idle;
        }
        locked.tick_signal = None;
    });
}
```

- [ ] **Step 2: Create `src/viewer_server/mod.rs`** (skeleton)

```rust
pub mod api;
pub mod state;

use axum::Router;
use state::AppState;

/// Build the axum Router with all API routes mounted.
pub fn create_app(app_state: AppState) -> Router {
    api::build_router(app_state)
}
```

- [ ] **Step 3: Create `src/viewer_server/api/mod.rs`** (stub — full impl in Tasks 3-5)

```rust
pub mod info;
pub mod run;
pub mod scenarios;

use axum::Router;
use crate::viewer_server::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        .merge(info::router())
        .merge(scenarios::router(state.clone()))
        .merge(run::router(state))
}
```

- [ ] **Step 4: Create stub files for each API module**

Create `src/viewer_server/api/info.rs` with just a placeholder:

```rust
use axum::Router;

pub fn router() -> Router {
    Router::new()
}
```

Create `src/viewer_server/api/scenarios.rs` with just a placeholder:

```rust
use axum::Router;
use crate::viewer_server::state::AppState;

pub fn router(_state: AppState) -> Router {
    Router::new()
}
```

Create `src/viewer_server/api/run.rs` with just a placeholder:

```rust
use axum::Router;
use crate::viewer_server::state::AppState;

pub fn router(_state: AppState) -> Router {
    Router::new()
}
```

- [ ] **Step 5: Expose `viewer_server` in `src/lib.rs`**

```rust
extern crate self as alife;

pub mod core;
pub mod runner;

pub mod cell;
pub mod organism;
pub mod physics;
pub mod renderer;
pub mod simulation;
pub mod world;

pub mod process {
    pub use crate::core::process::*;
}

pub mod observer;
pub mod viewer_server;
```

- [ ] **Step 6: Build to verify it compiles**

```bash
cargo build
```

Expected: компілюється без помилок (stub routes, nothing implemented yet).

- [ ] **Step 7: Commit**

```bash
git add src/viewer_server/ src/lib.rs
git commit -m "feat(viewer-server): add SharedState, TickLoopSignal, app skeleton"
```

---

## Task 3: GET /server/info

**Files:**
- Modify: `src/viewer_server/api/info.rs`
- Test: `tests/runner_http_info.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_http_info.rs
use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use http::Request;
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::path::PathBuf;

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
        .oneshot(Request::builder().uri("/server/info").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn get_server_info_returns_json_with_required_fields() {
    let app = create_app(make_state());
    let response = app
        .oneshot(Request::builder().uri("/server/info").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json.get("engine_version").is_some(), "engine_version required");
    assert!(json.get("api_version").is_some(), "api_version required");
    assert!(json.get("allow_remote_viewer").is_some(), "allow_remote_viewer required");
}

#[tokio::test]
async fn get_server_info_api_version_is_string_1() {
    let app = create_app(make_state());
    let response = app
        .oneshot(Request::builder().uri("/server/info").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["api_version"].as_str().unwrap(), "1");
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_http_info
```

Expected: `FAIL` — router returns 404 (routes not yet registered).

- [ ] **Step 3: Implement `src/viewer_server/api/info.rs`**

```rust
use axum::{Json, Router, routing::get};
use serde::Serialize;

#[derive(Serialize)]
struct ServerInfo {
    engine_version: &'static str,
    api_version: &'static str,
    allow_remote_viewer: bool,
}

async fn handle_server_info() -> Json<ServerInfo> {
    Json(ServerInfo {
        engine_version: env!("CARGO_PKG_VERSION"),
        api_version: "1",
        allow_remote_viewer: false,
    })
}

pub fn router() -> Router {
    Router::new().route("/server/info", get(handle_server_info))
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test runner_http_info
```

Expected: всі 3 тести `PASS`.

- [ ] **Step 5: Commit**

```bash
git add src/viewer_server/api/info.rs tests/runner_http_info.rs
git commit -m "feat(viewer-server): implement GET /server/info"
```

---

## Task 4: GET /scenarios and GET /scenarios/{id}

**Files:**
- Modify: `src/viewer_server/api/scenarios.rs`
- Test: `tests/runner_http_scenarios.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_http_scenarios.rs
use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use http::Request;
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::path::PathBuf;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        10,
    )
}

#[tokio::test]
async fn get_scenarios_returns_200() {
    let app = create_app(make_state());
    let response = app
        .oneshot(Request::builder().uri("/scenarios").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);
}

#[tokio::test]
async fn get_scenarios_returns_array_with_ids() {
    let app = create_app(make_state());
    let response = app
        .oneshot(Request::builder().uri("/scenarios").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = json.as_array().expect("scenarios must be a JSON array");
    assert!(!arr.is_empty(), "Expected at least one scenario");

    let first = &arr[0];
    assert!(first.get("id").is_some(), "scenario must have id");
    assert!(first.get("path").is_some(), "scenario must have path");
}

#[tokio::test]
async fn get_scenarios_includes_single_cell_survival() {
    let app = create_app(make_state());
    let response = app
        .oneshot(Request::builder().uri("/scenarios").body(Body::empty()).unwrap())
        .await
        .unwrap();

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    let arr = json.as_array().unwrap();
    let found = arr.iter().any(|s| s["id"].as_str() == Some("single_cell_survival"));
    assert!(found, "single_cell_survival scenario must be listed");
}

#[tokio::test]
async fn get_scenario_by_id_returns_200_and_toml() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios/single_cell_survival")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["id"].as_str().unwrap(), "single_cell_survival");
    assert!(
        json["config_toml"].as_str().is_some(),
        "config_toml must be present"
    );
}

#[tokio::test]
async fn get_scenario_by_unknown_id_returns_404() {
    let app = create_app(make_state());
    let response = app
        .oneshot(
            Request::builder()
                .uri("/scenarios/this_does_not_exist")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 404);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_http_scenarios
```

Expected: `FAIL` — routes not implemented.

- [ ] **Step 3: Implement `src/viewer_server/api/scenarios.rs`**

```rust
use axum::{
    Router,
    extract::{Path, State},
    http::StatusCode,
    routing::get,
    Json,
};
use serde::Serialize;

use crate::runner::scenario::scan_scenarios;
use crate::viewer_server::state::AppState;

#[derive(Serialize)]
struct ScenarioEntry {
    id: String,
    path: String,
}

#[derive(Serialize)]
struct ScenarioDetail {
    id: String,
    config_toml: String,
}

async fn handle_list_scenarios(State(state): State<AppState>) -> Json<Vec<ScenarioEntry>> {
    let scenarios_dir = {
        let locked = state.lock().unwrap();
        locked.scenarios_dir.clone()
    };

    let metas = scan_scenarios(&scenarios_dir).unwrap_or_default();
    let entries = metas
        .into_iter()
        .map(|m| ScenarioEntry {
            path: m.path.display().to_string(),
            id: m.id,
        })
        .collect();

    Json(entries)
}

async fn handle_get_scenario(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<ScenarioDetail>, StatusCode> {
    let scenarios_dir = {
        let locked = state.lock().unwrap();
        locked.scenarios_dir.clone()
    };

    let metas = scan_scenarios(&scenarios_dir).unwrap_or_default();
    let meta = metas
        .into_iter()
        .find(|m| m.id == id)
        .ok_or(StatusCode::NOT_FOUND)?;

    let config_toml = std::fs::read_to_string(&meta.path)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(ScenarioDetail { id, config_toml }))
}

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/scenarios", get(handle_list_scenarios))
        .route("/scenarios/{id}", get(handle_get_scenario))
        .with_state(state)
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test runner_http_scenarios
```

Expected: всі 5 тестів `PASS`.

- [ ] **Step 5: Commit**

```bash
git add src/viewer_server/api/scenarios.rs tests/runner_http_scenarios.rs
git commit -m "feat(viewer-server): implement GET /scenarios and GET /scenarios/{id}"
```

---

## Task 5: Run control endpoints

**Files:**
- Modify: `src/viewer_server/api/run.rs`
- Test: `tests/runner_http_run_control.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_http_run_control.rs
use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use http::{Request, Method};
use http_body_util::BodyExt;
use tower::ServiceExt;
use serde_json::json;
use std::path::PathBuf;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        20,
    )
}

fn json_body(value: serde_json::Value) -> Body {
    Body::from(serde_json::to_vec(&value).unwrap())
}

#[tokio::test]
async fn get_run_status_initially_idle() {
    let app = create_app(make_state());
    let response = app
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["state"].as_str().unwrap(), "idle");
    assert_eq!(json["tick"].as_u64().unwrap(), 0);
}

#[tokio::test]
async fn post_run_start_with_valid_scenario_returns_ok() {
    let state = make_state();
    let app = create_app(state);
    let response = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["ok"].as_bool().unwrap(), true);
    assert!(json.get("config_hash").is_some());
}

#[tokio::test]
async fn post_run_start_sets_state_to_running() {
    let state = make_state();
    let app = create_app(state.clone());

    // Start
    app.clone()
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();

    // Check status reflects Running
    let response = create_app(state.clone())
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["state"].as_str().unwrap(), "running");
    assert_eq!(json["scenario_id"].as_str().unwrap(), "single_cell_survival");
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
                .body(json_body(json!({ "scenario_id": "nonexistent_scenario" })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 400);
}

#[tokio::test]
async fn post_run_start_when_already_running_returns_409() {
    let state = make_state();

    // First start
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();

    // Second start should fail
    let response = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 409);
}

#[tokio::test]
async fn post_run_step_executes_n_ticks() {
    let state = make_state();

    // Start first
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();

    // Stop the background loop so step is deterministic in test
    {
        let locked = state.lock().unwrap();
        if let Some(signal) = &locked.tick_signal {
            signal.request_stop();
        }
    }
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Step 5 ticks
    let response = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/step")
                .header("content-type", "application/json")
                .body(json_body(json!({ "ticks": 5 })))
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(response.status(), 200);

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["tick_after"].as_u64().unwrap() >= 5);
}

#[tokio::test]
async fn post_run_stop_returns_to_idle() {
    let state = make_state();

    // Start
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();

    // Stop
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/stop")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // Allow background thread to react
    std::thread::sleep(std::time::Duration::from_millis(50));

    // Verify idle
    let response = create_app(state.clone())
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["state"].as_str().unwrap(), "idle");
}

#[tokio::test]
async fn post_run_pause_and_resume_changes_state() {
    let state = make_state();

    // Start
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(json_body(json!({ "scenario_id": "single_cell_survival" })))
                .unwrap(),
        )
        .await
        .unwrap();

    // Pause
    let pause_resp = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/pause")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(pause_resp.status(), 200);

    // Verify paused
    let status_body = create_app(state.clone())
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap()
        .into_body()
        .collect()
        .await
        .unwrap()
        .to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&status_body).unwrap();
    assert_eq!(json["state"].as_str().unwrap(), "paused");

    // Resume
    let resume_resp = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/resume")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();
    assert_eq!(resume_resp.status(), 200);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_http_run_control
```

Expected: `FAIL` — routes return 404.

- [ ] **Step 3: Implement `src/viewer_server/api/run.rs`**

```rust
use axum::{
    Router,
    extract::{Json as ExtractJson, State},
    http::StatusCode,
    routing::{get, post},
    Json,
};
use serde::{Deserialize, Serialize};

use crate::runner::engine::{RunEngine, RunEngineConfig, RunState};
use crate::runner::scenario::scan_scenarios;
use crate::viewer_server::state::{AppState, spawn_tick_loop};

// ── Status ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RunStatus {
    state: String,
    tick: u64,
    scenario_id: Option<String>,
    config_hash: Option<String>,
    collapse_reason: Option<String>,
}

fn state_label(s: RunState) -> &'static str {
    match s {
        RunState::Idle => "idle",
        RunState::Running => "running",
        RunState::Paused => "paused",
    }
}

async fn handle_run_status(State(state): State<AppState>) -> Json<RunStatus> {
    let locked = state.lock().unwrap();
    Json(RunStatus {
        state: state_label(locked.run_state).to_string(),
        tick: locked.current_tick as u64,
        scenario_id: locked.scenario_id.clone(),
        config_hash: None, // TODO: derive from config in Runner-4
        collapse_reason: locked.collapse_reason.clone(),
    })
}

// ── Start ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct StartRequest {
    scenario_id: String,
    seed: Option<u64>,
}

#[derive(Serialize)]
struct StartResponse {
    ok: bool,
    config_hash: String,
    seed: u64,
}

#[derive(Serialize)]
struct ErrorResponse {
    ok: bool,
    error: String,
}

async fn handle_run_start(
    State(state): State<AppState>,
    ExtractJson(req): ExtractJson<StartRequest>,
) -> Result<Json<StartResponse>, (StatusCode, Json<ErrorResponse>)> {
    // 409 if already active
    {
        let locked = state.lock().unwrap();
        if locked.is_active() {
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    ok: false,
                    error: format!("Run already active in state: {:?}", locked.run_state),
                }),
            ));
        }
    }

    // Find scenario
    let scenarios_dir = state.lock().unwrap().scenarios_dir.clone();
    let metas = scan_scenarios(&scenarios_dir).unwrap_or_default();
    let meta = metas
        .into_iter()
        .find(|m| m.id == req.scenario_id)
        .ok_or_else(|| {
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    ok: false,
                    error: format!("Scenario not found: {}", req.scenario_id),
                }),
            )
        })?;

    let snapshot_buffer_size = state.lock().unwrap().snapshot_buffer_size;
    let engine_cfg = RunEngineConfig { snapshot_buffer_size };

    let mut engine = RunEngine::from_scenario(&meta, engine_cfg).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                error: format!("Failed to load scenario: {}", e),
            }),
        )
    })?;

    engine.start().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                error: format!("Failed to start engine: {}", e),
            }),
        )
    })?;

    let seed = req.seed.unwrap_or(42); // default seed if not specified

    {
        let mut locked = state.lock().unwrap();
        locked.engine = Some(engine);
        locked.run_state = RunState::Running;
        locked.scenario_id = Some(req.scenario_id);
        locked.current_tick = 0;
        locked.collapse_reason = None;
    }

    // Spawn background tick loop
    spawn_tick_loop(state);

    Ok(Json(StartResponse {
        ok: true,
        config_hash: "tbd".to_string(), // derive from TOML hash in Runner-4
        seed,
    }))
}

// ── Pause ─────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PauseResponse {
    ok: bool,
    tick: u64,
}

async fn handle_run_pause(
    State(state): State<AppState>,
) -> Result<Json<PauseResponse>, (StatusCode, Json<ErrorResponse>)> {
    let mut locked = state.lock().unwrap();

    if !matches!(locked.run_state, RunState::Running) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                ok: false,
                error: format!("Cannot pause from state: {:?}", locked.run_state),
            }),
        ));
    }

    locked.run_state = RunState::Paused;
    if let Some(signal) = &locked.tick_signal {
        signal.request_pause();
    }

    let tick = locked.current_tick as u64;
    Ok(Json(PauseResponse { ok: true, tick }))
}

// ── Resume ────────────────────────────────────────────────────────────────────

async fn handle_run_resume(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let mut locked = state.lock().unwrap();

    if !matches!(locked.run_state, RunState::Paused) {
        return Err((
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                ok: false,
                error: format!("Cannot resume from state: {:?}", locked.run_state),
            }),
        ));
    }

    locked.run_state = RunState::Running;
    if let Some(signal) = &locked.tick_signal {
        signal.request_resume();
    }

    Ok(Json(serde_json::json!({ "ok": true })))
}

// ── Step ──────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct StepRequest {
    ticks: u32,
}

#[derive(Serialize)]
struct StepResponse {
    ok: bool,
    tick_after: u64,
}

async fn handle_run_step(
    State(state): State<AppState>,
    ExtractJson(req): ExtractJson<StepRequest>,
) -> Result<Json<StepResponse>, (StatusCode, Json<ErrorResponse>)> {
    let ticks_to_run = req.ticks.min(10_000); // guard against runaway

    let tick_after = {
        let mut locked = state.lock().unwrap();
        if locked.engine.is_none() {
            return Err((
                StatusCode::CONFLICT,
                Json(ErrorResponse {
                    ok: false,
                    error: "No active engine. Call /run/start first.".to_string(),
                }),
            ));
        }
        let engine = locked.engine.as_mut().unwrap();
        engine.step(ticks_to_run).map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    ok: false,
                    error: format!("Step failed: {}", e),
                }),
            )
        })?;
        locked.current_tick = engine.current_tick();
        locked.current_tick as u64
    };

    Ok(Json(StepResponse { ok: true, tick_after }))
}

// ── Stop ──────────────────────────────────────────────────────────────────────

async fn handle_run_stop(State(state): State<AppState>) -> Json<serde_json::Value> {
    let mut locked = state.lock().unwrap();
    if let Some(signal) = &locked.tick_signal {
        signal.request_stop();
    }
    locked.run_state = RunState::Idle;
    locked.engine = None;
    locked.tick_signal = None;
    locked.scenario_id = None;
    Json(serde_json::json!({ "ok": true }))
}

// ── Router ────────────────────────────────────────────────────────────────────

pub fn router(state: AppState) -> Router {
    Router::new()
        .route("/run/status", get(handle_run_status))
        .route("/run/start", post(handle_run_start))
        .route("/run/pause", post(handle_run_pause))
        .route("/run/resume", post(handle_run_resume))
        .route("/run/step", post(handle_run_step))
        .route("/run/stop", post(handle_run_stop))
        .with_state(state)
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test runner_http_run_control
```

Expected: всі 8 тестів `PASS`. Якщо деякі тести залежать від часу (thread sleep), спробуй збільшити sleep до 100ms.

- [ ] **Step 5: Run entire test suite**

```bash
cargo test --workspace
```

Expected: без регресій у sweep_analyzer та попередніх тестах.

- [ ] **Step 6: Commit**

```bash
git add src/viewer_server/api/run.rs tests/runner_http_run_control.rs
git commit -m "feat(viewer-server): implement all /run/* HTTP endpoints"
```

---

## Task 6: `--serve` flag у runner binary

**Files:**
- Modify: `src/bin/runner.rs`

- [ ] **Step 1: Додати async main і --serve обробку**

Замінити `fn main()` на `#[tokio::main] async fn main()`:

```rust
//! ALife headless runner binary.
//!
//! Usage:
//!   cargo run --bin runner -- config/scenarios/single_cell_survival.toml
//!   cargo run --bin runner -- --list
//!   cargo run --bin runner -- --serve [config/scenarios/my_world.toml]
//!   cargo run --bin runner -- --serve --port 9090

use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::scenario::{scan_scenarios, ScenarioMeta};
use alife::runner::server_config::{load_server_config, ServerConfig};
use alife::viewer_server::{create_app, state::new_app_state};
use std::path::{Path, PathBuf};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: runner <scenario.toml> | --list | --serve [scenario.toml]");
        std::process::exit(1);
    }

    let scenarios_dir = PathBuf::from("config/scenarios");
    let server_config_path = PathBuf::from("config/server.toml");

    match args[1].as_str() {
        "--list" => {
            list_scenarios(&scenarios_dir);
        }
        "--serve" => {
            let server_cfg = load_server_config(&server_config_path)
                .unwrap_or_else(|e| {
                    eprintln!("[runner] Warning: cannot load server.toml: {}. Using defaults.", e);
                    ServerConfig::default()
                });
            serve(server_cfg, scenarios_dir).await;
        }
        path => {
            run_headless(Path::new(path), &scenarios_dir).await;
        }
    }
}

async fn serve(cfg: ServerConfig, scenarios_dir: PathBuf) {
    let bind_addr = cfg.bind_addr();
    let state = new_app_state(scenarios_dir, cfg.snapshot_buffer_size);
    let app = create_app(state);

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| {
            eprintln!("[runner] Cannot bind to {}: {}", bind_addr, e);
            std::process::exit(1);
        });

    println!("[runner] HTTP server listening on http://{}", bind_addr);
    println!("[runner] Endpoints:");
    println!("  GET  http://{}/server/info", bind_addr);
    println!("  GET  http://{}/scenarios", bind_addr);
    println!("  POST http://{}/run/start", bind_addr);
    println!("  GET  http://{}/run/status", bind_addr);
    println!("[runner] Press Ctrl+C to stop.");

    axum::serve(listener, app)
        .await
        .expect("Server failed");
}

async fn run_headless(scenario_path: &Path, scenarios_dir: &Path) {
    let meta = resolve_scenario(scenario_path, scenarios_dir);

    println!("[runner] Loading scenario: {} ({})", meta.id, meta.path.display());

    let engine_cfg = RunEngineConfig {
        snapshot_buffer_size: 300,
    };

    let mut engine = match RunEngine::from_scenario(&meta, engine_cfg) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("[runner] Failed to load scenario: {}", e);
            std::process::exit(1);
        }
    };

    engine.start().expect("RunEngine::start failed");

    let rt_config = alife::runner::scenario::load_scenario(&meta).expect("load ok");
    let total_ticks = rt_config.world.tick_count.raw() as u32;

    let start = std::time::Instant::now();
    println!("[runner] Running {} ticks...", total_ticks);

    engine.step(total_ticks).unwrap_or_else(|e| {
        eprintln!("[runner] Simulation error at tick {}: {}", engine.current_tick(), e);
    });

    let elapsed = start.elapsed();
    let tps = if elapsed.as_secs_f32() > 0.0 {
        engine.current_tick() as f32 / elapsed.as_secs_f32()
    } else {
        0.0
    };

    println!(
        "[runner] Completed {} ticks in {:.2}s ({:.0} ticks/sec)",
        engine.current_tick(),
        elapsed.as_secs_f32(),
        tps
    );

    if let Some(snap) = engine.snapshots().newest() {
        println!(
            "[runner] Final tick: {}, cells: {}, heat: {:.2}, waste: {:.2}",
            snap.tick.raw(),
            snap.cells.len(),
            snap.heat,
            snap.waste,
        );
    }

    engine.stop().expect("RunEngine::stop failed");
    println!("[runner] Done.");
}

fn list_scenarios(dir: &Path) {
    match scan_scenarios(dir) {
        Ok(metas) => {
            println!("Available scenarios in {}:", dir.display());
            for meta in &metas {
                println!("  {} ({})", meta.id, meta.path.display());
            }
        }
        Err(e) => {
            eprintln!("Error scanning scenarios: {}", e);
            std::process::exit(1);
        }
    }
}

fn resolve_scenario(path: &Path, scenarios_dir: &Path) -> ScenarioMeta {
    if path.exists() {
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        return ScenarioMeta { id, path: path.to_path_buf() };
    }

    let id = path.to_string_lossy().to_string();
    match scan_scenarios(scenarios_dir) {
        Ok(metas) => {
            if let Some(meta) = metas.into_iter().find(|m| m.id == id) {
                return meta;
            }
        }
        Err(e) => eprintln!("[runner] Cannot scan scenarios: {}", e),
    }

    eprintln!(
        "[runner] Scenario not found: '{}'. Use --list to see available scenarios.",
        path.display()
    );
    std::process::exit(1);
}
```

- [ ] **Step 2: Build binary**

```bash
cargo build --bin runner
```

Expected: компілюється без помилок.

- [ ] **Step 3: Smoke test — headless mode (раніше не ламається)**

```bash
cargo run --bin runner -- config/scenarios/single_cell_survival.toml
```

Expected: виводить статистику і завершується.

- [ ] **Step 4: Smoke test — serve mode**

```bash
# Запустити в окремому терміналі (або з &):
cargo run --bin runner -- --serve &
SERVER_PID=$!

sleep 2

curl -s http://127.0.0.1:8080/server/info | python -m json.tool
curl -s http://127.0.0.1:8080/scenarios | python -m json.tool
curl -s -X POST http://127.0.0.1:8080/run/start \
     -H "Content-Type: application/json" \
     -d '{"scenario_id":"single_cell_survival"}' | python -m json.tool
curl -s http://127.0.0.1:8080/run/status | python -m json.tool
curl -s -X POST http://127.0.0.1:8080/run/stop | python -m json.tool

kill $SERVER_PID
```

Expected:
- `/server/info` → JSON з engine_version, api_version: "1"
- `/scenarios` → масив зі сценаріями
- `/run/start` → `{"ok": true, ...}`
- `/run/status` → `{"state": "running", ...}`
- `/run/stop` → `{"ok": true}`

- [ ] **Step 5: Final workspace test**

```bash
cargo test --workspace
```

Expected: всі тести `PASS`.

- [ ] **Step 6: Commit**

```bash
git add src/bin/runner.rs
git commit -m "feat(runner): add --serve flag with tokio HTTP server"
```

---

## Task 7: Integration smoke test

**Files:**
- Create: `tests/runner_serve_smoke.rs`

> Цей тест реально прив'язується до порту і перевіряє end-to-end через reqwest.

- [ ] **Step 1: Додати reqwest у dev-dependencies**

У `Cargo.toml`:

```toml
[dev-dependencies]
tower = { version = "0.5", features = ["util"] }
http-body-util = "0.1"
reqwest = { version = "0.12", features = ["json"] }
```

- [ ] **Step 2: Write the integration test**

```rust
// tests/runner_serve_smoke.rs
//! Integration test: spawns a real HTTP server on a random port, sends requests.

use alife::runner::server_config::ServerConfig;
use alife::viewer_server::{create_app, state::new_app_state};
use std::path::PathBuf;

async fn spawn_test_server() -> (String, tokio::task::JoinHandle<()>) {
    let scenarios_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios");
    let state = new_app_state(scenarios_dir, 20);
    let app = create_app(state);

    // Bind to OS-assigned port
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{}", addr);

    let handle = tokio::spawn(async move {
        axum::serve(listener, app).await.ok();
    });

    (base_url, handle)
}

#[tokio::test]
async fn smoke_server_info_roundtrip() {
    let (base_url, _handle) = spawn_test_server().await;

    // Give server a moment to start
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    let resp = reqwest::get(format!("{}/server/info", base_url))
        .await
        .unwrap();
    assert_eq!(resp.status(), 200);

    let json: serde_json::Value = resp.json().await.unwrap();
    assert_eq!(json["api_version"].as_str().unwrap(), "1");
}

#[tokio::test]
async fn smoke_start_and_status_roundtrip() {
    let (base_url, _handle) = spawn_test_server().await;
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;

    let client = reqwest::Client::new();

    // Start
    let start_resp = client
        .post(format!("{}/run/start", base_url))
        .json(&serde_json::json!({ "scenario_id": "single_cell_survival" }))
        .send()
        .await
        .unwrap();
    assert_eq!(start_resp.status(), 200);
    let start_json: serde_json::Value = start_resp.json().await.unwrap();
    assert_eq!(start_json["ok"].as_bool().unwrap(), true);

    // Status
    let status_resp = client
        .get(format!("{}/run/status", base_url))
        .send()
        .await
        .unwrap();
    let status_json: serde_json::Value = status_resp.json().await.unwrap();
    assert_eq!(status_json["state"].as_str().unwrap(), "running");

    // Stop
    client
        .post(format!("{}/run/stop", base_url))
        .send()
        .await
        .unwrap();
}
```

- [ ] **Step 3: Run the integration test**

```bash
cargo test --test runner_serve_smoke
```

Expected: обидва тести `PASS`.

- [ ] **Step 4: Final workspace check**

```bash
cargo test --workspace
```

Expected: всі тести `PASS` (включно з runner_ring_buffer, runner_state_machine, runner_scenario_loader, runner_headless_e2e з Runner-1).

- [ ] **Step 5: Final commit**

```bash
git add tests/runner_serve_smoke.rs Cargo.toml
git commit -m "test(viewer-server): add real-port integration smoke test"
```

---

## Self-Review

### Spec coverage

| Вимога з Runner-2 spec | Реалізована? |
|---|---|
| `--serve` прапорець | ✅ Task 6 |
| `alife-viewer-server` crate skeleton | ✅ Task 2 (`src/viewer_server/`) |
| HTTP server (axum + tokio) | ✅ Task 6 |
| `config/server.toml` + default config | ✅ Task 1 |
| `GET /server/info` | ✅ Task 3 |
| `GET /scenarios` | ✅ Task 4 |
| `GET /scenarios/{id}` | ✅ Task 4 |
| `GET /run/status` | ✅ Task 5 |
| `POST /run/start` з valid scenario_id | ✅ Task 5 |
| `POST /run/start` → 409 якщо вже активний | ✅ Task 5 |
| `POST /run/pause` | ✅ Task 5 |
| `POST /run/resume` | ✅ Task 5 |
| `POST /run/step { ticks: N }` | ✅ Task 5 |
| `POST /run/stop` | ✅ Task 5 |
| integration tests через reqwest | ✅ Task 7 |
| config hash і seed у /run/status | ⚠️ config_hash = "tbd" — Runner-4 |
| `ticks_per_second` у /run/status | ⚠️ Runner-4 |

**Gaps:** config_hash та ticks_per_second позначені як Runner-4 hardening. Не блокують прийнятність Runner-2.

### Placeholder scan

Немає "TBD" без пояснення. config_hash відкрито позначений як Runner-4.

### Type consistency

- `RunState` — `Idle`/`Running`/`Paused` однакові у engine.rs і run.rs
- `AppState = Arc<Mutex<SharedState>>` — однакове у state.rs і всіх handlers
- `RunEngine::from_scenario(&meta, engine_cfg)` — відповідає Runner-1 API
- `engine.step(n)` — та сама сигнатура
- `TickLoopSignal::request_pause/resume/stop` — однакові у state.rs і run.rs

---

## Acceptance Gate

Цей slice вважається завершеним, коли:

```
cargo test --test runner_server_config        → всі 3 PASS
cargo test --test runner_http_info            → всі 3 PASS
cargo test --test runner_http_scenarios       → всі 5 PASS
cargo test --test runner_http_run_control     → всі 8 PASS
cargo test --test runner_serve_smoke          → обидва 2 PASS
cargo test --workspace                        → без регресій

cargo run --bin runner -- --serve             → стартує HTTP на 127.0.0.1:8080
curl http://127.0.0.1:8080/server/info        → JSON з api_version:"1"
curl http://127.0.0.1:8080/scenarios          → масив з сценаріями
POST /run/start single_cell_survival          → {"ok": true}
GET  /run/status                              → {"state": "running", ...}
POST /run/pause                               → {"ok": true, "tick": N}
POST /run/resume                              → {"ok": true}
POST /run/stop                                → {"ok": true}
POST /run/start коли вже Running             → HTTP 409
GET  /scenarios/nonexistent                   → HTTP 404
```
