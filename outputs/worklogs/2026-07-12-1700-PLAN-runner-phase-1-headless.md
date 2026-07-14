# Runner Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` (recommended) or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Реалізувати headless runner з явним run state machine, ring buffer committed snapshots і `config/scenarios/` директорією — без HTTP або WS (це фундамент для Runner-2 і Runner-3).

**Architecture:** `src/runner/` отримує новий модуль `engine.rs` з `RunEngine` struct, яка обгортає `TickExecutor` і `RunState`. `src/bin/runner.rs` — новий binary entry point, що читає TOML з `config/scenarios/`, будує `RuntimeConfig` через існуючий `config_parser.rs`, і делегує виконання в `RunEngine`. Ring buffer (`RingBuffer<CommittedSnapshot>`) зберігає N останніх snapshot-ів у пам'яті.

**Tech Stack:** Rust 2024 edition, `TickExecutor` з `alife::core::tick`, `CommittedSnapshot` з `alife::core::snapshot`, `RuntimeConfig` з `alife::core::config`, TOML через `alife::runner::config_parser`.

---

## File Structure

```
src/
  bin/
    runner.rs            [NEW] — binary entry point, CLI arg parsing, debug progress output
    sweep_analyzer.rs    [EXISTING, не змінюємо]
  runner/
    mod.rs               [MODIFY] — додати pub mod engine, pub mod scenario, pub mod progress
    config_parser.rs     [EXISTING, не змінюємо]
    engine.rs            [NEW] — RunEngine, RunState, run state machine
    scenario.rs          [NEW] — ScenarioConfig, scan_scenarios, load_scenario
    ring_buffer.rs       [NEW] — RingBuffer<T>
    progress.rs          [NEW] — debug progress status table formatting
  lib.rs                 [MODIFY] — додати pub mod bin::runner
config/
  scenarios/
    single_cell_survival.toml  [NEW] — demo scenario
    division_test.toml          [NEW] — demo scenario
tests/
  runner_state_machine.rs      [NEW] — state machine unit tests
  runner_ring_buffer.rs        [NEW] — ring buffer unit tests
  runner_scenario_loader.rs    [NEW] — scenario scan + load tests
  runner_headless_e2e.rs       [NEW] — end-to-end: build RunEngine, run N ticks
  runner_progress.rs           [NEW] — debug progress formatting and CLI interval behavior
Cargo.toml                     [MODIFY] — додати [[bin]] для runner
```

---

## Task 1: RingBuffer

**Files:**
- Create: `src/runner/ring_buffer.rs`
- Test: `tests/runner_ring_buffer.rs`

- [ ] **Step 1: Write the failing test**

```rust
// tests/runner_ring_buffer.rs
use alife::runner::ring_buffer::RingBuffer;

#[test]
fn ring_buffer_stores_items_up_to_capacity() {
    let mut buf: RingBuffer<u32> = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.get(0), Some(&1));
    assert_eq!(buf.get(2), Some(&3));
}

#[test]
fn ring_buffer_overwrites_oldest_when_full() {
    let mut buf: RingBuffer<u32> = RingBuffer::new(3);
    buf.push(1);
    buf.push(2);
    buf.push(3);
    buf.push(4); // overwrites 1
    assert_eq!(buf.len(), 3);
    assert_eq!(buf.get(0), Some(&2));
    assert_eq!(buf.get(2), Some(&4));
}

#[test]
fn ring_buffer_newest_returns_last_pushed() {
    let mut buf: RingBuffer<u32> = RingBuffer::new(5);
    buf.push(10);
    buf.push(20);
    assert_eq!(buf.newest(), Some(&20));
}

#[test]
fn ring_buffer_newest_returns_none_when_empty() {
    let buf: RingBuffer<u32> = RingBuffer::new(3);
    assert_eq!(buf.newest(), None);
}

#[test]
fn ring_buffer_iter_returns_items_oldest_first() {
    let mut buf: RingBuffer<u32> = RingBuffer::new(3);
    buf.push(10);
    buf.push(20);
    buf.push(30);
    buf.push(40); // overwrites 10
    let items: Vec<u32> = buf.iter().copied().collect();
    assert_eq!(items, vec![20, 30, 40]);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_ring_buffer
```

Expected: `FAIL` — `alife::runner::ring_buffer` does not exist.

- [ ] **Step 3: Create `src/runner/ring_buffer.rs`**

```rust
/// Circular ring buffer that overwrites oldest entries when full.
/// Items are stored oldest-first; `get(0)` is oldest, `get(len-1)` is newest.
pub struct RingBuffer<T> {
    data: Vec<Option<T>>,
    capacity: usize,
    head: usize,  // index of oldest item
    count: usize, // how many items are stored
}

impl<T: Clone> RingBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        assert!(capacity > 0, "RingBuffer capacity must be > 0");
        Self {
            data: vec![None; capacity],
            capacity,
            head: 0,
            count: 0,
        }
    }

    /// Push a new item. If full, overwrites the oldest.
    pub fn push(&mut self, item: T) {
        let write_index = (self.head + self.count) % self.capacity;
        self.data[write_index] = Some(item);
        if self.count < self.capacity {
            self.count += 1;
        } else {
            // Full: advance head to drop oldest
            self.head = (self.head + 1) % self.capacity;
        }
    }

    /// Get item by logical index (0 = oldest, len-1 = newest).
    pub fn get(&self, index: usize) -> Option<&T> {
        if index >= self.count {
            return None;
        }
        let physical = (self.head + index) % self.capacity;
        self.data[physical].as_ref()
    }

    pub fn newest(&self) -> Option<&T> {
        if self.count == 0 {
            return None;
        }
        self.get(self.count - 1)
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.count == 0
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        (0..self.count).filter_map(move |i| self.get(i))
    }
}
```

