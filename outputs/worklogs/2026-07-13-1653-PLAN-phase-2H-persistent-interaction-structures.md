# Phase 2H Persistent Interaction Structures Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:subagent-driven-development` or `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Use `test-driven-development` for every behavior change: write the failing test, run it and verify the expected failure, then implement minimal code.

**Goal:** Add persistent, material-backed local `Joint` structures with mechanical, Resource, scalar Signal and Heat channels before Genome Runtime can regulate them.

**Architecture:** `JointStore` is World-owned hot state with typed `JointId` and SoA-style endpoint/channel arrays. Transient `ContactCache` remains only an eligibility input; observer/analyzer projections read committed joint state and never affect behavior. Tick execution applies joints through deterministic collect/commit phases, with explicit accounting and no direct `Energy Buffer`, Genome or organism-controller transfer.

**Tech Stack:** Rust core (`src/core`), TOML runtime/parser configs (`config`, `src/runner/config_parser.rs`), observer projection (`src/observer`), analyzer binary (`src/bin/sweep_analyzer.rs`), Rust integration tests (`tests/phase2h_*.rs`).

---

## Source Constraints

Must preserve:

- `docs/PRINCIPLES.md`: no hardcoded species, organs, brains, role classes, organism controllers or observer-driven behavior.
- `docs/biology/joint.md`: Joint is local, material/cost backed, can transfer Resources/Signals/Heat/mechanical force, never transfers `Energy Buffer` or Genome, degrades/breaks, death disables living channels, division breaks external joints by default.
- `docs/biology/communication.md`: base signal is scalar, not a command; signal emitted in Tick N is readable no earlier than Tick N+1.
- `docs/biology/division-partition.md`: external Joints are not copied or silently preserved during division.
- `docs/world/tick-semantics.md`: no same-phase feedback or behavior based on partial writes.
- `docs/engine/performance.md`: use separate `JointStore`, stable ordering, no `Rc<RefCell<_>>`, no hot `HashMap<CellId, Cell>`.

## File Structure

Create:

- `src/core/joints.rs` — `JointId`, `JointStore`, channel configs/state, deterministic creation/break/degradation helpers.
- `tests/phase2h_joint_store.rs` — storage invariants, stable IDs, endpoint ordering, active/inert/broken transitions.
- `tests/phase2h_joint_creation.rs` — local material/cost gated creation and rejection cases.
- `tests/phase2h_joint_channels.rs` — passive Resource, delayed scalar Signal and Heat channel behavior.
- `tests/phase2h_joint_lifecycle.rs` — death and division behavior.
- `tests/phase2h_observer_outputs.rs` — projection features and observer-only `OrganismView` connected components.
- `tests/phase2h_sweep_parser.rs` — analyzer config includes full and smoke Phase 2H sweeps.
- `tests/phase2h_reachability.rs` — raw CSV scenarios prove all Phase 2H reachability gates.
- `config/scenarios/joints/phase2h.toml` — canonical scenario fixture for parser/config tests.

Modify:

- `src/core/ids.rs` — add typed `JointId`.
- `src/core/mod.rs` — expose `joints`.
- `src/core/config.rs` — add `JointConfig` under `RuntimeConfig`.
- `src/runner/config_parser.rs` — parse `[joints]` and phase2h scenario fields.
- `src/core/process.rs` — add `ProcessId::JointCreate`, `ProcessId::JointRepair` and rejection reasons.
- `src/core/world.rs` — own `JointStore`, expose accessors, validate joint feasibility, break joints on division/death.
- `src/core/tick.rs` — schedule joint creation, mechanics, Resource channel, signal write/read, Heat channel, degradation/break, metrics.
- `src/core/summary.rs` — add joint metrics.
- `src/observer/projection.rs` — expose joint metrics and optional observer-only connected-component features.
- `src/bin/sweep_analyzer.rs` — phase2h runtime setup, raw columns, warnings, full and smoke scenarios.
- `config/analyzer/sweep_analyzer.toml` — full Phase 2H sweeps.
- `config/analyzer/sweep_analyzer_smoke.toml` — minimal fast Phase 2H sweeps.
- `outputs/worklogs/index.md` — add this plan link after implementation starts or before final report.

---

## Task 1: Joint Identity And Store

**Files:**

- Modify: `src/core/ids.rs`
- Create: `src/core/joints.rs`
- Modify: `src/core/mod.rs`
- Test: `tests/phase2h_joint_store.rs`

- [ ] **Step 1: Write failing store tests**

Add `tests/phase2h_joint_store.rs`:

```rust
use alife::core::cell_store::CellIndex;
use alife::core::ids::JointId;
use alife::core::joints::{JointChannelConfig, JointEndpoints, JointStore};
use alife::core::units::{MaterialAmount, Tick};

#[test]
fn joint_store_allocates_stable_ids_and_orders_endpoints() {
    let mut joints = JointStore::with_capacity(4);
    let cfg = JointChannelConfig::mechanical_only(1.0);

    let first = joints.create(
        JointEndpoints::new(CellIndex::from_raw(3), CellIndex::from_raw(1)).unwrap(),
        MaterialAmount::new(2.0).unwrap(),
        cfg,
        Tick::from_raw(7),
    );
    let second = joints.create(
        JointEndpoints::new(CellIndex::from_raw(2), CellIndex::from_raw(1)).unwrap(),
        MaterialAmount::new(1.0).unwrap(),
        cfg,
        Tick::from_raw(8),
    );

    assert_eq!(first, JointId::from_raw(0));
    assert_eq!(second, JointId::from_raw(1));
    assert_eq!(joints.endpoints(first).unwrap().a, CellIndex::from_raw(1));
    assert_eq!(joints.endpoints(first).unwrap().b, CellIndex::from_raw(3));
    assert_eq!(joints.active_ids().collect::<Vec<_>>(), vec![first, second]);
}

#[test]
fn joint_store_rejects_self_endpoint_and_keeps_broken_joint_material() {
    assert!(JointEndpoints::new(CellIndex::from_raw(2), CellIndex::from_raw(2)).is_none());

    let mut joints = JointStore::with_capacity(1);
    let id = joints.create(
        JointEndpoints::new(CellIndex::from_raw(0), CellIndex::from_raw(1)).unwrap(),
        MaterialAmount::new(3.0).unwrap(),
        JointChannelConfig::mechanical_only(1.0),
        Tick::from_raw(0),
    );

    joints.break_joint(id, Tick::from_raw(3)).unwrap();

    assert!(!joints.is_active(id).unwrap());
    assert!(joints.is_broken(id).unwrap());
    assert_eq!(joints.material_amount(id).unwrap().raw(), 3.0);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_store -- --nocapture
```

Expected: FAIL because `JointId`, `core::joints`, `JointStore`, `JointEndpoints` and `JointChannelConfig` are missing.

- [ ] **Step 3: Implement minimal store**

In `src/core/ids.rs` add:

```rust
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JointId(u32);

impl_id!(JointId);
```

In `src/core/mod.rs` add:

```rust
pub mod joints;
```

Create `src/core/joints.rs`:

```rust
use crate::core::cell_store::CellIndex;
use crate::core::ids::JointId;
use crate::core::units::{MaterialAmount, Tick};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct JointEndpoints {
    pub a: CellIndex,
    pub b: CellIndex,
}

impl JointEndpoints {
    pub fn new(a: CellIndex, b: CellIndex) -> Option<Self> {
        if a == b {
            return None;
        }
        Some(if a.raw() < b.raw() {
            Self { a, b }
        } else {
            Self { a: b, b: a }
        })
    }

    pub fn contains(self, cell: CellIndex) -> bool {
        self.a == cell || self.b == cell
    }

    pub fn other(self, cell: CellIndex) -> Option<CellIndex> {
        if self.a == cell {
            Some(self.b)
        } else if self.b == cell {
            Some(self.a)
        } else {
            None
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointChannelConfig {
    pub mechanical_strength: f32,
    pub resource_transfer_rate: f32,
    pub max_resource_transfer_per_tick: f32,
    pub signal_conductivity: f32,
    pub signal_decay: f32,
    pub heat_conductivity: f32,
}

impl JointChannelConfig {
    pub const fn mechanical_only(mechanical_strength: f32) -> Self {
        Self {
            mechanical_strength,
            resource_transfer_rate: 0.0,
            max_resource_transfer_per_tick: 0.0,
            signal_conductivity: 0.0,
            signal_decay: 0.0,
            heat_conductivity: 0.0,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JointLifecycle {
    Active,
    Inert,
    Broken,
}

#[derive(Clone, Debug, Default)]
pub struct JointStore {
    endpoints: Vec<JointEndpoints>,
    material_amounts: Vec<MaterialAmount>,
    configs: Vec<JointChannelConfig>,
    lifecycle: Vec<JointLifecycle>,
    damage: Vec<f32>,
    created_tick: Vec<Tick>,
    broken_tick: Vec<Option<Tick>>,
}

impl JointStore {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            endpoints: Vec::with_capacity(capacity),
            material_amounts: Vec::with_capacity(capacity),
            configs: Vec::with_capacity(capacity),
            lifecycle: Vec::with_capacity(capacity),
            damage: Vec::with_capacity(capacity),
            created_tick: Vec::with_capacity(capacity),
            broken_tick: Vec::with_capacity(capacity),
        }
    }

    pub fn create(
        &mut self,
        endpoints: JointEndpoints,
        material_amount: MaterialAmount,
        config: JointChannelConfig,
        tick: Tick,
    ) -> JointId {
        let id = JointId::from_raw(self.endpoints.len() as u32);
        self.endpoints.push(endpoints);
        self.material_amounts.push(material_amount);
        self.configs.push(config);
        self.lifecycle.push(JointLifecycle::Active);
        self.damage.push(0.0);
        self.created_tick.push(tick);
        self.broken_tick.push(None);
        id
    }

    pub fn len(&self) -> usize {
        self.endpoints.len()
    }

    pub fn endpoints(&self, id: JointId) -> Option<JointEndpoints> {
        self.endpoints.get(id.raw() as usize).copied()
    }

    pub fn config(&self, id: JointId) -> Option<JointChannelConfig> {
        self.configs.get(id.raw() as usize).copied()
    }

    pub fn material_amount(&self, id: JointId) -> Option<MaterialAmount> {
        self.material_amounts.get(id.raw() as usize).copied()
    }

    pub fn is_active(&self, id: JointId) -> Option<bool> {
        self.lifecycle
            .get(id.raw() as usize)
            .map(|state| *state == JointLifecycle::Active)
    }

    pub fn is_broken(&self, id: JointId) -> Option<bool> {
        self.lifecycle
            .get(id.raw() as usize)
            .map(|state| *state == JointLifecycle::Broken)
    }

    pub fn active_ids(&self) -> impl Iterator<Item = JointId> + '_ {
        self.lifecycle.iter().enumerate().filter_map(|(index, state)| {
            (*state == JointLifecycle::Active).then_some(JointId::from_raw(index as u32))
        })
    }

    pub fn all_ids(&self) -> impl Iterator<Item = JointId> + '_ {
        (0..self.endpoints.len()).map(|index| JointId::from_raw(index as u32))
    }

    pub fn break_joint(&mut self, id: JointId, tick: Tick) -> Result<(), JointStoreError> {
        let index = id.raw() as usize;
        let Some(state) = self.lifecycle.get_mut(index) else {
            return Err(JointStoreError::UnknownJoint);
        };
        *state = JointLifecycle::Broken;
        self.broken_tick[index] = Some(tick);
        Ok(())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum JointStoreError {
    UnknownJoint,
}
```

- [ ] **Step 4: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_store -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/core/ids.rs src/core/mod.rs src/core/joints.rs tests/phase2h_joint_store.rs
git commit -m "feat: add phase 2h joint store"
```

---

## Task 2: Runtime Joint Configuration And Parser

**Files:**

- Modify: `src/core/config.rs`
- Modify: `src/runner/config_parser.rs`
- Create: `config/scenarios/joints/phase2h.toml`
- Test: `tests/phase2h_sweep_parser.rs`

- [ ] **Step 1: Write failing config/parser tests**

Add `tests/phase2h_sweep_parser.rs`:

```rust
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parses_phase2h_joint_config_from_toml() {
    let toml = r#"
        [joints]
        enabled = true
        creation_distance_margin = 0.25
        creation_material_cost = 1.5
        creation_resource_cost = 2.0
        creation_energy_cost = 0.5
        upkeep_material_decay_per_tick = 0.02
        break_damage_threshold = 1.0
        max_joints_per_cell = 3
        mechanical_strength = 0.4
        resource_transfer_rate = 0.5
        max_resource_transfer_per_tick = 1.0
        signal_conductivity = 0.75
        signal_decay = 0.2
        heat_conductivity = 0.3
    "#;

    let raw: RawScenarioConfig = toml::from_str(toml).unwrap();
    let config = raw.into_runtime_config().unwrap();

    assert!(config.joints.enabled);
    assert_eq!(config.joints.creation_material_cost.raw(), 1.5);
    assert_eq!(config.joints.max_joints_per_cell, 3);
    assert_eq!(config.joints.signal_decay, 0.2);
}

