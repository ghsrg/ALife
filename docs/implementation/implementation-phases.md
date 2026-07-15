---
tags:
  - alife
  - implementation
  - roadmap
---

# Implementation Phases

> High-level roadmap for building and testing the first ALife Engine implementation.

---

# Purpose

This document defines the implementation phases after accepting the technology stack.

It is intentionally not a detailed task plan. Before starting each phase, create a separate detailed phase plan with concrete modules, tests, commands and acceptance gates.

---

# Implementation Principles

- Headless simulation is the source of truth.
- Core architecture is data-oriented and deterministic; outer adapters may use clean / hexagonal boundaries.
- Viewer is read-only.
- Deterministic replay is required from the first runnable phase.
- Performance tests are part of every phase.
- Canon and ADR documents remain above implementation plans.
- Storage, viewer and analytics must not enter the simulation hot path.
- Stability/calibration tools start early and grow with the engine.

---

# Phase 0: Technical Foundation

Goal: create the project skeleton and deterministic execution foundation.

Build:

```text
Rust workspace
core crate
headless CLI
TOML config loading
deterministic seed/RNG contract
basic fixed-point accounting types
snapshot/event-log skeleton
performance benchmark harness
```

Gate:

```text
same seed + same config + same binary -> same result
headless smoke run exists
no viewer dependency in core
```

---

# Phase 1: Deterministic World Smoke

Goal: run a minimal deterministic world with one Cell and no division.

Required pre-implementation artifact:

```text
docs/implementation/phase-1-design.md
```

Build:

```text
2D world space
uniform spatial grid
Cell entity state
Resource grid / sparse grid baseline
Energy Buffer accounting
mandatory costs
simple lifecycle state
local WebGL2 viewer skeleton
binary frame stream
```

Early side-track after Phase 1 design:

```text
docs/implementation/early-stability-tool.md
future tool path: tools/early-stability/
```

The Early Stability Tool starts as a static calculator and later grows into a small micro headless simulator. It is not part of the simulation hot path.

Gate:

```text
single Cell can exist or die deterministically
viewer renders committed snapshots only
headless run and rendered run produce same simulation state
```

---

# Phase 2: Processes, Feasibility And Lifecycle

Goal: make Cells act through registered processes and explicit Feasibility Check.

Build:

```text
Action / Process Registry implementation
Feasibility Check
resource uptake/export
energy conversion
material synthesis / repair baseline
division preparation
division partition
death and decomposition
basic stability calculator scenarios
local cell contact primitives
typed chemistry and matter dynamics
persistent material-backed Joints
shared Observer metrics/projections consumed by Core tools and sweep_analyzer
```

Runner side-track during Phase 2:

```text
docs/implementation/implementation-plan-runner.md
```

Runner enables live UI connection and is required before UI-1B can begin. It runs in parallel with Phase 2 mechanics work and must be complete before Phase 3 starts. The side-track covers: HTTP command API, WebSocket frame stream, scenario discovery, run state machine, internal committed-snapshot cache policy, and remote viewer mode. Server-side frame history and seek remain out of scope; UI scroll-back is client-side.

Gate:

```text
Cell can live, grow, divide and die
rejected actions have explicit reasons
matter and Energy accounting pass validation
single_cell_survival and single_cell_division configs exist
Reaction/MaterialFragment/Joint gates pass before Genome Runtime starts
Runner-1 and Runner-2 complete before UI-1B starts
Runner-3 complete before UI-1C starts
```

---

# Phase 3: Genome Runtime And Inheritance

Goal: introduce scheduled Genome Runtime and heritable variation over registered material, reaction and Joint mechanics.

Build:

```text
Genome as direct regulatory graph
scheduled Genome Runtime cadence
ActionPlan generation
runtime_state integration
physical genome carrier
genome copying
mutation during copying/repair
lineage event log
regulation of registered controlled reactions, repair and Joint intents
```

Gate:

```text
Genome affects priorities, not direct actions
Genome does not create ResourceTypes, MaterialTypes, reactions or Joint channels
same seed reproduces mutation and inheritance events
Genome Runtime does not need to run every Tick
lineage replay can reconstruct births/divisions/deaths
```

---

