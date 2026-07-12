---
tags:
  - alife
  - implementation
  - phase/1
  - rust
  - data-model
---

# Phase 1 Data Model

> Rust-side data model for the deterministic Phase 1 world smoke.

---

# Purpose

This document defines the Phase 1 data model for the first Rust implementation.

It is an implementation specification, not Canon. Canon documents, `docs/PRINCIPLES.md`, ADRs and accepted architecture remain authoritative above this file.

Phase 1 proves a minimal deterministic world:

```text
World + one Cell + Resources + Energy Buffer + mandatory costs + simple lifecycle + committed outputs
```

The data model must be small enough to implement and test first, but strict enough that later phases can extend it without replacing the core ownership model.

---

# Scope

Phase 1 includes active runtime data for:

```text
WorldState
TickState
CellStore
ResourceGrid
EnvironmentState
SpatialIndex
DeltaBuffer
EventBuffer
Snapshot / viewer frame projection
validated runtime config
```

Phase 1 does not implement active runtime data for:

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
organism-like views
selection analytics
```

Those concepts may appear only as explicit future extension boundaries. They must not participate in Phase 1 hot state.

---

# Modeling Decision

Use a custom data-oriented model:

```text
typed ids
typed accounting wrappers
central World-owned storage
SoA / hybrid storage for hot state
double-buffered hot columns where direct per-Tick overwrite is cheaper than deltas
preallocated transient buffers where deltas/events are required
read snapshot -> systems -> deltas -> deterministic commit
observer-only projections after commit
```

Do not model the world as an object graph of mutable entities.

Rejected defaults for Phase 1 hot state:

```text
Cell holding references to other world objects
Rc<RefCell<Cell>>
Arc<Mutex<Cell>>
Vec<Box<Cell>>
HashMap<CellId, Cell> for per-Tick hot iteration
raw f32 for Energy / Resource / Material / capacity accounting
viewer or storage state as simulation input
```

---

# Rust Modeling Principles

## Identity Is Separate From Data

Domain identity should be typed ids:

```rust
CellId
ResourceTypeId
MaterialTypeId
EventId
```

An id is not a dense index. Storage may maintain an internal dense index, but behavior-facing APIs should use typed ids.

Phase 1 may run one Cell, but it should still use `CellId` and `CellStore` so Phase 2+ does not need a rewrite.

## Accounting Uses Typed Wrappers

Behavior-critical amounts must not be raw primitives in public core APIs:

```rust
EnergyAmount
ResourceAmount
MaterialAmount
CapacityAmount
HeatAmount
WasteAmount
Tick
```

Implementation may choose fixed-point or integer scaled units later. The model requires typed boundaries either way.

Continuous spatial values may use `f32` with deterministic order:

```rust
Position
Radius
Distance
Temperature
```

Any continuous value that crosses a behavior threshold must be wrapped and validated.

Hot-loop rule:

```text
validated construction at boundaries
unchecked/internal arithmetic only inside narrow hot kernels
validation/clamp at commit boundary
```

Rust wrappers should make the safe path obvious. If unchecked constructors or arithmetic are needed later, they must be internal APIs with documented invariants and tests. Do not use `unsafe` only to avoid normal validation cost until profiling shows it is needed.

## World Owns Persistent State

Persistent simulation state belongs to `WorldState`.

Systems receive read-only snapshots and emit deltas. They do not store long-lived references to world entities.

## Derived Data Is Not Authority

`SpatialIndex`, viewer frames, debug summaries and metrics are derived from committed state.

They may accelerate lookup or help inspection, but they must not become source-of-truth state.

---

# Core Data Categories

## Entity Identity

Active Phase 1 ids:

```text
CellId
ResourceTypeId
MaterialTypeId
EventId
```

Reserved future ids:

```text
JointId
GenomeId
ProcessId
OrganismViewId
```

Reserved ids must not imply active stores in Phase 1. They are naming boundaries for later documents.

## Value Objects

Minimum value objects:

```text
Tick
Position
Radius
WorldSize
GridCoord
EnergyAmount
ResourceAmount
MaterialAmount
CapacityAmount
HeatAmount
WasteAmount
Temperature
Seed
```

Value objects should have validated constructors where invalid values are possible.

Examples:

```text
Radius > 0
EnergyAmount >= 0
CapacityAmount >= 0
Tick >= 0
ResourceAmount >= 0
MaterialAmount >= 0
```

## Persistent State

Persistent committed state:

```text
WorldState
CellStore
ResourceGrid
EnvironmentState
```

## Transient Tick State

Per-Tick transient state:

```text
TickState
TickSnapshot
DeltaBuffer
EventBuffer
CommitSummary
```

Transient state may be reused between ticks to avoid allocations.

## Derived State

Derived or rebuildable state:

```text
SpatialIndex
NeighborCache
ViewerFrame
DebugMetrics
```

Phase 1 may keep `NeighborCache` empty or minimal because the first world has one Cell.

---

# WorldState

`WorldState` is the root committed state owned by `alife-core`.

Conceptual shape:

```text
WorldState
  tick: Tick
  config: RuntimeConfig
  cells: CellStore
  resources: ResourceGrid
  environment: EnvironmentState
  spatial_index: SpatialIndex
  events: EventBuffer
