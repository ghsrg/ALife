# Runner Debug Snapshot Cadence Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task. Use superpowers:test-driven-development for every production change. Use rust-domain-modeling when touching Core/Runner boundaries. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Remove every-Tick full snapshot work from headless debug runs, make snapshot cadence explicit, and make terminal debug output prove scheduler cadence is active before UI implementation starts.

**Architecture:** Keep authoritative `WorldState` commits every Tick. Replace numeric-only `snapshot_every_ticks` with an explicit `SnapshotCadence` policy so headless debug can use `OnDemandOnly` while server/UI paths keep their current compatibility behavior. CLI debug progress builds snapshots only when printing, and progress output includes scheduler diagnostics.

**Tech Stack:** Rust, Cargo integration tests, existing `RunEngine`, `TickExecutor`, `runner --debug`, scenario TOML configs.

---

## Canon and context

Read before implementation:

- `docs/PRINCIPLES.md`
- `docs/engine/scheduler.md`
- `docs/runner/projections.md`
- `docs/RUNNER_USAGE.md`
- `outputs/worklogs/2026-07-15-1011-PLAN-scheduler-cadence.md`
- `outputs/worklogs/2026-07-15-1510-REPORT-scheduler-cadence.md`

Current facts:

- `TickExecutor` commits authoritative simulation state every Tick.
- `RunEngineConfig::default()` currently uses `snapshot_every_ticks = 1`.
- `runner --debug` currently prepares `RunEngine` with `RunEngineConfig::default()`.
- `print_progress(...)` reads `engine.snapshots().newest()`, so debug status depends on cached snapshots.
- `CommittedSnapshot::from_world(...)` is a full read-only projection and should not be built every Tick for terminal debug.
- `demo_living_world` has scheduler defaults, but `[genome_templates.balanced].runtime_interval_ticks = 1` overrides the scheduler default and forces every-Tick Genome Runtime refresh for the demo template.
- At plan creation time the worktree already had an uncommitted related edit changing `demo_living_world` `runtime_interval_ticks` from `1` to `10`. Treat that as a pre-existing user/worktree change: either include it deliberately in Task 3 after tests verify it, or restore the clean baseline before executing the RED step.

Non-goals:

- Do not change Tick semantics.
- Do not cadence-gate diffusion/passive reactions/heat/field systems without elapsed-tick semantics.
- Do not make UI projection depend on `snapshot_every_ticks`.
- Do not add a profiler or broad performance refactor in this plan.

---

## Files and responsibilities

- `src/runner/engine.rs`
  - Add explicit `SnapshotCadence`.
  - Add `RunEngineConfig::headless_debug()`.
  - Keep `RunEngineConfig::default()` compatible for tests/server unless explicitly changed.
  - Store the latest `RunSummary` or a lightweight diagnostics struct from the last committed Tick.
  - Build cached snapshots only when `SnapshotCadence` says due.
  - Keep `latest_committed_snapshot()` on-demand and accurate.
- `src/bin/runner.rs`
  - Use `RunEngineConfig::headless_debug()` when `--debug` is enabled.
  - Make `print_progress(...)` build an on-demand latest snapshot only when printing.
  - Add visible diagnostics to progress output.
- `src/runner/progress.rs`
  - Add scheduler/debug diagnostic fields to `ProgressSnapshot`.
  - Render these fields in the terminal table.
- `config/scenarios/demo/demo_living_world.toml`
  - Remove the every-Tick Genome template override or set it to scheduler-compatible cadence.
- `tests/runner_snapshot_cadence.rs`
  - Cover `SnapshotCadence`.
  - Cover headless debug on-demand snapshots.
- `tests/runner_progress.rs`
  - Cover new debug fields in progress table.
- `tests/runner_scenario_loader.rs`
  - Cover demo effective Genome cadence.
- `tests/runner_headless_e2e.rs`
  - Cover debug/headless config helper without invoking CLI subprocess.

---

## Task 0: Baseline and branch

**Files:**

- No source changes.

- [ ] **Step 1: Confirm branch and worktree state**

Run:

```powershell
git status --short --branch
```

Expected:

```text
## main...origin/main [ahead N]
```

If unrelated local changes exist, record them in the implementation report and avoid editing those files.

- [ ] **Step 2: Create implementation branch**

Run:

```powershell
git switch -c codex/runner-debug-snapshot-cadence
```

Expected:

```text
Switched to a new branch 'codex/runner-debug-snapshot-cadence'
```

