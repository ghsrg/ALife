# Phase 2E Material Profile Coverage Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `test-driven-development` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Do not commit unless the user explicitly asks.

**Goal:** Make Phase 2 Materials mechanically meaningful: material profiles and proportions must gate capabilities, change process strength/cost/effect, expose trade-offs, and produce sweep/observer evidence.

**Architecture:** Keep `alife-core` as source of truth. Add a small material mechanics layer that derives numeric capability levels from existing Cell material columns, then use those levels inside registered process feasibility/execution and observer/analyzer projections. This phase must not introduce deep chemistry, Genome behavior, hardcoded Cell types, or observer labels as behavior inputs.

**Tech Stack:** Rust 2024, std-only core, `serde`/`toml`/`serde_json` already in the project, `cargo test`, `cargo clippy`, `cargo run --bin sweep_analyzer`.

---

## Revision Notes After Phase 2D Calibration

This plan remains the single Phase 2E implementation plan, but execution must treat it as staged work. The Phase 2D analyzer calibration performed on 2026-07-10 changed the starting assumptions:

- `decomposition_viability` is now reachability-complete and mechanism-measurable through timing/rate metrics, not through final total released matter.
- `division_viability` is now clean enough to prove division activation and survival under analyzer conditions.
- analyzer scenario presets now support additional calibration fields:
  - `initial_cell_resources`;
  - `division_energy_cost`;
  - `max_uptake_per_tick`;
  - `metabolism_resource_per_tick`.
- analyzer CSV output already includes decomposition timing/remains fields.
- `BALANCE_ERROR` may still appear in division-style analyzer rows. Treat it as a separate accounting-audit signal unless it blocks a material-profile assertion. Do not treat it as a Phase 2E readiness blocker by itself.

Do not copy Phase 2D probe parameters directly into Phase 2E baseline world configs. Phase 2D values were chosen to isolate mechanisms. Phase 2E baseline configs must be neutral enough to expose material-profile effects without hardcoding success.

---

## Phase 2D Readiness Gate

Before starting Task 1, run:

```powershell
cargo test --test phase2_sweep_parser --test phase2_sweep_outputs --test phase2_sweep_warnings
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
```

Required checks in generated raw data:

```text
outputs/raw_data/decomposition_viability.csv:
  warning_codes does not contain LOW_INFORMATION_SWEEP
  death_tick is recorded
  first_decomposition_tick is recorded
  time_to_decomposed changes meaningfully across rates
  decomposition_released_resources_per_tick increases with decomposition_resources_per_tick

outputs/raw_data/division_viability.csv:
  at least one row has survived_to_end = true
  at least one row has divisions_count > 0
  at least one row has births_count > 0
  at least one row has energy_spent_division > 0
  warning_codes does not contain LOW_INFORMATION_SWEEP
```

If `BALANCE_ERROR` appears but the rows satisfy the checks above, continue Phase 2E and record it in the Phase 2E report as a known accounting warning. Stop only if material-profile claims depend on the failed accounting term.

---

## Context And Scope

Read before implementation:

- `docs/PRINCIPLES.md`
- `docs/mechanics/resource-material.md`
- `docs/mechanics/material-process-capability.md`
- `docs/world/materials.md`
- `docs/world/reactions.md`
- `docs/biology/process-capabilities.md`
- `docs/biology/processes.md`
- `docs/biology/feasibility.md`
- `docs/observer/classification-contract.md`
- `docs/observer/classification-registry.md`
- `outputs/worklogs/2026-07-02-1855-PLAN-phase-2-global-roadmap.md`
- `outputs/worklogs/2026-07-04-1405-PLAN-sweep_scenario-eval-coverage_refactor.md`

Phase 2E is allowed to fix core material mechanics. It is not only analyzer tuning.

Out of scope:

- full chemical reaction network;
- Genome Runtime, mutation, inheritance;
- Joints and organism-level behavior;
- pathfinding or learned behavior;
- UI visualization;
- Python `tools/early-stability` changes.

Canonical config root for runtime/analyzer/observer configuration files:

```text
config/
```

The requirement is one root-level config directory, not configuration files scattered through tools or subsystem folders.

---

## Execution Stages

Keep this as one plan, but execute it in four reviewable stages. Do not start a later stage until the previous stage's tests pass and the relevant raw evidence is checked.

### Stage 2E-A: Core Capability Levels

Purpose: replace boolean-only material capability checks with numeric material composition/capability levels.

Includes:

- Task 1: Add Core Material Mechanics Types.
- Task 2: Derive Numeric Capability Levels From CellStore.
- Negative controls for missing material capability.

Exit gate:

```powershell
cargo test --test phase2_material_profile_gating
```

Required evidence:

```text
zero material -> missing capability
non-zero material -> capability available
higher material amount -> higher numeric capability level
existing Phase 2A-D capability tests still pass
```

### Stage 2E-B: Process Effects And Trade-Offs

Purpose: make material proportions change process rate, cost, capacity, or placeholder metrics without creating matter/Energy.

Includes:

- Task 3: Add Material Effect Config.
- Task 4: Apply Material Strength To Feasibility And Execution.
- Task 5: Add Minimal Repair And Sensory Placeholder Effects.

Exit gate:

```powershell
cargo test --test phase2_material_profile_gating --test phase2_material_profile_effects
```

Required evidence:

```text
transport-rich increases resource_absorbed
metabolic-rich increases energy_produced
storage-rich increases capacity or scarcity survival with a cost/trade-off
structural-rich changes growth/radius/stability
contractile-rich changes deterministic displacement only when process is enabled
sensory-rich changes sensed input metrics but does not create command behavior
repair-rich is either a minimal placeholder effect or explicit tool_limited
```