- [ ] **Step 4: Додати модуль у `src/runner/mod.rs`**

```rust
pub mod config_parser;
pub mod ring_buffer;
```

- [ ] **Step 5: Run tests to verify they pass**

```bash
cargo test --test runner_ring_buffer
```

Expected: всі 5 тестів `PASS`.

- [ ] **Step 6: Commit**

```bash
git add src/runner/ring_buffer.rs src/runner/mod.rs tests/runner_ring_buffer.rs
git commit -m "feat(runner): add RingBuffer<T> with oldest-first iteration"
```

---

## Task 2: ScenarioConfig and scenario loader

**Files:**
- Create: `src/runner/scenario.rs`
- Create: `config/scenarios/single_cell_survival.toml`
- Create: `config/scenarios/division_test.toml`
- Modify: `src/runner/mod.rs`
- Test: `tests/runner_scenario_loader.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_scenario_loader.rs
use alife::runner::scenario::{ScenarioMeta, scan_scenarios, load_scenario};
use std::path::PathBuf;

fn scenarios_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios")
}

#[test]
fn scan_scenarios_finds_toml_files() {
    let metas = scan_scenarios(&scenarios_dir()).expect("scan should succeed");
    assert!(!metas.is_empty(), "Expected at least one scenario in config/scenarios/");
}

#[test]
fn scan_scenarios_returns_valid_meta() {
    let metas = scan_scenarios(&scenarios_dir()).expect("scan should succeed");
    for meta in &metas {
        assert!(!meta.id.is_empty(), "scenario id must not be empty");
        assert!(meta.path.exists(), "scenario file must exist");
    }
}

#[test]
fn load_scenario_returns_runtime_config_for_single_cell_survival() {
    let metas = scan_scenarios(&scenarios_dir()).expect("scan should succeed");
    let meta = metas.iter().find(|m| m.id == "single_cell_survival")
        .expect("single_cell_survival scenario must exist");
    let config = load_scenario(meta).expect("load must succeed");
    // RuntimeConfig should have a valid world size
    assert!(config.world.size.width() > 0.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_scenario_loader
```

Expected: `FAIL` — `alife::runner::scenario` does not exist.

- [ ] **Step 3: Create `config/scenarios/single_cell_survival.toml`**

```toml
# Single cell survival — базовий сценарій без поділу
[world]
size = [512.0, 512.0]
tick_count = 1000
seed = 42

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 4

[resources]
initial_distribution = "uniform"
density = 5.0
layer_count = 1
decay_rate = 0.001

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 0.5
energy_per_resource = 2.0
heat_per_resource = 0.1
waste_per_resource = 0.05

[cell]
position = [256.0, 256.0]
radius = 8.0
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 2.0
passive_energy_income = 0.0
capacity_limit = 200.0
initial_resource_amount = 10.0
initial_boundary_material = 3.0
initial_transport_material = 2.0
initial_metabolic_material = 2.0
initial_storage_material = 0.0
initial_synthesis_material = 0.0
initial_structural_material = 1.0
initial_repair_material = 0.0
initial_contractile_material = 0.0
initial_sensory_material = 0.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.25
critical_capacity_overrun = 10.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.05
waste_warning_threshold = 30.0
waste_death_threshold = 80.0
```

- [ ] **Step 4: Create `config/scenarios/division_test.toml`**

```toml
# Division test — клітина, яка ділиться при достатній енергії
[world]
size = [512.0, 512.0]
tick_count = 500
seed = 42

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 4

[resources]
initial_distribution = "uniform"
density = 10.0
layer_count = 1
decay_rate = 0.001

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 2.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 2.0
heat_per_resource = 0.1
waste_per_resource = 0.05

[cell]
position = [256.0, 256.0]
radius = 10.0
initial_energy = 80.0
energy_capacity = 150.0
mandatory_cost_per_tick = 2.0
passive_energy_income = 0.0
capacity_limit = 300.0
initial_resource_amount = 20.0
initial_boundary_material = 4.0
initial_transport_material = 3.0
initial_metabolic_material = 3.0
initial_storage_material = 0.0
initial_synthesis_material = 0.0
initial_structural_material = 2.0
initial_repair_material = 0.0
initial_contractile_material = 0.0
initial_sensory_material = 0.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.25
critical_capacity_overrun = 10.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.05
waste_warning_threshold = 30.0
waste_death_threshold = 80.0

[division]
enabled = true
energy_cost = 20.0
split_ratio = 0.5
partition_loss_fraction = 0.0
daughter_spacing = 1.0
min_daughter_radius = 3.0
```

- [ ] **Step 5: Create `src/runner/scenario.rs`**

