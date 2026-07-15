# Scheduler Cadence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Use superpowers:test-driven-development for every production change. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add explicit scheduler cadence so expensive decision, propagation, observer, snapshot, and viewer projection work no longer runs every Tick by default while committed simulation semantics stay deterministic.

**Architecture:** Add scheduler/time/viewer cadence config, then introduce small runtime gates around existing systems. Genome Runtime becomes an explicit decision refresh that updates cached ActionPlans at configured cadence; process execution/progress remains Tick-based. Viewer projection becomes a wall-clock sampler over latest committed state, independent from simulation Tick cadence.

**Tech Stack:** Rust, Cargo integration tests, existing `RuntimeConfig`, `TickExecutor`, `RunEngine`, Runner HTTP/WS server.

---

## Canon sources

Read before implementation:

- `docs/PRINCIPLES.md`
- `docs/engine/scheduler.md`
- `docs/world/tick-semantics.md`
- `docs/mechanics/tick-transaction.md`
- `docs/mechanics/deterministic-execution.md`
- `docs/mechanics/observer-projection.md`
- `docs/genetics/genome-runtime.md`
- `docs/runner/projections.md`

## Current-state facts

- `TickExecutor::step()` currently runs most systems every Tick.
- `GenomeTemplate.runtime_interval_ticks` is parsed and hashed but does not control `ActionPlan` refresh.
- `ActionPlan::from_genome(...)` is rebuilt inside every Tick loop for every living cell.
- `RunEngine::commit_one_tick()` stores a full `CommittedSnapshot` after every Tick.
- WS frame sending is throttled by `target_broadcast_fps`, but projection sampling is not yet an explicit reusable layer.
- Observer/resource totals are mixed into per-Tick `MetricsSummary`; expensive observer-only projections need separate cadence.

## Files and responsibilities

- `src/core/config.rs`
  - Add `SchedulerConfig`, `SchedulerFastConfig`, `SchedulerCellConfig`, `SchedulerWorldConfig`, `SchedulerObserverConfig`, `TimeConfig`.
  - Validate cadence values as positive non-zero integers.
- `src/runner/config_parser.rs`
  - Parse optional `[time]` and `[scheduler.*]` blocks from scenario TOML.
  - Keep backward compatibility: missing scheduler block uses current behavior where required, except new canonical scenario defaults may set Genome cadence to `10`.
- `src/core/action_plan.rs`
  - Keep `ActionPlan` value type reusable as cached per-cell decision state.
- `src/core/cell_store.rs`
  - Store committed action plan state and next Genome decision due tick per cell.
- `src/core/world.rs`
  - Initialize per-cell action plan cache during world creation and division.
- `src/core/tick.rs`
  - Add scheduler due checks.
  - Refresh Genome decision only when due.
  - Gate scheduled world propagation systems.
  - Keep mandatory upkeep/lifecycle/process progress every Tick.
- `src/runner/engine.rs`
  - Add snapshot cadence support while preserving latest committed state access.
- `src/viewer_server/state.rs`
  - Replace ad-hoc broadcast interval with explicit projection sampler.
- `src/runner/projections.rs`
  - Add metadata fields for viewer projection sequence/time in a later task if frame format is extended.
- `src/viewer_server/frame_encoder.rs`
  - Version-gate any ALIF frame metadata changes.
- `config/server.toml`
  - Add explicit `[viewer_projection]` defaults.
- `config/scenarios/demo/demo_living_world.toml`
  - Add explicit scheduler config after core behavior is implemented.
- `tests/scheduler_config.rs`
  - Config defaults and parsing tests.
- `tests/scheduler_genome_cadence.rs`
  - Genome decision cadence behavior.
- `tests/scheduler_world_cadence.rs`
  - World propagation cadence behavior.
- `tests/runner_projection_sampler.rs`
  - Viewer projection sampler and forced projection behavior.
- `tests/runner_snapshot_cadence.rs`
  - RunEngine snapshot cadence.

---

## Task 0: Branch and baseline

**Files:**

- No source changes.

- [ ] **Step 1: Confirm worktree state**

Run:

```powershell
git status --short --branch
```

Expected:

```text
## main...origin/main [ahead N]
```

If unrelated local changes exist, record them and avoid editing those files.

- [ ] **Step 2: Run focused baseline**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_scenario_loader --test runner_headless_e2e --test runner_ws_stream
```

Expected: all listed tests pass.

- [ ] **Step 3: Commit only if the branch contains uncommitted plan/doc changes**

Run:

```powershell
git status --short
```

Expected: clean or only intentional plan/docs from this setup.

---

## Task 1: Scheduler config model and parser

**Files:**

- Modify: `src/core/config.rs`
- Modify: `src/runner/config_parser.rs`
- Test: `tests/scheduler_config.rs`

- [ ] **Step 1: Write failing config tests**

Create `tests/scheduler_config.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;

fn base_config(extra: &str) -> String {
    format!(
        r#"
scenario_id = "scheduler_config_test"
seed = 7
tick_count = 100

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 2.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 0.0, sensory = 0.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.1
capacity_limit = 30.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 40.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

{extra}
"#
    )
}

#[test]
fn scheduler_config_defaults_to_current_compatibility_when_missing() {
    let config = RawScenarioConfig::parse(&base_config("")).unwrap();

    assert_eq!(config.scheduler.cell.genome_runtime_base_ticks, 1);
    assert_eq!(config.scheduler.world.resource_diffusion_ticks, 1);
    assert_eq!(config.scheduler.observer.resource_totals_ticks, 1);
}

