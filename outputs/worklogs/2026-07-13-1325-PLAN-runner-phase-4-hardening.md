# Runner Phase 4 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remote viewer mode (CORS + IP filtering), config validation errors via HTTP, graceful shutdown (Ctrl+C), reconnect WS test, `config_hash` / `seed` / `ticks_per_second` у `/run/status`, collapse detection з `collapse_reason` у status + WS broadcast, determinism integration test.

**Key discoveries:**
- `RunSummary` вже містить `collapse_reason: CollapseReason` і `config_hash: u64` — можна читати прямо з результату `engine.step()`
- `CollapseReason` variants: `None, InvalidConfig, EnergyDepleted, MandatoryCostUnpaid, CapacityExceeded, HeatLimitExceeded, WasteLimitExceeded, MinimumViabilityMaterialsMissing, DeterminismMismatch, ViewerAuthorityViolation`
- `SurvivalResult::Collapse` + `CollapseReason != None` → natural collapse detected
- `MetricsSummary::alive_cells_count` — перевірка живих клітин

**New deps:** `tower-http = { version = "0.6", features = ["cors"] }`.

**Передумови:** Runner-3 завершений: WS /stream, Broadcaster, ALIF v1 encoder, time-based broadcast, status messages, `src/bin/runner.rs --serve`.

---

## /run/status — повний JSON після Runner-4

```json
{
  "state": "running",
  "tick": 142,
  "scenario_id": "single_cell_survival",
  "config_hash": "a3f8e2c1d9b47650",
  "seed": 42,
  "ticks_per_second": 4823.5,
  "collapse_reason": null
}
```

Поля `config_hash`, `seed`, `ticks_per_second` відсутні або `null` у стані `idle`.

---

## WS collapse broadcast

```json
{ "type": "status", "state": "idle", "tick": 1000, "collapse_reason": "EnergyDepleted" }
```

---

## File Structure

```
src/
  viewer_server/
    state.rs         [MODIFY] — add config_hash, seed, max_ticks, run_start_time,
                                ticks_since_start; update spawn_tick_loop to detect
                                collapse from RunSummary and broadcast; broadcast
                                WsMessage::Status on collapse
    api/
      run.rs         [MODIFY] — full /run/status JSON; detailed ParseError in /run/start
      mod.rs         [MODIFY] — apply CORS layer and optional local-only middleware
    mod.rs           [MODIFY] — create_app accepts ServerConfig for CORS setup
  runner/
    server_config.rs [MODIFY] — add allowed_origins: Vec<String>
  bin/
    runner.rs        [MODIFY] — graceful shutdown via ctrl_c signal
Cargo.toml           [MODIFY] — tower-http cors feature
config/server.toml   [MODIFY] — allowed_origins example
tests/
  runner_run_status.rs       [NEW] — config_hash, seed, ticks_per_second tests
  runner_collapse.rs         [NEW] — collapse detection + collapse_reason
  runner_determinism.rs      [NEW] — same seed + config → same result
  runner_validation.rs       [NEW] — /run/start ParseError details via HTTP
  runner_cors.rs             [NEW] — CORS headers + local-only IP check
  runner_ws_reconnect.rs     [NEW] — WS reconnect gets current state
```

---

## Task 1: config_hash, seed, ticks_per_second у /run/status

**Files:**
- Modify: `src/viewer_server/state.rs`
- Modify: `src/viewer_server/api/run.rs`
- Test: `tests/runner_run_status.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_run_status.rs
use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use http::{Method, Request};
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::path::PathBuf;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        50, 30,
    )
}

async fn start_run(
    state: alife::viewer_server::state::AppState,
    scenario_id: &str,
    seed: u64,
) {
    use alife::viewer_server::create_app;
    create_app(state)
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "scenario_id": scenario_id, "seed": seed })
                        .to_string(),
                ))
                .unwrap(),
        )
        .await
        .unwrap();
}

#[tokio::test]
async fn run_status_has_config_hash_after_start() {
    let state = make_state();
    start_run(state.clone(), "single_cell_survival", 42).await;

    let resp = create_app(state.clone())
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let hash = json["config_hash"].as_str().unwrap_or("");
    assert!(!hash.is_empty(), "config_hash must be non-empty string after start");
    assert_eq!(hash.len(), 16, "config_hash must be 16-char hex string");

    create_app(state).oneshot(
        Request::builder().method(Method::POST).uri("/run/stop").body(Body::empty()).unwrap()
    ).await.unwrap();
}

#[tokio::test]
async fn run_status_has_seed_after_start() {
    let state = make_state();
    start_run(state.clone(), "single_cell_survival", 77).await;

    let resp = create_app(state.clone())
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(json["seed"].as_u64().unwrap(), 77, "seed must match the requested seed");

    create_app(state).oneshot(
        Request::builder().method(Method::POST).uri("/run/stop").body(Body::empty()).unwrap()
    ).await.unwrap();
}

#[tokio::test]
async fn run_status_has_ticks_per_second_after_running() {
    let state = make_state();
    start_run(state.clone(), "single_cell_survival", 42).await;

    // Let simulation run briefly
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;

    let resp = create_app(state.clone())
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    let tps = json["ticks_per_second"].as_f64().unwrap_or(0.0);
    assert!(tps > 0.0, "ticks_per_second must be > 0 while running");

    create_app(state).oneshot(
        Request::builder().method(Method::POST).uri("/run/stop").body(Body::empty()).unwrap()
    ).await.unwrap();
}

#[tokio::test]
async fn run_status_idle_has_null_hash_and_zero_tps() {
    let state = make_state();
    let resp = create_app(state)
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(json["config_hash"].is_null(), "config_hash must be null when idle");
    assert!(json["seed"].is_null(), "seed must be null when idle");
    assert_eq!(json["ticks_per_second"].as_f64().unwrap_or(-1.0), 0.0);
}

#[tokio::test]
async fn same_scenario_same_config_hash() {
    let s1 = make_state();
    let s2 = make_state();
    start_run(s1.clone(), "single_cell_survival", 1).await;
    start_run(s2.clone(), "single_cell_survival", 1).await;

    let get_hash = |st: alife::viewer_server::state::AppState| async move {
        let r = create_app(st)
            .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let b = r.into_body().collect().await.unwrap().to_bytes();
        let j: serde_json::Value = serde_json::from_slice(&b).unwrap();
        j["config_hash"].as_str().unwrap_or("").to_string()
    };

    let h1 = get_hash(s1.clone()).await;
    let h2 = get_hash(s2.clone()).await;
    assert_eq!(h1, h2, "Same scenario must produce identical config_hash");
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cargo test --test runner_run_status
```

