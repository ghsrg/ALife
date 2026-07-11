---
tags:
  - alife
  - worklog/plan
  - phase/2F
  - tdd
  - rust
---

# Phase 2F Local Cell-Cell Interaction Primitives Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add minimal local Cell-Cell interaction primitives before Genome: deterministic contact pairs, observable contact pressure, material-gated passive Resource exchange, scalar contact stimulus, and sweep_analyzer guardrails.

**Architecture:** Phase 2F must stay contact-derived, local and material-gated. It must not introduce full `JointStore`, organism-level control, semantic command signals, direct Energy transfer, Genome transfer, or observer-label feedback. Contact pairs are derived state from `SpatialIndex` and `CellStore`; passive exchange and scalar stimulus are deterministic Tick systems operating on stable pair order.

**Tech Stack:** Rust core (`src/core/*`), TOML runtime parser (`src/runner/config_parser.rs`), analyzer binary (`src/bin/sweep_analyzer.rs`), Cargo integration tests under `tests/`, analyzer configs under `config/analyzer/`.

---

## Domain Constraints

Read before implementation:

- `docs/PRINCIPLES.md`
- `docs/mechanics/joint-interaction.md`
- `docs/mechanics/signal-propagation.md`
- `docs/biology/joint.md`
- `docs/biology/communication.md`
- `docs/biology/membrane.md`
- `docs/world/materials.md`
- `docs/world/resources.md`
- `docs/world/physics.md`
- `docs/world/space.md`
- `docs/world/tick-semantics.md`

Phase 2F interpretations:

- `ContactPair` is derived runtime state, not a long-lived `Joint`.
- Passive contact exchange transfers only `ResourceAmount`, never `Energy Buffer`, Genome, Genome fragments, or Materials.
- Exchange requires local contact plus material capability. Minimum gate: both cells alive, both have `BoundaryPermeability`, source has `ResourceUptake`, target has free capacity.
- Exchange conservation rule: target capacity is authoritative. Remove from source only the exact amount the target actually accepts after capacity clamp.
- Scalar contact stimulus is a normalized physical input. It is not a command and has no semantic type.
- Stimulus produced during Tick N becomes readable only at Tick N+1.
- Tick N summary must report only stimulus that was readable at the start of Tick N; it must not include stimulus generated during Tick N.
- Observer and analyzer metrics are read-only diagnostics.

Deferred from Phase 2F:

- full `JointStore`;
- mechanical Joint constraints;
- Joint upkeep/degradation/repair;
- resource channels through persistent Joints;
- delayed signal traces beyond scalar contact stimulus;
- organism connected components;
- adhesion/binding unless explicitly split into a separate later plan.

## File Map

Create:

- `src/core/contact.rs` - derived contact pair/cache structs and deterministic contact pair construction.
- `tests/phase2_local_interaction_contact.rs` - contact pair order, contact pressure summary, and determinism tests.
- `tests/phase2_local_interaction_exchange.rs` - passive contact exchange tests and conservation checks.
- `tests/phase2_local_interaction_stimulus.rs` - scalar stimulus delay and material-gating tests.

Modify:

- `src/core/mod.rs` - export `contact`.
- `src/core/world.rs` - own `ContactCache`, expose read/commit accessors, call rebuild hooks.
- `src/core/cell_store.rs` - store contact stimulus readable/current and next buffers.
- `src/core/config.rs` - add `LocalInteractionConfig` and include it in `RuntimeConfig` and `config_hash`.
- `src/runner/config_parser.rs` - parse `[local_interaction]`.
- `src/core/tick.rs` - integrate contact cache, passive exchange, stimulus staging, and summary metrics.
- `src/core/summary.rs` - add local interaction metrics.
- `src/bin/sweep_analyzer.rs` - add `local_interaction_viability` scenario, metrics, CSV columns, warnings.
- `config/analyzer/sweep_analyzer.toml` - add local interaction sweep.
- `config/analyzer/sweep_analyzer_smoke.toml` - add smoke-scale local interaction sweep.
- `tests/phase2_sweep_parser.rs` - parser coverage for new analyzer preset fields.
- `tests/phase2_sweep_outputs.rs` - CSV columns and local interaction raw output acceptance.
- `tests/phase2_sweep_warnings.rs` - warning logic for flat/missing local interactions.

Do not modify:

- Observer classifiers as behavior input.
- Genome docs/runtime.
- Full Joint docs unless implementation discovers a real Canon gap.

---

### Task 1: Derived Contact Cache

**Files:**

- Create: `src/core/contact.rs`
- Modify: `src/core/mod.rs`
- Modify: `src/core/world.rs`
- Test: `tests/phase2_local_interaction_contact.rs`

- [ ] **Step 1: Write the failing contact cache test**

Add this test file:

```rust
use alife::core::{
    cell_store::{CellIndex, CellStore, EnergyBuffer, InitialCellState},
    contact::ContactCache,
    spatial::SpatialIndex,
    units::{CapacityAmount, EnergyAmount, MaterialAmount, Position, Radius, ResourceAmount, WorldSize},
};

fn cell_at(x: f32, y: f32, radius: f32) -> InitialCellState {
    InitialCellState {
        position: Position::new(x, y),
        radius: Radius::new(radius).unwrap(),
        energy: EnergyBuffer::new(EnergyAmount::new(10.0).unwrap(), EnergyAmount::new(20.0).unwrap()),
        resources: ResourceAmount::zero(),
        boundary_material: MaterialAmount::new(1.0).unwrap(),
        transport_material: MaterialAmount::new(1.0).unwrap(),
        metabolic_material: MaterialAmount::zero(),
        storage_material: MaterialAmount::zero(),
        synthesis_material: MaterialAmount::zero(),
        structural_material: MaterialAmount::new(1.0).unwrap(),
        repair_material: MaterialAmount::zero(),
        contractile_material: MaterialAmount::zero(),
        sensory_material: MaterialAmount::zero(),
        capacity_limit: CapacityAmount::new(20.0).unwrap(),
        temperature: alife::core::units::Temperature::new(25.0),
    }
}

#[test]
fn contact_cache_records_only_overlapping_pairs_in_stable_order() {
    let mut cells = CellStore::with_capacity(4);
    cells.insert_initial(cell_at(10.0, 10.0, 2.0));
    cells.insert_initial(cell_at(13.0, 10.0, 2.0));
    cells.insert_initial(cell_at(16.0, 10.0, 2.0));
    cells.insert_initial(cell_at(40.0, 40.0, 1.0));

    let mut spatial = SpatialIndex::new();
    spatial.rebuild(&cells, WorldSize::new(64.0, 64.0).unwrap(), 8.0);

    let mut cache = ContactCache::default();
    cache.rebuild(&cells, &spatial);

    let pairs: Vec<_> = cache
        .pairs()
        .iter()
        .map(|pair| (pair.a.raw(), pair.b.raw(), pair.overlap))
        .collect();

    assert_eq!(pairs.len(), 2);
    assert_eq!(pairs[0].0, CellIndex::from_raw(0).raw());
    assert_eq!(pairs[0].1, CellIndex::from_raw(1).raw());
    assert_eq!(pairs[1].0, CellIndex::from_raw(1).raw());
    assert_eq!(pairs[1].1, CellIndex::from_raw(2).raw());
    assert!(pairs[0].2 > 0.9 && pairs[0].2 < 1.1);
    assert!(cache.total_overlap() > 1.9 && cache.total_overlap() < 2.1);
    assert!(cache.max_overlap() > 0.9 && cache.max_overlap() < 1.1);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_local_interaction_contact -- contact_cache_records_only_overlapping_pairs_in_stable_order --nocapture
```

