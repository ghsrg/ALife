---
tags:
  - alife
  - engine
  - area/engine
  - canon
---

# technology-stack.md

> Technology Stack — практичний стек реалізації рушія, viewer, storage і analysis.

---

# Призначення

Цей документ фіксує прийнятий технологічний стек для першої реалізації.

Причини рішення описані в `docs/decisions/ADR-0001-tech-stack.md`.

---

# Stack Summary

```text
Core Simulation:
  Rust
  headless source of truth
  custom data-oriented SoA/ECS
  deterministic scheduler

Viewer:
  local Web Viewer
  WebSocket binary frames
  WebGL2 instanced rendering
  Canvas overlays optional
  read-only

Configs:
  TOML

Storage:
  versioned binary snapshots
  binary event logs
  SQLite metadata/index
  Parquet analytics exports

Analysis:
  Python
  DuckDB / Polars / Pandas
  graph analysis only for selected or offline datasets
```

---

# Performance Target

Initial architecture target:

```text
world: 512 x 512 su
cells: 20k target
cells stretch: 20k
joints: 20k-40k
resources: 4-8 types
fields: 3-5 layers
signals/traces: 1 scalar trace layer or sparse trace
rendered speed: 30+ ticks/sec
headless speed: 100+ ticks/sec
```

---

# Core Simulation

The core simulation is the source of truth.

Rules:

- run headless by default;
- use deterministic Tick semantics;
- use explicit deltas and commit boundaries;
- keep UI, storage and analytics outside behavior;
- keep hot path memory-oriented and allocation-aware;
- avoid object-heavy OOP in core state;
- prefer stable ids and dense indices.

Core state should use custom SoA/Data-Oriented storage rather than a heavy general-purpose ECS framework.

Example shape:

```text
cell_positions[]
cell_radius[]
cell_energy[]
cell_resources[]
cell_materials[]
cell_runtime_state[]

joints_from[]
joints_to[]
joints_state[]

resource_grid[]
field_layers[]
trace_layers[]
```

---

# Scheduler Cadence

Not every system must run every Tick.

Scheduled systems:

```text
Genome Runtime:
  configurable cadence
  staggered deterministic partition

Trace diffusion:
  configurable cadence
  sparse/dirty update where possible

Field updates:
  configurable cadence
  full-grid only when required

Observer metrics:
  sampled or interval-based
```

Fast Tick systems may include:

```text
mandatory costs
passive local accounting
lifecycle checks
physics/contact basics
process progress accounting
resource uptake/export execution
```

Scheduler optimizations must preserve `world/tick-semantics.md`.

---

# Numeric Model

Use a hybrid numeric model.

Behavior-critical accounting:

```text
EnergyAmount      -> fixed-point integer wrapper
ResourceAmount    -> fixed-point integer wrapper
MaterialAmount    -> fixed-point integer wrapper
ProcessCost       -> fixed-point integer wrapper
Capacity          -> fixed-point integer wrapper
```

Continuous/visual values:

```text
Position
Velocity
Radius
FieldValue
Temperature
```

Continuous values may start as `f32` if deterministic update order is preserved.

Rule:

```text
No untyped f32 accounting in Energy, Resources, Materials, costs or capacity.
```

---

# Determinism

Minimum replay contract:

```text
same config
same initial state
same seed
same engine version
same binary/runtime
same deterministic mode
same platform mode
  -> same committed states, events and observer metrics
```

Implementation rules:

- no unordered hash map iteration in behavior;
- no nondeterministic reductions;
- no global/thread-local RNG in behavior systems;
- stable ids and explicit RNG streams;
- parallel systems must use deterministic partitioning and deterministic merge order.

---

# Viewer

The first viewer is a local Web Viewer.

```text
Rust core
  -> WebSocket binary frame stream
  -> WebGL2 instanced rendering
  -> optional Canvas overlays
```

Viewer requirements:

- read-only;
- no simulation authority;
- no behavior changes from UI state;
- configurable colors and visual layers;
- able to render all Cells, Resources and basic Joints;
- UHD Graphics is used for rendering, not simulation compute.

GPU compute is future work.

---

# Storage

Storage is not in the simulation hot path.

Hot path:

```text
memory-only SoA/ECS state
deterministic event buffers
periodic binary snapshot buffers
```

Replay:

```text
versioned binary snapshots
binary event logs
```

Metadata/index:

```text
SQLite
```

Analytics:

```text
Parquet
DuckDB
Python analysis
```

Debug:

```text
small sampled JSON/CSV exports only
```

The simulation must not block on DB, Parquet or viewer output by default.

---

# 2D And Future 3D

Base runtime target:

```text
2D
```

Architecture requirement:

```text
2D-first, 3D-compatible
```

3D is future work. It should not shape the first runtime implementation enough to slow down 2D validation.

---

# Заборонено

Не вводити:

- DB writes in the hot path;
- renderer as simulation authority;
- game engine as source of truth;
- Python behavior in the core Tick loop;
- untyped float accounting for conserved amounts;
- nondeterministic parallel reductions;
- CUDA/GPU compute as a base requirement;
- 3D as a first implementation dependency.

---

# Semantic Links

- decided by: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- implements constraints from: [[docs/engine/performance|Performance]]
- constrains: [[docs/engine/scheduler|Scheduler]]
- constrains: [[docs/engine/storage|Storage]]
- preserves: [[docs/world/tick-semantics|Tick Semantics]]
- configured through: [[docs/config/stability_bounds|Stability Bounds]]

# Пов'язані документи

- `engine/performance.md`
- `engine/scheduler.md`
- `engine/storage.md`
- `engine/ecs.md`
- `engine/rendering.md`
- `world/tick-semantics.md`
- `world/space.md`
- `docs/decisions/ADR-0001-tech-stack.md`