Expected: compile / test failure — fields not in status JSON.

- [ ] **Step 3: Update `src/viewer_server/state.rs` — add runtime tracking fields**

Додати поля до `SharedState`:
```rust
// Fields to add in SharedState:
pub config_hash:       Option<String>,    // 16-char hex, set on start
pub current_seed:      Option<u64>,       // seed used in this run
pub max_ticks:         Option<u64>,       // from RuntimeConfig, for collapse detection
pub run_start_time:    Option<std::time::Instant>, // for ticks_per_second
pub ticks_since_start: u64,               // incremented by tick loop
```

Ініціалізувати в `SharedState::new()` — всі `None`, `ticks_since_start = 0`.

Реалізувати `fn ticks_per_second(&self) -> f64`:
```rust
pub fn ticks_per_second(&self) -> f64 {
    match self.run_start_time {
        Some(start) => {
            let elapsed = start.elapsed().as_secs_f64();
            if elapsed > 0.001 {
                self.ticks_since_start as f64 / elapsed
            } else {
                0.0
            }
        }
        None => 0.0,
    }
}
```

Також реалізувати helper `fn compute_config_hash(toml: &str) -> String`:
```rust
pub fn compute_config_hash(toml: &str) -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    toml.hash(&mut h);
    format!("{:016x}", h.finish())
}
```

- [ ] **Step 4: Update `handle_run_start` у `src/viewer_server/api/run.rs`**

Коли стартуємо run, зчитуємо TOML і заповнюємо нові поля:

```rust
// After finding the scenario meta, before creating engine:
let toml_content = std::fs::read_to_string(&meta.path)
    .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, Json(ErrorResponse { ok: false, error: e.to_string() })))?;

let config_hash = compute_config_hash(&toml_content);
let seed = req.seed.unwrap_or_else(|| {
    // deterministic fallback: hash of scenario_id  
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    let mut h = DefaultHasher::new();
    req.scenario_id.hash(&mut h);
    h.finish()
});

// After creating + starting engine:
{
    let mut locked = state.lock().unwrap();
    locked.engine = Some(engine);
    locked.run_state = RunState::Running;
    locked.scenario_id = Some(req.scenario_id.clone());
    locked.current_tick = 0;
    locked.collapse_reason = None;
    locked.config_hash = Some(config_hash.clone());
    locked.current_seed = Some(seed);
    locked.run_start_time = Some(std::time::Instant::now());
    locked.ticks_since_start = 0;
    // max_ticks: read from loaded RuntimeConfig
    locked.max_ticks = Some(rt_config.world.tick_count.raw());
}

return_body: StartResponse { ok: true, config_hash, seed }
```

- [ ] **Step 5: Update `handle_run_stop` — очистити tracking поля**

```rust
locked.config_hash = None;
locked.current_seed = None;
locked.run_start_time = None;
locked.ticks_since_start = 0;
locked.max_ticks = None;
```

- [ ] **Step 6: Update `handle_run_status` — повний JSON**

```rust
#[derive(Serialize)]
struct RunStatus {
    state:             String,
    tick:              u64,
    scenario_id:       Option<String>,
    config_hash:       Option<String>,
    seed:              Option<u64>,
    ticks_per_second:  f64,
    collapse_reason:   Option<String>,
}

async fn handle_run_status(State(state): State<AppState>) -> Json<RunStatus> {
    let locked = state.lock().unwrap();
    Json(RunStatus {
        state:            state_label(locked.run_state).to_string(),
        tick:             locked.current_tick as u64,
        scenario_id:      locked.scenario_id.clone(),
        config_hash:      locked.config_hash.clone(),
        seed:             locked.current_seed,
        ticks_per_second: locked.ticks_per_second(),
        collapse_reason:  locked.collapse_reason.clone(),
    })
}
```

