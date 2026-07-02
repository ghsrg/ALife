# REPORT: Realtime Architecture Algorithms

Date: 2026-07-02 11:16

## Goal

Incorporate realtime architecture analysis into implementation documentation and define required algorithms for hot paths and future phases.

## Scope

Updated:

- `docs/implementation/phase-1-data-model.md`
- `docs/engine/performance.md`

## Decisions Added

- Use double-buffered hot columns for routine per-Tick scalar updates when cheaper than delta records.
- Keep `DeltaBuffer` for semantic changes, validation, events and deterministic parallel merge, not for every scalar update.
- Require preallocated reusable transient buffers.
- Require flat `ResourceGrid` / field arrays with ping-pong buffers.
- Define diffusion/trace propagation as deterministic stencil operations with cadence, tiling and profiling before SIMD/GPU.
- Require spatial index rebuild through counting-sort / prefix-sum flat grid.
- Define typed wrapper policy: checked boundaries, narrow internal unchecked math only under documented invariants, commit-time validation/clamp.
- Add Phase 3 requirement for scheduled Genome Runtime through `next_genome_tick` and batched deterministic inference.
- Add Phase 4 requirement for SoA `JointStore` and explicit conflict strategy.
- Add Phase 2 requirement for dense process/Feasibility buffers and mandatory semantics preservation.

## Risks / Research Hooks

- Joint solving can become a random-access bottleneck. Phase 4 must choose graph coloring, independent batches or deterministic accumulate-and-commit corrections before implementation.
- SIMD/GPU should not be accepted by assumption; require release-mode benchmark against flat-array CPU baseline.
- Unchecked arithmetic in typed wrappers must stay internal and tested. `unsafe` requires profiling evidence.

## Verification

- Placeholder scan should be run after this report.
- No code was changed.

## Next Step

Continue with Phase 1 module/API documentation, using the data model and performance constraints as implementation gates.