#[test]
fn analyzer_configs_include_phase2h_full_and_smoke_scenarios() {
    let full = std::fs::read_to_string("config/analyzer/sweep_analyzer.toml").unwrap();
    let smoke = std::fs::read_to_string("config/analyzer/sweep_analyzer_smoke.toml").unwrap();

    for scenario in [
        "joint_creation_viability",
        "joint_resource_channel",
        "joint_signal_delay",
        "joint_heat_channel",
        "joint_degradation_break",
        "joint_lifecycle_division",
    ] {
        assert!(full.contains(scenario), "missing full scenario {scenario}");
        assert!(smoke.contains(scenario), "missing smoke scenario {scenario}");
    }
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_sweep_parser -- --nocapture
```

Expected: FAIL because `RuntimeConfig::joints`, raw `[joints]` parsing and analyzer scenarios are missing.

- [ ] **Step 3: Add config structs**

In `src/core/config.rs` add:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct JointConfig {
    pub enabled: bool,
    pub creation_distance_margin: f32,
    pub creation_material_cost: MaterialAmount,
    pub creation_resource_cost: ResourceAmount,
    pub creation_energy_cost: EnergyAmount,
    pub upkeep_material_decay_per_tick: f32,
    pub break_damage_threshold: f32,
    pub max_joints_per_cell: u32,
    pub mechanical_strength: f32,
    pub resource_transfer_rate: f32,
    pub max_resource_transfer_per_tick: ResourceAmount,
    pub signal_conductivity: f32,
    pub signal_decay: f32,
    pub heat_conductivity: f32,
}

impl Default for JointConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            creation_distance_margin: 0.25,
            creation_material_cost: MaterialAmount::new_unchecked(1.0),
            creation_resource_cost: ResourceAmount::zero(),
            creation_energy_cost: EnergyAmount::zero(),
            upkeep_material_decay_per_tick: 0.0,
            break_damage_threshold: 1.0,
            max_joints_per_cell: 4,
            mechanical_strength: 0.25,
            resource_transfer_rate: 0.0,
            max_resource_transfer_per_tick: ResourceAmount::zero(),
            signal_conductivity: 0.0,
            signal_decay: 0.0,
            heat_conductivity: 0.0,
        }
    }
}
```

Add `pub joints: JointConfig` to `RuntimeConfig`, initialize it in `Default`, validate finite non-negative values in `RuntimeConfig::validate`, and include the fields in `config_hash`.

- [ ] **Step 4: Parse `[joints]`**

In `src/runner/config_parser.rs` add:

```rust
#[derive(Debug, Clone, Default, Deserialize)]
pub struct RawJointConfig {
    pub enabled: Option<bool>,
    pub creation_distance_margin: Option<f32>,
    pub creation_material_cost: Option<f32>,
    pub creation_resource_cost: Option<f32>,
    pub creation_energy_cost: Option<f32>,
    pub upkeep_material_decay_per_tick: Option<f32>,
    pub break_damage_threshold: Option<f32>,
    pub max_joints_per_cell: Option<u32>,
    pub mechanical_strength: Option<f32>,
    pub resource_transfer_rate: Option<f32>,
    pub max_resource_transfer_per_tick: Option<f32>,
    pub signal_conductivity: Option<f32>,
    pub signal_decay: Option<f32>,
    pub heat_conductivity: Option<f32>,
}
```

Add `pub joints: Option<RawJointConfig>` to `RawScenarioConfig` and map it in `into_runtime_config()`:

```rust
if let Some(raw) = self.joints {
    runtime_config.joints.enabled = raw.enabled.unwrap_or(runtime_config.joints.enabled);
    runtime_config.joints.creation_distance_margin = raw
        .creation_distance_margin
        .unwrap_or(runtime_config.joints.creation_distance_margin);
    runtime_config.joints.creation_material_cost = MaterialAmount::new(
        raw.creation_material_cost
            .unwrap_or(runtime_config.joints.creation_material_cost.raw()),
    )
    .map_err(|e| ParseError::InvalidValue(format!("Invalid joints creation_material_cost: {e:?}")))?;
    runtime_config.joints.creation_resource_cost = ResourceAmount::new(
        raw.creation_resource_cost
            .unwrap_or(runtime_config.joints.creation_resource_cost.raw()),
    )
    .map_err(|e| ParseError::InvalidValue(format!("Invalid joints creation_resource_cost: {e:?}")))?;
    runtime_config.joints.creation_energy_cost = EnergyAmount::new(
        raw.creation_energy_cost
            .unwrap_or(runtime_config.joints.creation_energy_cost.raw()),
    )
    .map_err(|e| ParseError::InvalidValue(format!("Invalid joints creation_energy_cost: {e:?}")))?;
    runtime_config.joints.upkeep_material_decay_per_tick = raw
        .upkeep_material_decay_per_tick
        .unwrap_or(runtime_config.joints.upkeep_material_decay_per_tick);
    runtime_config.joints.break_damage_threshold = raw
        .break_damage_threshold
        .unwrap_or(runtime_config.joints.break_damage_threshold);
    runtime_config.joints.max_joints_per_cell =
        raw.max_joints_per_cell.unwrap_or(runtime_config.joints.max_joints_per_cell);
    runtime_config.joints.mechanical_strength =
        raw.mechanical_strength.unwrap_or(runtime_config.joints.mechanical_strength);
    runtime_config.joints.resource_transfer_rate = raw
        .resource_transfer_rate
        .unwrap_or(runtime_config.joints.resource_transfer_rate);
    runtime_config.joints.max_resource_transfer_per_tick = ResourceAmount::new(
        raw.max_resource_transfer_per_tick
            .unwrap_or(runtime_config.joints.max_resource_transfer_per_tick.raw()),
    )
    .map_err(|e| ParseError::InvalidValue(format!("Invalid joints max_resource_transfer_per_tick: {e:?}")))?;
    runtime_config.joints.signal_conductivity =
        raw.signal_conductivity.unwrap_or(runtime_config.joints.signal_conductivity);
    runtime_config.joints.signal_decay =
        raw.signal_decay.unwrap_or(runtime_config.joints.signal_decay);
    runtime_config.joints.heat_conductivity =
        raw.heat_conductivity.unwrap_or(runtime_config.joints.heat_conductivity);
}
```

- [ ] **Step 5: Add scenario fixtures**

Create `config/scenarios/joints/phase2h.toml`:

```toml
[joints]
enabled = true
creation_distance_margin = 0.25
creation_material_cost = 1.0
creation_resource_cost = 0.5
creation_energy_cost = 0.25
upkeep_material_decay_per_tick = 0.01
break_damage_threshold = 1.0
max_joints_per_cell = 4
mechanical_strength = 0.35
resource_transfer_rate = 0.5
max_resource_transfer_per_tick = 1.0
signal_conductivity = 0.75
signal_decay = 0.1
heat_conductivity = 0.25
```

Add the six scenario names from the test to both analyzer TOMLs with `steps = 2` in smoke.

- [ ] **Step 6: Run GREEN**

Run:

```bash
cargo test --test phase2h_sweep_parser -- --nocapture
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/core/config.rs src/runner/config_parser.rs config/scenarios/joints/phase2h.toml config/analyzer/sweep_analyzer.toml config/analyzer/sweep_analyzer_smoke.toml tests/phase2h_sweep_parser.rs
git commit -m "feat: add phase 2h joint config"
```

---

## Task 3: World Ownership And Material-Costed Joint Creation

**Files:**

- Modify: `src/core/process.rs`
- Modify: `src/core/world.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/phase2h_joint_creation.rs`

- [ ] **Step 1: Write failing creation tests**

Add `tests/phase2h_joint_creation.rs`:

```rust
use alife::core::cell_store::CellIndex;
use alife::core::config::RuntimeConfig;
use alife::core::tick::TickExecutor;

fn two_touching_cells_config() -> RuntimeConfig {
    let mut config = RuntimeConfig::default();
    config.joints.enabled = true;
    config.joints.creation_material_cost = alife::core::units::MaterialAmount::new(1.0).unwrap();
    config.joints.creation_resource_cost = alife::core::units::ResourceAmount::new(0.0).unwrap();
    config.joints.creation_energy_cost = alife::core::units::EnergyAmount::new(0.0).unwrap();
    config.joints.resource_transfer_rate = 0.0;
    config.local_interaction.enabled = true;
    config.world.size = alife::core::units::WorldSize::new(64.0, 64.0).unwrap();
    config.initial_cells = vec![
        alife::core::config::CellInitialConfig {
            position: alife::core::units::Position::new(30.0, 32.0),
            radius: alife::core::units::Radius::new(2.0).unwrap(),
            initial_boundary_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_transport_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_structural_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            ..Default::default()
        },
        alife::core::config::CellInitialConfig {
            position: alife::core::units::Position::new(33.9, 32.0),
            radius: alife::core::units::Radius::new(2.0).unwrap(),
            initial_boundary_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_transport_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_structural_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            ..Default::default()
        },
    ];
    config
}

#[test]
fn joint_creation_requires_local_contact_and_consumes_material_cost() {
    let mut exec = TickExecutor::new(two_touching_cells_config()).unwrap();
    let before_a = exec.world().cells().structural_material(CellIndex::from_raw(0)).raw();
    let before_b = exec.world().cells().structural_material(CellIndex::from_raw(1)).raw();

    let summary = exec.step().unwrap();

    assert_eq!(exec.world().joints().len(), 1);
    assert_eq!(summary.metrics.joint_created_count, 1);
    assert!(exec.world().cells().structural_material(CellIndex::from_raw(0)).raw() < before_a);
    assert!(exec.world().cells().structural_material(CellIndex::from_raw(1)).raw() < before_b);
}

#[test]
fn joint_creation_rejects_distant_or_material_free_cells() {
    let mut distant = two_touching_cells_config();
    distant.initial_cells[1].position = alife::core::units::Position::new(50.0, 32.0);
    let mut exec = TickExecutor::new(distant).unwrap();
    let summary = exec.step().unwrap();
    assert_eq!(exec.world().joints().len(), 0);
    assert_eq!(summary.metrics.joint_creation_rejected_count, 1);

    let mut material_free = two_touching_cells_config();
    material_free.initial_cells[0].initial_structural_material =
        alife::core::units::MaterialAmount::zero();
    let mut exec = TickExecutor::new(material_free).unwrap();
    let summary = exec.step().unwrap();
    assert_eq!(exec.world().joints().len(), 0);
    assert_eq!(summary.metrics.joint_creation_rejected_count, 1);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_creation -- --nocapture
```

Expected: FAIL because `WorldState::joints`, joint metrics and creation logic are missing.

- [ ] **Step 3: Add process IDs and feasibility reasons**

In `src/core/process.rs` add:

```rust
JointCreate,
JointRepair,
```

to `ProcessId`, and add rejection reasons:

```rust
JointNotLocal,
JointEndpointLimitReached,
JointAlreadyExists,
InsufficientMaterial,
```

Register `JointCreate` with required capabilities `BoundaryPermeability` and `StructuralGrowth`. Register `JointRepair` with `Repair`.

- [ ] **Step 4: Own JointStore in WorldState**

In `src/core/world.rs` add field:

```rust
joints: crate::core::joints::JointStore,
```

Initialize:

```rust
joints: crate::core::joints::JointStore::with_capacity(config.initial_cells.len().saturating_mul(2)),
```

Add accessors:

```rust
pub fn joints(&self) -> &crate::core::joints::JointStore {
    &self.joints
}

pub fn joints_mut_for_commit(&mut self) -> &mut crate::core::joints::JointStore {
    &mut self.joints
}
```

- [ ] **Step 5: Implement minimal creation in TickExecutor**

In `src/core/tick.rs`, after contact cache rebuild and before active cell process loop, add a deterministic joint creation pass:

```rust
let mut joint_created_count = 0_u32;
let mut joint_creation_rejected_count = 0_u32;

if config.joints.enabled {
    let pairs = self.world.contact_cache().pairs().to_vec();
    for pair in pairs {
        let Some(endpoints) = crate::core::joints::JointEndpoints::new(pair.a, pair.b) else {
            joint_creation_rejected_count += 1;
            continue;
        };
        if self.world.joints().has_active_between(endpoints) {
            joint_creation_rejected_count += 1;
            continue;
        }
        if pair.overlap + config.joints.creation_distance_margin < 0.0 {
            joint_creation_rejected_count += 1;
            continue;
        }
        let cost_each = config.joints.creation_material_cost.raw() * 0.5;
        let a_structural = self.world.cells().structural_material(pair.a).raw();
        let b_structural = self.world.cells().structural_material(pair.b).raw();
        if a_structural < cost_each || b_structural < cost_each {
            joint_creation_rejected_count += 1;
            continue;
        }

        {
            let cells = self.world.cells_mut_for_commit();
            cells.set_structural_material(
                pair.a,
                MaterialAmount::new_unchecked(a_structural - cost_each),
            );
            cells.set_structural_material(
                pair.b,
                MaterialAmount::new_unchecked(b_structural - cost_each),
            );
        }
        self.world.joints_mut_for_commit().create(
            endpoints,
            config.joints.creation_material_cost,
            crate::core::joints::JointChannelConfig {
                mechanical_strength: config.joints.mechanical_strength,
                resource_transfer_rate: config.joints.resource_transfer_rate,
                max_resource_transfer_per_tick: config.joints.max_resource_transfer_per_tick.raw(),
                signal_conductivity: config.joints.signal_conductivity,
                signal_decay: config.joints.signal_decay,
                heat_conductivity: config.joints.heat_conductivity,
            },
            self.world.tick(),
        );
        joint_created_count += 1;
    }
}
```

Add `has_active_between` to `JointStore`:

```rust
pub fn has_active_between(&self, endpoints: JointEndpoints) -> bool {
    self.active_ids()
        .any(|id| self.endpoints(id) == Some(endpoints))
}
```

- [ ] **Step 6: Add metrics**

In `src/core/summary.rs` add to `MetricsSummary`:

```rust
pub joint_count: u32,
pub joint_created_count: u32,
pub joint_creation_rejected_count: u32,
pub joint_broken_count: u32,
pub joint_resource_transfer_amount: f32,
pub joint_signal_generated_total: f32,
pub joint_signal_readable_total: f32,
pub joint_heat_transfer_amount: f32,
pub joint_degradation_amount: f32,
```

Initialize all existing constructors/tests with zeroes. In `build_metrics_summary`, set `joint_count` from `self.world.joints().active_ids().count()`.

- [ ] **Step 7: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_creation -- --nocapture
```

Expected: PASS.

- [ ] **Step 8: Commit**

```bash
git add src/core/process.rs src/core/world.rs src/core/tick.rs src/core/summary.rs src/core/joints.rs tests/phase2h_joint_creation.rs
git commit -m "feat: create material backed joints"
```

---

## Task 4: Mechanical Constraint

**Files:**

- Modify: `src/core/joints.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/phase2h_joint_channels.rs`

- [ ] **Step 1: Add failing mechanical test**

Append to `tests/phase2h_joint_channels.rs`:

```rust
#[test]
fn joint_mechanical_constraint_limits_endpoint_separation() {
    let mut config = two_touching_cells_config();
    config.joints.mechanical_strength = 1.0;
    config.joints.creation_material_cost = alife::core::units::MaterialAmount::new(0.1).unwrap();
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_position(CellIndex::from_raw(1), alife::core::units::Position::new(45.0, 32.0));

    let summary = exec.step().unwrap();

    let a = exec.world().cells().position(CellIndex::from_raw(0));
    let b = exec.world().cells().position(CellIndex::from_raw(1));
    let dx = a.x() - b.x();
    let dy = a.y() - b.y();
    let distance = (dx * dx + dy * dy).sqrt();
    assert!(distance < 15.0);
    assert!(summary.metrics.joint_mechanical_correction_amount > 0.0);
}
```

Also add `joint_mechanical_correction_amount: f32` to `MetricsSummary`.

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_channels joint_mechanical_constraint_limits_endpoint_separation -- --nocapture
```

Expected: FAIL because no joint mechanical pass exists.

- [ ] **Step 3: Implement deterministic correction**

In `src/core/tick.rs`, after ordinary overlap physics, add:

```rust
let mut joint_mechanical_correction_amount = 0.0_f32;
if config.joints.enabled {
    let joint_ids = self.world.joints().active_ids().collect::<Vec<_>>();
    for joint_id in joint_ids {
        let endpoints = self.world.joints().endpoints(joint_id).unwrap();
        let channel = self.world.joints().config(joint_id).unwrap();
        if channel.mechanical_strength <= 0.0 {
            continue;
        }
        let pos_a = self.world.cells().position(endpoints.a);
        let pos_b = self.world.cells().position(endpoints.b);
        let dx = pos_b.x() - pos_a.x();
        let dy = pos_b.y() - pos_a.y();
        let distance = (dx * dx + dy * dy).sqrt();
        let rest = self.world.cells().radius(endpoints.a).raw()
            + self.world.cells().radius(endpoints.b).raw();
        if distance <= rest || distance <= 0.001 {
            continue;
        }
        let correction = (distance - rest) * channel.mechanical_strength.clamp(0.0, 1.0) * 0.5;
        let nx = dx / distance;
        let ny = dy / distance;
        let cells = self.world.cells_mut_for_commit();
        cells.set_position(endpoints.a, Position::new(pos_a.x() + nx * correction, pos_a.y() + ny * correction));
        cells.set_position(endpoints.b, Position::new(pos_b.x() - nx * correction, pos_b.y() - ny * correction));
        joint_mechanical_correction_amount += correction * 2.0;
    }
}
```

Clamp positions to world bounds using the same boundary logic as the physics solver before committing positions.

- [ ] **Step 4: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_channels joint_mechanical_constraint_limits_endpoint_separation -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/core/tick.rs src/core/summary.rs tests/phase2h_joint_channels.rs
git commit -m "feat: apply joint mechanical constraint"
```

---

## Task 5: Passive Resource Channel

**Files:**

- Modify: `src/core/joints.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/phase2h_joint_channels.rs`

- [ ] **Step 1: Add failing Resource channel tests**

Append:

```rust
#[test]
fn joint_resource_channel_moves_resource_down_gradient_and_conserves_total() {
    let mut config = two_touching_cells_config();
    config.joints.resource_transfer_rate = 0.5;
    config.joints.max_resource_transfer_per_tick = alife::core::units::ResourceAmount::new(2.0).unwrap();
    config.initial_cells[0].initial_resource_amount = alife::core::units::ResourceAmount::new(10.0).unwrap();
    config.initial_cells[1].initial_resource_amount = alife::core::units::ResourceAmount::new(0.0).unwrap();
    config.initial_cells[1].capacity_limit = alife::core::units::CapacityAmount::new(50.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    let before_total = exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw()
        + exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();

    let summary = exec.step().unwrap();
    let after_a = exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw();
    let after_b = exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();

    assert!(after_a < 10.0);
    assert!(after_b > 0.0);
    assert!(((after_a + after_b) - before_total).abs() < 0.0001);
    assert!(summary.metrics.joint_resource_transfer_amount > 0.0);
}

#[test]
fn joint_resource_channel_does_not_transfer_energy_buffer() {
    let mut config = two_touching_cells_config();
    config.joints.resource_transfer_rate = 0.5;
    config.initial_cells[0].initial_energy = alife::core::units::EnergyAmount::new(20.0).unwrap();
    config.initial_cells[1].initial_energy = alife::core::units::EnergyAmount::new(0.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    exec.step().unwrap();

    assert_eq!(exec.world().cells().energy(CellIndex::from_raw(1)).current().raw(), 0.0);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_channels joint_resource_channel -- --nocapture
```

Expected: FAIL because Resource channel is not applied.

- [ ] **Step 3: Implement passive channel**

In `src/core/tick.rs`, after joint creation and before metabolism, add:

```rust
let mut joint_resource_transfer_amount = 0.0_f32;
if config.joints.enabled {
    let joint_ids = self.world.joints().active_ids().collect::<Vec<_>>();
    for joint_id in joint_ids {
        let endpoints = self.world.joints().endpoints(joint_id).unwrap();
        let channel = self.world.joints().config(joint_id).unwrap();
        if channel.resource_transfer_rate <= 0.0 || channel.max_resource_transfer_per_tick <= 0.0 {
            continue;
        }
        let a = self.world.cells().resource_amount(endpoints.a).raw();
        let b = self.world.cells().resource_amount(endpoints.b).raw();
        if (a - b).abs() <= f32::EPSILON {
            continue;
        }
        let (source, target, gradient) = if a > b {
            (endpoints.a, endpoints.b, a - b)
        } else {
            (endpoints.b, endpoints.a, b - a)
        };
        let free_target = self
            .world
            .cells()
            .effective_free_capacity(target, config.material_effects.storage_capacity_per_unit)
            .raw();
        let requested = (gradient * channel.resource_transfer_rate)
            .min(channel.max_resource_transfer_per_tick)
            .min(free_target);
        if requested <= 0.0 {
            continue;
        }
        let moved = self.world.cells_mut_for_commit().transfer_resources_limited_by_effective_capacity(
            source,
            target,
            ResourceAmount::new(requested).unwrap(),
            config.material_effects.storage_capacity_per_unit,
        );
        joint_resource_transfer_amount += moved.raw();
    }
}
```

- [ ] **Step 4: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_channels joint_resource_channel -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/core/tick.rs src/core/summary.rs tests/phase2h_joint_channels.rs
git commit -m "feat: add passive joint resource channel"
```

---

## Task 6: Delayed Scalar Signal Channel

**Files:**

- Modify: `src/core/joints.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/phase2h_joint_channels.rs`

- [ ] **Step 1: Add failing signal delay test**

Append:

```rust
#[test]
fn joint_signal_written_in_tick_n_is_readable_in_tick_n_plus_one() {
    let mut config = two_touching_cells_config();
    config.joints.signal_conductivity = 1.0;
    config.joints.signal_decay = 0.0;
    config.local_interaction.contact_stimulus_per_overlap = 1.0;
    config.initial_cells[0].initial_sensory_material = alife::core::units::MaterialAmount::new(1.0).unwrap();
    config.initial_cells[1].initial_sensory_material = alife::core::units::MaterialAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    let first = exec.step().unwrap();
    assert_eq!(first.metrics.joint_signal_readable_total, 0.0);
    assert!(first.metrics.joint_signal_generated_total > 0.0);

    let second = exec.step().unwrap();
    assert!(second.metrics.joint_signal_readable_total > 0.0);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_channels joint_signal_written -- --nocapture
```

Expected: FAIL because joint signal state and Tick N+1 readability are missing.

- [ ] **Step 3: Add signal buffers to JointStore**

In `src/core/joints.rs` add arrays:

```rust
signal_current: Vec<f32>,
signal_next: Vec<f32>,
signal_readable_from: Vec<Tick>,
```

Add methods:

```rust
pub fn readable_signal(&self, id: JointId, tick: Tick) -> Option<f32> {
    let index = id.raw() as usize;
    if self.signal_readable_from.get(index).copied()? <= tick {
        Some(self.signal_current[index])
    } else {
        Some(0.0)
    }
}

pub fn add_next_signal(&mut self, id: JointId, amount: f32, readable_from: Tick) {
    let index = id.raw() as usize;
    self.signal_next[index] = (self.signal_next[index] + amount).clamp(0.0, 1.0);
    self.signal_readable_from[index] = readable_from;
}

pub fn commit_signal_buffers(&mut self, decay: f32) {
    for index in 0..self.signal_current.len() {
        self.signal_current[index] = (self.signal_next[index] * (1.0 - decay.clamp(0.0, 1.0))).clamp(0.0, 1.0);
        self.signal_next[index] = 0.0;
    }
}
```

- [ ] **Step 4: Emit and read scalar signal through joint**

In `TickExecutor::step`, read `joint_signal_readable_total` before writing new signal:

```rust
let mut joint_signal_readable_total = 0.0_f32;
for joint_id in self.world.joints().active_ids().collect::<Vec<_>>() {
    joint_signal_readable_total += self
        .world
        .joints()
        .readable_signal(joint_id, self.world.tick())
        .unwrap_or(0.0);
}
```

After contact stimulus generation, write next joint signal from contact overlap:

```rust
let mut joint_signal_generated_total = 0.0_f32;
if config.joints.enabled {
    for joint_id in self.world.joints().active_ids().collect::<Vec<_>>() {
        let endpoints = self.world.joints().endpoints(joint_id).unwrap();
        let channel = self.world.joints().config(joint_id).unwrap();
        if channel.signal_conductivity <= 0.0 {
            continue;
        }
        let overlap = self
            .world
            .contact_cache()
            .pairs()
            .iter()
            .find(|pair| crate::core::joints::JointEndpoints::new(pair.a, pair.b) == Some(endpoints))
            .map(|pair| pair.overlap)
            .unwrap_or(0.0);
        let signal = (overlap * channel.signal_conductivity).clamp(0.0, 1.0);
        if signal > 0.0 {
            self.world
                .joints_mut_for_commit()
                .add_next_signal(joint_id, signal, self.world.tick().next());
            joint_signal_generated_total += signal;
        }
    }
    self.world
        .joints_mut_for_commit()
        .commit_signal_buffers(config.joints.signal_decay);
}
```

If this commits too early and makes the test fail by same-tick visibility, split into `begin_tick_signal_rollover()` and `end_tick_signal_write()` so `signal_current` is read before `signal_next` is promoted for Tick N+1.

- [ ] **Step 5: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_channels joint_signal_written -- --nocapture
```

Expected: PASS, with first tick readable total `0.0`.

- [ ] **Step 6: Commit**

```bash
git add src/core/joints.rs src/core/tick.rs src/core/summary.rs tests/phase2h_joint_channels.rs
git commit -m "feat: add delayed joint signal channel"
```

---

## Task 7: Heat Channel

**Files:**

- Modify: `src/core/tick.rs`
- Test: `tests/phase2h_joint_channels.rs`

- [ ] **Step 1: Add failing heat test**

Append:

```rust
#[test]
fn joint_heat_channel_moves_temperature_without_energy_transfer() {
    let mut config = two_touching_cells_config();
    config.joints.heat_conductivity = 0.5;
    config.chemistry.heat.capacity = 1.0;
    config.initial_cells[0].initial_energy = alife::core::units::EnergyAmount::new(10.0).unwrap();
    config.initial_cells[1].initial_energy = alife::core::units::EnergyAmount::new(1.0).unwrap();

    let mut exec = TickExecutor::new(config).unwrap();
    exec.step().unwrap();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_temperature(CellIndex::from_raw(0), alife::core::units::Temperature::new(40.0));
    exec.world_mut()
        .cells_mut_for_commit()
        .set_temperature(CellIndex::from_raw(1), alife::core::units::Temperature::new(20.0));

    let before_energy_b = exec.world().cells().energy(CellIndex::from_raw(1)).current().raw();
    let summary = exec.step().unwrap();

    assert!(exec.world().cells().temperature(CellIndex::from_raw(0)).raw() < 40.0);
    assert!(exec.world().cells().temperature(CellIndex::from_raw(1)).raw() > 20.0);
    assert_eq!(exec.world().cells().energy(CellIndex::from_raw(1)).current().raw(), before_energy_b);
    assert!(summary.metrics.joint_heat_transfer_amount > 0.0);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_channels joint_heat_channel -- --nocapture
```

Expected: FAIL because joint heat transfer is missing.

- [ ] **Step 3: Implement local heat transfer**

In `src/core/tick.rs` add:

```rust
let mut joint_heat_transfer_amount = 0.0_f32;
if config.joints.enabled {
    for joint_id in self.world.joints().active_ids().collect::<Vec<_>>() {
        let endpoints = self.world.joints().endpoints(joint_id).unwrap();
        let channel = self.world.joints().config(joint_id).unwrap();
        if channel.heat_conductivity <= 0.0 {
            continue;
        }
        let temp_a = self.world.cells().temperature(endpoints.a).raw();
        let temp_b = self.world.cells().temperature(endpoints.b).raw();
        let delta = (temp_a - temp_b) * channel.heat_conductivity.clamp(0.0, 1.0) * 0.5;
        if delta.abs() <= f32::EPSILON {
            continue;
        }
        let cells = self.world.cells_mut_for_commit();
        cells.set_temperature(endpoints.a, crate::core::units::Temperature::new(temp_a - delta));
        cells.set_temperature(endpoints.b, crate::core::units::Temperature::new(temp_b + delta));
        joint_heat_transfer_amount += delta.abs();
    }
}
```

- [ ] **Step 4: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_channels joint_heat_channel -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/core/tick.rs src/core/summary.rs tests/phase2h_joint_channels.rs
git commit -m "feat: add joint heat channel"
```

---

## Task 8: Degradation, Break, Death And Division Behavior

**Files:**

- Modify: `src/core/joints.rs`
- Modify: `src/core/world.rs`
- Modify: `src/core/tick.rs`
- Test: `tests/phase2h_joint_lifecycle.rs`

- [ ] **Step 1: Write failing lifecycle tests**

Add `tests/phase2h_joint_lifecycle.rs`:

```rust
use alife::core::cell_store::{CellIndex, LifecycleState};
use alife::core::config::RuntimeConfig;
use alife::core::tick::TickExecutor;

fn two_touching_cells_config() -> RuntimeConfig {
    let mut config = RuntimeConfig::default();
    config.joints.enabled = true;
    config.joints.creation_material_cost = alife::core::units::MaterialAmount::new(0.5).unwrap();
    config.local_interaction.enabled = true;
    config.world.size = alife::core::units::WorldSize::new(64.0, 64.0).unwrap();
    config.initial_cells = vec![
        alife::core::config::CellInitialConfig {
            position: alife::core::units::Position::new(30.0, 32.0),
            radius: alife::core::units::Radius::new(2.0).unwrap(),
            initial_energy: alife::core::units::EnergyAmount::new(20.0).unwrap(),
            initial_boundary_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_transport_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_structural_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            capacity_limit: alife::core::units::CapacityAmount::new(50.0).unwrap(),
            ..Default::default()
        },
        alife::core::config::CellInitialConfig {
            position: alife::core::units::Position::new(33.9, 32.0),
            radius: alife::core::units::Radius::new(2.0).unwrap(),
            initial_energy: alife::core::units::EnergyAmount::new(20.0).unwrap(),
            initial_boundary_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_transport_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            initial_structural_material: alife::core::units::MaterialAmount::new(2.0).unwrap(),
            capacity_limit: alife::core::units::CapacityAmount::new(50.0).unwrap(),
            ..Default::default()
        },
    ];
    config
}

#[test]
fn joint_degrades_and_breaks_deterministically() {
    let mut config = two_touching_cells_config();
    config.joints.upkeep_material_decay_per_tick = 0.6;
    config.joints.break_damage_threshold = 1.0;
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    let first_joint = exec.world().joints().active_ids().next().unwrap();
    let first_material = exec.world().joints().material_amount(first_joint).unwrap().raw();

    let summary = exec.step().unwrap();

    assert!(summary.metrics.joint_degradation_amount > 0.0);
    assert!(exec.world().joints().material_amount(first_joint).unwrap().raw() < first_material);
    assert!(exec.world().joints().is_broken(first_joint).unwrap());
    assert_eq!(summary.metrics.joint_broken_count, 1);
}

#[test]
fn endpoint_death_disables_living_joint_channels_without_instant_material_loss() {
    let mut config = two_touching_cells_config();
    config.joints.resource_transfer_rate = 1.0;
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    let joint = exec.world().joints().active_ids().next().unwrap();
    let material = exec.world().joints().material_amount(joint).unwrap().raw();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_lifecycle_state(CellIndex::from_raw(0), LifecycleState::Dead);

    let summary = exec.step().unwrap();

    assert!(!exec.world().joints().is_active(joint).unwrap());
    assert_eq!(exec.world().joints().material_amount(joint).unwrap().raw(), material);
    assert_eq!(summary.metrics.joint_resource_transfer_amount, 0.0);
}

#[test]
fn division_breaks_external_joints_without_duplication() {
    let mut config = two_touching_cells_config();
    config.division.enabled = true;
    config.growth_enabled = true;
    config.growth.growth_target_radius = alife::core::units::Radius::new(2.0).unwrap();
    config.division.energy_cost = alife::core::units::EnergyAmount::zero();
    let mut exec = TickExecutor::new(config).unwrap();

    exec.step().unwrap();
    exec.world_mut()
        .cells_mut_for_commit()
        .set_runtime_flags(CellIndex::from_raw(0), alife::core::cell_store::RuntimeFlags {
            division_ready: true,
            ..Default::default()
        });

    exec.step().unwrap();

    assert_eq!(exec.world().joints().len(), 1);
    assert_eq!(exec.world().joints().active_ids().count(), 0);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_joint_lifecycle -- --nocapture
```

Expected: FAIL because lifecycle hooks are missing.

- [ ] **Step 3: Add degradation and inert state methods**

In `src/core/joints.rs` add:

```rust
pub fn degrade_active(&mut self, rate: f32, threshold: f32, tick: Tick) -> (f32, u32) {
    let mut degraded = 0.0_f32;
    let mut broken = 0_u32;
    let rate = rate.clamp(0.0, 1.0);
    for index in 0..self.lifecycle.len() {
        if self.lifecycle[index] != JointLifecycle::Active {
            continue;
        }
        let current = self.material_amounts[index].raw();
        let amount = current * rate;
        if amount > 0.0 {
            degraded += amount;
            self.material_amounts[index] = MaterialAmount::new_unchecked((current - amount).max(0.0));
            self.damage[index] += amount;
        }
        if self.damage[index] >= threshold || self.material_amounts[index].raw() <= 0.0 {
            self.lifecycle[index] = JointLifecycle::Broken;
            self.broken_tick[index] = Some(tick);
            broken += 1;
        }
    }
    (degraded, broken)
}

pub fn make_inert_for_endpoint(&mut self, endpoint: CellIndex) -> u32 {
    let mut changed = 0_u32;
    for index in 0..self.lifecycle.len() {
        if self.lifecycle[index] == JointLifecycle::Active && self.endpoints[index].contains(endpoint) {
            self.lifecycle[index] = JointLifecycle::Inert;
            changed += 1;
        }
    }
    changed
}

pub fn break_for_endpoint(&mut self, endpoint: CellIndex, tick: Tick) -> u32 {
    let mut changed = 0_u32;
    for index in 0..self.lifecycle.len() {
        if self.lifecycle[index] != JointLifecycle::Broken && self.endpoints[index].contains(endpoint) {
            self.lifecycle[index] = JointLifecycle::Broken;
            self.broken_tick[index] = Some(tick);
            changed += 1;
        }
    }
    changed
}
```

- [ ] **Step 4: Hook death and division**

In `src/core/tick.rs`, when a cell transitions to `LifecycleState::Dead`, call:

```rust
self.world
    .joints_mut_for_commit()
    .make_inert_for_endpoint(idx);
```

In `WorldState::execute_division`, before partitioning parent state, call:

```rust
let current_tick = self.tick;
self.joints.break_for_endpoint(cell_idx, current_tick);
```

- [ ] **Step 5: Add degradation pass**

Near the end of `TickExecutor::step`, before summary creation:

```rust
let (joint_degradation_amount, joint_broken_count) = if config.joints.enabled {
    self.world.joints_mut_for_commit().degrade_active(
        config.joints.upkeep_material_decay_per_tick,
        config.joints.break_damage_threshold,
        self.world.tick(),
    )
} else {
    (0.0, 0)
};
```

- [ ] **Step 6: Run GREEN**

Run:

```bash
cargo test --test phase2h_joint_lifecycle -- --nocapture
```

Expected: PASS.

- [ ] **Step 7: Commit**

```bash
git add src/core/joints.rs src/core/world.rs src/core/tick.rs src/core/summary.rs tests/phase2h_joint_lifecycle.rs
git commit -m "feat: add joint lifecycle behavior"
```

---

## Task 9: Observer Projection And OrganismView

**Files:**

- Modify: `src/observer/projection.rs`
- Test: `tests/phase2h_observer_outputs.rs`

- [ ] **Step 1: Write failing observer tests**

Add `tests/phase2h_observer_outputs.rs`:

```rust
use alife::observer::projection::{metrics_summary_features, organism_view_features};

#[test]
fn metrics_projection_exposes_joint_features_without_behavior_authority() {
    let mut metrics = alife::core::summary::MetricsSummary::default();
    metrics.joint_count = 2;
    metrics.joint_resource_transfer_amount = 1.5;
    metrics.joint_signal_readable_total = 0.25;
    metrics.joint_heat_transfer_amount = 0.75;

    let features = metrics_summary_features(&metrics);

    assert_eq!(features["joint_count"], 2.0);
    assert_eq!(features["joint_resource_transfer_amount"], 1.5);
    assert_eq!(features["joint_signal_readable_total"], 0.25);
    assert_eq!(features["joint_heat_transfer_amount"], 0.75);
}

#[test]
fn organism_view_is_connected_component_projection_only() {
    let components = organism_view_features(4, &[(0, 1), (1, 2)]);

    assert_eq!(components.component_count, 2);
    assert_eq!(components.largest_component_size, 3);
    assert_eq!(components.isolated_cell_count, 1);
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_observer_outputs -- --nocapture
```

Expected: FAIL because projection fields and `organism_view_features` are missing.

- [ ] **Step 3: Implement projection helper**

In `src/observer/projection.rs` add joint features to `metrics_summary_features` and add:

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OrganismViewFeatures {
    pub component_count: u32,
    pub largest_component_size: u32,
    pub isolated_cell_count: u32,
}

pub fn organism_view_features(cell_count: usize, active_edges: &[(usize, usize)]) -> OrganismViewFeatures {
    let mut parent: Vec<usize> = (0..cell_count).collect();

    fn find(parent: &mut [usize], x: usize) -> usize {
        if parent[x] != x {
            parent[x] = find(parent, parent[x]);
        }
        parent[x]
    }

    for &(a, b) in active_edges {
        if a >= cell_count || b >= cell_count || a == b {
            continue;
        }
        let ra = find(&mut parent, a);
        let rb = find(&mut parent, b);
        if ra != rb {
            parent[rb] = ra;
        }
    }

    let mut sizes = std::collections::BTreeMap::<usize, u32>::new();
    for cell in 0..cell_count {
        let root = find(&mut parent, cell);
        *sizes.entry(root).or_insert(0) += 1;
    }

    OrganismViewFeatures {
        component_count: sizes.len() as u32,
        largest_component_size: sizes.values().copied().max().unwrap_or(0),
        isolated_cell_count: sizes.values().filter(|&&size| size == 1).count() as u32,
    }
}
```

Keep this in observer; do not add any read path from core processes or Genome Runtime to this projection.

- [ ] **Step 4: Run GREEN**

Run:

```bash
cargo test --test phase2h_observer_outputs -- --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add src/observer/projection.rs tests/phase2h_observer_outputs.rs
git commit -m "feat: expose observer joint projections"
```

---

## Task 10: Sweep Analyzer Phase 2H Scenarios And Raw Outputs

**Files:**

- Modify: `src/bin/sweep_analyzer.rs`
- Modify: `config/analyzer/sweep_analyzer.toml`
- Modify: `config/analyzer/sweep_analyzer_smoke.toml`
- Test: `tests/phase2h_reachability.rs`

- [ ] **Step 1: Write failing reachability test**

Add `tests/phase2h_reachability.rs`:

```rust
use std::process::Command;

#[test]
fn smoke_sweep_outputs_phase2h_joint_reachability_csvs() {
    let status = Command::new(env!("CARGO_BIN_EXE_sweep_analyzer"))
        .arg("--config")
        .arg("config/analyzer/sweep_analyzer_smoke.toml")
        .status()
        .expect("sweep analyzer runs");
    assert!(status.success());

    for file in [
        "outputs/raw_data/joint_creation_viability.csv",
        "outputs/raw_data/joint_resource_channel.csv",
        "outputs/raw_data/joint_signal_delay.csv",
        "outputs/raw_data/joint_heat_channel.csv",
        "outputs/raw_data/joint_degradation_break.csv",
        "outputs/raw_data/joint_lifecycle_division.csv",
    ] {
        let csv = std::fs::read_to_string(file).unwrap_or_else(|err| panic!("{file}: {err}"));
        assert!(csv.contains("joint_count"), "{file} missing joint_count");
        assert!(csv.contains("joint_created_count"), "{file} missing joint_created_count");
        assert!(csv.contains("joint_resource_transfer_amount"), "{file} missing resource metric");
        assert!(csv.contains("joint_signal_readable_total"), "{file} missing signal metric");
        assert!(csv.contains("joint_heat_transfer_amount"), "{file} missing heat metric");
        assert!(csv.contains("joint_degradation_amount"), "{file} missing degradation metric");
    }
}
```

- [ ] **Step 2: Run RED**

Run:

```bash
cargo test --test phase2h_reachability -- --nocapture
```

Expected: FAIL because analyzer does not emit Phase 2H CSVs/columns/scenarios.

- [ ] **Step 3: Add analyzer runtime preset**

In `src/bin/sweep_analyzer.rs` add:

```rust
fn configure_phase2h_runtime(rt: &mut RuntimeConfig, scenario: &str, overrides: &BTreeMap<String, f32>) {
    rt.joints.enabled = true;
    rt.local_interaction.enabled = true;
    rt.joints.creation_material_cost = MaterialAmount::new(0.5).unwrap();
    rt.joints.mechanical_strength = overrides.get("joint_mechanical_strength").copied().unwrap_or(0.35);
    rt.joints.resource_transfer_rate = overrides.get("joint_resource_transfer_rate").copied().unwrap_or(0.5);
    rt.joints.max_resource_transfer_per_tick = ResourceAmount::new(1.0).unwrap();
    rt.joints.signal_conductivity = overrides.get("joint_signal_conductivity").copied().unwrap_or(0.75);
    rt.joints.signal_decay = 0.0;
    rt.joints.heat_conductivity = overrides.get("joint_heat_conductivity").copied().unwrap_or(0.25);
    rt.joints.upkeep_material_decay_per_tick = overrides.get("joint_decay_rate").copied().unwrap_or(0.0);
    rt.joints.break_damage_threshold = 1.0;

    if scenario == "joint_degradation_break" {
        rt.joints.upkeep_material_decay_per_tick = overrides.get("joint_decay_rate").copied().unwrap_or(0.5);
    }
    if scenario == "joint_lifecycle_division" {
        rt.division.enabled = true;
        rt.growth_enabled = true;
    }
}
```

Call it for scenarios starting with `joint_`.

- [ ] **Step 4: Add raw data columns and warnings**

Add cumulative joint metrics to `SimResult.raw_data`:

```rust
raw_data.insert("joint_count".to_string(), summary.metrics.joint_count as f32);
raw_data.insert("joint_created_count".to_string(), joint_created_count_cumulative as f32);
raw_data.insert("joint_creation_rejected_count".to_string(), joint_creation_rejected_count_cumulative as f32);
raw_data.insert("joint_resource_transfer_amount".to_string(), joint_resource_transfer_cumulative);
raw_data.insert("joint_signal_generated_total".to_string(), joint_signal_generated_cumulative);
raw_data.insert("joint_signal_readable_total".to_string(), joint_signal_readable_cumulative);
raw_data.insert("joint_heat_transfer_amount".to_string(), joint_heat_transfer_cumulative);
raw_data.insert("joint_degradation_amount".to_string(), joint_degradation_cumulative);
raw_data.insert("joint_broken_count".to_string(), joint_broken_count_cumulative as f32);
```

In `detect_warnings`, add:

```rust
if scenario_lower.starts_with("joint_") {
    if max_of(results, "joint_created_count") <= 0.0 {
        warnings.push("JOINT_NOT_CREATED".to_string());
    }
    if scenario_lower.contains("resource") && max_of(results, "joint_resource_transfer_amount") <= 0.0 {
        warnings.push("JOINT_RESOURCE_CHANNEL_INACTIVE".to_string());
    }
    if scenario_lower.contains("signal") && max_of(results, "joint_signal_readable_total") <= 0.0 {
        warnings.push("JOINT_SIGNAL_CHANNEL_INACTIVE".to_string());
    }
    if scenario_lower.contains("heat") && max_of(results, "joint_heat_transfer_amount") <= 0.0 {
        warnings.push("JOINT_HEAT_CHANNEL_INACTIVE".to_string());
    }
    if scenario_lower.contains("degradation") && max_of(results, "joint_broken_count") <= 0.0 {
        warnings.push("JOINT_BREAK_NOT_REACHED".to_string());
    }
}
```

Implement `max_of(results, key)` near warning helpers:

```rust
fn max_of(results: &[SimResult], key: &str) -> f32 {
    results
        .iter()
        .filter_map(|result| result.raw_data.get(key).copied())
        .fold(0.0, f32::max)
}
```

- [ ] **Step 5: Add full and smoke sweeps**

Add to `config/analyzer/sweep_analyzer.toml`:

```toml
[[sweep]]
name = "joint_creation_viability"
scenario = "joint_creation_viability"
param = "joint_mechanical_strength"
from = 0.1
to = 1.0
steps = 8

[[sweep]]
name = "joint_resource_channel"
scenario = "joint_resource_channel"
param = "joint_resource_transfer_rate"
from = 0.0
to = 1.0
steps = 8

[[sweep]]
name = "joint_signal_delay"
scenario = "joint_signal_delay"
param = "joint_signal_conductivity"
from = 0.0
to = 1.0
steps = 8

[[sweep]]
name = "joint_heat_channel"
scenario = "joint_heat_channel"
param = "joint_heat_conductivity"
from = 0.0
to = 1.0
steps = 8

[[sweep]]
name = "joint_degradation_break"
scenario = "joint_degradation_break"
param = "joint_decay_rate"
from = 0.0
to = 0.8
steps = 8

[[sweep]]
name = "joint_lifecycle_division"
scenario = "joint_lifecycle_division"
param = "joint_mechanical_strength"
from = 0.1
to = 1.0
steps = 8
```

Add the same names to `config/analyzer/sweep_analyzer_smoke.toml` with `steps = 2`, low tick count, and matching `[scenarios.*]` sections.

- [ ] **Step 6: Run GREEN**

Run:

```bash
cargo test --test phase2h_reachability -- --nocapture
```

Expected: PASS and Phase 2H CSV files exist in `outputs/raw_data`.

- [ ] **Step 7: Commit**

```bash
git add src/bin/sweep_analyzer.rs config/analyzer/sweep_analyzer.toml config/analyzer/sweep_analyzer_smoke.toml tests/phase2h_reachability.rs
git commit -m "feat: add phase 2h sweep coverage"
```

---

## Task 11: Full Verification And Phase 2H Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2H-persistent-interaction-structures.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run focused Phase 2H tests**

Run:

```bash
cargo test --test phase2h_joint_store -- --nocapture
cargo test --test phase2h_joint_creation -- --nocapture
cargo test --test phase2h_joint_channels -- --nocapture
cargo test --test phase2h_joint_lifecycle -- --nocapture
cargo test --test phase2h_observer_outputs -- --nocapture
cargo test --test phase2h_sweep_parser -- --nocapture
cargo test --test phase2h_reachability -- --nocapture
```

Expected: all PASS.

- [ ] **Step 2: Run regression tests for Phase 2F/2G surfaces**

Run:

```bash
cargo test --test phase2_local_interaction_exchange -- --nocapture
cargo test --test phase2_local_interaction_stimulus -- --nocapture
cargo test --test phase2g_sweep_parser -- --nocapture
cargo test --test phase2g_tick_integration -- --nocapture
```

Expected: all PASS.

- [ ] **Step 3: Run workspace verification**

Run:

```bash
cargo fmt --check
cargo test --workspace --all-targets
```

Expected: all PASS. If runtime is too long, record the exact command, elapsed time and stopping reason in the report; do not claim full verification passed.

- [ ] **Step 4: Run analyzer smoke**

Run:

```bash
cargo run --bin sweep_analyzer -- --config config/analyzer/sweep_analyzer_smoke.toml
```

Expected: PASS and these files exist:

```text
outputs/raw_data/joint_creation_viability.csv
outputs/raw_data/joint_resource_channel.csv
outputs/raw_data/joint_signal_delay.csv
outputs/raw_data/joint_heat_channel.csv
outputs/raw_data/joint_degradation_break.csv
outputs/raw_data/joint_lifecycle_division.csv
```

- [ ] **Step 5: Write report**

Create the report with this structure:

```markdown
# REPORT: Phase 2H Persistent Interaction Structures

## Summary

Implemented persistent material-backed Joints with deterministic creation, mechanics, Resource, Signal and Heat channels, degradation/break, death/division behavior, observer projections and sweep analyzer coverage.

## Acceptance Gates

- stable connected Cell structures: PASS/FAIL with test name
- Joint costs matter: PASS/FAIL with test name
- Joint degrades/breaks: PASS/FAIL with test name
- Resource channel local/accounted: PASS/FAIL with test name
- Signal delayed Tick N+1: PASS/FAIL with test name
- Heat channel local/no Energy transfer: PASS/FAIL with test name
- death/division behavior: PASS/FAIL with test name
- OrganismView observer-only: PASS/FAIL with test name
- analyzer raw outputs: PASS/FAIL with CSV list

## Verification

List commands and results.

## Known Limits

- No Genome-directed joint creation.
- No active directed Resource transport.
- No HGT/Genome transfer.
- No organism-level controller.
- Joint repair is minimal or deferred unless implemented by Task 8 extension.
```

- [ ] **Step 6: Update worklog index and commit**

```bash
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2H-persistent-interaction-structures.md outputs/worklogs/index.md
git commit -m "docs: report phase 2h completion"
```

---

## Self-Review Checklist

- [ ] Every new behavior starts with a failing Rust test.
- [ ] `JointStore` is World-owned and uses typed `JointId`, not persistent references.
- [ ] Contact cache remains derived input, not persistent state.
- [ ] Joint creation is local and material/cost backed.
- [ ] No direct `Energy Buffer` transfer through Joint.
- [ ] No Genome, HGT or heritable information transfer.
- [ ] Signal is scalar and delayed to Tick N+1.
- [ ] Resource, Signal and Heat channels are explicit and local.
- [ ] Death disables living channels without instant material disappearance.
- [ ] Division breaks external Joints without duplication.
- [ ] `OrganismView` is observer-only projection and is not readable by core behavior.
- [ ] Full and smoke analyzer configs include Phase 2H scenarios.
- [ ] `outputs/raw_data` has Phase 2H CSV proof files after analyzer smoke.
- [ ] Phase 1 and Phase 2A-G regression tests still pass or failures are documented with exact cause.

## Execution Notes For Agents

- Do not start by adding all production code. Each task must go RED, then GREEN.
- Keep commits task-sized.
- If a test exposes missing existing helpers, add only the smallest helper needed for that test.
- If a proposed snippet conflicts with current signatures, preserve Canon behavior and current storage style; update the test first only if the test was asserting the wrong public API, not to fit a broken implementation.
- If borrow checker pressure appears in joint channel code, prefer deterministic collect-then-commit buffers over cloning `CellStore` or adding `RefCell`.
