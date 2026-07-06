# Phase 2A Multi-Cell and Physics Solver Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Extend the ALife simulation core to support multiple cells initialized from TOML configs, query neighborhood cells using a custom counting-sort-based uniform SpatialIndex, and resolve cell-cell and cell-wall overlaps deterministically using a position-based solver.

**Architecture:** Cells are modeled as soft circles (Position, Radius) inside the central `CellStore`. Overlaps are resolved iteratively via Verlet-relaxation positional correction. Determism is guaranteed by sorting contact pairs by CellIndex and avoiding unordered iterations or floating-point non-determinism.

**Tech Stack:** Rust 2024, Cargo integration tests, Serde/TOML parser.

---

## File Structure

Modify:
- [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs): Add multi-cell config fields, solver options, validation, and update `config_hash()`.
- [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs): Update `RawScenarioConfig` to parse initial cells as a list and handle backward compatibility.
- [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs): Initialize multiple cells inside `WorldState::from_config`.
- [spatial.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/spatial.rs): Implement counting-sort uniform grid rebuild and query.
- [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs): Integrate spatial index rebuilding and the positional solver loop into `TickExecutor::step`.
- [summary.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/summary.rs): Add metrics fields for collisions and overlap resolution.

Create:
- [phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs): Integration test suite verifying multi-cell layouts, deterministic collision relaxation, and solid wall boundaries.

---

## Task 1: Add Multi-Cell Config, Parsing & Validation

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)

- [ ] **Step 1: Write failing config validation test**
Add a test in [tests/phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs) that verifies multi-cell config initialization, fallback, and validation.

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig,
    ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Temperature, Tick, WasteAmount, WorldSize,
};