```

Rules:

- `WorldState` is the source of truth during headless simulation.
- Viewer, storage and analysis read committed projections only.
- `WorldState` must be serializable or projectable into deterministic snapshots.
- `WorldState` should not depend on CLI, filesystem, WebSocket, SQLite, Parquet or Python tools.

---

# RuntimeConfig

`RuntimeConfig` is the validated representation of TOML config.

Minimum groups:

```text
WorldConfig
SpaceConfig
ResourceConfig
CellInitialConfig
EnvironmentConfig
LifecycleConfig
OutputConfig
```

Rules:

- TOML parsing belongs to runner/config adapter.
- Core receives validated config, not arbitrary TOML tables.
- Invalid config must fail before Tick 0.
- Config values that affect behavior must be typed at the core boundary.

Minimum Phase 1 config fields match [[docs/implementation/phase-1-design|Phase 1 Design]].

---

# CellStore

`CellStore` owns all committed Cell data.

Phase 1 may contain one Cell, but storage should be shaped for dense iteration:

```text
CellStore
  ids: Vec<CellId>
  positions: Vec<Position>
  radii: Vec<Radius>
  resources: Vec<ResourceInventory>
  materials: Vec<MaterialInventory>
  energy_buffers: Vec<EnergyBuffer>
  temperatures: Vec<Temperature>
  lifecycle_states: Vec<LifecycleState>
  runtime_flags: Vec<RuntimeFlags>
  debug_metrics: Vec<CellDebugMetrics>
```

This is a hybrid SoA model:

- hot scalar columns stay separate;
- inventories may be small fixed arrays or compact per-cell vectors;
- debug metrics are cold and may be split later.

For fields updated every Tick, Phase 1 should prefer double buffering:

```text
energy_read / energy_write
lifecycle_read / lifecycle_write if needed
runtime_flags_read / runtime_flags_write if needed
environment_read / environment_write for grid/layer expansion
```

Systems read the committed/read side and write the next side linearly. Commit swaps buffers or copies only the small committed summary. This avoids thousands of tiny deltas for routine per-cell updates.

Use `DeltaBuffer` for semantic changes that must be validated, ordered, logged or merged from parallel partitions. Do not generate one heap-allocated delta object per scalar update in the hot loop.

Rules:

- `CellId` is stable identity.
- Dense index is internal storage detail.
- hot systems must iterate dense indices, not resolve `CellId` inside hot loops.
- `CellId -> dense index` lookup is a cold boundary operation for tests, runner, storage, events and debug tools.
- A Cell does not own references to World, Resources, Fields or other Cells.
- A Cell does not read observer metrics.

---

# Cell State Subset

Phase 1 Cell state:

```text
CellState
  id
  position
  radius
  resources
  materials
  energy_buffer
  temperature
  lifecycle_state
  runtime_flags
  debug_metrics
```

`CellState` may exist as a read-only projection or fixture type, but persistent hot storage should remain in `CellStore`.

Do not add role-specific cell classes.

Rejected examples:

```text
NeuronCell
MuscleCell
PredatorCell
PlantCell
```

All future differentiation must emerge from Materials, Resources, Genome Runtime and state, not hardcoded classes.

---

# EnergyBuffer

Conceptual shape:

```text
EnergyBuffer
  current: EnergyAmount
  capacity: EnergyAmount
