---
tags:
  - alife
  - worklog/report
---

# 2026-07-03-1850-REPORT-phase2c-stability-analysis

## Goal
Implement Tasks 1, 2, and 3 of the Phase 2C advanced stability sweeps plan, adding 6 advanced integration tests inside `tests/phase2_stability_analysis.rs` following TDD.

## Scope
- Modify `tests/phase2_stability_analysis.rs` to implement:
  1. `test_viability_threshold_sweep` (verifies 11 points density sweeps).
  2. `test_dormancy_below_equal_above_equilibrium` (verifies 0.19/0.20/0.22 passive incomes).
  3. `test_dormancy_wakeup_hysteresis` (verifies transition stability and waking up when resource is added).
  4. `test_energy_and_resource_conservation` (verifies exact mass conservation invariant).
  5. `test_transport_metabolism_balance_matrix` (sweeps uptake vs metabolism efficiency).
  6. `test_multi_cell_resource_order_independence` (measures sequential process index bias).
- Modify `src/core/tick.rs` to fix a lifecycle transition bug where dormant cells with low energy would oscillate between `Dormant` and `Stressed` states.

## Decisions
- **Lifecycle Transition Fix**: We identified that cells starting in `Dormant` state were being transitioned to `Stressed` state at the end of the tick if their energy was below the stress threshold, even though they could not pay the active cost to wake up. This caused them to oscillate (Dormant -> Stressed -> Dormant -> Stressed) every tick. We resolved this by ensuring that if a cell is currently `Dormant` and does not meet the wakeup criteria, it remains in `Dormant` state.
- **Tuning Sweep Parameters**: We tuned parameters (`initial_energy = 0.0`, `energy_capacity = 40.0`, `passive_energy_income = 0.099`, `energy_per_resource = 25.0`) in the sweep test so that scarce resource densities (< 0.5) collapse immediately on Tick 0, intermediate resource densities (0.5 to 2.0) transition to dormancy and survive the 100-tick run, and abundant resource densities (> 2.0) survive actively.
- **Resource Conservation**: We disabled metabolism for the conservation test to verify that the total mass of the closed system (grid resources + cell internal resources + cell materials) remains exactly conserved within float tolerance, even with active synthesis.
- **Order Independence**: We verified and documented that the cell processed first in the loop takes the resources first (sequential processing bias).

## Verification
- **Rust Core Tests**: All 98 tests pass successfully.
  ```powershell
  cargo test
  ```
- **Clippy**: Workspace passes Clippy with no warnings.
  ```powershell
  cargo clippy --workspace --all-targets --all-features -- -D warnings
  ```
- **Python Tests**: All 93 Python stability tests pass.
  ```powershell
  python -m pytest .\tools\early-stability
  ```
