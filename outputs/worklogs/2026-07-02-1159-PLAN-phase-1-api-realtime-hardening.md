# PLAN: Phase 1 API Realtime Hardening

Date: 2026-07-02 11:59

## Goal

Harden `docs/implementation/phase-1-module-api.md` and related implementation docs against realtime bottlenecks before Rust coding starts.

## Scope

This is a documentation/API contract plan. It does not change Rust code.

## P0. Hot-loop `CellId -> CellIndex` lookup risk

Problem:

`CellStore::index_of(&self, id: CellId) -> Option<CellIndex>` can be misused inside hot loops. If implemented as linear search it becomes O(N2). If implemented through `HashMap`, it creates hashing overhead and cache misses.

Impact:

Future interactions, Joints or neighbor systems may accidentally resolve ids repeatedly instead of iterating dense indices.

Proposal:

- Keep `index_of` only as boundary/debug/cold API.
- Add explicit rule: hot systems iterate dense `CellIndex` ranges.
- Add API shape:
  - `CellStore::indices(&self) -> CellIndexRange`
  - `CellStore::iter_indices(&self) -> impl Iterator<Item = CellIndex>`
  - `CellStore::resolve_id_cold(&self, id: CellId) -> Option<CellIndex>`
- Add rule: future Joints resolve endpoints to dense indices at phase start or maintain validated dense endpoint arrays, never resolve `CellId` per constraint iteration.

## P0. Per-operation `Result` math in hot loops

Problem:

`EnergyAmount::checked_add(self, rhs) -> Result<...>` as a prominent API can push implementation toward branching on every operation.

Impact:

Branching in hot arithmetic blocks vectorization and causes branch misprediction at scale.

Proposal:

- Keep checked constructors and checked boundary APIs.
- Move `checked_add` out of hot-path examples.
- Promote hot-path APIs:
  - `saturating_add`
  - `saturating_sub`
  - `clamp_max`
  - crate-private `add_unchecked_internal` only with documented invariants.
- Add explicit rule: validation and event generation happen at commit or boundary, not after every intermediate arithmetic operation.

## P0. DeltaBuffer as global bottleneck

Problem:

Global single `DeltaBuffer::push` and `sort_for_commit` can become a synchronization point when parallel systems arrive.

Impact:

Rayon/domain parallelism would require locks or contention if all threads push into the same buffer.

Proposal:

- Clarify Phase 1 single-thread starts with one buffer, but API must reserve future partitioned buffers.
- Add `DeltaBuffers`/`PartitionDeltaBuffers` concept:
  - one local buffer per deterministic partition/thread;
  - stable merge order;
  - no per-scalar deltas for hot columns.
- Keep double-buffered arrays as preferred path for energy, position, lifecycle flags and environment hot values.

## P0. Event spam from successful mandatory costs

Problem:

`MandatoryCostPaid` as a per-cell per-tick event would create 20k events per tick at target scale.

Impact:

EventBuffer and storage would become memory/I/O bottlenecks.

Proposal:

- Remove `MandatoryCostPaid` from default emitted event set.
- Represent success as `RuntimeFlags.mandatory_paid`.
- Keep events for rare/anomalous or semantic transitions:
  - `MandatoryCostFailed`
  - `LifecycleChanged`
  - `CapacityWarning`
  - `HeatWarning`
  - `WasteWarning`
  - `CellDead`
- Allow `MandatoryCostPaid` only as optional sampled/debug event, disabled by default.

## P1. ResourceGrid dense-to-sparse/LOD path

Problem:

Flat dense grid works for 512x512, but not for very large worlds such as 5120x5120 with many layers.

Impact:

Dense diffusion across tens of millions of cells becomes memory- and time-bound.

Proposal:

- Keep flat dense grid as Phase 1 implementation.
- Add scaling requirement:
  - configurable resource grid cell size independent of simulation unit;
  - downsampled grid resolution for large worlds;
  - sparse/chunked active regions as future path;
  - no full-world diffusion every Tick for huge worlds.
- Record as Phase 2/6 performance requirement, not Phase 1 blocker.

## P1. SpatialIndex large-world counting array risk

Problem:

Counting-sort grid with one counter per physical grid cell can become expensive when world size grows far beyond active population area.

Impact:

Prefix sum over millions of empty cells wastes Tick budget.

Proposal:

- Keep prefix-sum grid for Phase 1 target 512x512.
- Add future requirement:
  - fixed-size spatial hash buckets or chunked grid for large worlds;
  - index build cost should scale mainly with active cells, not physical world area;
  - deterministic collision handling and stable bucket ordering are mandatory.
- Mark as Phase 6 scale-up requirement.

## P1. Domain decomposition for parallelism

Problem:

Naive Rayon over all cells can create write conflicts for neighbor interactions.

Impact:

Joints, collisions, transfer and local interactions require deterministic conflict handling.

Proposal:

- Add future parallelism requirement:
  - spatial chunks;
  - halo zones;
  - per-chunk buffers;
  - deterministic boundary synchronization;
  - stable merge order.
- Keep Phase 1 single-thread deterministic baseline.

## P1. Snapshot/viewer I/O volume

Problem:

Full JSON snapshots of hundreds of thousands of cells and large grids are not viable.

Impact:

Disk/network I/O can dominate runtime and make viewer unusable.

Proposal:

- Clarify Phase 1 JSON/debug outputs are small-run only.
- Add viewer/storage requirements:
  - binary snapshot/event format for real runs;
  - viewport/frustum culling;
  - LOD/density maps for zoomed-out views;
  - delta-compressed viewer frames;
  - storage outside hot path.
- Link to storage/rendering docs.

## P1. L3 cache / memory-bound risk

Problem:

Even SoA can become memory-bound when hot columns exceed cache.

Impact:

Large cell counts can become limited by RAM bandwidth instead of CPU arithmetic.

Proposal:

- Add hot/cold field split requirement.
- Add compact numeric representation guidance:
  - flags as bitsets or compact integers;
  - secondary values as `u8/u16` scaled where domain allows;
  - grouped AoS for fields always read together, such as `CellPhysics { position, radius }`;
  - keep accounting wrappers but allow compact underlying representation.
- Add performance gate: measure hot type size and allocations/tick.

## Recommended Execution

1. Apply P0 fixes to `phase-1-module-api.md` before Rust implementation.
2. Add P1 scale-up requirements to `phase-1-data-model.md`, `performance.md`, and possibly `storage.md`.
3. Create a short report after applying changes.