### Stage 2E-C: Canonical Configs And Baseline Envelope

Purpose: create reusable root `config/` artifacts for material profiles and a deterministic 10+ Cell baseline scenario.

Includes:

- Task 6: Add Material Profile Configs.
- Task 9: Add 10+ Cell Deterministic Material Profile Scenario.

Baseline envelope rules:

```text
resource inflow must permit survival without guaranteeing runaway growth
growth/division may be possible but must not be required for every profile
heat/waste/decomposition must not mask the targeted material effect
baseline must not reuse Phase 2D probe values blindly
all compared profiles must run under equal non-material conditions
```

Exit gate:

```powershell
cargo test --test phase2_material_profile_effects material_profile_baseline_is_deterministic_for_10_plus_cells
```

Required evidence:

```text
10+ Cell material-profile scenario is deterministic
profiles differ only in material profile inputs unless a test states otherwise
baseline does not encode hardcoded cell roles or success scripts
```

### Stage 2E-D: Analyzer, Observer, And Phase Gate

Purpose: expose material-profile coverage through analyzer CSV/report artifacts and observer-derived labels.

Includes:

- Task 7: Extend Sweep Analyzer For Material Profile Coverage.
- Task 8: Expose Material Profile Evidence To Observer.
- Task 10: Update Docs And Worklog Report.

Exit gate:

```powershell
cargo run --bin sweep_analyzer -- config/analyzer/material_profile_sweeps.toml
cargo test --workspace --all-targets
```

Required evidence:

```text
outputs/raw_data/material_profile_summary.csv exists
outputs/raw_data/material_profile_coverage.csv exists
outputs/reports/material-profile-coverage-<timestamp>.md exists
observer labels are derived from material effects, not behavior inputs
warning codes distinguish inactive mechanics from expected placeholder/tool_limited mechanics
```

---

## File Structure

Create:

- `src/core/materials.rs` — material slots, profile ids, capability level derivation, material effect config.
- `tests/phase2_material_profile_gating.rs` — capability gating and negative controls.
- `tests/phase2_material_profile_effects.rs` — directional effects from material proportions.
- `tests/phase2_material_profile_analyzer.rs` — sweep analyzer profile output/coverage contract.
- `config/material_profiles/phase2_profiles.toml` — canonical material profile definitions.
- `config/scenarios/material_profiles/material_profile_baseline.toml` — 10+ Cell baseline scenario.
- `config/scenarios/material_profiles/material_profile_negative_controls.toml` — missing-capability controls.
- `config/analyzer/material_profile_sweeps.toml` — Phase 2E sweep definitions.
- `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2E-material-profile-coverage.md` — implementation report after verification.

Modify:

- `src/core/mod.rs` — export `materials`.
- `src/core/config.rs` — add `MaterialEffectConfig` to `RuntimeConfig`; include it in `config_hash`.
- `src/core/cell_store.rs` — expose material slot amounts/fractions and numeric capability levels.
- `src/core/process.rs` — add capability-level helpers and optional `RepairPlaceholder` / `SensoryInput` process ids if needed for metrics.
- `src/core/tick.rs` — apply material-level modifiers to uptake, metabolism, storage/capacity, growth, contractility, and sensory placeholder metrics.
- `src/core/summary.rs` — add material-aware metrics to `MetricsSummary`/diagnostics.
- `src/core/world.rs` — ensure `validate_feasibility` uses numeric material capability levels.
- `src/runner/config_parser.rs` — parse `[material_effects]`, material profiles, and root `config/` paths.
- `src/bin/sweep_analyzer.rs` — add material profile sweeps, coverage CSV/report, warning codes, root `config/` default paths.
- `src/observer/projection.rs` — expose material effect features for classifiers.
- `src/observer/classifiers.rs` — map observed role labels from material-aware evidence.
- `config/observer/*.toml` — keep observer configuration files loadable from the root config directory.
- `outputs/worklogs/index.md` — link the new plan/report.

Do not modify:

- `tools/early-stability/**` unless a Rust test proves a broken project-wide path assumption. Python is no longer the source of truth for Phase 2 behavior.

---

## Material Profiles Required

The implementation must support these profile ids:

```text
balanced_baseline
boundary_rich
transport_rich
metabolic_rich
storage_rich
structural_rich
repair_rich
contractile_rich
sensory_rich
weak_cell
mixed_specialized
```

Each profile row in config must include:

```toml
[[profile]]
id = "transport_rich"
expected_observer_role = "transport-like"
expected_mechanics = ["local_resource_uptake"]
expected_benefit = ["higher_resource_absorbed"]
expected_cost = ["higher_capacity_pressure"]

[profile.materials]
boundary = 0.5
transport = 3.0
metabolic = 0.5
storage = 0.5
synthesis = 0.5
structural = 0.5
repair = 0.25
contractile = 0.25
sensory = 0.25
```

---

## Task 1: Add Core Material Mechanics Types

**Files:**

- Create: `src/core/materials.rs`
- Modify: `src/core/mod.rs`
- Test: `tests/phase2_material_profile_gating.rs`

- [ ] **Step 1: Write failing tests for material slot totals and fractions**

Add to `tests/phase2_material_profile_gating.rs`:

```rust
use alife::core::materials::{MaterialComposition, MaterialSlot};
use alife::core::units::MaterialAmount;

#[test]
fn material_composition_computes_total_and_fraction_deterministically() {
    let composition = MaterialComposition::from_amounts([
        (MaterialSlot::Boundary, MaterialAmount::new(1.0).unwrap()),
        (MaterialSlot::Transport, MaterialAmount::new(3.0).unwrap()),
        (MaterialSlot::Metabolic, MaterialAmount::new(2.0).unwrap()),
    ]);

    assert_eq!(composition.total().raw(), 6.0);
    assert_eq!(composition.amount(MaterialSlot::Transport).raw(), 3.0);
    assert!((composition.fraction(MaterialSlot::Transport) - 0.5).abs() < 0.0001);
    assert_eq!(composition.fraction(MaterialSlot::Sensory), 0.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_material_profile_gating material_composition_computes_total_and_fraction_deterministically
```

Expected: FAIL because `src/core/materials.rs` does not exist.

- [ ] **Step 3: Implement `src/core/materials.rs`**

Create:

```rust
use crate::core::process::MaterialCapability;
use crate::core::units::MaterialAmount;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MaterialSlot {
    Boundary,
    Transport,
    Metabolic,
    Storage,
    Synthesis,
    Structural,
    Repair,
    Contractile,
    Sensory,
}

impl MaterialSlot {
    pub const ALL: [MaterialSlot; 9] = [
        MaterialSlot::Boundary,
        MaterialSlot::Transport,
        MaterialSlot::Metabolic,
        MaterialSlot::Storage,
        MaterialSlot::Synthesis,
        MaterialSlot::Structural,
        MaterialSlot::Repair,
        MaterialSlot::Contractile,
        MaterialSlot::Sensory,
    ];

    pub const fn as_str(self) -> &'static str {
        match self {
            MaterialSlot::Boundary => "boundary",
            MaterialSlot::Transport => "transport",
            MaterialSlot::Metabolic => "metabolic",
            MaterialSlot::Storage => "storage",
            MaterialSlot::Synthesis => "synthesis",
            MaterialSlot::Structural => "structural",
            MaterialSlot::Repair => "repair",
            MaterialSlot::Contractile => "contractile",
            MaterialSlot::Sensory => "sensory",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialComposition {
    amounts: [MaterialAmount; 9],
}

impl MaterialComposition {
    pub fn zero() -> Self {
        Self {
            amounts: [MaterialAmount::zero(); 9],
        }
    }

    pub fn from_amounts<const N: usize>(pairs: [(MaterialSlot, MaterialAmount); N]) -> Self {
        let mut composition = Self::zero();
        for (slot, amount) in pairs {
            composition.amounts[slot as usize] = amount;
        }
        composition
    }

    pub fn amount(self, slot: MaterialSlot) -> MaterialAmount {
        self.amounts[slot as usize]
    }

    pub fn total(self) -> MaterialAmount {
        let total = self.amounts.iter().map(|a| a.raw()).sum::<f32>();
        MaterialAmount::new_unchecked(total)
    }

    pub fn fraction(self, slot: MaterialSlot) -> f32 {
        let total = self.total().raw();
        if total > 0.0 {
            self.amount(slot).raw() / total
        } else {
            0.0
        }
    }

    pub fn capability_level(self, capability: MaterialCapability) -> f32 {
        let amount = match capability {
            MaterialCapability::BoundaryPermeability => self.amount(MaterialSlot::Boundary),
            MaterialCapability::ResourceUptake => self.amount(MaterialSlot::Transport),
            MaterialCapability::Metabolism => self.amount(MaterialSlot::Metabolic),
            MaterialCapability::StorageCapacity => self.amount(MaterialSlot::Storage),
            MaterialCapability::MaterialSynthesis => self.amount(MaterialSlot::Synthesis),
            MaterialCapability::StructuralGrowth => self.amount(MaterialSlot::Structural),
            MaterialCapability::Repair => self.amount(MaterialSlot::Repair),
            MaterialCapability::Contractility => self.amount(MaterialSlot::Contractile),
            MaterialCapability::ResourceSensing
            | MaterialCapability::PressureSensing
            | MaterialCapability::DamageSensing => self.amount(MaterialSlot::Sensory),
        };
        amount.raw()
    }
}
```

Modify `src/core/mod.rs`:

```rust
pub mod materials;
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cargo test --test phase2_material_profile_gating material_composition_computes_total_and_fraction_deterministically
```

Expected: PASS.

---

## Task 2: Derive Numeric Capability Levels From CellStore

**Files:**

- Modify: `src/core/cell_store.rs`
- Test: `tests/phase2_material_profile_gating.rs`

- [ ] **Step 1: Write failing capability-level tests**

Add:

```rust
use alife::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
use alife::core::materials::MaterialSlot;
use alife::core::process::MaterialCapability;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

fn insert_test_cell_with_materials(transport: f32, metabolic: f32) -> (CellStore, alife::core::cell_store::CellIndex) {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(EnergyAmount::new(10.0).unwrap(), EnergyAmount::new(20.0).unwrap()),
        resources: ResourceAmount::new(5.0).unwrap(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::new(transport).unwrap(),
        metabolic_material: MaterialAmount::new(metabolic).unwrap(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::zero(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(30.0).unwrap(),
        temperature: Temperature::new(0.0).unwrap(),
    });
    (cells, alife::core::cell_store::CellIndex::from_raw(0))
}

#[test]
fn cell_store_exposes_material_composition_and_capability_level() {
    let (cells, idx) = insert_test_cell_with_materials(3.0, 2.0);

    assert_eq!(cells.material_amount_for_slot(idx, MaterialSlot::Transport).raw(), 3.0);
    assert_eq!(cells.capability_level(idx, MaterialCapability::ResourceUptake), 3.0);
    assert!(cells.has_capability(idx, MaterialCapability::ResourceUptake));
}

#[test]
fn zero_material_means_missing_capability() {
    let (cells, idx) = insert_test_cell_with_materials(0.0, 2.0);

    assert_eq!(cells.capability_level(idx, MaterialCapability::ResourceUptake), 0.0);
    assert!(!cells.has_capability(idx, MaterialCapability::ResourceUptake));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_material_profile_gating cell_store_exposes_material_composition_and_capability_level
```