```rust
use crate::runner::config_parser::parse_runtime_config;
use crate::core::config::RuntimeConfig;
use std::path::{Path, PathBuf};

/// Lightweight metadata about a scenario — no full config loaded yet.
#[derive(Debug, Clone)]
pub struct ScenarioMeta {
    /// Filename stem, e.g. "single_cell_survival" for single_cell_survival.toml
    pub id: String,
    /// Absolute path to the TOML file
    pub path: PathBuf,
}

/// Scan `dir` and return metadata for every `.toml` file found (non-recursive).
pub fn scan_scenarios(dir: &Path) -> Result<Vec<ScenarioMeta>, String> {
    let entries = std::fs::read_dir(dir)
        .map_err(|e| format!("Cannot read scenarios dir {:?}: {}", dir, e))?;

    let mut metas = Vec::new();
    for entry in entries.flatten() {
        let path = entry.path();
        if path.extension().and_then(|e| e.to_str()) == Some("toml") {
            let id = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or("")
                .to_string();
            if !id.is_empty() {
                metas.push(ScenarioMeta { id, path });
            }
        }
    }
    metas.sort_by(|a, b| a.id.cmp(&b.id));
    Ok(metas)
}

/// Load and parse a scenario TOML into RuntimeConfig.
pub fn load_scenario(meta: &ScenarioMeta) -> Result<RuntimeConfig, String> {
    parse_runtime_config(&meta.path)
}
```

**Примітка:** `parse_runtime_config` має бути публічною функцією у `config_parser.rs`. Якщо вона не існує з такою сигнатурою, перевір що там є і адаптуй виклик. `config_parser.rs` вже містить логіку парсингу — ця функція має приймати `&Path` і повертати `Result<RuntimeConfig, String>`.

- [ ] **Step 6: Додати `pub mod scenario` у `src/runner/mod.rs`**

```rust
pub mod config_parser;
pub mod ring_buffer;
pub mod scenario;
```

- [ ] **Step 7: Run tests**

```bash
cargo test --test runner_scenario_loader
```

Expected: всі 3 тести `PASS`.

- [ ] **Step 8: Commit**

```bash
git add src/runner/scenario.rs src/runner/mod.rs \
        config/scenarios/single_cell_survival.toml \
        config/scenarios/division_test.toml \
        tests/runner_scenario_loader.rs
git commit -m "feat(runner): add scenario loader and demo TOML configs"
```

---

## Task 3: RunState machine

**Files:**
- Create: `src/runner/engine.rs` (state enum only, no TickExecutor yet)
- Modify: `src/runner/mod.rs`
- Test: `tests/runner_state_machine.rs`

- [ ] **Step 1: Write the failing tests**

```rust
// tests/runner_state_machine.rs
use alife::runner::engine::{RunState, RunStateTransition};

#[test]
fn initial_state_is_idle() {
    assert_eq!(RunState::Idle, RunState::default());
}

#[test]
fn idle_can_transition_to_running() {
    let state = RunState::Idle;
    assert_eq!(state.apply(RunStateTransition::Start), Ok(RunState::Running));
}

#[test]
fn running_can_pause() {
    let state = RunState::Running;
    assert_eq!(state.apply(RunStateTransition::Pause), Ok(RunState::Paused));
}

#[test]
fn running_can_stop() {
    let state = RunState::Running;
    assert_eq!(state.apply(RunStateTransition::Stop), Ok(RunState::Idle));
}

#[test]
fn paused_can_resume() {
    let state = RunState::Paused;
    assert_eq!(state.apply(RunStateTransition::Resume), Ok(RunState::Running));
}

#[test]
fn paused_can_stop() {
    let state = RunState::Paused;
    assert_eq!(state.apply(RunStateTransition::Stop), Ok(RunState::Idle));
}

#[test]
fn idle_cannot_pause() {
    let state = RunState::Idle;
    assert!(state.apply(RunStateTransition::Pause).is_err());
}

#[test]
fn idle_cannot_resume() {
    let state = RunState::Idle;
    assert!(state.apply(RunStateTransition::Resume).is_err());
}

#[test]
fn running_cannot_start_again() {
    let state = RunState::Running;
    assert!(state.apply(RunStateTransition::Start).is_err());
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_state_machine
```

Expected: `FAIL` — `alife::runner::engine` does not exist.

- [ ] **Step 3: Create `src/runner/engine.rs` (state machine only)**

```rust
/// Current lifecycle state of a simulation run.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunState {
    /// No active run. Ready to start.
    Idle,
    /// Simulation is executing ticks.
    Running,
    /// Simulation is paused. Can resume or stop.
    Paused,
}

impl Default for RunState {
    fn default() -> Self {
        RunState::Idle
    }
}

/// Commands that trigger state transitions.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RunStateTransition {
    Start,
    Pause,
    Resume,
    Stop,
}

impl RunState {
    /// Apply a transition. Returns Ok(new_state) or Err(description) if invalid.
    pub fn apply(&self, transition: RunStateTransition) -> Result<RunState, String> {
        match (self, transition) {
            (RunState::Idle, RunStateTransition::Start) => Ok(RunState::Running),
            (RunState::Running, RunStateTransition::Pause) => Ok(RunState::Paused),
            (RunState::Running, RunStateTransition::Stop) => Ok(RunState::Idle),
            (RunState::Paused, RunStateTransition::Resume) => Ok(RunState::Running),
            (RunState::Paused, RunStateTransition::Stop) => Ok(RunState::Idle),
            (state, transition) => Err(format!(
                "Invalid transition {:?} from state {:?}",
                transition, state
            )),
        }
    }
}
```

