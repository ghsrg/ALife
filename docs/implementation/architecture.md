---
tags:
  - alife
  - implementation
  - architecture
---

# Architecture

> Implementation architecture for the first ALife Engine runtime.

---

# Purpose

This document fixes the high-level implementation architecture after accepting the technology stack.

It does not define new world laws. Canon documents and ADRs remain above implementation architecture.

---

# Decision

Use a hybrid architecture:

```text
Simulation Core:
  data-oriented deterministic architecture
  custom SoA/ECS storage
  stable ids and dense indices
  explicit snapshots, deltas and commits
  deterministic scheduler phases

Application Shell:
  clean / hexagonal boundaries
  explicit adapters for config, storage, viewer and analysis
  no external adapter in the simulation hot path
```

Classical Clean Architecture is not the main architecture for the simulation core.

It may be used at the outer layer, where boundaries are more important than hot-loop memory layout.

---

# Core Rule

The core simulation is a deterministic data pipeline.

Core systems read stable state, produce deltas, pass validation, and commit changes in deterministic order.

```text
Tick Snapshot
  -> scheduled Systems
  -> Deltas
  -> Feasibility / validation
  -> deterministic Commit
  -> next Tick Snapshot
```

Core behavior must not depend on:

- UI state;
- database writes;
- analytics exports;
- unordered map iteration;
- nondeterministic parallel reduction;
- object construction side effects;
- runtime reflection or dynamic dispatch in hot loops.

---

# Layer Shape

Recommended first workspace shape:

```text
alife-core
  domain ids / numeric wrappers / contracts
  world state SoA
  scheduler
  systems
  deltas
  deterministic_rng
  validation

alife-runner
  CLI
  config loading
  scenario loading
  run loop
  benchmark harness

alife-storage
  binary snapshots
  binary event logs
  SQLite metadata
  Parquet export

alife-viewer-server
  committed frame extraction
  binary frame encoding
  WebSocket server

web-viewer
  WebGL2 renderer
  colors and visual layers
  debug overlays

analysis
  Python scripts
  DuckDB / Polars / Pandas tools
  calibration and research reports
```

Exact crate names may change during Phase 0, but the boundary must remain.

---

# Core Modules

The core should be organized around data and systems, not around deep object hierarchies.

Typical hot-path modules:

```text
world_state
spatial_index
resource_grid
field_layers
cell_store
joint_store
genome_runtime
process_registry
feasibility
process_execution
lifecycle
physics
events
deltas
commit
```

Domain terms such as `Cell`, `Joint`, `Genome`, `Material` and `Resource` may have typed ids and small contract structs.

They should not imply object-heavy per-entity classes with virtual behavior in the hot path.

---

# Application Boundaries

Outer layers may follow clean / hexagonal architecture:

```text
Config Adapter
  TOML -> validated runtime config

Storage Adapter
  committed snapshots/events -> binary files, SQLite metadata, Parquet exports

Viewer Adapter
  committed snapshot projection -> binary WebSocket frame

Analysis Adapter
  run outputs -> Python / DuckDB / Polars workflows
```

These adapters depend on core contracts.

The core does not depend on adapters.

---

# Data Flow

```text
TOML config
  -> validated config
  -> headless runner
  -> alife-core Tick loop
  -> committed snapshots and events
  -> storage / viewer / analysis projections
```

Viewer, storage and analysis consume committed outputs.

They cannot feed behavior back into the same run unless a future explicit control protocol is designed and accepted.

---

# File structure

Starting structure:
- src/lib.rs contains reusable simulation logic.
- src/main.rs is only the executable entry point.
- Domain modules are separated into world, physics, cell, organism, simulation, renderer.
- Do not place domain logic directly in main.rs.
- New code should be added to the appropriate module and exposed through lib.rs only when needed.

---

# Testing Strategy

Each phase should keep architecture testable through:

- deterministic replay tests;
- accounting tests for Energy, Resources, Materials and capacity;
- scheduler cadence tests;
- config validation tests;
- serialization round-trip tests;
- viewer-frame contract tests;
- performance smoke benchmarks.

Tests should prefer core-level deterministic fixtures over UI-driven checks.

---

# Invariants

```text
Core architecture is data-oriented deterministic simulation.
Application shell uses clean / hexagonal boundaries.
Headless simulation is the source of truth.
Viewer, storage and analysis are projections, not authorities.
No external adapter enters the hot path.
No behavior-critical accounting uses untyped f32.
No nondeterministic iteration or merge order is allowed in behavior.
```

---

# Semantic Links

- implements: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- follows: [[docs/engine/technology-stack|Technology Stack]]
- constrains implementation of: [[docs/engine/ecs|ECS]]
- constrains implementation of: [[docs/engine/scheduler|Scheduler]]
- preserves: [[docs/world/tick-semantics|Tick Semantics]]
- supports: [[docs/implementation/implementation-phases|Implementation Phases]]

# Related Documents

- `docs/implementation/implementation-phases.md`
- `docs/engine/technology-stack.md`
- `docs/engine/ecs.md`
- `docs/engine/scheduler.md`
- `docs/engine/storage.md`
- `docs/engine/performance.md`
- `docs/decisions/ADR-0001-tech-stack.md`
