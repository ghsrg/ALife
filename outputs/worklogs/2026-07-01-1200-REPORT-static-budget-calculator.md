# Worklog: Task 2 Static Budget Calculator

*   **Date:** 2026-07-01
*   **Goal:** Implement static stability budget checks.
*   **Scope:**
    *   Implement unit tests in `tests/test_static_calculator.py`.
    *   Implement `evaluate_static_bounds(config: dict) -> tuple[str, str]` in `src/static_calculator.py`.
*   **Decisions:**
    *   Checked mandatory costs against combined initial energy and passive income.
    *   Evaluated initial capacity limits by summing values from dictionaries/lists of initial resources and materials.
    *   Compared heat generation and dissipation to prevent unbounded thermal accumulation.
    *   Compared waste generation and sink rate to prevent unbounded waste accumulation.
    *   Allowed checking to run on direct configuration objects to decouple logic from I/O.
*   **Files Created/Changed:**
    *   `tools/early-stability/tests/test_static_calculator.py` (Created)
    *   `tools/early-stability/src/static_calculator.py` (Created)
*   **Verification:**
    *   Executed test suite using pytest.
    *   All 6 new test scenarios passed (raising total passing tests from 19 to 25).
*   **Open Questions:**
    *   None.