- [ ] **Step 4: Додати `pub mod engine` у `src/runner/mod.rs`**

```rust
pub mod config_parser;
pub mod engine;
pub mod ring_buffer;
pub mod scenario;
```

- [ ] **Step 5: Run tests**

```bash
cargo test --test runner_state_machine
```

Expected: всі 9 тестів `PASS`.

- [ ] **Step 6: Commit**

```bash
git add src/runner/engine.rs src/runner/mod.rs tests/runner_state_machine.rs
git commit -m "feat(runner): add RunState machine with transition validation"
```

---

## Task 4: RunEngine — повна логіка запуску

**Files:**
- Modify: `src/runner/engine.rs` — додати `RunEngine` struct
- Test: `tests/runner_headless_e2e.rs`

- [ ] **Step 1: Write the failing end-to-end test**

```rust
// tests/runner_headless_e2e.rs
use alife::runner::engine::{RunEngine, RunEngineConfig, RunState};
use alife::runner::scenario::{ScenarioMeta, scan_scenarios};
use std::path::PathBuf;

fn scenarios_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("config/scenarios")
}

fn load_single_cell_meta() -> ScenarioMeta {
    scan_scenarios(&scenarios_dir())
        .expect("scan ok")
        .into_iter()
        .find(|m| m.id == "single_cell_survival")
        .expect("single_cell_survival must exist")
}

#[test]
fn run_engine_starts_in_idle() {
    let meta = load_single_cell_meta();
    let engine_cfg = RunEngineConfig { snapshot_buffer_size: 10 };
    let engine = RunEngine::from_scenario(&meta, engine_cfg).expect("should build");
    assert_eq!(engine.state(), RunState::Idle);
}

#[test]
fn run_engine_executes_n_ticks_and_updates_snapshot_buffer() {
    let meta = load_single_cell_meta();
    let engine_cfg = RunEngineConfig { snapshot_buffer_size: 50 };
    let mut engine = RunEngine::from_scenario(&meta, engine_cfg).expect("should build");

    engine.start().expect("start ok");
    assert_eq!(engine.state(), RunState::Running);

    engine.step(10).expect("step ok");
    assert_eq!(engine.current_tick(), 10);
    assert_eq!(engine.snapshots().len(), 10);
}

#[test]
fn run_engine_ring_buffer_does_not_grow_beyond_capacity() {
    let meta = load_single_cell_meta();
    let engine_cfg = RunEngineConfig { snapshot_buffer_size: 5 };
    let mut engine = RunEngine::from_scenario(&meta, engine_cfg).expect("should build");

    engine.start().expect("start ok");
    engine.step(20).expect("step ok"); // more than buffer size
    assert_eq!(engine.snapshots().len(), 5); // capped at capacity
}

#[test]
fn run_engine_newest_snapshot_matches_current_tick() {
    let meta = load_single_cell_meta();
    let engine_cfg = RunEngineConfig { snapshot_buffer_size: 20 };
    let mut engine = RunEngine::from_scenario(&meta, engine_cfg).expect("should build");

    engine.start().expect("start ok");
    engine.step(15).expect("step ok");

    let newest = engine.snapshots().newest().expect("should have snapshot");
    assert_eq!(newest.tick.raw(), 15);
}

#[test]
fn run_engine_pause_and_resume_preserves_tick() {
    let meta = load_single_cell_meta();
    let engine_cfg = RunEngineConfig { snapshot_buffer_size: 20 };
    let mut engine = RunEngine::from_scenario(&meta, engine_cfg).expect("should build");

    engine.start().expect("start ok");
    engine.step(5).expect("step ok");
    engine.pause().expect("pause ok");
    assert_eq!(engine.state(), RunState::Paused);
    assert_eq!(engine.current_tick(), 5);

    engine.resume().expect("resume ok");
    engine.step(5).expect("step ok");
    assert_eq!(engine.current_tick(), 10);
}

#[test]
fn run_engine_same_seed_produces_same_final_snapshot() {
    let meta = load_single_cell_meta();
    let cfg = RunEngineConfig { snapshot_buffer_size: 20 };

    let mut engine_a = RunEngine::from_scenario(&meta, cfg.clone()).expect("ok");
    engine_a.start().expect("ok");
    engine_a.step(50).expect("ok");

    let mut engine_b = RunEngine::from_scenario(&meta, cfg).expect("ok");
    engine_b.start().expect("ok");
    engine_b.step(50).expect("ok");

    let snap_a = engine_a.snapshots().newest().expect("ok");
    let snap_b = engine_b.snapshots().newest().expect("ok");
    assert_eq!(snap_a, snap_b, "Same seed must produce identical snapshots");
}
```

- [ ] **Step 2: Run test to verify it fails**

```bash
cargo test --test runner_headless_e2e
```

Expected: `FAIL` — `RunEngine`, `RunEngineConfig` не існують.

- [ ] **Step 3: Додати `RunEngine` до `src/runner/engine.rs`**

Додати після існуючого коду:

