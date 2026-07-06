# Phase 1C Resource Interaction Smoke Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add the first deterministic resource interaction smoke: one Cell can sample a local ResourceGrid cell, uptake bounded resource into internal inventory, metabolize it into Energy, and pay mandatory costs without using `passive_energy_income_placeholder`.

**Architecture:** Keep `alife-core` as the source of truth. Phase 1C adds a narrow resource interaction system inside the existing deterministic Tick path, but does not introduce Genome, Process Registry, Feasibility, diffusion, synthesis, division, Joints, signals, or variable cell size. Cell radius remains fixed for the run and is not used as an uptake footprint in Phase 1C; local sampling uses cell center position mapped to one `GridCoord`.

**Tech Stack:** Rust 2024, existing `serde`/`toml` runner adapter, Cargo integration tests, no new dependencies.

---

## Authority And Scope

Phase 1C implements:

```text
position -> ResourceGrid coord mapping
local resource sampling from one configured ResourceGrid layer
capacity-limited resource uptake into Cell internal resource inventory
simple metabolism from internal Resources into Energy
metabolism heat/waste byproduct accounting
survival scenario without passive energy income placeholder
snapshot radius reads stored Cell radius instead of hardcoded 1.0
cleanup of parser `_used_capacity` dead variable
```

Phase 1C must not implement:

```text
variable Cell radius or growth
uptake footprint by radius
Resource diffusion/stencil
multiple internal resource types
Material synthesis/degradation
division/reproduction
Genome Runtime
ActionPlan / Feasibility
Process Registry
Joints
signals/traces
viewer/server
database persistence
```

Important simplification:

```text
All Cells use the configured fixed radius for now.
Phase 1C local sampling maps the Cell center to one ResourceGrid cell.
The radius is exposed correctly in snapshots, but does not affect uptake area yet.
```

---

## Required Reading