fn base_test_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(100),
            seed: Seed::from_raw(42),
            size: WorldSize::new(16.0, 16.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
        },
        ResourceConfig::new(vec![ResourceAmount::new(10.0).unwrap()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
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
fn runtime_config_supports_multi_cell_list() {
    let base = base_test_config();
    assert_eq!(base.initial_cells.len(), 1);

    let cell_2 = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        ..base.cell
    };
    let multi = base.with_cells(vec![base.cell, cell_2]);
    assert_eq!(multi.initial_cells.len(), 2);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_core_smoke runtime_config_supports_multi_cell_list`
Expected: Compilation failure due to missing `initial_cells` and `with_cells` on `RuntimeConfig`.

- [ ] **Step 3: Add `initial_cells` field and builder to RuntimeConfig**
In [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs), add `initial_cells` field and `with_cells` method. Also update `config_hash()` to hash all initial cells.

```rust
#[derive(Clone, Debug, PartialEq)]
pub struct RuntimeConfig {
    pub world: WorldConfig,
    pub space: SpaceConfig,
    pub resources: ResourceConfig,
    pub resource_interaction: ResourceInteractionConfig,
    pub cell: CellInitialConfig,
    pub environment: EnvironmentConfig,
    pub lifecycle: LifecycleConfig,
    pub initial_cells: Vec<CellInitialConfig>,
}

impl RuntimeConfig {
    pub fn new(
        world: WorldConfig,
        space: SpaceConfig,
        resources: ResourceConfig,
        resource_interaction: ResourceInteractionConfig,
        cell: CellInitialConfig,
        environment: EnvironmentConfig,
        lifecycle: LifecycleConfig,
    ) -> Result<Self, ConfigError> {
        // ... (existing validations)
        let initial_cells = vec![cell];
        Ok(Self {
            world,
            space,
            resources,
            resource_interaction,
            cell,
            environment,
            lifecycle,
            initial_cells,
        })
    }

    pub fn with_cells(mut self, cells: Vec<CellInitialConfig>) -> Self {
        self.initial_cells = cells;
        self
    }
}
```

Update `config_hash()`:
```rust
        // ... hash other fields ...
        for cell in &self.initial_cells {
            for value in [
                cell.initial_energy.raw().to_bits() as u64,
                cell.energy_capacity.raw().to_bits() as u64,
                cell.mandatory_cost_per_tick.raw().to_bits() as u64,
                cell.passive_energy_income.raw().to_bits() as u64,
                cell.capacity_limit.raw().to_bits() as u64,
            ] {
                hash ^= value;
                hash = hash.wrapping_mul(0x100000001b3);
            }
        }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_core_smoke runtime_config_supports_multi_cell_list`
Expected: PASS

---

## Task 2: Implement Multi-Cell TOML Parsing

**Files:**
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)

- [ ] **Step 1: Write failing parser test**
Add a test in [tests/phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs) that parses a scenario containing multiple cells.

```rust
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_loads_multiple_initial_cells() {
    let toml = r#"
scenario_id = "multi_cell_test"
seed = 42
tick_count = 100

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = {}
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

[[cells]]
initial_position = [2.0, 2.0]
radius = 1.0
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

[[cells]]
initial_position = [3.0, 3.0]
radius = 1.5
initial_energy = 8.0
energy_capacity = 12.0
mandatory_cost_per_tick = 2.0
capacity_limit = 25.0

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
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();
    assert_eq!(config.initial_cells.len(), 2); // [[cells]] blocks parse.
    assert_eq!(config.initial_cells[0].position.x(), 2.0);
    assert_eq!(config.initial_cells[1].position.x(), 3.0);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_core_smoke parser_loads_multiple_initial_cells`
Expected: FAIL/Compilation Error due to missing `cells` in `RawScenarioConfig`.

- [ ] **Step 3: Modify RawScenarioConfig fields and mapping**
In [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs), add `cells: Option<Vec<RawCell>>` to `RawScenarioConfig` and update `to_runtime_config` to map them.

```rust
#[derive(Deserialize, Debug)]
pub struct RawScenarioConfig {
    pub scenario_id: String,
    pub seed: u64,
    pub tick_count: u64,
    pub world: RawWorld,
    pub space: RawSpace,
    pub resources: RawResources,
    pub resource_interaction: Option<RawResourceInteraction>,
    pub cell: RawCell,
    pub cells: Option<Vec<RawCell>>,
    pub environment: RawEnvironment,
    pub lifecycle: RawLifecycle,
}
```

In `to_runtime_config()`:
```rust
        // ... (parse cell config)
        let main_cell = CellInitialConfig { ... };

        let mut initial_cells = Vec::new();
        if let Some(ref raw_cells) = self.cells {
            for raw_cell in raw_cells {
                let cell_conf = CellInitialConfig {
                    position: Position::new(raw_cell.initial_position[0], raw_cell.initial_position[1])
                        .map_err(|e| ParseError::ValidationError(format!("Invalid cell position: {:?}", e)))?,
                    radius: Radius::new(raw_cell.radius)
                        .map_err(|e| ParseError::ValidationError(format!("Invalid cell radius: {:?}", e)))?,
                    initial_energy: EnergyAmount::new(raw_cell.initial_energy)
                        .map_err(|e| ParseError::ValidationError(format!("Invalid initial_energy: {:?}", e)))?,
                    energy_capacity: EnergyAmount::new(raw_cell.energy_capacity)
                        .map_err(|e| ParseError::ValidationError(format!("Invalid energy_capacity: {:?}", e)))?,
                    mandatory_cost_per_tick: EnergyAmount::new(raw_cell.mandatory_cost_per_tick)
                        .map_err(|e| ParseError::ValidationError(format!("Invalid mandatory_cost: {:?}", e)))?,
                    passive_energy_income: EnergyAmount::zero(),
                    capacity_limit: CapacityAmount::new(raw_cell.capacity_limit)
                        .map_err(|e| ParseError::ValidationError(format!("Invalid capacity_limit: {:?}", e)))?,
                    initial_resource_amount: ResourceAmount::zero(),
                    initial_material_amount: MaterialAmount::new(
                        raw_cell.initial_materials.values().sum(),
                    )
                    .map_err(|e| ParseError::ValidationError(format!("Invalid material amount: {:?}", e)))?,
                };
                initial_cells.push(cell_conf);
            }
        }

        let mut runtime_config = RuntimeConfig::new(
            world,
            space,
            resources,
            resource_interaction,
            main_cell,
            environment,
            lifecycle,
        )
        .map_err(ParseError::ConfigValidationError)?;

        if !initial_cells.is_empty() {
            runtime_config = runtime_config.with_cells(initial_cells);
        }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_core_smoke parser_loads_multiple_initial_cells`
Expected: PASS

---

## Task 3: Initialize Multiple Cells in WorldState

**Files:**
- Modify: [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs)

- [ ] **Step 1: Write failing initialization test**
Add a test in [tests/phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs) that verifies many cells are loaded into the world state.

```rust
use alife::core::world::WorldState;

#[test]
fn world_state_initializes_multiple_cells_from_config() {
    let base = base_test_config();
    let cell_2 = CellInitialConfig {
        position: Position::new(5.0, 5.0),
        ..base.cell
    };
    let config = base.with_cells(vec![base.cell, cell_2]);

    let world = WorldState::from_config(config).unwrap();
    assert_eq!(world.cells().len(), 2);
    assert_eq!(world.cells().position(CellIndex::from_raw(0)).x(), 1.0);
    assert_eq!(world.cells().position(CellIndex::from_raw(1)).x(), 5.0);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_core_smoke world_state_initializes_multiple_cells_from_config`
Expected: FAIL (world cells len is 1, not 2).

- [ ] **Step 3: Update `from_config` in `world.rs`**
Modify [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs:26-37) to loop over all `config.initial_cells`.

```rust
    pub fn from_config(config: RuntimeConfig) -> Result<Self, WorldInitError> {
        let mut cells = CellStore::with_capacity(config.initial_cells.len());
        for cell_config in &config.initial_cells {
            cells.insert_initial(InitialCellState {
                position: cell_config.position,
                radius: cell_config.radius,
                energy: EnergyBuffer::new(cell_config.initial_energy, cell_config.energy_capacity),
                resources: cell_config.initial_resource_amount,
                materials: cell_config.initial_material_amount,
                capacity_limit: cell_config.capacity_limit,
                temperature: crate::core::units::Temperature::new(25.0),
            });
        }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_core_smoke world_state_initializes_multiple_cells_from_config`
Expected: PASS

---

## Task 4: Implement Rebuildable Counting-Sort SpatialIndex

**Files:**
- Modify: [spatial.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/spatial.rs)

- [ ] **Step 1: Write failing spatial indexing and query test**
Add a test in [tests/phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs) that indexes cells and queries their neighborhood.

```rust
use alife::core::spatial::SpatialIndex;
use alife::core::cell_store::{CellStore, InitialCellState, EnergyBuffer};

#[test]
fn spatial_index_rebuilds_and_queries_neighbors() {
    let mut cells = CellStore::with_capacity(3);
    // Cell 0 at (1.0, 1.0)
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 1.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(EnergyAmount::new(5.0).unwrap(), EnergyAmount::new(10.0).unwrap()),
        resources: ResourceAmount::zero(),
        materials: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });
    // Cell 1 at (2.0, 1.0) -> same grid cell or neighbor
    cells.insert_initial(InitialCellState {
        position: Position::new(2.0, 1.0),
        ..cells.initial_state(CellIndex::from_raw(0))
    });
    // Cell 2 at (15.0, 15.0) -> far away
    cells.insert_initial(InitialCellState {
        position: Position::new(15.0, 15.0),
        ..cells.initial_state(CellIndex::from_raw(0))
    });

    let mut spatial = SpatialIndex::new();
    spatial.rebuild(&cells, WorldSize::new(16.0, 16.0).unwrap(), 8.0);

    let mut pairs = Vec::new();
    spatial.generate_candidate_pairs(&cells, &mut pairs);

    // Expect pairs (0, 1), not (0, 2) or (1, 2)
    assert_eq!(pairs.len(), 1);
    let pair = pairs[0];
    assert_eq!(pair.0.raw(), 0);
    assert_eq!(pair.1.raw(), 1);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_core_smoke spatial_index_rebuilds_and_queries_neighbors`
Expected: Compilation failure due to missing rebuild signatures and query methods on `SpatialIndex`.

- [ ] **Step 3: Implement uniform grid counting-sort index**
In [spatial.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/spatial.rs), implement counting-sort-based indexing.

```rust
use crate::core::cell_store::{CellIndex, CellStore, LifecycleState};
use crate::core::units::WorldSize;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct SpatialIndex {
    rebuild_count: u64,
    sorted_cells: Vec<CellIndex>,
    grid_offsets: Vec<usize>,
    cols: usize,
    rows: usize,
    grid_size: f32,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self {
            rebuild_count: 0,
            sorted_cells: Vec::new(),
            grid_offsets: Vec::new(),
            cols: 0,
            rows: 0,
            grid_size: 1.0,
        }
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count
    }

    pub fn rebuild(&mut self, cells: &CellStore, world_size: WorldSize, grid_size: f32) {
        self.rebuild_count += 1;
        let cell_count = cells.len();
        self.sorted_cells.clear();
        self.sorted_cells.resize(cell_count, CellIndex::from_raw(0));

        let cols = (world_size.width() / grid_size).ceil() as usize;
        let rows = (world_size.height() / grid_size).ceil() as usize;
        let total_cells = cols * rows;
        self.cols = cols;
        self.rows = rows;
        self.grid_size = grid_size;

        self.grid_offsets.clear();
        self.grid_offsets.resize(total_cells + 1, 0);

        // 1. Count cells per grid cell
        let mut active_count = 0;
        for i in 0..cell_count {
            let idx = CellIndex::from_raw(i);
            if cells.lifecycle_state(idx) == LifecycleState::Dead {
                continue;
            }
            let pos = cells.position(idx);
            let cx = ((pos.x() / grid_size).floor() as usize).min(cols - 1);
            let cy = ((pos.y() / grid_size).floor() as usize).min(rows - 1);
            let grid_idx = cy * cols + cx;
            self.grid_offsets[grid_idx] += 1;
            active_count += 1;
        }

        self.sorted_cells.truncate(active_count);

        // 2. Prefix sum (offsets)
        let mut sum = 0;
        for i in 0..total_cells {
            let count = self.grid_offsets[i];
            self.grid_offsets[i] = sum;
            sum += count;
        }
        self.grid_offsets[total_cells] = sum;

        // 3. Populate sorted_cells
        let mut insertion_offsets = self.grid_offsets.clone();
        for i in 0..cell_count {
            let idx = CellIndex::from_raw(i);
            if cells.lifecycle_state(idx) == LifecycleState::Dead {
                continue;
            }
            let pos = cells.position(idx);
            let cx = ((pos.x() / grid_size).floor() as usize).min(cols - 1);
            let cy = ((pos.y() / grid_size).floor() as usize).min(rows - 1);
            let grid_idx = cy * cols + cx;
            let dest = insertion_offsets[grid_idx];
            self.sorted_cells[dest] = idx;
            insertion_offsets[grid_idx] += 1;
        }
    }

    pub fn generate_candidate_pairs(&self, cells: &CellStore, pairs: &mut Vec<(CellIndex, CellIndex)>) {
        pairs.clear();
        if self.cols == 0 || self.rows == 0 {
            return;
        }

        let neighbors: [(isize, isize); 4] = [
            (1, 0),   // right
            (-1, 1),  // bottom-left
            (0, 1),   // bottom
            (1, 1),   // bottom-right
        ];

        for cy in 0..self.rows {
            for cx in 0..self.cols {
                let grid_idx = cy * self.cols + cx;
                let start = self.grid_offsets[grid_idx];
                let end = self.grid_offsets[grid_idx + 1];

                // CellIndex in same grid cell
                for i in start..end {
                    let idx_i = self.sorted_cells[i];
                    for j in (i + 1)..end {
                        let idx_j = self.sorted_cells[j];
                        if idx_i.raw() < idx_j.raw() {
                            pairs.push((idx_i, idx_j));
                        } else {
                            pairs.push((idx_j, idx_i));
                        }
                    }

                    // Cells in adjacent grid cells
                    for &(dx, dy) in &neighbors {
                        let nx = cx as isize + dx;
                        let ny = cy as isize + dy;
                        if nx >= 0 && nx < self.cols as isize && ny >= 0 && ny < self.rows as isize {
                            let neighbor_idx = (ny as usize) * self.cols + (nx as usize);
                            let n_start = self.grid_offsets[neighbor_idx];
                            let n_end = self.grid_offsets[neighbor_idx + 1];
                            for k in n_start..n_end {
                                let idx_k = self.sorted_cells[k];
                                if idx_i.raw() < idx_k.raw() {
                                    pairs.push((idx_i, idx_k));
                                } else {
                                    pairs.push((idx_k, idx_i));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Sort pairs to ensure stable, deterministic iteration order
        pairs.sort_unstable_by_key(|&(i, j)| (i.raw(), j.raw()));
    }
}
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_core_smoke spatial_index_rebuilds_and_queries_neighbors`
Expected: PASS

---

## Task 5: Add Solver Config Options

**Files:**
- Modify: [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs)
- Modify: [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs)

- [ ] **Step 1: Write failing config compilation test**
Add a test in [tests/phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs) validating solver iterations in config parsing.

```rust
#[test]
fn parser_loads_solver_iterations_from_space_config() {
    let toml = r#"
scenario_id = "solver_test"
seed = 42
tick_count = 100

[world]
size = [16.0, 16.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 8

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [1.0, 1.0]
radius = 1.0
initial_resources = {}
initial_materials = {}
initial_energy = 5.0
energy_capacity = 10.0
mandatory_cost_per_tick = 2.0
capacity_limit = 20.0

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
stress_energy_threshold = 2.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();
    assert_eq!(config.space.physics_solver_iterations, 8);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_core_smoke parser_loads_solver_iterations_from_space_config`
Expected: Compilation failure due to missing `physics_solver_iterations` inside `SpaceConfig` and `RawSpace`.

- [ ] **Step 3: Modify SpaceConfig and RawSpace**
In [config.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/config.rs), update `SpaceConfig`:
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceConfig {
    pub spatial_grid_size: f32,
    pub physics_solver_iterations: usize,
}
```

Update default `SpaceConfig` creation (in existing tests) to set `physics_solver_iterations: 4`.

In [config_parser.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/runner/config_parser.rs), update `RawSpace`:
```rust
#[derive(Deserialize, Debug)]
pub struct RawSpace {
    pub spatial_grid_size: f32,
    pub physics_solver_iterations: Option<usize>,
}
```

In `to_runtime_config`:
```rust
        let space = SpaceConfig {
            spatial_grid_size: self.space.spatial_grid_size,
            physics_solver_iterations: self.space.physics_solver_iterations.unwrap_or(4),
        };
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_core_smoke parser_loads_solver_iterations_from_space_config`
Expected: PASS

---

## Task 6: Implement Positional Physics Solver in TickExecutor

**Files:**
- Modify: [tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs)
- Modify: [world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs)
- Modify: [summary.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/summary.rs)

- [ ] **Step 1: Write failing physics solver resolution test**
Add a test in [tests/phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs) that sets up two heavily overlapping cells and verifies they are pushed apart after one tick.

```rust
#[test]
fn tick_executor_resolves_overlaps_and_clamped_by_walls() {
    let base = base_test_config();
    // Cell 0 at (4.0, 4.0), radius 2.0
    let cell_1 = CellInitialConfig {
        position: Position::new(4.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..base.cell
    };
    // Cell 1 at (5.0, 4.0), radius 2.0 -> Overlap distance = 3.0
    let cell_2 = CellInitialConfig {
        position: Position::new(5.0, 4.0),
        radius: Radius::new(2.0).unwrap(),
        ..base.cell
    };
    let config = base.with_cells(vec![cell_1, cell_2]);

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    let p1 = executor.world().cells().position(CellIndex::from_raw(0));
    let p2 = executor.world().cells().position(CellIndex::from_raw(1));

    // Cells should separate along the X axis
    assert!(p1.x() < 4.0);
    assert!(p2.x() > 5.0);
    // Distance should be close to 4.0 (combined radii)
    let dist = ((p1.x() - p2.x()).powi(2) + (p1.y() - p2.y()).powi(2)).sqrt();
    assert!(dist >= 3.9);
}
```

- [ ] **Step 2: Run test to verify failure**
Run: `cargo test --test phase2_core_smoke tick_executor_resolves_overlaps_and_clamped_by_walls`
Expected: FAIL (positions remain unchanged after tick).

- [ ] **Step 3: Integrate SpatialIndex and positional solver in tick.rs**
First, expose mutable getters for position in `CellStore` [src/core/cell_store.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/cell_store.rs):

```rust
    pub fn set_position(&mut self, index: CellIndex, position: Position) {
        self.positions[index.raw()] = position;
    }
```

In `world.rs` [src/core/world.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/world.rs), expose:
```rust
    pub fn spatial_index_mut_for_commit(&mut self) -> &mut SpatialIndex {
        &mut self.spatial_index
    }
```

In `summary.rs` [src/core/summary.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/summary.rs), add `overlap_resolved`:
```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetricsSummary {
    // ... other metrics ...
    pub overlap_resolved: f32,
}
```
Update all `build_metrics_summary` calls (in `tick.rs` and other tests) to output `overlap_resolved: 0.0` or the computed value.

In `tick.rs` [src/core/tick.rs](file:///c:/Users/korsr/PycharmProjects/ALife/src/core/tick.rs), update `step()` to rebuild the spatial index and run the Verlet positional solver iterations.

```rust
    pub fn step(&mut self) -> Result<RunSummary, TickError> {
        let config = self.world.config().clone();
        let len = self.world.cells().len();

        let mut metabolism_heat_total = 0.0_f32;
        let mut metabolism_waste_total = 0.0_f32;

        // Rebuild Spatial Index at the start of tick
        {
            let world_size = config.world.size;
            let grid_size = config.space.spatial_grid_size;
            let cells = self.world.cells();
            let spatial = self.world.spatial_index_mut_for_commit();
            spatial.rebuild(cells, world_size, grid_size);
        }

        // ... (Phase A: Uptake and Metabolism) ...

        // Positional Overlap Solver Loop
        let mut overlap_resolved = 0.0;
        {
            let mut pairs = Vec::new();
            {
                let cells = self.world.cells();
                self.world.spatial_index().generate_candidate_pairs(cells, &mut pairs);
            }

            let iterations = config.space.physics_solver_iterations;
            let world_size = config.world.size;

            for _ in 0..iterations {
                // 1. Resolve cell-cell overlaps
                for &(idx_i, idx_j) in &pairs {
                    let (pos_i, r_i) = {
                        let cells = self.world.cells();
                        if cells.lifecycle_state(idx_i) == LifecycleState::Dead
                            || cells.lifecycle_state(idx_j) == LifecycleState::Dead
                        {
                            continue;
                        }
                        (cells.position(idx_i), cells.radius(idx_i))
                    };
                    let (pos_j, r_j) = {
                        let cells = self.world.cells();
                        (cells.position(idx_j), cells.radius(idx_j))
                    };

                    let dx = pos_i.x() - pos_j.x();
                    let dy = pos_i.y() - pos_j.y();
                    let dist_sq = dx * dx + dy * dy;
                    let target_dist = r_i.raw() + r_j.raw();

                    if dist_sq < target_dist * target_dist {
                        let dist = dist_sq.sqrt();
                        let overlap = target_dist - dist;
                        overlap_resolved += overlap;

                        let (ux, uy) = if dist > 0.0 {
                            (dx / dist, dy / dist)
                        } else {
                            // If exactly overlapping, push along X axis deterministically based on ID order
                            let sign = if idx_i.raw() < idx_j.raw() { 1.0 } else { -1.0 };
                            (sign, 0.0)
                        };

                        // Push each cell by half of the overlap distance
                        let push_dist = overlap * 0.5;
                        let new_pos_i = Position::new(
                            pos_i.x() + ux * push_dist,
                            pos_i.y() + uy * push_dist,
                        )
                        .unwrap_or(pos_i);
                        let new_pos_j = Position::new(
                            pos_j.x() - ux * push_dist,
                            pos_j.y() - uy * push_dist,
                        )
                        .unwrap_or(pos_j);

                        let cells = self.world.cells_mut_for_commit();
                        cells.set_position(idx_i, new_pos_i);
                        cells.set_position(idx_j, new_pos_j);
                    }
                }

                // 2. Resolve wall boundaries (solid_wall)
                for i in 0..len {
                    let idx = CellIndex::from_raw(i);
                    let (pos, r) = {
                        let cells = self.world.cells();
                        if cells.lifecycle_state(idx) == LifecycleState::Dead {
                            continue;
                        }
                        (cells.position(idx), cells.radius(idx))
                    };

                    let radius = r.raw();
                    let mut px = pos.x();
                    let mut py = pos.y();
                    let mut clamped = false;

                    if px - radius < 0.0 {
                        px = radius;
                        clamped = true;
                    } else if px + radius > world_size.width() {
                        px = world_size.width() - radius;
                        clamped = true;
                    }

                    if py - radius < 0.0 {
                        py = radius;
                        clamped = true;
                    } else if py + radius > world_size.height() {
                        py = world_size.height() - radius;
                        clamped = true;
                    }

                    if clamped {
                        let cells = self.world.cells_mut_for_commit();
                        cells.set_position(idx, Position::new(px, py).unwrap_or(pos));
                    }
                }
            }
        }

        // ... (Phase B: Environment Updates) ...
        // ... (Phase C: Pay cost and check lifecycle) ...
```

In `build_metrics_summary` in `tick.rs`:
```rust
        MetricsSummary {
            // ... other fields ...
            overlap_resolved,
        }
```

- [ ] **Step 4: Run test to verify it passes**
Run: `cargo test --test phase2_core_smoke tick_executor_resolves_overlaps_and_clamped_by_walls`
Expected: PASS

---

## Task 7: Verify Determinism and Boundary Behavior

**Files:**
- Modify: [phase2_core_smoke.rs](file:///c:/Users/korsr/PycharmProjects/ALife/tests/phase2_core_smoke.rs)

- [ ] **Step 1: Write determinism & boundary test**
Add a test verifying that multi-cell configurations resolve identically for the same config and seed, and that cells never leave the solid walls even under high boundary pressure.

```rust
#[test]
fn multi_cell_world_retains_perfect_determinism_and_solid_walls() {
    let base = base_test_config();
    let cell_1 = CellInitialConfig {
        position: Position::new(1.0, 1.0), // Near left-top corner
        radius: Radius::new(1.5).unwrap(),
        ..base.cell
    };
    let cell_2 = CellInitialConfig {
        position: Position::new(1.5, 1.0), // Pushing cell_1 further into corner
        radius: Radius::new(1.5).unwrap(),
        ..base.cell
    };
    let config_a = base.with_cells(vec![cell_1, cell_2]);
    let config_b = config_a.clone();

    let mut exec_a = TickExecutor::new(config_a).unwrap();
    let mut exec_b = TickExecutor::new(config_b).unwrap();

    let summary_a = exec_a.run_until_configured_tick().unwrap();
    let summary_b = exec_b.run_until_configured_tick().unwrap();

    assert_eq!(summary_a, summary_b);

    // Verify cell_1 did not exit the 0.0 X/Y boundary
    let p1 = exec_a.world().cells().position(CellIndex::from_raw(0));
    assert!(p1.x() >= 1.5);
    assert!(p1.y() >= 1.5);
}
```

- [ ] **Step 2: Run test**
Run: `cargo test --test phase2_core_smoke multi_cell_world_retains_perfect_determinism_and_solid_walls`
Expected: PASS

- [ ] **Step 3: Run full Rust test suite**
Run: `cargo test`
Expected: PASS (all 48 existing + new phase 2A tests pass).

- [ ] **Step 4: Format and Lints check**
Run:
```bash
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```
Expected: All pass.

- [ ] **Step 5: Write completion report**
Document changes in worklog. Do not commit.

---

## Acceptance Check

The Phase 2A plan is complete when:
- Multiple cells can be configured and initialized.
- A rebuildable uniform SpatialIndex uses counting sort to index cell positions.
- Overlaps between cells and solid walls are resolved deterministically.
- Determinism replay runs perfectly for multi-cell setups.
- Formatter, linter, and all cargo tests are green.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
