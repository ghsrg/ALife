# Worklog: Task 5 Refinement Plan - Tool-Only Scenarios

*   **Date:** 2026-07-01
*   **Goal:** Create and validate standard scenario files for testing simulation constraints.
*   **Scope:**
    *   Create 5 new scenario configs under `scenarios/`.
    *   Add validation/batch run integration test in `tests/test_cli.py`.
*   **Decisions:**
    *   Implemented `single_cell_growth_budget.toml` defining cells under expansion energy bounds.
    *   Implemented `single_cell_division_loop_estimate.toml` checking cycles of division energy thresholds.
    *   Implemented `waste_heat_balance.toml` verifying accumulation equilibria.
    *   Implemented `population_growth_bound.toml` validating large spatial and carrying capacity limits.
    *   Implemented `joint_upkeep_budget.toml` testing joint multicellular overhead maintenance estimates.
    *   Created `test_cli_batch_on_real_scenarios` in `tests/test_cli.py` to recursively load and validate all scenarios through CLI batch mode.
*   **Files Created/Changed:**
    *   `tools/early-stability/scenarios/single_cell_growth_budget.toml` (Created)
    *   `tools/early-stability/scenarios/single_cell_division_loop_estimate.toml` (Created)
    *   `tools/early-stability/scenarios/waste_heat_balance.toml` (Created)
    *   `tools/early-stability/scenarios/population_growth_bound.toml` (Created)
    *   `tools/early-stability/scenarios/joint_upkeep_budget.toml` (Created)
    *   `tools/early-stability/tests/test_cli.py` (Modified)
*   **Verification:**
    *   Added integration test verifying no scenarios raise `ValidationError` or return `"invalid"`.
*   **Open Questions:**
    *   None.
