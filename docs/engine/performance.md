---
tags:
  - alife
  - engine
  - area/engine
---

# performance.md

> Performance — обмеження й оптимізації без зміни семантики симуляції.

---

# Призначення

Performance описує, як зберігати стабільність виконання при зростанні кількості cells, joints, resources і traces.

---

# Канонічні правила

- Determinism важливіший за швидкість.
- Optimization не змінює Canon behavior.
- Spatial indexing потрібен для locality.
- Trace/debug мають бути sampled або configurable.
- Scheduler budgets не повинні мовчки пропускати mandatory semantics.
- Technology stack and practical performance choices are defined in `engine/technology-stack.md`.
- Headless simulation is the source of truth; rendering is a read-only projection.

---

# Target Scale

Initial architecture target:

```text
world: 512 x 512 su
cells: 20k target
joints: 20k-40k
resources: 4-8 types
fields: 3-5 layers
signals/traces: 1 scalar trace layer or sparse trace
rendered speed: 30+ ticks/sec
headless speed: 100+ ticks/sec
```

---

# Мінімальні Напрями

```text
spatial index
dirty regions
bounded traces
configurable debug sampling
deterministic parallel reductions
resource field chunking
profile-guided budgets
scheduled Genome Runtime cadence
scheduled diffusion / trace cadence
SoA memory layout
fixed-point accounting for conserved amounts
```

Expected hot spots:

```text
Cell hot loop
Genome Runtime
Feasibility and conflict resolution
Resource / Field / Trace diffusion
Spatial locality queries
Joint graph updates
Observer/debug trace output
```

---

# Required Hot-Path Algorithms

These are implementation constraints for real-time targets. They do not change simulation semantics.

## Cell Hot Loop

Use SoA or hybrid SoA storage for Cell hot fields.

Routine per-Tick scalar updates should use double-buffered arrays when this is cheaper than writing per-cell deltas:

```text
read columns -> write columns -> deterministic swap/commit
```

Use `DeltaBuffer` only for semantic changes that require ordering, validation, event emission or deterministic merge from parallel partitions.

Transient buffers must be preallocated and reused:

```text
Vec::with_capacity(...)
clear without dropping capacity
per-partition buffers for future deterministic parallelism
stable merge order
```

## Spatial Index

Spatial index rebuild must use a flat deterministic counting-sort / prefix-sum grid:

```text
cell positions
  -> grid cell ids
  -> counts per grid cell
  -> prefix sums / offsets
  -> flat sorted cell id array
  -> neighbor range scans
```

Do not use hot-path `HashMap<GridCoord, Vec<CellId>>` or freshly allocated `Vec<Vec<CellId>>` for every Tick.

For large mostly empty worlds, prefix-sum over every possible grid cell becomes too expensive. Keep a migration path to:

```text
chunked spatial grids
fixed-size flat spatial hashing
active-region rebuilds
stable bucket iteration order
```

These options must remain deterministic in deterministic mode.

## Resource / Field / Trace Layers

Grid-like layers must use flat arrays and ping-pong buffers:

```text
current: Vec<T>
next: Vec<T>
scratch if needed
```

Diffusion or trace propagation should be implemented as deterministic stencil operations over flat arrays.

Optimization order:

```text
flat arrays
cadence scheduling
dirty regions / sparse regions where valid
cache-friendly tiling
deterministic parallel chunks
SIMD after profiling
GPU only after CPU baseline and determinism constraints are understood
```

Do not assume ndarray/Rayon/SIMD/GPU performance without release-mode measurement on target hardware.

For large worlds, resource and field resolution may be lower than world-space resolution:

```text
1 resource grid cell = multiple spatial units
inactive chunks are skipped
dirty regions update before full-grid diffusion
LOD summaries feed viewer output
```

Full dense diffusion over the whole world is not a default beyond bounded Phase 1 scale.

## Type Wrappers

Typed wrappers are required at core boundaries for behavior-critical accounting.

Hot-loop arithmetic may use internal unchecked operations only under documented invariants:

```text
validated input
unchecked internal math in narrow kernels
commit-time clamp/validation
tests for boundary cases
```

Do not use `unsafe` for convenience. Require profiling evidence before introducing unsafe or hand-written SIMD.

## Scheduled Genome Runtime

Genome Runtime must be scheduled by cadence rather than assumed every Tick:

```text
next_genome_tick per Cell
due-cell collection
batched deterministic inference
stable output commit
```

This is a Phase 3 requirement and should be planned before Genome Runtime implementation.

## Joints

Joints are expected to be a random-access hot path.

Phase 4 must use a separate `JointStore` with SoA endpoint arrays and an explicit conflict strategy:

```text
graph coloring
independent joint batches
or deterministic accumulate-and-commit corrections
```

If the conflict strategy cannot meet target scale, Joint physics requires deeper research before implementation.

## Parallelism

Parallelism must use deterministic partitioning:

```text
domain chunks
thread-local buffers
stable partition merge order
halo zones for cross-boundary interactions
```

Do not add shared locks, global synchronized push buffers or nondeterministic reductions to the simulation hot path.

## Viewer And Storage Output

Viewer and storage output must stay outside the behavior hot path.

Scaling requirements:

```text
viewport/frustum culling for visible Cells
LOD density maps when zoomed out
binary or compact frame formats for real-time output
delta-compressed snapshots where useful
JSON only for small debug exports
```

The engine must not require full-world JSON snapshots for normal visualization or replay.

## Memory Bound Guardrails

Hot data must be designed for cache pressure:

```text
hot/cold split
compact flags
bounded integer or fixed-point representations where useful
hybrid AoS only for fields always read together
measured hot bytes per Cell
```

For machines with small L3 cache, this is a design constraint, not a late cleanup.

---

# Заборонено

Не вводити:

- nondeterministic updates by default;
- silent action drops due to budget;
- observer metrics in hot path as behavior inputs;
- optimization-only behavior differences;
- untyped float accounting for Energy, Resources, Materials, costs or capacity.
- CellId lookup inside hot loops;
- success-event spam for routine per-Cell accounting;
- full-world JSON frames as a normal real-time path.

---

# Semantic Links

- optimizes execution of: [[docs/engine/scheduler|Scheduler]]
- constrained by: [[docs/engine/technology-stack|Technology Stack]]
- optimizes fields from: [[docs/world/fields|Fields]]
- optimizes many: [[docs/biology/cell|Cells]]
- must preserve: [[docs/world/tick-semantics|Tick Semantics]]

# Пов'язані документи

- `engine/scheduler.md`
- `engine/technology-stack.md`
- `world/tick-semantics.md`
- `docs/config/stability_bounds.md`
- `docs/implementation/optimization-paths.md`