Expected: compile failure because `alife::core::contact::ContactCache` does not exist.

- [ ] **Step 3: Implement minimal contact module**

Create `src/core/contact.rs`:

```rust
use crate::core::cell_store::{CellIndex, CellStore};
use crate::core::spatial::SpatialIndex;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct ContactPair {
    pub a: CellIndex,
    pub b: CellIndex,
    pub overlap: f32,
    pub normal_x_from_b_to_a: f32,
    pub normal_y_from_b_to_a: f32,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct ContactCache {
    pairs: Vec<ContactPair>,
    total_overlap: f32,
    max_overlap: f32,
}

impl ContactCache {
    pub fn rebuild(&mut self, cells: &CellStore, spatial_index: &SpatialIndex) {
        self.pairs.clear();
        self.total_overlap = 0.0;
        self.max_overlap = 0.0;

        let mut candidates = Vec::new();
        spatial_index.generate_candidate_pairs(cells, &mut candidates);

        for (a, b) in candidates {
            let pos_a = cells.position(a);
            let pos_b = cells.position(b);
            let dx = pos_a.x() - pos_b.x();
            let dy = pos_a.y() - pos_b.y();
            let dist_sq = dx * dx + dy * dy;
            let target_dist = cells.radius(a).raw() + cells.radius(b).raw();

            if dist_sq >= target_dist * target_dist {
                continue;
            }

            let dist = dist_sq.sqrt();
            let overlap = target_dist - dist;
            let (normal_x, normal_y) = if dist > 0.0 {
                (dx / dist, dy / dist)
            } else if a.raw() < b.raw() {
                (1.0, 0.0)
            } else {
                (-1.0, 0.0)
            };

            self.total_overlap += overlap;
            self.max_overlap = self.max_overlap.max(overlap);
            self.pairs.push(ContactPair {
                a,
                b,
                overlap,
                normal_x_from_b_to_a: normal_x,
                normal_y_from_b_to_a: normal_y,
            });
        }

        self.pairs.sort_unstable_by_key(|pair| (pair.a.raw(), pair.b.raw()));
    }

    pub fn pairs(&self) -> &[ContactPair] {
        &self.pairs
    }

    pub fn total_overlap(&self) -> f32 {
        self.total_overlap
    }

    pub fn max_overlap(&self) -> f32 {
        self.max_overlap
    }
}
```

Modify `src/core/mod.rs`:

```rust
pub mod contact;
```

Modify `src/core/world.rs`:

```rust
use crate::core::contact::ContactCache;
```

Add field:

```rust
contact_cache: ContactCache,
```

Initialize in `WorldState::from_config`:

```rust
let mut contact_cache = ContactCache::default();
contact_cache.rebuild(&cells, &spatial_index);
```

Add to `Self`:

```rust
contact_cache,
```

Add accessors:

```rust
pub fn contact_cache(&self) -> &ContactCache {
    &self.contact_cache
}

pub fn rebuild_contact_cache(&mut self) {
    self.contact_cache.rebuild(&self.cells, &self.spatial_index);
}
```

Call in `advance_tick` after spatial rebuild:

```rust
self.contact_cache.rebuild(&self.cells, &self.spatial_index);
```

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cargo test --test phase2_local_interaction_contact -- contact_cache_records_only_overlapping_pairs_in_stable_order --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit checkpoint**

```powershell
git add src/core/contact.rs src/core/mod.rs src/core/world.rs tests/phase2_local_interaction_contact.rs
git commit -m "feat: add deterministic contact cache"
```

---

### Task 2: Contact Pressure Summary Uses Contact Cache

**Files:**

- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Test: `tests/phase2_local_interaction_contact.rs`

- [ ] **Step 1: Write failing summary test**

Append:

```rust
use alife::core::{
    config::{CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig},
    tick::TickExecutor,
    units::{HeatAmount, Seed, Tick, WasteAmount},
};

fn two_overlapping_cells_config() -> RuntimeConfig {
    let first = cell_at(10.0, 10.0, 2.0);
    let second = cell_at(13.0, 10.0, 2.0);
    RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(1),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        first,
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
    .with_cells(vec![first, second])
}

#[test]
fn run_summary_reports_contact_pairs_and_pressure() {
    let mut exec = TickExecutor::new(two_overlapping_cells_config()).unwrap();
    let summary = exec.step().unwrap();

    assert_eq!(summary.metrics.contact_pairs_count, 1);
    assert!(summary.metrics.contact_pressure_pre_total > 0.0);
    assert!(summary.metrics.contact_pressure_max_over_tick > 0.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_local_interaction_contact -- run_summary_reports_contact_pairs_and_pressure --nocapture
```

Expected: compile failure because `MetricsSummary` lacks contact metrics.

- [ ] **Step 3: Add summary fields and use contact cache in tick**

Modify `src/core/summary.rs`:

```rust
pub contact_pairs_count: u32,
pub contact_pressure_pre_total: f32,
pub contact_pressure_post_total: f32,
pub contact_pressure_max_over_tick: f32,
```

Modify `TickExecutor::build_metrics_summary` signature:

```rust
contact_pairs_count: u32,
contact_pressure_pre_total: f32,
contact_pressure_post_total: f32,
contact_pressure_max_over_tick: f32,
```

Set fields in returned `MetricsSummary`.

In `TickExecutor::step`, after `self.world.rebuild_spatial_index();`, call:

```rust
self.world.rebuild_contact_cache();
```

Before physics loop, derive:

```rust
let contact_pairs_count = self.world.contact_cache().pairs().len() as u32;
let contact_pressure_pre_total = self.world.contact_cache().total_overlap();
let mut contact_pressure_max_over_tick = self.world.contact_cache().max_overlap();
```

Pressure semantics:

- `contact_pressure_pre_total`: sum of overlaps from contact cache before physics resolution in this Tick.
- `contact_pressure_post_total`: sum of overlaps after physics resolution in this Tick.
- `contact_pressure_max_over_tick`: maximum pair overlap observed before or during physics resolution in this Tick.

During overlap solver, replace local candidate `pairs` generation with a cache rebuild at each solver iteration:

```rust
self.world.rebuild_spatial_index();
self.world.rebuild_contact_cache();
let pairs: Vec<_> = self.world.contact_cache().pairs().to_vec();
```

Keep pressure writes based on overlap. During each iteration, update max-over-tick:

```rust
contact_pressure_max_over_tick =
    contact_pressure_max_over_tick.max(self.world.contact_cache().max_overlap());
```

At the end of physics, rebuild the cache once more and set post pressure:

```rust
self.world.rebuild_spatial_index();
self.world.rebuild_contact_cache();
let contact_pressure_post_total = self.world.contact_cache().total_overlap();
contact_pressure_max_over_tick =
    contact_pressure_max_over_tick.max(self.world.contact_cache().max_overlap());
```

Pass new values into `build_metrics_summary`.

- [ ] **Step 4: Run test to verify it passes**

Run:

```powershell
cargo test --test phase2_local_interaction_contact -- run_summary_reports_contact_pairs_and_pressure --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit checkpoint**

```powershell
git add src/core/tick.rs src/core/summary.rs tests/phase2_local_interaction_contact.rs
git commit -m "feat: expose contact pressure metrics"
```

---

### Task 3: Local Interaction Config And Parser

**Files:**

- Modify: `src/core/config.rs`
- Modify: `src/runner/config_parser.rs`
- Test: `tests/phase2_local_interaction_exchange.rs`
- Test: `tests/phase2_config_hash.rs`

- [ ] **Step 1: Write failing parser/config test**

Create `tests/phase2_local_interaction_exchange.rs` with:

```rust
use alife::runner::config_parser::load_config_from_str;

#[test]
fn parser_loads_local_interaction_config() {
    let toml = r#"
[world]
tick_count = 10
seed = 7
width = 32.0
height = 32.0

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 1

[resources]
initial_distribution = [0.0]
decay_rate = 0.0

[cell]
position = [10.0, 10.0]
radius = 1.0
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.0
passive_energy_income = 0.0
capacity_limit = 20.0
initial_resource_amount = 0.0
initial_boundary_material = 1.0
initial_transport_material = 1.0
initial_metabolic_material = 0.0
initial_storage_material = 0.0
initial_synthesis_material = 0.0
initial_structural_material = 1.0
initial_repair_material = 0.0
initial_contractile_material = 0.0
initial_sensory_material = 1.0

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
dormant_mandatory_cost_modifier = 1.0
critical_capacity_overrun = 100.0

[resource_interaction]
enabled = false
uptake_layer_index = 0
max_uptake_per_tick = 0.0
metabolism_resource_per_tick = 0.0
energy_per_resource = 0.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[local_interaction]
enabled = true
contact_exchange_rate = 0.5
max_exchange_per_pair = 2.0
min_boundary_capability = 0.1
min_transport_capability = 0.1
contact_stimulus_per_overlap = 0.25
stimulus_decay_per_tick = 0.5
"#;

    let config = load_config_from_str(toml).unwrap();
    assert!(config.local_interaction.enabled);
    assert_eq!(config.local_interaction.contact_exchange_rate, 0.5);
    assert_eq!(config.local_interaction.max_exchange_per_pair.raw(), 2.0);
    assert_eq!(config.local_interaction.contact_stimulus_per_overlap, 0.25);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_local_interaction_exchange -- parser_loads_local_interaction_config --nocapture
```

Expected: compile failure because `RuntimeConfig.local_interaction` does not exist.

- [ ] **Step 3: Implement config model**

Modify `src/core/config.rs`:

```rust
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct LocalInteractionConfig {
    pub enabled: bool,
    pub contact_exchange_rate: f32,
    pub max_exchange_per_pair: ResourceAmount,
    pub min_boundary_capability: f32,
    pub min_transport_capability: f32,
    pub contact_stimulus_per_overlap: f32,
    pub stimulus_decay_per_tick: f32,
}

impl Default for LocalInteractionConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            contact_exchange_rate: 0.0,
            max_exchange_per_pair: ResourceAmount::zero(),
            min_boundary_capability: 0.0,
            min_transport_capability: 0.0,
            contact_stimulus_per_overlap: 0.0,
            stimulus_decay_per_tick: 0.0,
        }
    }
}
```

Add to `RuntimeConfig`:

```rust
pub local_interaction: LocalInteractionConfig,
```

Initialize in `RuntimeConfig::new`:

```rust
local_interaction: LocalInteractionConfig::default(),
```

Add a config error:

```rust
InvalidLocalInteractionRate,
```

Add validation method:

```rust
pub fn validate_phase2f_options(&self) -> Result<(), ConfigError> {
    let cfg = self.local_interaction;
    for value in [
        cfg.contact_exchange_rate,
        cfg.min_boundary_capability,
        cfg.min_transport_capability,
        cfg.contact_stimulus_per_overlap,
        cfg.stimulus_decay_per_tick,
    ] {
        if !value.is_finite() || value < 0.0 {
            return Err(ConfigError::InvalidLocalInteractionRate);
        }
    }
    Ok(())
}
```

Include local interaction values in `config_hash`.

- [ ] **Step 4: Implement parser block**

Modify `src/runner/config_parser.rs`. Add TOML struct:

```rust
#[derive(Debug, Deserialize, Default)]
struct LocalInteractionToml {
    enabled: Option<bool>,
    contact_exchange_rate: Option<f32>,
    max_exchange_per_pair: Option<f32>,
    min_boundary_capability: Option<f32>,
    min_transport_capability: Option<f32>,
    contact_stimulus_per_overlap: Option<f32>,
    stimulus_decay_per_tick: Option<f32>,
}
```

Add to root parsed config:

```rust
local_interaction: Option<LocalInteractionToml>,
```

After `RuntimeConfig::new`, map:

```rust
if let Some(local) = parsed.local_interaction {
    config.local_interaction.enabled = local.enabled.unwrap_or(false);
    config.local_interaction.contact_exchange_rate = local.contact_exchange_rate.unwrap_or(0.0);
    config.local_interaction.max_exchange_per_pair =
        ResourceAmount::new(local.max_exchange_per_pair.unwrap_or(0.0))?;
    config.local_interaction.min_boundary_capability =
        local.min_boundary_capability.unwrap_or(0.0);
    config.local_interaction.min_transport_capability =
        local.min_transport_capability.unwrap_or(0.0);
    config.local_interaction.contact_stimulus_per_overlap =
        local.contact_stimulus_per_overlap.unwrap_or(0.0);
    config.local_interaction.stimulus_decay_per_tick =
        local.stimulus_decay_per_tick.unwrap_or(0.0);
}
config.validate_phase2f_options()?;
```

Use the repository's existing parse-error conversion pattern when `ResourceAmount::new` or validation fails.

- [ ] **Step 5: Run tests**

Run:

```powershell
cargo test --test phase2_local_interaction_exchange -- parser_loads_local_interaction_config --nocapture
cargo test --test phase2_config_hash
```

Expected: both PASS. If config hash test fails because new fields are absent from hash, add a dedicated `changing_local_interaction_config_changes_hash` test and then update hash.

- [ ] **Step 6: Commit checkpoint**

```powershell
git add src/core/config.rs src/runner/config_parser.rs tests/phase2_local_interaction_exchange.rs tests/phase2_config_hash.rs
git commit -m "feat: add local interaction config"
```

---

### Task 4: Passive Contact Resource Exchange

**Files:**

- Modify: `src/core/process.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Test: `tests/phase2_local_interaction_exchange.rs`

- [ ] **Step 1: Write failing positive exchange test**

Append to `tests/phase2_local_interaction_exchange.rs`:

```rust
use alife::core::{
    cell_store::CellIndex,
    config::{CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig},
    tick::TickExecutor,
    units::{CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount, Seed, Tick, WasteAmount, WorldSize},
};

fn exchange_cell(x: f32, resources: f32, boundary: f32, transport: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, 10.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::new(resources).unwrap(),
        initial_boundary_material: MaterialAmount::new(boundary).unwrap(),
        initial_transport_material: MaterialAmount::new(transport).unwrap(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::zero(),
    }
}

fn contact_exchange_config(boundary_a: f32, transport_a: f32, boundary_b: f32, transport_b: f32) -> RuntimeConfig {
    let a = exchange_cell(10.0, 10.0, boundary_a, transport_a);
    let b = exchange_cell(13.0, 0.0, boundary_b, transport_b);
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(1),
            seed: Seed::from_raw(2),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        a,
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
    .with_cells(vec![a, b]);
    config.local_interaction.enabled = true;
    config.local_interaction.contact_exchange_rate = 0.5;
    config.local_interaction.max_exchange_per_pair = ResourceAmount::new(2.0).unwrap();
    config.local_interaction.min_boundary_capability = 0.1;
    config.local_interaction.min_transport_capability = 0.1;
    config
}

#[test]
fn passive_contact_exchange_moves_resources_down_gradient_and_conserves_total() {
    let mut exec = TickExecutor::new(contact_exchange_config(1.0, 1.0, 1.0, 1.0)).unwrap();
    let before_total =
        exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw()
        + exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();

    let summary = exec.step().unwrap();

    let a_after = exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw();
    let b_after = exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();
    assert!(a_after < 10.0);
    assert!(b_after > 0.0);
    assert!(((a_after + b_after) - before_total).abs() < 0.0001);
    assert!(summary.metrics.contact_exchange_amount > 0.0);
    assert_eq!(summary.metrics.contact_exchange_pairs_count, 1);
}

#[test]
fn passive_contact_exchange_removes_only_what_target_capacity_accepts() {
    let mut config = contact_exchange_config(1.0, 1.0, 1.0, 1.0);
    config.initial_cells[1].capacity_limit = CapacityAmount::new(9.5).unwrap();
    let mut exec = TickExecutor::new(config).unwrap();

    let before_total =
        exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw()
        + exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();

    let summary = exec.step().unwrap();

    let a_after = exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw();
    let b_after = exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();
    assert!(summary.metrics.contact_exchange_amount > 0.0);
    assert!(summary.metrics.contact_exchange_amount < 2.0);
    assert!(((a_after + b_after) - before_total).abs() < 0.0001);
}

#[test]
fn local_interaction_metrics_are_deterministic_for_same_seed_and_config() {
    let mut config = contact_exchange_config(1.0, 1.0, 1.0, 1.0);
    config.local_interaction.contact_stimulus_per_overlap = 0.5;

    let mut first = TickExecutor::new(config.clone()).unwrap();
    let mut second = TickExecutor::new(config).unwrap();

    let first_tick_1 = first.step().unwrap();
    let second_tick_1 = second.step().unwrap();
    let first_tick_2 = first.step().unwrap();
    let second_tick_2 = second.step().unwrap();

    let metric_tuple = |summary: &alife::core::summary::RunSummary| {
        (
            summary.metrics.contact_pairs_count,
            summary.metrics.contact_pressure_pre_total.to_bits(),
            summary.metrics.contact_pressure_post_total.to_bits(),
            summary.metrics.contact_pressure_max_over_tick.to_bits(),
            summary.metrics.contact_exchange_amount.to_bits(),
            summary.metrics.contact_exchange_pairs_count,
            summary.metrics.contact_stimulus_generated_total.to_bits(),
            summary.metrics.contact_stimulus_readable_total.to_bits(),
        )
    };

    assert_eq!(metric_tuple(&first_tick_1), metric_tuple(&second_tick_1));
    assert_eq!(metric_tuple(&first_tick_2), metric_tuple(&second_tick_2));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_local_interaction_exchange -- passive_contact_exchange_moves_resources_down_gradient_and_conserves_total --nocapture
```

Expected: compile failure because contact exchange summary fields do not exist.

- [ ] **Step 3: Add process identity and summary fields**

Modify `src/core/process.rs`:

```rust
PassiveContactExchange,
```

Add registry entry:

```rust
ProcessSpec {
    process_id: ProcessId::PassiveContactExchange,
    status: ProcessStatus::Now,
    required_capabilities: &[
        MaterialCapability::BoundaryPermeability,
        MaterialCapability::ResourceUptake,
    ],
    description: "Passively moves internal resources between contacting cells down a resource gradient.",
},
```

Modify `src/core/summary.rs`:

```rust
pub contact_exchange_amount: f32,
pub contact_exchange_pairs_count: u32,
pub contact_exchange_rejections_no_capability: u32,
```

Modify `src/core/cell_store.rs` with a conservation-safe transfer helper:

```rust
pub fn transfer_resources_limited_by_effective_capacity(
    &mut self,
    source: CellIndex,
    target: CellIndex,
    requested: ResourceAmount,
    storage_capacity_per_unit: f32,
) -> ResourceAmount {
    let source_available = self.resource_amount(source).raw();
    let target_free = self
        .effective_free_capacity(target, storage_capacity_per_unit)
        .raw();
    let accepted_raw = requested.raw().min(source_available).min(target_free);
    let accepted = ResourceAmount::new(accepted_raw).expect("accepted transfer is clamped");
    self.resources[source.raw()] = self.resources[source.raw()].saturating_sub(accepted);
    self.resources[target.raw()] = self.resources[target.raw()].saturating_add(accepted);
    accepted
}
```

- [ ] **Step 4: Implement deterministic exchange**

In `src/core/tick.rs`, add a private helper near `run_process`:

```rust
fn has_contact_exchange_capability(
    world: &WorldState,
    index: CellIndex,
    config: &crate::core::config::LocalInteractionConfig,
) -> bool {
    let cells = world.cells();
    cells.capability_level(index, crate::core::process::MaterialCapability::BoundaryPermeability)
        >= config.min_boundary_capability
        && cells.capability_level(index, crate::core::process::MaterialCapability::ResourceUptake)
            >= config.min_transport_capability
}
```

After contact cache rebuild and before lifecycle checks, apply exchange:

```rust
let mut contact_exchange_amount = 0.0_f32;
let mut contact_exchange_pairs_count = 0_u32;
let mut contact_exchange_rejections_no_capability = 0_u32;

if config.local_interaction.enabled {
    let pairs: Vec<_> = self.world.contact_cache().pairs().to_vec();
    for pair in pairs {
        let a = pair.a;
        let b = pair.b;
        let a_res = self.world.cells().resource_amount(a).raw();
        let b_res = self.world.cells().resource_amount(b).raw();
        if (a_res - b_res).abs() <= f32::EPSILON {
            continue;
        }

        let (source, target, gradient) = if a_res > b_res {
            (a, b, a_res - b_res)
        } else {
            (b, a, b_res - a_res)
        };

        if !has_contact_exchange_capability(&self.world, source, &config.local_interaction)
            || !has_contact_exchange_capability(&self.world, target, &config.local_interaction)
        {
            contact_exchange_rejections_no_capability += 1;
            continue;
        }

        let free_target = self
            .world
            .cells()
            .effective_free_capacity(
                target,
                config.material_effects.storage_capacity_per_unit,
            )
            .raw();
        let requested = gradient
            .mul_add(config.local_interaction.contact_exchange_rate, 0.0)
            .min(config.local_interaction.max_exchange_per_pair.raw())
            .min(free_target);
        if requested <= 0.0 {
            continue;
        }

        let moved = {
            let cells = self.world.cells_mut_for_commit();
            cells.transfer_resources_limited_by_effective_capacity(
                source,
                target,
                ResourceAmount::new(requested).unwrap(),
                config.material_effects.storage_capacity_per_unit,
            )
        };

        contact_exchange_amount += moved.raw();
        contact_exchange_pairs_count += 1;
    }
}
```

Pass the three metrics into `build_metrics_summary`.

- [ ] **Step 5: Run positive test**

Run:

```powershell
cargo test --test phase2_local_interaction_exchange -- passive_contact_exchange_moves_resources_down_gradient_and_conserves_total --nocapture
```

Expected: PASS.

- [ ] **Step 6: Commit checkpoint**

```powershell
git add src/core/process.rs src/core/tick.rs src/core/summary.rs tests/phase2_local_interaction_exchange.rs
git commit -m "feat: add passive contact exchange"
```

---

### Task 5: Negative Controls For Contact Exchange

**Files:**

- Modify: `tests/phase2_local_interaction_exchange.rs`
- Modify: `src/core/tick.rs` only if test fails for the wrong reason.

- [ ] **Step 1: Write failing negative-control tests**

Append:

```rust
#[test]
fn passive_contact_exchange_rejects_when_boundary_material_missing() {
    let mut exec = TickExecutor::new(contact_exchange_config(0.0, 1.0, 1.0, 1.0)).unwrap();
    let summary = exec.step().unwrap();

    let a_after = exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw();
    let b_after = exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw();

    assert_eq!(a_after, 10.0);
    assert_eq!(b_after, 0.0);
    assert_eq!(summary.metrics.contact_exchange_amount, 0.0);
    assert_eq!(summary.metrics.contact_exchange_rejections_no_capability, 1);
}

#[test]
fn passive_contact_exchange_rejects_when_transport_material_missing() {
    let mut exec = TickExecutor::new(contact_exchange_config(1.0, 0.0, 1.0, 1.0)).unwrap();
    let summary = exec.step().unwrap();

    assert_eq!(exec.world().cells().resource_amount(CellIndex::from_raw(0)).raw(), 10.0);
    assert_eq!(exec.world().cells().resource_amount(CellIndex::from_raw(1)).raw(), 0.0);
    assert_eq!(summary.metrics.contact_exchange_amount, 0.0);
    assert_eq!(summary.metrics.contact_exchange_rejections_no_capability, 1);
}
```

- [ ] **Step 2: Run tests to verify behavior**

Run:

```powershell
cargo test --test phase2_local_interaction_exchange -- passive_contact_exchange_rejects --nocapture
```

Expected: PASS if Task 4 gating was implemented correctly. If it fails because exchange still occurs, fix only the gating helper.

- [ ] **Step 3: Commit checkpoint**

```powershell
git add src/core/tick.rs tests/phase2_local_interaction_exchange.rs
git commit -m "test: cover contact exchange negative controls"
```

---

### Task 6: Scalar Contact Stimulus With Tick Delay

**Files:**

- Modify: `src/core/cell_store.rs`
- Modify: `src/core/tick.rs`
- Modify: `src/core/summary.rs`
- Test: `tests/phase2_local_interaction_stimulus.rs`

- [ ] **Step 1: Write failing stimulus delay test**

Create `tests/phase2_local_interaction_stimulus.rs`:

```rust
use alife::core::{
    cell_store::CellIndex,
    config::{CellInitialConfig, EnvironmentConfig, LifecycleConfig, ResourceConfig, ResourceInteractionConfig, RuntimeConfig, SpaceConfig, WorldConfig},
    tick::TickExecutor,
    units::{CapacityAmount, EnergyAmount, HeatAmount, MaterialAmount, Position, Radius, ResourceAmount, Seed, Tick, WasteAmount, WorldSize},
};

fn stimulus_cell(x: f32, sensory: f32) -> CellInitialConfig {
    CellInitialConfig {
        position: Position::new(x, 10.0),
        radius: Radius::new(2.0).unwrap(),
        initial_energy: EnergyAmount::new(20.0).unwrap(),
        energy_capacity: EnergyAmount::new(20.0).unwrap(),
        mandatory_cost_per_tick: EnergyAmount::zero(),
        passive_energy_income: EnergyAmount::zero(),
        capacity_limit: CapacityAmount::new(50.0).unwrap(),
        initial_resource_amount: ResourceAmount::zero(),
        initial_boundary_material: MaterialAmount::new(1.0).unwrap(),
        initial_transport_material: MaterialAmount::zero(),
        initial_metabolic_material: MaterialAmount::zero(),
        initial_storage_material: MaterialAmount::zero(),
        initial_synthesis_material: MaterialAmount::zero(),
        initial_structural_material: MaterialAmount::new(1.0).unwrap(),
        initial_repair_material: MaterialAmount::zero(),
        initial_contractile_material: MaterialAmount::zero(),
        initial_sensory_material: MaterialAmount::new(sensory).unwrap(),
    }
}

fn contact_stimulus_config(receiver_sensory: f32) -> RuntimeConfig {
    let a = stimulus_cell(10.0, receiver_sensory);
    let b = stimulus_cell(13.0, receiver_sensory);
    let mut config = RuntimeConfig::new(
        WorldConfig {
            tick_count: Tick::from_raw(2),
            seed: Seed::from_raw(3),
            size: WorldSize::new(64.0, 64.0).unwrap(),
        },
        SpaceConfig {
            spatial_grid_size: 8.0,
            physics_solver_iterations: 1,
        },
        ResourceConfig::new(vec![ResourceAmount::zero()], 0.0).unwrap(),
        ResourceInteractionConfig::disabled(),
        a,
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
    .with_cells(vec![a, b]);
    config.local_interaction.enabled = true;
    config.local_interaction.contact_stimulus_per_overlap = 0.5;
    config.local_interaction.stimulus_decay_per_tick = 0.0;
    config
}

#[test]
fn contact_stimulus_created_in_tick_n_is_readable_in_tick_n_plus_1() {
    let mut exec = TickExecutor::new(contact_stimulus_config(1.0)).unwrap();

    assert_eq!(exec.world().cells().contact_stimulus(CellIndex::from_raw(0)), 0.0);
    let first = exec.step().unwrap();
    assert_eq!(first.metrics.contact_stimulus_readable_total, 0.0);
    assert!(first.metrics.contact_stimulus_generated_total > 0.0);

    let second = exec.step().unwrap();
    assert!(exec.world().cells().contact_stimulus(CellIndex::from_raw(0)) > 0.0);
    assert!(exec.world().cells().contact_stimulus(CellIndex::from_raw(1)) > 0.0);
    assert!(second.metrics.contact_stimulus_readable_total > 0.0);
}

#[test]
fn contact_stimulus_requires_sensory_material() {
    let mut exec = TickExecutor::new(contact_stimulus_config(0.0)).unwrap();
    exec.step().unwrap();
    let second = exec.step().unwrap();

    assert_eq!(exec.world().cells().contact_stimulus(CellIndex::from_raw(0)), 0.0);
    assert_eq!(exec.world().cells().contact_stimulus(CellIndex::from_raw(1)), 0.0);
    assert_eq!(second.metrics.contact_stimulus_readable_total, 0.0);
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_local_interaction_stimulus -- contact_stimulus_created_in_tick_n_is_readable_in_tick_n_plus_1 --nocapture
```

