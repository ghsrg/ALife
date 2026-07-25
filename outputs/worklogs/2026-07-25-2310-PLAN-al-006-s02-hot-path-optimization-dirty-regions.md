# TDD Plan: AL-006-S02 Hot Path Optimization And Dirty Regions

## Context
High-cell-count simulations (e.g. 20k cells) require hot-path optimizations that maintain 100% bit-exact outcome determinism and match `prepared_state_hash` / `stable_state_hash` of baseline Ticks. Dirty region tracking allows spatial systems (uptake, displacement, neighbor queries) to skip inactive/clean spatial tiles without modifying simulation behavior or violating ALife Canon.

## Objectives
1. **Dirty Region Tracker (`src/core/dirty_regions.rs`)**:
   - Model `DirtyRegionTracker` spatial grid index.
   - Implement `RegionId`, tile mapping `(tile_x, tile_y)`, dirty set tracking, and tile-to-cell index lookup.
   - Implement `mark_dirty(pos)`, `clear_dirty()`, and `dirty_regions()` methods.
2. **Core Integration (`src/core/mod.rs`, `src/core/world.rs`)**:
   - Expose `pub mod dirty_regions;` in `src/core/mod.rs`.
   - Integrate `DirtyRegionTracker` into `World` state without altering execution order or snapshot output hashes.
3. **Automated Verification**:
   - Contract test suite `tests/hot_path_dirty_regions_contract.rs`.
   - Run `scale_scenarios_smoke.rs` and `scale_benchmark_harness.rs` to verify zero regression and 100% hash parity.
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Verification Plan
- `cargo test --test hot_path_dirty_regions_contract`
- `cargo test --test scale_scenarios_smoke`
- `cargo test --test scale_benchmark_harness`
- `cargo fmt --check`
- `cargo clippy --workspace --all-targets --all-features -- -D warnings`
