# Early Stability Tool Refinement Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Refine the Early Stability Tool (`tools/early-stability`) to fully meet documentation spec specifications. This includes writing complete documentation, example tuning configs, aligning tuning schemas, implementing detailed reports, robust validation rules, a smarter static calculator, and all missing tool-only scenarios.

**Architecture:** Extend the existing modular Python tool by updating config loaders, calculators, simulators, tuners, and report writers to use comprehensive calculations and explicit validation schemas.

**Tech Stack:** Python 3.10+, standard libraries (`tomllib`, `json`, `argparse`, `math`).

---

## Proposed Changes

We will modify and create files under `tools/early-stability/` to complete the required features:

*   `tools/early-stability/README.md` [MODIFY] — Expand with comprehensive setup, usage, commands, and formats.
*   `tools/early-stability/tuning/single_cell.toml` [NEW] — Example tuning config aligned with schemas.
*   `tools/early-stability/src/config_loader.py` [MODIFY] — Add robust validation checks for unknown IDs, min viability materials, and threshold consistency.
*   `tools/early-stability/src/static_calculator.py` [MODIFY] — Implement tick_count and threshold-aware heat/waste accumulation logic.
*   `tools/early-stability/src/tuner.py` [MODIFY] — Support allowed_parameters, parameter_ranges, candidate_profiles, and find_conservative_stable objective.
*   `tools/early-stability/src/report_writer.py` [MODIFY] — Expand markdown reporter with best candidates, recommended values, empirical ranges, and sensitivities.
*   `tools/early-stability/scenarios/*.toml` [NEW] — Create missing tool-only scenarios.

---

### Task 1: Robust Input Validation and Config Loader Upgrades

**Files:**
- Modify: `tools/early-stability/src/config_loader.py:1-40`
- Modify: `tools/early-stability/tests/test_config_loader.py:40-100`

- [ ] **Step 1: Write failing tests for validation logic**
Add tests to `tools/early-stability/tests/test_config_loader.py` to ensure `ValidationError` is raised for:
*   Warning threshold greater than death threshold (e.g. `heat_warning_threshold > heat_death_threshold`).
*   Empty `minimum_viability_materials`.
*   Unknown resources used in initial resource settings.
```python
def test_threshold_validation():
    toml = "... heat_warning_threshold = 90.0, heat_death_threshold = 80.0 ..."
    with pytest.raises(ValidationError):
        load_and_validate_config(toml)
```

- [ ] **Step 2: Run pytest to verify failures**
Command: `pytest tools/early-stability/tests/test_config_loader.py`

- [ ] **Step 3: Implement validation upgrades**
Update `tools/early-stability/src/config_loader.py`:
*   Add cross-field checks: `warning_threshold < death_threshold` for both heat and waste.
*   Validate `minimum_viability_materials` contains valid identifiers.
*   Check that cell initial resources map only to defined `resources.resource_type_ids`.

- [ ] **Step 4: Verify tests pass**
Command: `pytest tools/early-stability/tests/test_config_loader.py`

- [ ] **Step 5: Commit**
```bash
git add tools/early-stability/src/config_loader.py tools/early-stability/tests/test_config_loader.py
git commit -m "refactor(stability): add robust schema validation to config loader"
```

---

### Task 2: Advanced Static Calculator and Scenario-aware Math

**Files:**
- Modify: `tools/early-stability/src/static_calculator.py:1-50`
- Modify: `tools/early-stability/tests/test_static_calculator.py:20-80`

- [ ] **Step 1: Write tests for static calculator adjustments**
Test that if `heat_generated_per_tick > heat_dissipation_rate` but `(heat_generated_per_tick - heat_dissipation_rate) * tick_count` does not exceed the warning or death threshold, it does NOT collapse (or is flagged only as fragile/warning).

- [ ] **Step 2: Run pytest to verify failures**
Command: `pytest tools/early-stability/tests/test_static_calculator.py`

- [ ] **Step 3: Update static calculator formulas**
Modify `tools/early-stability/src/static_calculator.py`:
*   Integrate `tick_count` to project cumulative heat and waste:
    $$\Delta\text{Heat} = \text{tick\_count} \times \max(0, \text{heat\_gen} - \text{heat\_diss})$$
    $$\Delta\text{Waste} = \text{tick\_count} \times \max(0, \text{waste\_gen} - \text{waste\_sink})$$
*   Check if projected values exceed warning or death thresholds, returning `fragile` or `collapse` appropriately.

- [ ] **Step 4: Verify tests pass**
Command: `pytest tools/early-stability/tests/test_static_calculator.py`

- [ ] **Step 5: Commit**
```bash
git add tools/early-stability/src/static_calculator.py tools/early-stability/tests/test_static_calculator.py
git commit -m "feat(stability): integrate tick_count and warnings into static calculator"
```

---