```

Invariants:

```text
0 <= current <= capacity
capacity >= 0
```

Rules:

- Energy Buffer is local Cell state.
- Energy Buffer is not Resource.
- Energy Buffer is not Material.
- Energy Buffer is not transferred between independent Cells.
- Energy Buffer does not occupy internal capacity directly.
- Energy capacity may later be derived from storage-capable Materials.

Phase 1 may configure `capacity` directly, but must not allow unbounded Energy.

---

# ResourceInventory

`ResourceInventory` stores internal Resources held by a Cell.

Conceptual shape:

```text
ResourceInventory
  amounts_by_resource_type: compact array or small map
```

Rules:

- Resources are material substances.
- Resource amounts count toward Cell capacity.
- Unknown `ResourceTypeId` is invalid config or invalid delta.
- Phase 1 may keep resource type count small and config-defined.
- Resource transfer between Cells is out of scope.

Hot-path preference:

```text
small fixed resource type count -> Vec<ResourceAmount> indexed by ResourceTypeIndex
```

Do not expose `ResourceTypeIndex` as stable domain identity.

---

# MaterialInventory

`MaterialInventory` stores internal Materials held by a Cell.

Conceptual shape:

```text
MaterialInventory
  amounts_by_material_type: compact array or small map
```

Rules:

- Materials define capabilities in later phases.
- Phase 1 only needs material presence for capacity and minimum viability checks.
- Material amounts count toward Cell capacity.
- Unknown `MaterialTypeId` is invalid config or invalid delta.

Phase 1 does not execute material synthesis, repair or degradation.

---

# Capacity Accounting

Minimum Phase 1 accounting:

```text
used_capacity =
  sum(internal Resource amounts)
  + sum(internal Material amounts)
  + genome_capacity_placeholder
  + internal_fragments_capacity_used

free_capacity =
  capacity_limit - used_capacity
```

Invariants:

- Energy Buffer does not count directly toward capacity.
- Resources and Materials do count toward capacity.
- If `used_capacity > capacity_limit`, the Cell is over capacity.
- If overrun exceeds `critical_capacity_overrun`, scenario may collapse.

Phase 1 may set future placeholders to zero:

```text
genome_capacity_placeholder = 0
internal_fragments_capacity_used = 0
```

These placeholders must be explicit if present.

---

# EnvironmentState

Phase 1 environment is a minimal local accounting model:

```text
EnvironmentState
  ambient_temperature
  heat_current
  heat_generated_per_tick
  heat_dissipation_rate
  heat_warning_threshold
  heat_death_threshold
  waste_current
  waste_generated_per_tick
  waste_sink_rate
  waste_warning_threshold
  waste_death_threshold
```

Rules:

- Heat and waste are explicit accounting variables in Phase 1.
- Full thermodynamics is out of scope.
- Heat damage works through configured thresholds.
- Waste toxicity works through configured thresholds.
- Environment state must not be hidden inside viewer or analytics.

Later phases may split this into field layers or sparse grids.

---

# ResourceGrid

Phase 1 `ResourceGrid` represents external Resource availability.

Conceptual shape:

```text
ResourceGrid
  resource_type_ids
  quantities
  optional_decay_rates
```

Phase 1 may implement this as:

```text
single global/local bucket
or
small uniform grid compatible with future spatial expansion
```

The model should prefer a uniform grid boundary because Phase 1 already accepts `spatial_grid_size`.

Implementation shape:

```text
ResourceGrid
  width
  height
  resource_type_count
  quantities: flat Vec<ResourceAmount>
  scratch: flat Vec<ResourceAmount>
```

Indexing should be explicit and deterministic:

```text
index = resource_type * width * height + y * width + x
```

For Phase 1, a local/global bucket is acceptable only as a temporary smoke shortcut. The data model must allow a flat grid without replacing public contracts.

Rules:

- ResourceGrid is source-of-truth for external Resources.
- Internal Cell inventory is separate from ResourceGrid.
- Passive energy income is a Phase 1 placeholder, not a final metabolism model.

Future diffusion/field requirements:

- use flat arrays, not nested `Vec<Vec<_>>`;
- use ping-pong buffers for stencil updates;
- use configured cadence for expensive diffusion layers;
- tile/chunk loops for cache locality before adding SIMD or parallelism;
- allow grid resolution to differ from world-space units, for example one resource cell covering multiple `su`;
- preserve a path to sparse chunks or dirty-region processing when most of the world is empty;
- benchmark release builds before accepting Rayon/SIMD/GPU complexity.

Do not assume a library or SIMD path is fast enough without measurement on the target hardware.

Scale-up guardrail:

```text
Phase 1 / target world:
  flat dense grid is acceptable.

