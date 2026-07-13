# Phase 2I Integrated World Calibration Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Use `test-driven-development` for every behavior change: write the failing test, run it and verify the expected failure, then implement minimal code.

**Goal:** Add a short integrated-world calibration gate after Phase 2H and before Phase 3, proving that one reproducible baseline world can run long enough with clean accounting, bounded population, active matter dynamics, repair, fragments and Joints.

**Architecture:** Keep Core as the simulation authority and treat calibration as deterministic runtime configuration plus observer/analyzer validation. Add accounting and warning helpers as read-only projections over committed state and existing summaries; do not add Genome, organism control, fitness or ecosystem balancing. Keep hot state World-owned and typed; analyzer logic may aggregate but must not influence Tick execution.

**Tech Stack:** Rust core (`src/core`), runner/parser TOML config (`src/runner/config_parser.rs`, `config/scenarios/world/*.toml`), analyzer binary (`src/bin/sweep_analyzer.rs`), observer projection (`src/observer`), Rust integration tests (`tests/phase2i_*.rs`), worklogs (`outputs/worklogs`).

---

## Source Constraints

Must preserve:

- `docs/PRINCIPLES.md`: no hardcoded species, organs, predator/prey behavior, observer authority, fitness input or scripted ecosystem roles.
- `docs/mechanics/matter-accounting.md`: every Resource, Material, Fragment, Joint material, reaction input/output, repair cost, Joint cost, decay and sink must be explicit.
- `docs/mechanics/tick-transaction.md`: no same-phase feedback and no observer/analyzer feedback into behavior.
- `docs/mechanics/action-feasibility.md`: rejected actions have explicit reasons, no cost and no output.
- `docs/mechanics/joint-interaction.md`: Joint transfers Resources, Signal and Heat only; no Energy Buffer transfer.
- `docs/mechanics/config-to-runtime.md`: new scenario files must parse into validated runtime config and produce stable config hashes.
- `docs/mechanics/observer-projection.md`: analyzer warnings and integrated gates are projections only.

## Scope Boundaries

Implement:

- one integrated baseline profile, not evolutionary tuning;
- accounting normalization and analyzer diagnostics needed to trust the baseline;
- scenario-specific warning logic for Phase 2G/2H flat sweeps;
- no-op candidate suppression where a candidate has no inputs, no gradient or no damage target;
- three reproducible world scenario configs;
- a 10k+ tick integrated-world gate.

Do not implement:

- Genome Runtime;
- mutation, inheritance tuning or selection;
- organism controller;
- full ecosystem balancing;
- stochastic calibration search;
- UI or viewer work.

## File Structure

Create:

- `src/core/accounting.rs` - committed-state integrated matter accounting snapshot and tolerance checks.
- `tests/phase2i_accounting.rs` - global matter accounting unit/integration tests.
- `tests/phase2i_noop_candidates.rs` - no-op process candidate suppression tests.
- `tests/phase2i_analyzer_warnings.rs` - scenario-specific warning tests.
- `tests/phase2i_joint_transfer_audit.rs` - gross/net/endpoints Joint Resource transfer tests.
- `tests/phase2i_integrated_world.rs` - long-run deterministic integrated-world gates.
- `tests/phase2i_world_configs.rs` - parser/config fixture tests.
- `config/scenarios/world/world_baseline_stable.toml` - 10k+ tick reproducible integrated baseline.
- `config/scenarios/world/world_mechanism_showcase.toml` - short run that activates all mechanisms.
- `config/scenarios/world/world_stress_regression.toml` - deterministic failure/boundary regression scenario.

Modify:

- `src/core/mod.rs` - expose `accounting`.
- `src/core/summary.rs` - add integrated accounting and Joint transfer audit fields.
- `src/core/tick.rs` - collect accounting/audit metrics and suppress no-op candidates.
- `src/core/process.rs` - add missing rejection reasons for repair/reactions/Joint creation where needed.
- `src/bin/sweep_analyzer.rs` - scenario-specific warnings, new raw columns, integrated-world config path and analyzer gate.
- `src/runner/config_parser.rs` - parse the three new world scenario fixtures if current generic parser cannot load them.
- `config/analyzer/sweep_analyzer.toml` - recalibrate only listed flat sweeps.
- `config/analyzer/sweep_analyzer_smoke.toml` - keep fast smoke coverage for the new integrated world gate.
- `outputs/worklogs/index.md` - add final report link after implementation.

---

## Task 1: Integrated Accounting Snapshot

**Files:**

- Create: `src/core/accounting.rs`
- Modify: `src/core/mod.rs`
- Modify: `src/core/summary.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/phase2i_accounting.rs`

- [ ] **Step 1: Write the failing accounting tests**

Create `tests/phase2i_accounting.rs`:

```rust
use alife::core::accounting::{IntegratedAccountingSnapshot, MatterAccountingDelta};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn accounting_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(8.0, 8.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(50.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };

    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(21),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(20.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(100.0).unwrap(),
            heat_death_threshold: HeatAmount::new(200.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(100.0).unwrap(),
            waste_death_threshold: WasteAmount::new(200.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn integrated_accounting_snapshot_includes_resources_materials_fragments_and_joints() {
    let executor = TickExecutor::new(accounting_config()).unwrap();

    let snapshot = IntegratedAccountingSnapshot::from_world(executor.world());

    assert!(snapshot.world_resources >= 20.0);
    assert_eq!(snapshot.cell_internal_resources, 4.0);
    assert_eq!(snapshot.cell_materials, 7.0);
    assert_eq!(snapshot.fragment_materials, 0.0);
    assert_eq!(snapshot.joint_materials, 0.0);
    assert!(snapshot.total_matter() >= 31.0);
}

#[test]
fn accounting_delta_accepts_explicit_decay_sink_but_rejects_unclassified_loss() {
    let before = IntegratedAccountingSnapshot {
        world_resources: 10.0,
        cell_internal_resources: 4.0,
        cell_materials: 7.0,
        fragment_materials: 0.0,
        joint_materials: 0.0,
        explicit_sinks: 0.0,
    };
    let after = IntegratedAccountingSnapshot {
        world_resources: 8.0,
        cell_internal_resources: 4.0,
        cell_materials: 7.0,
        fragment_materials: 0.0,
        joint_materials: 0.0,
        explicit_sinks: 2.0,
    };

    let delta = MatterAccountingDelta::between(before, after);

    assert!(delta.is_clean(0.0001));
    assert_eq!(delta.unclassified_loss, 0.0);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_accounting
```