```rust
use crate::core::config::RuntimeConfig;
use crate::core::snapshot::CommittedSnapshot;
use crate::core::tick::TickExecutor;
use crate::runner::ring_buffer::RingBuffer;
use crate::runner::scenario::{load_scenario, ScenarioMeta};

/// Configuration for RunEngine (not simulation config).
#[derive(Debug, Clone)]
pub struct RunEngineConfig {
    /// Maximum number of snapshots kept in memory for scroll-back.
    pub snapshot_buffer_size: usize,
}

/// The main headless simulation engine.
/// Owns the TickExecutor, run state, and snapshot ring buffer.
pub struct RunEngine {
    executor: Option<TickExecutor>,
    rt_config: RuntimeConfig,
    state: RunState,
    snapshots: RingBuffer<CommittedSnapshot>,
    current_tick: u32,
}

impl RunEngine {
    /// Build RunEngine from a scenario file. Does NOT start the simulation.
    pub fn from_scenario(
        meta: &ScenarioMeta,
        engine_cfg: RunEngineConfig,
    ) -> Result<Self, String> {
        let rt_config = load_scenario(meta)?;
        Ok(Self {
            executor: None,
            rt_config,
            state: RunState::Idle,
            snapshots: RingBuffer::new(engine_cfg.snapshot_buffer_size),
            current_tick: 0,
        })
    }

    pub fn state(&self) -> RunState {
        self.state
    }

    pub fn current_tick(&self) -> u32 {
        self.current_tick
    }

    pub fn snapshots(&self) -> &RingBuffer<CommittedSnapshot> {
        &self.snapshots
    }

    /// Transition to Running. Creates and initializes the TickExecutor.
    pub fn start(&mut self) -> Result<(), String> {
        self.state = self.state.apply(RunStateTransition::Start)?;
        let executor = TickExecutor::new(self.rt_config.clone())
            .map_err(|e| format!("TickExecutor init failed: {:?}", e))?;
        self.executor = Some(executor);
        self.current_tick = 0;
        Ok(())
    }

    /// Execute exactly `n` ticks and store each committed snapshot.
    /// Only valid in Running or Paused state (caller must Resume before stepping
    /// from Paused if they want state semantics, or this method is state-agnostic).
    pub fn step(&mut self, n: u32) -> Result<(), String> {
        let executor = self
            .executor
            .as_mut()
            .ok_or_else(|| "No active executor — call start() first".to_string())?;

        for _ in 0..n {
            executor
                .step()
                .map_err(|e| format!("Tick failed: {:?}", e))?;
            self.current_tick += 1;
            let snapshot = CommittedSnapshot::from_world(executor.world());
            self.snapshots.push(snapshot);
        }
        Ok(())
    }

    /// Pause the running simulation.
    pub fn pause(&mut self) -> Result<(), String> {
        self.state = self.state.apply(RunStateTransition::Pause)?;
        Ok(())
    }

    /// Resume a paused simulation.
    pub fn resume(&mut self) -> Result<(), String> {
        self.state = self.state.apply(RunStateTransition::Resume)?;
        Ok(())
    }

    /// Stop the simulation and drop the executor.
    pub fn stop(&mut self) -> Result<(), String> {
        self.state = self.state.apply(RunStateTransition::Stop)?;
        self.executor = None;
        Ok(())
    }
}
```

- [ ] **Step 4: Run tests**

```bash
cargo test --test runner_headless_e2e
```

Expected: всі 6 тестів `PASS`. Якщо `parse_runtime_config` повертає інший тип або `TickExecutor::new` має іншу сигнатуру — звір з існуючим кодом і адаптуй.

- [ ] **Step 5: Commit**

```bash
git add src/runner/engine.rs tests/runner_headless_e2e.rs
git commit -m "feat(runner): add RunEngine with state machine and ring buffer"
```

---

## Task 5: Binary entry point `runner`

**Files:**
- Create: `src/bin/runner.rs`
- Create: `src/runner/progress.rs`
- Test: `tests/runner_progress.rs`
- Modify: `src/runner/mod.rs`
- Modify: `Cargo.toml` — додати `[[bin]]`

- [ ] **Step 1: Додати `[[bin]]` у `Cargo.toml`**

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
```

- [ ] **Step 1A: Add debug progress tests**

`--debug` is intentionally minimal: it is only an observability switch for early runner work. It must not become a full CLI control surface in Runner-1.

```rust
// tests/runner_progress.rs
use std::time::Duration;

use alife::runner::progress::{
    format_progress_table, ProgressInterval, ProgressSnapshot,
};

#[test]
fn debug_progress_interval_defaults_to_2000_ms() {
    assert_eq!(ProgressInterval::default().as_duration(), Duration::from_millis(2000));
}

#[test]
fn progress_interval_can_be_overridden_in_ms() {
    let interval = ProgressInterval::from_millis(500).expect("valid interval");
    assert_eq!(interval.as_duration(), Duration::from_millis(500));
}

#[test]
fn progress_interval_rejects_zero_ms() {
    assert!(ProgressInterval::from_millis(0).is_err());
}

