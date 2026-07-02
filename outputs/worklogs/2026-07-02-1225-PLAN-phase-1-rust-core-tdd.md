# Phase 1 Rust Core Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first deterministic Rust `alife-core` vertical slice: one Cell, mandatory Energy accounting, lifecycle, Phase 1 accounting, deterministic replay, events, summary and snapshot projection.

**Architecture:** Implement a data-oriented core under `src/core/` and keep `src/main.rs` as a thin runner shell. Phase 1 uses typed ids, typed accounting wrappers, `WorldState`, `CellStore`, `TickExecutor`, bounded events and read-only snapshots. Existing placeholder modules may remain, but simulation behavior must live in `src/core/`.

**Tech Stack:** Rust 2024, standard library only for Phase 1 core, Cargo unit/integration tests, TDD with `cargo test` after each task.

---

## Required Reading

Before executing this plan, read:

- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/ROADMAP.md`
- `docs/implementation/phase-1-design.md`
- `docs/implementation/phase-1-data-model.md`
- `docs/implementation/phase-1-module-api.md`
- `docs/implementation/optimization-paths.md`
- `docs/engine/performance.md`
- `docs/engine/scheduler.md`

Canon constraints to preserve:

- no Genome Runtime, division, mutation, Joints, signals or evolution analytics in Phase 1;
- no viewer/storage authority;
- no hardcoded biological roles or cell classes;
- deterministic mode is source of truth;
- no hot-loop `CellId -> CellIndex` lookup;
- no default `MandatoryCostPaid` event spam.

---

## File Structure

Create:

```text
src/core/mod.rs
src/core/ids.rs
src/core/units.rs
src/core/config.rs
src/core/cell_store.rs
src/core/resources.rs
src/core/environment.rs
src/core/spatial.rs
src/core/lifecycle.rs
src/core/events.rs
src/core/deltas.rs
src/core/world.rs
src/core/tick.rs
src/core/snapshot.rs
src/core/summary.rs
tests/phase1_core_smoke.rs
tests/phase1_accounting.rs
tests/phase1_determinism.rs
```

Modify:

```text
src/lib.rs
src/main.rs
```

Leave as compatibility placeholders for now:

```text
src/world/mod.rs
src/cell/mod.rs
src/simulation/mod.rs
src/physics/mod.rs
src/renderer/mod.rs
src/organism/mod.rs
```

Do not move simulation behavior into the compatibility modules.

---

## Milestones

```text
Phase 1A: Core Smoke
  config -> WorldState -> one Cell -> TickExecutor -> mandatory cost -> lifecycle -> RunSummary

Phase 1B: Accounting
  ResourceGrid placeholder -> heat/waste -> capacity -> energy clamp -> collapse reasons

Phase 1C: Determinism + Outputs
  deterministic replay test -> EventBuffer -> Snapshot -> ViewerFrame projection

Phase 1D: Tool Compatibility
  summary vocabulary and scenarios align with early-stability/reachability expectations
```

Each milestone must pass tests before the next milestone starts.

---

## Task 1: Core Module Skeleton

**Files:**

- Modify: `src/lib.rs`
- Create: `src/core/mod.rs`
- Create empty module files listed below.

- [ ] **Step 1: Write the failing import test**

Create `tests/phase1_core_smoke.rs`:

```rust
use alife::core;