Expected: FAIL because `alife::core::accounting` and the snapshot types do not exist.

- [ ] **Step 3: Implement the accounting module minimally**

Create `src/core/accounting.rs`:

```rust
use crate::core::world::WorldState;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct IntegratedAccountingSnapshot {
    pub world_resources: f32,
    pub cell_internal_resources: f32,
    pub cell_materials: f32,
    pub fragment_materials: f32,
    pub joint_materials: f32,
    pub explicit_sinks: f32,
}

impl IntegratedAccountingSnapshot {
    pub fn from_world(world: &WorldState) -> Self {
        let cells = world.cells();
        let mut cell_internal_resources = 0.0;
        let mut cell_materials = 0.0;
        for raw in 0..cells.len() {
            let index = crate::core::cell_store::CellIndex::from_raw(raw);
            cell_internal_resources += cells.resource_amount(index).raw();
            cell_materials += cells.total_material_amount(index).raw();
        }

        let mut world_resources = 0.0;
        for layer in 0..world.resources().layer_count() {
            let layer = crate::core::resources::ResourceLayerIndex::from_raw(layer);
            world_resources += world
                .resources()
                .total_amount_for_layer(layer)
                .map(|amount| amount.raw())
                .unwrap_or(0.0);
        }

        let fragment_materials = world.fragments().total_material_amount().raw();
        let joint_materials = world
            .joints()
            .all_ids()
            .map(|id| world.joints().material_amount(id).map(|m| m.raw()).unwrap_or(0.0))
            .sum();

        Self {
            world_resources,
            cell_internal_resources,
            cell_materials,
            fragment_materials,
            joint_materials,
            explicit_sinks: 0.0,
        }
    }

    pub fn total_matter(self) -> f32 {
        self.world_resources
            + self.cell_internal_resources
            + self.cell_materials
            + self.fragment_materials
            + self.joint_materials
            + self.explicit_sinks
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct MatterAccountingDelta {
    pub before_total: f32,
    pub after_total: f32,
    pub unclassified_loss: f32,
    pub unclassified_gain: f32,
}

impl MatterAccountingDelta {
    pub fn between(
        before: IntegratedAccountingSnapshot,
        after: IntegratedAccountingSnapshot,
    ) -> Self {
        let before_total = before.total_matter();
        let after_total = after.total_matter();
        let diff = after_total - before_total;
        Self {
            before_total,
            after_total,
            unclassified_loss: (-diff).max(0.0),
            unclassified_gain: diff.max(0.0),
        }
    }

    pub fn is_clean(self, tolerance: f32) -> bool {
        self.unclassified_loss <= tolerance && self.unclassified_gain <= tolerance
    }
}
```

Modify `src/core/mod.rs`:

```rust
pub mod accounting;
```

If `WorldState::fragments()` or `MaterialFragmentStore::total_material_amount()` is missing, add the narrow read-only accessors:

```rust
pub fn fragments(&self) -> &MaterialFragmentStore {
    &self.fragments
}

pub fn total_material_amount(&self) -> MaterialAmount {
    MaterialAmount::new_unchecked(self.amounts.iter().map(|amount| amount.raw()).sum())
}
```

- [ ] **Step 4: Add summary fields**

Modify `src/core/summary.rs`:

```rust
pub integrated_matter_before: f32,
pub integrated_matter_after: f32,
pub integrated_matter_unclassified_loss: f32,
pub integrated_matter_unclassified_gain: f32,
```

Modify the `MetricsSummary` construction in `src/core/tick.rs` to populate these fields from `IntegratedAccountingSnapshot` before and after Tick execution.

- [ ] **Step 5: Run GREEN**

Run:

```bash
cargo test --test phase2i_accounting
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/accounting.rs src/core/mod.rs src/core/summary.rs src/core/tick.rs tests/phase2i_accounting.rs
git commit -m "feat: add integrated matter accounting snapshot"
```

---

## Task 2: Suppress Repeated No-Op Candidates

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `src/core/process.rs`
- Test: `tests/phase2i_noop_candidates.rs`

- [ ] **Step 1: Write failing no-op candidate tests**

Create `tests/phase2i_noop_candidates.rs`:

```rust
use alife::core::cell_store::CellIndex;
use alife::core::config::*;
use alife::core::materials::MaterialSlot;
use alife::core::process::ProcessId;
use alife::core::tick::TickExecutor;
use alife::core::units::*;

fn base_cell(position: Position) -> CellInitialConfig {
    CellInitialConfig {
        position,
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(30.0).unwrap(),
        energy_capacity: EnergyAmount::new(60.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(10.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn two_cell_config() -> RuntimeConfig {
    let cell = base_cell(Position::new(8.0, 8.0));
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(31),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::new(50.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(100.0).unwrap(),
            heat_death_threshold: HeatAmount::new(200.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(100.0).unwrap(),
            waste_death_threshold: WasteAmount::new(200.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap();
    config.initial_cells = vec![
        base_cell(Position::new(8.0, 8.0)),
        base_cell(Position::new(9.9, 8.0)),
    ];
    config.local_interaction.enabled = true;
    config.joints.enabled = true;
    config.joints.creation_material_cost = MaterialAmount::new(0.5).unwrap();
    config
}

#[test]
fn joint_create_is_not_attempted_again_after_active_joint_exists() {
    let mut executor = TickExecutor::new(two_cell_config()).unwrap();

    let first = executor.step().unwrap();
    let second = executor.step().unwrap();

    assert_eq!(first.metrics.joint_created_count, 1);
    assert_eq!(second.metrics.joint_created_count, 0);
    assert_eq!(
        second
            .diagnostics
            .attempts_by_process
            .get(&ProcessId::JointCreate)
            .copied()
            .unwrap_or(0),
        0
    );
}

#[test]
fn repair_candidate_is_not_emitted_without_boundary_damage() {
    let mut config = two_cell_config();
    config.chemistry.repair.enabled = true;
    config.chemistry.repair.energy_cost = 0.1;
    config.chemistry.repair.max_amount_per_tick = 0.25;
    let mut executor = TickExecutor::new(config).unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(
        executor
            .world()
            .cells()
            .material_damage(CellIndex::from_raw(0), MaterialSlot::Boundary),
        0.0
    );
    assert_eq!(
        summary
            .diagnostics
            .attempts_by_process
            .get(&ProcessId::RepairBoundary)
            .copied()
            .unwrap_or(0),
        0
    );
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_noop_candidates
```