### Task 3: Tuner Upgrades (Tuning Config, Schema, Objectives, Profiles)

**Files:**
- Create: `tools/early-stability/tuning/single_cell.toml`
- Modify: `tools/early-stability/src/tuner.py:1-100`
- Modify: `tools/early-stability/tests/test_tuner.py:1-60`

- [ ] **Step 1: Create example tuning config**
Write complete specification parameters to `tools/early-stability/tuning/single_cell.toml`.

- [ ] **Step 2: Write failing tests for tuner improvements**
Add tests to verify:
*   Tuner explicitly rejects modifying parameters not listed in `allowed_parameters`.
*   Tuner executes `find_conservative_stable` objective (returns configuration with maximum distance from warning thresholds).
*   Tuner generates candidate profiles (`best_stable`, `conservative_stable`, `fragile_edge`).

- [ ] **Step 3: Run pytest to verify failures**
Command: `pytest tools/early-stability/tests/test_tuner.py`

- [ ] **Step 4: Update tuner logic**
Modify `tools/early-stability/src/tuner.py`:
*   Validate incoming configs using `allowed_parameters` and `parameter_ranges`.
*   Implement `find_conservative_stable` search algorithm.
*   Separate results to extract the requested profiles.

- [ ] **Step 5: Verify tests pass**
Command: `pytest tools/early-stability/tests/test_tuner.py`

- [ ] **Step 6: Commit**
```bash
git add tools/early-stability/src/tuner.py tools/early-stability/tests/test_tuner.py tools/early-stability/tuning/single_cell.toml
git commit -m "feat(stability): upgrade tuner schema, profiles and conservative objective"
```

---

### Task 4: Detailed Markdown Reports and Outputs

**Files:**
- Modify: `tools/early-stability/src/report_writer.py:1-80`
- Modify: `tools/early-stability/tests/test_writers.py:20-60`

- [ ] **Step 1: Write tests for report writer additions**
Verify generated `REPORT.md` contains best candidate configs, recommended values table, tested min/max stable ranges, and sensitivity rankings.

- [ ] **Step 2: Run pytest to verify failures**
Command: `pytest tools/early-stability/tests/test_writers.py`

- [ ] **Step 3: Update report writer formatting**
Modify `tools/early-stability/src/report_writer.py` to output the complete markdown sections matching spec guidelines.

- [ ] **Step 4: Verify tests pass**
Command: `pytest tools/early-stability/tests/test_writers.py`

- [ ] **Step 5: Commit**
```bash
git add tools/early-stability/src/report_writer.py tools/early-stability/tests/test_writers.py
git commit -m "feat(stability): enhance report markdown output details"
```

---

### Task 5: Tool-Only Scenarios Implementation

**Files:**
- Create: `tools/early-stability/scenarios/single_cell_growth_budget.toml`
- Create: `tools/early-stability/scenarios/single_cell_division_loop_estimate.toml`
- Create: `tools/early-stability/scenarios/waste_heat_balance.toml`
- Create: `tools/early-stability/scenarios/population_growth_bound.toml`
- Create: `tools/early-stability/scenarios/joint_upkeep_budget.toml`

- [ ] **Step 1: Create growth budget scenario**
Define initial variables for growth estimation equations.

- [ ] **Step 2: Create division loop scenario**
Write TOML simulating division criteria constraints.

- [ ] **Step 3: Create waste heat balance scenario**
Write TOML validating stable equilibrium thresholds.

- [ ] **Step 4: Create population limit and joint upkeep scenarios**
Define parameters for multi-cell stability checks.

- [ ] **Step 5: Run batch command to verify all scenarios run successfully**
Command: `python tools/early-stability/src/cli.py batch --scenarios tools/early-stability/scenarios/ --out outputs/stability/batch_test/`

- [ ] **Step 6: Commit**
```bash
git add tools/early-stability/scenarios/
git commit -m "feat(stability): add missing tool-only scenario files"
```

---

### Task 6: README Documentation and Final Verification

**Files:**
- Modify: `tools/early-stability/README.md`

- [ ] **Step 1: Write complete README**
Detail setup instructions, CLI inputs/outputs, TOML structure, outputs folder mapping, and verification examples.

- [ ] **Step 2: Run entire test suite**
Command: `pytest tools/early-stability/tests/`
Expected: 100% PASS.

- [ ] **Step 3: Commit**
```bash
git add tools/early-stability/README.md
git commit -m "docs(stability): update README documentation and wrap up tool implementation"
```

---

## Verification Plan

### Automated Tests
- Run all developed unit tests:
```bash
pytest tools/early-stability/tests/
```

### Manual Verification
- Run a full batch check over the new scenario directory:
```bash
python tools/early-stability/src/cli.py batch --scenarios tools/early-stability/scenarios/ --out outputs/stability/final_batch/
```
Verify generated aggregated reports and tables.