Expected: FAIL because `material_amount_for_slot` and `capability_level` do not exist.

- [ ] **Step 3: Implement CellStore helpers**

Add imports and methods to `src/core/cell_store.rs`:

```rust
use crate::core::materials::{MaterialComposition, MaterialSlot};
```

Add methods:

```rust
pub fn material_amount_for_slot(
    &self,
    index: CellIndex,
    slot: MaterialSlot,
) -> MaterialAmount {
    match slot {
        MaterialSlot::Boundary => self.boundary_material(index),
        MaterialSlot::Transport => self.transport_material(index),
        MaterialSlot::Metabolic => self.metabolic_material(index),
        MaterialSlot::Storage => self.storage_material(index),
        MaterialSlot::Synthesis => self.synthesis_material(index),
        MaterialSlot::Structural => self.structural_material(index),
        MaterialSlot::Repair => self.repair_material(index),
        MaterialSlot::Contractile => self.contractile_material(index),
        MaterialSlot::Sensory => self.sensory_material(index),
    }
}

pub fn material_composition(&self, index: CellIndex) -> MaterialComposition {
    MaterialComposition::from_amounts([
        (MaterialSlot::Boundary, self.boundary_material(index)),
        (MaterialSlot::Transport, self.transport_material(index)),
        (MaterialSlot::Metabolic, self.metabolic_material(index)),
        (MaterialSlot::Storage, self.storage_material(index)),
        (MaterialSlot::Synthesis, self.synthesis_material(index)),
        (MaterialSlot::Structural, self.structural_material(index)),
        (MaterialSlot::Repair, self.repair_material(index)),
        (MaterialSlot::Contractile, self.contractile_material(index)),
        (MaterialSlot::Sensory, self.sensory_material(index)),
    ])
}

pub fn capability_level(
    &self,
    index: CellIndex,
    capability: crate::core::process::MaterialCapability,
) -> f32 {
    if self.lifecycle_state(index) == LifecycleState::Dead {
        return 0.0;
    }
    self.material_composition(index).capability_level(capability)
}
```

Update `has_capability` to use `capability_level(index, capability) > 0.0`.

- [ ] **Step 4: Run tests**

Run:

```powershell
cargo test --test phase2_material_profile_gating
```

Expected: PASS.

---

## Task 3: Add Material Effect Config And Hashing

**Files:**

- Modify: `src/core/config.rs`
- Modify: `src/runner/config_parser.rs`
- Test: `tests/phase2_material_profile_effects.rs`

- [ ] **Step 1: Write failing parser/config test**

Create `tests/phase2_material_profile_effects.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parses_material_effects_config() {
    let toml = r#"
scenario_id = "material_effects_parse"
seed = 1
tick_count = 10

[world]
size = [64.0, 64.0]

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [100.0]

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[material_effects]
transport_uptake_per_unit = 0.5
metabolic_conversion_per_unit = 0.5
storage_capacity_per_unit = 2.0
structural_growth_per_unit = 0.25
contractile_force_per_unit = 0.1
sensory_input_per_unit = 0.2
boundary_retention_per_unit = 0.1
repair_stress_buffer_per_unit = 0.1

[cell]
initial_position = [8.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 0.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 1.0
capacity_limit = 20.0

[environment]
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
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 100.0
"#;

    let cfg = RawScenarioConfig::parse(toml).expect("config should parse");
    assert_eq!(cfg.material_effects.transport_uptake_per_unit, 0.5);
    assert_eq!(cfg.material_effects.storage_capacity_per_unit, 2.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_material_profile_effects parses_material_effects_config
```

Expected: FAIL because `material_effects` is not parsed.

- [ ] **Step 3: Implement `MaterialEffectConfig`**

In `src/core/config.rs` add:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MaterialEffectConfig {
    pub transport_uptake_per_unit: f32,
    pub metabolic_conversion_per_unit: f32,
    pub storage_capacity_per_unit: f32,
    pub structural_growth_per_unit: f32,
    pub contractile_force_per_unit: f32,
    pub sensory_input_per_unit: f32,
    pub boundary_retention_per_unit: f32,
    pub repair_stress_buffer_per_unit: f32,
}