- [ ] **Step 3: Run focused baseline**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_snapshot_cadence --test runner_progress --test runner_headless_e2e --test runner_scenario_loader
```

Expected: all listed tests pass before changes.

---

## Task 1: Make snapshot cadence an explicit policy

**Files:**

- Modify: `src/runner/engine.rs`
- Modify: `tests/runner_snapshot_cadence.rs`

- [ ] **Step 1: Write failing tests for explicit snapshot cadence**

Extend `tests/runner_snapshot_cadence.rs`:

```rust
use alife::runner::engine::{RunEngine, RunEngineConfig, SnapshotCadence};
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn document(id: &str) -> alife::runner::scenario_doc::ScenarioDocument {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios.iter().find(|scenario| scenario.id == id).unwrap();
    load_scenario_document(meta).unwrap()
}

#[test]
fn snapshot_cadence_on_demand_only_never_builds_tick_cache_snapshots() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine = RunEngine::prepare_from_document(
        &doc,
        RunEngineConfig {
            snapshot_buffer_size: 4,
            snapshot_cadence: SnapshotCadence::OnDemandOnly,
        },
    )
    .unwrap();

    assert_eq!(engine.snapshot_build_count_for_test(), 1);
    assert_eq!(engine.snapshots().len(), 1);

    engine.start().unwrap();
    for _ in 0..10 {
        engine.run_one_tick().unwrap();
    }

    assert_eq!(engine.current_tick(), 10);
    assert_eq!(engine.snapshots().len(), 1);
    assert_eq!(engine.snapshot_build_count_for_test(), 1);

    let snapshot = engine.latest_committed_snapshot();
    assert_eq!(snapshot.tick.raw(), 10);
    assert_eq!(engine.snapshot_build_count_for_test(), 2);
}

#[test]
fn snapshot_cadence_every_n_ticks_replaces_legacy_numeric_field() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine = RunEngine::prepare_from_document(
        &doc,
        RunEngineConfig {
            snapshot_buffer_size: 8,
            snapshot_cadence: SnapshotCadence::EveryNTicks(5),
        },
    )
    .unwrap();

    engine.start().unwrap();
    for _ in 0..4 {
        engine.run_one_tick().unwrap();
    }

    assert_eq!(engine.current_tick(), 4);
    assert_eq!(engine.snapshots().len(), 1);
    assert_eq!(engine.snapshot_build_count_for_test(), 1);

    engine.run_one_tick().unwrap();
    assert_eq!(engine.current_tick(), 5);
    assert_eq!(engine.snapshots().len(), 2);
    assert_eq!(engine.snapshot_build_count_for_test(), 2);
}