Expected: FAIL because current tick execution still records no-op attempts or does not expose expected process attempts.

- [ ] **Step 3: Implement no-op guards**

In `src/core/tick.rs`, before calling `run_process` for repair:

```rust
let boundary_damage = self
    .world
    .cells()
    .material_damage(index, MaterialSlot::Boundary);
if boundary_damage <= 0.0 {
    continue;
}
```

Before creating a Joint candidate:

```rust
let Some(endpoints) = crate::core::joints::JointEndpoints::new(pair.a, pair.b) else {
    continue;
};
if self.world.joints().has_active_between(endpoints) {
    continue;
}
```

Before passive or controlled reaction candidate creation, skip candidate construction when all required local inputs are zero:

```rust
if !reaction_has_available_inputs(&self.world, index, reaction, &config) {
    continue;
}
```

Add a private helper in `src/core/tick.rs`:

```rust
fn reaction_has_available_inputs(
    world: &crate::core::world::WorldState,
    cell_idx: CellIndex,
    reaction: &crate::core::reactions::ReactionDefinition,
    config: &RuntimeConfig,
) -> bool {
    reaction.inputs().iter().all(|input| {
        crate::core::world::resource_type_id(config, input.resource_id())
            .and_then(|resource_type| {
                world
                    .cells()
                    .typed_resource_amount(cell_idx, resource_type)
                    .ok()
            })
            .map(|amount| amount.raw() >= input.amount())
            .unwrap_or(false)
    })
}
```

If existing reaction definitions use a different API, implement the same behavior at the current reaction matching site with existing input tuples.

- [ ] **Step 4: Add rejection reasons only for real rejected candidates**

In `src/core/process.rs`, add missing reasons if absent:

```rust
NoTargetDamage,
MissingReactionInput,
MissingGradient,
JointAlreadyExists,
```

Only record these when a candidate is actually created and rejected. Do not count skipped no-op candidates as rejections.

- [ ] **Step 5: Run GREEN**

Run:

```bash
cargo test --test phase2i_noop_candidates
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/core/tick.rs src/core/process.rs tests/phase2i_noop_candidates.rs
git commit -m "fix: suppress no-op process candidates"
```

---

## Task 3: Scenario-Specific Analyzer Warnings

**Files:**

- Modify: `src/bin/sweep_analyzer.rs`
- Modify: `tests/phase2_sweep_warnings.rs`
- Test: `tests/phase2i_analyzer_warnings.rs`

- [ ] **Step 1: Write failing warning tests**

Create `tests/phase2i_analyzer_warnings.rs`:

```rust
use alife::bin::sweep_analyzer::{detect_warnings, SimResult};

fn result() -> SimResult {
    SimResult {
        collapsed: false,
        collapse_tick: None,
        dormant_ticks: 0,
        active_ticks: 100,
        stressed_ticks: 0,
        min_energy: 10.0,
        max_energy: 10.0,
        final_energy: 10.0,
        mean_energy: 10.0,
        initial_energy: 10.0,
        death_reason: "none".to_string(),
        energy_produced: 0.0,
        passive_energy_received: 0.0,
        energy_spent_upkeep: 0.0,
        energy_spent_dormant_upkeep: 0.0,
        energy_spent_movement: 0.0,
        energy_spent_growth: 0.0,
        energy_spent_repair: 0.0,
        energy_spent_division: 0.0,
        initial_world_resource: 0.0,
        final_world_resource: 0.0,
        resource_regenerated: 0.0,
        resource_absorbed: 0.0,
        resource_metabolized: 0.0,
        internal_resource_final: 0.0,
        resource_released: 0.0,
        resource_explicit_sink: 0.0,
        resource_balance_error: 0.0,
        energy_balance_error: 0.0,
        dormancy_enter_count: 0,
        dormancy_exit_count: 0,
        ticks_executed: 100,
        total_resource_consumed: 0.0,
        metabolism_count: 0,
        potential_role: "unknown".to_string(),
        observed_role: "unknown".to_string(),
        behavior_profile: "unknown".to_string(),
        ticks_per_second: 100.0,
        bhv_res: None,
        explicit_energy_loss: 0.0,
        death_cleanup_loss_energy: 0.0,
        death_cleanup_loss_resources: 0.0,
        clamping_loss: 0.0,
        unpaid_mandatory_cost: 0.0,
        resource_decay: 0.0,
        resource_sink: 0.0,
        numerical_error_energy: 0.0,
        numerical_error_resources: 0.0,
        unclassified_loss_energy: 0.0,
        unclassified_loss_resources: 0.0,
        divisions_count: 0,
        births_count: 0,
        dead_cells_count: 0,
        decomposing_cells_count: 0,
        decomposed_cells_count: 0,
        division_attempts: 0,
        division_successes: 0,
        division_rejections: 0,
        decomposition_released_resources: 0.0,
        death_tick: 0,
        first_decomposition_tick: 0,
        first_decomposed_tick: 0,
        decomposition_ticks: 0,
        decomposition_released_resources_per_tick: 0.0,
        time_to_decomposed: 0,
        remaining_dead_cell_resources: 0.0,
        remaining_dead_cell_materials: 0.0,
        contact_pairs_count: 0,
        contact_pressure_pre_total: 0.0,
        contact_pressure_post_total: 0.0,
        contact_pressure_max_over_tick: 0.0,
        contact_exchange_amount: 0.0,
        contact_exchange_pairs_count: 0,
        contact_exchange_rejections_no_capability: 0,
        contact_stimulus_generated_total: 0.0,
        contact_stimulus_readable_total: 0.0,
        reaction_matched_count: 0,
        reaction_executed_count: 0,
        reaction_rejected_count: 0,
        reaction_input_amount: 0.0,
        reaction_output_amount: 0.0,
        reaction_heat_generated: 0.0,
        reaction_energy_output: 0.0,
        reaction_accounting_error: 0.0,
        resource_diffused_amount: 0.0,
        resource_decay_amount: 0.0,
        fragment_created_amount: 0.0,
        fragment_converted_amount: 0.0,
        heat_peak_temperature: 0.0,
        material_degradation_amount: 0.0,
        boundary_leakage_amount: 0.0,
        repair_success_count: 0,
        repair_rejection_count: 0,
        joint_count: 0,
        joint_created_count: 0,
        joint_creation_rejected_count: 0,
        joint_broken_count: 0,
        joint_resource_transfer_amount: 0.0,
        joint_signal_generated_total: 0.0,
        joint_signal_readable_total: 0.0,
        joint_heat_transfer_amount: 0.0,
        joint_degradation_amount: 0.0,
        joint_mechanical_correction_amount: 0.0,
    }
}

#[test]
fn diffusion_variation_prevents_false_low_information_warning() {
    let mut a = result();
    let mut b = result();
    b.resource_diffused_amount = 5.0;

    let warnings = detect_warnings(&[a, b], "resource_type_decay_diffusion");

    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
}

#[test]
fn repair_success_variation_is_recognized() {
    let mut a = result();
    let mut b = result();
    b.repair_success_count = 3;

    let warnings = detect_warnings(&[a, b], "repair_viability");

    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
    assert!(!warnings.contains(&"REPAIR_NOT_ACTIVATED".to_string()));
}

#[test]
fn joint_transfer_signal_and_break_variation_is_recognized() {
    let mut a = result();
    let mut b = result();
    b.joint_resource_transfer_amount = 1.0;
    b.joint_signal_generated_total = 0.5;
    b.joint_broken_count = 1;

    let warnings = detect_warnings(&[a, b], "joint_degradation_break");

    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
    assert!(!warnings.contains(&"JOINT_NOT_ACTIVATED".to_string()));
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_analyzer_warnings
```