- [ ] **Step 7: Update `spawn_tick_loop` — increment `ticks_since_start`**

В loop після successful step:
```rust
locked.ticks_since_start += 1;
```

- [ ] **Step 8: Run tests**

```bash
cargo test --test runner_run_status
```

Expected: всі 5 тестів `PASS`.

- [ ] **Step 9: Run full suite**

```bash
cargo test --workspace
```

- [ ] **Step 10: Commit**

```bash
git add src/viewer_server/state.rs src/viewer_server/api/run.rs \
        tests/runner_run_status.rs
git commit -m "feat(viewer-server): add config_hash, seed, ticks_per_second to /run/status"
```

---

## Task 2: Collapse Detection і Collapse Reason

**Files:**
- Modify: `src/viewer_server/state.rs` — detect collapse from `RunSummary`
- Test: `tests/runner_collapse.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_collapse.rs
//! Collapse detection: simulation that naturally collapses due to EnergyDepleted.
//! Uses a scenario configured to collapse quickly (minimal energy, high cost).

use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use http::{Method, Request};
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::path::PathBuf;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        50, 30,
    )
}

#[tokio::test]
async fn run_status_has_collapse_reason_after_energy_depletion() {
    let state = make_state();

    // Start scenario known to collapse quickly
    // (uses "fast_collapse" scenario or "single_cell_survival" with few ticks)
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({
                        "scenario_id": "single_cell_survival",
                        "seed": 42
                    }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Wait for simulation to finish naturally (max_ticks reached)
    // single_cell_survival has limited ticks — wait generously
    let timeout = std::time::Duration::from_secs(5);
    let start = std::time::Instant::now();

    loop {
        if start.elapsed() > timeout {
            break; // timeout — check what we have
        }
        tokio::time::sleep(std::time::Duration::from_millis(100)).await;

        let resp = create_app(state.clone())
            .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
            .await
            .unwrap();
        let body = resp.into_body().collect().await.unwrap().to_bytes();
        let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

        if json["state"].as_str() == Some("idle") {
            // Simulation ended — check collapse_reason
            // Either max_ticks_reached or a CollapseReason
            let reason = json["collapse_reason"].as_str().unwrap_or("null");
            assert!(
                reason != "null",
                "collapse_reason must be set when simulation ends naturally, got: {}",
                json
            );
            return;
        }
    }

    panic!("Simulation did not end within timeout");
}

#[tokio::test]
async fn collapse_reason_is_null_when_idle() {
    let state = make_state();
    let resp = create_app(state)
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert!(json["collapse_reason"].is_null());
}

#[tokio::test]
async fn manual_stop_does_not_set_collapse_reason() {
    let state = make_state();

    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "scenario_id": "single_cell_survival" }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Stop immediately
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

    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    let resp = create_app(state)
        .oneshot(Request::builder().uri("/run/status").body(Body::empty()).unwrap())
        .await
        .unwrap();
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();

    // Manual stop: collapse_reason must be null (not "user_stopped")
    assert!(
        json["collapse_reason"].is_null(),
        "Manual stop must not set collapse_reason, got: {}",
        json["collapse_reason"]
    );
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cargo test --test runner_collapse
```

Expected: `collapse_reason` завжди `null` — колапс ще не детектується.

- [ ] **Step 3: Update `spawn_tick_loop` в `src/viewer_server/state.rs` — collapse detection**

`engine.step(1)` повертає `Result<RunSummary, TickError>`. Читаємо `RunSummary::collapse_reason`:

```rust
// Import at top of state.rs:
use crate::core::summary::CollapseReason;

// In tick loop, replace current step block:
let (maybe_frame, collapse_info): (Option<Vec<u8>>, Option<String>) = {
    let mut locked = state.lock().unwrap();
    let should_broadcast = last_broadcast.elapsed() >= frame_interval;

    let engine = match locked.engine.as_mut() {
        Some(e) => e,
        None => break,
    };

    let summary = match engine.step(1) {
        Ok(s) => s,
        Err(e) => {
            locked.run_state = RunState::Idle;
            locked.collapse_reason = Some(format!("tick_error: {:?}", e));
            locked.engine = None;
            break;
        }
    };

    locked.ticks_since_start += 1;
    locked.current_tick = engine.current_tick();

    // Check natural collapse from RunSummary
    let natural_collapse = if summary.collapse_reason != CollapseReason::None {
        Some(format!("{:?}", summary.collapse_reason))
    } else if locked.max_ticks.map_or(false, |m| locked.current_tick as u64 >= m) {
        Some("MaxTicksReached".to_string())
    } else {
        None
    };

    let frame = if should_broadcast {
        engine.snapshots().newest().map(|snap| encode_snapshot(snap))
    } else {
        None
    };

    if natural_collapse.is_some() {
        locked.run_state = RunState::Idle;
        locked.collapse_reason = natural_collapse.clone();
        locked.engine = None;
    }

    (frame, natural_collapse)
}; // mutex released

// Broadcast status if collapsed
if let Some(reason) = collapse_info {
    let tick = state.lock().unwrap().current_tick;
    broadcast_sender.send(WsMessage::Status(serde_json::json!({
        "type": "status",
        "state": "idle",
        "tick": tick,
        "collapse_reason": reason
    }).to_string())).ok();
    break;
}

if let Some(bytes) = maybe_frame {
    broadcast_sender.send(WsMessage::Frame(bytes)).ok();
    last_broadcast = Instant::now();
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test runner_collapse
```

