# Phase 1D Sustained Viability Gates Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Close Phase 1 with deterministic sustained-viability gates: long-run resource-loop survival, resource exhaustion collapse, heat/waste bound checks, and observer-only growth-readiness metrics without implementing division or Process Registry.

**Architecture:** Keep `alife-core` authoritative and single-thread deterministic. Phase 1D adds run-level metrics and scenario gates around existing Phase 1C resource interaction; it does not add active growth, division, Genome, Feasibility, or registered Processes. Growth readiness is a derived observer metric in `RunSummary`, not Cell behavior and not a simulation input.

**Tech Stack:** Rust 2024, existing Cargo integration tests, existing TOML runner adapter, no new dependencies.

---

## Review Result For Phase 1C

Phase 1C is acceptable to build on.

Verified locally:

```text
cargo test: pass
cargo fmt --check: pass
cargo clippy --workspace --all-targets --all-features -- -D warnings: pass
scenario dump matches 1C report
```

Non-blocking observations to keep in mind:

```text
TickExecutor::step is now large and phase-heavy.
Internal Cell resources are still one aggregate ResourceAmount.
Growth/division must not be smuggled into Phase 1D behavior.
```

Phase 1D should therefore add metrics and gates, not a new biological subsystem.

---

## Authority And Scope

Phase 1D implements:

```text
Run-level viability metrics across multi-tick runs
min/max Energy tracking
min/max Heat and Waste tracking
final internal Resource amount
final external Resource total
resource exhaustion scenario
sustained resource-loop scenario over 1_000 ticks
heat/waste bound scenarios over long runs
observer-only growth_readiness estimate
Phase 1 completion report
```

Phase 1D must not implement:

```text
Cell division
active growth that changes radius or mass
variable Cell radius
Genome Runtime
ActionPlan / Feasibility
Process Registry
Joints
signals
diffusion
multi-resource internal inventory
fitness/selection logic
```

Cell size rule:

```text
Cell radius remains fixed for the run.
Radius may be projected and reported, but it does not change and does not control uptake area in Phase 1D.
```

Growth readiness rule:

```text
growth_readiness is observer-only.
Cells cannot read it.
It does not affect Tick behavior, lifecycle, resource uptake, metabolism, or survival.
```

---

## Required Reading

