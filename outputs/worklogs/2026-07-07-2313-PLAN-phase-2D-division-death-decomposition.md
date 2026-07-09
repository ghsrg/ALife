---
tags:
  - alife
  - worklog/plan
  - tdd
  - phase/2d
  - rust
  - rust-domain-modeling
---

# Phase 2D Division, Death And Decomposition Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:test-driven-development` for every production-code change. Use `rust-domain-modeling` before changing Rust domain/storage APIs. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Complete the Phase 2 individual lifecycle loop in Rust core: living Cells can grow to division readiness, divide into two accounted daughter Cells, die, decompose into explicit local matter, and preserve deterministic replay.

**Architecture:** Keep `alife-core` as the only behavior authority. Division is a registered material/process action executed after Feasibility, not an object clone. Death keeps physical matter in-world long enough to decompose; no dead Cell disappears without an explicit accounted release/removal rule.

**Tech Stack:** Rust 2024 edition, current custom SoA `CellStore`, deterministic `TickExecutor`, existing `ResourceGrid`, `cargo test`, `cargo clippy -- -D warnings`, `cargo fmt`.

---

## Phase 2C/2I Calibration Nuances (MUST FOLLOW)

When implementing division, death, and decomposition, the following constraints and architectural design details from the Phase 2C/Phase 2I calibration MUST be strictly followed:

### 1. Root Config Directory Structure
- All new scenario configurations, parameters, sweeps, and matrices MUST be written exclusively in `config/analyzer/sweep_analyzer.toml` (and `sweep_analyzer_smoke.toml`).
- Any new observer rule adjustments MUST be stored in the generic TOML files under `config/observer/`.
- No configuration files may be placed in `docs/` or the root directory.

### 2. Scenario Registration and Validation
- Every new simulation mechanic (Division, Decomposition) MUST receive its own **activation scenario** (e.g., `division_viability`, `decomposition_viability`) immediately as it is developed.
- The new scenario IDs MUST be added to the `allowed_scenarios` whitelist inside `sweep_analyzer.rs`. Any sweeps specifying unregistered scenarios must cause validation panic.
- At least one good-condition run in the sweeps/matrices for each scenario MUST be configured to survive (`survived_to_end = true`) in an active state (exhibiting `active_fraction >= 0.70`, `metabolism_count > 0`, etc.) to prove that the mechanic provides a benefit under proper conditions and has a clear cost.
- Parameter presets (e.g., `decay_rate`, `passive_energy_income`, `initial_resources`) for these scenarios MUST be specified in the `[scenarios.<scenario_id>]` table of the TOML config to avoid warnings such as `SCENARIO_MECHANISM_NOT_ACTIVATED`.

### 3. Observer Projection & Classification
- Cell state transitions (birth of daughters, death, decomposition stages) must be properly tracked in intermediate projections (`ObservationWindow`).
- Ensure that the classifiers in `src/observer/classifiers.rs` are updated if necessary to handle the cell role, behavior, and archetype changes resulting from division and death.

### 4. Detailed Accounting Categories
- Division (splitting parent energy/resources/materials to daughters) and Decomposition (transferring dead cell matter to the grid) MUST be fully accounted for.
- Update the 11 accounting metrics in `SimResult` to trace division-related matter partitioning (including any partition loss) and decomposition resource release.
- Avoid any double counting of resources/materials in dead cell cleanup vs grid replenishment.
- Keep the `resource_balance_error` tolerance threshold at `0.05` and `energy_balance_error` at `0.01` to prevent floating-point accumulation warnings.

### 5. TPS Throughput Tracking
- Keep the `std::time::Instant` execution timer active across the simulation loop. The throughput metric `ticks_per_second` must cover the execution costs of division and decomposition.

---

## Scope

Phase 2D is based on [[outputs/worklogs/2026-07-02-1855-PLAN-phase-2-global-roadmap|Phase 2 Global Roadmap]].

Implement now:

- activate `ProcessId::Division` as a Phase 2D `Now` process;
- add explicit division config;
- create daughter Cells through deterministic partitioning;
- partition Energy Buffer, internal Resources and all Phase 2 Material buckets;
- reset runtime flags for daughters;
- place daughters near the parent and let the existing physics solver resolve remaining overlaps;
- emit deterministic division/birth/decomposition events without per-tick success-event spam;
- add simple dead-cell decomposition into local `ResourceGrid`;
- add population-level summary counters needed for Phase 2D reachability;
- keep Phase 1 and Phase 2A-2C behavior passing.

Do not implement now:

- Genome copy/mutation/HGT;
- Joints or signal channels;
- organism-level entities;
- species/fitness labels;
- proportional multi-cell uptake rewrite;
- full MaterialFragment storage model.

Phase 2D may use a documented **generic decomposition resource placeholder**. It must be explicit and accounted, because full `MaterialFragmentStore` belongs later.

## Required Context

Before implementation, read:

- [[docs/PRINCIPLES|Principles]]
- [[docs/mechanics/INDEX|Mechanics Index]]
- [[docs/mechanics/division-inheritance|Division -> Partition -> Inheritance]]
- [[docs/mechanics/material-decomposition|Material -> Fragment -> Resource / Remains]]
- [[docs/mechanics/lifecycle-transition|Cell State -> Lifecycle Transition]]
- [[docs/mechanics/matter-accounting|Matter Accounting]]
- [[docs/mechanics/capacity-accounting|Capacity Accounting]]
- [[docs/mechanics/tick-transaction|Tick Transaction]]
- [[docs/biology/lifecycle|Lifecycle]]
- [[docs/biology/division-partition|Division Partition]]
- [[docs/biology/cell-state|Cell State]]
- [[docs/world/resources|Resources]]
- [[docs/world/materials|Materials]]

Current implementation context:

- `src/core/cell_store.rs`
- `src/core/config.rs`
- `src/core/events.rs`
- `src/core/process.rs`
- `src/core/resources.rs`
- `src/core/summary.rs`
- `src/core/tick.rs`
- `src/core/world.rs`
- `tests/phase2_process_registry.rs`
- `tests/phase2_growth_smoke.rs`
- `tests/phase2_growth_accounting.rs`
- `tests/phase2_reachability.rs`

## Domain Constraints

- Division is physical partition, not clone.
- Energy Buffer is local state; during division it is partitioned, not transported.
- Matter cannot appear or silently disappear.
- Genome is Phase 3; in Phase 2D use no-op inherited placeholder only.
- RuntimeState/RuntimeFlags do not copy by default.
- Dead Cells do not execute active processes.
- Dead Cells do not disappear instantly.
- Observer metrics, lineage labels and summary counters are not behavior inputs.
- Hot loops use dense `CellIndex`; `CellId` is for events, snapshots and cold lookup.
- No `Rc`, `RefCell`, `Arc<Mutex<_>>`, `HashMap<CellId, Cell>` or object graph in hot core state.