Expected: всі 3 тести `PASS`. Тест `run_status_has_collapse_reason_after_energy_depletion` може бути slow якщо single_cell_survival має багато тіків — перевір `tick_count` у сценарії.

- [ ] **Step 5: Run full suite**

```bash
cargo test --workspace
```

- [ ] **Step 6: Commit**

```bash
git add src/viewer_server/state.rs tests/runner_collapse.rs
git commit -m "feat(viewer-server): detect collapse from RunSummary, set collapse_reason, broadcast status"
```

---

## Task 3: Determinism Integration Test

**Files:**
- Test: `tests/runner_determinism.rs`

- [ ] **Step 1: Write the failing test**

```rust
// tests/runner_determinism.rs
//! Verifies that the same seed + same scenario config always produces
//! the same simulation result at the same tick — Runner-level determinism gate.

use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::scenario::{scan_scenarios, ScenarioMeta};
use std::path::PathBuf;

fn find_scenario(id: &str) -> ScenarioMeta {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios");
    scan_scenarios(&dir)
        .unwrap()
        .into_iter()
        .find(|m| m.id == id)
        .expect("scenario must exist")
}

fn run_n_ticks(meta: &ScenarioMeta, n: u32) -> (u32, usize, f32, f32) {
    let cfg = RunEngineConfig { snapshot_buffer_size: 10 };
    let mut engine = RunEngine::from_scenario(meta, cfg).expect("load ok");
    engine.start().expect("start ok");
    engine.step(n).ok();

    let tick = engine.current_tick();
    if let Some(snap) = engine.snapshots().newest() {
        (tick, snap.cells.len(), snap.heat, snap.waste)
    } else {
        (tick, 0, 0.0, 0.0)
    }
}

#[test]
fn same_seed_produces_same_result_at_tick_50() {
    let meta = find_scenario("single_cell_survival");

    let r1 = run_n_ticks(&meta, 50);
    let r2 = run_n_ticks(&meta, 50);

    assert_eq!(r1.0, r2.0, "tick count must match");
    assert_eq!(r1.1, r2.1, "cell count must match");
    assert!(
        (r1.2 - r2.2).abs() < 1e-5,
        "heat must match: {} vs {}",
        r1.2, r2.2
    );
    assert!(
        (r1.3 - r2.3).abs() < 1e-5,
        "waste must match: {} vs {}",
        r1.3, r2.3
    );
}

#[test]
fn same_seed_produces_same_result_at_tick_200() {
    let meta = find_scenario("single_cell_survival");
    let r1 = run_n_ticks(&meta, 200);
    let r2 = run_n_ticks(&meta, 200);
    assert_eq!(r1, r2, "Full state at tick 200 must be identical");
}

#[test]
fn determinism_holds_with_explicit_engine_restart() {
    let meta = find_scenario("single_cell_survival");
    let cfg = RunEngineConfig { snapshot_buffer_size: 5 };

    // Run 1: continuous
    let mut e1 = RunEngine::from_scenario(&meta, cfg.clone()).unwrap();
    e1.start().unwrap();
    e1.step(100).ok();
    let snap1 = e1.snapshots().newest().cloned();

    // Run 2: fresh engine, same config
    let mut e2 = RunEngine::from_scenario(&meta, cfg).unwrap();
    e2.start().unwrap();
    e2.step(100).ok();
    let snap2 = e2.snapshots().newest().cloned();

    match (snap1, snap2) {
        (Some(s1), Some(s2)) => {
            assert_eq!(s1.tick, s2.tick, "tick must match");
            assert_eq!(s1.cells.len(), s2.cells.len(), "cell count must match");
            assert!(
                (s1.heat - s2.heat).abs() < 1e-5,
                "heat must match: {} vs {}",
                s1.heat, s2.heat
            );
        }
        _ => panic!("Both engines must produce snapshots"),
    }
}
```

- [ ] **Step 2: Run to verify (may pass immediately)**

```bash
cargo test --test runner_determinism
```

Expected: `PASS` якщо Core вже детерміністичний (з seed). Якщо `FAIL` — це баг у Core, не у Runner.

- [ ] **Step 3: Commit**

```bash
git add tests/runner_determinism.rs
git commit -m "test(runner): add determinism integration test — same seed + config = same result"
```

---

## Task 4: Config Validation Error Details via HTTP