# Phase 4: Shared Observer And Multicellular Stability Experiments

Goal: complete the shared Observer layer over Core and `sweep_analyzer`, then observe and calibrate multicellular structures made possible by Phase 2H and regulated in Phase 3, without organism-level control.

Build:

```text
one read-only Observer projection/metrics boundary for Core, viewer, storage, reports and sweep_analyzer
reuse and normalization of Phase 2 mechanism-local summaries
cross-mechanism coverage and balance projections
Joint graph/channel/degradation observer metrics
scalar signal trace analysis
OrganismView as observer-only connected component
multicellular stability scenarios
chemistry, fragment and local Heat interaction scenarios
```

Gate:

```text
existing Joints can be measured as persistent or collapsing
signals preserve their Phase 2H delayed-Tick visibility rules
Core and sweep_analyzer use the same Observer metric definitions and provenance
no analyzer-specific duplicate observer model exists
OrganismView does not affect behavior
multicellular structures can persist or collapse deterministically
```

---

# Phase 5: Long-Run Evolution Experiments

Goal: use the shared Observer outputs to study long-run evolution after the core mechanisms, Genome Runtime and baseline multicellular analysis are available.

Build:

```text
population metrics
selection/drift observer logs
adaptation observer metrics
species-like cluster observer labels
Parquet analytics exports
SQLite run metadata/index
Python calibration scripts
long-run mutation/selection experiments
evolutionary stability scenario suite
```

Gate:

```text
headless evolution runs are replayable
population metrics do not affect behavior
stability scenarios classify stable / fragile / collapse / invalid
analysis can compare config changes across runs
long-run evolutionary changes can be separated from observer-only drift candidates
```

---

# Phase 6: Performance Scale-Up

Goal: move from correctness-first implementation toward target scale.

Build:

```text
SoA hot path optimization
deterministic parallel partitions
dirty regions
bounded trace sampling
scheduled diffusion/Field updates
viewer binary frame optimization
benchmark reports
```

Gate:

```text
20k Cells target scenario is measured
20k-40k Joints scenario is measured
30+ ticks/sec with rendering target is evaluated
100+ ticks/sec headless target is evaluated
determinism checks still pass
```

---

# Phase 7: Advanced Evolution Capability

Goal: evaluate whether stable specialized multicellular life can emerge beyond survival-by-accident.

Build:

```text
specialization observer profiles
signal-plastic material experiments
multicellular reproduction-like events
long-run mutation/selection experiments
config search for stability bounds
research reports for emergent intelligence-like behavior prerequisites
```

Gate:

```text
stable multicellular structures are measurable
specialization persists over configured windows
evolutionary improvements can be distinguished from drift candidates
research report identifies blockers and next mechanisms
```

---

# Cross-Phase Requirements

Each phase must include:

```text
deterministic replay test
accounting/conservation test where applicable
performance smoke benchmark
config validation
docs update
phase report
```

Detailed phase plans should be created only when the phase starts.

---

# Semantic Links

- implements: [[docs/engine/technology-stack|Technology Stack]]
- follows accepted: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- follows architecture: [[docs/implementation/architecture|Architecture]]
- defines Phase 1: [[docs/implementation/phase-1-design|Phase 1 Design]]
- starts tool side-track: [[docs/implementation/early-stability-tool|Early Stability Tool]]
- starts runner side-track: [[docs/implementation/implementation-plan-runner|Runner Implementation Plan]]
- starts ui side-track: [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
- starts from: [[docs/world/space|Space]]
- starts from: [[docs/biology/cell|Cell]]
- uses: [[docs/biology/feasibility|Feasibility Check]]
- uses: [[docs/biology/action-process-registry|Action / Process Registry]]
- later introduces: [[docs/biology/joint|Joint]]
- later analyzes: [[docs/evolution/population-dynamics|Population Dynamics]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]

# Related Documents

- `docs/engine/technology-stack.md`
- `docs/engine/performance.md`
- `docs/engine/scheduler.md`
- `docs/config/stability_bounds.md`
- `docs/decisions/ADR-0001-tech-stack.md`
- `docs/implementation/implementation-plan-runner.md`
- `docs/implementation/implementation-plan-ui.md`
