# Phase 1B Accounting And ResourceGrid Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Harden Phase 1B accounting by replacing the one-layer `ResourceGrid` placeholder with a minimal deterministic flat grid, separating external Resources from internal Cell inventory, and preserving the current Energy/Heat/Waste/Capacity behavior.

**Architecture:** Keep `alife-core` as the behavior source of truth and `runner` as the TOML adapter. `ResourceGrid` becomes a flat, indexed, deterministic storage boundary with layer metadata; no diffusion, uptake, metabolism, Genome, Process Registry, Joints, or Phase 2 behavior is introduced. Tests drive each change and must keep current scenario outcomes stable.

**Tech Stack:** Rust 2024, existing `serde`/`toml` runner adapter, standard Cargo tests, no new dependencies.

---

## Authority And Scope

Phase 1B must implement:

```text
ResourceConfig in core config
flat ResourceGrid storage
deterministic grid dimensions from world size and spatial_grid_size
external ResourceGrid initialized from [resources].initial_distribution
internal Cell resource/material inventory remains separate
resource decay applies to every grid cell/layer
capacity accounting remains based on internal Cell Resources + Materials
heat/waste accounting behavior remains unchanged
scenario TOML parser maps resources into ResourceConfig
```

Phase 1B must not implement:

```text
Resource uptake/export
Resource diffusion/stencil
Energy production from Resources
Material synthesis/degradation
division
Genome Runtime
ActionPlan / Feasibility
Joints
signals
viewer/server
database persistence
```

Python `early-stability` remains an estimator/tuner only. Rust Phase 1 behavior is authoritative for implemented Phase 1 features.

---

## Required Reading