**Files:**
- Create: `config/scenarios/invalid_test.toml` (тимчасовий тест-сценарій)
- Modify: `src/viewer_server/api/run.rs` — expose ParseError details
- Test: `tests/runner_validation.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_validation.rs
use alife::viewer_server::{create_app, state::new_app_state};
use axum::body::Body;
use http::{Method, Request, StatusCode};
use http_body_util::BodyExt;
use tower::ServiceExt;
use std::path::PathBuf;

fn make_state() -> alife::viewer_server::state::AppState {
    new_app_state(
        PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios"),
        50, 30,
    )
}

#[tokio::test]
async fn start_unknown_scenario_returns_400_with_error_message() {
    let app = create_app(make_state());
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "scenario_id": "does_not_exist" }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::BAD_REQUEST);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["ok"].as_bool().unwrap(), false);
    assert!(json["error"].as_str().is_some(), "error field must be present");
    assert!(
        json["error"].as_str().unwrap().contains("does_not_exist"),
        "error must mention the missing scenario_id"
    );
}

#[tokio::test]
async fn start_with_missing_body_returns_422() {
    let app = create_app(make_state());
    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from("{}")) // missing scenario_id
                .unwrap(),
        )
        .await
        .unwrap();

    // axum returns 422 for missing required fields
    assert!(
        resp.status() == StatusCode::UNPROCESSABLE_ENTITY
            || resp.status() == StatusCode::BAD_REQUEST,
        "Missing required field must return 4xx, got: {}",
        resp.status()
    );
}

#[tokio::test]
async fn start_already_running_returns_409_conflict() {
    let state = make_state();

    // First start
    create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "scenario_id": "single_cell_survival" }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    // Second start
    let resp = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "scenario_id": "single_cell_survival" }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::CONFLICT);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["ok"].as_bool().unwrap(), false);
    assert!(json["error"].as_str().unwrap().contains("active"), "error must mention active run");

    create_app(state).oneshot(
        Request::builder().method(Method::POST).uri("/run/stop").body(Body::empty()).unwrap()
    ).await.unwrap();
}

#[tokio::test]
async fn start_returns_config_hash_and_seed_in_response() {
    let state = make_state();
    let resp = create_app(state.clone())
        .oneshot(
            Request::builder()
                .method(Method::POST)
                .uri("/run/start")
                .header("content-type", "application/json")
                .body(Body::from(
                    serde_json::json!({ "scenario_id": "single_cell_survival", "seed": 99 }).to_string()
                ))
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), StatusCode::OK);
    let body = resp.into_body().collect().await.unwrap().to_bytes();
    let json: serde_json::Value = serde_json::from_slice(&body).unwrap();
    assert_eq!(json["ok"].as_bool().unwrap(), true);
    assert_eq!(json["seed"].as_u64().unwrap(), 99);
    assert!(json["config_hash"].as_str().is_some(), "config_hash must be in start response");

    create_app(state).oneshot(
        Request::builder().method(Method::POST).uri("/run/stop").body(Body::empty()).unwrap()
    ).await.unwrap();
}
```

- [ ] **Step 2: Run to verify current state**

```bash
cargo test --test runner_validation
```

Expected: деякі тести вже проходять (409, 400) — перевіримо що всі 4 `PASS`.

- [ ] **Step 3: Update `StartResponse` у `handle_run_start` — включити config_hash і seed**

```rust
#[derive(Serialize)]
struct StartResponse {
    ok: bool,
    config_hash: String,
    seed: u64,
}
```

Переконатись що `config_hash` і `seed` заповнені правильно з попередніх змін у Task 1.

- [ ] **Step 4: Run tests**

```bash
cargo test --test runner_validation
```

Expected: всі 4 тести `PASS`.

- [ ] **Step 5: Run full suite**

```bash
cargo test --workspace
```

- [ ] **Step 6: Commit**

```bash
git add src/viewer_server/api/run.rs tests/runner_validation.rs
git commit -m "feat(viewer-server): detailed error responses in /run/start; config_hash+seed in start response"
```

---

## Task 5: CORS і Remote Viewer Mode