## File Map

| File | Role | Planned Change |
| --- | --- | --- |
| `src/core/config.rs` | Runtime config and deterministic config hash | Add `DivisionConfig`, `DecompositionConfig`, validation and hash coverage |
| `src/runner/config_parser.rs` | TOML scenario parser | Parse optional `[division]` and `[decomposition]` blocks with defaults |
| `src/core/events.rs` | Compact observer event stream | Add rare lifecycle events: `CellDivided`, `CellBorn`, `CellDecomposed` |
| `src/core/process.rs` | Process registry and Feasibility types | Promote `Division` to `Now`; add rejection reasons required by division/decomposition |
| `src/core/cell_store.rs` | SoA Cell storage | Add checked daughter insertion and partition helpers without removing rows in hot path |
| `src/core/world.rs` | Domain operations over stores | Add `validate_feasibility` division checks, `execute_division`, `execute_decomposition_for_dead_cells` |
| `src/core/tick.rs` | Deterministic tick pipeline | Schedule division after lifecycle flags and before final tick commit; run decomposition after death commit |
| `src/core/summary.rs` | Run summary | Add population counters for births/divisions/dead/decomposing/decomposed |
| `tests/phase2_division_smoke.rs` | New integration tests | Division registry, feasibility, partition and daughter placement |
| `tests/phase2_decomposition_smoke.rs` | New integration tests | Death remains, decomposition release, no active processes for dead cells |
| `tests/phase2_lifecycle_loop.rs` | New integration tests | Small population survives/collapses and deterministic replay |
| `tests/phase2_reachability.rs` | Existing reachability tests | Add Phase 2D reachable mechanisms |

## Proposed Minimal API Shape

Use this shape unless existing code demands a smaller local variation.

```rust
// src/core/config.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivisionConfig {
    pub enabled: bool,
    pub energy_cost: EnergyAmount,
    pub split_ratio: f32,
    pub daughter_spacing: f32,
    pub min_daughter_radius: Radius,
    pub partition_loss_fraction: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecompositionConfig {
    pub enabled: bool,
    pub resource_layer_index: usize,
    pub resources_per_tick: ResourceAmount,
    pub materials_per_tick: MaterialAmount,
    pub remove_when_empty: bool,
}
```

```rust
// src/core/world.rs
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivisionOutcome {
    pub parent_id: CellId,
    pub daughter_a_id: CellId,
    pub daughter_b_id: CellId,
    pub daughter_a_index: CellIndex,
    pub daughter_b_index: CellIndex,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecompositionOutcome {
    pub cell_id: CellId,
    pub released_resources: ResourceAmount,
    pub remaining_resources: ResourceAmount,
    pub remaining_materials: MaterialAmount,
    pub completed: bool,
}
```

Important: `DivisionOutcome` may return a parent row reused as `daughter_a_index` if implementation chooses in-place parent replacement. That is acceptable if events and accounting call it daughter A, not parent continuation.

## Tick Order Target

Keep the current Phase 2 order, but add division/decomposition at explicit boundaries:

```text
Phase 0: rebuild spatial index
Phase 1: contact sensing and pressure / division_ready flags
Phase 2: material reflex processes
Phase 3: physics overlap/wall solver
Phase 4: lifecycle upkeep/death commit
Phase 5: division execution for living division-ready Cells
Phase 6: decomposition for dead Cells
Phase 7: resource decay/passive update, advance tick, events/summary
```

If the current code computes `division_ready` during lifecycle commit, keep that behavior only if tests prove division execution reads a committed flag in the same deterministic tick boundary. Do not let division read stale pressure from the previous tick unless documented and tested.

## No-Commit Rule

Do not create git commits unless the project owner explicitly asks. This plan uses TDD checkpoints instead of commit steps:

```powershell
cargo test --test <test_name> <test_filter>
cargo test --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo fmt --check
```

