---
tags:
  - alife
  - worklog/report
---

# REPORT: Phase 2C Stability Bounds (Tasks 1, 2, 3)

## Goal
Implement stability, sweep, dormancy, and stress push escape tests in a new test file `tests/phase2_stability_analysis.rs` following TDD.

## Scope
- Task 1: Parametric sweeps over metabolic/transport material balances and initial resource densities.
- Task 2: "Freeze-and-wait" dormancy survival loop verification in zero-resource environments.
- Task 3: Overlapping stress push displacement escape bounds testing.

## Decisions
1. **Grid Parameter Sweep (`test_grid_parameter_sweep`)**:
   - Swept initial resource density with low (`2.0`) and high (`1000.0`) values. Under scarce resources, the cell collapses from starvation. With high resources, it metabolizes sustainably and survives the 300-tick run.
2. **Dormancy (`test_minimum_viability_kit_and_dormancy`)**:
   - Configured cell with `1.5` initial energy and `0.2` passive income. Since `1.5` is below the mandatory cost of `2.0`, the cell enters dormancy immediately on tick 0.
   - The passive income of `0.2` exactly offsets the dormant upkeep cost (`2.0 * 0.1 = 0.2`), stabilizing cell energy and allowing dormancy to be sustained for 200 ticks.
3. **Stress Push Escaping (`test_stress_push_escaping_bounds`)**:
   - Overlapping cells are initialized. Contact pressure is manually set to `0.5` prior to step execution to ensure the contractile displacement reflex fires on Tick 0 (since the physics solver resolves overlaps at the end of the tick).
   - High force factor (`0.8`) shifts the contractile cell significantly further than a low force factor (`0.01`).

## Files Changed
- `tests/phase2_stability_analysis.rs` (NEW)
- `outputs/worklogs/index.md` (MODIFIED)
- `outputs/worklogs/2026-07-03-1815-REPORT-phase-2c-stability-bounds.md` (NEW)

## Verification
- Cargo tests: Run `cargo test --workspace` (all 86 tests passed successfully).
- Formatting: Run `cargo fmt --check` (successful, formatted style).
- Linter: Run `cargo clippy --workspace --all-targets --all-features -- -D warnings` (passed with no warnings).
- Python tests: Run `python -m pytest .\tools\early-stability` (all 93 tests passed).

## Open Questions
None.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