Before editing code, read:

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/implementation/phase-1-design.md`
- `docs/implementation/phase-1-data-model.md`
- `docs/implementation/phase-1-module-api.md`
- `docs/implementation/optimization-paths.md`
- `outputs/worklogs/2026-07-02-1640-REPORT-over-capacity-resolution-hardening.md`

---

## File Structure

Modify:

```text
src/core/units.rs
src/core/config.rs
src/core/resources.rs
src/core/world.rs
src/core/tick.rs
src/core/snapshot.rs
src/runner/config_parser.rs
tests/phase1_accounting.rs
tests/phase1_core_smoke.rs
tests/phase1_config_validation.rs
tests/phase1_determinism.rs
outputs/worklogs/README.md
```

Create:

```text
tests/phase1_resource_grid.rs
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1B-accounting-and-resource-grid.md
```

Do not modify:

```text
tools/early-stability/*
```

Do not change scenario TOML files unless a separate user-approved config migration plan exists.

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
baseline cargo test: pass/fail
baseline cargo fmt --check: pass/fail
baseline cargo clippy: pass/fail
current active Rust test count
```

Do not edit code in this task.

---

## Task 2: Add GridCoord Value Object

**Files:**

- Modify: `src/core/units.rs`
- Create: `tests/phase1_resource_grid.rs`

- [ ] **Step 1: Write failing GridCoord tests**

Create `tests/phase1_resource_grid.rs`:

```rust
use alife::core::units::GridCoord;

#[test]
fn grid_coord_preserves_xy_indices() {
    let coord = GridCoord::new(3, 5);

    assert_eq!(coord.x(), 3);
    assert_eq!(coord.y(), 5);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_grid grid_coord_preserves_xy_indices
```

Expected:

```text
FAIL: unresolved import `GridCoord`
```

- [ ] **Step 3: Implement GridCoord**

Append to `src/core/units.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GridCoord {
    x: usize,
    y: usize,
}

impl GridCoord {
    pub const fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    pub const fn x(self) -> usize {
        self.x
    }

    pub const fn y(self) -> usize {
        self.y
    }
}
```

- [ ] **Step 4: Run test**

Run:

```bash
cargo test --test phase1_resource_grid grid_coord_preserves_xy_indices
```

Expected:

```text
test grid_coord_preserves_xy_indices ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/units.rs tests/phase1_resource_grid.rs
git commit -m "feat: add grid coordinate value object"
```

---

## Task 3: Add ResourceConfig To Core RuntimeConfig

**Files:**

- Modify: `src/core/config.rs`
- Modify: `tests/phase1_core_smoke.rs`
- Modify: `tests/phase1_accounting.rs`
- Modify: `tests/phase1_config_validation.rs`
- Modify: `tests/phase1_determinism.rs`

- [ ] **Step 1: Write failing ResourceConfig validation test**

Append to `tests/phase1_resource_grid.rs`:

```rust
use alife::core::config::{ConfigError, ResourceConfig};
use alife::core::units::ResourceAmount;

#[test]
fn resource_config_rejects_empty_initial_distribution() {
    let err = ResourceConfig::new(Vec::new(), 0.0).unwrap_err();

    assert_eq!(err, ConfigError::EmptyResourceDistribution);
}

#[test]
fn resource_config_rejects_invalid_decay_rate() {
    let err = ResourceConfig::new(vec![ResourceAmount::new(1.0).unwrap()], 1.5).unwrap_err();

    assert_eq!(err, ConfigError::InvalidDecayRate);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_grid resource_config_rejects
```

Expected:

```text
FAIL: missing ResourceConfig or ConfigError variants
```

- [ ] **Step 3: Implement ResourceConfig**

Modify `src/core/config.rs`:

1. Add `ResourceConfig` after `SpaceConfig`:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ResourceConfig {
    pub initial_distribution: Vec<ResourceAmount>,
    pub optional_decay_rate: f32,
}

impl ResourceConfig {
    pub fn new(
        initial_distribution: Vec<ResourceAmount>,
        optional_decay_rate: f32,
    ) -> Result<Self, ConfigError> {
        if initial_distribution.is_empty() {
            return Err(ConfigError::EmptyResourceDistribution);
        }
        if !optional_decay_rate.is_finite() || !(0.0..=1.0).contains(&optional_decay_rate) {
            return Err(ConfigError::InvalidDecayRate);
        }

        Ok(Self {
            initial_distribution,
            optional_decay_rate,
        })
    }

    pub fn layer_count(&self) -> usize {
        self.initial_distribution.len()
    }
}
```

2. Change `RuntimeConfig` to include resources and stop deriving `Copy`:

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeConfig {
    pub world: WorldConfig,
    pub space: SpaceConfig,
    pub resources: ResourceConfig,
    pub cell: CellInitialConfig,
    pub environment: EnvironmentConfig,
    pub lifecycle: LifecycleConfig,
}
```

3. Add `EmptyResourceDistribution` to `ConfigError`:

```rust
EmptyResourceDistribution,
```

4. Change `RuntimeConfig::new` signature:

```rust
pub fn new(
    world: WorldConfig,
    space: SpaceConfig,
    resources: ResourceConfig,
    cell: CellInitialConfig,
    environment: EnvironmentConfig,
    lifecycle: LifecycleConfig,
) -> Result<Self, ConfigError>
```

5. Remove `world.optional_decay_rate` from `WorldConfig` and all `InvalidDecayRate` validation in `RuntimeConfig::new`. `ResourceConfig::new` owns decay validation.

6. Store `resources` in `RuntimeConfig`.

7. Update `config_hash()` to include resource distribution and decay:

```rust
for amount in &self.resources.initial_distribution {
    hash ^= amount.raw().to_bits() as u64;
    hash = hash.wrapping_mul(0x100000001b3);
}
hash ^= self.resources.optional_decay_rate.to_bits() as u64;
hash = hash.wrapping_mul(0x100000001b3);
```

- [ ] **Step 4: Update test fixtures to pass ResourceConfig**

In every `RuntimeConfig::new(...)` call, insert:

```rust
ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.01).unwrap(),
```

For fixtures with custom decay, use the intended decay value:

```rust
ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.1).unwrap(),
```

Update imports in tests:

```rust
use alife::core::config::ResourceConfig;
```

Remove `optional_decay_rate` fields from `WorldConfig` literals.

- [ ] **Step 5: Run tests**

Run:

```bash
cargo test --test phase1_resource_grid resource_config_rejects
cargo test
```

Expected:

```text
resource_config_rejects_empty_initial_distribution ... ok
resource_config_rejects_invalid_decay_rate ... ok
all existing tests pass after fixture updates
```

- [ ] **Step 6: Commit**

```bash
git add src/core/config.rs tests
git commit -m "feat: add resource config to runtime config"
```

---

## Task 4: Implement Flat ResourceGrid

**Files:**

- Modify: `src/core/resources.rs`
- Modify: `tests/phase1_resource_grid.rs`

- [ ] **Step 1: Write failing flat-grid tests**

Append to `tests/phase1_resource_grid.rs`:

```rust
use alife::core::resources::{ResourceGrid, ResourceGridError, ResourceLayerIndex};
use alife::core::units::{GridCoord, WorldSize};

#[test]
fn resource_grid_builds_flat_layers_from_config() {
    let grid = ResourceGrid::new(
        WorldSize::new(16.0, 16.0).unwrap(),
        8.0,
        vec![
            ResourceAmount::new(10.0).unwrap(),
            ResourceAmount::new(5.0).unwrap(),
        ],
        0.1,
    )
    .unwrap();

    assert_eq!(grid.width(), 2);
    assert_eq!(grid.height(), 2);
    assert_eq!(grid.layer_count(), 2);
    assert_eq!(grid.cell_count(), 4);
    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(1, 1)).unwrap().raw(),
        10.0
    );
    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(1), GridCoord::new(0, 0)).unwrap().raw(),
        5.0
    );
}

#[test]
fn resource_grid_rejects_out_of_bounds_access() {
    let grid = ResourceGrid::new(
        WorldSize::new(16.0, 16.0).unwrap(),
        8.0,
        vec![ResourceAmount::new(10.0).unwrap()],
        0.0,
    )
    .unwrap();

    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(2, 0)).unwrap_err(),
        ResourceGridError::GridCoordOutOfBounds
    );
    assert_eq!(
        grid.amount_at(ResourceLayerIndex::from_raw(1), GridCoord::new(0, 0)).unwrap_err(),
        ResourceGridError::LayerOutOfBounds
    );
}
```

- [ ] **Step 2: Run tests to verify failure**

Run:

```bash
cargo test --test phase1_resource_grid resource_grid_builds resource_grid_rejects
```

Expected:

```text
FAIL: missing ResourceLayerIndex / ResourceGrid::new signature / amount_at
```

- [ ] **Step 3: Replace ResourceGrid implementation**

Replace `src/core/resources.rs` with:

```rust
use crate::core::units::{GridCoord, ResourceAmount, WorldSize};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceLayerIndex(usize);

impl ResourceLayerIndex {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ResourceGridError {
    EmptyInitialDistribution,
    InvalidGridSize,
    InvalidDecayRate,
    GridCoordOutOfBounds,
    LayerOutOfBounds,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceGrid {
    width: usize,
    height: usize,
    layer_count: usize,
    quantities: Vec<ResourceAmount>,
    optional_decay_rate: f32,
}

impl ResourceGrid {
    pub fn new(
        world_size: WorldSize,
        spatial_grid_size: f32,
        initial_distribution: Vec<ResourceAmount>,
        decay_rate: f32,
    ) -> Result<Self, ResourceGridError> {
        if initial_distribution.is_empty() {
            return Err(ResourceGridError::EmptyInitialDistribution);
        }
        if !spatial_grid_size.is_finite() || spatial_grid_size <= 0.0 {
            return Err(ResourceGridError::InvalidGridSize);
        }
        if !decay_rate.is_finite() || !(0.0..=1.0).contains(&decay_rate) {
            return Err(ResourceGridError::InvalidDecayRate);
        }

        let width = (world_size.width() / spatial_grid_size).ceil().max(1.0) as usize;
        let height = (world_size.height() / spatial_grid_size).ceil().max(1.0) as usize;
        let layer_count = initial_distribution.len();
        let cell_count = width * height;
        let mut quantities = Vec::with_capacity(layer_count * cell_count);

        for amount in initial_distribution {
            for _ in 0..cell_count {
                quantities.push(amount);
            }
        }

        Ok(Self {
            width,
            height,
            layer_count,
            quantities,
            optional_decay_rate: decay_rate,
        })
    }

    pub const fn width(&self) -> usize {
        self.width
    }

    pub const fn height(&self) -> usize {
        self.height
    }

    pub const fn layer_count(&self) -> usize {
        self.layer_count
    }

    pub const fn cell_count(&self) -> usize {
        self.width * self.height
    }

    pub fn amount_at(
        &self,
        layer: ResourceLayerIndex,
        coord: GridCoord,
    ) -> Result<ResourceAmount, ResourceGridError> {
        let index = self.index(layer, coord)?;
        Ok(self.quantities[index])
    }

    pub fn set_amount_at(
        &mut self,
        layer: ResourceLayerIndex,
        coord: GridCoord,
        amount: ResourceAmount,
    ) -> Result<(), ResourceGridError> {
        let index = self.index(layer, coord)?;
        self.quantities[index] = amount;
        Ok(())
    }

    pub fn total_amount_for_layer(
        &self,
        layer: ResourceLayerIndex,
    ) -> Result<ResourceAmount, ResourceGridError> {
        if layer.raw() >= self.layer_count {
            return Err(ResourceGridError::LayerOutOfBounds);
        }
        let start = layer.raw() * self.cell_count();
        let end = start + self.cell_count();
        let total: f32 = self.quantities[start..end].iter().map(|amount| amount.raw()).sum();
        ResourceAmount::new(total).map_err(|_| ResourceGridError::InvalidGridSize)
    }

    pub fn decay_or_passive_update(&mut self) {
        for amount in &mut self.quantities {
            let next_value = (amount.raw() * (1.0 - self.optional_decay_rate)).max(0.0);
            *amount = ResourceAmount::new(next_value).unwrap_or_else(|_| ResourceAmount::zero());
        }
    }

    pub fn quantities(&self) -> &[ResourceAmount] {
        &self.quantities
    }

    fn index(
        &self,
        layer: ResourceLayerIndex,
        coord: GridCoord,
    ) -> Result<usize, ResourceGridError> {
        if layer.raw() >= self.layer_count {
            return Err(ResourceGridError::LayerOutOfBounds);
        }
        if coord.x() >= self.width || coord.y() >= self.height {
            return Err(ResourceGridError::GridCoordOutOfBounds);
        }

        Ok(layer.raw() * self.cell_count() + coord.y() * self.width + coord.x())
    }
}
```

- [ ] **Step 4: Run flat-grid tests**

Run:

```bash
cargo test --test phase1_resource_grid resource_grid_builds resource_grid_rejects
```

Expected:

```text
resource_grid_builds_flat_layers_from_config ... ok
resource_grid_rejects_out_of_bounds_access ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/resources.rs tests/phase1_resource_grid.rs
git commit -m "feat: replace resource placeholder with flat grid"
```

---

## Task 5: Wire ResourceGrid Into WorldState

**Files:**

- Modify: `src/core/world.rs`
- Modify: `tests/phase1_core_smoke.rs`
- Modify: `tests/phase1_accounting.rs`

- [ ] **Step 1: Write failing WorldState ResourceGrid test**

Append to `tests/phase1_resource_grid.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig, RuntimeConfig,
    SpaceConfig, WorldConfig,
};
use alife::core::world::WorldState;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, Seed, Tick,
    WasteAmount,
};

fn grid_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(
            vec![
                ResourceAmount::new(10.0).unwrap(),
                ResourceAmount::new(5.0).unwrap(),
            ],
            0.1,
        )
        .unwrap(),
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(10.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::new(2.0).unwrap(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
            initial_material_amount: MaterialAmount::new(4.0).unwrap(),
        },
        EnvironmentConfig {
            heat_current: HeatAmount::zero(),
            heat_generated_per_tick: HeatAmount::new(0.1).unwrap(),
            heat_dissipation_rate: HeatAmount::new(0.2).unwrap(),
            heat_warning_threshold: HeatAmount::new(10.0).unwrap(),
            heat_death_threshold: HeatAmount::new(20.0).unwrap(),
            waste_current: WasteAmount::zero(),
            waste_generated_per_tick: WasteAmount::new(0.1).unwrap(),
            waste_sink_rate: WasteAmount::new(0.2).unwrap(),
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
fn world_initializes_resource_grid_from_resource_config_not_cell_inventory() {
    let world = WorldState::from_config(grid_config()).unwrap();

    assert_eq!(world.resources().width(), 2);
    assert_eq!(world.resources().height(), 2);
    assert_eq!(world.resources().layer_count(), 2);
    assert_eq!(
        world
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        10.0
    );
    assert_eq!(
        world
            .resources()
            .amount_at(ResourceLayerIndex::from_raw(1), GridCoord::new(0, 0))
            .unwrap()
            .raw(),
        5.0
    );
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_grid world_initializes_resource_grid
```

Expected:

```text
FAIL until WorldState uses ResourceConfig instead of CellInitialConfig.initial_resource_amount
```

- [ ] **Step 3: Update WorldState initialization**

In `src/core/world.rs`, replace ResourceGrid construction:

```rust
resources: ResourceGrid::new(
    config.world.size,
    config.space.spatial_grid_size,
    config.resources.initial_distribution.clone(),
    config.resources.optional_decay_rate,
)
.map_err(|_| WorldInitError::InvalidInitialState)?,
```

Remove use of `config.world.optional_decay_rate`.

- [ ] **Step 4: Update old tests that used `layers()`**

In `tests/phase1_accounting.rs`, replace:

```rust
executor.world().resources().layers()[0].raw()
```

with:

```rust
executor
    .world()
    .resources()
    .amount_at(ResourceLayerIndex::from_raw(0), GridCoord::new(0, 0))
    .unwrap()
    .raw()
```

Import:

```rust
use alife::core::resources::ResourceLayerIndex;
use alife::core::units::GridCoord;
```

- [ ] **Step 5: Run tests**

Run:

```bash
cargo test --test phase1_resource_grid world_initializes_resource_grid
cargo test --test phase1_accounting tick_executor_decays_resource_grid
cargo test
```

Expected:

```text
world_initializes_resource_grid_from_resource_config_not_cell_inventory ... ok
tick_executor_decays_resource_grid ... ok
all tests pass
```

- [ ] **Step 6: Commit**

```bash
git add src/core/world.rs tests/phase1_resource_grid.rs tests/phase1_accounting.rs
git commit -m "feat: initialize world resource grid from resource config"
```

---

## Task 6: Update Runner TOML Parser Resource Mapping

**Files:**

- Modify: `src/runner/config_parser.rs`
- Modify: `tests/phase1_config_validation.rs`
- Modify: `tests/phase1_resource_grid.rs`

- [ ] **Step 1: Write failing parser resource mapping test**

Append to `tests/phase1_resource_grid.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_maps_initial_distribution_to_resource_config_layers() {
    let toml = r#"
scenario_id = "resource_mapping"
seed = 42
tick_count = 10

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["water", "nutrient"]
initial_distribution = [10.0, 5.0]
optional_decay_rate = 0.1
passive_energy_income_placeholder = 2.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = { water = 2.0, nutrient = 1.0 }
initial_materials = { cell_wall = 5.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.1
capacity_limit = 30.0
minimum_viability_materials = { cell_wall = 1.0 }

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.1
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_current = 0.0
waste_generated_per_tick = 0.05
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();

    assert_eq!(config.resources.layer_count(), 2);
    assert_eq!(config.resources.initial_distribution[0].raw(), 10.0);
    assert_eq!(config.resources.initial_distribution[1].raw(), 5.0);
    assert_eq!(config.cell.initial_resource_amount.raw(), 3.0);
}
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_grid parser_maps_initial_distribution
```

Expected:

```text
FAIL until parser exposes resources.initial_distribution as ResourceConfig
```

- [ ] **Step 3: Update RawResources and parser mapping**

In `src/runner/config_parser.rs`, change `RawResources`:

```rust
#[derive(Deserialize, Debug)]
pub struct RawResources {
    pub resource_type_ids: Vec<String>,
    pub initial_distribution: Vec<f32>,
    pub optional_decay_rate: Option<f32>,
    pub passive_energy_income_placeholder: Option<f32>,
}
```

Import `ResourceConfig`:

```rust
use crate::core::config::{
    CellInitialConfig, ConfigError, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    RuntimeConfig, SpaceConfig, WorldConfig,
};
```

Before constructing `WorldConfig`, add:

```rust
if self.resources.resource_type_ids.len() != self.resources.initial_distribution.len() {
    return Err(ParseError::ValidationError(
        "resource_type_ids length must match initial_distribution length".to_string(),
    ));
}

let resource_amounts = self
    .resources
    .initial_distribution
    .iter()
    .map(|value| {
        ResourceAmount::new(*value).map_err(|e| {
            ParseError::ValidationError(format!("Invalid resource initial_distribution: {:?}", e))
        })
    })
    .collect::<Result<Vec<_>, _>>()?;
```

Create `resources`:

```rust
let resources = ResourceConfig::new(resource_amounts, optional_decay_rate)
    .map_err(ParseError::ConfigValidationError)?;
```

Call `RuntimeConfig::new` with `resources`:

```rust
RuntimeConfig::new(world, space, resources, cell, environment, lifecycle)
    .map_err(ParseError::ConfigValidationError)
```

- [ ] **Step 4: Run parser test**

Run:

```bash
cargo test --test phase1_resource_grid parser_maps_initial_distribution
cargo test --test phase1_config_validation native_toml_parser_loads_valid_scenarios
```

Expected:

```text
parser_maps_initial_distribution_to_resource_config_layers ... ok
native_toml_parser_loads_valid_scenarios ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/runner/config_parser.rs tests/phase1_resource_grid.rs tests/phase1_config_validation.rs
git commit -m "feat: map scenario resources into resource config"
```

---

## Task 7: Preserve Phase 1 Scenario Behavior

**Files:**

- Modify only if tests expose regression:
  - `src/core/tick.rs`
  - `src/core/resources.rs`
  - `tests/phase1_config_validation.rs`

- [ ] **Step 1: Run scenario validation**

Run:

```bash
cargo test --test phase1_config_validation
```

Expected:

```text
current_survival_config_is_stable_in_rust ... ok
current_starvation_config_collapses_in_rust ... ok
current_dormancy_config_reaches_dormancy_then_depletes_energy_in_rust ... ok
current_heat_death_config_collapses_in_rust ... ok
current_waste_death_config_collapses_in_rust ... ok
current_over_capacity_config_collapses_in_rust ... ok
native_toml_parser_loads_valid_scenarios ... ok
native_toml_parser_loads_over_capacity_scenario ... ok
parsed_over_capacity_toml_collapses_in_runtime ... ok
```

- [ ] **Step 2: Run result dump**

Run:

```bash
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
```

Expected output includes:

```text
single_cell_survival,Stable,None,100
single_cell_starvation,Collapse,MandatoryCostUnpaid,1
single_cell_dormancy
single_cell_heat_death,Collapse,HeatLimitExceeded,3
single_cell_waste_death,Collapse,WasteLimitExceeded,3
single_cell_over_capacity,Collapse,CapacityExceeded,1
```

Record full output for final report.

- [ ] **Step 3: Fix only real regressions**

If scenario behavior changes, classify before editing:

```text
ResourceGrid-only refactor should not change Energy/Heat/Waste/Capacity outcomes.
Parser mapping can change external ResourceGrid values but must not change internal Cell inventory.
```

Do not adjust expected values to hide regression.

- [ ] **Step 4: Commit if changes were required**

```bash
git add src/core/tick.rs src/core/resources.rs tests/phase1_config_validation.rs
git commit -m "fix: preserve phase 1 scenario behavior after resource grid refactor"
```

If no changes were needed, skip commit and record that scenario behavior stayed stable.

---

## Task 8: Snapshot Resource Summary

**Files:**

- Modify: `src/core/snapshot.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing snapshot resource summary test**

Append to `tests/phase1_resource_grid.rs`:

```rust
use alife::core::snapshot::CommittedSnapshot;

#[test]
fn snapshot_contains_resource_layer_totals() {
    let world = WorldState::from_config(grid_config()).unwrap();
    let snapshot = CommittedSnapshot::from_world(&world);

    assert_eq!(snapshot.resource_layer_totals.len(), 2);
    assert_eq!(snapshot.resource_layer_totals[0].raw(), 40.0);
    assert_eq!(snapshot.resource_layer_totals[1].raw(), 20.0);
}
```

Explanation of expected values:

```text
world 16 x 16
grid size 8
grid cells = 2 x 2 = 4
layer 0: 10 per grid cell -> total 40
layer 1: 5 per grid cell -> total 20
```

- [ ] **Step 2: Run test to verify failure**

Run:

```bash
cargo test --test phase1_resource_grid snapshot_contains_resource_layer_totals
```

Expected:

```text
FAIL: CommittedSnapshot has no resource_layer_totals
```

- [ ] **Step 3: Implement snapshot resource totals**

In `src/core/snapshot.rs`, add to `CommittedSnapshot`:

```rust
pub resource_layer_totals: Vec<ResourceAmount>,
```

Import:

```rust
use crate::core::resources::ResourceLayerIndex;
use crate::core::units::ResourceAmount;
```

Inside `CommittedSnapshot::from_world`, compute:

```rust
let resource_layer_totals = (0..world.resources().layer_count())
    .map(|layer| {
        world
            .resources()
            .total_amount_for_layer(ResourceLayerIndex::from_raw(layer))
            .expect("layer range is derived from layer_count")
    })
    .collect();
```

Then set:

```rust
resource_layer_totals,
```

Do not add mutation APIs to snapshot or viewer frame.

- [ ] **Step 4: Run snapshot test**

Run:

```bash
cargo test --test phase1_resource_grid snapshot_contains_resource_layer_totals
cargo test snapshot_and_viewer_frame_are_read_only_projections
```

Expected:

```text
snapshot_contains_resource_layer_totals ... ok
snapshot_and_viewer_frame_are_read_only_projections ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/snapshot.rs tests/phase1_resource_grid.rs
git commit -m "feat: expose resource layer totals in snapshots"
```

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

- [ ] **Step 4: Run Python tool tests only if parser/scenario semantics were changed beyond Rust mapping**

Run only if Rust parser changes require a Python sanity check:

```bash
python -m pytest .\tools\early-stability
```

Expected:

```text
all early-stability tests pass
```

If Python was not run, say so in the report and explain:

```text
Python tool code was not changed; Rust Phase 1 behavior was verified with Cargo tests.
```

---

## Task 10: Write Implementation Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1B-accounting-and-resource-grid.md`
- Modify: `outputs/worklogs/README.md`

- [ ] **Step 1: Create report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1B-accounting-and-resource-grid.md`:

```markdown
# REPORT: Phase 1B Accounting And ResourceGrid

## Goal

Implemented Phase 1B accounting hardening and minimal flat ResourceGrid boundary.

## Scope

- Added `GridCoord`.
- Added `ResourceConfig` to `RuntimeConfig`.
- Replaced one-layer ResourceGrid placeholder with flat layer/grid storage.
- Initialized external ResourceGrid from `[resources].initial_distribution`.
- Kept internal Cell resource inventory separate from external ResourceGrid.
- Preserved Energy, Heat, Waste and Capacity scenario behavior.
- Added resource layer totals to committed snapshots.

## Decisions

- ResourceGrid is flat and deterministic.
- Phase 1B applies simple per-cell/layer decay only.
- No Resource uptake/export, diffusion, metabolism or Process Registry behavior was added.
- Over-capacity remains valid config and resolves as runtime collapse.

## Scenario Results

Paste output from:

```text
cargo test --test phase1_config_validation dump_current_phase1_rust_config_results -- --ignored --nocapture
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
Python tool code was not changed; Python tests were not required for this Rust-only Phase 1B change.
```

## Open Questions

- Resource diffusion remains out of Phase 1B.
- Resource uptake/export remains out of Phase 1B.
- Per-resource identity is represented by layer order for now; richer typed Resource registries can be designed later.
```

- [ ] **Step 2: Add report to worklog index**

Add under `## Reports` in `outputs/worklogs/README.md`:

```markdown
- [[outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1B-accounting-and-resource-grid|outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1B-accounting-and-resource-grid]]
```

Use the actual report timestamp.

- [ ] **Step 3: Commit report**

```bash
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1B-accounting-and-resource-grid.md outputs/worklogs/README.md
git commit -m "docs: report phase 1B accounting and resource grid"
```

---

## Acceptance Gates

Phase 1B is complete only when:

```text
ResourceGrid is flat, indexed and deterministic.
ResourceGrid uses ResourceConfig.initial_distribution, not CellInitialConfig.initial_resource_amount.
Internal Cell inventory and external ResourceGrid remain separate.
Resource decay updates every grid cell/layer deterministically.
Snapshot exposes resource layer totals read-only.
Existing Phase 1 scenario outcomes remain stable.
No Genome/Process/Joints/division/diffusion/metabolism behavior is added.
cargo fmt --check passes.
cargo clippy --workspace --all-targets --all-features -- -D warnings passes.
cargo test passes.
Implementation report exists and is linked from outputs/worklogs/README.md.
```

---

## Self-Review

Spec coverage:

- ResourceGrid placeholder is replaced with flat storage.
- Heat/waste accounting remains explicit and unchanged.
- Capacity and Energy behavior remains covered by existing tests.
- Parser/runtime boundary is preserved.
- Future sparse/chunked/diffusion paths remain open.

Known limits:

- This plan does not implement diffusion.
- This plan does not implement Resource uptake/export.
- This plan does not add typed resource registries beyond layer order.
- This plan does not introduce performance benchmarks; that belongs to a later Phase 1 performance smoke plan.
