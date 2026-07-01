# Worklog: Task 6 CLI Interface & Batch Runner

*   **Date:** 2026-07-01
*   **Goal:** Implement the CLI entrypoint, batch runner, and populate default scenarios.
*   **Scope:**
    *   Implement unit tests in `tests/test_cli.py`.
    *   Implement CLI routing and subcommands in `src/cli.py`.
    *   Create example scenarios under `scenarios/`.
*   **Decisions:**
    *   Used standard library `argparse` to handle subcommand parsing for `evaluate`, `simulate`, `tune`, and `batch` modes.
    *   Programmed batch runner to dynamically load scenario files, sort them alphabetically, execute evaluation, and export aggregated metrics inside a single markdown table in `REPORT.md`.
    *   Wrote the 4 default scenarios to verify stable, cost starvation, initial capacity overrun, and heat stress behaviors.
*   **Files Created/Changed:**
    *   `tools/early-stability/tests/test_cli.py` (Created)
    *   `tools/early-stability/src/cli.py` (Created)
    *   `tools/early-stability/scenarios/single_cell_survival.toml` (Created)
    *   `tools/early-stability/scenarios/single_cell_starvation.toml` (Created)
    *   `tools/early-stability/scenarios/single_cell_over_capacity.toml` (Created)
    *   `tools/early-stability/scenarios/single_cell_heat_stress.toml` (Created)
*   **Verification:**
    *   Ran pytest checks. All 4 new CLI test cases passed (raising the total suite to 44 passing tests).
*   **Open Questions:**
    *   None.
