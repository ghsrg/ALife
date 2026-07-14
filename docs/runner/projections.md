---
tags: [alife, canon, area/runner, audience/agent]
---
# Runner Projections
> Versioned read-only data exposed by Runner to UI, storage, tests, and analysis.
## Purpose
Projections expose committed state without revealing mutable Core structures.
```text
Committed Core State -> Projection Builder -> Consumers
```

## Rules
- derive only from committed state;
- never mutate Core or Runner;
- use versioned public schemas;
- never expose internal `WorldState`;
- represent missing data explicitly;
- stay outside the simulation hot path.

## Projection Types

### RunStatusProjection
Contains process state, active-run state, run id, committed Tick, effective seed, Scenario hash, timing metadata, and terminal reason.

### WorldFrameProjection
Represents one committed spatial frame.

May contain:
- Cell position, radius, lifecycle, and visible state;
- Resource and Field layer values or sampled tiles;
- Material fragments and Joints when supported;
- frame Tick and schema version.

It contains simulation data, not colors, rendered heatmaps, labels, or interpolation.

### EventProjection
Contains ordered committed events.

Ordering must preserve authoritative commit order.

### SummaryProjection
Contains bounded aggregates such as:
- Cell counts by lifecycle state;
- Resource totals;
- global Heat and Waste;
- run progress and performance summaries.

Each aggregate declares its calculation scope and Tick.

### BootstrapManifestProjection
Contains:
- effective seed and Scenario hash;
- Bootstrap and generator versions;
- resolved entity counts;
- Resource totals and Field ranges;
- prepared-state hash;
- warnings.

## Delivery
Runner may expose projections through:
- in-process API;
- CLI output;
- HTTP;
- WebSocket or streaming transport;
- persisted files.

Transport encoding must not change semantics.

## Sampling And Resolution
Large grids may be tiled, sampled, compressed, or sent incrementally.

Reduced data declares:
- source Tick and layer;
- spatial bounds;
- resolution;
- aggregation method.

Reduced data never replaces authoritative Core state.

## Consistency
A projection represents one committed Tick unless mixed-time data is explicitly declared.

Cell, Resource, and Field coordinates in one frame use the same World coordinate system.

## Invariants
```text
Projections are read-only.
Only committed state is projected.
UI rendering is not projection semantics.
Projection schemas are versioned.
Transport does not change meaning.
WorldState is never exposed directly.
```

## Semantic Links
- [[docs/runner/runner|Runner]]
- [[docs/runner/run-lifecycle|Run Lifecycle]]
- [[docs/runner/command-contract|Command Contract]]
- [[docs/runner/bootstrap|Bootstrap]]
- [[docs/mechanics/observer-projection|Observer Projection]]
