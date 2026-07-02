---
tags:
  - alife
  - implementation
  - performance
  - architecture
---

# Optimization Paths

> Потенційні шляхи оптимізації, які треба зберегти відкритими при розробці Phase 1+.

---

# Purpose

This document is an implementation guardrail.

It does not require implementing every optimization immediately. It defines design paths that must remain possible so early code does not block real-time scale later.

Target pressure:

```text
20k+ Cells
20k-40k Joints
4-8 Resource types
3-5 Field layers
30+ rendered ticks/sec
100+ headless ticks/sec
deterministic replay mode
```

---

# Core Rule

Do not choose APIs or storage layouts that force:

```text
CellId lookup inside hot loops
per-operation Result arithmetic in scalar kernels
global synchronized delta/event push buffers
success event spam
full-world JSON snapshots
nested grid allocation
Cell-owned object graphs
Arc/Mutex/Rc/RefCell in simulation hot state
unordered iteration as behavior input
```

If a simple Phase 1 shortcut is used, the document or code comment must state the future replacement boundary.

---

# Cell Hot Loop

Required path:

```text
CellStore dense indices
linear iteration over hot columns
double-buffered hot values where useful
hot/cold data split
preallocated transient buffers
```

Allowed later optimizations:

```text
compact flags
u8/u16 scaled values for bounded states
fixed-point accounting for conserved values
hybrid AoS for fields always read together
SIMD over flat arrays after profiling
```

Do not expose a public API that makes systems ask for `CellId -> CellIndex` during every Cell operation.

---

# Amount Math

Required path:

```text
checked constructors at boundaries
saturating or bounded math in hot loops
commit-time clamp / validation
tests for boundary values
```

`Result` per addition/subtraction is a cold-path API only. It is acceptable for config loading, tests and deserialization, not for per-cell scalar kernels.

Fixed-point remains an available future path. Do not leak raw `f32` accounting primitives into public core APIs.

---

# Deltas And Events

Required path:

```text
double-buffer routine scalar updates
DeltaBuffer for semantic ordered changes
preallocated memory
per-partition buffers for future parallelism
stable deterministic merge order
```

Event log policy:

```text
emit rare, meaningful or anomalous events
store routine success as flags/counters
sample high-volume debug traces explicitly
```

Example:

```text
MandatoryCostFailed -> event
MandatoryCostPaid -> RuntimeFlags.mandatory_paid + aggregate counter
```

---

# Resource, Field And Trace Grids

Phase 1 may use a flat dense grid.

Required future paths:

```text
grid resolution independent from world-space units
ping-pong buffers for stencil updates
scheduled diffusion cadence
dirty-region processing
sparse or chunked active regions
cache-friendly tiling
deterministic parallel chunks
```

Avoid:

```text
Vec<Vec<T>> grids
full dense diffusion for huge mostly empty worlds
library-driven diffusion without benchmark evidence
GPU requirement before CPU deterministic baseline
```

---

# Spatial Index

Phase 1 target path:

```text
counting-sort / prefix-sum flat grid
counts
offsets
flat sorted dense CellIndex array
stable neighbor iteration
```

Large-world path:

```text
chunked spatial grid
fixed-size flat spatial hash
active-region rebuild
stable bucket order
```

The spatial index is derived state. It must not become Cell existence authority.

---

# Parallelism

Required path:

```text
domain decomposition
thread-local scratch/delta/event buffers
deterministic partition order
halo zones for cross-partition interaction
stable reductions
```

Avoid:

```text
shared mutable global buffers in hot phases
lock contention as normal operation
nondeterministic Rayon reductions in deterministic mode
```

If parallel mode and deterministic mode diverge later, the mode boundary must be explicit.

---

# Viewer And Storage

Required path:

```text
viewer reads projections only
viewport/frustum filtering
LOD density maps when zoomed out
resource/field tile summaries
binary snapshots for replay
bounded JSON/CSV only for debug exports
```

Do not make full-world JSON frames part of the normal viewer or replay contract.

---

# Genome Runtime

Phase 3 must preserve scheduled Genome Runtime:

```text
next_genome_tick per Cell
due-cell collection
batched deterministic inference
stable output commit
```

Genome Runtime must not become an every-Cell every-Tick obligation unless profiling and requirements prove it is acceptable.

---

# Joints

Phase 4 must use a separate `JointStore`:

```text
endpoint_a
endpoint_b
rest_length
integrity
channel_flags
```

Endpoint ids should be resolved to dense indices before hot solving.

Potential conflict strategies:

```text
graph coloring
independent joint batches
deterministic accumulate-and-commit corrections
domain decomposition with halo zones
```

If none of these can support target scale, Joint physics requires deeper research before implementation.

---

# Phase Gates

Phase 1 must implement:

```text
dense CellIndex iteration
typed wrappers
double-buffer-compatible CellStore
flat ResourceGrid boundary
prefix-sum SpatialIndex boundary
no routine success event spam
snapshot/viewer as read-only projection
```

Phase 2 must add:

```text
compact process/feasibility buffers
no per-action heap object requirement
deterministic conflict and rejection summaries
```

Phase 3 must add:

```text
scheduled Genome Runtime
batched due-cell execution
deterministic Genome output commit
```

Phase 4 must add:

```text
SoA JointStore
explicit endpoint resolution
explicit joint conflict strategy
```

Scale-up phase must add only after measurement:

```text
domain decomposition
sparse or chunked fields
viewer LOD pipeline
SIMD / GPU acceleration
compact numeric representation
```

---

# Semantic Links

- constrains: [[docs/implementation/phase-1-data-model|Phase 1 Data Model]]
- constrains: [[docs/implementation/phase-1-module-api|Phase 1 Module API]]
- follows: [[docs/engine/performance|Performance]]
- follows: [[docs/engine/storage|Storage]]
- follows: [[docs/implementation/architecture|Architecture]]
- prepares: [[docs/implementation/implementation-phases|Implementation Phases]]
- constrained by: [[docs/engine/scheduler|Scheduler]]