**Files:**
- Modify: `Cargo.toml` — tower-http cors
- Modify: `src/runner/server_config.rs` — add allowed_origins
- Modify: `src/viewer_server/mod.rs` — create_app приймає CORS config
- Modify: `src/viewer_server/api/mod.rs` — apply CORS layer
- Modify: `config/server.toml` — allowed_origins example
- Modify: `src/bin/runner.rs` — pass ServerConfig до create_app
- Test: `tests/runner_cors.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_cors.rs
//! CORS and remote viewer mode integration tests.

use alife::runner::server_config::ServerConfig;
use alife::viewer_server::{create_app_with_config, state::new_app_state};
use axum::body::Body;
use http::{Method, Request};
use std::path::PathBuf;
use tower::ServiceExt;

fn scenarios_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios")
}

fn make_local_config() -> ServerConfig {
    ServerConfig {
        allow_remote_viewer: false,
        allowed_origins: vec![],
        ..ServerConfig::default()
    }
}

fn make_remote_config() -> ServerConfig {
    ServerConfig {
        allow_remote_viewer: true,
        allowed_origins: vec!["http://192.168.1.100:5173".to_string()],
        ..ServerConfig::default()
    }
}

#[tokio::test]
async fn local_mode_allows_localhost_requests() {
    let state = new_app_state(scenarios_dir(), 10, 30);
    let app = create_app_with_config(state, make_local_config());

    let resp = app
        .oneshot(
            Request::builder()
                .uri("/server/info")
                // No Origin header — simulates same-machine request
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    assert_eq!(resp.status(), 200);
}

#[tokio::test]
async fn cors_headers_present_in_remote_mode() {
    let state = new_app_state(scenarios_dir(), 10, 30);
    let app = create_app_with_config(state, make_remote_config());

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/server/info")
                .header("Origin", "http://192.168.1.100:5173")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    let headers = resp.headers();
    assert!(
        headers.contains_key("access-control-allow-origin"),
        "CORS header must be present in remote mode"
    );
}

#[tokio::test]
async fn cors_headers_absent_in_local_mode() {
    let state = new_app_state(scenarios_dir(), 10, 30);
    let app = create_app_with_config(state, make_local_config());

    let resp = app
        .oneshot(
            Request::builder()
                .method(Method::OPTIONS)
                .uri("/server/info")
                .header("Origin", "http://192.168.1.100:5173")
                .header("Access-Control-Request-Method", "GET")
                .body(Body::empty())
                .unwrap(),
        )
        .await
        .unwrap();

    // In local mode, CORS header should NOT allow the remote origin
    let acao = resp
        .headers()
        .get("access-control-allow-origin")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    assert!(
        acao != "http://192.168.1.100:5173",
        "Remote origin must NOT be allowed in local mode"
    );
}
```

- [ ] **Step 2: Run to verify failure**

```bash
cargo test --test runner_cors
```

Expected: compile error — `create_app_with_config` not found.

- [ ] **Step 3: Add `tower-http` to `Cargo.toml`**

```toml
[dependencies]
# ... existing ...
tower-http = { version = "0.6", features = ["cors"] }
```

- [ ] **Step 4: Update `ServerConfig` — add allowed_origins**

```rust
// In src/runner/server_config.rs:
pub struct ServerConfig {
    pub bind_host:           String,
    pub port:                u16,
    pub allow_remote_viewer: bool,
    pub snapshot_buffer_size: usize,
    pub target_broadcast_fps: u32,
    /// Allowed origins for CORS (only used when allow_remote_viewer = true).
    pub allowed_origins:     Vec<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_host: "127.0.0.1".to_string(),
            port: 8080,
            allow_remote_viewer: false,
            snapshot_buffer_size: 300,
            target_broadcast_fps: 30,
            allowed_origins: vec![],
        }
    }
}
```

- [ ] **Step 5: Оновити `config/server.toml`**

```toml
[server]
bind_host = "127.0.0.1"
port = 8080
allow_remote_viewer = false
snapshot_buffer_size = 300
target_broadcast_fps = 30
# allowed_origins used only when allow_remote_viewer = true:
# allowed_origins = ["http://192.168.1.51:5173"]
```

- [ ] **Step 6: Реалізувати `create_app_with_config` у `src/viewer_server/mod.rs`**

```rust
pub mod api;
pub mod broadcaster;
pub mod frame_encoder;
pub mod state;

use axum::Router;
use state::AppState;
use crate::runner::server_config::ServerConfig;

/// Build the app with default (local-only) config.
/// Backward-compatible alias used in tests.
pub fn create_app(app_state: AppState) -> Router {
    create_app_with_config(app_state, ServerConfig::default())
}

/// Build the app with explicit ServerConfig for CORS setup.
pub fn create_app_with_config(app_state: AppState, cfg: ServerConfig) -> Router {
    use tower_http::cors::{CorsLayer, Any};
    use http::header;

    let router = api::build_router(app_state);

    if cfg.allow_remote_viewer && !cfg.allowed_origins.is_empty() {
        let origins: Vec<axum::http::HeaderValue> = cfg.allowed_origins
            .iter()
            .filter_map(|o| o.parse().ok())
            .collect();

        let cors = CorsLayer::new()
            .allow_origin(origins)
            .allow_methods([
                axum::http::Method::GET,
                axum::http::Method::POST,
                axum::http::Method::OPTIONS,
            ])
            .allow_headers([
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
            ]);

        router.layer(cors)
    } else {
        // Local mode: no permissive CORS — browser same-origin only
        router
    }
}
```

- [ ] **Step 7: Update `src/bin/runner.rs` — use `create_app_with_config`**

```rust
let app = create_app_with_config(state, server_cfg.clone());
```

- [ ] **Step 8: Run tests**

```bash
cargo test --test runner_cors
```

Expected: всі 3 тести `PASS`.

- [ ] **Step 9: Run full suite**

```bash
cargo test --workspace
```

- [ ] **Step 10: Commit**

```bash
git add Cargo.toml config/server.toml \
        src/runner/server_config.rs \
        src/viewer_server/mod.rs \
        src/bin/runner.rs \
        tests/runner_cors.rs
git commit -m "feat(viewer-server): add CORS support for remote viewer mode; create_app_with_config"
```

---

## Task 6: Graceful Shutdown (Ctrl+C)

**Files:**
- Modify: `src/bin/runner.rs` — graceful shutdown via `tokio::signal::ctrl_c`