#[test]
fn core_module_is_public() {
    let _ = core::CORE_MODULE_NAME;
    assert_eq!(core::CORE_MODULE_NAME, "alife-core");
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test core_module_is_public
```

Expected:

```text
FAIL: unresolved import `alife::core`
```

- [ ] **Step 3: Add core module skeleton**

Modify `src/lib.rs`:

```rust
pub mod core;

pub mod world;
pub mod physics;
pub mod cell;
pub mod organism;
pub mod simulation;
pub mod renderer;
```

Create `src/core/mod.rs`:

```rust
pub const CORE_MODULE_NAME: &str = "alife-core";

pub mod ids;
pub mod units;
pub mod config;
pub mod cell_store;
pub mod resources;
pub mod environment;
pub mod spatial;
pub mod lifecycle;
pub mod events;
pub mod deltas;
pub mod world;
pub mod tick;
pub mod snapshot;
pub mod summary;
```

Create empty files:

```text
src/core/ids.rs
src/core/units.rs
src/core/config.rs
src/core/cell_store.rs
src/core/resources.rs
src/core/environment.rs
src/core/spatial.rs
src/core/lifecycle.rs
src/core/events.rs
src/core/deltas.rs
src/core/world.rs
src/core/tick.rs
src/core/snapshot.rs
src/core/summary.rs
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```bash
cargo test core_module_is_public
```

Expected:

```text
test core_module_is_public ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/lib.rs src/core tests/phase1_core_smoke.rs
git commit -m "feat: add phase 1 core module skeleton"
```

---

## Task 2: Typed IDs

**Files:**

- Modify: `src/core/ids.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing ID tests**

Append to `tests/phase1_core_smoke.rs`:

```rust
use alife::core::ids::{CellId, EventId, MaterialTypeId, ResourceTypeId};

#[test]
fn typed_ids_preserve_raw_values() {
    assert_eq!(CellId::from_raw(7).raw(), 7);
    assert_eq!(ResourceTypeId::from_raw(2).raw(), 2);
    assert_eq!(MaterialTypeId::from_raw(3).raw(), 3);
    assert_eq!(EventId::from_raw(4).raw(), 4);
}

#[test]
fn typed_ids_are_orderable_and_copyable() {
    let a = CellId::from_raw(1);
    let b = a;
    assert_eq!(a, b);
    assert!(CellId::from_raw(1) < CellId::from_raw(2));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test typed_ids
```

Expected:

```text
FAIL: unresolved imports or missing methods
```

- [ ] **Step 3: Implement typed IDs**

Replace `src/core/ids.rs` with:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ResourceTypeId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MaterialTypeId(u32);

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct EventId(u32);

macro_rules! impl_id {
    ($name:ident) => {
        impl $name {
            pub const fn from_raw(raw: u32) -> Self {
                Self(raw)
            }

            pub const fn raw(self) -> u32 {
                self.0
            }
        }
    };
}

impl_id!(CellId);
impl_id!(ResourceTypeId);
impl_id!(MaterialTypeId);
impl_id!(EventId);
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```bash
cargo test typed_ids
```

Expected:

```text
test typed_ids_preserve_raw_values ... ok
test typed_ids_are_orderable_and_copyable ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/ids.rs tests/phase1_core_smoke.rs
git commit -m "feat: add typed core ids"
```

---

## Task 3: Typed Units And Amount Math

**Files:**

- Modify: `src/core/units.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing unit tests**

Append to `tests/phase1_core_smoke.rs`:

```rust
use alife::core::units::{
    AmountError, CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position,
    Radius, ResourceAmount, Temperature, Tick, WasteAmount, WorldSize,
};

#[test]
fn amount_wrappers_reject_negative_values() {
    assert_eq!(EnergyAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(ResourceAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(MaterialAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(CapacityAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(HeatAmount::new(-0.1), Err(AmountError::Negative));
    assert_eq!(WasteAmount::new(-0.1), Err(AmountError::Negative));
}

#[test]
fn energy_math_is_saturating_and_clamped() {
    let energy = EnergyAmount::new(2.0).unwrap();
    let cost = EnergyAmount::new(5.0).unwrap();
    let gain = EnergyAmount::new(10.0).unwrap();
    let cap = EnergyAmount::new(6.0).unwrap();

    assert_eq!(energy.saturating_sub(cost).raw(), 0.0);
    assert_eq!(energy.saturating_add(gain).clamp_max(cap).raw(), 6.0);
}

#[test]
fn spatial_wrappers_validate_basic_bounds() {
    assert!(Radius::new(1.0).is_ok());
    assert!(Radius::new(0.0).is_err());
    assert_eq!(Position::new(2.0, 3.0).x(), 2.0);
    assert_eq!(WorldSize::new(512.0, 512.0).unwrap().width(), 512.0);
    assert_eq!(Tick::from_raw(42).raw(), 42);
    assert_eq!(Temperature::new(25.0).raw(), 25.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test amount_wrappers energy_math spatial_wrappers
```

Expected:

```text
FAIL: missing unit types or methods
```

- [ ] **Step 3: Implement units**

Replace `src/core/units.rs` with:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Tick(u64);

impl Tick {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }

    pub const fn next(self) -> Self {
        Self(self.0 + 1)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Seed(u64);

impl Seed {
    pub const fn from_raw(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AmountError {
    Negative,
    NonFinite,
}

fn validate_non_negative(value: f32) -> Result<f32, AmountError> {
    if !value.is_finite() {
        return Err(AmountError::NonFinite);
    }
    if value < 0.0 {
        return Err(AmountError::Negative);
    }
    Ok(value)
}

macro_rules! amount_type {
    ($name:ident) => {
        #[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
        pub struct $name(f32);

        impl $name {
            pub fn new(value: f32) -> Result<Self, AmountError> {
                validate_non_negative(value).map(Self)
            }

            pub const fn zero() -> Self {
                Self(0.0)
            }

            pub const fn raw(self) -> f32 {
                self.0
            }

            pub fn saturating_add(self, rhs: Self) -> Self {
                Self((self.0 + rhs.0).max(0.0))
            }

            pub fn saturating_sub(self, rhs: Self) -> Self {
                Self((self.0 - rhs.0).max(0.0))
            }

            pub fn clamp_max(self, max: Self) -> Self {
                Self(self.0.min(max.0).max(0.0))
            }
        }
    };
}

amount_type!(EnergyAmount);
amount_type!(ResourceAmount);
amount_type!(MaterialAmount);
amount_type!(CapacityAmount);
amount_type!(HeatAmount);
amount_type!(WasteAmount);

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Temperature(f32);

impl Temperature {
    pub const fn new(value: f32) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Position {
    x: f32,
    y: f32,
}

impl Position {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const fn x(self) -> f32 {
        self.x
    }

    pub const fn y(self) -> f32 {
        self.y
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Radius(f32);

impl Radius {
    pub fn new(value: f32) -> Result<Self, AmountError> {
        if !value.is_finite() {
            return Err(AmountError::NonFinite);
        }
        if value <= 0.0 {
            return Err(AmountError::Negative);
        }
        Ok(Self(value))
    }

    pub const fn raw(self) -> f32 {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldSize {
    width: f32,
    height: f32,
}

impl WorldSize {
    pub fn new(width: f32, height: f32) -> Result<Self, AmountError> {
        if !width.is_finite() || !height.is_finite() {
            return Err(AmountError::NonFinite);
        }
        if width <= 0.0 || height <= 0.0 {
            return Err(AmountError::Negative);
        }
        Ok(Self { width, height })
    }

    pub const fn width(self) -> f32 {
        self.width
    }

    pub const fn height(self) -> f32 {
        self.height
    }
}
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```bash
cargo test amount_wrappers energy_math spatial_wrappers
```

Expected:

```text
test amount_wrappers_reject_negative_values ... ok
test energy_math_is_saturating_and_clamped ... ok
test spatial_wrappers_validate_basic_bounds ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/units.rs tests/phase1_core_smoke.rs
git commit -m "feat: add typed units and bounded amount math"
```

---

## Task 4: Runtime Config

**Files:**

- Modify: `src/core/config.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing config tests**

Append to `tests/phase1_core_smoke.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, ConfigError, EnvironmentConfig, LifecycleConfig, RuntimeConfig,
    SpaceConfig, WorldConfig,
};
use alife::core::units::{CapacityAmount, EnergyAmount, HeatAmount, Seed, WasteAmount};

fn valid_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig { spatial_grid_size: 8.0 },
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
fn runtime_config_validates_energy_capacity() {
    let err = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(10),
            seed: Seed::from_raw(1),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig { spatial_grid_size: 8.0 },
        CellInitialConfig {
            position: Position::new(1.0, 1.0),
            radius: Radius::new(1.0).unwrap(),
            initial_energy: EnergyAmount::new(30.0).unwrap(),
            energy_capacity: EnergyAmount::new(20.0).unwrap(),
            mandatory_cost_per_tick: EnergyAmount::new(2.0).unwrap(),
            passive_energy_income: EnergyAmount::new(2.0).unwrap(),
            capacity_limit: CapacityAmount::new(30.0).unwrap(),
            initial_resource_amount: ResourceAmount::new(4.0).unwrap(),
            initial_material_amount: MaterialAmount::new(4.0).unwrap(),
        },
        valid_config().environment,
        valid_config().lifecycle,
    )
    .unwrap_err();

    assert_eq!(err, ConfigError::InitialEnergyExceedsCapacity);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test runtime_config_validates_energy_capacity
```

Expected:

```text
FAIL: missing config types
```

- [ ] **Step 3: Implement config**

Replace `src/core/config.rs` with:

```rust
use crate::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct WorldConfig {
    pub tick_count: Tick,
    pub seed: Seed,
    pub size: WorldSize,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct SpaceConfig {
    pub spatial_grid_size: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellInitialConfig {
    pub position: Position,
    pub radius: Radius,
    pub initial_energy: EnergyAmount,
    pub energy_capacity: EnergyAmount,
    pub mandatory_cost_per_tick: EnergyAmount,
    pub passive_energy_income: EnergyAmount,
    pub capacity_limit: CapacityAmount,
    pub initial_resource_amount: ResourceAmount,
    pub initial_material_amount: MaterialAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvironmentConfig {
    pub heat_current: HeatAmount,
    pub heat_generated_per_tick: HeatAmount,
    pub heat_dissipation_rate: HeatAmount,
    pub heat_warning_threshold: HeatAmount,
    pub heat_death_threshold: HeatAmount,
    pub waste_current: WasteAmount,
    pub waste_generated_per_tick: WasteAmount,
    pub waste_sink_rate: WasteAmount,
    pub waste_warning_threshold: WasteAmount,
    pub waste_death_threshold: WasteAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifecycleConfig {
    pub stress_energy_threshold: EnergyAmount,
    pub dormancy_allowed: bool,
    pub dormant_mandatory_cost_modifier: f32,
    pub critical_capacity_overrun: CapacityAmount,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RuntimeConfig {
    pub world: WorldConfig,
    pub space: SpaceConfig,
    pub cell: CellInitialConfig,
    pub environment: EnvironmentConfig,
    pub lifecycle: LifecycleConfig,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfigError {
    InitialEnergyExceedsCapacity,
    InvalidSpatialGridSize,
    InvalidDormancyModifier,
}

impl RuntimeConfig {
    pub fn new(
        world: WorldConfig,
        space: SpaceConfig,
        cell: CellInitialConfig,
        environment: EnvironmentConfig,
        lifecycle: LifecycleConfig,
    ) -> Result<Self, ConfigError> {
        if cell.initial_energy.raw() > cell.energy_capacity.raw() {
            return Err(ConfigError::InitialEnergyExceedsCapacity);
        }
        if !space.spatial_grid_size.is_finite() || space.spatial_grid_size <= 0.0 {
            return Err(ConfigError::InvalidSpatialGridSize);
        }
        if !lifecycle.dormant_mandatory_cost_modifier.is_finite()
            || lifecycle.dormant_mandatory_cost_modifier < 0.0
            || lifecycle.dormant_mandatory_cost_modifier > 1.0
        {
            return Err(ConfigError::InvalidDormancyModifier);
        }

        Ok(Self {
            world,
            space,
            cell,
            environment,
            lifecycle,
        })
    }

    pub fn config_hash(&self) -> u64 {
        let mut hash = 0xcbf29ce484222325_u64;
        for value in [
            self.world.tick_count.raw() as u64,
            self.world.seed.raw(),
            self.cell.initial_energy.raw().to_bits() as u64,
            self.cell.energy_capacity.raw().to_bits() as u64,
            self.cell.mandatory_cost_per_tick.raw().to_bits() as u64,
            self.cell.passive_energy_income.raw().to_bits() as u64,
            self.cell.capacity_limit.raw().to_bits() as u64,
        ] {
            hash ^= value;
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash
    }
}
```

- [ ] **Step 4: Run tests**

Run:

```bash
cargo test runtime_config_validates_energy_capacity
```

Expected:

```text
test runtime_config_validates_energy_capacity ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/config.rs tests/phase1_core_smoke.rs
git commit -m "feat: add validated phase 1 runtime config"
```

---

## Task 5: CellStore And Capacity Accounting

**Files:**

- Modify: `src/core/cell_store.rs`
- Modify: `tests/phase1_accounting.rs`

- [ ] **Step 1: Write failing CellStore tests**

Create `tests/phase1_accounting.rs`:

```rust
use alife::core::cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState, LifecycleState};
use alife::core::ids::CellId;
use alife::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[test]
fn cell_store_inserts_one_cell_with_deterministic_id() {
    let mut cells = CellStore::with_capacity(1);
    let id = cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 2.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(5.0).unwrap(),
            EnergyAmount::new(10.0).unwrap(),
        ),
        resources: ResourceAmount::new(4.0).unwrap(),
        materials: MaterialAmount::new(3.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    assert_eq!(id, CellId::from_raw(1));
    assert_eq!(cells.len(), 1);
    assert_eq!(cells.id_at(CellIndex::from_raw(0)), id);
    assert_eq!(cells.position(CellIndex::from_raw(0)).x(), 1.0);
}

#[test]
fn capacity_accounting_excludes_energy_and_includes_resources_materials() {
    let mut cells = CellStore::with_capacity(1);
    cells.insert_initial(InitialCellState {
        position: Position::new(1.0, 2.0),
        radius: Radius::new(1.0).unwrap(),
        energy: EnergyBuffer::new(
            EnergyAmount::new(100.0).unwrap(),
            EnergyAmount::new(100.0).unwrap(),
        ),
        resources: ResourceAmount::new(4.0).unwrap(),
        materials: MaterialAmount::new(3.0).unwrap(),
        capacity_limit: CapacityAmount::new(10.0).unwrap(),
        temperature: Temperature::new(25.0),
    });

    assert_eq!(cells.used_capacity(CellIndex::from_raw(0)).raw(), 7.0);
    assert_eq!(cells.free_capacity(CellIndex::from_raw(0)).raw(), 3.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test cell_store_inserts capacity_accounting
```

Expected:

```text
FAIL: missing CellStore types
```

- [ ] **Step 3: Implement CellStore**

Replace `src/core/cell_store.rs` with:

```rust
use crate::core::ids::CellId;
use crate::core::units::{
    CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, Temperature,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CellIndex(usize);

impl CellIndex {
    pub const fn from_raw(raw: usize) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> usize {
        self.0
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnergyBuffer {
    current: EnergyAmount,
    capacity: EnergyAmount,
}

impl EnergyBuffer {
    pub fn new(current: EnergyAmount, capacity: EnergyAmount) -> Self {
        Self {
            current: current.clamp_max(capacity),
            capacity,
        }
    }

    pub const fn current(self) -> EnergyAmount {
        self.current
    }

    pub const fn capacity(self) -> EnergyAmount {
        self.capacity
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleState {
    Alive,
    Stressed,
    Dormant,
    Dead,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct RuntimeFlags {
    pub mandatory_paid: bool,
    pub stalled: bool,
    pub over_capacity: bool,
    pub inert: bool,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct InitialCellState {
    pub position: Position,
    pub radius: Radius,
    pub energy: EnergyBuffer,
    pub resources: ResourceAmount,
    pub materials: MaterialAmount,
    pub capacity_limit: CapacityAmount,
    pub temperature: Temperature,
}

#[derive(Clone, Debug, Default)]
pub struct CellStore {
    ids: Vec<CellId>,
    positions: Vec<Position>,
    radii: Vec<Radius>,
    energy_buffers: Vec<EnergyBuffer>,
    resources: Vec<ResourceAmount>,
    materials: Vec<MaterialAmount>,
    capacity_limits: Vec<CapacityAmount>,
    temperatures: Vec<Temperature>,
    lifecycle_states: Vec<LifecycleState>,
    runtime_flags: Vec<RuntimeFlags>,
    next_cell_id: u32,
}

impl CellStore {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            ids: Vec::with_capacity(capacity),
            positions: Vec::with_capacity(capacity),
            radii: Vec::with_capacity(capacity),
            energy_buffers: Vec::with_capacity(capacity),
            resources: Vec::with_capacity(capacity),
            materials: Vec::with_capacity(capacity),
            capacity_limits: Vec::with_capacity(capacity),
            temperatures: Vec::with_capacity(capacity),
            lifecycle_states: Vec::with_capacity(capacity),
            runtime_flags: Vec::with_capacity(capacity),
            next_cell_id: 1,
        }
    }

    pub fn insert_initial(&mut self, cell: InitialCellState) -> CellId {
        let id = CellId::from_raw(self.next_cell_id);
        self.next_cell_id += 1;
        self.ids.push(id);
        self.positions.push(cell.position);
        self.radii.push(cell.radius);
        self.energy_buffers.push(cell.energy);
        self.resources.push(cell.resources);
        self.materials.push(cell.materials);
        self.capacity_limits.push(cell.capacity_limit);
        self.temperatures.push(cell.temperature);
        self.lifecycle_states.push(LifecycleState::Alive);
        self.runtime_flags.push(RuntimeFlags::default());
        id
    }

    pub fn len(&self) -> usize {
        self.ids.len()
    }

    pub fn is_empty(&self) -> bool {
        self.ids.is_empty()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = CellIndex> {
        (0..self.len()).map(CellIndex::from_raw)
    }

    pub fn resolve_id_cold(&self, id: CellId) -> Option<CellIndex> {
        self.ids.iter().position(|candidate| *candidate == id).map(CellIndex)
    }

    pub fn id_at(&self, index: CellIndex) -> CellId {
        self.ids[index.raw()]
    }

    pub fn position(&self, index: CellIndex) -> Position {
        self.positions[index.raw()]
    }

    pub fn energy(&self, index: CellIndex) -> EnergyBuffer {
        self.energy_buffers[index.raw()]
    }

    pub fn lifecycle_state(&self, index: CellIndex) -> LifecycleState {
        self.lifecycle_states[index.raw()]
    }

    pub fn runtime_flags(&self, index: CellIndex) -> RuntimeFlags {
        self.runtime_flags[index.raw()]
    }

    pub fn used_capacity(&self, index: CellIndex) -> CapacityAmount {
        let used = self.resources[index.raw()].raw() + self.materials[index.raw()].raw();
        CapacityAmount::new(used).expect("resource/material amounts are validated")
    }

    pub fn free_capacity(&self, index: CellIndex) -> CapacityAmount {
        let free = (self.capacity_limits[index.raw()].raw() - self.used_capacity(index).raw()).max(0.0);
        CapacityAmount::new(free).expect("free capacity is clamped")
    }

    pub(crate) fn set_energy(&mut self, index: CellIndex, energy: EnergyBuffer) {
        self.energy_buffers[index.raw()] = energy;
    }

    pub(crate) fn set_lifecycle_state(&mut self, index: CellIndex, state: LifecycleState) {
        self.lifecycle_states[index.raw()] = state;
    }

    pub(crate) fn set_runtime_flags(&mut self, index: CellIndex, flags: RuntimeFlags) {
        self.runtime_flags[index.raw()] = flags;
    }
}
```

- [ ] **Step 4: Run tests**

Run:

```bash
cargo test cell_store_inserts capacity_accounting
```

Expected:

```text
test cell_store_inserts_one_cell_with_deterministic_id ... ok
test capacity_accounting_excludes_energy_and_includes_resources_materials ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/cell_store.rs tests/phase1_accounting.rs
git commit -m "feat: add phase 1 cell store and capacity accounting"
```

---

## Task 6: WorldState Initialization

**Files:**

- Modify: `src/core/world.rs`
- Modify: `src/core/resources.rs`
- Modify: `src/core/environment.rs`
- Modify: `src/core/spatial.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing world init test**

Append to `tests/phase1_core_smoke.rs`:

```rust
use alife::core::world::WorldState;

#[test]
fn world_initializes_one_cell_from_config() {
    let world = WorldState::from_config(valid_config()).unwrap();

    assert_eq!(world.tick().raw(), 0);
    assert_eq!(world.cells().len(), 1);
    assert_eq!(world.environment().heat().raw(), 0.0);
    assert_eq!(world.resources().layer_count(), 1);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test world_initializes_one_cell_from_config
```

Expected:

```text
FAIL: missing WorldState
```

- [ ] **Step 3: Implement ResourceGrid, EnvironmentState, SpatialIndex placeholder, WorldState**

Replace `src/core/resources.rs` with:

```rust
use crate::core::units::ResourceAmount;

#[derive(Clone, Debug, PartialEq)]
pub struct ResourceGrid {
    layers: Vec<ResourceAmount>,
}

impl ResourceGrid {
    pub fn phase1_placeholder(initial: ResourceAmount) -> Self {
        Self { layers: vec![initial] }
    }

    pub fn layer_count(&self) -> usize {
        self.layers.len()
    }
}
```

Replace `src/core/environment.rs` with:

```rust
use crate::core::config::EnvironmentConfig;
use crate::core::units::{HeatAmount, WasteAmount};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct EnvironmentState {
    heat: HeatAmount,
    waste: WasteAmount,
}

impl EnvironmentState {
    pub fn from_config(config: &EnvironmentConfig) -> Self {
        Self {
            heat: config.heat_current,
            waste: config.waste_current,
        }
    }

    pub const fn heat(self) -> HeatAmount {
        self.heat
    }

    pub const fn waste(self) -> WasteAmount {
        self.waste
    }

    pub(crate) fn set_heat(&mut self, heat: HeatAmount) {
        self.heat = heat;
    }

    pub(crate) fn set_waste(&mut self, waste: WasteAmount) {
        self.waste = waste;
    }
}
```

Replace `src/core/spatial.rs` with:

```rust
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct SpatialIndex {
    rebuild_count: u64,
}

impl SpatialIndex {
    pub fn new() -> Self {
        Self { rebuild_count: 0 }
    }

    pub fn rebuild(&mut self) {
        self.rebuild_count += 1;
    }

    pub fn rebuild_count(&self) -> u64 {
        self.rebuild_count
    }
}
```

Replace `src/core/world.rs` with:

```rust
use crate::core::cell_store::{CellStore, EnergyBuffer, InitialCellState};
use crate::core::config::RuntimeConfig;
use crate::core::environment::EnvironmentState;
use crate::core::resources::ResourceGrid;
use crate::core::spatial::SpatialIndex;
use crate::core::units::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorldInitError {
    InvalidInitialState,
}

#[derive(Clone, Debug)]
pub struct WorldState {
    tick: Tick,
    config: RuntimeConfig,
    cells: CellStore,
    resources: ResourceGrid,
    environment: EnvironmentState,
    spatial_index: SpatialIndex,
}

impl WorldState {
    pub fn from_config(config: RuntimeConfig) -> Result<Self, WorldInitError> {
        let mut cells = CellStore::with_capacity(1);
        cells.insert_initial(InitialCellState {
            position: config.cell.position,
            radius: config.cell.radius,
            energy: EnergyBuffer::new(config.cell.initial_energy, config.cell.energy_capacity),
            resources: config.cell.initial_resource_amount,
            materials: config.cell.initial_material_amount,
            capacity_limit: config.cell.capacity_limit,
            temperature: crate::core::units::Temperature::new(25.0),
        });

        let mut spatial_index = SpatialIndex::new();
        spatial_index.rebuild();

        Ok(Self {
            tick: Tick::from_raw(0),
            config,
            cells,
            resources: ResourceGrid::phase1_placeholder(config.cell.initial_resource_amount),
            environment: EnvironmentState::from_config(&config.environment),
            spatial_index,
        })
    }

    pub const fn tick(&self) -> Tick {
        self.tick
    }

    pub fn config(&self) -> &RuntimeConfig {
        &self.config
    }

    pub fn cells(&self) -> &CellStore {
        &self.cells
    }

    pub fn cells_mut_for_commit(&mut self) -> &mut CellStore {
        &mut self.cells
    }

    pub fn resources(&self) -> &ResourceGrid {
        &self.resources
    }

    pub fn environment(&self) -> EnvironmentState {
        self.environment
    }

    pub fn environment_mut_for_commit(&mut self) -> &mut EnvironmentState {
        &mut self.environment
    }

    pub fn spatial_index(&self) -> &SpatialIndex {
        &self.spatial_index
    }

    pub(crate) fn advance_tick(&mut self) {
        self.tick = self.tick.next();
        self.spatial_index.rebuild();
    }
}
```

- [ ] **Step 4: Run test**

Run:

```bash
cargo test world_initializes_one_cell_from_config
```

Expected:

```text
test world_initializes_one_cell_from_config ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/world.rs src/core/resources.rs src/core/environment.rs src/core/spatial.rs tests/phase1_core_smoke.rs
git commit -m "feat: initialize phase 1 world state"
```

---

## Task 7: Lifecycle Evaluation

**Files:**

- Modify: `src/core/lifecycle.rs`
- Modify: `tests/phase1_accounting.rs`

- [ ] **Step 1: Write failing lifecycle tests**

Append to `tests/phase1_accounting.rs`:

```rust
use alife::core::lifecycle::{evaluate_lifecycle, LifecycleInput, LifecycleReason};

#[test]
fn lifecycle_priority_prefers_death_over_dormancy_and_stress() {
    let decision = evaluate_lifecycle(
        LifecycleInput {
            mandatory_paid: false,
            energy_after_mandatory: EnergyAmount::zero(),
            stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
            over_capacity: true,
            critical_capacity_exceeded: true,
            heat_warning: true,
            heat_death: true,
            waste_warning: true,
            waste_death: true,
            dormancy_allowed: true,
            dormant_cost_payable: true,
        },
    );

    assert_eq!(decision.state, LifecycleState::Dead);
    assert_eq!(decision.reason, LifecycleReason::EnergyDepleted);
}

#[test]
fn lifecycle_enters_dormancy_when_unpaid_but_dormant_cost_is_payable() {
    let decision = evaluate_lifecycle(
        LifecycleInput {
            mandatory_paid: false,
            energy_after_mandatory: EnergyAmount::new(1.0).unwrap(),
            stress_energy_threshold: EnergyAmount::new(3.0).unwrap(),
            over_capacity: false,
            critical_capacity_exceeded: false,
            heat_warning: false,
            heat_death: false,
            waste_warning: false,
            waste_death: false,
            dormancy_allowed: true,
            dormant_cost_payable: true,
        },
    );

    assert_eq!(decision.state, LifecycleState::Dormant);
    assert_eq!(decision.reason, LifecycleReason::Dormancy);
}
```

- [ ] **Step 2: Run tests to verify they fail**

Run:

```bash
cargo test lifecycle_priority lifecycle_enters_dormancy
```

Expected:

```text
FAIL: missing lifecycle module API
```

- [ ] **Step 3: Implement lifecycle**

Replace `src/core/lifecycle.rs` with:

```rust
use crate::core::cell_store::LifecycleState;
use crate::core::units::EnergyAmount;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LifecycleReason {
    None,
    EnergyDepleted,
    CapacityExceeded,
    HeatLimitExceeded,
    WasteLimitExceeded,
    Dormancy,
    Stress,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LifecycleDecision {
    pub state: LifecycleState,
    pub reason: LifecycleReason,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LifecycleInput {
    pub mandatory_paid: bool,
    pub energy_after_mandatory: EnergyAmount,
    pub stress_energy_threshold: EnergyAmount,
    pub over_capacity: bool,
    pub critical_capacity_exceeded: bool,
    pub heat_warning: bool,
    pub heat_death: bool,
    pub waste_warning: bool,
    pub waste_death: bool,
    pub dormancy_allowed: bool,
    pub dormant_cost_payable: bool,
}

pub fn evaluate_lifecycle(input: LifecycleInput) -> LifecycleDecision {
    if input.energy_after_mandatory.raw() <= 0.0 {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::EnergyDepleted,
        };
    }
    if input.critical_capacity_exceeded {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::CapacityExceeded,
        };
    }
    if input.heat_death {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::HeatLimitExceeded,
        };
    }
    if input.waste_death {
        return LifecycleDecision {
            state: LifecycleState::Dead,
            reason: LifecycleReason::WasteLimitExceeded,
        };
    }
    if !input.mandatory_paid && input.dormancy_allowed && input.dormant_cost_payable {
        return LifecycleDecision {
            state: LifecycleState::Dormant,
            reason: LifecycleReason::Dormancy,
        };
    }
    if !input.mandatory_paid
        || input.energy_after_mandatory.raw() < input.stress_energy_threshold.raw()
        || input.over_capacity
        || input.heat_warning
        || input.waste_warning
    {
        return LifecycleDecision {
            state: LifecycleState::Stressed,
            reason: LifecycleReason::Stress,
        };
    }

    LifecycleDecision {
        state: LifecycleState::Alive,
        reason: LifecycleReason::None,
    }
}
```

- [ ] **Step 4: Run lifecycle tests**

Run:

```bash
cargo test lifecycle_priority lifecycle_enters_dormancy
```

Expected:

```text
test lifecycle_priority_prefers_death_over_dormancy_and_stress ... ok
test lifecycle_enters_dormancy_when_unpaid_but_dormant_cost_is_payable ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/lifecycle.rs tests/phase1_accounting.rs
git commit -m "feat: add deterministic lifecycle evaluation"
```

---

## Task 8: TickExecutor Core Smoke

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Modify: `src/core/deltas.rs`
- Modify: `tests/phase1_accounting.rs`

- [ ] **Step 1: Write failing tick tests**

Append to `tests/phase1_accounting.rs`:

```rust
use alife::core::summary::{CollapseReason, SurvivalResult};
use alife::core::tick::TickExecutor;

#[test]
fn tick_executor_pays_mandatory_cost_and_advances_tick() {
    let mut executor = TickExecutor::new(valid_config()).unwrap();
    let summary = executor.step().unwrap();
    let cell = executor.world().cells().energy(CellIndex::from_raw(0));

    assert_eq!(executor.world().tick().raw(), 1);
    assert_eq!(cell.current().raw(), 10.0);
    assert_eq!(summary.survival_result, SurvivalResult::Stable);
    assert_eq!(summary.collapse_reason, CollapseReason::None);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test tick_executor_pays_mandatory_cost_and_advances_tick
```

Expected:

```text
FAIL: missing TickExecutor or RunSummary
```

- [ ] **Step 3: Implement summary, deltas and TickExecutor**

Replace `src/core/summary.rs` with:

```rust
use crate::core::units::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SurvivalResult {
    Stable,
    Fragile,
    Collapse,
    Invalid,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CollapseReason {
    None,
    InvalidConfig,
    EnergyDepleted,
    MandatoryCostUnpaid,
    CapacityExceeded,
    HeatLimitExceeded,
    WasteLimitExceeded,
    MinimumViabilityMaterialsMissing,
    DeterminismMismatch,
    ViewerAuthorityViolation,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct MetricsSummary {
    pub final_energy: f32,
    pub heat: f32,
    pub waste: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct RunSummary {
    pub tick: Tick,
    pub config_hash: u64,
    pub survival_result: SurvivalResult,
    pub collapse_reason: CollapseReason,
    pub metrics: MetricsSummary,
}
```

Replace `src/core/deltas.rs` with:

```rust
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CommitSummary {
    pub ticks_committed: u64,
    pub events_emitted: usize,
}
```

Replace `src/core/tick.rs` with:

```rust
use crate::core::cell_store::{CellIndex, EnergyBuffer, LifecycleState, RuntimeFlags};
use crate::core::config::RuntimeConfig;
use crate::core::deltas::CommitSummary;
use crate::core::lifecycle::{evaluate_lifecycle, LifecycleInput, LifecycleReason};
use crate::core::summary::{CollapseReason, MetricsSummary, RunSummary, SurvivalResult};
use crate::core::units::{EnergyAmount, HeatAmount, WasteAmount};
use crate::core::world::{WorldInitError, WorldState};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TickError {
    WorldInit(WorldInitError),
}

impl From<WorldInitError> for TickError {
    fn from(value: WorldInitError) -> Self {
        Self::WorldInit(value)
    }
}

pub struct TickExecutor {
    world: WorldState,
}

impl TickExecutor {
    pub fn new(config: RuntimeConfig) -> Result<Self, TickError> {
        Ok(Self {
            world: WorldState::from_config(config)?,
        })
    }

    pub fn world(&self) -> &WorldState {
        &self.world
    }

    pub fn step(&mut self) -> Result<RunSummary, TickError> {
        let config = *self.world.config();
        let index = CellIndex::from_raw(0);
        let current = self.world.cells().energy(index);
        let available = current
            .current()
            .saturating_add(config.cell.passive_energy_income);
        let mandatory_paid = available.raw() >= config.cell.mandatory_cost_per_tick.raw();
        let energy_after = if mandatory_paid {
            available.saturating_sub(config.cell.mandatory_cost_per_tick)
        } else {
            available
        }
        .clamp_max(current.capacity());

        let heat_next = HeatAmount::new(
            (self.world.environment().heat().raw()
                + config.environment.heat_generated_per_tick.raw()
                - config.environment.heat_dissipation_rate.raw())
            .max(0.0),
        )
        .expect("heat accounting is clamped");

        let waste_next = WasteAmount::new(
            (self.world.environment().waste().raw()
                + config.environment.waste_generated_per_tick.raw()
                - config.environment.waste_sink_rate.raw())
            .max(0.0),
        )
        .expect("waste accounting is clamped");

        let used_capacity = self.world.cells().used_capacity(index);
        let over_capacity = used_capacity.raw() > config.cell.capacity_limit.raw();
        let critical_capacity_exceeded = used_capacity.raw()
            > config.cell.capacity_limit.raw() + config.lifecycle.critical_capacity_overrun.raw();
        let dormant_cost = EnergyAmount::new(
            config.cell.mandatory_cost_per_tick.raw()
                * config.lifecycle.dormant_mandatory_cost_modifier,
        )
        .expect("dormant cost modifier is validated");

        let decision = evaluate_lifecycle(LifecycleInput {
            mandatory_paid,
            energy_after_mandatory: energy_after,
            stress_energy_threshold: config.lifecycle.stress_energy_threshold,
            over_capacity,
            critical_capacity_exceeded,
            heat_warning: heat_next.raw() > config.environment.heat_warning_threshold.raw(),
            heat_death: heat_next.raw() > config.environment.heat_death_threshold.raw(),
            waste_warning: waste_next.raw() > config.environment.waste_warning_threshold.raw(),
            waste_death: waste_next.raw() > config.environment.waste_death_threshold.raw(),
            dormancy_allowed: config.lifecycle.dormancy_allowed,
            dormant_cost_payable: available.raw() >= dormant_cost.raw(),
        });

        {
            let cells = self.world.cells_mut_for_commit();
            cells.set_energy(index, EnergyBuffer::new(energy_after, current.capacity()));
            cells.set_lifecycle_state(index, decision.state);
            cells.set_runtime_flags(
                index,
                RuntimeFlags {
                    mandatory_paid,
                    stalled: !mandatory_paid,
                    over_capacity,
                    inert: decision.state == LifecycleState::Dead,
                },
            );
        }
        self.world.environment_mut_for_commit().set_heat(heat_next);
        self.world.environment_mut_for_commit().set_waste(waste_next);
        self.world.advance_tick();

        let collapse_reason = match decision.reason {
            LifecycleReason::EnergyDepleted => CollapseReason::EnergyDepleted,
            LifecycleReason::CapacityExceeded => CollapseReason::CapacityExceeded,
            LifecycleReason::HeatLimitExceeded => CollapseReason::HeatLimitExceeded,
            LifecycleReason::WasteLimitExceeded => CollapseReason::WasteLimitExceeded,
            _ if !mandatory_paid => CollapseReason::MandatoryCostUnpaid,
            _ => CollapseReason::None,
        };

        let survival_result = if decision.state == LifecycleState::Dead {
            SurvivalResult::Collapse
        } else if decision.state == LifecycleState::Stressed || decision.state == LifecycleState::Dormant {
            SurvivalResult::Fragile
        } else {
            SurvivalResult::Stable
        };

        Ok(RunSummary {
            tick: self.world.tick(),
            config_hash: config.config_hash(),
            survival_result,
            collapse_reason,
            metrics: MetricsSummary {
                final_energy: energy_after.raw(),
                heat: heat_next.raw(),
                waste: waste_next.raw(),
            },
        })
    }

    pub fn run_until_configured_tick(&mut self) -> Result<RunSummary, TickError> {
        let target = self.world.config().world.tick_count.raw();
        let mut latest = self.step()?;
        while self.world.tick().raw() < target {
            latest = self.step()?;
            if latest.survival_result == SurvivalResult::Collapse {
                break;
            }
        }
        Ok(latest)
    }

    pub fn last_commit_summary(&self) -> CommitSummary {
        CommitSummary {
            ticks_committed: self.world.tick().raw(),
            events_emitted: 0,
        }
    }
}
```

- [ ] **Step 4: Run tests**

Run:

```bash
cargo test tick_executor_pays_mandatory_cost_and_advances_tick
```

Expected:

```text
test tick_executor_pays_mandatory_cost_and_advances_tick ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/tick.rs src/core/summary.rs src/core/deltas.rs tests/phase1_accounting.rs
git commit -m "feat: add phase 1 tick executor smoke"
```

---

## Task 9: Phase 1B Accounting Scenarios

**Files:**

- Modify: `tests/phase1_accounting.rs`
- Modify: `src/core/tick.rs` only if tests expose a behavior mismatch.

- [ ] **Step 1: Write failing accounting scenario tests**

Append to `tests/phase1_accounting.rs`:

```rust
#[test]
fn tick_executor_collapses_on_energy_depletion() {
    let mut config = valid_config();
    config.cell.initial_energy = EnergyAmount::new(0.5).unwrap();
    config.cell.passive_energy_income = EnergyAmount::zero();
    config.cell.mandatory_cost_per_tick = EnergyAmount::new(2.0).unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::EnergyDepleted);
}

#[test]
fn tick_executor_reports_heat_limit_collapse() {
    let mut config = valid_config();
    config.environment.heat_current = HeatAmount::new(19.5).unwrap();
    config.environment.heat_generated_per_tick = HeatAmount::new(2.0).unwrap();
    config.environment.heat_dissipation_rate = HeatAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::HeatLimitExceeded);
}

#[test]
fn tick_executor_reports_waste_limit_collapse() {
    let mut config = valid_config();
    config.environment.waste_current = WasteAmount::new(19.5).unwrap();
    config.environment.waste_generated_per_tick = WasteAmount::new(2.0).unwrap();
    config.environment.waste_sink_rate = WasteAmount::zero();

    let mut executor = TickExecutor::new(config).unwrap();
    let summary = executor.step().unwrap();

    assert_eq!(summary.survival_result, SurvivalResult::Collapse);
    assert_eq!(summary.collapse_reason, CollapseReason::WasteLimitExceeded);
}
```

- [ ] **Step 2: Run tests**

Run:

```bash
cargo test tick_executor_collapses tick_executor_reports_heat tick_executor_reports_waste
```

Expected:

```text
PASS if Task 8 already covers accounting correctly, otherwise FAIL with exact mismatch.
```

- [ ] **Step 3: Fix only observed mismatches**

If a mismatch appears, edit only `src/core/tick.rs` to preserve this priority:

```text
death by energy
death by critical capacity
death by heat
death by waste
dormancy
stress
alive
```

Do not add new mechanics.

- [ ] **Step 4: Run tests**

Run:

```bash
cargo test --test phase1_accounting
```

Expected:

```text
all tests in phase1_accounting pass
```

- [ ] **Step 5: Commit**

```bash
git add src/core/tick.rs tests/phase1_accounting.rs
git commit -m "test: cover phase 1 accounting collapse scenarios"
```

---

## Task 10: EventBuffer Without Success Spam

**Files:**

- Modify: `src/core/events.rs`
- Modify: `src/core/world.rs`
- Modify: `src/core/tick.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing event tests**

Append to `tests/phase1_core_smoke.rs`:

```rust
use alife::core::events::EventKind;

#[test]
fn successful_mandatory_cost_does_not_emit_paid_event_spam() {
    let mut executor = TickExecutor::new(valid_config()).unwrap();
    executor.step().unwrap();

    assert!(executor
        .world()
        .events()
        .iter_ordered()
        .all(|event| event.kind != EventKind::MandatoryCostFailed));
    assert_eq!(executor.world().events().len(), 1);
    assert_eq!(executor.world().events().iter_ordered().next().unwrap().kind, EventKind::TickCommitted);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test successful_mandatory_cost_does_not_emit_paid_event_spam
```

Expected:

```text
FAIL: missing events API
```

- [ ] **Step 3: Implement EventBuffer and wire it into WorldState/TickExecutor**

Replace `src/core/events.rs` with:

```rust
use crate::core::ids::{CellId, EventId};
use crate::core::units::Tick;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EventKind {
    RunStarted,
    TickCommitted,
    MandatoryCostFailed,
    LifecycleChanged,
    CapacityWarning,
    HeatWarning,
    WasteWarning,
    CellDead,
    SnapshotEmitted,
    RunFinished,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Event {
    pub id: EventId,
    pub tick: Tick,
    pub kind: EventKind,
    pub cell_id: Option<CellId>,
}

#[derive(Clone, Debug, Default)]
pub struct EventBuffer {
    events: Vec<Event>,
    next_event_id: u32,
}

impl EventBuffer {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            events: Vec::with_capacity(capacity),
            next_event_id: 1,
        }
    }

    pub fn push(&mut self, tick: Tick, kind: EventKind, cell_id: Option<CellId>) -> EventId {
        let id = EventId::from_raw(self.next_event_id);
        self.next_event_id += 1;
        self.events.push(Event { id, tick, kind, cell_id });
        id
    }

    pub fn len(&self) -> usize {
        self.events.len()
    }

    pub fn iter_ordered(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }
}
```

Update `WorldState` fields and accessors:

```rust
use crate::core::events::EventBuffer;

// add field
events: EventBuffer,

// in from_config
events: EventBuffer::with_capacity(128),

pub fn events(&self) -> &EventBuffer {
    &self.events
}

pub fn events_mut_for_commit(&mut self) -> &mut EventBuffer {
    &mut self.events
}
```

Update `TickExecutor::step` after `self.world.advance_tick();`:

```rust
let cell_id = self.world.cells().id_at(index);
if !mandatory_paid {
    self.world
        .events_mut_for_commit()
        .push(self.world.tick(), EventKind::MandatoryCostFailed, Some(cell_id));
}
if decision.state == LifecycleState::Dead {
    self.world
        .events_mut_for_commit()
        .push(self.world.tick(), EventKind::CellDead, Some(cell_id));
}
self.world
    .events_mut_for_commit()
    .push(self.world.tick(), EventKind::TickCommitted, None);
```

Also add:

```rust
use crate::core::events::EventKind;
```

- [ ] **Step 4: Run event test**

Run:

```bash
cargo test successful_mandatory_cost_does_not_emit_paid_event_spam
```

Expected:

```text
test successful_mandatory_cost_does_not_emit_paid_event_spam ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/events.rs src/core/world.rs src/core/tick.rs tests/phase1_core_smoke.rs
git commit -m "feat: add bounded phase 1 event buffer"
```

---

## Task 11: Snapshot And ViewerFrame Projection

**Files:**

- Modify: `src/core/snapshot.rs`
- Modify: `src/core/world.rs`
- Modify: `tests/phase1_core_smoke.rs`

- [ ] **Step 1: Write failing snapshot test**

Append to `tests/phase1_core_smoke.rs`:

```rust
use alife::core::snapshot::{CommittedSnapshot, ViewerFrame};

#[test]
fn snapshot_and_viewer_frame_are_read_only_projections() {
    let mut executor = TickExecutor::new(valid_config()).unwrap();
    executor.step().unwrap();

    let snapshot = CommittedSnapshot::from_world(executor.world());
    let frame = ViewerFrame::from_snapshot(&snapshot);

    assert_eq!(snapshot.tick.raw(), 1);
    assert_eq!(snapshot.cells.len(), 1);
    assert_eq!(frame.cells.len(), 1);
    assert_eq!(executor.world().tick().raw(), 1);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```bash
cargo test snapshot_and_viewer_frame_are_read_only_projections
```

Expected:

```text
FAIL: missing snapshot types
```

- [ ] **Step 3: Implement snapshot projection**

Replace `src/core/snapshot.rs` with:

```rust
use crate::core::cell_store::{CellIndex, LifecycleState};
use crate::core::ids::CellId;
use crate::core::units::{EnergyAmount, Position, Radius, Tick};
use crate::core::world::WorldState;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct CellSnapshot {
    pub id: CellId,
    pub position: Position,
    pub radius: Radius,
    pub energy: EnergyAmount,
    pub lifecycle_state: LifecycleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct CommittedSnapshot {
    pub tick: Tick,
    pub cells: Vec<CellSnapshot>,
    pub heat: f32,
    pub waste: f32,
}

impl CommittedSnapshot {
    pub fn from_world(world: &WorldState) -> Self {
        let cells = world
            .cells()
            .iter_indices()
            .map(|index| CellSnapshot {
                id: world.cells().id_at(index),
                position: world.cells().position(index),
                radius: Radius::new(1.0).expect("Phase 1 radius is validated at init"),
                energy: world.cells().energy(index).current(),
                lifecycle_state: world.cells().lifecycle_state(index),
            })
            .collect();

        Self {
            tick: world.tick(),
            cells,
            heat: world.environment().heat().raw(),
            waste: world.environment().waste().raw(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ViewerCell {
    pub id: CellId,
    pub position: Position,
    pub radius: Radius,
    pub lifecycle_state: LifecycleState,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ViewerFrame {
    pub tick: Tick,
    pub cells: Vec<ViewerCell>,
    pub heat: f32,
    pub waste: f32,
}

impl ViewerFrame {
    pub fn from_snapshot(snapshot: &CommittedSnapshot) -> Self {
        Self {
            tick: snapshot.tick,
            cells: snapshot
                .cells
                .iter()
                .map(|cell| ViewerCell {
                    id: cell.id,
                    position: cell.position,
                    radius: cell.radius,
                    lifecycle_state: cell.lifecycle_state,
                })
                .collect(),
            heat: snapshot.heat,
            waste: snapshot.waste,
        }
    }
}
```

If the compiler warns that `CellIndex` is unused, remove it from the import list. Do not add mutating snapshot methods.

- [ ] **Step 4: Run test**

Run:

```bash
cargo test snapshot_and_viewer_frame_are_read_only_projections
```

Expected:

```text
test snapshot_and_viewer_frame_are_read_only_projections ... ok
```

- [ ] **Step 5: Commit**

```bash
git add src/core/snapshot.rs tests/phase1_core_smoke.rs
git commit -m "feat: add read-only phase 1 snapshot projection"
```

---

## Task 12: Deterministic Replay

**Files:**

- Create: `tests/phase1_determinism.rs`

- [ ] **Step 1: Write deterministic replay test**

Create `tests/phase1_determinism.rs`:

```rust
use alife::core::config::{
    CellInitialConfig, EnvironmentConfig, LifecycleConfig, RuntimeConfig, SpaceConfig, WorldConfig,
};
use alife::core::tick::TickExecutor;
use alife::core::units::{
    CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount,
    Seed, Tick, WasteAmount, WorldSize,
};

fn deterministic_config() -> RuntimeConfig {
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(50),
            seed: Seed::from_raw(42),
            size: WorldSize::new(512.0, 512.0).unwrap(),
        },
        SpaceConfig { spatial_grid_size: 8.0 },
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
fn same_config_seed_and_binary_produce_same_summary_and_events() {
    let mut first = TickExecutor::new(deterministic_config()).unwrap();
    let mut second = TickExecutor::new(deterministic_config()).unwrap();

    let first_summary = first.run_until_configured_tick().unwrap();
    let second_summary = second.run_until_configured_tick().unwrap();

    let first_events: Vec<_> = first.world().events().iter_ordered().copied().collect();
    let second_events: Vec<_> = second.world().events().iter_ordered().copied().collect();

    assert_eq!(first_summary, second_summary);
    assert_eq!(first_events, second_events);
}
```

- [ ] **Step 2: Run test**

Run:

```bash
cargo test same_config_seed_and_binary_produce_same_summary_and_events
```

Expected:

```text
test same_config_seed_and_binary_produce_same_summary_and_events ... ok
```

- [ ] **Step 3: Commit**

```bash
git add tests/phase1_determinism.rs
git commit -m "test: add phase 1 deterministic replay check"
```

---

## Task 13: Main Runner Smoke

**Files:**

- Modify: `src/main.rs`

- [ ] **Step 1: Run current binary**

Run:

```bash
cargo run
```

Expected current output may be:

```text
ALife simulation started
```

- [ ] **Step 2: Replace main with thin runner smoke**

Replace `src/main.rs` with:

```rust
fn main() {
    println!("ALife Phase 1 core ready. Run `cargo test` for deterministic smoke checks.");
}
```

Do not add simulation rules to `main.rs`.

- [ ] **Step 3: Run binary**

Run:

```bash
cargo run
```

Expected:

```text
ALife Phase 1 core ready. Run `cargo test` for deterministic smoke checks.
```

- [ ] **Step 4: Commit**

```bash
git add src/main.rs
git commit -m "chore: keep main as thin phase 1 runner shell"
```

---

## Task 14: Full Verification And Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-core.md`

- [ ] **Step 1: Run formatting**

Run:

```bash
cargo fmt --check
```

Expected:

```text
success with no formatting diff
```

If it fails, run:

```bash
cargo fmt
```

Then rerun:

```bash
cargo fmt --check
```

- [ ] **Step 2: Run all Rust tests**

Run:

```bash
cargo test
```

Expected:

```text
all Rust tests pass
```

- [ ] **Step 3: Run Python stability tool tests to avoid accidental collateral damage**

Run:

```bash
python -m pytest .\tools\early-stability
```

Expected:

```text
all early-stability tests pass
```

- [ ] **Step 4: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-core.md` with:

```markdown
# REPORT: Phase 1 Rust Core

## Goal

Implemented the first deterministic Phase 1 Rust core vertical slice.

## Scope

- Core module skeleton under `src/core/`.
- Typed ids and typed accounting units.
- Validated `RuntimeConfig`.
- One-cell `WorldState` and `CellStore`.
- Phase 1 mandatory Energy accounting.
- Heat/waste/capacity lifecycle checks.
- EventBuffer without success-event spam.
- Read-only snapshot/viewer projection.
- Deterministic replay test.

## Verification

```text
cargo fmt --check
cargo test
python -m pytest .\tools\early-stability
```

## Open Questions

- Full TOML runner parsing remains outside this Phase 1 core slice.
- Real viewer/server remains outside this Phase 1 core slice.
```

- [ ] **Step 5: Commit report**

```bash
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-1-rust-core.md
git commit -m "docs: report phase 1 rust core implementation"
```

---

## Acceptance Gates

Implementation is accepted only when:

```text
cargo fmt --check passes
cargo test passes
python -m pytest .\tools\early-stability passes
one Cell survives predictably
one Cell dies predictably
Energy Buffer clamps at capacity
capacity accounting excludes Energy and includes Resources/Materials
heat and waste collapse paths are tested
same config + seed + binary produces same summary/events
snapshot/viewer projection cannot mutate WorldState
default events do not include MandatoryCostPaid spam
no Genome/Joint/Process/division/mutation code is introduced
```

---

## Self-Review

Spec coverage:

- Phase 1A core smoke is covered by Tasks 1-8.
- Phase 1B accounting is covered by Tasks 5, 7, 8 and 9.
- Phase 1C determinism and outputs are covered by Tasks 10-12.
- Phase 1D compatibility is covered by summary vocabulary, collapse reasons and stability-tool verification in Task 14.

Risk notes:

- The plan intentionally uses standard library only. TOML parsing and CLI scenario loading are not part of this slice.
- `ResourceGrid` is a placeholder boundary, not final metabolism or diffusion.
- `SpatialIndex` is a placeholder counter boundary in this slice; scalable prefix-sum implementation is a later focused task after the one-cell smoke is stable.
- The provided code is minimal by design. Do not add Phase 2+ concepts while executing this plan.