Expected: compile failure because `contact_stimulus` APIs and metrics do not exist.

- [ ] **Step 3: Add stimulus state to CellStore**

Modify `src/core/cell_store.rs`:

Add fields:

```rust
contact_stimulus_current: Vec<f32>,
contact_stimulus_next: Vec<f32>,
```

Initialize with capacity and push zeros in `insert_initial`.

Add methods:

```rust
pub fn contact_stimulus(&self, index: CellIndex) -> f32 {
    self.contact_stimulus_current[index.raw()]
}

pub fn add_next_contact_stimulus(&mut self, index: CellIndex, amount: f32) {
    self.contact_stimulus_next[index.raw()] =
        (self.contact_stimulus_next[index.raw()] + amount.max(0.0)).clamp(0.0, 1.0);
}

pub fn commit_contact_stimulus(&mut self, decay_per_tick: f32) {
    let decay = decay_per_tick.clamp(0.0, 1.0);
    for i in 0..self.contact_stimulus_current.len() {
        let decayed_current = self.contact_stimulus_current[i] * (1.0 - decay);
        self.contact_stimulus_current[i] =
            (decayed_current + self.contact_stimulus_next[i]).clamp(0.0, 1.0);
        self.contact_stimulus_next[i] = 0.0;
    }
}
```

- [ ] **Step 4: Generate stimulus after contact cache, commit at Tick boundary**

In `src/core/tick.rs`, capture readable stimulus for Tick N before generating new stimulus:

```rust
let contact_stimulus_readable_total_for_summary = (0..self.world.cells().len())
    .map(|i| {
        self.world
            .cells()
            .contact_stimulus(CellIndex::from_raw(i))
    })
    .sum::<f32>();
```

After contact exchange, generate Tick N stimulus into the `next` buffer only:

```rust
let mut contact_stimulus_generated_total = 0.0_f32;
if config.local_interaction.enabled && config.local_interaction.contact_stimulus_per_overlap > 0.0 {
    let pairs: Vec<_> = self.world.contact_cache().pairs().to_vec();
    for pair in pairs {
        for target in [pair.a, pair.b] {
            let sensory = self.world.cells().capability_level(
                target,
                crate::core::process::MaterialCapability::ResourceSensing,
            );
            if sensory <= 0.0 {
                continue;
            }
            let stimulus = (pair.overlap
                * config.local_interaction.contact_stimulus_per_overlap
                * sensory)
                .clamp(0.0, 1.0);
            self.world
                .cells_mut_for_commit()
                .add_next_contact_stimulus(target, stimulus);
            contact_stimulus_generated_total += stimulus;
        }
    }
}
```

Do not read `contact_stimulus_next` in `build_metrics_summary`. Near the Tick commit boundary, immediately before `self.world.advance_tick();`, commit the next buffer so it becomes readable at Tick N+1:

```rust
self.world
    .cells_mut_for_commit()
    .commit_contact_stimulus(config.local_interaction.stimulus_decay_per_tick);
```

Add metrics:

```rust
pub contact_stimulus_generated_total: f32,
pub contact_stimulus_readable_total: f32,
```

Pass:

```rust
contact_stimulus_readable_total_for_summary
```

into `build_metrics_summary`. This is the only value used for `contact_stimulus_readable_total`, so the summary returned from Tick N cannot include stimulus generated during Tick N.

- [ ] **Step 5: Run stimulus tests**

Run:

```powershell
cargo test --test phase2_local_interaction_stimulus -- --nocapture
```

Expected: PASS.

- [ ] **Step 6: Commit checkpoint**

```powershell
git add src/core/cell_store.rs src/core/tick.rs src/core/summary.rs tests/phase2_local_interaction_stimulus.rs
git commit -m "feat: add delayed scalar contact stimulus"
```

---

### Task 7: Sweep Analyzer Scenario And CSV Metrics

**Files:**

- Modify: `src/bin/sweep_analyzer.rs`
- Modify: `config/analyzer/sweep_analyzer.toml`
- Modify: `config/analyzer/sweep_analyzer_smoke.toml`
- Test: `tests/phase2_sweep_outputs.rs`
- Test: `tests/phase2_sweep_parser.rs` if preset fields are added.

- [ ] **Step 1: Write failing analyzer output test**

Add to `tests/phase2_sweep_outputs.rs`:

```rust
#[test]
fn test_local_interaction_sweep_outputs_contact_metrics() {
    let toml_str = r#"
[run]
output_dir = "target/test_local_interaction_sweep"
seed = 42
ticks = 20

[cell]
radius = 2.0
initial_energy = 20.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.0
passive_energy_income = 0.0
capacity_limit = 50.0
initial_resources = 10.0
initial_boundary_material = 1.0
initial_transport_material = 1.0
initial_metabolic_material = 0.0
initial_storage_material = 0.0
initial_synthesis_material = 0.0
initial_structural_material = 1.0
initial_repair_material = 0.0
initial_contractile_material = 0.0
initial_sensory_material = 1.0

[resource_interaction]
enabled = false
initial_resources = [0.0]
decay_rate = 0.0

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

[scenarios.local_interaction_viability]
initial_cell_count = 2
overlap_offset = 3.0
source_cell_resources = 10.0
target_cell_resources = 0.0
enable_local_interaction = true

[[sweeps]]
name = "local_interaction_viability"
scenario = "local_interaction_viability"
parameter = "contact_exchange_rate"
start = 0.0
end = 1.0
steps = 3
"#;

    let cfg: alife::bin::sweep_analyzer::AnalyzerConfig = toml::from_str(toml_str).unwrap();
    let sweep = &cfg.sweeps[0];
    let preset = cfg.scenarios.as_ref().unwrap().get("local_interaction_viability");
    alife::bin::sweep_analyzer::run_sweep(&cfg, sweep, preset, "target/test_local_interaction_sweep");

    let csv = std::fs::read_to_string("target/test_local_interaction_sweep/local_interaction_viability.csv").unwrap();
    let header = csv.lines().next().unwrap();
    assert!(header.contains("contact_pairs_count"));
    assert!(header.contains("contact_pressure_pre_total"));
    assert!(header.contains("contact_pressure_post_total"));
    assert!(header.contains("contact_pressure_max_over_tick"));
    assert!(header.contains("contact_exchange_amount"));
    assert!(header.contains("contact_stimulus_readable_total"));
    assert!(csv.lines().skip(1).any(|line| line.contains("local_interaction_viability")));
}
```