Expected: FAIL because `detect_warnings` still treats several active mechanism sweeps as low-information.

- [ ] **Step 3: Add warning helper predicates**

In `src/bin/sweep_analyzer.rs`, add private helpers near `varies_meaningfully`:

```rust
fn any_u32(results: &[SimResult], f: impl Fn(&SimResult) -> u32) -> bool {
    results.iter().any(|result| f(result) > 0)
}

fn varies_u32(results: &[SimResult], f: impl Fn(&SimResult) -> u32) -> bool {
    if results.is_empty() {
        return false;
    }
    let min = results.iter().map(|result| f(result)).min().unwrap_or(0);
    let max = results.iter().map(|result| f(result)).max().unwrap_or(0);
    max > min
}

fn varies_f32(results: &[SimResult], f: impl Fn(&SimResult) -> f32, min_delta: f32) -> bool {
    let values: Vec<f32> = results.iter().map(f).collect();
    varies_meaningfully(&values, min_delta)
}
```

- [ ] **Step 4: Add scenario-specific warning logic**

Inside `detect_warnings`, before final `LOW_INFORMATION_SWEEP` push:

```rust
match scenario_lower.as_str() {
    "resource_type_decay_diffusion" => {
        if varies_f32(results, |r| r.resource_diffused_amount, 0.001)
            || varies_f32(results, |r| r.resource_decay_amount, 0.001)
        {
            push_low_info = false;
        }
    }
    "repair_viability" => {
        if any_u32(results, |r| r.repair_success_count)
            && varies_u32(results, |r| r.repair_success_count)
        {
            push_low_info = false;
        } else {
            warnings.push("REPAIR_NOT_ACTIVATED".to_string());
        }
    }
    "local_heat_degradation" | "material_type_degradation" => {
        if varies_f32(results, |r| r.heat_peak_temperature, 0.001)
            || varies_f32(results, |r| r.material_degradation_amount, 0.001)
        {
            push_low_info = false;
        }
    }
    "joint_resource_channel" => {
        if varies_f32(results, |r| r.joint_resource_transfer_amount, 0.001) {
            push_low_info = false;
        } else {
            warnings.push("JOINT_RESOURCE_TRANSFER_FLAT".to_string());
        }
    }
    "joint_signal_delay" => {
        if varies_f32(results, |r| r.joint_signal_generated_total, 0.001)
            || varies_f32(results, |r| r.joint_signal_readable_total, 0.001)
        {
            push_low_info = false;
        } else {
            warnings.push("JOINT_SIGNAL_FLAT".to_string());
        }
    }
    "joint_degradation_break" => {
        if any_u32(results, |r| r.joint_broken_count)
            || varies_f32(results, |r| r.joint_degradation_amount, 0.001)
        {
            push_low_info = false;
        } else {
            warnings.push("JOINT_BREAK_NOT_ACTIVATED".to_string());
        }
    }
    _ => {}
}
```

- [ ] **Step 5: Run GREEN and old warning tests**

Run:

```bash
cargo test --test phase2i_analyzer_warnings
cargo test --test phase2_sweep_warnings
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/bin/sweep_analyzer.rs tests/phase2i_analyzer_warnings.rs tests/phase2_sweep_warnings.rs
git commit -m "fix: add scenario-aware analyzer warnings"
```

---

## Task 4: Joint Resource Transfer Audit

**Files:**

- Modify: `src/core/summary.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/observer/projection.rs`
- Modify: `src/bin/sweep_analyzer.rs`
- Test: `tests/phase2i_joint_transfer_audit.rs`

- [ ] **Step 1: Write failing Joint transfer audit tests**

Create `tests/phase2i_joint_transfer_audit.rs`:

```rust
use alife::core::cell_store::CellIndex;
use alife::core::config::*;
use alife::core::tick::TickExecutor;
use alife::core::units::*;

fn config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(10.0, 10.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(40.0).unwrap(),
        energy_capacity: EnergyAmount::new(80.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(10.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::new(10.0).unwrap(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    };
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(3),
            seed: Seed::from_raw(41),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        cell,
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::zero(),
            heat_dissipation_rate: HeatAmount::zero(),
            heat_warning_threshold: HeatAmount::new(100.0).unwrap(),
            heat_death_threshold: HeatAmount::new(200.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::zero(),
            waste_sink_rate: WasteAmount::zero(),
            waste_warning_threshold: WasteAmount::new(100.0).unwrap(),
            waste_death_threshold: WasteAmount::new(200.0).unwrap(),
        },
        LifecycleConfig {
            stress_energy_threshold: EnergyAmount::new(1.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(100.0).unwrap(),
        },
    )
    .unwrap();
    let mut target = cell;
    target.position = Position::new(11.9, 10.0);
    target.initial_resource_amount = ResourceAmount::zero();
    config.initial_cells = vec![cell, target];
    config.local_interaction.enabled = true;
    config.joints.enabled = true;
    config.joints.creation_material_cost = MaterialAmount::new(0.5).unwrap();
    config.joints.resource_transfer_rate = 1.0;
    config.joints.max_resource_transfer_per_tick = ResourceAmount::new(2.0).unwrap();
    config
}

#[test]
fn joint_transfer_audit_reports_gross_net_and_endpoint_finals_without_backflow_inflation() {
    let mut executor = TickExecutor::new(config()).unwrap();
    executor.step().unwrap();

    let before_source = executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let before_target = executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();

    let summary = executor.step().unwrap();

    let after_source = executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(0))
        .raw();
    let after_target = executor
        .world()
        .cells()
        .resource_amount(CellIndex::from_raw(1))
        .raw();

    assert!(summary.metrics.joint_resource_transfer_gross_amount > 0.0);
    assert_eq!(
        summary.metrics.joint_resource_transfer_net_amount,
        (after_target - before_target).abs().max((after_source - before_source).abs())
    );
    assert_eq!(summary.metrics.joint_resource_source_final_amount, after_source);
    assert_eq!(summary.metrics.joint_resource_target_final_amount, after_target);
    assert!(summary.metrics.joint_resource_backflow_amount <= 0.0001);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_joint_transfer_audit
```

Expected: FAIL because audit fields do not exist.

- [ ] **Step 3: Add audit fields**

Modify `src/core/summary.rs`:

```rust
pub joint_resource_transfer_gross_amount: f32,
pub joint_resource_transfer_net_amount: f32,
pub joint_resource_source_final_amount: f32,
pub joint_resource_target_final_amount: f32,
pub joint_resource_backflow_amount: f32,
```

Modify `src/observer/projection.rs` to expose the same fields through `metrics_summary_features`.

- [ ] **Step 4: Populate audit metrics during Joint transfer**

In the Joint Resource channel block in `src/core/tick.rs`, record:

```rust
let source_before = self.world.cells().resource_amount(source).raw();
let target_before = self.world.cells().resource_amount(target).raw();
let moved = {
    let cells = self.world.cells_mut_for_commit();
    cells.transfer_resources_limited_by_effective_capacity(
        source,
        target,
        ResourceAmount::new(requested).expect("joint resource transfer is clamped"),
        config.material_effects.storage_capacity_per_unit,
    )
};
let source_after = self.world.cells().resource_amount(source).raw();
let target_after = self.world.cells().resource_amount(target).raw();

phase2h_metrics.joint_resource_transfer_amount += moved.raw();
phase2h_metrics.joint_resource_transfer_gross_amount += moved.raw();
phase2h_metrics.joint_resource_transfer_net_amount +=
    (target_after - target_before).abs().max((source_after - source_before).abs());
phase2h_metrics.joint_resource_source_final_amount = source_after;
phase2h_metrics.joint_resource_target_final_amount = target_after;
if (source_after - source_before) > 0.0 || (target_before - target_after) > 0.0 {
    phase2h_metrics.joint_resource_backflow_amount +=
        (source_after - source_before).max(0.0) + (target_before - target_after).max(0.0);
}
```

Use the existing `Phase2HMetricsDelta` struct and pass the new values into `MetricsSummary`.

- [ ] **Step 5: Extend analyzer raw columns**

In `src/bin/sweep_analyzer.rs`, add these fields to `SimResult`, raw data map, sweep CSV header and row writers:

```rust
pub joint_resource_transfer_gross_amount: f32,
pub joint_resource_transfer_net_amount: f32,
pub joint_resource_source_final_amount: f32,
pub joint_resource_target_final_amount: f32,
pub joint_resource_backflow_amount: f32,
```

Aggregate gross as sum, net as sum, source/target finals as last observed values, backflow as sum.

- [ ] **Step 6: Run GREEN**

Run:

```bash
cargo test --test phase2i_joint_transfer_audit
cargo test --test phase2h_reachability
```

Expected: PASS and `joint_resource_channel.csv` contains the five new audit columns.

- [ ] **Step 7: Commit**

```bash
git add src/core/summary.rs src/core/tick.rs src/observer/projection.rs src/bin/sweep_analyzer.rs tests/phase2i_joint_transfer_audit.rs
git commit -m "feat: add joint resource transfer audit metrics"
```

---

## Task 5: Integrated World Scenario Configs

**Files:**

- Create: `config/scenarios/world/world_baseline_stable.toml`
- Create: `config/scenarios/world/world_mechanism_showcase.toml`
- Create: `config/scenarios/world/world_stress_regression.toml`
- Modify: `src/runner/config_parser.rs`
- Test: `tests/phase2i_world_configs.rs`

- [ ] **Step 1: Write failing world config tests**

Create `tests/phase2i_world_configs.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;

fn parse(path: &str) -> alife::core::config::RuntimeConfig {
    let text = std::fs::read_to_string(path).unwrap();
    RawScenarioConfig::parse(&text).unwrap().into_runtime_config().unwrap()
}

#[test]
fn integrated_world_configs_parse_and_have_distinct_hashes() {
    let baseline = parse("config/scenarios/world/world_baseline_stable.toml");
    let showcase = parse("config/scenarios/world/world_mechanism_showcase.toml");
    let stress = parse("config/scenarios/world/world_stress_regression.toml");

    assert_ne!(baseline.config_hash(), showcase.config_hash());
    assert_ne!(baseline.config_hash(), stress.config_hash());
    assert_ne!(showcase.config_hash(), stress.config_hash());
}

#[test]
fn baseline_world_enables_required_phase2_mechanisms_without_genome() {
    let config = parse("config/scenarios/world/world_baseline_stable.toml");

    assert!(config.resource_interaction.enabled);
    assert!(config.growth_enabled);
    assert!(config.division.enabled);
    assert!(config.decomposition.enabled);
    assert!(config.local_interaction.enabled);
    assert!(config.joints.enabled);
    assert!(config.chemistry.repair.enabled);
    assert!(!config.chemistry.reactions.is_empty());
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_world_configs
```