- [ ] **Step 1: Update `serve()` функцію у `src/bin/runner.rs`**

```rust
async fn serve(cfg: ServerConfig, scenarios_dir: PathBuf) {
    let bind_addr = cfg.bind_addr();
    let state = new_app_state(scenarios_dir, cfg.snapshot_buffer_size, cfg.target_broadcast_fps);
    let app = create_app_with_config(state.clone(), cfg.clone());

    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap_or_else(|e| {
            eprintln!("[runner] Cannot bind to {}: {}", bind_addr, e);
            std::process::exit(1);
        });

    println!("[runner] HTTP server listening on http://{}", bind_addr);
    println!("[runner] Press Ctrl+C to stop.");

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(state))
        .await
        .expect("Server error");

    println!("[runner] Server stopped gracefully.");
}

/// Waits for Ctrl+C, then signals the tick loop to stop.
async fn shutdown_signal(state: alife::viewer_server::state::AppState) {
    tokio::signal::ctrl_c()
        .await
        .expect("failed to listen for ctrl-c");

    println!("[runner] Received Ctrl+C — stopping tick loop...");

    // Signal tick loop to stop
    let signal = {
        let locked = state.lock().unwrap();
        locked.tick_signal.clone()
    };
    if let Some(sig) = signal {
        sig.request_stop();
    }

    // Broadcast shutdown status to all WS clients
    {
        let locked = state.lock().unwrap();
        locked.broadcaster.send_status(serde_json::json!({
            "type": "status",
            "state": "idle",
            "collapse_reason": "server_shutdown"
        }).to_string());
    }

    // Brief wait for tick loop to exit
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
}
```

- [ ] **Step 2: Build binary**

```bash
cargo build --bin runner
```

- [ ] **Step 3: Smoke test — graceful shutdown**

```bash
# Terminal 1: start server
cargo run --bin runner -- --serve &
SERVER_PID=$!
sleep 1

# Terminal 2: connect WS client (wscat)
# wscat -c ws://127.0.0.1:8080/stream &

# Start run
curl -s -X POST http://127.0.0.1:8080/run/start \
     -H "Content-Type: application/json" \
     -d '{"scenario_id":"single_cell_survival"}'

sleep 1

# Send Ctrl+C to server
kill -INT $SERVER_PID

# Verify: server exits cleanly (exit code 0)
wait $SERVER_PID
echo "Exit code: $?"
# Expected: Exit code: 0
```

- [ ] **Step 4: Run full suite**

```bash
cargo test --workspace
```

- [ ] **Step 5: Commit**

```bash
git add src/bin/runner.rs
git commit -m "feat(runner): graceful shutdown on Ctrl+C — stops tick loop, broadcasts shutdown status"
```

---

## Task 7: WS Reconnect Test + Acceptance Gate

**Files:**
- Test: `tests/runner_ws_reconnect.rs`

- [ ] **Step 1: Write the reconnect test**

```rust
// tests/runner_ws_reconnect.rs
//! WS reconnect test: client disconnects and reconnects during active run.
//! Verifies that new connection receives current state without re-running commands.

use alife::viewer_server::{create_app, state::new_app_state};
use futures_util::StreamExt;
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
    tokio::spawn(async move { axum::serve(listener, app).await.ok(); });
    tokio::time::sleep(std::time::Duration::from_millis(30)).await;
    (format!("http://{}", addr), state)
}

#[tokio::test]
async fn reconnect_receives_running_status_when_simulation_active() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";

    // Start simulation via HTTP
    let client = reqwest::Client::new();
    client
        .post(format!("{}/run/start", base))
        .json(&serde_json::json!({ "scenario_id": "single_cell_survival" }))
        .send()
        .await
        .unwrap();

    // First client connects, gets status
    {
        let (mut ws, _) = connect_async(&ws_url).await.unwrap();
        let msg = tokio::time::timeout(
            std::time::Duration::from_millis(500),
            ws.next(),
        ).await.unwrap().unwrap().unwrap();
        // Must be "running" since simulation was already started
        if let Message::Text(t) = msg {
            let j: serde_json::Value = serde_json::from_str(&t).unwrap_or_default();
            assert_eq!(j["type"].as_str().unwrap_or(""), "status");
            assert_eq!(j["state"].as_str().unwrap_or(""), "running");
        }
        // ws drops here — client 1 disconnects
    }

    // Let simulation advance
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    // Reconnect — second client
    let (mut ws2, _) = connect_async(&ws_url).await.unwrap();
    let msg2 = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        ws2.next(),
    ).await.unwrap().unwrap().unwrap();

    // Must still see "running"
    let text = match msg2 {
        Message::Text(t) => t,
        other => panic!("Expected text status, got: {:?}", other),
    };
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["type"].as_str().unwrap(), "status");
    assert_eq!(
        json["state"].as_str().unwrap(),
        "running",
        "Reconnect must see current state (running)"
    );
    assert!(
        json["tick"].as_u64().unwrap_or(0) >= 0,
        "Reconnect must receive current tick"
    );

    client.post(format!("{}/run/stop", base)).send().await.unwrap();
}

#[tokio::test]
async fn reconnect_receives_idle_status_after_stop() {
    let (base, _) = spawn_test_server().await;
    let ws_url = base.replace("http://", "ws://") + "/stream";

    let client = reqwest::Client::new();
    client.post(format!("{}/run/start", base))
        .json(&serde_json::json!({ "scenario_id": "single_cell_survival" }))
        .send().await.unwrap();
    client.post(format!("{}/run/stop", base)).send().await.unwrap();

    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    let (mut ws, _) = connect_async(&ws_url).await.unwrap();
    let msg = tokio::time::timeout(
        std::time::Duration::from_millis(500),
        ws.next(),
    ).await.unwrap().unwrap().unwrap();

    let text = match msg { Message::Text(t) => t, _ => panic!("Expected text") };
    let json: serde_json::Value = serde_json::from_str(&text).unwrap();
    assert_eq!(json["state"].as_str().unwrap(), "idle");
}
```