At the end, create a report:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2D-division-death-decomposition.md
```

---

## Task 1: Activate Division Registry Contract

**Files:**

- Modify: `src/core/process.rs`
- Modify: `tests/phase2_process_registry.rs`

- [ ] **Step 1: Write the failing registry test**

Change the existing test that currently expects `Division` to be `Future`.

```rust
#[test]
fn test_division_is_now_status_in_phase_2d() {
    assert_eq!(
        ProcessSpec::for_id(ProcessId::Division).status,
        ProcessStatus::Now,
        "Division must be executable in Phase 2D"
    );
}
```

- [ ] **Step 2: Run the focused RED test**

```powershell
cargo test --test phase2_process_registry test_division_is_now_status_in_phase_2d
```

Expected: FAIL because `ProcessId::Division` is still registered as `ProcessStatus::Future`.

- [ ] **Step 3: Minimal GREEN implementation**

In `src/core/process.rs`, change only the `Division` registry entry:

```rust
ProcessSpec {
    process_id: ProcessId::Division,
    status: ProcessStatus::Now,
    required_capabilities: &[],
    description: "Splits one living cell into two accounted daughter cells.",
},
```

- [ ] **Step 4: Verify GREEN**

```powershell
cargo test --test phase2_process_registry test_division_is_now_status_in_phase_2d
```

Expected: PASS.

- [ ] **Step 5: Run local process registry suite**

```powershell
cargo test --test phase2_process_registry
```

Expected: PASS.

---

## Task 2: Add Division And Decomposition Config Contracts

**Files:**

- Modify: `src/core/config.rs`
- Modify: `src/runner/config_parser.rs`
- Create: `tests/phase2_division_smoke.rs`

- [ ] **Step 1: Write failing config defaults test**

Create `tests/phase2_division_smoke.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_phase2d_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(32.0, 32.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(80.0).unwrap(),
        energy_capacity: EnergyAmount::new(100.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(0.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(100.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(20.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(2.0).unwrap(),
        initial_transport_material: MaterialAmount::new(2.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(2.0).unwrap(),
        initial_storage_material: MaterialAmount::new(2.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(2.0).unwrap(),
        initial_structural_material: MaterialAmount::new(12.0).unwrap(),
        initial_repair_material: MaterialAmount::new(2.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(2.0).unwrap(),
    };

    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(20),
            seed: Seed::from_raw(7),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 4,
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
            stress_energy_threshold: EnergyAmount::new(5.0).unwrap(),
            dormancy_allowed: false,
            dormant_mandatory_cost_modifier: 1.0,
            critical_capacity_overrun: CapacityAmount::new(20.0).unwrap(),
        },
    )
    .unwrap();
    config.growth_enabled = true;
    config.growth.growth_target_radius = Radius::new(2.0).unwrap();
    config.growth.max_division_pressure = 0.5;
    config
}

#[test]
fn division_and_decomposition_configs_have_safe_defaults() {
    let config = base_phase2d_config();
    assert!(!config.division.enabled, "division must be explicit");
    assert!(!config.decomposition.enabled, "decomposition must be explicit");
    assert_eq!(config.division.split_ratio, 0.5);
    assert_eq!(config.division.partition_loss_fraction, 0.0);
}
```

- [ ] **Step 2: Run RED**

```powershell
cargo test --test phase2_division_smoke division_and_decomposition_configs_have_safe_defaults
```

Expected: FAIL because `RuntimeConfig` has no `division` or `decomposition`.

- [ ] **Step 3: Add minimal config structs**

Add to `src/core/config.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivisionConfig {
    pub enabled: bool,
    pub energy_cost: EnergyAmount,
    pub split_ratio: f32,
    pub daughter_spacing: f32,
    pub min_daughter_radius: Radius,
    pub partition_loss_fraction: f32,
}

impl Default for DivisionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            energy_cost: EnergyAmount::zero(),
            split_ratio: 0.5,
            daughter_spacing: 0.25,
            min_daughter_radius: Radius::new(0.5).unwrap(),
            partition_loss_fraction: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DecompositionConfig {
    pub enabled: bool,
    pub resource_layer_index: usize,
    pub resources_per_tick: ResourceAmount,
    pub materials_per_tick: MaterialAmount,
    pub remove_when_empty: bool,
}

impl Default for DecompositionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            resource_layer_index: 0,
            resources_per_tick: ResourceAmount::zero(),
            materials_per_tick: MaterialAmount::zero(),
            remove_when_empty: false,
        }
    }
}
```

Add fields to `RuntimeConfig`:

```rust
pub division: DivisionConfig,
pub decomposition: DecompositionConfig,
```

Initialize in `RuntimeConfig::new`:

```rust
division: DivisionConfig::default(),
decomposition: DecompositionConfig::default(),
```

- [ ] **Step 4: Add validation**

Extend `ConfigError`:

```rust
InvalidDivisionSplit,
InvalidDivisionLoss,
InvalidDaughterSpacing,
InvalidDecompositionLayer,
```

Add `RuntimeConfig::validate_phase2d_options(&self) -> Result<(), ConfigError>` or inline validation in places that mutate configs. The validation must enforce:

```rust
if !(0.35..=0.65).contains(&self.division.split_ratio) {
    return Err(ConfigError::InvalidDivisionSplit);
}
if !(0.0..=0.25).contains(&self.division.partition_loss_fraction) {
    return Err(ConfigError::InvalidDivisionLoss);
}
if !self.division.daughter_spacing.is_finite() || self.division.daughter_spacing < 0.0 {
    return Err(ConfigError::InvalidDaughterSpacing);
}
if self.decomposition.resource_layer_index >= self.resources.layer_count() {
    return Err(ConfigError::InvalidDecompositionLayer);
}
```

- [ ] **Step 5: Add hash coverage test**

In `tests/phase2_division_smoke.rs`:

```rust
#[test]
fn division_and_decomposition_config_changes_affect_config_hash() {
    let base = base_phase2d_config();

    let mut division_changed = base.clone();
    division_changed.division.enabled = true;
    assert_ne!(base.config_hash(), division_changed.config_hash());

    let mut decomposition_changed = base.clone();
    decomposition_changed.decomposition.enabled = true;
    assert_ne!(base.config_hash(), decomposition_changed.config_hash());
}
```

- [ ] **Step 6: Run RED for hash coverage**

```powershell
cargo test --test phase2_division_smoke division_and_decomposition_config_changes_affect_config_hash
```

Expected: FAIL until `config_hash()` includes new fields.

- [ ] **Step 7: Add config hash coverage**

In `RuntimeConfig::config_hash()`, fold:

```rust
self.division.enabled as u64,
self.division.energy_cost.raw().to_bits() as u64,
self.division.split_ratio.to_bits() as u64,
self.division.daughter_spacing.to_bits() as u64,
self.division.min_daughter_radius.raw().to_bits() as u64,
self.division.partition_loss_fraction.to_bits() as u64,
self.decomposition.enabled as u64,
self.decomposition.resource_layer_index as u64,
self.decomposition.resources_per_tick.raw().to_bits() as u64,
self.decomposition.materials_per_tick.raw().to_bits() as u64,
self.decomposition.remove_when_empty as u64,
```

- [ ] **Step 8: Add parser defaults**

In `src/runner/config_parser.rs`, parse optional blocks:

```toml
[division]
enabled = true
energy_cost = 2.0
split_ratio = 0.5
daughter_spacing = 0.25
min_daughter_radius = 0.5
partition_loss_fraction = 0.0

[decomposition]
enabled = true
resource_layer_index = 0
resources_per_tick = 1.0
materials_per_tick = 1.0
remove_when_empty = false
```

If block is absent, keep defaults.

- [ ] **Step 9: Verify**

```powershell
cargo test --test phase2_division_smoke
cargo test --workspace --all-targets
```

Expected: PASS.

---

## Task 3: Add Daughter Insertion And Partition Helpers In CellStore

**Files:**

- Modify: `src/core/cell_store.rs`
- Extend: `tests/phase2_division_smoke.rs`

- [ ] **Step 1: Write failing partition helper test**

Add:

```rust
#[test]
fn cell_store_can_insert_partitioned_daughter_without_copying_runtime_flags() {
    use alife::core::cell_store::{CellIndex, InitialCellState, RuntimeFlags};
    use alife::core::tick::TickExecutor;

    let mut exec = TickExecutor::new(base_phase2d_config()).unwrap();
    let parent = CellIndex::from_raw(0);

    exec.world_mut().cells_mut_for_commit().set_runtime_flags(
        parent,
        RuntimeFlags {
            mandatory_paid: true,
            stalled: true,
            over_capacity: true,
            inert: false,
            division_ready: true,
        },
    );

    let daughter_state = InitialCellState {
        position: Position::new(34.0, 32.0),
        radius: Radius::new(1.0).unwrap(),
        energy: exec.world().cells().energy(parent),
        resources: ResourceAmount::new(5.0).unwrap(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::new(1.0).unwrap(),
        metabolic_material: MaterialAmount::new(1.0).unwrap(),
        storage_material: MaterialAmount::new(1.0).unwrap(),
        synthesis_material: MaterialAmount::new(1.0).unwrap(),
        structural_material: MaterialAmount::new(6.0).unwrap(),
        repair_material: MaterialAmount::new(1.0).unwrap(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::new(1.0).unwrap(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        temperature: alife::core::units::Temperature::new(25.0),
    };

    let id = exec
        .world_mut()
        .cells_mut_for_commit()
        .insert_partitioned_daughter(daughter_state);
    let idx = exec.world().cells().resolve_id_cold(id).unwrap();
    assert_eq!(idx.raw(), 1);
    assert_eq!(exec.world().cells().runtime_flags(idx), RuntimeFlags::default());
    assert_eq!(exec.world().cells().lifecycle_state(idx), alife::core::cell_store::LifecycleState::Alive);
}
```

- [ ] **Step 2: Run RED**

```powershell
cargo test --test phase2_division_smoke cell_store_can_insert_partitioned_daughter_without_copying_runtime_flags
```

Expected: FAIL because `insert_partitioned_daughter` does not exist.

- [ ] **Step 3: Minimal GREEN implementation**

Add to `impl CellStore`:

```rust
pub fn insert_partitioned_daughter(&mut self, cell: InitialCellState) -> CellId {
    self.insert_initial(cell)
}
```

This intentionally reuses `insert_initial`, which creates a new stable `CellId`, `Alive` lifecycle state and default runtime flags. Do not add a parallel insertion path unless later tests require it.

- [ ] **Step 4: Verify**

```powershell
cargo test --test phase2_division_smoke cell_store_can_insert_partitioned_daughter_without_copying_runtime_flags
```

Expected: PASS.

---

## Task 4: Implement Division Feasibility With Costs And Space Pressure

**Files:**

- Modify: `src/core/process.rs`
- Modify: `src/core/world.rs`
- Extend: `tests/phase2_division_smoke.rs`

- [ ] **Step 1: Write failing feasibility tests**

Add:

```rust
#[test]
fn division_rejects_when_disabled_or_energy_cost_unpaid() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId, RejectionReason};
    use alife::core::tick::TickExecutor;

    let mut disabled = base_phase2d_config();
    disabled.division.enabled = false;
    let exec = TickExecutor::new(disabled).unwrap();
    let candidate = ActionCandidate { process_id: ProcessId::Division, requested_amount: 0.0 };
    assert!(matches!(
        exec.world().validate_feasibility(CellIndex::from_raw(0), &candidate),
        FeasibilityResult::Rejected(RejectionReason::ProcessDisabled)
    ));

    let mut low_energy = base_phase2d_config();
    low_energy.division.enabled = true;
    low_energy.division.energy_cost = EnergyAmount::new(90.0).unwrap();
    let exec = TickExecutor::new(low_energy).unwrap();
    assert!(matches!(
        exec.world().validate_feasibility(CellIndex::from_raw(0), &candidate),
        FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy)
    ));
}

#[test]
fn division_allowed_when_ready_enabled_low_pressure_and_energy_available() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, FeasibilityResult, ProcessId};
    use alife::core::tick::TickExecutor;

    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(2.0).unwrap();
    let exec = TickExecutor::new(config).unwrap();
    let candidate = ActionCandidate { process_id: ProcessId::Division, requested_amount: 0.0 };

    assert!(matches!(
        exec.world().validate_feasibility(CellIndex::from_raw(0), &candidate),
        FeasibilityResult::Allowed { energy_cost: 2.0, .. }
    ));
}
```

- [ ] **Step 2: Run RED**

```powershell
cargo test --test phase2_division_smoke division_rejects_when_disabled_or_energy_cost_unpaid division_allowed_when_ready_enabled_low_pressure_and_energy_available
```

Expected: FAIL because `ProcessDisabled` is missing and Division ignores config/energy cost.

- [ ] **Step 3: Add rejection reason**

In `src/core/process.rs`:

```rust
ProcessDisabled,
```

Add it to derives already used for diagnostics: `Clone, Copy, Debug, PartialEq, Eq, Hash`.

- [ ] **Step 4: Update division feasibility**

In `WorldState::validate_feasibility`, inside `ProcessId::Division`:

```rust
if !self.config.division.enabled {
    return FeasibilityResult::Rejected(RejectionReason::ProcessDisabled);
}

