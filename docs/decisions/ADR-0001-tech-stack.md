---
tags:
  - alife
  - adr
  - area/decisions
---

# ADR-0001: Technology Stack

> Also referenced as `ADR-001-tech-stack`.

---

# Status

Accepted.

---

# Context

The project needs a deterministic Artificial Life simulation with:

- headless simulation as the source of truth;
- local read-only visualization;
- 512 x 512 su world target;
- 20k Cells target and stretch;
- 20k-40k Joints;
- 4-8 Resource types;
- 3-5 Field layers;
- 30+ ticks/sec with rendering;
- 100+ ticks/sec headless;
- deterministic replay within the same engine version, binary and platform mode;
- CPU-only simulation for the first implementation;
- UHD Graphics used for rendering only.

The documentation requires deterministic Tick semantics, explicit deltas/commits, observer-only metrics, bounded traces and no hidden behavior through UI, storage or analytics.

---

# Decision

Use a two-layer stack:

```text
Simulation Core:
  Rust
  custom data-oriented SoA/ECS storage
  deterministic scheduler
  explicit snapshots/deltas/commits
  CPU-only simulation first

Viewer:
  local Web Viewer
  WebSocket binary frames
  WebGL2 instanced rendering
  Canvas overlays optional
  read-only projection of committed snapshots

Control / Analysis:
  Python
  TOML config generation
  Parquet/DuckDB/Polars/Pandas analysis
  selected graph analysis only
```

The simulation core is headless. Rendering, UI, analytics and storage do not own simulation truth and must not affect behavior.

---

# Numeric Model

Use a practical hybrid numeric model:

```text
Accounting-critical state:
  fixed-point integer / typed amount wrappers

Continuous geometry and visualization:
  f32 with deterministic update order
```

Initial accounting types:

```text
EnergyAmount
ResourceAmount
MaterialAmount
ProcessCost
Capacity
```

These must not be represented as untyped `f32` in behavior-critical accounting.

Initial continuous types:

```text
Position
Velocity
Radius
FieldValue
Temperature
```

`f32` is acceptable for geometry, rendering, soft Fields and approximate physical values when deterministic order is preserved. Any `f32` value that can cross a behavior threshold must be reviewed carefully.

---

# Scheduler Cadence

Genome Runtime and expensive environment systems are scheduled systems, not necessarily every-Tick systems.

```text
Genome Runtime:
  configurable cadence
  staggered by stable Cell id or deterministic partition

Traces / diffusion / expensive Field updates:
  configurable cadence
  dirty-region or sparse update where possible
```

Between Genome Runtime evaluations, a Cell uses committed `runtime_state`, `action_plan`, process progress and lifecycle state. Mandatory costs, passive processes, lifecycle checks, resource accounting and physics may run more frequently according to scheduler rules.

Scheduler cadence is an engine rule and must preserve `world/tick-semantics.md`.

---

# Determinism Contract

Minimum deterministic replay contract:

```text
same config
same initial state
same seed
same engine version
same binary/runtime
same deterministic mode
same platform mode
  -> same committed states, events, births, deaths, mutations,
     divisions and observer metrics
```

Cross-CPU, cross-compiler, cross-version and different parallel execution modes are not guaranteed to match unless explicitly validated later.

Parallelism is allowed only through deterministic partitioning and deterministic reductions. Rayon or any parallel executor must not be used in a way that changes behavior through unordered reductions or unordered map iteration.

---

# Storage Boundary

Storage is outside the hot path.

```text
Hot path:
  memory-only SoA/ECS state
  deterministic event buffers
  periodic binary snapshots

Replay:
  versioned binary snapshots
  binary event logs

Experiment index:
  SQLite for run metadata, config hashes, artifact index and summary metrics

Analytics:
  Parquet time-series / event tables
  DuckDB or Python analysis over exported data

Debug export:
  JSON/CSV only for small sampled traces
```

The simulation must not wait on SQLite, Parquet or viewer I/O by default.

---

# 2D / 3D Position

Base implementation is 2D.

```text
Base runtime target:
  2D

Architecture:
  2D-first, 3D-compatible

Future:
  3D mode only after 2D core semantics and performance are stable
```

The first implementation may use 2D position types directly, but APIs should avoid hardcoding assumptions that make future 3D impossible without rewriting core semantics.

---

# Alternatives Considered

## C++ Core

C++ gives maximum low-level control, mature SIMD support and direct access to assembly-level optimizations.

Rejected as the first choice because memory safety, undefined behavior risk and deterministic parallel maintenance cost are higher than Rust for this project.

## Python Core

Python is excellent for analysis and calibration.

Rejected for the simulation core because 20k Cells, 40k Joints, 100+ headless ticks/sec and deterministic hot loops require lower overhead and tighter memory control.

## Unity / Godot / Game Engine As Source Of Truth

Game engines are convenient for rendering and interaction.

Rejected as simulation source of truth because deterministic replay, headless experiments, strict data layout, storage boundaries and CI-style verification are more important than immediate UI convenience.

## Bevy / Godot Viewer

Viable future options.

Not selected for first viewer because a local WebGL2 viewer is simpler, read-only, easier to configure and keeps the simulation core independent from a game engine.

## GPU Compute First

Rejected for the first implementation. UHD Graphics can help with rendering, but simulation compute stays CPU-first to preserve determinism and reduce implementation complexity.

---

# Consequences

- Core implementation must be designed around SoA storage, stable ids, deterministic deltas and explicit commit boundaries.
- Viewer must consume committed snapshots/frame data and remain read-only.
- Configs should start as TOML.
- Rust/Python/Web boundaries must be explicit from the start.
- Numeric wrappers are required for accounting values to avoid spreading primitive numeric assumptions through the codebase.
- Performance work starts with data layout, scheduler cadence, dirty/sparse updates and deterministic reductions before low-level ASM/SIMD.
- A future 3D mode remains possible, but not a first implementation target.

---

# Related Documents

- `docs/engine/technology-stack.md`
- `docs/engine/performance.md`
- `docs/engine/scheduler.md`
- `docs/engine/storage.md`
- `docs/world/tick-semantics.md`
- `docs/world/space.md`
- `docs/config/stability_bounds.md`

# Semantic Links

- decides implementation of: [[docs/engine/technology-stack|Technology Stack]]
- constrains: [[docs/engine/performance|Performance]]
- constrains: [[docs/engine/scheduler|Scheduler]]
- constrains: [[docs/engine/storage|Storage]]