#[test]
fn headless_debug_config_uses_on_demand_snapshots() {
    let config = RunEngineConfig::headless_debug();

    assert_eq!(config.snapshot_buffer_size, 4);
    assert_eq!(config.snapshot_cadence, SnapshotCadence::OnDemandOnly);
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_snapshot_cadence
```

Expected: FAIL because `SnapshotCadence`, `RunEngineConfig::snapshot_cadence`, and `RunEngineConfig::headless_debug()` do not exist.

- [ ] **Step 3: Add explicit `SnapshotCadence`**

In `src/runner/engine.rs`, replace the numeric field with:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SnapshotCadence {
    EveryTick,
    EveryNTicks(u64),
    OnDemandOnly,
}

impl SnapshotCadence {
    fn should_cache_after_tick(self, committed_tick: u64) -> bool {
        match self {
            Self::EveryTick => true,
            Self::EveryNTicks(ticks) => {
                let ticks = ticks.max(1);
                committed_tick % ticks == 0
            }
            Self::OnDemandOnly => false,
        }
    }
}
```

Update `RunEngineConfig`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RunEngineConfig {
    pub snapshot_buffer_size: usize,
    pub snapshot_cadence: SnapshotCadence,
}

impl Default for RunEngineConfig {
    fn default() -> Self {
        Self {
            snapshot_buffer_size: 300,
            snapshot_cadence: SnapshotCadence::EveryTick,
        }
    }
}

impl RunEngineConfig {
    pub const fn headless_debug() -> Self {
        Self {
            snapshot_buffer_size: 4,
            snapshot_cadence: SnapshotCadence::OnDemandOnly,
        }
    }
}
```

Update `commit_one_tick()`:

```rust
let committed_tick = executor.world().tick().raw();
if self.config.snapshot_cadence.should_cache_after_tick(committed_tick) {
    self.snapshots
        .push(CommittedSnapshot::from_world(executor.world()));
    self.snapshot_build_count += 1;
}
```

- [ ] **Step 4: Update existing struct literals**

Search:

```powershell
rg -n "snapshot_every_ticks|RunEngineConfig \\{" src tests
```

Replace existing `snapshot_every_ticks: 1` with:

```rust
snapshot_cadence: SnapshotCadence::EveryTick,
```

Replace existing `snapshot_every_ticks: 5` with:

```rust
snapshot_cadence: SnapshotCadence::EveryNTicks(5),
```

Add imports where needed:

```rust
use alife::runner::engine::SnapshotCadence;
```

or inside crate code:

```rust
use crate::runner::engine::SnapshotCadence;
```

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
cargo test --test runner_headless_e2e --test runner_http_run_control --test runner_ws_stream --test runner_projection_world_frame
```

Expected: PASS.

- [ ] **Step 7: Commit**

Run:

```powershell
git add src/runner/engine.rs src/viewer_server/api/run.rs tests/runner_snapshot_cadence.rs tests/runner_headless_e2e.rs tests/runner_projection_world_frame.rs
git commit -m "feat(runner): make snapshot cadence explicit"
```

---

## Task 2: Use on-demand snapshots for CLI debug progress

**Files:**

- Modify: `src/bin/runner.rs`
- Modify: `src/runner/engine.rs`
- Test: `tests/runner_headless_e2e.rs`

- [ ] **Step 1: Write failing test for headless debug engine behavior**

Extend `tests/runner_headless_e2e.rs`:

```rust
use alife::runner::engine::{RunEngine, RunEngineConfig};
use alife::runner::scenario::{load_scenario_document, scan_scenarios};

fn document(id: &str) -> alife::runner::scenario_doc::ScenarioDocument {
    let scenarios = scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios.iter().find(|scenario| scenario.id == id).unwrap();
    load_scenario_document(meta).unwrap()
}

#[test]
fn headless_debug_can_run_ticks_without_building_cached_snapshots() {
    let doc = document("bootstrap_minimal_viable_world");
    let mut engine =
        RunEngine::prepare_from_document(&doc, RunEngineConfig::headless_debug()).unwrap();

    engine.start().unwrap();
    for _ in 0..20 {
        engine.run_one_tick().unwrap();
    }

    assert_eq!(engine.current_tick(), 20);
    assert_eq!(engine.snapshot_build_count_for_test(), 1);

    let progress_snapshot = engine.latest_committed_snapshot();
    assert_eq!(progress_snapshot.tick.raw(), 20);
    assert_eq!(engine.snapshot_build_count_for_test(), 2);
}
```

- [ ] **Step 2: Verify RED or compile failure**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_headless_e2e headless_debug_can_run_ticks_without_building_cached_snapshots
```

Expected: FAIL if Task 1 was not completed; otherwise PASS. If it already passes after Task 1, keep it as regression coverage for CLI debug semantics.

- [ ] **Step 3: Use debug config in CLI**

In `src/bin/runner.rs`, replace:

```rust
let mut engine = RunEngine::prepare_from_document(&document, RunEngineConfig::default())
    .map_err(|err| err.to_string())?;
```

with:

```rust
let engine_config = if debug {
    RunEngineConfig::headless_debug()
} else {
    RunEngineConfig::default()
};
let mut engine = RunEngine::prepare_from_document(&document, engine_config)
    .map_err(|err| err.to_string())?;
```

- [ ] **Step 4: Make progress snapshots on-demand**

Change `print_progress` signature in `src/bin/runner.rs`:

```rust
fn print_progress(engine: &mut RunEngine, start: Instant) {
    let snapshot = engine.latest_committed_snapshot();
    // keep existing progress construction
}
```

Update call site:

```rust
print_progress(&mut engine, start);
```

Do not use `engine.snapshots().newest()` for debug progress. Terminal status must sample the latest committed state on progress interval only.

- [ ] **Step 5: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_headless_e2e headless_debug_can_run_ticks_without_building_cached_snapshots
```

Expected: PASS.

- [ ] **Step 6: Manual smoke**

Run a short debug smoke:

```powershell
cargo run --release --bin runner -- --debug --progress-interval-ms 2000 bootstrap_minimal_viable_world
```

Expected:

- progress table appears;
- run completes;
- no panic;
- final tick is the configured scenario tick.

- [ ] **Step 7: Commit**

Run:

```powershell
git add src/bin/runner.rs tests/runner_headless_e2e.rs
git commit -m "fix(runner): sample debug snapshots on demand"
```

---

## Task 3: Fix demo Genome cadence so scheduler is actually active

**Files:**

- Modify: `config/scenarios/demo/demo_living_world.toml`
- Modify: `tests/runner_scenario_loader.rs`

- [ ] **Step 1: Write failing test for demo effective Genome cadence**

Extend `tests/runner_scenario_loader.rs` in the existing demo test or as a new test:

```rust
#[test]
fn demo_living_world_uses_non_every_tick_genome_cadence() {
    let scenarios = alife::runner::scenario::scan_scenarios("config/scenarios").unwrap();
    let meta = scenarios
        .iter()
        .find(|scenario| scenario.id == "demo_living_world")
        .unwrap();
    let document = alife::runner::scenario::load_scenario_document(meta).unwrap();
    let config = document.runtime_config;
    let template = config
        .genome_templates
        .iter()
        .find(|template| template.id().as_str() == "balanced")
        .unwrap();

    assert!(
        config.effective_genome_runtime_cadence_ticks_for_template(template) > 1,
        "demo_living_world must not override Genome Runtime back to every Tick"
    );
}
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_scenario_loader demo_living_world_uses_non_every_tick_genome_cadence
```

Expected: FAIL because current `runtime_interval_ticks = 1` overrides the scheduler default.

If the worktree already contains the pre-existing `runtime_interval_ticks = 10` edit, this test may PASS immediately. In that case, record that Task 3's production config change was already present before execution, keep the test, and commit the config edit together with the test in Task 3.

- [ ] **Step 3: Update demo config**

In `config/scenarios/demo/demo_living_world.toml`, change:

```toml
[genome_templates.balanced]
variation_amplitude = 0.08
runtime_interval_ticks = 1
```

to:

```toml
[genome_templates.balanced]
variation_amplitude = 0.08
runtime_interval_ticks = 10
regulatory_depth = 1
```

Rationale:

- explicit template override remains visible;
- effective cadence is `10 + regulatory_depth * genome_runtime_ticks_per_layer` if that is the current helper semantics;
- if implementation semantics are `runtime_interval_ticks + (regulatory_depth - 1) * layer_ticks`, the effective cadence is still greater than `1`;
- tests should assert `> 1`, not a brittle exact value unless the current helper contract is already exact in existing tests.

- [ ] **Step 4: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_scenario_loader demo_living_world_uses_non_every_tick_genome_cadence
```

Expected: PASS.

- [ ] **Step 5: Regression**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_scenario_loader --test scheduler_genome_cadence --test scheduler_determinism
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add config/scenarios/demo/demo_living_world.toml tests/runner_scenario_loader.rs
git commit -m "config(demo): use scheduled genome cadence"
```

---

## Task 4: Add scheduler/debug counters to terminal progress

**Files:**

- Modify: `src/runner/engine.rs`
- Modify: `src/runner/progress.rs`
- Modify: `src/bin/runner.rs`
- Test: `tests/runner_progress.rs`
- Test: `tests/runner_headless_e2e.rs`

- [ ] **Step 1: Write failing progress formatting test**

Update `tests/runner_progress.rs`:

```rust
#[test]
fn progress_table_contains_scheduler_diagnostics() {
    let rendered = format_progress_table(&ProgressSnapshot {
        elapsed_ms: 2500,
        tick: 10,
        max_ticks: 20,
        ticks_per_second: 40.0,
        cells: 3,
        alive_cells: Some(2),
        dead_cells: Some(1),
        heat: 1.5,
        waste: 2.5,
        state: "Running".to_string(),
        collapse_reason: None,
        snapshot_builds: 2,
        genome_refreshes: 0,
        resource_decay_elapsed_ticks: 5,
    });

    assert!(rendered.contains("snapshots"));
    assert!(rendered.contains("genome"));
    assert!(rendered.contains("decay_dt"));
    assert!(rendered.contains("2"));
    assert!(rendered.contains("5"));
}
```

Also update the existing `progress_table_contains_required_status_fields` fixture to include:

```rust
snapshot_builds: 1,
genome_refreshes: 0,
resource_decay_elapsed_ticks: 0,
```

- [ ] **Step 2: Verify RED**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_progress
```

Expected: FAIL because the new `ProgressSnapshot` fields do not exist.

- [ ] **Step 3: Store last tick diagnostics in `RunEngine`**

In `src/runner/engine.rs`, add:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RunEngineDiagnostics {
    pub last_genome_decision_refresh_count: u32,
    pub last_resource_decay_scheduler_elapsed_ticks: u64,
}
```

Add field:

```rust
diagnostics: RunEngineDiagnostics,
```

Initialize:

```rust
diagnostics: RunEngineDiagnostics::default(),
```

Expose:

```rust
pub const fn diagnostics(&self) -> RunEngineDiagnostics {
    self.diagnostics
}
```

Update `commit_one_tick()`:

```rust
let summary = executor.step()?;
self.diagnostics = RunEngineDiagnostics {
    last_genome_decision_refresh_count: summary.metrics.genome_decision_refresh_count,
    last_resource_decay_scheduler_elapsed_ticks: summary
        .metrics
        .resource_decay_scheduler_elapsed_ticks,
};
```

Then use `executor.world()` for snapshot cadence as before.

- [ ] **Step 4: Add progress fields and render them**

In `src/runner/progress.rs`, add fields:

```rust
pub snapshot_builds: u64,
pub genome_refreshes: u32,
pub resource_decay_elapsed_ticks: u64,
```

Update table format to include a second compact diagnostics line or additional columns. Prefer a second line to keep the main table readable:

```rust
format!(
    "\
...existing table...\n\
| snapshots | genome | decay_dt |\n\
| {snapshot_builds:<9} | {genome_refreshes:<6} | {decay_dt:<8} |",
    snapshot_builds = snapshot.snapshot_builds,
    genome_refreshes = snapshot.genome_refreshes,
    decay_dt = snapshot.resource_decay_elapsed_ticks,
)
```

Keep exact spacing flexible; tests should check labels and values, not full table equality.

- [ ] **Step 5: Wire CLI progress diagnostics**

In `src/bin/runner.rs`, inside `print_progress(...)`:

```rust
let diagnostics = engine.diagnostics();
let progress = ProgressSnapshot {
    // existing fields
    snapshot_builds: engine.snapshot_build_count_for_test(),
    genome_refreshes: diagnostics.last_genome_decision_refresh_count,
    resource_decay_elapsed_ticks: diagnostics.last_resource_decay_scheduler_elapsed_ticks,
};
```

If `snapshot_build_count_for_test()` feels test-only, rename it to:

```rust
pub const fn snapshot_build_count(&self) -> u64
```

and keep `snapshot_build_count_for_test()` as a delegating compatibility helper:

```rust
pub const fn snapshot_build_count_for_test(&self) -> u64 {
    self.snapshot_build_count()
}
```

- [ ] **Step 6: Add headless diagnostic regression**

Extend `tests/runner_headless_e2e.rs`:

```rust
#[test]
fn run_engine_records_scheduler_diagnostics_from_last_tick() {
    let doc = document("demo_living_world");
    let mut engine =
        RunEngine::prepare_from_document(&doc, RunEngineConfig::headless_debug()).unwrap();

    engine.start().unwrap();
    engine.run_one_tick().unwrap();

    let diagnostics = engine.diagnostics();
    assert!(
        diagnostics.last_genome_decision_refresh_count <= engine.latest_committed_snapshot().cells.len() as u32
    );
}
```

This test intentionally checks availability and sane bounds, not exact cadence timing.

- [ ] **Step 7: Verify GREEN**

Run:

```powershell
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_progress --test runner_headless_e2e
```

Expected: PASS.

- [ ] **Step 8: Manual release smoke**

Run:

```powershell
cargo run --release --bin runner -- --debug --progress-interval-ms 2000 demo_living_world
```

Expected:

- progress table prints every ~2000 ms;
- `snapshots` increases only when progress is printed plus initial snapshot;
- `genome` is often `0` between refresh ticks, proving Genome Runtime is not every Tick;
- `decay_dt` is `5` only on resource decay cadence ticks and `0` otherwise;
- TPS should be measured from release build, not debug build.

- [ ] **Step 9: Commit**

Run:

```powershell
git add src/runner/engine.rs src/runner/progress.rs src/bin/runner.rs tests/runner_progress.rs tests/runner_headless_e2e.rs
git commit -m "feat(runner): expose scheduler diagnostics in debug progress"
```

---

## Task 5: Documentation and final report

**Files:**

- Modify: `docs/RUNNER_USAGE.md`
- Modify: `docs/engine/scheduler.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-runner-debug-snapshot-cadence.md`

- [ ] **Step 1: Document runner debug semantics**

Update `docs/RUNNER_USAGE.md` with a short section:

```markdown
## Debug progress and snapshots

`--debug` prints a terminal progress table at `--progress-interval-ms`.
The table samples the latest committed state on demand. It does not build a full
`CommittedSnapshot` after every simulation Tick.

Use release builds for performance checks:

```powershell
cargo run --release --bin runner -- --debug --progress-interval-ms 2000 demo_living_world
```

The debug table includes scheduler diagnostics:

- `snapshots`: number of full snapshots built;
- `genome`: Genome Runtime refreshes during the last committed Tick;
- `decay_dt`: elapsed Tick integration used by scheduled resource decay.
```

- [ ] **Step 2: Clarify scheduler snapshot/projection boundary**

Update `docs/engine/scheduler.md` implementation notes so they do not say `RunEngine` stores a committed snapshot after every Tick as current behavior. Replace that note with:

```markdown
- `RunEngine` supports explicit snapshot cache cadence. Headless debug uses
  on-demand snapshots for terminal progress. Viewer projection remains governed
  by `ViewerProjectionSampler`, not by the snapshot cache cadence.
```

- [ ] **Step 3: Write report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-runner-debug-snapshot-cadence.md`:

```markdown
# Runner Debug Snapshot Cadence Report

## Summary

- Added explicit `SnapshotCadence`.
- Made headless debug use on-demand snapshots.
- Fixed `demo_living_world` Genome cadence override.
- Added scheduler diagnostics to terminal progress.

## Verification

- `cargo fmt --check`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_snapshot_cadence --test runner_progress --test runner_headless_e2e --test runner_scenario_loader`
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace`
- `cargo run --release --bin runner -- --debug --progress-interval-ms 2000 demo_living_world`

## Notes

- UI projection remains wall-clock limited through `ViewerProjectionSampler`.
- `SnapshotCadence` is cache/debug policy, not simulation authority.
```

- [ ] **Step 4: Verify docs references**

Run:

```powershell
rg -n "snapshot_every_ticks|stores a committed snapshot after every Tick|runtime_interval_ticks = 1" docs config src tests
```

Expected:

- no stale `snapshot_every_ticks` references;
- no claim that every-Tick snapshotting is the current required behavior;
- no `runtime_interval_ticks = 1` in `demo_living_world`.

- [ ] **Step 5: Final verification**

Run:

```powershell
cargo fmt --check
$env:CARGO_PROFILE_TEST_DEBUG='0'
cargo test --test runner_snapshot_cadence --test runner_progress --test runner_headless_e2e --test runner_scenario_loader --test scheduler_genome_cadence --test scheduler_determinism
cargo test --workspace
```

Expected: PASS.

- [ ] **Step 6: Commit**

Run:

```powershell
git add docs/RUNNER_USAGE.md docs/engine/scheduler.md outputs/worklogs/*REPORT-runner-debug-snapshot-cadence.md
git commit -m "docs(runner): report debug snapshot cadence fix"
```

---

## Acceptance gate

Implementation is complete only when:

- `demo_living_world` effective Genome Runtime cadence is greater than `1`.
- `runner --debug` does not build cached full snapshots every Tick.
- `runner --debug` still prints accurate latest committed state at progress intervals.
- `SnapshotCadence` has explicit modes and no magic large integer is used for on-demand behavior.
- `SnapshotCadence` is not treated as UI FPS control.
- UI/server projection remains governed by `ViewerProjectionSampler`.
- Progress table exposes enough diagnostics to see whether scheduler is active:
  - snapshot build count;
  - last Tick Genome refresh count;
  - last Tick resource decay elapsed ticks.
- Existing Runner HTTP/WS tests still pass.
- `cargo fmt --check` passes.
- `CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace` passes.

## Expected manual validation

Use release build for performance checks:

```powershell
cargo run --release --bin runner -- --debug --progress-interval-ms 2000 demo_living_world
```

Expected qualitative result:

- terminal progress remains human-readable;
- snapshot count grows with progress prints, not with Tick count;
- Genome refresh count is not equal to total cell count every printed row;
- TPS comparison is made against previous release build, not debug build.

## Self-review

- Spec coverage: closes the observed issue by addressing demo Genome override, headless debug snapshot policy, and terminal diagnostics.
- Placeholder scan: no `TBD`/`TODO`/open implementation placeholders.
- Type consistency: uses `SnapshotCadence`, `RunEngineConfig::headless_debug`, `RunEngineDiagnostics`, and `ProgressSnapshot` consistently.
- Scope: intentionally does not optimize world propagation systems lacking elapsed-tick semantics and does not start UI implementation.
