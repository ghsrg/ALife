---
tags:
  - alife
  - implementation
  - phase/1
  - design
---

# Phase 1 Design

> Deterministic World Smoke design for the first runnable ALife Engine world.

---

# Purpose

Phase 1 proves that the engine can run a minimal deterministic world with one `Cell`, local Resources, Energy Buffer accounting, mandatory costs, simple lifecycle and committed snapshot output.

This document is an implementation design, not Canon. It translates accepted Canon, ADR and architecture rules into the first runnable implementation target.

---

# Goal

Build a deterministic smoke world:

```text
world + one Cell + Resources + Energy Buffer + mandatory costs
```

The Cell may survive, stall, become stressed, enter dormancy or die according to explicit accounting.

Phase 1 does not include division, Genome Runtime, mutations, Joints, multicellular structures or evolution analytics.

---

# Runtime Boundary

```text
alife-core
  source of truth for WorldState, Tick execution, deltas and commits

alife-runner
  loads config, creates initial state, runs headless scenario, writes outputs

alife-viewer-server
  reads committed snapshots or frames only

web-viewer
  visualizes committed frame data only

alife-storage
  receives snapshots/events after commit
```

Viewer, storage and analysis must not affect behavior.

`alife-core` must be runnable without viewer, storage DB, Python tools or Web UI.

---

# Phase 1 State Model

The first implementation should use data-oriented storage and stable ids. Exact Rust names may change during implementation, but the responsibilities should remain.

```text
WorldState
  tick
  world_config
  cell_store
  resource_grid
  field_layers
  spatial_index
  event_buffer

TickState
  read_snapshot
  delta_buffer
  commit_summary

CellStore
  cell_ids
  positions
  radii
  resources
  materials
  energy_buffers
  temperatures
  lifecycle_states
  runtime_flags
  debug_metrics

ResourceGrid
  resource_type_ids
  quantities_by_cell_or_grid
  diffusion_or_decay_placeholders

FieldLayers
  temperature_or_heat_support_placeholders
  simple ambient values if configured

SpatialIndex
  uniform grid index
  position-to-grid mapping
  neighbor lookup support

EventBuffer
  deterministic ordered events for replay/debug

DeltaBuffer
  proposed state changes before commit
```

Phase 1 should keep Joints, Genome Runtime, mutation, process registry execution and population analytics out of the active model.

---

# Cell State Subset

Phase 1 uses this subset of Canon `Cell` state:

```text
Cell
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

Required details:

```text
EnergyBuffer
  current
  capacity

LifecycleState
  alive
  stressed
  dormant
  dead

RuntimeFlags
  mandatory_paid
  stalled
  over_capacity
  inert

DebugMetrics
  age_ticks
  energy_balance_snapshot
  capacity_snapshot
  last_rejection_reasons
```

`debug_metrics` are observer-only and cannot affect behavior.

---

# Accounting Contract

Phase 1 must make Energy, capacity, Heat and waste accounting explicit enough for deterministic replay and early stability checks.

Minimum formulas:

```text
energy_after_mandatory =
  min(
    energy_capacity,
    energy_current + passive_energy_income - mandatory_cost_per_tick
  )

mandatory_paid =
  energy_current + passive_energy_income >= mandatory_cost_per_tick

used_capacity =
  resources_capacity_used
  + materials_capacity_used
  + genome_capacity_placeholder
  + internal_fragments_capacity_used

free_capacity =
  capacity_limit - used_capacity

heat_next =
  max(0, heat_current + heat_generated - heat_dissipation_rate)

waste_next =
  max(0, waste_current + waste_generated - waste_sink_rate)
```

Phase 1 may use `passive_energy_income = 0` unless a scenario explicitly configures a simple placeholder input.

Energy Buffer does not occupy capacity directly. Storage-capable Materials may define Energy capacity, but Phase 1 may start with configured `energy_capacity` plus validation that it is not silently unbounded.

---

# Lifecycle Contract

Phase 1 lifecycle is intentionally small:

```text
alive
stressed
dormant
dead
```

Starting transition rules:

```text
alive -> stressed
  if mandatory_paid = false
  OR energy_after_mandatory < stress_energy_threshold
  OR free_capacity < 0
  OR heat_next > heat_warning_threshold
  OR waste_next > waste_warning_threshold

stressed -> dormant
  if mandatory_paid = false
  AND dormancy_allowed = true
  AND dormant_mandatory_cost can be paid

stressed -> dead
  if energy_after_mandatory <= 0
  OR free_capacity < -critical_capacity_overrun
  OR heat_next > heat_death_threshold
  OR waste_next > waste_death_threshold
  OR minimum_viability_materials missing

dormant -> alive
  if mandatory cost can be paid again
  AND stress conditions are below configured recovery thresholds