Before editing code, read:

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/implementation/phase-1-design.md`
- `docs/implementation/phase-1-data-model.md`
- `docs/implementation/phase-1-module-api.md`
- `docs/implementation/optimization-paths.md`
- `outputs/worklogs/2026-07-02-1730-REPORT-phase-1B-accounting-and-resource-grid.md`

---

## File Structure

Modify:

```text
src/core/config.rs
src/core/cell_store.rs
src/core/resources.rs
src/core/tick.rs
src/core/snapshot.rs
src/runner/config_parser.rs
tests/phase1_accounting.rs
tests/phase1_config_validation.rs
tests/phase1_core_smoke.rs
tests/phase1_determinism.rs
tests/phase1_resource_grid.rs
outputs/worklogs/index.md
```

Create:

```text
tests/phase1_resource_interaction.rs
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1C-resource-interaction-smoke.md
```

Do not modify:

```text
tools/early-stability/*
```

Do not change existing scenario TOML files unless a separate config migration is explicitly approved.

Do not commit automatically unless the user explicitly asks. Use the checklist and final report as the checkpoint record.

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

Record in implementation notes:

```text
baseline cargo test result
baseline cargo fmt --check result
baseline cargo clippy result
active Rust test count
```

Do not edit code in this task.

---

## Task 2: Fix Snapshot Radius Projection

**Files:**

- Modify: `src/core/cell_store.rs`
- Modify: `src/core/snapshot.rs`
- Modify: `tests/phase1_core_smoke.rs`

Reason:

```text
Phase 1 uses fixed configured Cell radius, but snapshots must project the stored configured radius.
Hardcoding 1.0 in snapshots hides config mistakes and will confuse Phase 1C local sampling tests.
```

- [ ] **Step 1: Write failing snapshot radius test**

Append to `tests/phase1_core_smoke.rs`:

```rust
#[test]
fn snapshot_uses_stored_cell_radius_from_config() {
    let mut config = valid_config();
    config.cell.radius = Radius::new(2.0).unwrap();

    let world = WorldState::from_config(config).unwrap();
    let snapshot = CommittedSnapshot::from_world(&world);

    assert_eq!(snapshot.cells[0].radius.raw(), 2.0);
}
```

Make sure these imports exist in `tests/phase1_core_smoke.rs`:

```rust
use alife::core::snapshot::CommittedSnapshot;
use alife::core::world::WorldState;
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_core_smoke snapshot_uses_stored_cell_radius_from_config
```

Expected:

```text
FAIL: snapshot returns hardcoded radius 1.0
```

- [ ] **Step 3: Add CellStore radius accessor**

In `src/core/cell_store.rs`, add:

```rust
pub fn radius(&self, index: CellIndex) -> Radius {
    self.radii[index.raw()]
}
```

- [ ] **Step 4: Use stored radius in snapshots**

In `src/core/snapshot.rs`, replace:

```rust
radius: Radius::new(1.0).expect("Phase 1 radius is validated at init"),
```

with:

```rust
radius: world.cells().radius(index),
```

Remove the now-unused `Radius` import from `src/core/snapshot.rs` if needed.

- [ ] **Step 5: Run test**

Run:

```bash
cargo test --test phase1_core_smoke snapshot_uses_stored_cell_radius_from_config
```

Expected:

```text
test snapshot_uses_stored_cell_radius_from_config ... ok
```

---

## Task 3: Add Resource Interaction Config

**Files:**

- Modify: `src/core/config.rs`
- Modify: existing tests that construct `RuntimeConfig`
- Create: `tests/phase1_resource_interaction.rs`

Design:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceInteractionConfig {
    pub enabled: bool,
    pub uptake_layer_index: usize,
    pub max_uptake_per_tick: ResourceAmount,
    pub metabolism_resource_per_tick: ResourceAmount,
    pub energy_per_resource: f32,
    pub heat_per_resource: f32,
    pub waste_per_resource: f32,
}
```

Rules:

```text
enabled=false preserves Phase 1A/1B behavior.
uptake_layer_index identifies one external ResourceGrid layer by dense layer index.
internal Cell inventory remains one aggregate ResourceAmount in Phase 1C.
energy_per_resource, heat_per_resource and waste_per_resource are non-negative finite conversion rates.
```

- [ ] **Step 1: Write failing config tests**

Create `tests/phase1_resource_interaction.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, ConfigError, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn base_interaction_config(interaction: ResourceInteractionConfig) -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        interaction,
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
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
            stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.25,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap()
}

#[test]
fn resource_interaction_config_disabled_preserves_default_behavior() {
    let interaction = ResourceInteractionConfig::disabled();

    assert!(!interaction.enabled);
    assert_eq!(interaction.uptake_layer_index, 0);
    assert_eq!(interaction.max_uptake_per_tick.raw(), 0.0);
}

#[test]
fn runtime_config_rejects_enabled_interaction_with_missing_resource_layer() {
    let err = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig {
            enabled: true,
            uptake_layer_index: 1,
            max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
            metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
            energy_per_resource: 2.0,
            heat_per_resource: 0.1,
            waste_per_resource: 0.2,
        },
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(5.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::zero(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
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
            stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
            dormancy_allowed: true,
            dormant_mandatory_cost_modifier: 0.25,
            critical_capacity_overrun: CapacityAmount::new(5.0).unwrap(),
        },
    )
    .unwrap_err();

    assert_eq!(err, ConfigError::InvalidResourceInteractionLayer);
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```bash
cargo test --test phase1_resource_interaction resource_interaction_config_disabled_preserves_default_behavior
cargo test --test phase1_resource_interaction runtime_config_rejects_enabled_interaction_with_missing_resource_layer
```

Expected:

```text
FAIL: missing ResourceInteractionConfig and ConfigError variant
```

- [ ] **Step 3: Implement ResourceInteractionConfig**

In `src/core/config.rs`, after `ResourceConfig`, add:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ResourceInteractionConfig {
    pub enabled: bool,
    pub uptake_layer_index: usize,
    pub max_uptake_per_tick: ResourceAmount,
    pub metabolism_resource_per_tick: ResourceAmount,
    pub energy_per_resource: f32,
    pub heat_per_resource: f32,
    pub waste_per_resource: f32,
}

impl ResourceInteractionConfig {
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            uptake_layer_index: 0,
            max_uptake_per_tick: ResourceAmount::zero(),
            metabolism_resource_per_tick: ResourceAmount::zero(),
            energy_per_resource: 0.0,
            heat_per_resource: 0.0,
            waste_per_resource: 0.0,
        }
    }

    pub fn validate(self, resources: &ResourceConfig) -> Result<(), ConfigError> {
        if !self.enabled {
            return Ok(());
        }
        if self.uptake_layer_index >= resources.layer_count() {
            return Err(ConfigError::InvalidResourceInteractionLayer);
        }
        for value in [
            self.energy_per_resource,
            self.heat_per_resource,
            self.waste_per_resource,
        ] {
            if !value.is_finite() || value < 0.0 {
                return Err(ConfigError::InvalidResourceInteractionRate);
            }
        }
        Ok(())
    }
}
```

Add config errors:

```rust
InvalidResourceInteractionLayer,
InvalidResourceInteractionRate,
```

Change `RuntimeConfig`:

```rust
pub struct RuntimeConfig {
    pub world: WorldConfig,
    pub space: SpaceConfig,
    pub resources: ResourceConfig,
    pub resource_interaction: ResourceInteractionConfig,
    pub cell: CellInitialConfig,
    pub environment: EnvironmentConfig,
    pub lifecycle: LifecycleConfig,
}
```

Change `RuntimeConfig::new` signature:

```rust
pub fn new(
    world: WorldConfig,
    space: SpaceConfig,
    resources: ResourceConfig,
    resource_interaction: ResourceInteractionConfig,
    cell: CellInitialConfig,
    environment: EnvironmentConfig,
    lifecycle: LifecycleConfig,
) -> Result<Self, ConfigError>
```

Inside `RuntimeConfig::new`, after spatial/dormancy validation, add:

```rust
resource_interaction.validate(&resources)?;
```

Store `resource_interaction` in `Self`.

Update `config_hash()` to include interaction config:

```rust
for value in [
    self.resource_interaction.enabled as u64,
    self.resource_interaction.uptake_layer_index as u64,
    self.resource_interaction.max_uptake_per_tick.raw().to_bits() as u64,
    self.resource_interaction.metabolism_resource_per_tick.raw().to_bits() as u64,
    self.resource_interaction.energy_per_resource.to_bits() as u64,
    self.resource_interaction.heat_per_resource.to_bits() as u64,
    self.resource_interaction.waste_per_resource.to_bits() as u64,
] {
    hash ^= value;
    hash = hash.wrapping_mul(0x100000001b3);
}
```

- [ ] **Step 4: Update existing `RuntimeConfig::new` calls**

In every test/config fixture, insert:

```rust
ResourceInteractionConfig::disabled(),
```

immediately after `ResourceConfig::new(...).unwrap(),`.

Update imports:

```rust
use alife::core::config::ResourceInteractionConfig;
```

or in core files:

```rust
use crate::core::config::ResourceInteractionConfig;
```

- [ ] **Step 5: Run tests**

Run:

```bash
cargo test --test phase1_resource_interaction resource_interaction_config_disabled_preserves_default_behavior
cargo test --test phase1_resource_interaction runtime_config_rejects_enabled_interaction_with_missing_resource_layer
cargo test
```

Expected:

```text
new config tests pass
all existing tests pass after fixture updates
```

---

## Task 4: Add ResourceGrid Position Mapping

**Files:**

- Modify: `src/core/resources.rs`
- Modify: `tests/phase1_resource_interaction.rs`

Design:

```text
GridCoord = floor(position / spatial_grid_size), clamped to ResourceGrid bounds.
This uses Cell center only. Radius does not affect sampled cells in Phase 1C.
```

- [ ] **Step 1: Write failing mapping tests**

Append to `tests/phase1_resource_interaction.rs`:

```rust
use alife::core::resources::{ResourceGrid, ResourceLayerIndex};
use alife::core::units::GridCoord;

#[test]
fn resource_grid_maps_position_to_clamped_grid_coord() {
    let grid = ResourceGrid::new(
        WorldSize::new(16.0, 16.0).unwrap(),
        8.0,
        vec![ResourceAmount::new(10.0).unwrap()],
        0.0,
    )
    .unwrap();

    assert_eq!(grid.coord_for_position(Position::new(1.0, 1.0)), GridCoord::new(0, 0));
    assert_eq!(grid.coord_for_position(Position::new(8.0, 1.0)), GridCoord::new(1, 0));
    assert_eq!(grid.coord_for_position(Position::new(99.0, 99.0)), GridCoord::new(1, 1));
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_interaction resource_grid_maps_position_to_clamped_grid_coord
```

Expected:

```text
FAIL: missing ResourceGrid::coord_for_position
```

- [ ] **Step 3: Store spatial grid size in ResourceGrid**

In `src/core/resources.rs`, import `Position`:

```rust
use crate::core::units::{GridCoord, Position, ResourceAmount, WorldSize};
```

Add field:

```rust
spatial_grid_size: f32,
```

Set it in `ResourceGrid::new`:

```rust
spatial_grid_size,
```

Add method:

```rust
pub fn coord_for_position(&self, position: Position) -> GridCoord {
    let max_x = self.width.saturating_sub(1);
    let max_y = self.height.saturating_sub(1);
    let x = (position.x() / self.spatial_grid_size).floor().max(0.0) as usize;
    let y = (position.y() / self.spatial_grid_size).floor().max(0.0) as usize;

    GridCoord::new(x.min(max_x), y.min(max_y))
}
```

- [ ] **Step 4: Run mapping test**

Run:

```bash
cargo test --test phase1_resource_interaction resource_grid_maps_position_to_clamped_grid_coord
```

Expected:

```text
test resource_grid_maps_position_to_clamped_grid_coord ... ok
```

---

## Task 5: Add Cell Resource Inventory Mutation Helpers

**Files:**

- Modify: `src/core/cell_store.rs`
- Modify: `tests/phase1_resource_interaction.rs`

Design:

```text
Phase 1C internal resource inventory remains one aggregate ResourceAmount.
Uptake is limited by free capacity.
Metabolism consumes from this aggregate inventory.
```

- [ ] **Step 1: Write failing inventory tests**

Append to `tests/phase1_resource_interaction.rs`:

```rust
use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState};

fn one_cell_store_with_resources(resources: ResourceAmount, capacity_limit: CapacityAmount) -> CellStore {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(20.0).unwrap(),
        ),
        resources,
        materials: MaterialAmount::new(4.0).unwrap(),
        capacity_limit,
        temperature: alife::core::units::Temperature::new(25.0),
    });
    cells
}

#[test]
fn cell_resource_uptake_is_limited_by_free_capacity() {
    let mut cells = one_cell_store_with_resources(
        ResourceAmount::new(4.0).unwrap(),
        CapacityAmount::new(10.0).unwrap(),
    );

    let accepted = cells.add_resources_limited_by_capacity(
        CellIndex::from_raw(0),
        ResourceAmount::new(10.0).unwrap(),
    );

    assert_eq!(accepted.raw(), 2.0);
    assert_eq!(cells.resource_amount(CellIndex::from_raw(0)).raw(), 6.0);
    assert_eq!(cells.used_capacity(CellIndex::from_raw(0)).raw(), 10.0);
}

#[test]
fn cell_resource_consumption_is_limited_by_available_inventory() {
    let mut cells = one_cell_store_with_resources(
        ResourceAmount::new(3.0).unwrap(),
        CapacityAmount::new(10.0).unwrap(),
    );

    let consumed = cells.consume_resources(
        CellIndex::from_raw(0),
        ResourceAmount::new(5.0).unwrap(),
    );

    assert_eq!(consumed.raw(), 3.0);
    assert_eq!(cells.resource_amount(CellIndex::from_raw(0)).raw(), 0.0);
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```bash
cargo test --test phase1_resource_interaction cell_resource_uptake_is_limited_by_free_capacity
cargo test --test phase1_resource_interaction cell_resource_consumption_is_limited_by_available_inventory
```

Expected:

```text
FAIL: missing CellStore resource helper methods
```

- [ ] **Step 3: Implement CellStore helpers**

In `src/core/cell_store.rs`, add public read accessor:

```rust
pub fn resource_amount(&self, index: CellIndex) -> ResourceAmount {
    self.resources[index.raw()]
}
```

Add crate-private mutation helpers:

```rust
pub(crate) fn add_resources_limited_by_capacity(
    &mut self,
    index: CellIndex,
    requested: ResourceAmount,
) -> ResourceAmount {
    let accepted_raw = requested.raw().min(self.free_capacity(index).raw());
    let accepted = ResourceAmount::new(accepted_raw).expect("accepted uptake is clamped");
    self.resources[index.raw()] = self.resources[index.raw()].saturating_add(accepted);
    accepted
}

pub(crate) fn consume_resources(
    &mut self,
    index: CellIndex,
    requested: ResourceAmount,
) -> ResourceAmount {
    let available = self.resources[index.raw()];
    let consumed_raw = requested.raw().min(available.raw());
    let consumed = ResourceAmount::new(consumed_raw).expect("consumed resource is clamped");
    self.resources[index.raw()] = available.saturating_sub(consumed);
    consumed
}
```

- [ ] **Step 4: Run inventory tests**

Run:

```bash
cargo test --test phase1_resource_interaction cell_resource_uptake_is_limited_by_free_capacity
cargo test --test phase1_resource_interaction cell_resource_consumption_is_limited_by_available_inventory
```

Expected:

```text
both inventory tests pass
```

---

## Task 6: Implement Local Uptake From ResourceGrid

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `tests/phase1_resource_interaction.rs`

Tick phase insertion:

```text
for each Cell:
  if resource_interaction.enabled:
    coord = resources.coord_for_position(cell.position)
    available_external = resources.amount_at(layer, coord)
    requested = min(max_uptake_per_tick, available_external)
    accepted = cells.add_resources_limited_by_capacity(index, requested)
    resources.set_amount_at(layer, coord, available_external - accepted)
```

Rules:

```text
Uptake has no Energy cost in Phase 1C.
Uptake does not use cell radius footprint.
Uptake cannot exceed external amount, max uptake, or internal free capacity.
```

- [ ] **Step 1: Write failing local uptake test**

Append to `tests/phase1_resource_interaction.rs`:

```rust
use alife::core::summary::{CollapseReason, SurvivalResult};
use alife::core::tick::TickExecutor;

#[test]
fn tick_uptakes_local_resource_into_cell_inventory() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(3.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::zero(),
        energy_per_resource: 0.0,
        heat_per_resource: 0.0,
        waste_per_resource: 0.0,
    };
    let config = base_interaction_config(interaction);
    let mut executor = TickExecutor::new(config).unwrap();

    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert_eq!(
        executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw(),
        3.0
    );
    assert_eq!(
        executor
            .world()
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        7.0
    );
}
```

This test uses initial energy `5.0`, mandatory cost `2.0`, no passive income and no metabolism. It should stay stable because it can pay from initial Energy while uptake changes resource accounting.

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_interaction tick_uptakes_local_resource_into_cell_inventory
```

Expected:

```text
FAIL: resource inventory/grid do not change
```

- [ ] **Step 3: Add uptake helper in TickExecutor**

In `src/core/tick.rs`, import:

```rust
use crate::core::resources::ResourceLayerIndex;
use crate::core::units::ResourceAmount;
```

Inside the per-cell loop, before Energy availability is calculated, insert:

```rust
if config.resource_interaction.enabled {
    let layer = ResourceLayerIndex::from_raw(config.resource_interaction.uptake_layer_index);
    let coord = self
        .world
        .resources()
        .coord_for_position(self.world.cells().position(index));
    let external_available = self
        .world
        .resources()
        .amount_at(layer, coord)
        .expect("resource interaction layer is config-validated");
    let requested = ResourceAmount::new(
        external_available
            .raw()
            .min(config.resource_interaction.max_uptake_per_tick.raw()),
    )
    .expect("requested uptake is clamped");

    let accepted = {
        let cells = self.world.cells_mut_for_commit();
        cells.add_resources_limited_by_capacity(index, requested)
    };

    let remaining_external = external_available.saturating_sub(accepted);
    self.world
        .resources_mut_for_commit()
        .set_amount_at(layer, coord, remaining_external)
        .expect("resource interaction coord is derived from grid bounds");
}
```

Keep this as a direct Phase 1C system block. Do not introduce Process Registry or Feasibility here.

- [ ] **Step 4: Run uptake test**

Run:

```bash
cargo test --test phase1_resource_interaction tick_uptakes_local_resource_into_cell_inventory
```

Expected:

```text
test tick_uptakes_local_resource_into_cell_inventory ... ok
```

---

## Task 7: Implement Simple Metabolism Into Energy, Heat And Waste

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `tests/phase1_resource_interaction.rs`

Metabolism formula:

```text
consumed = min(cell_internal_resources, metabolism_resource_per_tick)
energy_gain = consumed * energy_per_resource
heat_byproduct = consumed * heat_per_resource
waste_byproduct = consumed * waste_per_resource
```

Rules:

```text
Energy is clamped by EnergyBuffer capacity.
Consumed Resource is removed from internal inventory.
Heat and Waste byproducts are added to normal environment accounting.
Energy production does not create matter; it consumes internal Resource.
```

- [ ] **Step 1: Write failing metabolism test**

Append to `tests/phase1_resource_interaction.rs`:

```rust
#[test]
fn tick_metabolizes_internal_resource_into_energy_heat_and_waste() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(2.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(2.0).unwrap(),
        energy_per_resource: 3.0,
        heat_per_resource: 0.5,
        waste_per_resource: 0.25,
    };
    let mut config = base_interaction_config(interaction);
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert_eq!(
        executor
            .world()
            .cells()
            .energy(CellIndex::from_raw(0))
            .current()
            .raw(),
        5.0
    );
    assert_eq!(
        executor
            .world()
            .cells()
            .resource_amount(CellIndex::from_raw(0))
            .raw(),
        0.0
    );
    assert_eq!(executor.world().environment().heat().raw(), 1.0);
    assert_eq!(executor.world().environment().waste().raw(), 0.5);
}
```

Expected accounting:

```text
initial energy 1
uptake 2 resources
metabolize 2 resources -> +6 energy, +1 heat, +0.5 waste
mandatory cost 2
final energy 5
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_interaction tick_metabolizes_internal_resource_into_energy_heat_and_waste
```

Expected:

```text
FAIL: energy does not increase from resource metabolism
```

- [ ] **Step 3: Add metabolism before mandatory cost**

In `src/core/tick.rs`, inside the per-cell loop after the uptake block and before calculating `available`, add:

```rust
let mut metabolism_heat = 0.0_f32;
let mut metabolism_waste = 0.0_f32;
let mut metabolism_energy = EnergyAmount::zero();

if config.resource_interaction.enabled {
    let consumed = {
        let cells = self.world.cells_mut_for_commit();
        cells.consume_resources(index, config.resource_interaction.metabolism_resource_per_tick)
    };

    metabolism_energy = EnergyAmount::new(
        consumed.raw() * config.resource_interaction.energy_per_resource,
    )
    .expect("metabolism energy is config-validated");
    metabolism_heat = consumed.raw() * config.resource_interaction.heat_per_resource;
    metabolism_waste = consumed.raw() * config.resource_interaction.waste_per_resource;
}
```

Change available energy from:

```rust
let available = current
    .current()
    .saturating_add(config.cell.passive_energy_income);
```

to:

```rust
let available = current
    .current()
    .saturating_add(config.cell.passive_energy_income)
    .saturating_add(metabolism_energy)
    .clamp_max(current.capacity());
```

Accumulate byproducts outside the loop:

1. Before the loop:

```rust
let mut metabolism_heat_total = 0.0_f32;
let mut metabolism_waste_total = 0.0_f32;
```

2. After metabolism calculation inside the loop:

```rust
metabolism_heat_total += metabolism_heat;
metabolism_waste_total += metabolism_waste;
```

3. Move `heat_next` and `waste_next` calculation after the loop, or compute base heat/waste before the loop and add totals after the loop:

```rust
let heat_next = HeatAmount::new(
    (self.world.environment().heat().raw()
        + config.environment.heat_generated_per_tick.raw()
        + metabolism_heat_total
        - config.environment.heat_dissipation_rate.raw())
    .max(0.0),
)
.expect("heat accounting is clamped");

let waste_next = WasteAmount::new(
    (self.world.environment().waste().raw()
        + config.environment.waste_generated_per_tick.raw()
        + metabolism_waste_total
        - config.environment.waste_sink_rate.raw())
    .max(0.0),
)
.expect("waste accounting is clamped");
```

4. Because lifecycle heat/waste thresholds need `heat_next` and `waste_next`, keep the implementation single-threaded and deterministic by doing resource interaction first, calculating `heat_next`/`waste_next`, then running lifecycle. If restructuring is needed, split the current per-cell loop into:

```text
phase A: uptake/metabolism over dense CellIndex range, accumulate byproducts
phase B: compute heat_next/waste_next and warnings
phase C: mandatory cost/lifecycle over dense CellIndex range
```

Do not allocate per-cell delta objects for this.

- [ ] **Step 4: Run metabolism test**

Run:

```bash
cargo test --test phase1_resource_interaction tick_metabolizes_internal_resource_into_energy_heat_and_waste
```

Expected:

```text
test tick_metabolizes_internal_resource_into_energy_heat_and_waste ... ok
```

---

## Task 8: Add Integrated Survival And Collapse Scenarios

**Files:**

- Modify: `tests/phase1_resource_interaction.rs`

- [ ] **Step 1: Write survival-through-resource test**

Append to `tests/phase1_resource_interaction.rs`:

```rust
#[test]
fn cell_survives_from_local_resource_without_passive_income() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
        energy_per_resource: 2.5,
        heat_per_resource: 0.05,
        waste_per_resource: 0.05,
    };
    let mut config = base_interaction_config(interaction);
    config.world.tick_count = Tick::from_raw(5);
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.energy_capacity = EnergyAmount::new(10.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.run_until_configured_tick().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
    assert_eq!(summary.tick.raw(), 5);
}
```

- [ ] **Step 2: Write no-resource collapse test**

Append to `tests/phase1_resource_interaction.rs`:

```rust
#[test]
fn cell_collapses_without_local_resource_or_passive_income() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
        energy_per_resource: 2.5,
        heat_per_resource: 0.05,
        waste_per_resource: 0.05,
    };
    let mut config = base_interaction_config(interaction);
    config.resources.initial_distribution = vec![ResourceAmount::zero()];
    config.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::MandatoryCostUnpaid);
}
```

- [ ] **Step 3: Run integrated tests**

Run:

```bash
cargo test --test phase1_resource_interaction cell_survives_from_local_resource_without_passive_income
cargo test --test phase1_resource_interaction cell_collapses_without_local_resource_or_passive_income
```

Expected:

```text
both integrated resource interaction tests pass
```

---

## Task 9: Update TOML Parser Boundary

**Files:**

- Modify: `src/runner/config_parser.rs`
- Modify: `tests/phase1_resource_interaction.rs`

TOML shape:

```toml
[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 2.5
heat_per_resource = 0.05
waste_per_resource = 0.05
```

If `[resource_interaction]` is missing:

```text
ResourceInteractionConfig::disabled()
```

- [ ] **Step 1: Write failing parser test**

Append to `tests/phase1_resource_interaction.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_maps_resource_interaction_block() {
    let toml = r#"
scenario_id = "resource_interaction"
seed = 42
tick_count = 10

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0
passive_energy_income_placeholder = 0.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 2.5
heat_per_resource = 0.05
waste_per_resource = 0.05

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = { cell_wall = 4.0 }
initial_energy = 1.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.25
capacity_limit = 30.0
minimum_viability_materials = { cell_wall = 1.0 }

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 10.0
heat_death_threshold = 20.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 3.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();

    assert!(config.resource_interaction.enabled);
    assert_eq!(config.resource_interaction.uptake_layer_index, 0);
    assert_eq!(config.resource_interaction.max_uptake_per_tick.raw(), 1.0);
    assert_eq!(config.resource_interaction.metabolism_resource_per_tick.raw(), 1.0);
    assert_eq!(config.resource_interaction.energy_per_resource, 2.5);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_interaction parser_maps_resource_interaction_block
```

Expected:

```text
FAIL: parser does not know resource_interaction
```

- [ ] **Step 3: Add RawResourceInteraction**

In `src/runner/config_parser.rs`, import:

```rust
ResourceInteractionConfig,
```

Add:

```rust
#[derive(Deserialize, Debug)]
pub struct RawResourceInteraction {
    pub enabled: Option<bool>,
    pub uptake_layer_index: Option<usize>,
    pub max_uptake_per_tick: Option<f32>,
    pub metabolism_resource_per_tick: Option<f32>,
    pub energy_per_resource: Option<f32>,
    pub heat_per_resource: Option<f32>,
    pub waste_per_resource: Option<f32>,
}
```

Add optional field to `RawScenarioConfig`:

```rust
pub resource_interaction: Option<RawResourceInteraction>,
```

Map it in `to_runtime_config`:

```rust
let resource_interaction = if let Some(raw_interaction) = self.resource_interaction {
    ResourceInteractionConfig {
        enabled: raw_interaction.enabled.unwrap_or(false),
        uptake_layer_index: raw_interaction.uptake_layer_index.unwrap_or(0),
        max_uptake_per_tick: ResourceAmount::new(
            raw_interaction.max_uptake_per_tick.unwrap_or(0.0),
        )
        .map_err(|e| {
            ParseError::ValidationError(format!("Invalid max_uptake_per_tick: {:?}", e))
        })?,
        metabolism_resource_per_tick: ResourceAmount::new(
            raw_interaction.metabolism_resource_per_tick.unwrap_or(0.0),
        )
        .map_err(|e| {
            ParseError::ValidationError(format!(
                "Invalid metabolism_resource_per_tick: {:?}",
                e
            ))
        })?,
        energy_per_resource: raw_interaction.energy_per_resource.unwrap_or(0.0),
        heat_per_resource: raw_interaction.heat_per_resource.unwrap_or(0.0),
        waste_per_resource: raw_interaction.waste_per_resource.unwrap_or(0.0),
    }
} else {
    ResourceInteractionConfig::disabled()
};
```

Pass it to `RuntimeConfig::new`:

```rust
RuntimeConfig::new(
    world,
    space,
    resources,
    resource_interaction,
    cell,
    environment,
    lifecycle,
)
```

Remove the dead parser variable:

```rust
let _used_capacity = initial_resources_sum + initial_materials_sum;
```

- [ ] **Step 4: Run parser tests**

Run:

```bash
cargo test --test phase1_resource_interaction parser_maps_resource_interaction_block
cargo test --test phase1_config_validation native_toml_parser_loads_valid_scenarios
```

Expected:

```text
new parser test passes
existing scenario parser tests still pass with missing [resource_interaction]
```

---

## Task 10: Preserve Phase 1A/1B Scenario Behavior

**Files:**

- Modify only if verification exposes a regression:
  - `src/core/tick.rs`
  - `src/core/config.rs`
  - `src/runner/config_parser.rs`
  - `tests/phase1_config_validation.rs`

- [ ] **Step 1: Run existing scenario validation**

Run:

```bash
cargo test --test phase1_config_validation
```

Expected:

```text
all existing phase1_config_validation tests pass
```

- [ ] **Step 2: Run scenario dump**

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

If any old scenario changes, first check:

```text
Was resource_interaction disabled by default?
Was passive_energy_income_placeholder preserved for old scenarios?
Did metabolism byproducts accidentally apply while disabled?
Did the Tick phase order change old heat/waste thresholds?
```

Do not update expected scenario results to hide a regression.

---

## Task 11: Deterministic Replay For Resource Interaction

**Files:**

- Modify: `tests/phase1_resource_interaction.rs`

- [ ] **Step 1: Write deterministic interaction test**

Append to `tests/phase1_resource_interaction.rs`:

```rust
#[test]
fn resource_interaction_is_deterministic_for_same_config_and_seed() {
    let interaction = ResourceInteractionConfig {
        enabled: true,
        uptake_layer_index: 0,
        max_uptake_per_tick: ResourceAmount::new(1.0).unwrap(),
        metabolism_resource_per_tick: ResourceAmount::new(1.0).unwrap(),
        energy_per_resource: 2.5,
        heat_per_resource: 0.05,
        waste_per_resource: 0.05,
    };
    let mut config_a = base_interaction_config(interaction);
    config_a.world.tick_count = Tick::from_raw(5);
    config_a.cell.initial_energy = EnergyAmount::new(1.0).unwrap();
    config_a.cell.passive_energy_income = EnergyAmount::zero();

    let config_b = config_a.clone();

    let mut executor_a = TickExecutor::new(config_a).unwrap();
    let mut executor_b = TickExecutor::new(config_b).unwrap();

    let summary_a = executor_a.run_until_configured_tick().unwrap();
    let summary_b = executor_b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a, summary_b);
    assert_eq!(
        executor_a
            .world()
            .resources()
            .quantities()
            .iter()
            .map(|amount| amount.raw())
            .collect::<Vec<_>>(),
        executor_b
            .world()
            .resources()
            .quantities()
            .iter()
            .map(|amount| amount.raw())
            .collect::<Vec<_>>()
    );
}
```

- [ ] **Step 2: Run deterministic test**

Run:

```bash
cargo test --test phase1_resource_interaction resource_interaction_is_deterministic_for_same_config_and_seed
```

Expected:

```text
test resource_interaction_is_deterministic_for_same_config_and_seed ... ok
```

---

## Task 12: Full Verification

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

- [ ] **Step 4: Run Python tool tests only if scenario TOML files or tool assumptions were changed**

Run only if this implementation modifies `tools/early-stability/*` or scenario files:

```bash
python -m pytest .\tools\early-stability
```

Expected:

```text
all early-stability tests pass
```

If Python was not run, write in the report:

```text
Python tool code and scenario files were not changed; Rust Phase 1C behavior was verified with Cargo tests.
```

---

## Task 13: Write Implementation Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1C-resource-interaction-smoke.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Create report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1C-resource-interaction-smoke.md`:

```markdown
# REPORT: Phase 1C Resource Interaction Smoke

## Goal

Implemented deterministic local Resource interaction smoke for Phase 1C.

## Scope

- Fixed snapshot radius projection to use stored Cell radius.
- Added `ResourceInteractionConfig`.
- Added ResourceGrid position-to-grid mapping.
- Added capacity-limited local uptake from external ResourceGrid.
- Added simple metabolism from internal Resource inventory into Energy.
- Added metabolism Heat/Waste byproduct accounting.
- Added survival scenario without `passive_energy_income_placeholder`.
- Preserved Phase 1A/1B scenario behavior.

## Decisions

- Cell radius remains fixed for the run.
- Phase 1C uptake samples only the grid cell under Cell center position.
- Internal Cell Resources remain one aggregate `ResourceAmount`.
- Resource interaction is disabled by default.
- Existing scenarios without `[resource_interaction]` preserve old behavior.
- No Process Registry, Feasibility, Genome, diffusion, growth or division was added.

## Scenario Results

Paste output from:

```text
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
```

## New Resource Interaction Tests

List all tests from:

```text
cargo test --test phase1_resource_interaction
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
Python tool code and scenario files were not changed; Python tests were not required for this Rust-only Phase 1C change.
```

## Open Questions

- Full per-resource internal inventory is still future work.
- Resource diffusion remains out of Phase 1C.
- Uptake by cell radius/footprint remains out of Phase 1C.
- Growth/division remains out of Phase 1C.
```

- [ ] **Step 2: Add report to worklog index**

Add under `## Reports` in `outputs/worklogs/index.md`:

```markdown
- [[outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1C-resource-interaction-smoke|outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1C-resource-interaction-smoke]]
```

Use the actual report timestamp.

---

## Acceptance Gates

Phase 1C is complete only when:

```text
resource interaction is disabled by default
old Phase 1A/1B scenarios produce the same outcomes
snapshot uses stored Cell radius
Cell radius remains fixed and does not affect uptake footprint
Cell center maps deterministically to ResourceGrid coord
local uptake reduces external ResourceGrid amount
local uptake increases internal Cell Resource inventory within free capacity
metabolism consumes internal Resource inventory
metabolism adds Energy, Heat and Waste deterministically
a Cell can survive without passive_energy_income_placeholder when local resource is available
a Cell collapses without passive income and without local resource
same config + seed produces same summary/resource quantities
no Genome/Process/Joints/division/diffusion/growth behavior is added
cargo fmt --check passes
cargo clippy --workspace --all-targets --all-features -- -D warnings passes
cargo test passes
implementation report exists and is linked from outputs/worklogs/index.md
```

---

## Self-Review

Spec coverage:

- Covers the user's warning that Cell size is fixed for now.
- Fixes snapshot hardcoded radius without introducing variable radius behavior.
- Removes parser dead `_used_capacity` during parser boundary work.
- Adds the first survival path through resource use, not placeholder passive income.
- Keeps Phase 1C narrow and deterministic.

Known limits:

- Internal resources are still aggregate, not per ResourceType.
- Uptake uses only one configured ResourceGrid layer.
- No diffusion, Resource capability matrix, process registry, Feasibility or active transport.
- No stability tuner integration in this plan; this is a Rust core behavior slice.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