impl Default for MaterialEffectConfig {
    fn default() -> Self {
        Self {
            transport_uptake_per_unit: 1.0,
            metabolic_conversion_per_unit: 1.0,
            storage_capacity_per_unit: 1.0,
            structural_growth_per_unit: 1.0,
            contractile_force_per_unit: 1.0,
            sensory_input_per_unit: 1.0,
            boundary_retention_per_unit: 0.0,
            repair_stress_buffer_per_unit: 0.0,
        }
    }
}
```

Add `pub material_effects: MaterialEffectConfig` to `RuntimeConfig`, default it in `RuntimeConfig::new`, and add every field to `config_hash()`.

In `src/runner/config_parser.rs` add:

```rust
#[derive(Deserialize, Debug)]
pub struct RawMaterialEffects {
    pub transport_uptake_per_unit: Option<f32>,
    pub metabolic_conversion_per_unit: Option<f32>,
    pub storage_capacity_per_unit: Option<f32>,
    pub structural_growth_per_unit: Option<f32>,
    pub contractile_force_per_unit: Option<f32>,
    pub sensory_input_per_unit: Option<f32>,
    pub boundary_retention_per_unit: Option<f32>,
    pub repair_stress_buffer_per_unit: Option<f32>,
}
```

Add `pub material_effects: Option<RawMaterialEffects>` to `RawScenarioConfig`.

After creating `runtime_config`, set:

```rust
if let Some(raw_effects) = self.material_effects {
    runtime_config.material_effects = MaterialEffectConfig {
        transport_uptake_per_unit: raw_effects.transport_uptake_per_unit.unwrap_or(1.0),
        metabolic_conversion_per_unit: raw_effects.metabolic_conversion_per_unit.unwrap_or(1.0),
        storage_capacity_per_unit: raw_effects.storage_capacity_per_unit.unwrap_or(1.0),
        structural_growth_per_unit: raw_effects.structural_growth_per_unit.unwrap_or(1.0),
        contractile_force_per_unit: raw_effects.contractile_force_per_unit.unwrap_or(1.0),
        sensory_input_per_unit: raw_effects.sensory_input_per_unit.unwrap_or(1.0),
        boundary_retention_per_unit: raw_effects.boundary_retention_per_unit.unwrap_or(0.0),
        repair_stress_buffer_per_unit: raw_effects.repair_stress_buffer_per_unit.unwrap_or(0.0),
    };
}
```

Reject non-finite or negative material effect values with `ParseError::ValidationError`.

- [ ] **Step 4: Run tests**

Run:

```powershell
cargo test --test phase2_material_profile_effects parses_material_effects_config
```

Expected: PASS.

---

## Task 4: Apply Material Strength To Feasibility And Execution

**Files:**

- Modify: `src/core/world.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Test: `tests/phase2_material_profile_gating.rs`
- Test: `tests/phase2_material_profile_effects.rs`

- [ ] **Step 1: Write failing negative-control tests**

Add tests:

```rust
use alife::core::process::{ActionCandidate, ProcessId, RejectionReason, FeasibilityResult};
use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

fn config_with_materials(transport: f32, metabolic: f32, storage: f32, structural: f32, contractile: f32, sensory: f32) -> alife::core::config::RuntimeConfig {
    let text = format!(r#"
scenario_id = "material_profile_test"
seed = 1
tick_count = 5
[world]
size = [64.0, 64.0]
[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2
[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [100.0]
[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 10.0
metabolism_resource_per_tick = 10.0
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0
[material_effects]
transport_uptake_per_unit = 1.0
metabolic_conversion_per_unit = 1.0
storage_capacity_per_unit = 2.0
structural_growth_per_unit = 1.0
contractile_force_per_unit = 1.0
sensory_input_per_unit = 1.0
[cell]
initial_position = [8.0, 8.0]
radius = 1.0
initial_resources = {{ nutrient = 5.0 }}
initial_materials = {{ boundary = 1.0, transport = {transport}, metabolic = {metabolic}, storage = {storage}, synthesis = 1.0, structural = {structural}, repair = 0.0, contractile = {contractile}, sensory = {sensory} }}
initial_energy = 10.0
energy_capacity = 50.0
mandatory_cost_per_tick = 0.0
capacity_limit = 50.0
[environment]
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
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 100.0
[growth]
growth_cost_resource = 1.0
growth_cost_energy = 0.0
growth_target_radius = 2.0
max_division_pressure = 10.0
"#);
    RawScenarioConfig::parse(&text).unwrap()
}

#[test]
fn missing_transport_rejects_active_uptake() {
    let cfg = config_with_materials(0.0, 1.0, 1.0, 1.0, 0.0, 0.0);
    let exec = TickExecutor::new(cfg).unwrap();
    let idx = alife::core::cell_store::CellIndex::from_raw(0);
    let result = exec.world().validate_feasibility(
        idx,
        &ActionCandidate { process_id: ProcessId::LocalResourceUptake, requested_amount: 1.0 },
    );
    assert_eq!(result, FeasibilityResult::Rejected(RejectionReason::MissingCapability(alife::core::process::MaterialCapability::ResourceUptake)));
}

#[test]
fn higher_transport_material_increases_absorbed_resource() {
    let low = {
        let mut exec = TickExecutor::new(config_with_materials(0.5, 1.0, 1.0, 1.0, 0.0, 0.0)).unwrap();
        exec.run_until_configured_tick().unwrap().metrics.final_internal_resources
    };
    let high = {
        let mut exec = TickExecutor::new(config_with_materials(3.0, 1.0, 1.0, 1.0, 0.0, 0.0)).unwrap();
        exec.run_until_configured_tick().unwrap().metrics.final_internal_resources
    };
    assert!(high > low, "transport-rich profile should absorb more resource");
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test --test phase2_material_profile_gating --test phase2_material_profile_effects
```

Expected: at least the directional effect test FAILS because current code treats any non-zero material as a boolean gate.

- [ ] **Step 3: Implement material modifiers**

Rules to implement:

```text
uptake_limit = max_uptake_per_tick * transport_level * transport_uptake_per_unit
metabolism_limit = metabolism_resource_per_tick * metabolic_level * metabolic_conversion_per_unit
effective_capacity_limit = base_capacity_limit + storage_level * storage_capacity_per_unit
growth_amount = base_growth_amount * structural_level * structural_growth_per_unit
contractile_displacement = base_force * contractile_level * contractile_force_per_unit
sensory_input_metric = local_resource_or_pressure_input * sensory_level * sensory_input_per_unit
```

Do not let any modifier create matter or Energy for free. Modifiers may only bound rate, efficiency, capacity, cost, or observer metrics.

Implementation notes:

- In `validate_feasibility`, `MissingCapability` remains when capability level is `0.0`.
- In execution, use `accepted_amount` returned from `FeasibilityResult::Allowed`; do not re-read raw config constants after Feasibility.
- If a process is not fully implemented, emit a metric/diagnostic with explicit `tool_limited` or `placeholder_effect` naming.

- [ ] **Step 4: Run focused tests**

Run:

```powershell
cargo test phase2_material_profile
```

Expected: PASS.

---

## Task 5: Add Minimal Repair And Sensory Placeholder Effects

**Files:**

- Modify: `src/core/process.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Modify: `src/observer/projection.rs`
- Test: `tests/phase2_material_profile_effects.rs`

- [ ] **Step 1: Write failing tests**

Add:

```rust
#[test]
fn sensory_material_changes_sensed_input_metric_without_command_behavior() {
    let low = {
        let mut exec = TickExecutor::new(config_with_materials(1.0, 1.0, 1.0, 1.0, 0.0, 0.0)).unwrap();
        exec.run_until_configured_tick().unwrap().metrics.sensory_input_accumulated
    };
    let high = {
        let mut exec = TickExecutor::new(config_with_materials(1.0, 1.0, 1.0, 1.0, 0.0, 3.0)).unwrap();
        exec.run_until_configured_tick().unwrap().metrics.sensory_input_accumulated
    };
    assert!(high > low);
}

