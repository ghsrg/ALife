---
tags:
  - alife
  - implementation
  - phase/1
  - rust
  - api
---

# Phase 1 Module API

> Rust module and public API contract for the first deterministic Phase 1 implementation.

---

# Purpose

This document translates [[docs/implementation/phase-1-data-model|Phase 1 Data Model]] into Rust module boundaries and public API contracts.

It is an implementation document, not Canon. It should guide the first Rust implementation plan and prevent accidental coupling between core simulation, runner, viewer, storage and analysis.

---

# Scope

Phase 1 Module API covers:

```text
typed ids
typed amounts / value objects
validated runtime config contracts
WorldState
CellStore
ResourceGrid
EnvironmentState
SpatialIndex
Lifecycle system
Tick execution
DeltaBuffer
EventBuffer
Snapshot / viewer frame projection
run summary
```

Out of scope:

```text
Genome Runtime
ActionPlan
Feasibility Check
registered Processes
division
mutation
inheritance
Joints
signals
organism views
selection analytics
database persistence
WebSocket viewer server
```

Out-of-scope concepts may appear only as future extension boundaries.

---

# Crate Boundary

Initial repository may start as one Rust crate, but module boundaries should match the future workspace:

```text
alife-core equivalent:
  ids
  units
  config
  world
  cell
  resources
  environment
  spatial
  lifecycle
  deltas
  events
  tick
  snapshot

alife-runner equivalent:
  CLI
  TOML parsing
  scenario loading
  run loop
  output writing
```

`alife-core` modules must not depend on CLI, filesystem, database, viewer or Python tools.

---

# Module Map

Recommended module layout:

```text
src/
  lib.rs
  main.rs
  core/
    mod.rs
    ids.rs
    units.rs
    config.rs
    world.rs
    cell_store.rs
    resources.rs
    environment.rs
    spatial.rs
    lifecycle.rs
    deltas.rs
    events.rs
    tick.rs
    snapshot.rs
    summary.rs
  runner/
    mod.rs
    scenario.rs
    output.rs
```

If the project later becomes a Cargo workspace, `core/` becomes `alife-core` and `runner/` becomes `alife-runner`.

---

# Dependency Direction

Allowed core dependencies:

```text
ids <- units
config -> ids, units
cell_store -> ids, units, config
resources -> ids, units, config
environment -> units, config
spatial -> ids, units, config, cell_store read projections
lifecycle -> units, config, cell_store/environment read data
deltas -> ids, units, lifecycle
events -> ids, units, lifecycle
tick -> world, lifecycle, deltas, events
snapshot -> world read projections
summary -> events/snapshot/read-only world
```

Forbidden dependencies:

```text
core -> runner
core -> filesystem
core -> WebSocket/viewer
core -> SQLite/Parquet
core -> Python tools
core hot path -> HashMap iteration as behavior input
```

---

# ids Module

Purpose:

```text
stable typed identity for domain entities and registries
```

Types:

```rust
CellId
ResourceTypeId
MaterialTypeId
EventId
```

Reserved future ids:

```rust
JointId
GenomeId
ProcessId
OrganismViewId
```

Rules:

- ids are `Copy`, `Eq`, `Ord`, `Hash`, `Debug`.
- ids are not exposed dense indices.
- allocation order must be deterministic.
- `CellId` should not be reused inside a run.

Public API shape:

```rust
CellId::from_raw(raw: u32) -> CellId
CellId::raw(self) -> u32
```

`from_raw` is acceptable because ids are value identity wrappers. Validation belongs to storage lookup when resolving ids.

---

# units Module

Purpose:

```text
typed wrappers for behavior-critical accounting and spatial values
```

Core types:

```rust
Tick
Seed
EnergyAmount
ResourceAmount
MaterialAmount
CapacityAmount
HeatAmount
WasteAmount
Temperature
Position
Radius
WorldSize
GridCoord
Distance
```

Rules:

- accounting types have checked public constructors;
- hot internal math may use crate-private unchecked helpers only under documented invariants;
- commit boundary clamps or validates behavior-critical amounts;
- no public API should accept raw `f32` or integer for Energy/Resource/Material/capacity accounting.

Public API shape:

```rust
EnergyAmount::new(value) -> Result<EnergyAmount, AmountError>
EnergyAmount::zero() -> EnergyAmount
EnergyAmount::saturating_add(self, rhs) -> EnergyAmount
EnergyAmount::saturating_sub(self, rhs) -> EnergyAmount
EnergyAmount::clamp_max(self, max) -> EnergyAmount
EnergyAmount::raw(self) -> underlying numeric
pub(crate) EnergyAmount::add_unchecked_internal(self, rhs) -> EnergyAmount
```

For Phase 1, exact numeric representation may be chosen during implementation, but all behavior-critical values must stay behind typed wrappers.