#[test]
fn scheduler_config_parses_explicit_cadence_blocks() {
    let config = RawScenarioConfig::parse(&base_config(
        r#"
[time]
tick_duration_ms = 100
realtime_target_tps = 10
headless_target_tps = 50

[scheduler.cell]
genome_runtime_base_ticks = 10
genome_runtime_ticks_per_layer = 10

[scheduler.world]
resource_diffusion_ticks = 2
resource_decay_ticks = 5
passive_reactions_ticks = 2

[scheduler.observer]
observer_metrics_ticks = 10
resource_totals_ticks = 10
graph_analysis_ticks = 50
"#,
    ))
    .unwrap();

    assert_eq!(config.time.tick_duration_ms, 100);
    assert_eq!(config.time.realtime_target_tps, 10);
    assert_eq!(config.time.headless_target_tps, 50);
    assert_eq!(config.scheduler.cell.genome_runtime_base_ticks, 10);
    assert_eq!(config.scheduler.world.resource_diffusion_ticks, 2);
    assert_eq!(config.scheduler.world.resource_decay_ticks, 5);
    assert_eq!(config.scheduler.observer.graph_analysis_ticks, 50);
}

#[test]
fn scheduler_config_rejects_zero_cadence() {
    let err = RawScenarioConfig::parse(&base_config(
        r#"
[scheduler.cell]
genome_runtime_base_ticks = 0
"#,
    ))
    .unwrap_err();

    assert!(format!("{err:?}").contains("Invalid scheduler cadence"));
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_config
```

Expected: FAIL because `RuntimeConfig.scheduler` and `RuntimeConfig.time` do not exist.

- [ ] **Step 3: Add config types**

In `src/core/config.rs`, add focused structs:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct TimeConfig {
    pub tick_duration_ms: u32,
    pub realtime_target_tps: u32,
    pub headless_target_tps: u32,
}

impl Default for TimeConfig {
    fn default() -> Self {
        Self {
            tick_duration_ms: 100,
            realtime_target_tps: 10,
            headless_target_tps: 50,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchedulerCellConfig {
    pub genome_runtime_base_ticks: u64,
    pub genome_runtime_ticks_per_layer: u64,
    pub signal_emit_ticks: u64,
    pub controlled_reaction_ticks: u64,
    pub simple_synthesis_ticks: u64,
    pub basic_repair_ticks: u64,
    pub internal_rebalance_ticks: u64,
}

impl Default for SchedulerCellConfig {
    fn default() -> Self {
        Self {
            genome_runtime_base_ticks: 1,
            genome_runtime_ticks_per_layer: 1,
            signal_emit_ticks: 1,
            controlled_reaction_ticks: 1,
            simple_synthesis_ticks: 1,
            basic_repair_ticks: 1,
            internal_rebalance_ticks: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchedulerWorldConfig {
    pub resource_diffusion_ticks: u64,
    pub resource_decay_ticks: u64,
    pub passive_reactions_ticks: u64,
    pub background_material_degradation_ticks: u64,
    pub environment_heat_diffusion_ticks: u64,
    pub field_update_ticks: u64,
}

impl Default for SchedulerWorldConfig {
    fn default() -> Self {
        Self {
            resource_diffusion_ticks: 1,
            resource_decay_ticks: 1,
            passive_reactions_ticks: 1,
            background_material_degradation_ticks: 1,
            environment_heat_diffusion_ticks: 1,
            field_update_ticks: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SchedulerObserverConfig {
    pub observer_metrics_ticks: u64,
    pub resource_totals_ticks: u64,
    pub graph_analysis_ticks: u64,
    pub debug_trace_ticks: u64,
}

impl Default for SchedulerObserverConfig {
    fn default() -> Self {
        Self {
            observer_metrics_ticks: 1,
            resource_totals_ticks: 1,
            graph_analysis_ticks: 1,
            debug_trace_ticks: 1,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct SchedulerConfig {
    pub cell: SchedulerCellConfig,
    pub world: SchedulerWorldConfig,
    pub observer: SchedulerObserverConfig,
}
```

Add `time: TimeConfig` and `scheduler: SchedulerConfig` to `RuntimeConfig`.

Add validation method:

```rust
pub fn validate_scheduler_options(&self) -> Result<(), ConfigError> {
    let values = [
        self.scheduler.cell.genome_runtime_base_ticks,
        self.scheduler.cell.genome_runtime_ticks_per_layer,
        self.scheduler.cell.signal_emit_ticks,
        self.scheduler.cell.controlled_reaction_ticks,
        self.scheduler.cell.simple_synthesis_ticks,
        self.scheduler.cell.basic_repair_ticks,
        self.scheduler.cell.internal_rebalance_ticks,
        self.scheduler.world.resource_diffusion_ticks,
        self.scheduler.world.resource_decay_ticks,
        self.scheduler.world.passive_reactions_ticks,
        self.scheduler.world.background_material_degradation_ticks,
        self.scheduler.world.environment_heat_diffusion_ticks,
        self.scheduler.world.field_update_ticks,
        self.scheduler.observer.observer_metrics_ticks,
        self.scheduler.observer.resource_totals_ticks,
        self.scheduler.observer.graph_analysis_ticks,
        self.scheduler.observer.debug_trace_ticks,
    ];
    if values.iter().any(|value| *value == 0) {
        return Err(ConfigError::InvalidSchedulerCadence);
    }
    Ok(())
}
```

Add `InvalidSchedulerCadence` to `ConfigError`.

- [ ] **Step 4: Parse optional TOML blocks**

In `src/runner/config_parser.rs`, add raw structs:

```rust
#[derive(Deserialize, Debug, Default)]
pub struct RawTimeConfig {
    pub tick_duration_ms: Option<u32>,
    pub realtime_target_tps: Option<u32>,
    pub headless_target_tps: Option<u32>,
}

#[derive(Deserialize, Debug, Default)]
pub struct RawSchedulerConfig {
    pub cell: Option<RawSchedulerCellConfig>,
    pub world: Option<RawSchedulerWorldConfig>,
    pub observer: Option<RawSchedulerObserverConfig>,
}
```

Add raw child structs with `Option<u64>` fields matching `SchedulerConfig`.

Add to `RawScenarioConfig`:

```rust
#[serde(default)]
pub time: RawTimeConfig,
#[serde(default)]
pub scheduler: RawSchedulerConfig,
```

After `RuntimeConfig::new(...)`, assign parsed `runtime_config.time` and `runtime_config.scheduler`, then call:

```rust
runtime_config
    .validate_scheduler_options()
    .map_err(ParseError::ConfigValidationError)?;
```

- [ ] **Step 5: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_config
```

Expected: PASS.

- [ ] **Step 6: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_scenario_loader --test phase3a_genome_config
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add src/core/config.rs src/runner/config_parser.rs tests/scheduler_config.rs
git commit -m "feat(scheduler): parse cadence config"
```

---

## Task 2: Genome decision cadence and cached ActionPlan

**Files:**

- Modify: `src/core/action_plan.rs`
- Modify: `src/core/cell_store.rs`
- Modify: `src/core/world.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/scheduler_genome_cadence.rs`

- [ ] **Step 1: Write failing cadence test**

Create `tests/scheduler_genome_cadence.rs`:

```rust
use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

fn config_with_genome_cadence(cadence: u64) -> alife::core::config::RuntimeConfig {
    RawScenarioConfig::parse(&format!(
        r#"
scenario_id = "scheduler_genome_cadence"
seed = 11
tick_count = 30
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [50.0]
optional_decay_rate = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 0.5
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 5.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.01
capacity_limit = 40.0

[cell.genome]
template = "balanced"

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 50.0
waste_death_threshold = 100.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 50.0

[scheduler.cell]
genome_runtime_base_ticks = {cadence}
genome_runtime_ticks_per_layer = {cadence}

[genome_templates.balanced]
variation_amplitude = 0.0
runtime_interval_ticks = {cadence}

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.8
energy_conversion_priority = 0.7
material_synthesis_priority = 0.1
division_preparation_priority = 0.0
repair_priority = 0.0
"#
    ))
    .unwrap()
}

#[test]
fn genome_action_plan_refreshes_only_on_configured_cadence() {
    let mut executor = TickExecutor::new(config_with_genome_cadence(10)).unwrap();

    let first = executor.step().unwrap();
    assert_eq!(first.metrics.genome_decision_refresh_count, 1);
    assert!(first
        .diagnostics
        .attempted_processes
        .contains(&ProcessId::LocalResourceUptake));

    for _ in 0..8 {
        let summary = executor.step().unwrap();
        assert_eq!(summary.metrics.genome_decision_refresh_count, 0);
    }

    let tenth = executor.step().unwrap();
    assert_eq!(tenth.metrics.genome_decision_refresh_count, 1);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_genome_cadence
```

Expected: FAIL because action plan cache and `genome_decision_refresh_count` do not exist.

- [ ] **Step 3: Make ActionPlan copyable and cacheable**

In `src/core/action_plan.rs`, ensure:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ActionPlan {
    ordered_processes: Vec<ProcessId>,
}
```

Expose:

```rust
impl ActionPlan {
    pub fn empty() -> Self {
        Self {
            ordered_processes: Vec::new(),
        }
    }
}
```

- [ ] **Step 4: Add per-cell decision cache**

In `src/core/cell_store.rs`, add vectors:

```rust
action_plans: Vec<ActionPlan>,
next_genome_decision_due_ticks: Vec<u64>,
```

Initialize on insert:

```rust
self.action_plans.push(ActionPlan::empty());
self.next_genome_decision_due_ticks.push(0);
```

Expose:

```rust
pub fn action_plan(&self, index: CellIndex) -> &ActionPlan { ... }
pub fn set_action_plan(&mut self, index: CellIndex, plan: ActionPlan) { ... }
pub fn next_genome_decision_due_tick(&self, index: CellIndex) -> u64 { ... }
pub fn set_next_genome_decision_due_tick(&mut self, index: CellIndex, tick: u64) { ... }
```

- [ ] **Step 5: Refresh only when due**

In `src/core/tick.rs`, before per-cell process execution:

```rust
let current_tick = self.world.tick().raw();
let genome_cadence = config.scheduler.cell.genome_runtime_base_ticks.max(1);
let mut genome_decision_refresh_count = 0_u32;
```

For each living cell:

```rust
let should_refresh = self
    .world
    .cells()
    .next_genome_decision_due_tick(index)
    <= current_tick;

if should_refresh {
    let genome = self
        .world
        .cells()
        .genome_id(index)
        .and_then(|id| self.world.genome(id));
    let plan = ActionPlan::from_genome(genome);
    {
        let cells = self.world.cells_mut_for_commit();
        cells.set_action_plan(index, plan);
        cells.set_next_genome_decision_due_tick(index, current_tick + genome_cadence);
    }
    genome_decision_refresh_count += 1;
}

let action_plan = self.world.cells().action_plan(index).clone();
```

Add `genome_decision_refresh_count` to `MetricsSummary`.

- [ ] **Step 6: Preserve division behavior**

In `src/core/world.rs::execute_division`, when creating daughters, initialize their `ActionPlan` as empty and next due tick as current tick so daughters make their first Genome decision on their first executable Tick.

- [ ] **Step 7: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_genome_cadence
```

Expected: PASS.

- [ ] **Step 8: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test phase3a_action_plan --test phase3a_tick_integration --test runner_headless_e2e
```

Expected: PASS.

- [ ] **Step 9: Commit**

```powershell
git add src/core/action_plan.rs src/core/cell_store.rs src/core/world.rs src/core/tick.rs src/core/summary.rs tests/scheduler_genome_cadence.rs
git commit -m "feat(scheduler): cache genome action plans by cadence"
```

---

## Task 3: Scenario-level canonical scheduler defaults

**Files:**

- Modify: `config/scenarios/demo/demo_living_world.toml`
- Modify: `config/scenarios/genome/phase3a_genome_bootstrap.toml`
- Modify: targeted tests that require every-Tick Genome decision
- Test: `tests/scheduler_config.rs`

- [ ] **Step 1: Write failing test for canonical demo config**

Extend `tests/scheduler_config.rs`:

```rust
#[test]
fn demo_living_world_declares_canonical_scheduler_cadence() {
    let scenarios = alife::runner::scenario::scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "demo_living_world")
        .unwrap();
    let document = alife::runner::scenario::load_scenario_document(meta).unwrap();

    assert_eq!(document.runtime_config.scheduler.cell.genome_runtime_base_ticks, 10);
    assert_eq!(document.runtime_config.scheduler.world.resource_diffusion_ticks, 2);
    assert_eq!(document.runtime_config.scheduler.observer.resource_totals_ticks, 10);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_config demo_living_world_declares_canonical_scheduler_cadence
```

Expected: FAIL until demo config declares scheduler blocks.

- [ ] **Step 3: Add explicit scheduler blocks to demo scenario**

Add to `config/scenarios/demo/demo_living_world.toml`:

```toml
[time]
tick_duration_ms = 100
realtime_target_tps = 10
headless_target_tps = 50

[scheduler.cell]
genome_runtime_base_ticks = 10
genome_runtime_ticks_per_layer = 10
signal_emit_ticks = 2
controlled_reaction_ticks = 2
simple_synthesis_ticks = 5
basic_repair_ticks = 10
internal_rebalance_ticks = 5

[scheduler.world]
resource_diffusion_ticks = 2
resource_decay_ticks = 5
passive_reactions_ticks = 2
background_material_degradation_ticks = 5
environment_heat_diffusion_ticks = 2
field_update_ticks = 5

[scheduler.observer]
observer_metrics_ticks = 10
resource_totals_ticks = 10
graph_analysis_ticks = 50
debug_trace_ticks = 10
```

For tests that require old behavior, add:

```toml
[scheduler.cell]
genome_runtime_base_ticks = 1
genome_runtime_ticks_per_layer = 1
```

- [ ] **Step 4: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_config
```

Expected: PASS.

- [ ] **Step 5: Commit**

```powershell
git add config/scenarios/demo/demo_living_world.toml config/scenarios/genome/phase3a_genome_bootstrap.toml tests/scheduler_config.rs
git commit -m "config(scheduler): declare canonical cadence for demo scenarios"
```

---

## Task 4: World propagation cadence gates

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Test: `tests/scheduler_world_cadence.rs`

- [ ] **Step 1: Write failing world cadence test**

Create `tests/scheduler_world_cadence.rs`:

```rust
use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

fn config_with_world_cadence(diffusion_ticks: u64, decay_ticks: u64) -> alife::core::config::RuntimeConfig {
    RawScenarioConfig::parse(&format!(
        r#"
scenario_id = "scheduler_world_cadence"
seed = 12
tick_count = 20

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [100.0]
optional_decay_rate = 0.01

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 1.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.01
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 50.0
waste_death_threshold = 100.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 50.0

[scheduler.world]
resource_diffusion_ticks = {diffusion_ticks}
resource_decay_ticks = {decay_ticks}
passive_reactions_ticks = 1
"#
    ))
    .unwrap()
}

#[test]
fn resource_decay_runs_only_when_due_and_reports_elapsed_ticks() {
    let mut executor = TickExecutor::new(config_with_world_cadence(1, 5)).unwrap();

    for tick in 1..5 {
        let summary = executor.step().unwrap();
        assert_eq!(summary.metrics.resource_decay_scheduler_elapsed_ticks, 0, "tick {tick}");
        assert_eq!(summary.metrics.resource_decay_amount, 0.0, "tick {tick}");
    }

    let fifth = executor.step().unwrap();
    assert_eq!(fifth.metrics.resource_decay_scheduler_elapsed_ticks, 5);
    assert!(fifth.metrics.resource_decay_amount > 0.0);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_world_cadence
```

Expected: FAIL because scheduled elapsed metrics and gates do not exist.

- [ ] **Step 3: Add due helper**

In `src/core/tick.rs`, add:

```rust
fn is_due(committed_tick_before_step: u64, cadence_ticks: u64) -> bool {
    let cadence = cadence_ticks.max(1);
    (committed_tick_before_step + 1) % cadence == 0
}

fn elapsed_for_due(cadence_ticks: u64) -> u64 {
    cadence_ticks.max(1)
}
```

- [ ] **Step 4: Gate resource decay first**

Move resource decay so it only runs if:

```rust
let decay_due = is_due(self.world.tick().raw(), config.scheduler.world.resource_decay_ticks);
```

If due, apply elapsed integration. If the current resource grid decay function cannot accept elapsed ticks yet, call the existing one once and record elapsed; then add a follow-up test before changing physical integration.

Add metrics:

```rust
resource_decay_scheduler_elapsed_ticks: u64,
resource_decay_scheduler_skipped: bool,
```

- [ ] **Step 5: Gate diffusion and passive reactions**

Apply the same pattern to:

```text
resource_diffusion_ticks
passive_reactions_ticks
background_material_degradation_ticks
environment_heat_diffusion_ticks
```

For each system, add a focused metric field:

```text
*_scheduler_elapsed_ticks
*_scheduler_skipped
```

- [ ] **Step 6: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_world_cadence
```

Expected: PASS.

- [ ] **Step 7: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test phase2g_resource_types --test phase2g_tick_integration --test phase2g_determinism
```

Expected: PASS. If a regression test depends on every-Tick propagation, explicitly set the relevant scheduler cadence to `1` in its test config.

- [ ] **Step 8: Commit**

```powershell
git add src/core/tick.rs src/core/summary.rs tests/scheduler_world_cadence.rs
git commit -m "feat(scheduler): gate world propagation by cadence"
```

---

## Task 5: Snapshot cadence in RunEngine

**Files:**

- Modify: `src/runner/engine.rs`
- Modify: `src/runner/commands.rs` only if status projection needs latest committed tick plumbing
- Test: `tests/runner_snapshot_cadence.rs`

- [ ] **Step 1: Write failing snapshot cadence test**

Create `tests/runner_snapshot_cadence.rs`:

```rust
use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn document(id: &str) -> alife::runner::scenario_doc::ScenarioDocument {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios.iter().find(|scenario| scenario.id == id).unwrap();
    load_scenario_document(meta).unwrap()
}

#[test]
fn run_engine_keeps_latest_committed_state_without_snapshotting_every_tick() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine = RunEngine::prepare_from_document(
        &doc,
        RunEngineConfig {
            snapshot_buffer_size: 8,
            snapshot_every_ticks: 5,
        },
    )
    .unwrap();

    engine.start().unwrap();
    for _ in 0..4 {
        engine.run_one_tick().unwrap();
    }

    assert_eq!(engine.current_tick(), 4);
    assert_eq!(engine.snapshots().len(), 1);
    assert_eq!(engine.latest_committed_snapshot().tick.raw(), 4);

    engine.run_one_tick().unwrap();
    assert_eq!(engine.current_tick(), 5);
    assert_eq!(engine.snapshots().len(), 2);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_snapshot_cadence
```

Expected: FAIL because `snapshot_every_ticks` and `latest_committed_snapshot()` do not exist.

- [ ] **Step 3: Extend RunEngineConfig**

In `src/runner/engine.rs`:

```rust
pub struct RunEngineConfig {
    pub snapshot_buffer_size: usize,
    pub snapshot_every_ticks: u64,
}

impl Default for RunEngineConfig {
    fn default() -> Self {
        Self {
            snapshot_buffer_size: 300,
            snapshot_every_ticks: 1,
        }
    }
}
```

Add `latest_committed_snapshot: CommittedSnapshot` to `RunEngine`.

- [ ] **Step 4: Update commit_one_tick**

After `executor.step()?`:

```rust
let snapshot = CommittedSnapshot::from_world(executor.world());
self.latest_committed_snapshot = snapshot.clone();
if self.current_tick() % self.config.snapshot_every_ticks.max(1) == 0 {
    self.snapshots.push(snapshot);
}
```

Keep tick 0 snapshot in the ring buffer.

- [ ] **Step 5: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_snapshot_cadence
```

Expected: PASS.

- [ ] **Step 6: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_headless_e2e --test runner_http_run_control --test runner_ws_stream
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add src/runner/engine.rs tests/runner_snapshot_cadence.rs
git commit -m "feat(runner): make snapshot cadence explicit"
```

---

## Task 6: ViewerProjectionSampler

**Files:**

- Create: `src/viewer_server/projection_sampler.rs`
- Modify: `src/viewer_server/mod.rs`
- Modify: `src/viewer_server/state.rs`
- Modify: `src/runner/server_config.rs`
- Modify: `config/server.toml`
- Test: `tests/runner_projection_sampler.rs`

- [ ] **Step 1: Write failing sampler tests**

Create `tests/runner_projection_sampler.rs`:

```rust
use alife::viewer_server::projection_sampler::{ProjectionDecision, ViewerProjectionConfig, ViewerProjectionSampler};
use std::time::{Duration, Instant};

#[test]
fn sampler_allows_target_fps_but_drops_intermediate_frames() {
    let config = ViewerProjectionConfig {
        target_frames_per_second: 10,
        minimum_frames_per_second: 1,
        render_target_frames_per_second: 30,
        maximum_frame_age_ms: 1000,
        drop_intermediate_frames: true,
        latest_frame_only: true,
        force_frame_on_start: true,
        force_frame_on_pause: true,
        force_frame_on_step: true,
        force_frame_on_resume_if_stale: true,
        force_frame_on_terminal_state: true,
    };
    let mut sampler = ViewerProjectionSampler::new(config);
    let now = Instant::now();

    assert_eq!(sampler.on_committed_tick(1, now), ProjectionDecision::Emit);
    assert_eq!(sampler.on_committed_tick(2, now + Duration::from_millis(50)), ProjectionDecision::Skip);
    assert_eq!(sampler.on_committed_tick(3, now + Duration::from_millis(100)), ProjectionDecision::Emit);
}

#[test]
fn sampler_forces_pause_step_and_terminal_frames() {
    let mut sampler = ViewerProjectionSampler::new(ViewerProjectionConfig::default());
    let now = Instant::now();

    assert_eq!(sampler.on_pause(now), ProjectionDecision::EmitForced);
    assert_eq!(sampler.on_step(now), ProjectionDecision::EmitForced);
    assert_eq!(sampler.on_terminal(now), ProjectionDecision::EmitForced);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_projection_sampler
```

Expected: FAIL because sampler module does not exist.

- [ ] **Step 3: Implement sampler**

Create `src/viewer_server/projection_sampler.rs`:

```rust
use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProjectionDecision {
    Emit,
    EmitForced,
    Skip,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ViewerProjectionConfig {
    pub target_frames_per_second: u32,
    pub minimum_frames_per_second: u32,
    pub render_target_frames_per_second: u32,
    pub maximum_frame_age_ms: u64,
    pub drop_intermediate_frames: bool,
    pub latest_frame_only: bool,
    pub force_frame_on_start: bool,
    pub force_frame_on_pause: bool,
    pub force_frame_on_step: bool,
    pub force_frame_on_resume_if_stale: bool,
    pub force_frame_on_terminal_state: bool,
}

impl Default for ViewerProjectionConfig {
    fn default() -> Self {
        Self {
            target_frames_per_second: 10,
            minimum_frames_per_second: 1,
            render_target_frames_per_second: 30,
            maximum_frame_age_ms: 1000,
            drop_intermediate_frames: true,
            latest_frame_only: true,
            force_frame_on_start: true,
            force_frame_on_pause: true,
            force_frame_on_step: true,
            force_frame_on_resume_if_stale: true,
            force_frame_on_terminal_state: true,
        }
    }
}

pub struct ViewerProjectionSampler {
    config: ViewerProjectionConfig,
    last_emit_at: Option<Instant>,
    projection_sequence: u64,
}

impl ViewerProjectionSampler {
    pub fn new(config: ViewerProjectionConfig) -> Self {
        Self {
            config,
            last_emit_at: None,
            projection_sequence: 0,
        }
    }

    pub fn on_committed_tick(&mut self, _committed_tick: u64, now: Instant) -> ProjectionDecision {
        let interval = Duration::from_millis(1000 / self.config.target_frames_per_second.max(1) as u64);
        let should_emit = self
            .last_emit_at
            .map(|last| now.duration_since(last) >= interval)
            .unwrap_or(true);
        if should_emit {
            self.record_emit(now);
            ProjectionDecision::Emit
        } else {
            ProjectionDecision::Skip
        }
    }

    pub fn on_pause(&mut self, now: Instant) -> ProjectionDecision {
        self.force(now, self.config.force_frame_on_pause)
    }

    pub fn on_step(&mut self, now: Instant) -> ProjectionDecision {
        self.force(now, self.config.force_frame_on_step)
    }

    pub fn on_terminal(&mut self, now: Instant) -> ProjectionDecision {
        self.force(now, self.config.force_frame_on_terminal_state)
    }

    fn force(&mut self, now: Instant, enabled: bool) -> ProjectionDecision {
        if enabled {
            self.record_emit(now);
            ProjectionDecision::EmitForced
        } else {
            ProjectionDecision::Skip
        }
    }

    fn record_emit(&mut self, now: Instant) {
        self.last_emit_at = Some(now);
        self.projection_sequence += 1;
    }
}
```

- [ ] **Step 4: Wire config**

Add `[viewer_projection]` to `config/server.toml`:

```toml
[viewer_projection]
target_frames_per_second = 10
minimum_frames_per_second = 1
render_target_frames_per_second = 30
maximum_frame_age_ms = 1000
drop_intermediate_frames = true
latest_frame_only = true
force_frame_on_start = true
force_frame_on_pause = true
force_frame_on_step = true
force_frame_on_resume_if_stale = true
force_frame_on_terminal_state = true
```

Parse this in `src/runner/server_config.rs` and store `ViewerProjectionConfig`.

- [ ] **Step 5: Use sampler in tick loop**

In `src/viewer_server/state.rs`, replace interval math with sampler:

```rust
let decision = sampler.on_committed_tick(committed_tick, Instant::now());
let frame = match decision {
    ProjectionDecision::Emit | ProjectionDecision::EmitForced => engine.latest_committed_snapshot(),
    ProjectionDecision::Skip => None,
};
```

Do not hold the app mutex while sending WS messages.

- [ ] **Step 6: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_projection_sampler --test runner_ws_stream
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add src/viewer_server/projection_sampler.rs src/viewer_server/mod.rs src/viewer_server/state.rs src/runner/server_config.rs config/server.toml tests/runner_projection_sampler.rs
git commit -m "feat(viewer-server): add viewer projection sampler"
```

---

## Task 7: Forced projection on control transitions

**Files:**

- Modify: `src/viewer_server/api/run.rs`
- Modify: `src/viewer_server/state.rs`
- Test: `tests/runner_ws_stream.rs`

- [ ] **Step 1: Write failing WS tests**

Extend `tests/runner_ws_stream.rs`:

```rust
#[tokio::test]
async fn ws_receives_forced_binary_frame_after_step_even_when_fps_cadence_would_skip() {
    let (base_url, _) = spawn_test_server().await;
    let (mut ws, _) = tokio_tungstenite::connect_async(format!("{base_url}/stream").replace("http", "ws"))
        .await
        .unwrap();
    let _initial = next_text(&mut ws).await;

    let client = reqwest::Client::new();
    client
        .post(format!("{base_url}/run/start"))
        .json(&serde_json::json!({ "scenario_id": "world_baseline_stable" }))
        .send()
        .await
        .unwrap();
    client.post(format!("{base_url}/run/pause")).send().await.unwrap();
    client
        .post(format!("{base_url}/run/step"))
        .json(&serde_json::json!({}))
        .send()
        .await
        .unwrap();

    let binary = tokio::time::timeout(std::time::Duration::from_secs(1), async {
        loop {
            let msg = ws.next().await.unwrap().unwrap();
            if matches!(msg, tokio_tungstenite::tungstenite::Message::Binary(_)) {
                break;
            }
        }
    })
    .await;

    assert!(binary.is_ok(), "StepRun must force a viewer frame");
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_ws_stream ws_receives_forced_binary_frame_after_step_even_when_fps_cadence_would_skip
```

Expected: FAIL until forced frame broadcast exists.

- [ ] **Step 3: Add helper to broadcast latest frame**

In `src/viewer_server/state.rs`:

```rust
pub fn encode_latest_frame(state: &SharedState) -> Option<Vec<u8>> {
    let engine = state.engine.as_ref()?;
    let projection = WorldFrameProjection::from_committed_snapshot(engine.latest_committed_snapshot());
    Some(encode_world_frame(&projection))
}
```

- [ ] **Step 4: Force frames after pause/step/terminal**

In `src/viewer_server/api/run.rs`, after successful control commands:

```rust
let frame = {
    let locked = state.lock().unwrap();
    encode_latest_frame(&locked)
};
if let Some(bytes) = frame {
    let _ = broadcaster.send(WsMessage::Frame(bytes));
}
```

Keep status broadcasts as text messages.

- [ ] **Step 5: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_ws_stream
```

Expected: PASS.

- [ ] **Step 6: Commit**

```powershell
git add src/viewer_server/api/run.rs src/viewer_server/state.rs tests/runner_ws_stream.rs
git commit -m "feat(viewer-server): force projections on run control transitions"
```

---

## Task 8: Observer/resource totals cadence boundary

**Files:**

- Modify: `src/core/summary.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/observer/projection.rs`
- Test: `tests/scheduler_observer_cadence.rs`

- [ ] **Step 1: Write failing observer cadence test**

Create `tests/scheduler_observer_cadence.rs`:

```rust
use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn resource_totals_are_marked_stale_between_observer_cadence_ticks() {
    let config = RawScenarioConfig::parse(
        r#"
scenario_id = "scheduler_observer_cadence"
seed = 13
tick_count = 20

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient_A = 1.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.01
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 50.0
waste_death_threshold = 100.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 50.0

[scheduler.observer]
resource_totals_ticks = 10
"#,
    )
    .unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let first = executor.step().unwrap();
    assert!(!first.metrics.resource_totals_recomputed);

    for _ in 0..8 {
        let summary = executor.step().unwrap();
        assert!(!summary.metrics.resource_totals_recomputed);
    }

    let tenth = executor.step().unwrap();
    assert!(tenth.metrics.resource_totals_recomputed);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_observer_cadence
```

Expected: FAIL because observer cadence flags do not exist.

- [ ] **Step 3: Add explicit freshness fields**

In `MetricsSummary` add:

```rust
pub resource_totals_recomputed: bool,
pub observer_metrics_recomputed: bool,
pub graph_analysis_recomputed: bool,
```

These fields describe observer work, not mechanics.

- [ ] **Step 4: Gate expensive observer-only work**

In `TickExecutor::step()`, keep mechanics-critical accounting intact. For expensive observer-only totals, compute only when:

```rust
is_due(self.world.tick().raw(), config.scheduler.observer.resource_totals_ticks)
```

If a field is still required every Tick for conservation tests, keep it incremental and mark it as not observer recompute.

- [ ] **Step 5: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_observer_cadence
```

Expected: PASS.

- [ ] **Step 6: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test phase2_sweep_conservation --test phase2g_accounting --test phase2g_observer_outputs
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add src/core/summary.rs src/core/tick.rs src/observer/projection.rs tests/scheduler_observer_cadence.rs
git commit -m "feat(observer): make resource totals cadence explicit"
```

---

## Task 9: Frame metadata for interpolation

**Files:**

- Modify: `src/runner/projections.rs`
- Modify: `src/viewer_server/frame_encoder.rs`
- Test: `tests/runner_frame_encoder.rs`
- Test: `tests/runner_projection_world_frame.rs`

- [ ] **Step 1: Write failing frame metadata tests**

Extend `tests/runner_projection_world_frame.rs`:

```rust
#[test]
fn world_frame_projection_includes_interpolation_metadata() {
    let snapshot = load_snapshot_after_one_tick();
    let frame = WorldFrameProjection::from_committed_snapshot_with_metadata(
        &snapshot,
        7,
        1_725_000_000_000,
        Some(0),
    );

    assert_eq!(frame.projection_sequence, 7);
    assert_eq!(frame.wall_clock_generated_at_ms, 1_725_000_000_000);
    assert_eq!(frame.previous_committed_tick, Some(0));
}
```

Extend `tests/runner_frame_encoder.rs` to assert versioned metadata roundtrip. If changing binary format, bump ALIF frame version from `1` to `2` and keep decode errors clear for old/truncated frames.

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_projection_world_frame --test runner_frame_encoder
```

Expected: FAIL because metadata fields and encoder format do not exist.

- [ ] **Step 3: Add projection metadata**

In `src/runner/projections.rs` add:

```rust
pub projection_sequence: u64,
pub wall_clock_generated_at_ms: u64,
pub previous_committed_tick: Option<u64>,
```

Keep `from_committed_snapshot(...)` for existing callers with default metadata:

```rust
projection_sequence: 0,
wall_clock_generated_at_ms: 0,
previous_committed_tick: None,
```

- [ ] **Step 4: Version ALIF format**

In `src/viewer_server/frame_encoder.rs`, bump:

```rust
pub const VERSION: u8 = 2;
```

Encode metadata after current header and before cell records:

```text
projection_sequence: u64
wall_clock_generated_at_ms: u64
previous_committed_tick_present: u8
previous_committed_tick: u64
```

- [ ] **Step 5: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_projection_world_frame --test runner_frame_encoder
```

Expected: PASS.

- [ ] **Step 6: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_ws_stream --test runner_binary_serve
```

Expected: PASS.

- [ ] **Step 7: Commit**

```powershell
git add src/runner/projections.rs src/viewer_server/frame_encoder.rs tests/runner_frame_encoder.rs tests/runner_projection_world_frame.rs
git commit -m "feat(runner): add viewer frame interpolation metadata"
```

---

## Task 10: Performance and determinism acceptance

**Files:**

- Create: `tests/scheduler_determinism.rs`
- Create: `tests/scheduler_performance_smoke.rs`
- Modify: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-scheduler-cadence.md`

- [ ] **Step 1: Write determinism test**

Create `tests/scheduler_determinism.rs`:

```rust
use alife::core::snapshot::ViewerFrame;
use alife::core::tick::TickExecutor;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn run_summary(id: &str, ticks: usize) -> String {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios.iter().find(|scenario| scenario.id == id).unwrap();
    let document = load_scenario_document(meta).unwrap();
    let mut executor = TickExecutor::new(document.runtime_config).unwrap();
    for _ in 0..ticks {
        executor.step().unwrap();
    }
    let frame = ViewerFrame::from_world(executor.world());
    format!(
        "{}:{}:{}:{}",
        frame.tick.raw(),
        frame.cells.len(),
        frame.heat,
        frame.waste
    )
}

#[test]
fn scheduler_cadence_is_deterministic_for_same_seed_and_config() {
    assert_eq!(
        run_summary("demo_living_world", 200),
        run_summary("demo_living_world", 200)
    );
}
```

- [ ] **Step 2: Write performance smoke**

Create `tests/scheduler_performance_smoke.rs`:

```rust
use alife::core::tick::TickExecutor;
use alife::runner::scenario::{load_scenario_document, scan_scenarios};
use std::time::Instant;

#[test]
fn demo_living_world_reaches_minimum_headless_throughput() {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "demo_living_world")
        .unwrap();
    let document = load_scenario_document(meta).unwrap();
    let mut executor = TickExecutor::new(document.runtime_config).unwrap();

    let start = Instant::now();
    for _ in 0..250 {
        executor.step().unwrap();
    }
    let elapsed = start.elapsed().as_secs_f64();
    let tps = 250.0 / elapsed;

    assert!(tps >= 30.0, "expected at least 30 TPS, got {tps:.2}");
}
```

- [ ] **Step 3: Verify RED or baseline**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_determinism --test scheduler_performance_smoke
```

Expected:

- determinism PASS;
- performance may FAIL before enough cadence tasks are implemented.

If performance passes before optimization, keep the test as a regression guard.

- [ ] **Step 4: Full focused regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test scheduler_config --test scheduler_genome_cadence --test scheduler_world_cadence --test scheduler_observer_cadence --test scheduler_determinism --test scheduler_performance_smoke --test runner_ws_stream --test runner_headless_e2e
```

Expected: PASS.

- [ ] **Step 5: Full workspace verification**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 6: Write report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-scheduler-cadence.md` with:

```markdown
# Scheduler Cadence Implementation Report

## Summary

- Added scheduler/time/viewer cadence config.
- Added Genome ActionPlan cadence.
- Added world propagation cadence gates.
- Added explicit snapshot/projection cadence.
- Added observer/resource totals cadence.

## Verification

- cargo fmt --check
- CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace

## Performance

- demo_living_world TPS before:
- demo_living_world TPS after:
```

- [ ] **Step 7: Commit report**

```powershell
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-scheduler-cadence.md
git commit -m "docs(scheduler): report cadence implementation"
```

---

## Acceptance gate

Implementation is complete only when:

- `docs/engine/scheduler.md` remains consistent with implementation.
- Missing scheduler config keeps old behavior where tests depend on it.
- `demo_living_world` explicitly uses canonical scheduler cadence.
- Genome decision refresh is not every Tick by default in canonical scheduler config.
- Process progress and mandatory upkeep still run every Tick.
- World propagation systems integrate elapsed ticks or explicitly report scheduled execution.
- Viewer projection is wall-clock limited and latest-state only.
- Pause, StepRun, Completed, Failed force projection.
- Slow viewer cannot block simulation.
- Same seed + same config + same scheduler config remains deterministic.
- `cargo fmt --check` passes.
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace` passes.

## Self-review

- Spec coverage: four scheduler levels, Genome cadence, process progress separation, world propagation, viewer projection, forced projections, and deterministic commit boundaries are covered by Tasks 1-10.
- Placeholder scan: no task uses unresolved `TBD` or open-ended implementation language.
- Type consistency: planned public names are `SchedulerConfig`, `TimeConfig`, `ViewerProjectionSampler`, `ViewerProjectionConfig`, and `ProjectionDecision`.
- Scope: implementation is intentionally incremental and avoids a full `TickExecutor` rewrite in one task.