#[test]
fn repair_material_has_stress_buffer_placeholder_cost_or_status() {
    let cfg = config_with_materials(1.0, 1.0, 1.0, 1.0, 0.0, 0.0);
    let mut exec = TickExecutor::new(cfg).unwrap();
    let summary = exec.run_until_configured_tick().unwrap();
    assert!(summary.metrics.repair_placeholder_available || summary.diagnostics.tool_limited_mechanisms.contains(&"repair".to_string()));
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```powershell
cargo test --test phase2_material_profile_effects sensory_material_changes_sensed_input_metric_without_command_behavior repair_material_has_stress_buffer_placeholder_cost_or_status
```

Expected: FAIL because the metrics do not exist.

- [ ] **Step 3: Implement metrics-only placeholders**

Add to `MetricsSummary`:

```rust
pub sensory_input_accumulated: f32,
pub repair_placeholder_available: bool,
```

Add to `ProcessDiagnostics`:

```rust
pub tool_limited_mechanisms: Vec<String>,
```

For sensory:

```text
local sensed input = sensory_level * sensory_input_per_unit * local_resource_amount_or_contact_pressure
```

This is observer/debug input only in Phase 2E. It must not choose movement, uptake, or growth.

For repair:

```text
If repair material exists but no damage/repair process is enabled:
  mark repair_placeholder_available = true
  add tool_limited "repair" only if no stress-buffer placeholder is configured
```

Do not invent HP. Do not heal lifecycle state for free.

- [ ] **Step 4: Run focused tests**

Run:

```powershell
cargo test --test phase2_material_profile_effects
```

Expected: PASS.

---

## Task 6: Create Canonical Material Profile Configs Under `config/`

**Files:**

- Create: `config/material_profiles/phase2_profiles.toml`
- Create: `config/scenarios/material_profiles/material_profile_baseline.toml`
- Create: `config/scenarios/material_profiles/material_profile_negative_controls.toml`
- Create: `config/analyzer/material_profile_sweeps.toml`
- Modify: `src/runner/config_parser.rs`
- Test: `tests/phase2_material_profile_analyzer.rs`

- [ ] **Step 1: Write failing parser/path test**

Create `tests/phase2_material_profile_analyzer.rs`:

```rust
#[test]
fn material_profile_files_exist_under_config_root() {
    assert!(std::path::Path::new("config/material_profiles/phase2_profiles.toml").exists());
    assert!(std::path::Path::new("config/scenarios/material_profiles/material_profile_baseline.toml").exists());
    assert!(std::path::Path::new("config/analyzer/material_profile_sweeps.toml").exists());
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```powershell
cargo test --test phase2_material_profile_analyzer material_profile_files_exist_under_config_root
```

Expected: FAIL because files do not exist.

- [ ] **Step 3: Create material profile config**

Create `config/material_profiles/phase2_profiles.toml` with the 11 required profiles. Use the shape shown in the "Material Profiles Required" section.

Profile amounts must intentionally create trade-offs:

```text
rich profile:
  dominant material = 3.0
  secondary support = 1.0
  unrelated materials = 0.25..0.5

weak_cell:
  all materials low = 0.1

mixed_specialized:
  transport + metabolic + storage high
  structural/contractile lower
```

- [ ] **Step 4: Create scenario files**

Create baseline with 10+ cells:

```toml
scenario_id = "material_profile_baseline"
seed = 42
tick_count = 250

[world]
size = [128.0, 128.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 4

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [500.0]
optional_decay_rate = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 2.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 2.0
heat_per_resource = 0.05
waste_per_resource = 0.05

[material_effects]
transport_uptake_per_unit = 0.5
metabolic_conversion_per_unit = 0.5
storage_capacity_per_unit = 2.0
structural_growth_per_unit = 0.25
contractile_force_per_unit = 0.1
sensory_input_per_unit = 0.2
boundary_retention_per_unit = 0.05
repair_stress_buffer_per_unit = 0.05

[cell]
initial_position = [8.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [8.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [16.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [24.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [32.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [40.0, 8.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [8.0, 16.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [24.0, 16.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [32.0, 16.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0

[[cells]]
initial_position = [40.0, 16.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0, sensory = 1.0 }
initial_energy = 15.0
energy_capacity = 40.0
mandatory_cost_per_tick = 1.0
capacity_limit = 40.0
```

This fixture intentionally keeps radius identical for every Cell in Phase 2E.

- [ ] **Step 5: Run parser tests**

Run:

```powershell
cargo test --test phase2_material_profile_analyzer material_profile_files_exist_under_config_root
```

Expected: PASS.

---

## Task 7: Extend Sweep Analyzer For Material Profile Coverage

**Files:**

- Modify: `src/bin/sweep_analyzer.rs`
- Test: `tests/phase2_material_profile_analyzer.rs`

- [ ] **Step 1: Write failing analyzer output test**

Add:

```rust
use std::process::Command;

#[test]
fn sweep_analyzer_writes_material_profile_outputs() {
    let output = Command::new("cargo")
        .args(["run", "--quiet", "--bin", "sweep_analyzer", "--", "config/analyzer/material_profile_sweeps.toml"])
        .output()
        .expect("sweep analyzer should run");

    assert!(output.status.success(), "stderr: {}", String::from_utf8_lossy(&output.stderr));
    assert!(std::path::Path::new("outputs/raw_data/material_profile_summary.csv").exists());
    assert!(std::path::Path::new("outputs/raw_data/material_profile_coverage.csv").exists());
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```powershell
cargo test --test phase2_material_profile_analyzer sweep_analyzer_writes_material_profile_outputs -- --nocapture
```

Expected: FAIL because analyzer does not write material profile files.

- [ ] **Step 3: Implement analyzer support**

Add material-profile output rows with these columns:

```text
run_id
scenario_id
profile_id
seed
tick_count
material_boundary
material_transport
material_metabolic
material_storage
material_synthesis
material_structural
material_repair
material_contractile
material_sensory
activated_processes
expected_processes
missing_expected_processes
uptake_executed
metabolism_executed
growth_executed
contractile_executed
sensory_input_accumulated
energy_produced
resource_absorbed
resource_metabolized
heat_generated
waste_generated
capacity_used
capacity_free
survival_result
collapse_reason
observed_role
expected_role
warning_codes
```

Add coverage rows:

```text
material_id
profile_id
expected_capability
activation_scenario
negative_control
directional_effect_test
observed_role_test
status
warning_codes
```

Add warning codes:

```rust
const MATERIAL_PROFILE_WARNINGS: &[&str] = &[
    "MATERIAL_PROFILE_NOT_ACTIVATED",
    "EXPECTED_CAPABILITY_MISSING",
    "PROCESS_NOT_GATED_BY_MATERIAL",
    "MATERIAL_PROPORTION_HAS_NO_EFFECT",
    "OBSERVER_ROLE_MISMATCH",
    "SCENARIO_TOO_EASY",
    "SCENARIO_TOO_HARD",
    "PASSIVE_INCOME_DOMINATED",
    "ENVIRONMENT_DOMINATED_RESULT",
    "LOW_INFORMATION_MATERIAL_SWEEP",
];
```

Default analyzer path should become:

```text
config/analyzer/sweep_analyzer.toml
```

Do not introduce a parallel plural config root.

- [ ] **Step 4: Run analyzer output test**

Run:

```powershell
cargo test --test phase2_material_profile_analyzer sweep_analyzer_writes_material_profile_outputs -- --nocapture
```

Expected: PASS.

---

## Task 8: Observer Role Sanity For Material Profiles

**Files:**

- Modify: `src/observer/projection.rs`
- Modify: `src/observer/classifiers.rs`
- Modify: `config/observer/cell-functional-role-classifier.toml`
- Test: `tests/phase2_observer_role_classifier.rs`
- Test: `tests/phase2_material_profile_analyzer.rs`

- [ ] **Step 1: Write failing observer sanity test**

Add:

```rust
#[test]
fn transport_rich_profile_maps_to_transport_like_as_observer_only_label() {
    let mut raw = std::collections::HashMap::new();
    raw.insert("transport_material".to_string(), 3.0);
    raw.insert("metabolic_material".to_string(), 0.5);
    raw.insert("boundary_material".to_string(), 0.5);
    raw.insert("storage_material".to_string(), 0.5);
    raw.insert("synthesis_material".to_string(), 0.5);
    raw.insert("structural_material".to_string(), 0.5);
    raw.insert("repair_material".to_string(), 0.25);
    raw.insert("contractile_material".to_string(), 0.25);
    raw.insert("sensory_material".to_string(), 0.25);
    raw.insert("total_materials".to_string(), 6.25);
    raw.insert("ActiveUptake_executed".to_string(), 5.0);

    let window = alife::observer::projection::extract_features(
        "test",
        alife::observer::projection::EntityType::Cell,
        "cell-1",
        0,
        100,
        &raw,
    );
    let cfg = alife::observer::config::load_cell_role_classifier(
        "config/observer/cell-functional-role-classifier.toml",
    )
    .unwrap();
    let result = alife::observer::classifiers::classify_cell_roles_observed(&window, &cfg);

    assert_eq!(result.primary_label.as_deref(), Some("transport-like"));
}
```

- [ ] **Step 2: Run test to verify failure if config/path/label is missing**

Run:

```powershell
cargo test --test phase2_observer_role_classifier transport_rich_profile_maps_to_transport_like_as_observer_only_label
```

Expected: FAIL if classifier/config lacks this mapping.

- [ ] **Step 3: Update observer classification**

Ensure these observer-only labels can be produced:

```text
boundary-supporting
transport-like
metabolic-like
storage-like
structural-like
repair-focused
contractile-like
sensory-like
mixed-function
undifferentiated
insufficient-data
```

Do not let any core behavior read classification output.

- [ ] **Step 4: Run observer tests**

Run:

```powershell
cargo test --test phase2_observer_role_classifier --test phase2_observer_behavior_classifier
```

Expected: PASS.

---

## Task 9: Add Integration Scenario With 10+ Cells And Deterministic Replay

**Files:**

- Test: `tests/phase2_material_profile_effects.rs`
- Config: `config/scenarios/material_profiles/material_profile_baseline.toml`

- [ ] **Step 1: Write failing deterministic replay test**

Add:

```rust
#[test]
fn material_profile_baseline_is_deterministic_for_10_plus_cells() {
    let text = std::fs::read_to_string("config/scenarios/material_profiles/material_profile_baseline.toml").unwrap();
    let cfg_a = RawScenarioConfig::parse(&text).unwrap();
    let cfg_b = RawScenarioConfig::parse(&text).unwrap();

    assert!(cfg_a.initial_cells.len() >= 10);

    let mut a = TickExecutor::new(cfg_a).unwrap();
    let mut b = TickExecutor::new(cfg_b).unwrap();

    let summary_a = a.run_until_configured_tick().unwrap();
    let summary_b = b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a.config_hash, summary_b.config_hash);
    assert_eq!(summary_a.survival_result, summary_b.survival_result);
    assert_eq!(summary_a.collapse_reason, summary_b.collapse_reason);
    assert_eq!(summary_a.metrics.alive_cells_count, summary_b.metrics.alive_cells_count);
    assert_eq!(summary_a.metrics.final_energy, summary_b.metrics.final_energy);
}
```

- [ ] **Step 2: Run test**

Run:

```powershell
cargo test --test phase2_material_profile_effects material_profile_baseline_is_deterministic_for_10_plus_cells
```

Expected: PASS after config/parser paths are correct.

---

## Task 10: Update Docs And Worklog Report

**Files:**

- Modify: `docs/implementation/implementation-phases.md`
- Modify: `docs/implementation/INDEX.md`
- Modify: `docs/config/INDEX.md`
- Modify: `outputs/worklogs/index.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2E-material-profile-coverage.md`

- [ ] **Step 1: Update implementation docs**

Add Phase 2E summary:

```text
Phase 2E validates and implements Material profile mechanics before local Cell-Cell interaction primitives.
It verifies capability gating, proportion effects, trade-offs, observer labels and sweep analyzer coverage.
```

Add links to:

```text
[[outputs/worklogs/2026-07-09-1950-PLAN-phase-2E-material-profile-coverage]]
[[docs/mechanics/material-process-capability]]
[[docs/mechanics/resource-material]]
[[docs/observer/classification-registry]]
```

- [ ] **Step 2: Create report after implementation**

Report must include:

```text
what Material mechanics were changed in core
which profiles were added
which directional effects pass
which mechanics remain tool_limited/deferred
which analyzer artifacts were produced
warning codes observed
Phase 2D readiness gate evidence used before implementation
whether BALANCE_ERROR appeared and whether it affected material-profile claims
baseline envelope values and why they are not copied from Phase 2D probe scenarios
test commands and results
whether Phase 2F can start
```

- [ ] **Step 3: Update worklog index**

Add links under Plans and Reports.

---

## Verification

Run:

```powershell
cargo test --test phase2_sweep_parser --test phase2_sweep_outputs --test phase2_sweep_warnings
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test
cargo run --bin sweep_analyzer -- config/analyzer/material_profile_sweeps.toml
```

Expected:

```text
Phase 2D readiness gate passes
all tests pass
clippy has zero warnings
outputs/raw_data/material_profile_summary.csv exists
outputs/raw_data/material_profile_coverage.csv exists
outputs/reports/material-profile-coverage-<timestamp>.md exists
```

Do not run Python `tools/early-stability` as Phase 2E authority.

---

## Acceptance Gates

Phase 2E is complete only when:

```text
Phase 2D decomposition_viability remains mechanism-measurable
Phase 2D division_viability remains clean enough to prove division survival and non-zero division energy cost
every current Material has activation scenario and negative control
no transport material rejects/reduces active uptake
transport-rich increases resource_absorbed under equal conditions
no metabolic material rejects/reduces metabolism
metabolic-rich increases energy_produced under equal conditions
storage-rich increases capacity or scarcity survival and has a cost/trade-off
structural-rich changes growth/radius/stability under equal conditions
contractile-rich changes deterministic displacement if process is enabled
sensory-rich changes sensed input metric but does not create command behavior
repair-rich has a documented placeholder effect or explicit tool_limited status
analyzer detects inactive/missing material mechanics
observer labels match profiles only as derived roles
10+ Cell material-profile scenario is deterministic
material-profile baseline uses a neutral balance envelope and does not blindly copy analyzer probe configs
no hardcoded CellType/species/organ labels are introduced
Phase 1 and Phase 2A-D tests still pass
```

---

## Self-Review

Spec coverage:

- Material profile list is covered in config and analyzer tasks.
- Capability gating is covered in Tasks 2 and 4.
- Proportion effects are covered in Tasks 3 and 4.
- Sweep analyzer artifacts and warnings are covered in Task 7.
- Observer roles are covered in Task 8.
- Root `config/` placement is covered in Tasks 6 and 7.
- 10+ Cell deterministic scenario is covered in Task 9.
- Phase 2D readiness is covered by the preflight gate and verification commands.
- Staged execution is covered by Stage 2E-A through Stage 2E-D while preserving this as one plan.
- Baseline-world caution is covered by the Stage 2E-C baseline envelope rules.

Known deferred items are explicit and do not block Phase 2E.
