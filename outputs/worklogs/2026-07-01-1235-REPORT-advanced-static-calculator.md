# Worklog: Task 2 Advanced Static Calculator Upgrades

*   **Date:** 2026-07-01
*   **Goal:** Improve the static calculator to project heat and waste accumulation over the entire simulation duration (`tick_count`), preventing premature collapse classification for configurations where generation exceeds sink rate slightly.
*   **Scope:**
    *   Implement projected accumulation calculations:
        $$\text{projected\_heat} = \text{heat\_current} + \text{tick\_count} \times \max(0, \text{heat\_gen} - \text{heat\_diss})$$
        $$\text{projected\_waste} = \text{waste\_current} + \text{tick\_count} \times \max(0, \text{waste\_gen} - \text{waste\_sink})$$
    *   Implement warning and death boundary checks.
*   **Decisions:**
    *   Allowed configurations to be marked as `fragile` or `stable` if cumulative values stay within warning/death limits over the run, matching dynamic simulation expectations.
*   **Files Created/Changed:**
    *   `tools/early-stability/src/static_calculator.py` (Modified)
    *   `tools/early-stability/tests/test_static_calculator.py` (Modified)
*   **Verification:**
    *   Verified passing test cases for short-term survival with high thresholds and long-term collapse using pytest.
*   **Open Questions:**
    *   None.
