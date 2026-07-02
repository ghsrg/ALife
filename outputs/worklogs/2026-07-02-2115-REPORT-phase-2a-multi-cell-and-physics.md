# REPORT: Phase 2A Multi-Cell and Physics Solver Baseline

## Goal
Implement support for multiple cells and a deterministic physical solver loop using a rebuildable uniform SpatialIndex.

## Scope & Decisions
- **Multi-Cell Configuration**: Added `initial_cells` to `RuntimeConfig`. Backwards compatibility is maintained by default-initializing `initial_cells = vec![cell]` and synchronizing the single-cell `self.cell` field to reference the first cell in `with_cells()`.
- **TOML Parser Support**: Extended `RawScenarioConfig` to parse `[[cells]]` arrays of tables from scenario files, mapping them to coordinate wrappers.
- **Counting-Sort SpatialIndex**: Replaced the mock `SpatialIndex` with a uniform grid structure. It calculates rows and columns from `WorldSize` and `grid_size`, registers alive cells to grid cells without dynamic heap allocations via counting-sort prefix sums, and yields unique sorted candidate neighbor pairs in `O(N + C)` time complexity.
- **Position-Based Dynamics (PBD) Solver**: Implemented Verlet relaxation iterations inside `TickExecutor::step()`. Cell-cell overlaps push cells apart equally along their normal; solid-wall boundary clamping restricts alive cells to the grid area bounds `[radius, size - radius]`.
- **Summary Metrics**: Added `overlap_resolved` distance to `MetricsSummary` to monitor collision intensity.

## Files Changed
- `src/core/config.rs`
- `src/runner/config_parser.rs`
- `src/core/world.rs`
- `src/core/spatial.rs`
- `src/core/tick.rs`
- `src/core/summary.rs`
- `src/core/cell_store.rs`
- `tests/phase1_accounting.rs`
- `tests/phase1_config_validation.rs`
- `tests/phase1_core_smoke.rs`
- `tests/phase1_determinism.rs`
- `tests/phase1_resource_grid.rs`
- `tests/phase1_resource_interaction.rs`
- `tests/phase1_sustained_viability.rs`
- `tests/phase2_core_smoke.rs` [NEW]
- `outputs/worklogs/README.md`

## Verification
- **Rust Core Suite**: `cargo test` -> **PASS (61 tests)**.
- **New Tests**: `tests/phase2_core_smoke.rs` validates multi-cell parser mapping, counting sort neighborhood query, positional correction separating cells, and perfect bit-determinism across ticks.
- **Formatting and Lints**: `cargo fmt` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` -> **PASS**.
- **Python Suite**: `pytest tools/early-stability` -> **PASS (93 tests)**.

## Open Questions
- None.