#[test]
fn progress_table_contains_minimum_runtime_status() {
    let output = format_progress_table(&ProgressSnapshot {
        elapsed_ms: 2000,
        tick: 120,
        max_ticks: 1000,
        ticks_per_second: 60.0,
        cells: 5,
        alive_cells: Some(4),
        dead_cells: Some(1),
        heat: 0.25,
        waste: 0.10,
        state: "Running".to_string(),
        collapse_reason: None,
    });

    assert!(output.contains("elapsed"));
    assert!(output.contains("tick"));
    assert!(output.contains("tps"));
    assert!(output.contains("cells"));
    assert!(output.contains("alive"));
    assert!(output.contains("dead"));
    assert!(output.contains("heat"));
    assert!(output.contains("waste"));
    assert!(output.contains("Running"));
}
```

- [ ] **Step 1B: Create `src/runner/progress.rs`**

Keep this module formatting-only. It reads already-committed runner/snapshot data and returns strings. It must not own the simulation loop, mutate `RunEngine`, or read random state.

```rust
use std::time::Duration;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ProgressInterval(Duration);

impl Default for ProgressInterval {
    fn default() -> Self {
        Self(Duration::from_millis(2000))
    }
}

impl ProgressInterval {
    pub fn from_millis(ms: u64) -> Result<Self, String> {
        if ms == 0 {
            return Err("--progress-interval-ms must be greater than 0".to_string());
        }
        Ok(Self(Duration::from_millis(ms)))
    }

    pub fn as_duration(&self) -> Duration {
        self.0
    }
}

#[derive(Debug, Clone)]
pub struct ProgressSnapshot {
    pub elapsed_ms: u128,
    pub tick: u32,
    pub max_ticks: u32,
    pub ticks_per_second: f32,
    pub cells: usize,
    pub alive_cells: Option<usize>,
    pub dead_cells: Option<usize>,
    pub heat: f32,
    pub waste: f32,
    pub state: String,
    pub collapse_reason: Option<String>,
}

pub fn format_progress_table(snapshot: &ProgressSnapshot) -> String {
    let elapsed_s = snapshot.elapsed_ms as f32 / 1000.0;
    let alive = snapshot
        .alive_cells
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".to_string());
    let dead = snapshot
        .dead_cells
        .map(|v| v.to_string())
        .unwrap_or_else(|| "-".to_string());
    let status = snapshot
        .collapse_reason
        .as_deref()
        .unwrap_or(snapshot.state.as_str());

    format!(
        "\
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+\n\
| elapsed_s | tick        | tps     | cells | alive | dead | heat  | waste | state     |\n\
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+\n\
| {elapsed_s:<9.2} | {tick:<4}/{max:<5} | {tps:<7.1} | {cells:<5} | {alive:<5} | {dead:<4} | {heat:<5.2} | {waste:<5.2} | {status:<9} |\n\
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+",
        tick = snapshot.tick,
        max = snapshot.max_ticks,
        tps = snapshot.ticks_per_second,
        cells = snapshot.cells,
        heat = snapshot.heat,
        waste = snapshot.waste,
    )
}
```

- [ ] **Step 1C: Export progress module**

```rust
// src/runner/mod.rs
pub mod progress;
```

- [ ] **Step 2: Create `src/bin/runner.rs`**

```rust
//! ALife headless runner binary.
//!
//! Usage:
//!   cargo run --bin runner -- config/scenarios/single_cell_survival.toml
//!   cargo run --bin runner -- --debug config/scenarios/single_cell_survival.toml
//!   cargo run --bin runner -- --debug --progress-interval-ms 500 config/scenarios/single_cell_survival.toml
//!   cargo run --bin runner -- --list
//!   cargo run --bin runner -- --serve config/scenarios/single_cell_survival.toml  (future)

use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::progress::{format_progress_table, ProgressInterval, ProgressSnapshot};
use alife::runner::scenario::{scan_scenarios, load_scenario, ScenarioMeta};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone)]
struct RunnerCli {
    scenario: Option<PathBuf>,
    list: bool,
    serve: bool,
    debug: bool,
    progress_interval: ProgressInterval,
}

fn parse_cli(args: &[String]) -> Result<RunnerCli, String> {
    let mut cli = RunnerCli {
        scenario: None,
        list: false,
        serve: false,
        debug: false,
        progress_interval: ProgressInterval::default(),
    };

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--list" => cli.list = true,
            "--serve" => cli.serve = true,
            "--debug" => cli.debug = true,
            "--progress-interval-ms" => {
                i += 1;
                let raw = args.get(i).ok_or("--progress-interval-ms requires a value")?;
                let ms = raw
                    .parse::<u64>()
                    .map_err(|_| "--progress-interval-ms must be an integer")?;
                cli.progress_interval = ProgressInterval::from_millis(ms)?;
            }
            value if value.starts_with("--") => return Err(format!("unknown flag: {value}")),
            value => {
                if cli.scenario.is_some() {
                    return Err("only one scenario path/id is supported".to_string());
                }
                cli.scenario = Some(PathBuf::from(value));
            }
        }
        i += 1;
    }

    Ok(cli)
}