Checked constructors and checked arithmetic belong at config, test, deserialization and commit boundaries.

Hot-loop arithmetic must not return `Result` per scalar operation. Use saturating math, bounded subtraction or narrow crate-private unchecked helpers under documented invariants, then validate or clamp the final committed value once per phase.

---

# config Module

Purpose:

```text
validated runtime configuration consumed by core
```

Types:

```rust
RuntimeConfig
WorldConfig
SpaceConfig
ResourceConfig
CellInitialConfig
EnvironmentConfig
LifecycleConfig
OutputConfig
ConfigError
```

Rules:

- TOML parsing belongs to runner.
- Core receives `RuntimeConfig`.
- Invalid config fails before Tick 0.
- Config ids and type ids must be resolved deterministically.
- Unknown Resource or Material ids are errors.

Public API shape:

```rust
RuntimeConfig::validate(raw: RawScenarioConfig) -> Result<RuntimeConfig, ConfigError>
RuntimeConfig::config_hash(&self) -> ConfigHash
```

If raw TOML structs are generated by serde later, keep them outside core or under a feature-gated adapter boundary.

---

# world Module

Purpose:

```text
root committed simulation state
```

Types:

```rust
WorldState
WorldInitError
```

Fields by responsibility:

```text
tick
config
cells
resources
environment
spatial_index
events
```

Public API shape:

```rust
WorldState::from_config(config: RuntimeConfig) -> Result<WorldState, WorldInitError>
WorldState::tick(&self) -> Tick
WorldState::cells(&self) -> &CellStore
WorldState::resources(&self) -> &ResourceGrid
WorldState::environment(&self) -> &EnvironmentState
WorldState::events(&self) -> &EventBuffer
WorldState::snapshot(&self) -> CommittedSnapshot
```

Rules:

- `WorldState` is the core source of truth.
- No external adapter mutates `WorldState`.
- World mutation happens through Tick execution/commit.

---

# cell_store Module

Purpose:

```text
World-owned SoA/hybrid storage for Cell data
```

Types:

```rust
CellStore
CellIndex
EnergyBuffer
ResourceInventory
MaterialInventory
LifecycleState
RuntimeFlags
CellDebugMetrics
CellLookupError
```

Public API shape:

```rust
CellStore::with_capacity(capacity: usize) -> CellStore
CellStore::insert_initial(cell: InitialCellState) -> CellId
CellStore::len(&self) -> usize
CellStore::is_empty(&self) -> bool
CellStore::resolve_id_cold(&self, id: CellId) -> Option<CellIndex>
CellStore::indices(&self) -> CellIndexRange
CellStore::iter_indices(&self) -> impl Iterator<Item = CellIndex>
CellStore::id_at(&self, index: CellIndex) -> CellId
CellStore::position(&self, index: CellIndex) -> Position
CellStore::energy(&self, index: CellIndex) -> EnergyBuffer
CellStore::lifecycle_state(&self, index: CellIndex) -> LifecycleState
```

Mutation API should be crate-private or commit-only:

```rust
pub(crate) fn write_energy_next(...)
pub(crate) fn swap_energy_buffers(...)
pub(crate) fn set_lifecycle_state(...)
```

Rules:

- external callers do not mutate hot columns directly;
- routine hot updates use write buffers where useful;
- debug metrics are observer-only;
- dense `CellIndex` is internal to core/storage systems.
- hot systems iterate dense `CellIndex` ranges only;
- `CellId -> CellIndex` resolution is a cold boundary API for runner, tests, events, debug and storage;
- future graph-like stores such as `JointStore` must resolve endpoint ids before hot solving, not during every constraint operation.

---

# resources Module

Purpose:

```text
external Resource storage and internal Resource accounting helpers
```

Types:

```rust
ResourceGrid
ResourceGridConfig
ResourceLayerIndex
ResourceGridError
```

Public API shape:

```rust
ResourceGrid::from_config(config: &ResourceConfig, space: &SpaceConfig) -> Result<ResourceGrid, ResourceGridError>
ResourceGrid::amount_at(resource: ResourceTypeId, coord: GridCoord) -> ResourceAmount
ResourceGrid::set_amount_at(...)
ResourceGrid::decay_or_passive_update(...)
```

Rules:

- grid storage is flat;
- internal Cell inventory is not ResourceGrid;
- diffusion is out of Phase 1 unless explicitly enabled as simple decay/passive placeholder;
- future diffusion uses ping-pong flat buffers and deterministic stencil operations.

---

# environment Module

Purpose:

```text
Phase 1 heat/waste/ambient accounting
```

Types:

```rust
EnvironmentState
EnvironmentConfig
EnvironmentDelta
EnvironmentError
```

Public API shape:

```rust
EnvironmentState::from_config(config: &EnvironmentConfig) -> EnvironmentState
EnvironmentState::heat(&self) -> HeatAmount
EnvironmentState::waste(&self) -> WasteAmount
EnvironmentState::apply_passive_update(&self, config: &EnvironmentConfig) -> EnvironmentDelta
EnvironmentState::commit(delta: EnvironmentDelta)
```

Rules:

- Phase 1 heat/waste are explicit accounting variables.
- Full thermodynamics and field diffusion are out of Phase 1.
- Threshold checks are deterministic and configured.

---

# spatial Module

Purpose:

```text
derived spatial index for locality and future neighbor queries
```

Types:

```rust
SpatialIndex
SpatialConfig
SpatialBuildScratch
SpatialError
```

Required build algorithm:

```text
positions -> grid cell ids -> counts -> prefix sums -> flat sorted cell indices
```

Public API shape:

```rust
SpatialIndex::new(config: &SpaceConfig, world_size: WorldSize) -> SpatialIndex
SpatialIndex::rebuild(&mut self, cells: &CellStore, scratch: &mut SpatialBuildScratch)
SpatialIndex::query_neighbors(coord: GridCoord) -> CellRangeIterator
```

Rules:

- `SpatialIndex` is derived and rebuildable.
- no hot `HashMap<GridCoord, Vec<CellId>>`;
- no behavior depends on unordered iteration;
- Phase 1 may return empty/small ranges, but the API must support scalable rebuild.

---

# lifecycle Module

Purpose:

```text
deterministic Phase 1 lifecycle transition logic
```

Types:

```rust
LifecycleState
LifecycleInput
LifecycleDecision
LifecycleReason
LifecycleConfig
```

Public API shape:

```rust
evaluate_lifecycle(input: LifecycleInput, config: &LifecycleConfig) -> LifecycleDecision
```

Rules:

- lifecycle logic is pure over explicit input;
- transition priority is `death > dormancy > stressed > alive`;
- debug metrics are not input;
- output is committed through Tick/Delta/CellStore boundary.

---

# deltas Module

Purpose:

```text
ordered semantic state changes before commit
```

Types:

```rust
Delta
DeltaBuffer
PartitionDeltaBuffers
CommitError
CommitSummary
```

Minimum delta variants:

```rust
EnergyChanged
EnvironmentChanged
LifecycleChanged
RuntimeFlagsChanged
DebugMetricsChanged
CellDied
```

Public API shape:

```rust
DeltaBuffer::with_capacity(capacity: usize) -> DeltaBuffer
DeltaBuffer::push(delta: Delta)
DeltaBuffer::clear_retain_capacity()
DeltaBuffer::sort_for_commit()
DeltaBuffer::iter()
PartitionDeltaBuffers::with_partitions(count, per_partition_capacity)
PartitionDeltaBuffers::buffer_for_partition(partition_id)
PartitionDeltaBuffers::merge_deterministic(target: &mut DeltaBuffer)
```

Rules:

- routine scalar hot updates should not become per-cell delta spam;
- delta memory is reused;
- Phase 1 may use a single-threaded `DeltaBuffer`;
- parallel future systems use per-partition buffers and deterministic merge;
- no hot phase should push to a shared synchronized global delta buffer.

---

# events Module

Purpose:

```text
deterministic event log for replay/debug/storage
```

Types:

```rust
Event
EventKind
EventBuffer
EventId
```

Minimum event kinds:

```rust
RunStarted
TickCommitted
MandatoryCostFailed
LifecycleChanged
CapacityWarning
HeatWarning
WasteWarning
CellDead
SnapshotEmitted
RunFinished
```

Public API shape:

```rust
EventBuffer::with_capacity(capacity: usize) -> EventBuffer
EventBuffer::push(event: Event) -> EventId
EventBuffer::clear_tick_events_retain_capacity()
EventBuffer::iter_ordered()
```

Rules:

- event order is deterministic;
- debug-only high-volume events may be sampled later;
- mandatory failure and lifecycle transition events are not sampled;
- routine successful accounting, such as mandatory cost paid, is stored as state or summary, not emitted as one event per Cell per Tick.

`MandatoryCostPaid` is not a default event. If it is ever needed for debugging, it must be behind explicit sampled/selected-cell debug tracing. Phase 1 success state is represented by `RuntimeFlags.mandatory_paid` and aggregate metrics.

---

# tick Module

Purpose:

```text
one deterministic Phase 1 Tick execution
```

Types:

```rust
TickState
TickSnapshot
TickExecutor
TickError
```

Public API shape:

```rust
TickExecutor::new(config: RuntimeConfig) -> TickExecutor
TickExecutor::world(&self) -> &WorldState
TickExecutor::step(&mut self) -> Result<CommitSummary, TickError>
TickExecutor::run_until(tick_count: Tick) -> Result<RunSummary, TickError>
```