- [ ] **Step 2: Run test to verify it fails**

Run:

```powershell
cargo test --test phase2_sweep_outputs -- test_local_interaction_sweep_outputs_contact_metrics --nocapture
```

Expected: fail because scenario is not allowed and/or CSV columns are absent.

- [ ] **Step 3: Extend analyzer config structures**

In `src/bin/sweep_analyzer.rs`, extend `RawScenarioPreset`:

```rust
initial_cell_count: Option<usize>,
overlap_offset: Option<f32>,
source_cell_resources: Option<f32>,
target_cell_resources: Option<f32>,
enable_local_interaction: Option<bool>,
```

In `build_config`, when preset scenario is `local_interaction_viability`, create two initial cells:

```rust
let overlap_offset = preset.overlap_offset.unwrap_or(3.0);
let mut source = cfg.cell_initial_config();
let mut target = source;
source.position = Position::new(16.0, 16.0);
target.position = Position::new(16.0 + overlap_offset, 16.0);
source.initial_resource_amount =
    ResourceAmount::new(preset.source_cell_resources.unwrap_or(10.0)).unwrap();
target.initial_resource_amount =
    ResourceAmount::new(preset.target_cell_resources.unwrap_or(0.0)).unwrap();
config = config.with_cells(vec![source, target]);
config.local_interaction.enabled = preset.enable_local_interaction.unwrap_or(true);
```

Map sweep parameter:

```rust
"contact_exchange_rate" => config.local_interaction.contact_exchange_rate = value,
"contact_stimulus_per_overlap" => config.local_interaction.contact_stimulus_per_overlap = value,
```

Add `"local_interaction_viability"` to `allowed_scenarios`.

- [ ] **Step 4: Extend SimResult and CSV**

Add fields to `SimResult`:

```rust
contact_pairs_count: u32,
contact_pressure_pre_total: f32,
contact_pressure_post_total: f32,
contact_pressure_max_over_tick: f32,
contact_exchange_amount: f32,
contact_exchange_pairs_count: u32,
contact_exchange_rejections_no_capability: u32,
contact_stimulus_generated_total: f32,
contact_stimulus_readable_total: f32,
```

Populate from `summary.metrics`.

Add CSV columns for both 1D and matrix output paths:

```text
contact_pairs_count,contact_pressure_pre_total,contact_pressure_post_total,contact_pressure_max_over_tick,contact_exchange_amount,contact_exchange_pairs_count,contact_exchange_rejections_no_capability,contact_stimulus_generated_total,contact_stimulus_readable_total
```

- [ ] **Step 5: Add analyzer config sweeps**

Add to `config/analyzer/sweep_analyzer.toml`:

```toml
[scenarios.local_interaction_viability]
initial_cell_count = 2
overlap_offset = 3.0
source_cell_resources = 10.0
target_cell_resources = 0.0
enable_local_interaction = true

[[sweeps]]
name = "local_interaction_viability"
scenario = "local_interaction_viability"
parameter = "contact_exchange_rate"
start = 0.0
end = 1.0
steps = 6
```

Add smoke-scale version to `config/analyzer/sweep_analyzer_smoke.toml` with `steps = 3`.

- [ ] **Step 6: Run analyzer output test**

Run:

```powershell
cargo test --test phase2_sweep_outputs -- test_local_interaction_sweep_outputs_contact_metrics --nocapture
```

Expected: PASS.

- [ ] **Step 7: Commit checkpoint**

```powershell
git add src/bin/sweep_analyzer.rs config/analyzer/sweep_analyzer.toml config/analyzer/sweep_analyzer_smoke.toml tests/phase2_sweep_outputs.rs tests/phase2_sweep_parser.rs
git commit -m "feat: add local interaction sweep metrics"
```

---

### Task 8: Sweep Analyzer Guardrail Warnings

**Files:**

- Modify: `src/bin/sweep_analyzer.rs`
- Modify: `tests/phase2_sweep_warnings.rs`

- [ ] **Step 1: Write failing warning tests**

Add to `tests/phase2_sweep_warnings.rs`:

```rust
#[test]
fn test_scenario_local_interaction_viability_rate_responsive() {
    let mut slow = mock_result(false, 10.0, 0);
    slow.contact_pairs_count = 1;
    slow.contact_pressure_pre_total = 1.0;
    slow.contact_pressure_max_over_tick = 1.0;
    slow.contact_exchange_amount = 0.0;
    slow.contact_stimulus_readable_total = 0.5;

    let mut fast = slow.clone();
    fast.contact_exchange_amount = 2.0;

    let warnings = detect_warnings(&[slow, fast], "local_interaction_viability");
    assert!(!warnings.contains(&"LOW_INFORMATION_SWEEP".to_string()));
    assert!(!warnings.contains(&"LOCAL_INTERACTION_NOT_ACTIVATED".to_string()));
}

#[test]
fn test_scenario_local_interaction_viability_flags_flat_exchange() {
    let mut a = mock_result(false, 10.0, 0);
    a.contact_pairs_count = 1;
    a.contact_pressure_pre_total = 1.0;
    a.contact_pressure_max_over_tick = 1.0;
    a.contact_exchange_amount = 0.0;

    let mut b = a.clone();
    b.contact_exchange_amount = 0.0;

    let warnings = detect_warnings(&[a, b], "local_interaction_viability");
    assert!(warnings.contains(&"LOCAL_INTERACTION_EXCHANGE_FLAT".to_string()));
}

#[test]
fn test_scenario_local_interaction_viability_flags_no_contact() {
    let mut a = mock_result(false, 10.0, 0);
    a.contact_pairs_count = 0;
    a.contact_pressure_pre_total = 0.0;
    a.contact_pressure_max_over_tick = 0.0;

    let mut b = a.clone();
    b.contact_exchange_amount = 1.0;

    let warnings = detect_warnings(&[a, b], "local_interaction_viability");
    assert!(warnings.contains(&"LOCAL_INTERACTION_NOT_ACTIVATED".to_string()));
}
```

If `mock_result` has no new fields, add them with zero defaults in the helper.

- [ ] **Step 2: Run warning tests to verify failure**

Run:

```powershell
cargo test --test phase2_sweep_warnings -- local_interaction --nocapture
```

Expected: fail because warning logic does not know the local interaction scenario.

- [ ] **Step 3: Implement warning logic**

In `detect_warnings`, add scenario-specific branch:

```rust
if scenario_lower == "local_interaction_viability" {
    let any_contact = results
        .iter()
        .any(|r| r.contact_pairs_count > 0 && r.contact_pressure_max_over_tick > 0.0);
    let exchange_values: Vec<f32> = results.iter().map(|r| r.contact_exchange_amount).collect();
    let stimulus_values: Vec<f32> = results.iter().map(|r| r.contact_stimulus_readable_total).collect();

    if !any_contact {
        warnings.push("LOCAL_INTERACTION_NOT_ACTIVATED".to_string());
    }
    if !varies_meaningfully(&exchange_values, 0.001) {
        warnings.push("LOCAL_INTERACTION_EXCHANGE_FLAT".to_string());
    }
    if !varies_meaningfully(&stimulus_values, 0.001) {
        warnings.push("LOCAL_INTERACTION_STIMULUS_FLAT".to_string());
    }
    if any_contact && (varies_meaningfully(&exchange_values, 0.001) || varies_meaningfully(&stimulus_values, 0.001)) {
        warnings.retain(|w| w != "LOW_INFORMATION_SWEEP");
    }
}
```