Larger worlds:
  downsampled resource grid
  sparse or chunked active regions
  scheduled diffusion cadence
  no full-world dense diffusion by default
```

---

# SpatialIndex

`SpatialIndex` is derived from committed Cell positions.

Conceptual shape:

```text
SpatialIndex
  grid_size
  cell_counts_by_grid_coord
  cell_offsets_by_grid_coord
  sorted_cell_ids
```

Rules:

- Derived and rebuildable.
- Not source of truth for Cell existence.
- Must be deterministic for same committed state.
- Phase 1 may rebuild it each Tick because one Cell is cheap.

Avoid O(n^2) neighbor search as a long-term default, even if Phase 1 has one Cell.

Required scalable construction algorithm:

```text
1. compute grid cell id for every Cell position
2. count cells per grid cell into a preallocated counts array
3. prefix-sum counts into offsets
4. scatter CellId / dense cell index into one flat sorted array
5. query neighbors by scanning adjacent grid-cell ranges
```

This is a counting-sort / prefix-sum spatial hash grid. It is O(N + grid_cells), deterministic, cache-friendly and avoids per-grid-cell heap allocation.

Avoid for hot rebuild:

```text
HashMap<GridCoord, Vec<CellId>>
Vec<Vec<CellId>> rebuilt with fresh allocations
unordered iteration as behavior input
```

Parallel construction is a future optimization. If introduced, each partition must write to deterministic local counts/buffers and merge in stable partition order.

Large-world guardrail:

```text
Prefix-sum over every possible grid cell is acceptable only while grid cell count is bounded.
If world size grows much faster than active Cell count, use a chunked grid or fixed-size spatial hash.
```

Future fixed-size spatial hashing must still preserve deterministic bucket order and stable neighbor iteration.

---

# LifecycleState

Phase 1 lifecycle states:

```text
Alive
Stressed
Dormant
Dead
```

Rules:

- Lifecycle state is behavior state, not debug label.
- Death is terminal in Phase 1.
- Dormancy reduces mandatory cost using configured modifier.
- Stress is reachable through Energy, heat, waste or capacity warning thresholds.

Transition priority:

```text
death > dormancy > stressed > alive
```

This priority must be deterministic and tested.

---

# RuntimeFlags

Phase 1 runtime flags:

```text
mandatory_paid
stalled
over_capacity
inert
```

Rules:

- Runtime flags describe current Tick/lifecycle execution state.
- Runtime flags are not Genome memory.
- Runtime flags are not observer metrics.
- Runtime flags may be reset or recomputed during Tick execution.

Future flags such as `division_ready`, `blocked_by_cooldown` or `mandatory_paid` for process phases belong to later phase documents.

---

# CellDebugMetrics

Phase 1 debug metrics:

```text
age_ticks
energy_balance_snapshot
capacity_snapshot
last_rejection_reasons
```

Rules:

- Debug metrics are observer-only.
- Debug metrics must not affect behavior.
- Debug metrics may be disabled or split from hot storage later.
- Derived summaries such as `stress_level` must remain debug summaries, not Cell input.

---

# TickState

`TickState` is transient state for one Tick.

Conceptual shape:

```text
TickState
  tick
  snapshot
  delta_buffer
  commit_summary
```

Rules:

- Snapshot is read-only.
- Systems write deltas, not direct state mutation.
- Commit applies deltas in deterministic order.
- TickState should reuse buffers where possible.

---

# DeltaBuffer

`DeltaBuffer` stores proposed state changes before commit.

Minimum Phase 1 delta kinds:

```text
EnergyChanged
EnvironmentChanged
LifecycleChanged
RuntimeFlagsChanged
DebugMetricsChanged
CellDied
```

Rules:

- Deltas are transient.
- Deltas are ordered deterministically before commit.
- Invalid deltas are rejected or converted into explicit failure events.
- Systems must not mutate committed state directly.
- DeltaBuffer memory is preallocated and reused with `clear`, not recreated each Tick.
- Routine hot scalar updates should use double-buffered arrays instead of per-cell delta records.

Phase 1 does not need a general process delta model yet.

When parallel systems arrive, use one delta buffer per deterministic partition and merge buffers in stable partition order.

---

# EventBuffer

`EventBuffer` stores deterministic events after validation or commit.

Minimum event kinds:

```text
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

Event ordering:

```text
tick
event kind stable order
entity id
sequence number if needed
```

