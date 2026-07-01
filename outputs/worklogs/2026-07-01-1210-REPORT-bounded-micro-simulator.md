# Worklog: Task 3 Bounded Micro Simulator

*   **Date:** 2026-07-01
*   **Goal:** Implement a headless micro simulator to trace cell states and evaluate dynamic collapse and warning triggers.
*   **Scope:**
    *   Implement unit tests in `tests/test_micro_simulator.py`.
    *   Implement `run_micro_simulation(config: dict) -> tuple[list, str, str]` in `src/micro_simulator.py`.
*   **Decisions:**
    *   Modeled cell states: `"alive"`, `"stressed"`, `"dormant"`, `"dead"`.
    *   Applied Phase 1 formulas:
        *   `energy_next = energy + passive_energy_income`
        *   `mandatory_paid = energy_next >= mandatory_cost`
        *   `if mandatory_paid: energy_next = min(energy_capacity, energy_next - mandatory_cost)`
        *   `heat_next = max(0.0, heat + heat_generated_per_tick - heat_dissipation_rate)`
        *   `waste_next = max(0.0, waste + waste_generated_per_tick - waste_sink_rate)`
    *   Dormancy allows cell to pay a scaled cost if allowed and payable.
    *   Death check evaluates energy <= 0 (energy depletion), heat > death threshold, waste > death threshold, or unpaid mandatory costs.
    *   Identified crossing warning thresholds (energy stress, heat, waste warning, capacity overrun, dormancy state) to classify final survival result as `"fragile"` if cell survives.
*   **Files Created/Changed:**
    *   `tools/early-stability/tests/test_micro_simulator.py` (Created)
    *   `tools/early-stability/src/micro_simulator.py` (Created)
*   **Verification:**
    *   All 31 tests passed.
*   **Open Questions:**
    *   None.