Expected: FAIL because the three TOML files do not exist or parser does not support all fields.

- [ ] **Step 3: Add `world_baseline_stable.toml`**

Create `config/scenarios/world/world_baseline_stable.toml`:

```toml
scenario_id = "world_baseline_stable"
version = "1.0"

[world]
tick_count = 10000
seed = 42
size = [128.0, 128.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
initial_distribution = [1200.0, 0.0]
decay_rate = 0.0001

[cell]
position = [32.0, 32.0]
radius = 1.0
initial_energy = 30.0
energy_capacity = 80.0
mandatory_cost_per_tick = 0.03
passive_energy_income = 0.0
capacity_limit = 120.0
initial_resources = 4.0
initial_materials = { boundary = 2.0, transport = 2.0, metabolic = 2.0, storage = 4.0, synthesis = 2.0, structural = 4.0, repair = 2.0, contractile = 0.5, sensory = 0.5 }

[[cells]]
position = [32.0, 32.0]
radius = 1.0
initial_energy = 30.0
energy_capacity = 80.0
mandatory_cost_per_tick = 0.03
passive_energy_income = 0.0
capacity_limit = 120.0
initial_resources = 4.0
initial_materials = { boundary = 2.0, transport = 2.0, metabolic = 2.0, storage = 4.0, synthesis = 2.0, structural = 4.0, repair = 2.0, contractile = 0.5, sensory = 0.5 }

[[cells]]
position = [34.0, 32.0]
radius = 1.0
initial_energy = 28.0
energy_capacity = 80.0
mandatory_cost_per_tick = 0.03
passive_energy_income = 0.0
capacity_limit = 120.0
initial_resources = 2.0
initial_materials = { boundary = 2.0, transport = 2.0, metabolic = 2.0, storage = 4.0, synthesis = 2.0, structural = 4.0, repair = 2.0, contractile = 0.5, sensory = 0.5 }

[resource_interaction]
enabled = true
resource_layer_index = 0
max_uptake_per_tick = 0.25
metabolism_resource_per_tick = 0.12
energy_per_resource = 0.65
heat_per_resource = 0.02
waste_per_resource = 0.01

[growth]
enabled = true
structural_material_per_radius = 2.5
resource_per_radius = 1.0
energy_per_radius = 1.0
max_radius_increase_per_tick = 0.002
target_radius = 1.8

[division]
enabled = true
radius_threshold = 1.75
energy_cost = 8.0
split_ratio = 0.5
partition_loss_fraction = 0.02
daughter_spacing = 2.2
min_daughter_radius = 0.7
max_division_pressure = 0.4

[decomposition]
enabled = true
resource_layer_index = 0
resources_per_tick = 0.1
materials_per_tick = 0.05

[local_interaction]
enabled = true
contact_exchange_rate = 0.08
max_exchange_per_pair = 0.1
min_boundary_capability = 0.1
min_transport_capability = 0.1
contact_stimulus_per_overlap = 0.05
stimulus_decay_per_tick = 0.05

[joints]
enabled = true
creation_material_cost = 0.25
creation_resource_cost = 0.05
creation_energy_cost = 0.05
mechanical_strength = 0.2
resource_transfer_rate = 0.08
signal_conductivity = 0.2
heat_conductivity = 0.1
decay_rate = 0.001
break_damage_threshold = 2.0
max_joints_per_cell = 4

[[chemistry.resources]]
id = "nutrient_A"
volume = 1.0
diffusion_rate = 0.05
energy_value = 0.65
decay_rate = 0.0001
reactivity_profile = "reactive"
permeability = "passive"
tags = ["energy_source"]

[[chemistry.resources]]
id = "waste_A"
volume = 1.0
diffusion_rate = 0.02
energy_value = 0.0
decay_rate = 0.001
reactivity_profile = "stable"
permeability = "blocked"
tags = ["waste"]

[[chemistry.materials]]
id = "boundary_polymer_A"
volume = 1.0
stability = 0.9
strength = 0.8
permeability = 0.1
energy_capacity = 0.0
decay_rate = 0.0002
repair_resource = "nutrient_A"
repair_amount = 0.2

[[chemistry.materials]]
id = "structural_polymer_A"
volume = 1.0
stability = 0.7
strength = 0.9
permeability = 0.0
energy_capacity = 0.0
decay_rate = 0.0004
repair_resource = "nutrient_A"
repair_amount = 0.3

[[chemistry.reactions]]
id = "passive_decay"
mode = "passive"
inputs = [{ resource = "nutrient_A", amount = 0.05 }]
outputs = [{ resource = "waste_A", amount = 0.05 }]
configured_sink_amount = 0.0
energy_output = 0.0
heat_output = 0.005
rate = 0.02
probability = 1.0
accounting_destination = "waste_A"

[chemistry.repair]
enabled = true
energy_cost = 0.05
max_amount_per_tick = 0.05
```

- [ ] **Step 4: Add showcase and stress configs**

Create `config/scenarios/world/world_mechanism_showcase.toml` by copying `world_baseline_stable.toml` and changing:

```toml
scenario_id = "world_mechanism_showcase"

[world]
tick_count = 500
seed = 43

[growth]
max_radius_increase_per_tick = 0.01
target_radius = 1.5

[division]
radius_threshold = 1.45
energy_cost = 4.0

[joints]
creation_material_cost = 0.1
resource_transfer_rate = 0.25
signal_conductivity = 0.5
heat_conductivity = 0.25
decay_rate = 0.005
```

Create `config/scenarios/world/world_stress_regression.toml` by copying `world_baseline_stable.toml` and changing:

```toml
scenario_id = "world_stress_regression"

[world]
tick_count = 1000
seed = 44

[resources]
initial_distribution = [30.0, 0.0]

[cell]
initial_energy = 8.0
mandatory_cost_per_tick = 0.12

[environment]
heat_warning_threshold = 8.0
heat_death_threshold = 12.0
```

- [ ] **Step 5: Extend parser only if tests require it**

If `RawScenarioConfig` cannot parse one of these blocks, add the missing raw struct fields in `src/runner/config_parser.rs` using existing patterns. Keep names identical to the TOML keys above.

- [ ] **Step 6: Run GREEN**

Run:

```bash
cargo test --test phase2i_world_configs
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add config/scenarios/world src/runner/config_parser.rs tests/phase2i_world_configs.rs
git commit -m "test: add integrated world scenario configs"
```

---

## Task 6: Flat Sweep Recalibration

**Files:**

- Modify: `config/analyzer/sweep_analyzer.toml`
- Modify: `config/analyzer/sweep_analyzer_smoke.toml`
- Modify: `tests/phase2g_sweep_parser.rs`
- Modify: `tests/phase2h_sweep_parser.rs`
- Test: `tests/phase2i_analyzer_warnings.rs`

- [ ] **Step 1: Write failing recalibration assertions**

Append to `tests/phase2i_analyzer_warnings.rs`:

```rust
#[test]
fn flat_sweeps_have_non_degenerate_calibration_ranges() {
    let full = std::fs::read_to_string("config/analyzer/sweep_analyzer.toml").unwrap();

    assert!(full.contains("name = \"material_type_degradation\""));
    assert!(full.contains("from = 0.0"));
    assert!(full.contains("to = 0.5"));

    assert!(full.contains("name = \"joint_creation_viability\""));
    assert!(full.contains("from = 0.05"));
    assert!(full.contains("to = 1.25"));

    assert!(full.contains("name = \"boundary_retention_leakage\""));
    assert!(full.contains("from = 0.0"));
    assert!(full.contains("to = 1.5"));
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_analyzer_warnings flat_sweeps_have_non_degenerate_calibration_ranges
```

Expected: FAIL until the configured ranges match the new calibration gate.

- [ ] **Step 3: Recalibrate only the specified flat sweeps**

In `config/analyzer/sweep_analyzer.toml`, change only:

```toml
[[sweep]]
name = "material_type_degradation"
scenario = "material_type_degradation"
param = "material_decay_rate"
from = 0.0
to = 0.5
steps = 10

[[sweep]]
name = "boundary_retention_leakage"
scenario = "boundary_retention_leakage"
param = "contact_exchange_rate"
from = 0.0
to = 1.5
steps = 10

[[sweep]]
name = "joint_creation_viability"
scenario = "joint_creation_viability"
param = "joint_mechanical_strength"
from = 0.05
to = 1.25
steps = 10
```

In `config/analyzer/sweep_analyzer_smoke.toml`, keep fast ranges:

```toml
steps = 3
```

Use the same `from` and `to` values for the three named sweeps.

- [ ] **Step 4: Run GREEN**

Run:

```bash
cargo test --test phase2i_analyzer_warnings flat_sweeps_have_non_degenerate_calibration_ranges
cargo test --test phase2g_sweep_parser
cargo test --test phase2h_sweep_parser
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add config/analyzer/sweep_analyzer.toml config/analyzer/sweep_analyzer_smoke.toml tests/phase2i_analyzer_warnings.rs tests/phase2g_sweep_parser.rs tests/phase2h_sweep_parser.rs
git commit -m "chore: recalibrate selected flat sweeps"
```

---

## Task 7: Integrated World Long-Run Gate

**Files:**

- Modify: `src/bin/sweep_analyzer.rs`
- Test: `tests/phase2i_integrated_world.rs`

- [ ] **Step 1: Write failing integrated gate tests**

Create `tests/phase2i_integrated_world.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;
use alife::core::tick::TickExecutor;

fn load(path: &str) -> alife::core::config::RuntimeConfig {
    let text = std::fs::read_to_string(path).unwrap();
    RawScenarioConfig::parse(&text).unwrap().into_runtime_config().unwrap()
}

fn run_for_ticks(path: &str, ticks: u32) -> alife::core::summary::RunSummary {
    let mut config = load(path);
    config.world.tick_count = alife::core::units::Tick::from_raw(ticks);
    let mut executor = TickExecutor::new(config).unwrap();
    let mut summary = executor.step().unwrap();
    for _ in 1..ticks {
        summary = executor.step().unwrap();
    }
    summary
}

#[test]
fn baseline_world_runs_10k_ticks_with_clean_accounting_and_bounded_population() {
    let summary = run_for_ticks("config/scenarios/world/world_baseline_stable.toml", 10_000);

    assert!(summary.metrics.alive_cells_count >= 1);
    assert!(summary.metrics.alive_cells_count <= 64);
    assert!(summary.metrics.integrated_matter_unclassified_loss <= 0.01);
    assert!(summary.metrics.integrated_matter_unclassified_gain <= 0.01);
    assert!(summary.metrics.resource_diffused_amount > 0.0);
    assert!(summary.metrics.repair_success_count > 0);
    assert!(summary.metrics.fragment_created_amount > 0.0);
    assert!(summary.metrics.fragment_converted_amount > 0.0);
    assert!(summary.metrics.joint_created_count > 0);
    assert!(summary.metrics.joint_broken_count > 0 || summary.metrics.joint_degradation_amount > 0.0);
}

#[test]
fn baseline_world_replay_is_deterministic() {
    let first = run_for_ticks("config/scenarios/world/world_baseline_stable.toml", 2_000);
    let second = run_for_ticks("config/scenarios/world/world_baseline_stable.toml", 2_000);

    assert_eq!(first.config_hash, second.config_hash);
    assert_eq!(first.metrics.alive_cells_count, second.metrics.alive_cells_count);
    assert_eq!(first.metrics.dead_cells_count, second.metrics.dead_cells_count);
    assert_eq!(first.metrics.divisions_count, second.metrics.divisions_count);
    assert_eq!(first.metrics.joint_created_count, second.metrics.joint_created_count);
    assert!((first.metrics.final_energy - second.metrics.final_energy).abs() < 0.0001);
}

#[test]
fn stress_world_fails_deterministically_without_accounting_leak() {
    let first = run_for_ticks("config/scenarios/world/world_stress_regression.toml", 1_000);
    let second = run_for_ticks("config/scenarios/world/world_stress_regression.toml", 1_000);

    assert_eq!(first.collapse_reason, second.collapse_reason);
    assert_eq!(first.metrics.dead_cells_count, second.metrics.dead_cells_count);
    assert!(first.metrics.integrated_matter_unclassified_loss <= 0.01);
    assert!(first.metrics.integrated_matter_unclassified_gain <= 0.01);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2i_integrated_world
```

