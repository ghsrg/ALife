# Worklog: Task 4 Refinement Plan - Detailed Markdown Reports

*   **Date:** 2026-07-01
*   **Goal:** Upgrade markdown report generation with comprehensive metadata, parameter tables, sensitivity rankings, and failure lists.
*   **Scope:**
    *   Update unit tests in `tests/test_writers.py`.
    *   Upgrade `write_report_markdown` in `src/report_writer.py`.
*   **Decisions:**
    *   Enhanced `REPORT.md` layout to render detailed base config paths, execution iterations, and final best stable candidate energy/heat/waste metrics.
    *   Programmed generation of dynamically compiled Recommended Values and Empirical Tested & Stable Ranges tables.
    *   Designed a relative parameter sensitivity ranking algorithm using the formula:
        `sensitivity_score = 1.0 - (stable_range / tested_range)`. Sorted parameters descending.
    *   Added summary logic to inspect individual run collapse outputs and list all unique collapse reasons encountered.
*   **Files Created/Changed:**
    *   `tools/early-stability/tests/test_writers.py` (Modified)
    *   `tools/early-stability/src/report_writer.py` (Modified)
*   **Verification:**
    *   Pytest asserts that all new layout headers, tables, ranks, and failure summaries are written correctly.
*   **Open Questions:**
    *   None.
