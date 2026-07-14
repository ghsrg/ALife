# Runner Phase 2 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Додати `--serve` прапорець до runner binary, який стартує HTTP сервер (axum + tokio) з повним набором command API ендпоінтів: `/server/info`, `/scenarios`, `/scenarios/{id}`, `/run/status`, `/run/start`, `/run/pause`, `/run/resume`, `/run/step`, `/run/stop`.

**Architecture:** `src/viewer_server/` — новий adapter module (crate в межах workspace — пізніше, зараз internal module). HTTP handlers не володіють lifecycle, не стартують Core і не викликають Bootstrap напряму. Вони транслюють HTTP request/response у shared Runner command layer (`RunnerCommand` dispatcher) і читають versioned projections (`RunStatusProjection`, Scenario discovery DTOs). Shared server state може тримати `Arc<Mutex<RunnerControlState>>` або еквівалентний handle до Runner command dispatcher, але canonical lifecycle залишається у Runner: `RunnerProcessState` + `ActiveRunState`. Tick advancement виконується тільки через Runner-owned engine methods (`run_one_tick`, `step_one_paused`) з command validation. Тести використовують `tower::ServiceExt::oneshot()` без реального порту.

**Tech Stack:** `axum 0.8`, `tokio 1 (full)`, `tower` (dev-dep для тестування), `serde_json` (вже є). Всі endpoint-тести — `#[tokio::test]` async.

**Передумови:** Runner-1 завершений і Canon-compatible: `RunEngine`, `RunnerProcessState`, `ActiveRunState`, `RunnerCommand`, `ScenarioDocument`, `ScenarioHash`, `RunStatusProjection` або мінімальний projection DTO, `RunEngine::prepare_from_document`, `RunEngine::run_one_tick`, `RunEngine::step_one_paused`, `src/bin/runner.rs` — все існує. Legacy names such as `RunState`, `load_scenario`, direct TOML-to-Core startup, and multi-tick `engine.step(n)` are forbidden in this phase.

---

## Canon Supersession: Runner-2 HTTP Adapter

This plan is valid only after the Runner-1 Canon foundation exists:

```text
ScenarioDocument
scenario_hash
Bootstrap / PreparedWorld / BootstrapManifest
RunnerProcessState
ActiveRunState
RunnerCommand dispatcher
RunStatusProjection
```

HTTP handlers are adapters. They must translate request/response bodies to shared Runner commands and must not call Core or Bootstrap directly.

Required endpoint mapping:

```text
GET  /run/status -> GetRunStatus
POST /run/start  -> StartRun
POST /run/pause  -> PauseRun
POST /run/resume -> ResumeRun
POST /run/step   -> StepRun
POST /run/stop   -> StopRun
```

`POST /run/step` Canon contract:

```text
Request body: {}
Valid only when Active Run is Paused.
Executes exactly one committed Tick.
Returns active_run_state = "paused" and committed_tick.
Multi-tick advancement is intentionally out of scope for this Runner phase.
A future command may add bounded tick advancement, but it must not reuse StepRun semantics.
```

Status and start responses use Canon fields:

```text
process_state
active_run_state
run_id
committed_tick
scenario_hash
effective_seed
bootstrap_manifest summary
terminal_reason
```

Do not use `"tbd"` hashes. `scenario_hash` comes from canonical `ScenarioDocument`, created before Bootstrap. `config_hash` wording in older snippets means `scenario_hash` unless a later ADR creates a separate config hash.

Server-side `SharedState.snapshots: RingBuffer<CommittedSnapshot>` is not a public scroll-back contract. If retained temporarily, it is internal only and must not be exposed as seek/history behavior.

Additional acceptance:

```text
HTTP StartRun failure during Scenario Resolution or Bootstrap returns stable Runner error category
HTTP invalid command returns state_conflict without changing state
HTTP /run/step from Running returns 409 state_conflict
HTTP /run/step from Paused commits exactly one Tick and remains Paused
```

---

## File Structure