Events are for replay/debug/storage. Events do not feed behavior back into the same run.

Event memory policy:

- preallocate expected per-Tick capacity;
- reuse buffers between ticks;
- allow configurable sampling for debug-only high-volume events;
- keep mandatory failures, lifecycle transitions and collapse events deterministic and unsampled;
- do not emit a per-Cell success event for routine mandatory cost payment.

Routine successful accounting is represented through hot state, aggregate counters and `RuntimeFlags.mandatory_paid`. If detailed paid-cost tracing is needed, it must be explicit debug sampling for selected cells or selected ticks.

---

# Snapshot And Viewer Frame

Snapshot projection:

```text
CommittedSnapshot
  tick
  cells projection
  environment projection
  event refs
```

Viewer frame projection:

```text
ViewerFrame
  tick
  cell positions
  cell radii
  lifecycle colors/status
  resource layer summary
  heat/waste summary
```

Rules:

- Viewer reads committed projections only.
- Viewer frame is not source of truth.
- Viewer frame may omit fields that are not needed visually.
- Frame extraction must not mutate `WorldState`.
- full-world JSON frames are allowed only for tiny debug scenarios.
- real-time viewer output should use viewport filtering, LOD summaries or compact binary frames.

Scale-up viewer guardrail:

```text
camera viewport -> visible cells only
zoomed out -> density / aggregate maps
large resource fields -> summarized tiles, not every grid value
storage/replay -> binary snapshots and event logs
```

---

# Tick Data Flow

Phase 1 data flow:

```text
validated RuntimeConfig
  -> initial WorldState
  -> TickSnapshot
  -> mandatory accounting system writes hot write buffers
  -> environment accounting system writes hot write buffers
  -> lifecycle system writes hot write buffers and semantic deltas/events
  -> DeltaBuffer for ordered semantic changes
  -> deterministic commit
  -> swap/write-buffer commit for hot columns
  -> EventBuffer
  -> committed Snapshot / ViewerFrame
```

The first implementation may combine systems in code, but the data boundaries should remain visible.

---

# Ownership And Mutation Boundaries

Allowed mutation:

```text
WorldState initialization before Tick 0
DeltaBuffer writes during systems
WorldState mutation during commit
EventBuffer append during ordered event emission
```

Disallowed mutation:

```text
systems mutating read snapshot directly
viewer mutating WorldState
storage mutating WorldState
debug metrics changing behavior
observer analytics feeding back into same run
```

Borrowing guideline:

```text
read snapshot first
collect deltas second
commit with scoped mutable access third
```

Do not fix ownership problems with long-lived references, `Rc<RefCell<_>>`, `Arc<Mutex<_>>` or broad cloning.

---

# Determinism Rules

Phase 1 deterministic replay contract:

```text
same config
same initial state
same seed
same engine version
same deterministic mode
same binary/platform mode
=> same committed states, events and summary
```

Data model requirements:

- no unordered map iteration in behavior;
- deterministic event order;
- deterministic delta commit order;
- explicit RNG boundary even if Phase 1 does not use random mutation;
- stable config hashing;
- typed ids allocated deterministically.

Performance optimizations must preserve these rules. If an optimization requires nondeterministic merge order, unordered reduction or platform-dependent behavior, it is not allowed in deterministic mode.

---

# Error And Validation Model

Config validation errors happen before Tick 0.

Runtime validation errors become explicit events or collapse reasons.

Minimum error categories:

```text
InvalidConfig
UnknownResourceType
UnknownMaterialType
NegativeAmount
CapacityExceeded
EnergyCapacityExceeded
InvalidLifecycleTransition
ViewerAuthorityViolation
DeterminismMismatch
```

Phase 1 run result vocabulary:

```text
stable
fragile
collapse
invalid
```

Phase 1 collapse reasons follow [[docs/implementation/phase-1-design|Phase 1 Design]].

---

# Future Extension Boundaries

## Genome

Phase 1 does not store active Genome Runtime state.

Reserved boundary:

```text
GenomeId
GenomeCarrierState
```

These must be designed in Phase 3 before entering hot state.

Phase 3 requirement:

```text
CellStore.next_genome_tick: Vec<Tick>
CellStore.last_regulatory_outputs: future cold/hot split
scheduled Genome Runtime cadence
batched inference over due cells only
deterministic staggering and merge order
```

Genome Runtime must not run for every Cell every Tick unless profiling and scenario requirements justify it.