let current_eng = self.cells.energy(cell_idx).current().raw();
let cost_eng = self.config.division.energy_cost.raw();
if current_eng < cost_eng {
    return FeasibilityResult::Rejected(RejectionReason::InsufficientEnergy);
}
```

Keep existing checks:

- reject `LifecycleStateDead`;
- reject below radius target;
- reject pressure above max.

Return:

```rust
FeasibilityResult::Allowed {
    accepted_amount: 1.0,
    energy_cost: cost_eng,
    resource_cost: 0.0,
}
```

- [ ] **Step 5: Verify**

```powershell
cargo test --test phase2_division_smoke division_rejects_when_disabled_or_energy_cost_unpaid division_allowed_when_ready_enabled_low_pressure_and_energy_available
```

Expected: PASS.

---

## Task 5: Execute Division With Accounted Partition

**Files:**

- Modify: `src/core/world.rs`
- Extend: `tests/phase2_division_smoke.rs`

- [ ] **Step 1: Write failing execution test**

Add:

```rust
#[test]
fn division_creates_two_daughters_and_partitions_accounted_state() {
    use alife::core::cell_store::CellIndex;
    use alife::core::process::{ActionCandidate, ProcessId};
    use alife::core::tick::TickExecutor;

    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(2.0).unwrap();
    config.division.split_ratio = 0.5;
    config.division.partition_loss_fraction = 0.0;

    let mut exec = TickExecutor::new(config).unwrap();
    let parent = CellIndex::from_raw(0);
    let before_energy = exec.world().cells().energy(parent).current().raw();
    let before_resources = exec.world().cells().resource_amount(parent).raw();
    let before_materials = exec.world().cells().total_materials(parent).raw();

    let outcome = exec
        .world_mut()
        .execute_division(parent, &ActionCandidate {
            process_id: ProcessId::Division,
            requested_amount: 1.0,
        })
        .expect("division should execute");

    assert_eq!(exec.world().cells().len(), 2);
    assert_ne!(outcome.daughter_a_id, outcome.daughter_b_id);

    let a = outcome.daughter_a_index;
    let b = outcome.daughter_b_index;
    let after_energy = exec.world().cells().energy(a).current().raw()
        + exec.world().cells().energy(b).current().raw();
    let after_resources = exec.world().cells().resource_amount(a).raw()
        + exec.world().cells().resource_amount(b).raw();
    let after_materials = exec.world().cells().total_materials(a).raw()
        + exec.world().cells().total_materials(b).raw();

    assert!((after_energy - (before_energy - 2.0)).abs() < 0.001);
    assert!((after_resources - before_resources).abs() < 0.001);
    assert!((after_materials - before_materials).abs() < 0.001);
    assert_eq!(exec.world().cells().runtime_flags(a), Default::default());
    assert_eq!(exec.world().cells().runtime_flags(b), Default::default());
}
```

- [ ] **Step 2: Run RED**

```powershell
cargo test --test phase2_division_smoke division_creates_two_daughters_and_partitions_accounted_state
```

Expected: FAIL because `execute_division` and `DivisionOutcome` do not exist.

- [ ] **Step 3: Add outcome type**

In `src/core/world.rs`:

```rust
use crate::core::ids::CellId;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct DivisionOutcome {
    pub parent_id: CellId,
    pub daughter_a_id: CellId,
    pub daughter_b_id: CellId,
    pub daughter_a_index: CellIndex,
    pub daughter_b_index: CellIndex,
}
```

- [ ] **Step 4: Implement minimal deterministic split**

Add `WorldState::execute_division`.

Implementation rules:

- call `validate_feasibility` first;
- if rejected, return `Err(format!("{:?}", reason))`;
- subtract `division.energy_cost` before splitting Energy;
- split ratio comes from config and is already validated;
- `partition_loss_fraction` applies only to Resources and Materials in Phase 2D; Energy loss is only `energy_cost`;
- parent row becomes daughter A;
- new inserted row becomes daughter B;
- runtime flags reset for both daughters;
- lifecycle state for both daughters is `Alive`;
- daughter radius starts at `max(parent_radius * sqrt(split), min_daughter_radius)`;
- capacity limit splits by ratio;
- positions are offset left/right along X by `radius + daughter_spacing`, then clamped to solid wall bounds.

Pseudo-code to implement:

```rust
let ratio = self.config.division.split_ratio;
let inv_ratio = 1.0 - ratio;
let loss_keep = 1.0 - self.config.division.partition_loss_fraction;