```
src/
  viewer_server/
    mod.rs           [NEW] — pub mod state, api; pub fn create_app
    state.rs         [NEW] — HTTP adapter state, RunnerCommandHandle, TickLoopSignal if still needed
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
  server.toml        [NEW] — bind_host, port, allow_remote_viewer, target_broadcast_fps
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
    assert_eq!(cfg.target_broadcast_fps, 30);
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
target_broadcast_fps = 30
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
    /// Max WS frame push rate for Runner-3. Parsed here so HTTP and WS phases share one config.
    /// Runner-2 does not implement WS streaming yet.
    pub target_broadcast_fps: u32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            port: 8080,
            allow_remote_viewer: false,
            target_broadcast_fps: 30,
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
use crate::runner::commands::RunnerCommand;
use crate::runner::engine::{RunEngine, RunEngineConfig, RunEngineError};
use crate::runner::lifecycle::{ActiveRunState, RunnerProcessState};
use crate::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicBool, Ordering};
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

/// Transport-independent command result used by HTTP adapters.
/// Runner-2 may replace this with a richer enum, but handlers must keep this boundary:
/// HTTP -> RunnerCommand -> projection/command result -> HTTP response.
pub struct RunnerCommandResult {
    pub process_state: RunnerProcessState,
    pub active_run_state: ActiveRunState,
    pub run_id: Option<String>,
    pub committed_tick: u64,
    pub scenario_hash: Option<String>,
    pub effective_seed: Option<u64>,
    pub terminal_reason: Option<String>,
}

/// Server adapter state shared between HTTP handlers and the Runner-owned tick loop.
/// This type is not the canonical lifecycle. It only holds the active Runner engine handle
/// and delegates every control operation through RunnerCommand validation.
pub struct SharedState {
    pub engine: Option<RunEngine>,
    pub process_state: RunnerProcessState,
    pub active_run_state: ActiveRunState,
    pub run_id: Option<String>,
    pub scenario_hash: Option<String>,
    pub effective_seed: Option<u64>,
    /// Scenario ID of the active run.
    pub scenario_id: Option<String>,
    /// Tick count from the last completed tick.
    pub committed_tick: u64,
    pub terminal_reason: Option<String>,
    /// Path to config/scenarios/ directory.
    pub scenarios_dir: PathBuf,
    /// Loop signal — Some when a tick loop thread is running.
    pub tick_signal: Option<Arc<TickLoopSignal>>,
    /// Internal RunEngine snapshot capacity. Not a public HTTP/WS scroll-back contract.
    pub engine_snapshot_buffer_size: usize,
}

impl SharedState {
    pub fn new(scenarios_dir: PathBuf, engine_snapshot_buffer_size: usize) -> Self {
        Self {
            engine: None,
            process_state: RunnerProcessState::Ready,
            active_run_state: ActiveRunState::Idle,
            run_id: None,
            scenario_hash: None,
            effective_seed: None,
            scenario_id: None,
            committed_tick: 0,
            terminal_reason: None,
            scenarios_dir,
            tick_signal: None,
            engine_snapshot_buffer_size,
        }
    }

    /// Check whether a run is already active (Running or Paused).
    pub fn is_active(&self) -> bool {
        matches!(
            self.active_run_state,
            ActiveRunState::Preparing | ActiveRunState::Running | ActiveRunState::Paused | ActiveRunState::Stopping
        )
    }

    pub fn status_projection(&self) -> RunnerCommandResult {
        RunnerCommandResult {
            process_state: self.process_state,
            active_run_state: self.active_run_state,
            run_id: self.run_id.clone(),
            committed_tick: self.committed_tick,
            scenario_hash: self.scenario_hash.clone(),
            effective_seed: self.effective_seed,
            terminal_reason: self.terminal_reason.clone(),
        }
    }
}

/// Type alias used by axum handlers.
pub type AppState = Arc<Mutex<SharedState>>;

/// Build a new AppState.
pub fn new_app_state(scenarios_dir: PathBuf, engine_snapshot_buffer_size: usize) -> AppState {
    Arc::new(Mutex::new(SharedState::new(
        scenarios_dir,
        engine_snapshot_buffer_size,
    )))
}

/// Resolve a scenario id/path into the canonical ScenarioDocument before Bootstrap.
pub fn resolve_scenario_document(state: &SharedState, scenario_id_or_path: &str) -> Result<ScenarioDocument, String> {
    let candidate = Path::new(scenario_id_or_path);
    let source_path = if candidate.exists() {
        candidate.to_path_buf()
    } else {
        state.scenarios_dir.join(format!("{scenario_id_or_path}.toml"))
    };
    ScenarioDocument::resolve(ScenarioSource::Path(source_path)).map_err(|err| err.to_string())
}

/// Dispatch a Runner command. HTTP handlers call this function instead of mutating state directly.
pub fn dispatch_command(state: &AppState, command: RunnerCommand) -> Result<RunnerCommandResult, String> {
    let mut locked = state.lock().unwrap();
    command
        .validate(locked.active_run_state)
        .map_err(|_| "state_conflict".to_string())?;

    match command {
        RunnerCommand::GetRunStatus => Ok(locked.status_projection()),
        RunnerCommand::PauseRun => {
            let engine = locked.engine.as_mut().ok_or_else(|| "run_not_found".to_string())?;
            engine.pause().map_err(|err| err.to_string())?;
            locked.active_run_state = ActiveRunState::Paused;
            if let Some(signal) = &locked.tick_signal {
                signal.request_pause();
            }
            Ok(locked.status_projection())
        }
        RunnerCommand::ResumeRun => {
            let engine = locked.engine.as_mut().ok_or_else(|| "run_not_found".to_string())?;
            engine.resume().map_err(|err| err.to_string())?;
            locked.active_run_state = ActiveRunState::Running;
            if let Some(signal) = &locked.tick_signal {
                signal.request_resume();
            }
            Ok(locked.status_projection())
        }
        RunnerCommand::StepRun => {
            let engine = locked.engine.as_mut().ok_or_else(|| "run_not_found".to_string())?;
            engine.step_one_paused().map_err(|err| err.to_string())?;
            locked.committed_tick = engine.current_tick();
            locked.active_run_state = ActiveRunState::Paused;
            Ok(locked.status_projection())
        }
        RunnerCommand::StopRun => {
            if let Some(signal) = &locked.tick_signal {
                signal.request_stop();
            }
            if let Some(engine) = locked.engine.as_mut() {
                engine.stop().map_err(|err| err.to_string())?;
            }
            locked.active_run_state = ActiveRunState::Completed;
            locked.engine = None;
            locked.tick_signal = None;
            Ok(locked.status_projection())
        }
        RunnerCommand::ValidateScenario | RunnerCommand::PrepareScenario | RunnerCommand::StartRun => {
            Err("use dedicated handlers because these commands require scenario input".to_string())
        }
    }
}

/// Spawn the background tick loop. Stores the signal in `state.tick_signal`.
/// Assumes engine is already initialised and active_run_state == Running.
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
                    let r = engine.run_one_tick();
                    if r.is_ok() {
                        locked.committed_tick = engine.current_tick();
                    }
                    r
                } else {
                    break;
                }
            };

            if result.is_err() {
                let mut locked = state.lock().unwrap();
                locked.active_run_state = ActiveRunState::Failed;
                locked.terminal_reason = Some("core_error".to_string());
                break;
            }
        }

        // Ensure state is cleaned up
        let mut locked = state.lock().unwrap();
        if matches!(locked.active_run_state, ActiveRunState::Running) {
            locked.active_run_state = ActiveRunState::Completed;
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
    assert_eq!(json["active_run_state"].as_str().unwrap(), "idle");
    assert_eq!(json["committed_tick"].as_u64().unwrap(), 0);
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
    assert!(json.get("scenario_hash").is_some());
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
    assert_eq!(json["active_run_state"].as_str().unwrap(), "running");
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
async fn post_run_step_executes_exactly_one_tick_from_paused() {
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

    // Pause first; StepRun is valid only while ActiveRunState is Paused.
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

    let tick_before = {
        let response = create_app(state.clone())
            .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = response.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
        json["committed_tick"].as_u64().unwrap()
    };

    // StepRun executes exactly one committed Tick and returns Paused.
    let response = create_app(state.clone())
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

    let body = response.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["active_run_state"].as_str().unwrap(), "paused");
    assert_eq!(json["committed_tick"].as_u64().unwrap(), tick_before + 1);
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

use crate::runner::commands::RunnerCommand;
use crate::runner::engine::{RunEngine, RunEngineConfig};
use crate::runner::lifecycle::ActiveRunState;
use crate::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
use crate::viewer_server::state::{AppState, dispatch_command, spawn_tick_loop};

// ── Status ────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct RunStatus {
    process_state: String,
    active_run_state: String,
    committed_tick: u64,
    scenario_id: Option<String>,
    scenario_hash: Option<String>,
    effective_seed: Option<u64>,
    terminal_reason: Option<String>,
}

fn active_state_label(s: ActiveRunState) -> &'static str {
    match s {
        ActiveRunState::Idle => "idle",
        ActiveRunState::Preparing => "preparing",
        ActiveRunState::Running => "running",
        ActiveRunState::Paused => "paused",
        ActiveRunState::Stopping => "stopping",
        ActiveRunState::Completed => "completed",
        ActiveRunState::Failed => "failed",
    }
}

async fn handle_run_status(State(state): State<AppState>) -> Json<RunStatus> {
    let locked = state.lock().unwrap();
    Json(RunStatus {
        process_state: format!("{:?}", locked.process_state).to_lowercase(),
        active_run_state: active_state_label(locked.active_run_state).to_string(),
        committed_tick: locked.committed_tick,
        scenario_id: locked.scenario_id.clone(),
        scenario_hash: locked.scenario_hash.clone(),
        effective_seed: locked.effective_seed,
        terminal_reason: locked.terminal_reason.clone(),
    })
}

// ── Start ─────────────────────────────────────────────────────────────────────

#[derive(Deserialize)]
struct StartRequest {
    scenario_id: String,
    seed_override: Option<u64>,
    request_id: Option<String>,
}

#[derive(Serialize)]
struct StartResponse {
    ok: bool,
    run_id: String,
    scenario_hash: String,
    effective_seed: u64,
    active_run_state: String,
    bootstrap_manifest: serde_json::Value,
}

#[derive(Serialize)]
struct ErrorResponse {
    ok: bool,
    category: String,
    message: String,
    current_state: String,
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
                    category: "state_conflict".to_string(),
                    message: format!("Run already active in state: {:?}", locked.active_run_state),
                    current_state: active_state_label(locked.active_run_state).to_string(),
                }),
            ));
        }
    }

    // Resolve ScenarioDocument before Bootstrap. HTTP must not call Core/Bootstrap directly
    // and must not keep path/request metadata in the canonical document.
    let scenarios_dir = state.lock().unwrap().scenarios_dir.clone();
    let scenario_path = scenarios_dir.join(format!("{}.toml", req.scenario_id));
    let document = ScenarioDocument::resolve(ScenarioSource::Path(scenario_path)).map_err(|e| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                ok: false,
                category: "scenario_error".to_string(),
                message: format!("Failed to resolve scenario: {}", e),
                current_state: "idle".to_string(),
            }),
        )
    })?;

    let scenario_hash = document.scenario_hash.to_string();
    let effective_seed = req.seed_override.unwrap_or(document.runtime_config.seed.raw());
    let engine_cfg = RunEngineConfig { snapshot_buffer_size: 300 };

    let mut engine = RunEngine::prepare_from_document(&document, engine_cfg).map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                category: "bootstrap_error".to_string(),
                message: format!("Failed to prepare scenario: {}", e),
                current_state: "preparing".to_string(),
            }),
        )
    })?;

    engine.start().map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ErrorResponse {
                ok: false,
                category: "core_error".to_string(),
                message: format!("Failed to start engine: {}", e),
                current_state: "preparing".to_string(),
            }),
        )
    })?;

    {
        let mut locked = state.lock().unwrap();
        locked.engine = Some(engine);
        locked.active_run_state = ActiveRunState::Running;
        locked.run_id = Some(req.request_id.unwrap_or_else(|| format!("run-{}", scenario_hash)));
        locked.scenario_hash = Some(scenario_hash.clone());
        locked.effective_seed = Some(effective_seed);
        locked.scenario_id = Some(req.scenario_id);
        locked.committed_tick = 0;
        locked.terminal_reason = None;
    }

    // Spawn background tick loop
    spawn_tick_loop(state);

    Ok(Json(StartResponse {
        ok: true,
        run_id: state.lock().unwrap().run_id.clone().unwrap(),
        scenario_hash,
        effective_seed,
        active_run_state: "running".to_string(),
        bootstrap_manifest: serde_json::json!({
            "available": true,
            "source": "PreparedWorld created before Core start"
        }),
    }))
}

// ── Pause ─────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct PauseResponse {
    ok: bool,
    active_run_state: String,
    committed_tick: u64,
}

async fn handle_run_pause(
    State(state): State<AppState>,
) -> Result<Json<PauseResponse>, (StatusCode, Json<ErrorResponse>)> {
    let projection = dispatch_command(&state, RunnerCommand::PauseRun).map_err(|category| {
        (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                ok: false,
                category,
                message: "Cannot pause from the current active_run_state".to_string(),
                current_state: "see /run/status".to_string(),
            }),
        )
    })?;
    Ok(Json(PauseResponse {
        ok: true,
        active_run_state: active_state_label(projection.active_run_state).to_string(),
        committed_tick: projection.committed_tick,
    }))
}

// ── Resume ────────────────────────────────────────────────────────────────────

async fn handle_run_resume(
    State(state): State<AppState>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<ErrorResponse>)> {
    let projection = dispatch_command(&state, RunnerCommand::ResumeRun).map_err(|category| {
        (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                ok: false,
                category,
                message: "Cannot resume from the current active_run_state".to_string(),
                current_state: "see /run/status".to_string(),
            }),
        )
    })?;
    Ok(Json(serde_json::json!({
        "ok": true,
        "active_run_state": active_state_label(projection.active_run_state),
        "committed_tick": projection.committed_tick
    })))
}

// ── Step ──────────────────────────────────────────────────────────────────────

#[derive(Serialize)]
struct StepResponse {
    ok: bool,
    active_run_state: String,
    committed_tick: u64,
}

async fn handle_run_step(
    State(state): State<AppState>,
) -> Result<Json<StepResponse>, (StatusCode, Json<ErrorResponse>)> {
    let projection = dispatch_command(&state, RunnerCommand::StepRun).map_err(|category| {
        (
            StatusCode::CONFLICT,
            Json(ErrorResponse {
                ok: false,
                category,
                message: "StepRun is valid only when active_run_state is paused".to_string(),
                current_state: "see /run/status".to_string(),
            }),
        )
    })?;
    Ok(Json(StepResponse {
        ok: true,
        active_run_state: active_state_label(projection.active_run_state).to_string(),
        committed_tick: projection.committed_tick,
    }))
}

// ── Stop ──────────────────────────────────────────────────────────────────────

async fn handle_run_stop(State(state): State<AppState>) -> Json<serde_json::Value> {
    match dispatch_command(&state, RunnerCommand::StopRun) {
        Ok(projection) => Json(serde_json::json!({
            "ok": true,
            "active_run_state": active_state_label(projection.active_run_state),
            "committed_tick": projection.committed_tick
        })),
        Err(category) => Json(serde_json::json!({
            "ok": false,
            "category": category
        })),
    }
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
use alife::runner::scenario_doc::{ScenarioDocument, ScenarioSource};
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
    let state = new_app_state(scenarios_dir, 300);
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

    let document = match ScenarioDocument::resolve(ScenarioSource::Path(meta.path.clone())) {
        Ok(document) => document,
        Err(e) => {
            eprintln!("[runner] Failed to resolve scenario: {}", e);
            std::process::exit(1);
        }
    };

    let mut engine = match RunEngine::prepare_from_document(&document, engine_cfg) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("[runner] Failed to prepare scenario: {}", e);
            std::process::exit(1);
        }
    };

    engine.start().expect("RunEngine::start failed");

    let total_ticks = engine.max_ticks();

    let start = std::time::Instant::now();
    println!("[runner] Running {} ticks...", total_ticks);

    while engine.current_tick() < total_ticks {
        engine.run_one_tick().unwrap_or_else(|e| {
            eprintln!("[runner] Simulation error at tick {}: {}", engine.current_tick(), e);
        });
    }

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
| `POST /run/step {}` exactly one committed Tick from Paused | ✅ Task 5 + Canon Supersession |
| `POST /run/stop` | ✅ Task 5 |
| integration tests через reqwest | ✅ Task 7 |
| scenario_hash і seed у /run/status | ✅ from Runner-1 ScenarioDocument + Runner-2 GetRunStatusProjection |
| `ticks_per_second` у /run/status | ⚠️ Runner-4 |

**Gaps:** `ticks_per_second` позначений як Runner-4 hardening. Не блокує прийнятність Runner-2.

### Placeholder scan

Немає `"tbd"` hashes. Усі response fields використовують `scenario_hash` з canonical `ScenarioDocument`.

### Type consistency

- `RunnerProcessState` / `ActiveRunState` — Canon lifecycle model from `docs/runner/run-lifecycle.md`
- `AppState = Arc<Mutex<SharedState>>` — однакове у state.rs і всіх handlers
- `RunEngine::from_scenario(&meta, engine_cfg)` — відповідає Runner-1 API
- `RunEngine::run_one_tick()` for Running loop; `RunEngine::step_one_paused()` for `StepRun`
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
POST /run/step from Paused                    → exactly one committed Tick, remains Paused
POST /run/step from Running                   → HTTP 409 state_conflict
POST /run/stop                                → {"ok": true}
POST /run/start коли вже Running             → HTTP 409
GET  /scenarios/nonexistent                   → HTTP 404
```