## Joint

Phase 1 does not store active Joints.

Reserved boundary:

```text
JointId
JointStore
```

Joints enter in a later phase after Cell/Resource/Space data is stable.

Phase 2H requirement:

```text
JointStore
  endpoint_a: Vec<CellId or dense CellIndex>
  endpoint_b: Vec<CellId or dense CellIndex>
  rest_length: Vec<Distance>
  integrity: Vec<Normalized>
  channel_flags: Vec<JointChannelFlags>
```

Joint solving is random-access by nature. The first design must avoid making Joints part of Cell-owned object graphs.

Parallel Joint solving requires a conflict strategy:

```text
graph coloring
partitioned independent joint batches
or deterministic accumulate-and-commit corrections
```

If none of these is acceptable for the target scale, Joint physics needs deeper research before Phase 2H implementation.

## Processes And Feasibility

Phase 1 does not execute `Action / Process Registry`.

Reserved boundary:

```text
ProcessId
ActionPlan
FeasibilityResult
ProcessProgress
```

Mandatory upkeep in Phase 1 is direct lifecycle accounting, not general registered process execution.

Phase 2 requirement:

- process outputs should write compact action/process buffers, not allocate one object per action;
- Feasibility should operate over dense candidate arrays and write compact result arrays;
- process progress should be SoA-compatible for long-running processes;
- mandatory consumers must remain semantically mandatory even if scheduler budgets are introduced.

## OrganismView

Phase 1 does not create organism-like runtime state.

Reserved boundary:

```text
OrganismViewId
```

Any future `OrganismView` is observer-only and must not become Cell input.

---

# Memory And Cache Guardrails

Phase 1 should stay simple, but the implementation must not block later cache-aware optimization.

Required design direction:

```text
hot columns:
  compact, linearly iterated, double-bufferable

cold columns:
  debug metrics, observer summaries, names, trace details

flags:
  bitsets, u8/u16 or compact flag words where possible

values with small bounded domains:
  may use scaled integers later

values always read together:
  may use small hybrid AoS structs
```

Example hybrid AoS candidate:

```rust
CellPhysics {
  position,
  radius,
}
```

This is allowed because position and radius are normally read together for spatial indexing and viewer projection. Do not turn the whole Cell into an object graph.

Implementation must measure:

```text
hot state bytes per Cell
allocations per Tick
snapshot extraction cost
spatial rebuild cost
resource/grid update cost
```

These measurements decide when to introduce compact numeric types, domain decomposition, sparse grids or SIMD.

---

# Testing Targets

Phase 1 data model should support tests for:

```text
config validation rejects invalid inputs
initial WorldState is deterministic
CellId allocation is deterministic
Energy Buffer clamps to capacity
mandatory cost changes Energy predictably
capacity accounting excludes Energy and includes Resources/Materials
stress/dormancy/death transitions follow priority order
heat and waste thresholds produce expected warnings/collapse
deltas commit in deterministic order
events are ordered deterministically
viewer frame cannot mutate WorldState
same config + seed produces same summary/events
```

Performance smoke tests should measure:

```text
ticks/sec headless
allocations per Tick
snapshot/frame extraction cost
```

---

# Acceptance Criteria

This data model is acceptable when:

```text
it can represent every Phase 1 required scenario
it does not include active Genome/Joint/Process runtime state
it preserves deterministic Tick boundaries
it keeps viewer/storage/analytics outside behavior
it uses typed wrappers for behavior-critical accounting
it avoids object graph ownership traps
it gives clear Rust module boundaries for implementation planning
```

---

# Semantic Links

- refines: [[docs/implementation/phase-1-design|Phase 1 Design]]
- follows: [[docs/implementation/architecture|Architecture]]
- follows accepted: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- uses: [[docs/world/space|Space]]
- uses: [[docs/world/energy|Energy Buffer]]
- uses: [[docs/biology/cell|Cell]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
- constrained by: [[docs/implementation/optimization-paths|Optimization Paths]]
- prepares: [[docs/implementation/implementation-phases|Implementation Phases]]

# Related Documents

- `docs/implementation/phase-1-design.md`
- `docs/implementation/architecture.md`
- `docs/implementation/implementation-phases.md`
- `docs/engine/technology-stack.md`
- `docs/engine/scheduler.md`
- `docs/engine/storage.md`
- `docs/implementation/optimization-paths.md`
- `docs/config/stability_bounds.md`