If there is no existing helper, define:

```rust
fn varies_meaningfully(values: &[f32], min_delta: f32) -> bool {
    if values.is_empty() {
        return false;
    }
    let min = values.iter().copied().fold(f32::INFINITY, f32::min);
    let max = values.iter().copied().fold(f32::NEG_INFINITY, f32::max);
    (max - min).abs() >= min_delta
}
```

- [ ] **Step 4: Run warning tests**

Run:

```powershell
cargo test --test phase2_sweep_warnings -- local_interaction --nocapture
```

Expected: PASS.

- [ ] **Step 5: Commit checkpoint**

```powershell
git add src/bin/sweep_analyzer.rs tests/phase2_sweep_warnings.rs
git commit -m "feat: add local interaction analyzer guardrails"
```

---

### Task 9: End-To-End Phase 2F Reachability Test

**Files:**

- Create: `tests/phase2_local_interaction_reachability.rs`

- [ ] **Step 1: Write failing reachability test**

Create:

```rust
use std::process::Command;

#[test]
fn phase2f_reachability_sweep_has_contact_exchange_and_no_low_information_warning() {
    let output = Command::new("cargo")
        .args([
            "run",
            "--quiet",
            "--bin",
            "sweep_analyzer",
            "--",
            "config/analyzer/sweep_analyzer_smoke.toml",
        ])
        .output()
        .expect("sweep analyzer should run");

    assert!(
        output.status.success(),
        "stdout: {}\nstderr: {}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    let csv = std::fs::read_to_string("outputs/raw_data/smoke/local_interaction_viability.csv")
        .expect("local interaction raw csv should exist");

    let header: Vec<&str> = csv.lines().next().unwrap().split(',').collect();
    let idx = |name: &str| header.iter().position(|field| *field == name).unwrap();
    let rows: Vec<Vec<&str>> = csv.lines().skip(1).map(|line| line.split(',').collect()).collect();

    assert!(rows.iter().any(|row| row[idx("contact_pairs_count")].parse::<u32>().unwrap() > 0));
    assert!(rows.iter().any(|row| row[idx("contact_exchange_amount")].parse::<f32>().unwrap() > 0.0));
    assert!(rows.iter().any(|row| row[idx("contact_stimulus_readable_total")].parse::<f32>().unwrap() > 0.0));
    assert!(rows.iter().all(|row| !row[idx("warning_codes")].contains("LOW_INFORMATION_SWEEP")));
}
```

- [ ] **Step 2: Run test to verify it fails or passes for the right reason**

Run:

```powershell
cargo test --test phase2_local_interaction_reachability -- --nocapture
```

Expected before Tasks 7-8: fail because CSV does not exist. Expected after Tasks 7-8: PASS.

- [ ] **Step 3: Commit checkpoint**

```powershell
git add tests/phase2_local_interaction_reachability.rs
git commit -m "test: add phase 2f analyzer reachability gate"
```

---

### Task 10: Documentation And Worklog Report

**Files:**

- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2F-local-interactions.md`
- Modify: `outputs/worklogs/index.md`

- [ ] **Step 1: Run verification**

Run:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
```

Expected:

- all commands exit 0;
- `outputs/raw_data/local_interaction_viability.csv` exists;
- `warning_codes` for local interaction rows do not contain `LOW_INFORMATION_SWEEP` when contact exchange/stimulus varies;
- contact metrics are nonzero in good-condition rows.

- [ ] **Step 2: Create implementation report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2F-local-interactions.md` with:

```markdown
---
tags:
  - alife
  - worklog/report
  - phase/2F
---

# Phase 2F Local Cell-Cell Interaction Report

## Status

Phase 2F is reachability-complete and analyzer-measurable.

## Implemented

- deterministic derived contact pairs;
- contact pressure summary;
- material-gated passive contact Resource exchange;
- delayed scalar contact stimulus;
- local interaction sweep_analyzer guardrail.

## Explicitly Not Implemented

- full JointStore;
- organism-level control;
- semantic command signals;
- direct Energy or Genome transfer;
- persistent adhesion/binding.

## Verification

- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
- `cargo test --workspace --all-targets`
- `cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml`

## Phase Gate

Phase 2F can proceed when `local_interaction_viability.csv` proves:

- contact pairs are detected;
- contact pressure is observable;
- exchange requires material capability;
- exchange rate changes measurable output;
- scalar contact stimulus is measurable and delayed;
- analyzer does not report `LOW_INFORMATION_SWEEP` for the valid scenario.
```

- [ ] **Step 3: Update worklog index**

Add the report link under `## Reports`:

```markdown
- [[outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2F-local-interactions|YYYY-MM-DD-HHMM-REPORT-phase-2F-local-interactions]]
```

- [ ] **Step 4: Commit checkpoint**

```powershell
git add outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-phase-2F-local-interactions.md outputs/worklogs/index.md
git commit -m "docs: report phase 2f local interactions"
```

---

## Final Acceptance

Phase 2F is complete only when all are true:

- `contact_pair_reachable`: overlapping cells produce stable ordered contact pairs.
- `contact_pressure_reachable`: contact pressure appears in `RunSummary` and analyzer CSV.
- `passive_contact_exchange_reachable`: resources move down gradient between contacting living cells.
- `contact_exchange_reject_without_capability_reachable`: missing Boundary or Transport material prevents exchange.
- `scalar_contact_stimulus_reachable`: contact generates scalar stimulus.
- `local_interaction_determinism_reachable`: same config/seed produces identical interaction metrics.
- `sweep_analyzer guardrail`: `local_interaction_viability.csv` contains contact metrics, exchange metrics, stimulus metrics, and no false `LOW_INFORMATION_SWEEP` for meaningful runs.
- No direct Energy Buffer, Genome, MaterialFragment, or organism-label transfer is introduced.
- No full `JointStore` is introduced in Phase 2F.
- Existing Phase 1 and Phase 2A-E tests still pass.

## Required Commands

Targeted first:

```powershell
cargo test --test phase2_local_interaction_contact
cargo test --test phase2_local_interaction_exchange
cargo test --test phase2_local_interaction_stimulus
cargo test --test phase2_local_interaction_reachability
cargo test --test phase2_sweep_outputs -- local_interaction
cargo test --test phase2_sweep_warnings -- local_interaction
```

Final:

```powershell
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
```

## Self-Review

- Spec coverage: roadmap Phase 2F requirements are mapped to Tasks 1-9; final analyzer guardrail is Task 9 plus Task 10 verification.
- Placeholder scan: no open implementation placeholder is required for this plan; adhesion is explicitly deferred.
- Type consistency: `ContactCache`, `ContactPair`, `LocalInteractionConfig`, and new `MetricsSummary` fields use the same names across tests and implementation steps.