let parent_id = self.cells.id_at(cell_idx);
let parent_pos = self.cells.position(cell_idx);
let parent_radius = self.cells.radius(cell_idx).raw();
let parent_energy = self.cells.energy(cell_idx);
let energy_after_cost = parent_energy
    .current()
    .saturating_sub(self.config.division.energy_cost);

let a_energy = EnergyAmount::new_unchecked(energy_after_cost.raw() * ratio);
let b_energy = EnergyAmount::new_unchecked(energy_after_cost.raw() * inv_ratio);
let a_capacity = EnergyAmount::new_unchecked(parent_energy.capacity().raw() * ratio);
let b_capacity = EnergyAmount::new_unchecked(parent_energy.capacity().raw() * inv_ratio);

let split_resource = |amount: ResourceAmount, r: f32| {
    ResourceAmount::new_unchecked(amount.raw() * r * loss_keep)
};
let split_material = |amount: MaterialAmount, r: f32| {
    MaterialAmount::new_unchecked(amount.raw() * r * loss_keep)
};
```

Do not use `set_materials()` because it redistributes material types and can grant capabilities. Use specific setters for each material bucket.

For parent/daughter A:

```rust
self.cells.set_energy(cell_idx, EnergyBuffer::new(a_energy, a_capacity));
self.cells.set_resources(cell_idx, split_resource(parent_resources, ratio));
self.cells.set_boundary_material(cell_idx, split_material(parent_boundary, ratio));
// repeat all material buckets
self.cells.set_radius(cell_idx, a_radius);
self.cells.set_capacity_limit(cell_idx, a_capacity_limit);
self.cells.set_position(cell_idx, a_position);
self.cells.set_lifecycle_state(cell_idx, LifecycleState::Alive);
self.cells.set_runtime_flags(cell_idx, RuntimeFlags::default());
```

For daughter B, build `InitialCellState` and call `insert_partitioned_daughter`.

- [ ] **Step 5: Verify focused test**

```powershell
cargo test --test phase2_division_smoke division_creates_two_daughters_and_partitions_accounted_state
```

Expected: PASS.

---

## Task 6: Wire Division Into Tick Executor

**Files:**

- Modify: `src/core/events.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Extend: `tests/phase2_division_smoke.rs`

- [ ] **Step 1: Write failing tick-level division test**

Add:

```rust
#[test]
fn tick_executor_divides_ready_cell_once_and_emits_events() {
    use alife::core::events::EventKind;
    use alife::core::tick::TickExecutor;

    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(2.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let summary = exec.step().unwrap();

    assert_eq!(exec.world().cells().len(), 2);
    assert_eq!(summary.metrics.divisions_count, 1);
    assert_eq!(summary.metrics.births_count, 1);
    assert!(exec
        .world()
        .events()
        .iter_ordered()
        .any(|event| event.kind == EventKind::CellDivided));
    assert!(exec
        .world()
        .events()
        .iter_ordered()
        .any(|event| event.kind == EventKind::CellBorn));
}
```

- [ ] **Step 2: Run RED**

```powershell
cargo test --test phase2_division_smoke tick_executor_divides_ready_cell_once_and_emits_events
```

Expected: FAIL because events and summary counters do not exist and tick does not execute division.

- [ ] **Step 3: Add event kinds**

In `src/core/events.rs`:

```rust
CellDivided,
CellBorn,
CellDecomposed,
```

These are rare events. Do not add success events for mandatory upkeep, uptake, metabolism, synthesis or growth.

- [ ] **Step 4: Add summary counters**

In `src/core/summary.rs`, add fields to `MetricsSummary`:

```rust
pub alive_cells_count: u32,
pub dead_cells_count: u32,
pub divisions_count: u32,
pub births_count: u32,
pub decomposed_cells_count: u32,
```

Update all `MetricsSummary` constructors in `src/core/tick.rs`.

- [ ] **Step 5: Execute division in tick**

In `TickExecutor::step`, after lifecycle commit and before resource decay/advance:

