---
tags:
  - alife
  - worklog/report
  - phase/2d
  - sweep-analyzer
  - decomposition
---

# REPORT: Phase 2D Decomposition Sweep Calibration

## Summary

Phase 2D decomposition remains a simple placeholder mechanism: dead Cell matter is released into local grid resources. This change does not implement material-specific decomposition or chemistry.

The `decomposition_viability` sweep is now mechanism-measurable instead of flat. It records death/decomposition timing, active decomposition duration, per-tick release rate, and remaining dead-cell matter.

## Changes

- Added `initial_cell_resources` to analyzer scenario presets and mapped it into `CellInitialConfig.initial_resource_amount`.
- Added decomposition observer metrics to `SimResult` and sweep CSV output:
  - `first_decomposition_tick`
  - `first_decomposed_tick`
  - `decomposition_ticks`
  - `decomposition_released_resources_per_tick`
  - `time_to_decomposed`
  - `remaining_dead_cell_resources`
  - `remaining_dead_cell_materials`
- Reused existing CSV `death_tick` column and populated it from analyzer-tracked death timing.
- Updated `LOW_INFORMATION_SWEEP` detection for `decomposition_viability` so timing/rate variation can make the sweep informative even when final energy/zone are flat.
- Recalibrated `config/analyzer/sweep_analyzer.toml` for `decomposition_viability`:
  - small local world;
  - low background resource;
  - `initial_cell_resources = 20.0`;
  - `dormancy_allowed = false`;
  - `mandatory_cost_per_tick = 50.0`;
  - `continue_after_collapse_ticks = 80`.

## Acceptance Evidence

Latest `outputs/raw_data/decomposition_viability.csv` after running `cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml`:

| decomposition_resources_per_tick | warning_codes | death_tick | first_decomposition_tick | time_to_decomposed | decomposition_ticks | release_per_tick |
|---:|---|---:|---:|---:|---:|---:|
| 1.0 | none | 0 | 0 | 19 | 20 | 1.1751 |
| 5.0 | none | 0 | 0 | 3 | 4 | 5.8752 |

The final total release remains equal across rates (`23.5010`), which is expected for conserved matter. The useful signal is rate and time-to-decomposed.

Final `remaining_dead_cell_resources` and `remaining_dead_cell_materials` are `0.0` for all rows because the analyzer now observes completion before ending the run.

## Verification

Passed:

```text
cargo test --test phase2_sweep_parser --test phase2_sweep_outputs --test phase2_sweep_warnings --test phase2_decomposition_smoke
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
cargo test --workspace --all-targets
```

## Phase Gate

Phase 2D decomposition is reachability-complete and the decomposition sweep is now mechanism-measurable.

Phase 2D still does not claim full material-specific balance. Phase 2E can proceed to material profile effects with decomposition no longer hidden behind a flat sweep.