```

If multiple transitions are possible in the same Tick, death wins over dormancy, and dormancy wins over remaining stressed.

---

# Tick Pipeline

Phase 1 Tick execution:

```text
1. Load validated config before run.
2. Build deterministic initial WorldState.
3. Create read-only Tick Snapshot.
4. Apply mandatory costs.
5. Apply passive Resource / Field placeholder updates.
6. Update simple lifecycle state.
7. Write proposed changes into DeltaBuffer.
8. Commit deltas in deterministic order.
9. Emit ordered events.
10. Produce committed snapshot/frame projection.
```

Mandatory costs are paid before any future planned action logic. If mandatory costs cannot be paid, the Cell does not perform planned actions and transitions according to lifecycle rules.

Phase 1 may use a fixed scripted scenario config for initial state, but the Tick loop itself must not contain hidden scenario-specific behavior.

---

# Minimal Config Surface

Phase 1 configs should provide the same field concepts that the Early Stability Tool will inspect.

```text
world:
  size
  boundary_mode
  tick_count
  seed

space:
  spatial_grid_size

resources:
  resource_type_ids
  initial_distribution
  optional_decay_rate
  passive_energy_income_placeholder

cell:
  initial_position
  radius
  initial_resources
  initial_materials
  initial_energy
  energy_capacity
  mandatory_cost_per_tick
  dormant_mandatory_cost_modifier
  capacity_limit
  minimum_viability_materials

environment:
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

lifecycle:
  stress_energy_threshold
  dormancy_allowed
  critical_capacity_overrun

output:
  event_log_enabled
  snapshot_interval
  viewer_frame_interval
```

Energy conversion may exist only as a placeholder accounting rule in Phase 1. Full registered process execution belongs to later phases.

---

# Event And Output Contract

Phase 1 events should be deterministic and ordered by Tick, event kind and stable entity id.

Minimum event kinds:

```text
run_started
tick_committed
mandatory_cost_paid
mandatory_cost_failed
lifecycle_changed
capacity_warning
heat_warning
waste_warning
cell_dead
snapshot_emitted
run_finished
```

Minimum run summary:

```text
scenario_id
config_hash
seed
tick_count
survival_result
collapse_reason
metrics_summary
```

`metrics_summary` is observer-only.

---

# Required Scenarios

Phase 1 should support at least:

```text
single_cell_survival
single_cell_starvation
single_cell_over_capacity
single_cell_heat_stress
deterministic_replay_smoke
viewer_snapshot_smoke
```

Each scenario must have deterministic seed and bounded tick count.

---

# Survival Results And Collapse Reasons

Use the same result vocabulary as Early Stability Tool:

```text
survival_result:
  stable
  fragile
  collapse
  invalid
```

Minimum `collapse_reason` values:

```text
none
invalid_config
energy_depleted
mandatory_cost_unpaid
capacity_exceeded
heat_limit_exceeded
waste_limit_exceeded
minimum_viability_materials_missing
determinism_mismatch
viewer_authority_violation
```

`stable` means the scenario reaches its configured `tick_count` without entering a collapse state.

`fragile` means the scenario reaches `tick_count`, but at least one configured warning threshold was crossed.

`collapse` means the scenario ends because the Cell dies or the world violates a configured stability boundary.

`invalid` means the scenario must not run because validation failed before Tick 0.

---

# Acceptance Gates

Phase 1 is acceptable only if:

```text
same config + same seed + same binary -> same committed states and events
one Cell can survive predictably
one Cell can die predictably
mandatory costs are visible in Energy accounting
capacity overrun is detected
survival_result and collapse_reason are emitted
viewer reads committed snapshots only
headless run does not depend on viewer/storage
local documentation links remain valid
```

---

# Out Of Scope

Phase 1 must not implement:

```text
division
Genome Runtime
mutation
inheritance
Joints
signal communication
full Action / Process Registry execution
evolution analytics
species-like clusters
organism-level behavior
GPU compute
DB writes in the hot path
```

---

# Early Stability Tool Hook

After this design is accepted, the early stability tool may use the same config concepts, `scenario_id` values, `survival_result` values and `collapse_reason` values to estimate whether baseline configs are viable before the full simulation exists.

The tool handoff is defined in [[docs/implementation/early-stability-tool|Early Stability Tool]].

---

# Semantic Links

- follows: [[docs/implementation/architecture|Architecture]]
- implements phase from: [[docs/implementation/implementation-phases|Implementation Phases]]
- follows accepted: [[docs/decisions/ADR-0001-tech-stack|ADR-0001 Technology Stack]]
- uses: [[docs/world/space|Space]]
- uses: [[docs/biology/cell|Cell]]
- uses: [[docs/world/energy|Energy Buffer]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
- prepares: [[docs/implementation/early-stability-tool|Early Stability Tool]]

# Related Documents

- `docs/implementation/architecture.md`
- `docs/implementation/implementation-phases.md`
- `docs/engine/technology-stack.md`
- `docs/engine/scheduler.md`
- `docs/engine/storage.md`
- `docs/config/stability_bounds.md`