Expected: FAIL until accounting, configs and baseline calibration satisfy the gate.

- [ ] **Step 3: Add analyzer integrated-world scenario support**

In `src/bin/sweep_analyzer.rs`, add allowed scenarios:

```rust
"world_baseline_stable",
"world_mechanism_showcase",
"world_stress_regression",
```

Add a smoke sweep:

```toml
[[sweep]]
name = "integrated_world_baseline"
scenario = "world_baseline_stable"
param = "mandatory_cost_per_tick"
from = 0.02
to = 0.04
steps = 3

[scenarios.world_baseline_stable]
scenario_path = "config/scenarios/world/world_baseline_stable.toml"
```

If `RawScenarioPreset` lacks `scenario_path`, add:

```rust
pub scenario_path: Option<String>,
```

In `build_config`, if `scenario_path` exists, parse that file through `RawScenarioConfig` and then apply numeric overrides.

- [ ] **Step 4: Tune only `world_baseline_stable.toml`**

Adjust only the baseline profile values needed to pass the tests:

- `mandatory_cost_per_tick`
- `max_uptake_per_tick`
- `metabolism_resource_per_tick`
- `max_radius_increase_per_tick`
- `division.energy_cost`
- `division.radius_threshold`
- `decomposition.resources_per_tick`
- `decomposition.materials_per_tick`
- `joints.creation_material_cost`
- `joints.decay_rate`

Do not tune all sweeps and do not add special case code for this scenario.

- [ ] **Step 5: Run GREEN**

Run:

```bash
cargo test --test phase2i_integrated_world
cargo test --test phase2i_world_configs
```

Expected: PASS.

- [ ] **Step 6: Commit**

```bash
git add src/bin/sweep_analyzer.rs config/analyzer/sweep_analyzer_smoke.toml config/scenarios/world tests/phase2i_integrated_world.rs
git commit -m "feat: add integrated world calibration gate"
```

---

## Task 8: Full Verification And Phase 2I Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2I-integrated-world-calibration.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run focused Phase 2I tests**

Run:

```bash
cargo test --test phase2i_accounting
cargo test --test phase2i_noop_candidates
cargo test --test phase2i_analyzer_warnings
cargo test --test phase2i_joint_transfer_audit
cargo test --test phase2i_world_configs
cargo test --test phase2i_integrated_world
```

Expected: PASS.

- [ ] **Step 2: Run regression tests**

Run:

```bash
cargo test --test phase2g_heat_boundary_repair
cargo test --test phase2g_tick_integration
cargo test --test phase2h_joint_channels
cargo test --test phase2h_joint_lifecycle
cargo test --test phase2h_reachability
```

Expected: PASS.

- [ ] **Step 3: Run analyzer smoke**

Run:

```bash
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer_smoke.toml
```

Expected: PASS and no false warnings for integrated world, repair, heat/degradation, diffusion or Joint scenarios.

- [ ] **Step 4: Run full workspace verification**

Run:

```bash
cargo fmt --check
cargo test --workspace --all-targets
```

Expected: PASS.

- [ ] **Step 5: Create final report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2I-integrated-world-calibration.md`:

```markdown
# REPORT: Phase 2I Integrated World Calibration

## Summary

Implemented the integrated-world calibration gate after Phase 2H and before Phase 3.

## Implemented

- Normalized integrated matter accounting.
- Added no-op candidate suppression.
- Added scenario-aware analyzer warnings.
- Added Joint Resource transfer audit metrics.
- Added integrated world configs.
- Added 10k+ tick baseline gate.

## Verification

- `cargo fmt --check`
- `cargo test --workspace --all-targets`
- `cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer_smoke.toml`

## Known Limits

- This is not full ecosystem balancing.
- Genome Runtime remains out of scope.
- Full analyzer matrix runtime may remain slower than smoke.
```

- [ ] **Step 6: Update worklog index**

Append:

```markdown
- [[outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2I-integrated-world-calibration|YYYY-MM-DD-HHMM-REPORT-phase-2I-integrated-world-calibration]]
```

- [ ] **Step 7: Commit**

```bash
git add outputs/worklogs
git commit -m "docs: report phase 2i integrated world calibration"
```

---

## Final Acceptance Checklist

- [ ] `world_baseline_stable.toml` runs 10k+ ticks.
- [ ] Replay is deterministic for the same seed/config hash.
- [ ] Population remains bounded and does not immediately collapse.
- [ ] Resource cycle is active.
- [ ] Division and death are reachable, not explosive.
- [ ] Repair is useful and costly.
- [ ] Material fragments are created and converted.
- [ ] Joints are created, useful, degradable and non-duplicated.
- [ ] Joint Resource transfer audit shows no back-and-forth metric inflation.
- [ ] Matter and energy accounting are clean or explicitly accounted.
- [ ] Analyzer warnings are scenario-aware and do not flag active flat sweeps as low-information.
- [ ] No Genome, organism controller, fitness or observer input is introduced.
- [ ] Phase 1, Phase 2G and Phase 2H regression tests still pass.

## Self-Review Notes

- Spec coverage: requirements map to Tasks 1-8.
- Scope: focused calibration gate only; no Genome/evolution/ecosystem balancing.
- TDD: every behavior-changing task starts with a failing test and a named RED/GREEN command.
- Rust modeling: new state is read-only accounting projection or existing World-owned state; no `Rc<RefCell<_>>`, no observer authority, no new hot-path object graph.