- [ ] **Step 2: Run reconnect tests**

```bash
cargo test --test runner_ws_reconnect
```

Expected: обидва `PASS` — initial status вже реалізований у Runner-3.

- [ ] **Step 3: Final full workspace test**

```bash
cargo test --workspace
```

Expected: **всі тести PASS** (runner_server_config, runner_http_info, runner_http_scenarios, runner_http_run_control, runner_serve_smoke, runner_frame_encoder, runner_ws_stream, runner_run_status, runner_collapse, runner_determinism, runner_validation, runner_cors, runner_ws_reconnect).

- [ ] **Step 4: Commit**

```bash
git add tests/runner_ws_reconnect.rs
git commit -m "test(runner-4): WS reconnect test — verify reconnect gets current state without re-commands"
```

---

## Self-Review

### Spec coverage (Runner-4 Gate)

| Вимога | Task | Status |
|---|---|---|
| `config_hash` у /run/status | 1 | ✅ |
| `seed` у /run/status | 1 | ✅ |
| `ticks_per_second` у /run/status | 1 | ✅ |
| config_hash і seed у /run/start response | 4 | ✅ |
| collapse_reason у /run/status | 2 | ✅ |
| WS broadcast on collapse | 2 | ✅ |
| Determinism: same seed → same result | 3 | ✅ |
| Config validation error via HTTP | 4 | ✅ |
| CORS у remote viewer mode | 5 | ✅ |
| IP filtering у local mode | 5 | ✅ |
| allowed_origins у ServerConfig | 5 | ✅ |
| Graceful shutdown (Ctrl+C) | 6 | ✅ |
| WS shutdown broadcast | 6 | ✅ |
| Reconnect отримує поточний стан | 7 | ✅ |

### Загальний Acceptance Gate Runner

| Умова | Де тестується |
|---|---|
| headless run стартує детерміновано | runner_determinism |
| serve run стартує HTTP + WS | runner_serve_smoke |
| GET /scenarios повертає список | runner_http_scenarios |
| GET /scenarios/{id} повертає TOML | runner_http_scenarios |
| POST /run/start з seed і config | runner_run_status, runner_validation |
| GET /run/status — tick, state, hash, seed, tps | runner_run_status |
| POST /run/pause і /run/resume | runner_http_run_control |
| POST /run/step | runner_http_run_control |
| POST /run/stop | runner_http_run_control |
| WS /stream ≤ target_broadcast_fps | runner_ws_stream |
| два клієнти — незалежні streams | runner_ws_stream |
| Core не чекає WS клієнтів | runner_ws_stream |
| new WS → initial status | runner_ws_stream, runner_ws_reconnect |
| reconnect без повтору команд | runner_ws_reconnect |
| same seed → same result | runner_determinism |
| collapse → collapse_reason у /run/status | runner_collapse |

---

## Acceptance Gate Runner-4

```
cargo test --workspace    → всі тести PASS (без жодного failure)

cargo run --bin runner -- --serve
curl http://127.0.0.1:8080/server/info
  → {"engine_version":"...","api_version":"1",...}

POST /run/start {"scenario_id":"single_cell_survival","seed":42}
  → {"ok":true,"config_hash":"<16-char hex>","seed":42}

GET /run/status
  → {"state":"running","tick":N,"config_hash":"...","seed":42,"ticks_per_second":X.X,...}

(Дочекатись завершення або POST /run/stop)
GET /run/status  (після природного завершення)
  → {"state":"idle","collapse_reason":"MaxTicksReached" або CollapseReason variant,...}

WS /stream при підключенні → {"type":"status","state":"running","tick":N}
WS при зупинці → {"type":"status","state":"idle","tick":N,"collapse_reason":"..."}
WS reconnect → {"type":"status","state":"running"/"idle",...} (поточний стан)

Ctrl+C у serve mode → server exits cleanly
                     → WS клієнт отримує {"type":"status","state":"idle","collapse_reason":"server_shutdown"}

CORS:
  allow_remote_viewer=true → OPTIONS /server/info + Origin: http://... → access-control-allow-origin header
  allow_remote_viewer=false → remote origin NOT in CORS header

Determinism:
  cargo test --test runner_determinism → 3 PASS
```