```rust
let mut division_candidates = Vec::new();
for i in 0..self.world.cells().len() {
    let idx = CellIndex::from_raw(i);
    if self.world.cells().lifecycle_state(idx) == LifecycleState::Dead {
        continue;
    }
    if self.world.cells().runtime_flags(idx).division_ready {
        division_candidates.push(idx);
    }
}

let mut divisions_count = 0_u32;
let mut births_count = 0_u32;
for idx in division_candidates {
    if idx.raw() >= self.world.cells().len() {
        continue;
    }
    let candidate = ActionCandidate {
        process_id: ProcessId::Division,
        requested_amount: 1.0,
    };
    if let Ok(outcome) = self.world.execute_division(idx, &candidate) {
        divisions_count += 1;
        births_count += 1;
        let tick = self.world.tick();
        self.world.events_mut_for_commit().push(
            tick,
            EventKind::CellDivided,
            Some(outcome.parent_id),
        );
        self.world.events_mut_for_commit().push(
            tick,
            EventKind::CellBorn,
            Some(outcome.daughter_b_id),
        );
    }
}
```

Important:

- collect candidates before mutating `CellStore`;
- do not iterate over daughters inserted during the same phase;
- if pressure/collision makes a later candidate infeasible, it must be rejected deterministically.

- [ ] **Step 6: Update `build_metrics_summary`**

Count living and dead cells across the full store:

```rust
let mut alive_cells_count = 0_u32;
let mut dead_cells_count = 0_u32;
for i in 0..cells.len() {
    let idx = CellIndex::from_raw(i);
    match cells.lifecycle_state(idx) {
        LifecycleState::Dead => dead_cells_count += 1,
        _ => alive_cells_count += 1,
    }
}
```

Thread `divisions_count`, `births_count`, `decomposed_cells_count` into `build_metrics_summary`.

- [ ] **Step 7: Verify focused test**

```powershell
cargo test --test phase2_division_smoke tick_executor_divides_ready_cell_once_and_emits_events
```

Expected: PASS.

---

## Task 7: Prevent Same-Tick Division Cascade And Preserve Determinism

**Files:**

- Extend: `tests/phase2_division_smoke.rs`
- Create: `tests/phase2_lifecycle_loop.rs`

- [ ] **Step 1: Write cascade prevention test**

Add:

```rust
#[test]
fn daughters_do_not_divide_again_in_same_tick() {
    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::zero();
    config.division.min_daughter_radius = Radius::new(0.5).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();

    assert_eq!(
        exec.world().cells().len(),
        2,
        "division must use candidates collected before daughter insertion"
    );
}
```

- [ ] **Step 2: Write deterministic replay test**

Create `tests/phase2_lifecycle_loop.rs`:

```rust
use alife::core::tick::TickExecutor;

#[test]
fn division_replay_is_deterministic_for_same_seed_and_config() {
    // Copy the full `base_phase2d_config()` helper from `tests/phase2_division_smoke.rs`
    // into this integration test file so the test remains standalone.
    let mut config_a = base_phase2d_config();
    config_a.division.enabled = true;

    let config_b = config_a.clone();

    let mut a = TickExecutor::new(config_a).unwrap();
    let mut b = TickExecutor::new(config_b).unwrap();

    let summary_a = a.run_until_configured_tick().unwrap();
    let summary_b = b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a, summary_b);
    assert_eq!(a.world().cells().len(), b.world().cells().len());
    for i in 0..a.world().cells().len() {
        let idx = alife::core::cell_store::CellIndex::from_raw(i);
        assert_eq!(a.world().cells().position(idx), b.world().cells().position(idx));
        assert_eq!(a.world().cells().radius(idx), b.world().cells().radius(idx));
        assert_eq!(a.world().cells().energy(idx), b.world().cells().energy(idx));
    }
}
```

Do not introduce production test-only APIs for sharing test helpers.

- [ ] **Step 3: Run RED/GREEN**

```powershell
cargo test --test phase2_division_smoke daughters_do_not_divide_again_in_same_tick
cargo test --test phase2_lifecycle_loop division_replay_is_deterministic_for_same_seed_and_config
```

Expected: PASS if Task 6 collected candidates before insertion. If failing, fix tick candidate collection, not the test.

---

## Task 8: Add Decomposition Release For Dead Cells

**Files:**

- Modify: `src/core/world.rs`
- Modify: `src/core/tick.rs`
- Create: `tests/phase2_decomposition_smoke.rs`

- [ ] **Step 1: Write failing decomposition test**

Create `tests/phase2_decomposition_smoke.rs`:

```rust
use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::resources::ResourceLayerIndex;
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn decomposition_config() -> RuntimeConfig {
    let cell = CellInitialConfig {
        position: Position::new(16.0, 16.0),
        radius: Radius::new(1.0).unwrap(),
        initial_energy: EnergyAmount::new(1.0).unwrap(),
        energy_capacity: EnergyAmount::new(1.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::new(10.0).unwrap(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(6.0).unwrap(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::new(1.0).unwrap(),
        initial_metabolic_material: MaterialAmount::new(1.0).unwrap(),
        initial_storage_material: MaterialAmount::new(1.0).unwrap(),
        initial_synthesis_material: MaterialAmount::new(1.0).unwrap(),
        initial_structural_material: MaterialAmount::new(2.0).unwrap(),
        initial_repair_material: MaterialAmount::new(1.0).unwrap(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(1.0).unwrap(),
    };

    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(5),
            seed: Seed::from_raw(3),
            size: WorldSize::new(32.0, 32.0).unwrap(),
        },
        SpaceConfig { spatial_grid_size: 8.0, physics_solver_iterations: 1 },
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
            critical_capacity_overrun: CapacityAmount::new(20.0).unwrap(),
        },
    )
    .unwrap();
    config.decomposition.enabled = true;
    config.decomposition.resource_layer_index = 0;
    config.decomposition.resources_per_tick = ResourceAmount::new(2.0).unwrap();
    config.decomposition.materials_per_tick = MaterialAmount::new(2.0).unwrap();
    config
}

#[test]
fn dead_cell_releases_resources_to_local_grid_without_disappearing() {
    let mut exec = TickExecutor::new(decomposition_config()).unwrap();
    let coord = exec
        .world()
        .resources()
        .coord_for_position(exec.world().cells().position(CellIndex::from_raw(0)));

    let before = exec
        .world()
        .resources()
        .amount_at(ResourceLayerIndex::from_raw(0), coord)
        .unwrap()
        .raw();

    let summary = exec.step().unwrap();

    assert_eq!(
        exec.world().cells().lifecycle_state(CellIndex::from_raw(0)),
        LifecycleState::Dead
    );
    assert_eq!(exec.world().cells().len(), 1, "dead cell must remain as physical state");
    assert!(summary.metrics.dead_cells_count >= 1);

    let after = exec
        .world()
        .resources()
        .amount_at(ResourceLayerIndex::from_raw(0), coord)
        .unwrap()
        .raw();
    assert!(after > before, "decomposition must release local resources");
}
```