fn main() {
    let args: Vec<String> = std::env::args().collect();

    let cli = parse_cli(&args).unwrap_or_else(|e| {
        eprintln!("Error: {e}");
        eprintln!("Usage: runner [--debug] [--progress-interval-ms N] <scenario.toml> | --list");
        eprintln!("  --debug                  Print progress status table while running");
        eprintln!("  --progress-interval-ms   Override debug progress interval; default is 2000");
        eprintln!("  --list                   List available scenarios in config/scenarios/");
        std::process::exit(1);
    });

    let scenarios_dir = PathBuf::from("config/scenarios");

    if cli.list {
        list_scenarios(&scenarios_dir);
        return;
    }

    if cli.serve {
        eprintln!("[runner] --serve mode: not yet implemented (Runner-2)");
        std::process::exit(1);
    }

    let Some(path) = cli.scenario.as_deref() else {
        eprintln!("Usage: runner [--debug] [--progress-interval-ms N] <scenario.toml> | --list");
        std::process::exit(1);
    };

    run_headless(path, &scenarios_dir, cli.debug, cli.progress_interval);
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

fn run_headless(
    scenario_path: &Path,
    scenarios_dir: &Path,
    debug: bool,
    progress_interval: ProgressInterval,
) {
    // Accept either a full path or a scenario id
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

    if debug {
        let mut next_progress_at = start + progress_interval.as_duration();

        while engine.current_tick() < total_ticks {
            if let Err(e) = engine.step(1) {
                eprintln!("[runner] Simulation error at tick {}: {}", engine.current_tick(), e);
                break;
            }

            let now = std::time::Instant::now();
            if engine.current_tick() == 1 || now >= next_progress_at {
                print_debug_progress(&engine, start, total_ticks);
                next_progress_at = now + progress_interval.as_duration();
            }
        }
    } else {
        engine.step(total_ticks).unwrap_or_else(|e| {
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

fn print_debug_progress(engine: &RunEngine, start: std::time::Instant, total_ticks: u32) {
    let elapsed = start.elapsed();
    let elapsed_s = elapsed.as_secs_f32();
    let tps = if elapsed_s > 0.0 {
        engine.current_tick() as f32 / elapsed_s
    } else {
        0.0
    };

    if let Some(snap) = engine.snapshots().newest() {
        let progress = ProgressSnapshot {
            elapsed_ms: elapsed.as_millis(),
            tick: engine.current_tick(),
            max_ticks: total_ticks,
            ticks_per_second: tps,
            cells: snap.cells.len(),
            alive_cells: None, // Fill from lifecycle state when snapshot exposes stable alive/dead semantics.
            dead_cells: None,
            heat: snap.heat,
            waste: snap.waste,
            state: format!("{:?}", engine.state()),
            collapse_reason: None,
        };

        println!("{}", format_progress_table(&progress));
    }
}

fn resolve_scenario(path: &Path, scenarios_dir: &Path) -> ScenarioMeta {
    // If path exists as-is, use it directly
    if path.exists() {
        let id = path
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("unknown")
            .to_string();
        return ScenarioMeta { id, path: path.to_path_buf() };
    }

    // Otherwise try to find by id in scenarios_dir
    let id = path.to_string_lossy().to_string();
    match scan_scenarios(scenarios_dir) {
        Ok(metas) => {
            if let Some(meta) = metas.into_iter().find(|m| m.id == id) {
                return meta;
            }
        }
        Err(e) => {
            eprintln!("[runner] Cannot scan scenarios: {}", e);
        }
    }

    eprintln!(
        "[runner] Scenario not found: '{}'. Use --list to see available scenarios.",
        path.display()
    );
    std::process::exit(1);
}
```

- [ ] **Step 3: Додати публічний `pub fn load_scenario` у `scenario.rs` (якщо ще не є)**

`load_scenario` вже є з попереднього task-а. Але нам потрібен доступ до `rt_config.world.tick_count` у binary. Перевір що `WorldConfig::tick_count` є pub і `Tick::raw()` повертає `u64` або `u32`. Адаптуй cast у `runner.rs` відповідно.

- [ ] **Step 4: Build binary**

```bash
cargo build --bin runner
```

Expected: компілюється без помилок і warnings.

- [ ] **Step 5: Test binary — список сценаріїв**

```bash
cargo run --bin runner -- --list
```

Expected output:
```
Available scenarios in config/scenarios:
  division_test (config/scenarios/division_test.toml)
  single_cell_survival (config/scenarios/single_cell_survival.toml)
```

- [ ] **Step 6: Test binary — запуск сценарію**

```bash
cargo run --bin runner -- config/scenarios/single_cell_survival.toml
```

Expected output (приблизно):
```
[runner] Loading scenario: single_cell_survival (config/scenarios/single_cell_survival.toml)
[runner] Running 1000 ticks...
[runner] Completed 1000 ticks in X.XXs (XXXX ticks/sec)
[runner] Final tick: 1000, cells: N, heat: X.XX, waste: X.XX
[runner] Done.
```

- [ ] **Step 7: Test binary — debug progress table**

Use a scenario long enough to run for at least one interval, or temporarily set a short interval for the test run.

```bash
cargo run --bin runner -- --debug --progress-interval-ms 200 config/scenarios/single_cell_survival.toml
```

Expected output includes at least one progress table before the final summary:

```
[runner] Loading scenario: single_cell_survival (config/scenarios/single_cell_survival.toml)
[runner] Running 1000 ticks...
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+
| elapsed_s | tick        | tps     | cells | alive | dead | heat  | waste | state     |
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+
| ...       | .../1000    | ...     | ...   | ...   | ...  | ...   | ...   | Running   |
+-----------+-------------+---------+-------+-------+------+-------+-------+-----------+
[runner] Completed 1000 ticks in X.XXs (XXXX ticks/sec)
[runner] Final tick: 1000, cells: N, heat: X.XX, waste: X.XX
[runner] Done.
```

Default behavior requirement:

```bash
cargo run --bin runner -- --debug config/scenarios/single_cell_survival.toml
```

Expected: first progress table is printed after the first committed tick, then the progress table interval is `2000 ms`.

Determinism requirement:

```bash
cargo run --bin runner -- config/scenarios/single_cell_survival.toml
cargo run --bin runner -- --debug --progress-interval-ms 200 config/scenarios/single_cell_survival.toml
```

Expected: final tick/cell/heat/waste summary matches for the same seed and scenario. Debug output must not change final simulation state.

- [ ] **Step 8: Verify all workspace tests still pass**

```bash
cargo test --workspace
```

Expected: всі існуючі тести `PASS`, без регресій.

- [ ] **Step 9: Commit**

```bash
git add src/bin/runner.rs src/runner/progress.rs src/runner/mod.rs tests/runner_progress.rs Cargo.toml
git commit -m "feat(runner): add headless runner binary with debug progress output"
```

---

## Task 6: Expose `pub mod bin::runner` у lib.rs

**Files:**
- Modify: `src/lib.rs`

- [ ] **Step 1: Перевір, що sweep_analyzer binary не конфліктує**

```bash
cargo build --bin sweep_analyzer
cargo build --bin runner
```

Якщо `src/lib.rs` декларує `pub mod bin { pub mod sweep_analyzer; }` — це може конфліктувати з bin crate. Перевір і при потребі прибери або ізолюй.

- [ ] **Step 2: Якщо конфлікт є — прибрати `pub mod bin` з `lib.rs`**

```rust
// src/lib.rs — видалити цей блок якщо є:
// pub mod bin {
//     pub mod sweep_analyzer;
// }
```

Binaries в `src/bin/` доступні автоматично через Cargo — їх не треба декларувати в `lib.rs`.

- [ ] **Step 3: Final check — весь workspace**

```bash
cargo test --workspace
cargo build --bin runner
cargo build --bin sweep_analyzer
```

Expected: все компілюється, всі тести `PASS`.

- [ ] **Step 4: Final commit**

```bash
git add src/lib.rs
git commit -m "fix(runner): resolve lib.rs bin module conflict if present"
```

---

## Self-Review

### Spec coverage

| Вимога з Runner-1 spec | Реалізована? |
|---|---|
| `cargo run --bin runner -- <scenario.toml>` CLI | ✅ Task 5 |
| `config/scenarios/` directory зі demo сценаріями | ✅ Task 2 |
| run state machine (Idle / Running / Paused / Stopping) | ✅ Task 3 |
| ring buffer (CommittedSnapshot, configurable size) | ✅ Task 1 |
| deterministic replay test (same seed = same result) | ✅ Task 4 `runner_headless_e2e::same_seed_produces_same` |
| scenario TOML знаходиться і валідується при старті | ✅ Task 2 + Task 5 |
| State transitions покриті тестами | ✅ Task 3 (9 тестів) |
| Ring buffer зберігає N останніх snapshot-ів | ✅ Task 1 + Task 4 |
| `--list` команда | ✅ Task 5 |
| `--debug` progress table | ✅ Task 5 Step 1A/1B/7 |
| `--progress-interval-ms` override | ✅ Task 5 Step 1A/7 |
| debug output does not change deterministic result | ✅ Task 5 Step 7 |

**Gaps:** `Stopping` state не реалізований як окремий transitional state (є тільки `Stop` → `Idle` direct). Це прийнятно для Runner-1 — transitional state потрібен тільки коли є HTTP async stop.

### Placeholder scan

Немає жодного "TBD", "TODO implement later" або "add error handling" без коду.

### Type consistency

- `RunState` — однакове у всіх файлах
- `RingBuffer<CommittedSnapshot>` — однакове у engine.rs і тестах
- `ScenarioMeta` — однакове у scenario.rs і тестах
- `CommittedSnapshot::from_world(executor.world())` — відповідає існуючому API у `core/snapshot.rs`
- `engine.snapshots().newest().tick.raw()` — відповідає `CommittedSnapshot { tick: Tick }` і `Tick::raw() -> u64`

---

## Acceptance Gate

Цей slice вважається завершеним, коли:

```
cargo test --test runner_ring_buffer       → всі 5 PASS
cargo test --test runner_state_machine     → всі 9 PASS
cargo test --test runner_scenario_loader   → всі 3 PASS
cargo test --test runner_headless_e2e      → всі 6 PASS
cargo test --test runner_progress          → всі PASS
cargo test --workspace                     → без регресій
cargo run --bin runner -- --list           → виводить сценарії
cargo run --bin runner -- config/scenarios/single_cell_survival.toml → запускається і друкує статистику
cargo run --bin runner -- --debug --progress-interval-ms 200 config/scenarios/single_cell_survival.toml → друкує progress table під час виконання
cargo run --bin runner -- config/scenarios/division_test.toml → запускається
```