Phase 1 Tick responsibilities:

```text
read snapshot
mandatory cost accounting
environment passive accounting
lifecycle evaluation
hot buffer commit/swap
semantic delta commit
event emission
summary update
```

Rules:

- no viewer/storage authority;
- no Genome/Joint/Process execution;
- routine successful accounting updates flags and summaries, not per-cell success events;
- same config + seed + binary/platform mode produces same committed states/events.

---

# snapshot Module

Purpose:

```text
committed read-only projections for viewer/storage/tests
```

Types:

```rust
CommittedSnapshot
ViewerFrame
SnapshotError
```

Public API shape:

```rust
CommittedSnapshot::from_world(world: &WorldState) -> CommittedSnapshot
ViewerFrame::from_snapshot(snapshot: &CommittedSnapshot) -> ViewerFrame
```

Rules:

- projections do not mutate `WorldState`;
- snapshots may omit internal-only fields;
- viewer frame is not source of truth.

---

# summary Module

Purpose:

```text
machine-readable run result compatible with early-stability outputs
```

Types:

```rust
RunSummary
SurvivalResult
CollapseReason
MetricsSummary
```

Public API shape:

```rust
RunSummary::from_world(world: &WorldState, config_hash: ConfigHash) -> RunSummary
```

Required vocabulary:

```text
SurvivalResult:
  Stable
  Fragile
  Collapse
  Invalid

CollapseReason:
  None
  InvalidConfig
  EnergyDepleted
  MandatoryCostUnpaid
  CapacityExceeded
  HeatLimitExceeded
  WasteLimitExceeded
  MinimumViabilityMaterialsMissing
  DeterminismMismatch
  ViewerAuthorityViolation
```

`MetricsSummary` is observer-only.

---

# runner Boundary

Runner responsibilities:

```text
parse TOML
build raw scenario config
call RuntimeConfig validation
create TickExecutor
run scenario
write summary/events/snapshots
invoke benchmarks or smoke tests
```

Runner must not implement simulation rules.

Core should be testable without runner.

---

# Error Model

Use explicit error enums per module:

```rust
ConfigError
WorldInitError
AmountError
CellLookupError
ResourceGridError
TickError
CommitError
SnapshotError
```

Rules:

- invalid config is recoverable and reported before Tick 0;
- internal invariant violations should be test failures or explicit `TickError`;
- public APIs should not panic for normal invalid user config;
- panic is acceptable only for documented internal logic errors.

---

# Test API Requirements

Tests should be able to construct:

```text
minimal valid RuntimeConfig
one-cell WorldState
single TickExecutor
synthetic LifecycleInput
synthetic CellStore with deterministic CellId
```

Required test categories:

```text
ids allocate deterministically
amount wrappers reject invalid values
config validation rejects invalid scenario
WorldState initializes from config
mandatory cost updates Energy
Energy clamps at capacity
capacity accounting ignores Energy and includes Resources/Materials
lifecycle priority is deterministic
events emit in stable order
snapshot cannot mutate WorldState
same scenario run produces same RunSummary
```

---

# Future Extension Boundaries

Do not implement these in Phase 1, but keep module names available for future design:

```text
genome_runtime
process_registry
feasibility
process_execution
joint_store
signals
organism_view
analytics
```

Future modules must follow the same rules:

- typed ids;
- World-owned storage;
- no Cell-owned object graphs;
- deterministic scheduler phases;
- observer metrics do not affect behavior.

---

# Acceptance Criteria

This module API is ready for coding when:

```text
every Phase 1 data model concept has a module owner
core does not depend on runner/viewer/storage
public APIs expose typed wrappers, not raw accounting primitives
mutation is limited to Tick/commit boundaries
runner can parse/load/run without owning simulation rules
tests can build a one-cell deterministic smoke world
future Genome/Joint/Process concepts remain placeholders only
```

---

# Semantic Links

- implements: [[docs/implementation/phase-1-data-model|Phase 1 Data Model]]
- refines: [[docs/implementation/phase-1-design|Phase 1 Design]]
- follows: [[docs/implementation/architecture|Architecture]]
- constrained by: [[docs/engine/performance|Performance]]
- constrained by: [[docs/implementation/optimization-paths|Optimization Paths]]
- constrained by: [[docs/engine/scheduler|Scheduler]]
- follows accepted: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]

# Related Documents

- `docs/implementation/phase-1-data-model.md`
- `docs/implementation/phase-1-design.md`
- `docs/implementation/architecture.md`
- `docs/engine/performance.md`
- `docs/engine/scheduler.md`
- `docs/engine/technology-stack.md`
- `docs/implementation/optimization-paths.md`