- [ ] **Step 2: Run RED**

```powershell
cargo test --test phase2_decomposition_smoke dead_cell_releases_resources_to_local_grid_without_disappearing
```

Expected: FAIL because decomposition does not exist.

- [ ] **Step 3: Add `WorldState::execute_decomposition_for_dead_cells`**

Implementation rules:

- if `config.decomposition.enabled == false`, do nothing;
- iterate dense indices in ascending order;
- only `LifecycleState::Dead` decomposes;
- release up to `resources_per_tick` from internal resources into the configured `ResourceGrid` layer at the dead Cell position;
- release up to `materials_per_tick` from materials by reducing material buckets in deterministic order:
  1. boundary
  2. transport
  3. metabolic
  4. storage
  5. synthesis
  6. structural
  7. repair
  8. contractile
  9. sensory
- converted material amount becomes generic resource in the configured layer;
- no `MaterialFragmentStore` yet; document this as Phase 2D placeholder in code comment;
- dead row remains in `CellStore`;
- if `remove_when_empty` is true, do not physically remove rows in Phase 2D; instead set runtime `inert = true` and count `completed`. Row removal causes index instability and belongs to a later compaction task.

Expected helper shape:

```rust
pub fn execute_decomposition_for_dead_cells(&mut self) -> u32 {
    if !self.config.decomposition.enabled {
        return 0;
    }
    let mut completed = 0_u32;
    for i in 0..self.cells.len() {
        let idx = CellIndex::from_raw(i);
        if self.cells.lifecycle_state(idx) != LifecycleState::Dead {
            continue;
        }
        // release resources and materials into ResourceGrid
        // set inert flag when no remaining resources/materials
    }
    completed
}
```

- [ ] **Step 4: Wire into `TickExecutor::step`**

After lifecycle death commit and after division execution:

```rust
let decomposed_cells_count = self.world.execute_decomposition_for_dead_cells();
```

Emit `EventKind::CellDecomposed` only when a Cell reaches completed inert/no-matter state, not on every release tick.

- [ ] **Step 5: Verify**

```powershell
cargo test --test phase2_decomposition_smoke
```

Expected: PASS.

---

## Task 9: Ensure Dead Cells Do Not Execute Active Processes

**Files:**

- Extend: `tests/phase2_decomposition_smoke.rs`

- [ ] **Step 1: Write failing/characterization test**

Add:

```rust
#[test]
fn dead_cells_do_not_uptake_metabolize_grow_or_divide() {
    use alife::core::process::ProcessId;

    let mut config = decomposition_config();
    config.resource_interaction.enabled = true;
    config.resource_interaction.max_uptake_per_tick = ResourceAmount::new(10.0).unwrap();
    config.resource_interaction.metabolism_resource_per_tick = ResourceAmount::new(1.0).unwrap();
    config.growth_enabled = true;
    config.division.enabled = true;

    let mut exec = TickExecutor::new(config).unwrap();
    let _ = exec.step().unwrap();
    let dead_energy_after_first_step = exec.world().cells().energy(CellIndex::from_raw(0)).current().raw();
    let _ = exec.step().unwrap();

    assert_eq!(
        exec.world().cells().lifecycle_state(CellIndex::from_raw(0)),
        LifecycleState::Dead
    );
    assert!(
        exec.world().cells().energy(CellIndex::from_raw(0)).current().raw()
            <= dead_energy_after_first_step + f32::EPSILON,
        "dead cells must not generate energy through metabolism"
    );
    assert_eq!(exec.world().cells().len(), 1, "dead cell must not divide");

    let division_attempts = exec
        .run_until_configured_tick()
        .unwrap()
        .diagnostics
        .attempts_by_process
        .get(&ProcessId::Division)
        .copied()
        .unwrap_or(0);
    assert_eq!(division_attempts, 0, "dead cells must not attempt division");
}
```

- [ ] **Step 2: Run test**

```powershell
cargo test --test phase2_decomposition_smoke dead_cells_do_not_uptake_metabolize_grow_or_divide
```

Expected: PASS if existing `Dead` skip logic is correct. If it fails, fix `TickExecutor` so all active process phases skip dead Cells.

---

## Task 10: Small Population Lifecycle Scenarios

**Files:**

- Extend: `tests/phase2_lifecycle_loop.rs`

- [ ] **Step 1: Add stable small population test**

```rust
#[test]
fn small_population_can_grow_divide_and_remain_deterministic_for_n_ticks() {
    let mut config = base_phase2d_config();
    config.world.tick_count = alife::core::units::Tick::from_raw(25);
    config.division.enabled = true;
    config.division.energy_cost = EnergyAmount::new(1.0).unwrap();
    config.decomposition.enabled = true;
    config.decomposition.resources_per_tick = ResourceAmount::new(1.0).unwrap();
    config.decomposition.materials_per_tick = MaterialAmount::new(1.0).unwrap();

    let mut a = TickExecutor::new(config.clone()).unwrap();
    let mut b = TickExecutor::new(config).unwrap();

    let summary_a = a.run_until_configured_tick().unwrap();
    let summary_b = b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a, summary_b);
    assert!(a.world().cells().len() >= 2);
    assert!(summary_a.metrics.divisions_count >= 1);
}
```

- [ ] **Step 2: Add resource exhaustion collapse test**

```rust
#[test]
fn small_population_collapses_when_energy_path_is_exhausted() {
    let mut config = base_phase2d_config();
    config.world.tick_count = Tick::from_raw(30);
    config.cell.initial_energy = EnergyAmount::new(3.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(5.0).unwrap();
    config.cell.initial_resource_amount = ResourceAmount::zero();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.division.enabled = true;
    config.resource_interaction = ResourceInteractionConfig::disabled();

    let mut exec = TickExecutor::new(config).unwrap();
    let summary = exec.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, alife::core::summary::SurvivalResult::Collapse);
    assert!(summary.metrics.dead_cells_count >= 1);
}
```

- [ ] **Step 3: Verify lifecycle suite**

```powershell
cargo test --test phase2_lifecycle_loop
```

Expected: PASS.

---

## Task 11: Phase 2D Reachability Coverage

**Files:**

- Modify: `tests/phase2_reachability.rs`

- [ ] **Step 1: Add reachability assertions**

Add or extend tests so the following mechanisms are explicitly covered:

```text
division_execution_reachable
daughter_partition_reachable
birth_event_reachable
death_event_reachable
decomposition_reachable
small_population_stable_reachable
small_population_collapse_reachable
```

Example shape:

```rust
#[test]
fn phase2d_reachability_division_death_decomposition() {
    let mut config = base_phase2d_config();
    config.division.enabled = true;
    config.decomposition.enabled = true;

    let mut exec = TickExecutor::new(config).unwrap();
    let summary = exec.step().unwrap();

    assert!(summary.metrics.divisions_count >= 1, "division_execution_reachable");
    assert!(summary.metrics.births_count >= 1, "birth_event_reachable");
    assert!(exec.world().cells().len() >= 2, "daughter_partition_reachable");
}
```

For death/decomposition, use the config from `tests/phase2_decomposition_smoke.rs` or duplicate a compact local builder.

- [ ] **Step 2: Run reachability**

```powershell
cargo test --test phase2_reachability
```

Expected: PASS.

---

## Task 12: Parser Scenario Coverage

**Files:**

- Modify: `src/runner/config_parser.rs`
- Create or extend: `tests/phase2_division_smoke.rs`

- [ ] **Step 1: Add parser test for `[division]` and `[decomposition]`**

```rust
#[test]
fn parser_loads_phase2d_division_and_decomposition_blocks() {
    let toml = r#"
scenario_id = "phase2d_parser"
seed = 7
tick_count = 10

[world]
size = [64.0, 64.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 4

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [0.0]
optional_decay_rate = 0.0

[cell]
initial_position = [32.0, 32.0]
radius = 2.0
initial_resources = { nutrient = 20.0 }
initial_materials = { boundary = 2.0, transport = 2.0, metabolic = 2.0, storage = 2.0, synthesis = 2.0, structural = 12.0, repair = 2.0, sensory = 2.0 }
initial_energy = 80.0
energy_capacity = 100.0
mandatory_cost_per_tick = 0.0
capacity_limit = 100.0

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 100.0
heat_death_threshold = 200.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 100.0
waste_death_threshold = 200.0

[lifecycle]
stress_energy_threshold = 5.0
dormancy_allowed = false
critical_capacity_overrun = 20.0

[growth]
growth_cost_resource = 2.0
growth_cost_energy = 1.0
growth_target_radius = 2.0
max_division_pressure = 0.5

[division]
enabled = true
energy_cost = 2.0
split_ratio = 0.5
daughter_spacing = 0.25
min_daughter_radius = 0.5
partition_loss_fraction = 0.0

[decomposition]
enabled = true
resource_layer_index = 0
resources_per_tick = 1.0
materials_per_tick = 1.0
remove_when_empty = false
"#;

    let config = alife::runner::config_parser::RawScenarioConfig::parse(toml).unwrap();
    assert!(config.division.enabled);
    assert!(config.decomposition.enabled);
    assert_eq!(config.division.energy_cost.raw(), 2.0);
    assert_eq!(config.decomposition.resources_per_tick.raw(), 1.0);
}
```

- [ ] **Step 2: Run RED/GREEN**

```powershell
cargo test --test phase2_division_smoke parser_loads_phase2d_division_and_decomposition_blocks
```

Expected: FAIL before parser support, PASS after parser support.

---

## Task 13: Documentation And Report

**Files:**

- Modify: `docs/implementation/implementation-phases.md` if Phase 2D status is tracked there
- Modify: `docs/implementation/INDEX.md` only if a new implementation document is created
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2D-division-death-decomposition.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run full verification**

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
python -m pytest .\tools\early-stability
```

Expected:

- Rust tests PASS;
- clippy 0 warnings;
- format check PASS;
- Python early-stability PASS or explicitly report if unrelated environment failure occurs.

- [ ] **Step 2: Create implementation report**

Report must include:

```text
Goal
Implemented files
Division accounting rules actually used
Decomposition placeholder decision
New tests
Verification output summary
Known limitations / deferred work
```

Known limitations to include if still true:

- no Genome copy/mutation;
- no MaterialFragmentStore;
- dead Cell rows are not compacted/removed;
- no Joints;
- no fair proportional uptake allocation.

- [ ] **Step 3: Update worklogs index**

Use the existing project approach for `outputs/worklogs/index.md`. Do not manually invent a partial index if there is a generator/script already used in the repo.

---

## Acceptance Gates

- [ ] `ProcessId::Division` is `ProcessStatus::Now`.
- [ ] Division is disabled by default and explicit when enabled.
- [ ] Division Feasibility rejects disabled process, dead Cell, insufficient Energy, below target radius and pressure too high.
- [ ] Division execution creates exactly two daughter states from one parent state.
- [ ] Energy after division equals parent Energy minus division Energy cost, then partitioned.
- [ ] Resources and Materials are partitioned by explicit ratio/loss, not cloned.
- [ ] Material buckets are partitioned by bucket; no capability is granted through generic redistribution.
- [ ] RuntimeFlags reset for daughters.
- [ ] Daughter placement is deterministic and bounded by solid_wall world limits.
- [ ] Daughters do not divide again in the same Tick they are created.
- [ ] Cell death leaves a physical dead state.
- [ ] Dead Cells do not execute uptake, metabolism, synthesis, growth, displacement or division.
- [ ] Decomposition releases explicit local resources and reduces internal resources/materials.
- [ ] Dead Cell rows are not removed in Phase 2D unless a later explicit compaction plan exists.
- [ ] `RunSummary.metrics` exposes births/divisions/alive/dead/decomposed counts.
- [ ] Rare events are emitted: `CellDivided`, `CellBorn`, `CellDead`, `CellDecomposed` when completed.
- [ ] Deterministic replay passes for division/decomposition scenarios.
- [ ] Phase 1 and Phase 2A-2C tests still pass.
- [ ] No viewer/storage/analytics state affects behavior.

## Deferred To Later Phases

- Full `MaterialFragmentStore`.
- Genome carrier copy, mutation and inheritance.
- Joints and Joint breakage during division.
- Signal or trace propagation.
- OrganismView and component-level lifecycle.
- Cell row compaction/removal with stable-id remapping.
- Fair proportional resource allocation for crowded same-tile uptake.
- Multi-threaded deterministic partitioning.

## Self-Review

- Spec coverage: Phase 2D roadmap requirements map to Tasks 1-12 and Acceptance Gates.
- Placeholder scan: No unresolved placeholder markers.
- Type consistency: New config names are `DivisionConfig` and `DecompositionConfig`; new process remains `ProcessId::Division`; new event names are `CellDivided`, `CellBorn`, `CellDecomposed`; new metrics names use plural counters.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