Before editing code, read:

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/implementation/phase-1-design.md`
- `docs/implementation/phase-1-data-model.md`
- `docs/implementation/phase-1-module-api.md`
- `outputs/worklogs/2026-07-02-1805-REPORT-phase-1C-resource-interaction-smoke.md`

---

## File Structure

Modify:

```text
src/core/summary.rs
src/core/tick.rs
src/core/cell_store.rs
tests/phase1_config_validation.rs
tests/phase1_resource_interaction.rs
outputs/worklogs/index.md
```

Create:

```text
tests/phase1_sustained_viability.rs
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1D-sustained-viability-gates.md
```

Do not modify:

```text
tools/early-stability/*
```

Do not change scenario TOML files unless the user explicitly approves a config migration.

Do not commit automatically unless the user explicitly asks.

---

## Task 1: Baseline Verification

**Files:**

- Read only.

- [ ] **Step 1: Run current Rust tests**

Run:

```bash
cargo test
```

Expected:

```text
all current Rust tests pass
```

- [ ] **Step 2: Run formatter and linter**

Run:

```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected:

```text
both commands pass
```

- [ ] **Step 3: Record baseline**

Record in working notes:

```text
baseline cargo test result
baseline cargo fmt --check result
baseline cargo clippy result
active Rust test count
```

Do not edit code in this task.

---

## Task 2: Add Public Cell Resource And Capacity Metrics Needed By Summaries

**Files:**

- Modify: `src/core/cell_store.rs`
- Create: `tests/phase1_sustained_viability.rs`

- [ ] **Step 1: Write failing CellStore summary accessor test**

Create `tests/phase1_sustained_viability.rs`:

```rust
use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

fn one_cell_store() -> CellStore {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(8.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::new(3.0).unwrap(),
        materials: MaterialAmount::new(4.0).unwrap(),
        capacity_limit: CapacityAmount::new(12.0).unwrap(),
        temperature: Temperature::new(25.0),
    });
    cells
}

#[test]
fn cell_store_exposes_capacity_limit_for_observer_summary() {
    let cells = one_cell_store();

    assert_eq!(cells.capacity_limit(CellIndex::from_raw(0)).raw(), 12.0);
    assert_eq!(cells.used_capacity(CellIndex::from_raw(0)).raw(), 7.0);
    assert_eq!(cells.free_capacity(CellIndex::from_raw(0)).raw(), 5.0);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_sustained_viability cell_store_exposes_capacity_limit_for_observer_summary
```

Expected:

```text
FAIL: missing CellStore::capacity_limit
```

- [ ] **Step 3: Implement capacity_limit accessor**

In `src/core/cell_store.rs`, add:

```rust
pub fn capacity_limit(&self, index: CellIndex) -> CapacityAmount {
    self.capacity_limits[index.raw()]
}
```

- [ ] **Step 4: Run test**

Run:

```bash
cargo test --test phase1_sustained_viability cell_store_exposes_capacity_limit_for_observer_summary
```

Expected:

```text
test cell_store_exposes_capacity_limit_for_observer_summary ... ok
```

---

## Task 3: Extend MetricsSummary With Viability Fields

**Files:**

- Modify: `src/core/summary.rs`
- Modify: `src/core/tick.rs`
- Modify: existing tests that construct or compare `RunSummary`
- Modify: `tests/phase1_sustained_viability.rs`

New metrics:

```rust
pub min_energy: f32,
pub max_energy: f32,
pub final_internal_resources: f32,
pub final_external_resources: f32,
pub final_used_capacity: f32,
pub final_free_capacity: f32,
pub growth_readiness: bool,
```

Rules:

```text
growth_readiness is observer-only.
growth_readiness is true when final Energy is at least 80% of capacity and final free capacity is positive.
growth_readiness must not affect behavior.
```

- [ ] **Step 1: Write failing summary metrics test**

Append to `tests/phase1_sustained_viability.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::summary::{CollapseReason, SurvivalResult};
use alife::core::tick::TickExecutor;
use alife::core::units::{HeatAmount, Seed, Tick, WasteAmount, WorldSize};

fn viability_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(3),
            seed: Seed::from_raw(1),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 3.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        },
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(10.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(20.0).unwrap(),
            initial_resource_amount: ResourceAmount::zero(),
            initial_material_amount: MaterialAmount::new(4.0).unwrap(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(10.0).unwrap(),
            waste_death_threshold: WasteAmount::new(20.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(2.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.25,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn run_summary_reports_resource_capacity_and_growth_readiness_metrics() {
    let mut executor = TickExecutor::new(viability_config()).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert!(summary.metrics.max_energy >= summary.metrics.min_energy);
    assert_eq!(summary.metrics.final_internal_resources, 0.0);
    assert!(summary.metrics.final_external_resources > 0.0);
    assert_eq!(summary.metrics.final_used_capacity, 4.0);
    assert_eq!(summary.metrics.final_free_capacity, 16.0);
    assert!(summary.metrics.growth_readiness);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_sustained_viability run_summary_reports_resource_capacity_and_growth_readiness_metrics
```

Expected:

```text
FAIL: MetricsSummary fields are missing
```

- [ ] **Step 3: Extend MetricsSummary**

In `src/core/summary.rs`, change `MetricsSummary`:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetricsSummary {
    pub final_energy: f32,
    pub heat: f32,
    pub waste: f32,
    pub min_energy: f32,
    pub max_energy: f32,
    pub final_internal_resources: f32,
    pub final_external_resources: f32,
    pub final_used_capacity: f32,
    pub final_free_capacity: f32,
    pub growth_readiness: bool,
}
```

- [ ] **Step 4: Add TickExecutor summary helper**

In `src/core/tick.rs`, import:

```rust
use crate::core::resources::ResourceLayerIndex;
```

If already imported, do not duplicate it.

Add private helper methods inside `impl TickExecutor`:

```rust
fn aggregate_external_resources(&self) -> f32 {
    (0..self.world.resources().layer_count())
        .map(|layer| {
            self.world
                .resources()
                .total_amount_for_layer(ResourceLayerIndex::from_raw(layer))
                .expect("layer range is derived from layer_count")
                .raw()
        })
        .sum()
}

fn build_metrics_summary(
    &self,
    final_energy: f32,
    heat: f32,
    waste: f32,
    min_energy: f32,
    max_energy: f32,
) -> MetricsSummary {
    let first = CellIndex::from_raw(0);
    let cells = self.world.cells();
    let final_internal_resources = cells.resource_amount(first).raw();
    let final_used_capacity = cells.used_capacity(first).raw();
    let final_free_capacity = cells.free_capacity(first).raw();
    let energy_capacity = cells.energy(first).capacity().raw();
    let growth_readiness = final_energy >= energy_capacity * 0.8 && final_free_capacity > 0.0;

    MetricsSummary {
        final_energy,
        heat,
        waste,
        min_energy,
        max_energy,
        final_internal_resources,
        final_external_resources: self.aggregate_external_resources(),
        final_used_capacity,
        final_free_capacity,
        growth_readiness,
    }
}
```

Phase 1 currently has one Cell. This helper may use `CellIndex::from_raw(0)` while Phase 1 remains one-cell-only.

- [ ] **Step 5: Update `step()` metrics**

Inside `TickExecutor::step`, after the lifecycle loop has computed `final_energy`, add:

```rust
let min_energy = final_energy;
let max_energy = final_energy;
```

Replace:

```rust
metrics: MetricsSummary {
    final_energy,
    heat: heat_next.raw(),
    waste: waste_next.raw(),
},
```

with:

```rust
metrics: self.build_metrics_summary(
    final_energy,
    heat_next.raw(),
    waste_next.raw(),
    min_energy,
    max_energy,
),
```

This makes one-step summaries compile. Multi-tick min/max aggregation is added in the next task.

- [ ] **Step 6: Update existing direct `MetricsSummary` construction**

If any tests directly construct `MetricsSummary`, add the new fields:

```rust
min_energy: final_energy,
max_energy: final_energy,
final_internal_resources: 0.0,
final_external_resources: 0.0,
final_used_capacity: 0.0,
final_free_capacity: 0.0,
growth_readiness: false,
```

- [ ] **Step 7: Run summary metrics test**

Run:

```bash
cargo test --test phase1_sustained_viability run_summary_reports_resource_capacity_and_growth_readiness_metrics
```

Expected:

```text
test run_summary_reports_resource_capacity_and_growth_readiness_metrics ... ok
```

---

## Task 4: Aggregate Min/Max Energy Across `run_until_configured_tick`

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `tests/phase1_sustained_viability.rs`

- [ ] **Step 1: Write failing min/max aggregation test**

Append to `tests/phase1_sustained_viability.rs`:

```rust
#[test]
fn run_until_configured_tick_tracks_energy_range_across_ticks() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(4);
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(10.0).unwrap();
    config.resource_interaction.energy_per_resource = 3.0;

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.metrics.min_energy, 2.0);
    assert_eq!(summary.metrics.max_energy, 5.0);
}
```

Expected energy after each Tick:

```text
tick 1: 1 + 3 - 2 = 2
tick 2: 2 + 3 - 2 = 3
tick 3: 3 + 3 - 2 = 4
tick 4: 4 + 3 - 2 = 5
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_sustained_viability run_until_configured_tick_tracks_energy_range_across_ticks
```

Expected:

```text
FAIL: min_energy/max_energy only reflect latest step
```

- [ ] **Step 3: Aggregate in `run_until_configured_tick`**

In `src/core/tick.rs`, replace `run_until_configured_tick` with:

```rust
pub fn run_until_configured_tick(&mut self) -> Result<RunSummary, TickError> {
    let target = self.world.config().world.tick_count.raw();
    let mut latest = self.step()?;
    let mut min_energy = latest.metrics.final_energy;
    let mut max_energy = latest.metrics.final_energy;

    if latest.survival_result == SurvivalResult::Collapse {
        latest.metrics.min_energy = min_energy;
        latest.metrics.max_energy = max_energy;
        return Ok(latest);
    }

    while self.world.tick().raw() < target {
        latest = self.step()?;
        min_energy = min_energy.min(latest.metrics.final_energy);
        max_energy = max_energy.max(latest.metrics.final_energy);
        if latest.survival_result == SurvivalResult::Collapse {
            break;
        }
    }

    latest.metrics.min_energy = min_energy;
    latest.metrics.max_energy = max_energy;
    Ok(latest)
}
```

- [ ] **Step 4: Run aggregation test**

Run:

```bash
cargo test --test phase1_sustained_viability run_until_configured_tick_tracks_energy_range_across_ticks
```

Expected:

```text
test run_until_configured_tick_tracks_energy_range_across_ticks ... ok
```

---

## Task 5: Add Sustained Viability Gate

**Files:**

- Modify: `tests/phase1_sustained_viability.rs`

- [ ] **Step 1: Write long-run sustained viability test**

Append to `tests/phase1_sustained_viability.rs`:

```rust
#[test]
fn cell_remains_stable_for_1000_ticks_on_local_resource_loop() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(1_000);
    config.resources.initial_distribution = vec![ResourceAmount::new(2_000.0).unwrap()];
    config.cell.initial_energy = EnergyAmount::new(5.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(20.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 2.2;
    config.resource_interaction.heat_per_resource = 0.01;
    config.resource_interaction.waste_per_resource = 0.01;
    config.environment.heat_dissipation_rate = HeatAmount::new(0.02).unwrap();
    config.environment.waste_sink_rate = WasteAmount::new(0.02).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.tick.raw(), 1_000);
    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert!(summary.metrics.min_energy > 0.0);
    assert!(summary.metrics.heat < 10.0);
    assert!(summary.metrics.waste < 10.0);
}
```

- [ ] **Step 2: Run sustained viability test**

Run:

```bash
cargo test --test phase1_sustained_viability cell_remains_stable_for_1000_ticks_on_local_resource_loop
```

Expected:

```text
test cell_remains_stable_for_1000_ticks_on_local_resource_loop ... ok
```

If this fails, do not tune by hiding collapse. Diagnose whether the failure is from:

```text
resource depletion
energy budget
heat accumulation
waste accumulation
capacity overrun
```

Then adjust only the test config values to represent a stable intended scenario.

---

## Task 6: Add Resource Exhaustion Gate

**Files:**

- Modify: `tests/phase1_sustained_viability.rs`

- [ ] **Step 1: Write resource exhaustion collapse test**

Append to `tests/phase1_sustained_viability.rs`:

```rust
#[test]
fn cell_collapses_when_local_resources_are_exhausted() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(20);
    config.resources.initial_distribution = vec![ResourceAmount::new(2.0).unwrap()];
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(10.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.resource_interaction.energy_per_resource = 2.5;

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::MandatoryCostUnpaid);
    assert!(summary.tick.raw() < 20);
    assert_eq!(summary.metrics.final_external_resources, 0.0);
}
```

- [ ] **Step 2: Run resource exhaustion test**

Run:

```bash
cargo test --test phase1_sustained_viability cell_collapses_when_local_resources_are_exhausted
```

Expected:

```text
test cell_collapses_when_local_resources_are_exhausted ... ok
```

---

## Task 7: Add Heat/Waste Long-Run Bound Gates

**Files:**

- Modify: `tests/phase1_sustained_viability.rs`

- [ ] **Step 1: Write heat accumulation collapse test**

Append to `tests/phase1_sustained_viability.rs`:

```rust
#[test]
fn sustained_metabolism_collapses_when_heat_has_no_sufficient_sink() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(50);
    config.resources.initial_distribution = vec![ResourceAmount::new(100.0).unwrap()];
    config.resource_interaction.heat_per_resource = 1.0;
    config.resource_interaction.waste_per_resource = 0.0;
    config.environment.heat_dissipation_rate = HeatAmount::zero();
    config.environment.heat_warning_threshold = HeatAmount::new(5.0).unwrap();
    config.environment.heat_death_threshold = HeatAmount::new(8.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::HeatLimitExceeded);
    assert!(summary.tick.raw() < 50);
}
```

- [ ] **Step 2: Write waste accumulation collapse test**

Append to `tests/phase1_sustained_viability.rs`:

```rust
#[test]
fn sustained_metabolism_collapses_when_waste_has_no_sufficient_sink() {
    let mut config = viability_config();
    config.world.tick_count = Tick::from_raw(50);
    config.resources.initial_distribution = vec![ResourceAmount::new(100.0).unwrap()];
    config.resource_interaction.heat_per_resource = 0.0;
    config.resource_interaction.waste_per_resource = 1.0;
    config.environment.waste_sink_rate = WasteAmount::zero();
    config.environment.waste_warning_threshold = WasteAmount::new(5.0).unwrap();
    config.environment.waste_death_threshold = WasteAmount::new(8.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::WasteLimitExceeded);
    assert!(summary.tick.raw() < 50);
}
```

- [ ] **Step 3: Run heat/waste gate tests**

Run:

```bash
cargo test --test phase1_sustained_viability sustained_metabolism_collapses_when_heat_has_no_sufficient_sink
cargo test --test phase1_sustained_viability sustained_metabolism_collapses_when_waste_has_no_sufficient_sink
```

Expected:

```text
both long-run heat/waste gate tests pass
```

---

## Task 8: Preserve Existing Phase 1A/1B/1C Behavior

**Files:**

- Modify only if verification exposes a regression:
  - `src/core/tick.rs`
  - `src/core/summary.rs`
  - `tests/phase1_config_validation.rs`
  - `tests/phase1_resource_interaction.rs`

- [ ] **Step 1: Run old scenario validation**

Run:

```bash
cargo test --test phase1_config_validation
```

Expected:

```text
all existing config validation tests pass
```

- [ ] **Step 2: Run resource interaction tests**

Run:

```bash
cargo test --test phase1_resource_interaction
```

Expected:

```text
all 1C resource interaction tests pass
```

- [ ] **Step 3: Run scenario dump**

Run:

```bash
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
```

Expected output still includes:

```text
single_cell_survival,Stable,None,100
single_cell_starvation,Collapse,MandatoryCostUnpaid,1
single_cell_dormancy,Collapse,EnergyDepleted,2
single_cell_heat_death,Collapse,HeatLimitExceeded,3
single_cell_waste_death,Collapse,WasteLimitExceeded,3
single_cell_over_capacity,Collapse,CapacityExceeded,1
```

If any old scenario changes, do not adjust expected output until the cause is identified.

---

## Task 9: Full Verification

**Files:**

- No source changes expected.

- [ ] **Step 1: Run formatter**

Run:

```bash
cargo fmt --check
```

Expected:

```text
passes
```

If it fails, run:

```bash
cargo fmt
cargo fmt --check
```

- [ ] **Step 2: Run linter**

Run:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Expected:

```text
passes with no warnings
```

- [ ] **Step 3: Run all Rust tests**

Run:

```bash
cargo test
```

Expected:

```text
all Rust tests pass
```

- [ ] **Step 4: Run Python tests only if tool/scenario files changed**

Run only if `tools/early-stability/*` or scenario TOML files were changed:

```bash
python -m pytest .\tools\early-stability
```

If not run, write in the report:

```text
Python tool code and scenario files were not changed; Rust Phase 1D behavior was verified with Cargo tests.
```

---

## Task 10: Write Implementation Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1D-sustained-viability-gates.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Create report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1D-sustained-viability-gates.md`:

```markdown
# REPORT: Phase 1D Sustained Viability Gates

## Goal

Closed Phase 1 with deterministic sustained-viability gates and observer-only run metrics.

## Scope

- Added run-level viability metrics.
- Added min/max Energy tracking across configured runs.
- Added final internal/external Resource metrics.
- Added final capacity metrics.
- Added observer-only growth_readiness metric.
- Added 1_000 tick sustained viability test.
- Added resource exhaustion collapse test.
- Added long-run Heat/Waste collapse tests.
- Preserved Phase 1A/1B/1C behavior.

## Decisions

- Cell radius remains fixed.
- No active growth or division was implemented.
- `growth_readiness` is observer-only and cannot affect behavior.
- Internal Resources remain aggregate in Phase 1D.
- Phase 2 will own Process Registry, Feasibility and real growth/division mechanics.

## Scenario Results

Paste output from:

```text
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
```

## New Sustained Viability Tests

List tests from:

```text
cargo test --test phase1_sustained_viability
```

## Verification

```text
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test
```

If Python was run:

```text
python -m pytest .\tools\early-stability
```

If Python was not run:

```text
Python tool code and scenario files were not changed; Python tests were not required for this Rust-only Phase 1D change.
```

## Open Questions

- Real growth remains Phase 2+.
- Division remains Phase 2+.
- Process Registry and Feasibility remain Phase 2+.
- Multi-resource internal inventories remain future work.
```

- [ ] **Step 2: Add report to worklog index**

Add under `## Reports` in `outputs/worklogs/index.md`:

```markdown
- [[outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1D-sustained-viability-gates|outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1D-sustained-viability-gates]]
```

Use the actual report timestamp.

---

## Acceptance Gates

Phase 1D is complete only when:

```text
Phase 1C verification remains green.
RunSummary exposes min/max Energy across configured runs.
RunSummary exposes final internal/external Resource metrics.
RunSummary exposes final used/free capacity metrics.
growth_readiness is observer-only and does not affect behavior.
One Cell can remain Stable for 1_000 ticks on a configured local resource loop.
One Cell collapses predictably when local resources are exhausted.
Heat accumulation can collapse the run when sink is insufficient.
Waste accumulation can collapse the run when sink is insufficient.
Existing Phase 1A/1B/1C scenario outcomes are preserved.
No active growth, division, Genome, Process Registry, Feasibility, Joints, signals or diffusion is added.
cargo fmt --check passes.
cargo clippy --workspace --all-targets --all-features -- -D warnings passes.
cargo test passes.
Implementation report exists and is linked from outputs/worklogs/index.md.
```

---

## Self-Review

Spec coverage:

- Closes Phase 1 with long-run viability instead of one-tick smoke only.
- Keeps Cell radius fixed and non-behavioral for uptake footprint.
- Avoids implementing division before Process Registry and Feasibility.
- Provides metrics needed to compare Rust behavior with future stability tooling.
- Preserves existing Phase 1 scenario expectations.

Known limits:

- `growth_readiness` is a rough observer metric, not a biological mechanism.
- No growth progress, mass increase or division occurs in Phase 1D.
- Internal Cell Resources remain one aggregate amount.
- No resource diffusion or resource-type-specific metabolism is added.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
