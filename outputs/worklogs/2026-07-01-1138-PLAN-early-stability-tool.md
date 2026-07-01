# Early Stability Tool Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Create a standalone Python-based offline calibration tool (`tools/early-stability`) to validate scenario configurations, execute budget calculations, run micro headless cell survival simulations, and deterministically tune stable parameters.

**Architecture:** A modular Python pipeline that decouples headless math from full engine execution. The tool consists of a Config Loader (TOML validation), Static Budget Calculator, micro Simulator (Phase 1 accounting), Tuner (grid-search tuning), and result/report exporters.

**Tech Stack:** Python 3.10+, `tomllib` (Python standard library for TOML), `json`, `hashlib`, `argparse`. No external runtime dependencies except `pytest` for testing.

---

## 1. File Structure

```text
tools/early-stability/
├── pyproject.toml
├── README.md
├── scenarios/
│   ├── single_cell_survival.toml
│   ├── single_cell_starvation.toml
│   ├── single_cell_over_capacity.toml
│   └── single_cell_heat_stress.toml
├── src/
│   ├── __init__.py
│   ├── cli.py
│   ├── config_loader.py
│   ├── static_calculator.py
│   ├── micro_simulator.py
│   ├── tuner.py
│   ├── result_writer.py
│   └── report_writer.py
└── tests/
    ├── __init__.py
    ├── test_config_loader.py
    ├── test_static_calculator.py
    ├── test_micro_simulator.py
    ├── test_tuner.py
    └── test_cli.py
```

---

## 2. CLI Entrypoint

The CLI supports four subcommands:
*   `evaluate`: Static validation + budget calculations.
*   `simulate`: Runs a micro simulation over a fixed number of ticks.
*   `tune`: Runs grid-search parameter optimization.
*   `batch`: Runs all scenarios in a directory alphabetically.

### Usage Commands:
```bash
early-stability evaluate --scenario scenarios/single_cell_survival.toml --out outputs/stability/<run_id>/
early-stability simulate --scenario scenarios/single_cell_survival.toml --ticks 1000 --out outputs/stability/<run_id>/
early-stability tune --scenario scenarios/single_cell_survival.toml --tuning tuning/single_cell.toml --out outputs/stability/<run_id>/
early-stability batch --scenarios scenarios/ --out outputs/stability/<run_id>/
```

---

## 3. Format Specifications

### A. Scenario TOML Format (`scenarios/single_cell_survival.toml`)
```toml
scenario_id = "single_cell_survival"
seed = 42
tick_count = 100

[world]
size = [512.0, 512.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["water", "nutrient"]
initial_distribution = [10.0, 5.0]
optional_decay_rate = 0.01
passive_energy_income_placeholder = 1.0

[cell]
initial_position = [256.0, 256.0]
radius = 1.0
initial_resources = [2.0, 1.0]
initial_materials = [5.0]
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.1
capacity_limit = 50.0
minimum_viability_materials = [1.0]

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.1
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_current = 0.0
waste_generated_per_tick = 0.05
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
critical_capacity_overrun = 5.0

[estimates]
growth_cost_estimate = 10.0
division_cost_estimate = 20.0
resource_regeneration_or_inflow = 5.0
population_space_limit = 100
joint_count_estimate = 0
joint_upkeep_cost = 0.0
```

### B. Tuning TOML Format (`tuning/single_cell.toml`)
```toml
[tuning]
max_iterations = 100
seeds = [42, 100, 2026]
objective = "find_first_stable"

[tuning.ranges]
"cell.initial_energy" = [5.0, 50.0, 5.0]
"environment.heat_dissipation_rate" = [0.05, 0.5, 0.05]
"cell.mandatory_cost_per_tick" = [1.0, 10.0, 1.0]
```

### C. JSON Schemas / Examples of Output

#### `results.json`
```json
{
  "scenario_id": "single_cell_survival",
  "config_hash": "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855",
  "seed": 42,
  "tick_count": 100,
  "survival_result": "stable",
  "collapse_reason": "none",
  "metrics_summary": {
    "final_energy": 45.2,
    "final_heat": 0.0,
    "final_waste": 0.0,
    "warnings_triggered": false
  }
}
```

#### `ranges.json`
```json
[
  {
    "parameter_id": "cell.initial_energy",
    "tested_min": 5.0,
    "tested_max": 50.0,
    "stable_min": 20.0,
    "stable_max": 50.0,
    "recommended": 35.0,
    "confidence": "high",
    "notes": "Values below 20.0 lead to mandatory_cost_unpaid collapse"
  }
]
```

---

## 4. Algorithms

### A. Algorithm: `evaluate`
1.  **Parse & Validate Config**: Use `config_loader` to parse the scenario TOML. Validate that all required properties exist and have positive values. If validation fails, return `survival_result = invalid`, `collapse_reason = invalid_config`.
2.  **Static Budget Calculations**:
    *   Verify that `initial_energy + passive_energy_income >= mandatory_cost_per_tick`. If not, return `collapse`, `mandatory_cost_unpaid`.
    *   Verify that `sum(initial_resources) + sum(initial_materials) <= capacity_limit`. If not, return `collapse`, `capacity_exceeded`.
    *   Verify that `heat_generated_per_tick <= heat_dissipation_rate`. If not, return `collapse`, `heat_limit_exceeded`.
    *   Verify that `waste_generated_per_tick <= waste_sink_rate`. If not, return `collapse`, `waste_limit_exceeded`.
3.  **Return Classification**: If all static checks pass, evaluate returns `stable` and `none`.

### B. Algorithm: `tune`
1.  **Initialize Tuning**: Load base scenario TOML and tuning TOML parameters. Extract objective (`find_first_stable` or `map_stable_ranges`) and parameter ranges.
2.  **Generate Grid Candidates**: Create the Cartesian product or linear lists of parameter values defined in `tuning.ranges`.
3.  **Evaluate Loop**: For each candidate configuration:
    *   Set the nested parameter value in the configuration.
    *   For each seed in the seed list:
        *   Run `simulate` for the configured `tick_count`.
        *   Save intermediate simulation outputs to `runs/run_XXXX.json`.
    *   If all seeds achieve `stable` result, mark parameter values as `stable`.
    *   If objective is `find_first_stable` and stable values exist, stop search.
4.  **Export Recommendations**: Determine `stable_min`, `stable_max`, and write recommended TOML files under `recommended-configs/`.

---

## 5. Implementation Tasks

### Task 1: Setup and Config Validation

**Files:**
- Create: `tools/early-stability/pyproject.toml`
- Create: `tools/early-stability/src/config_loader.py`
- Create: `tools/early-stability/tests/test_config_loader.py`

- [ ] **Step 1: Create pyproject.toml**
Write Hatch-based manifest to `tools/early-stability/pyproject.toml`.

- [ ] **Step 2: Write tests for Validation Rules**
Verify validation raises `ValidationError` for negative radius, stored resource capacity overrun, negative values, and unknown keys.
```python
# test_config_loader.py code to implement
def test_negative_radius_fails():
    toml_str = "cell = { radius = -1.0 }"
    with pytest.raises(ValidationError):
        load_and_validate_config(toml_str)
```

- [ ] **Step 3: Implement Config Loader**
Write standard `tomllib` parsing and dictionary validation rules.

- [ ] **Step 4: Run tests to verify correctness**
Command: `pytest tools/early-stability/tests/test_config_loader.py`

- [ ] **Step 5: Commit**
Command: `git add tools/early-stability/ && git commit -m "feat(stability): setup config loader and validation tests"`

---

### Task 2: Static Budget Calculator

**Files:**
- Create: `tools/early-stability/src/static_calculator.py`
- Create: `tools/early-stability/tests/test_static_calculator.py`

- [ ] **Step 1: Write static budget tests**
Test scenarios where `mandatory_cost` exceeds budget, resources exceed capacity limits, or heat/waste sinks are smaller than generation rates.

- [ ] **Step 2: Implement static calculator logic**
Compute equations:
*   `initial_energy + passive_income >= mandatory_cost`
*   `used_capacity <= capacity_limit`
*   `heat_gen <= heat_dissipation`
*   `waste_gen <= waste_sink`

- [ ] **Step 3: Run pytest**
Verify all static bound tests pass.

- [ ] **Step 4: Commit**
Command: `git add tools/early-stability/src/static_calculator.py && git commit -m "feat(stability): static budget calculator"`

---

### Task 3: Bounded Micro Simulator

**Files:**
- Create: `tools/early-stability/src/micro_simulator.py`
- Create: `tools/early-stability/tests/test_micro_simulator.py`

- [ ] **Step 1: Write tests for simulation states**
Test transitions: `alive -> stressed` (warning threshold crossed), `stressed -> dormant` (dormancy allowed), and `stressed -> dead` (energy depleted, heat death, waste death).

- [ ] **Step 2: Implement micro simulator tick loop**
Implement Phase 1 accounting:
```python
# Formulas
energy_after_mandatory = min(energy_capacity, energy + passive_income - mandatory_cost)
heat_next = max(0.0, heat + heat_gen - heat_diss)
waste_next = max(0.0, waste + waste_gen - waste_sink)
```

- [ ] **Step 3: Verify execution and thresholds**
Ensure correct classifications of `survival_result` and `collapse_reason`.

- [ ] **Step 4: Commit**
Command: `git add tools/early-stability/src/micro_simulator.py && git commit -m "feat(stability): micro simulation tick loop"`

---

### Task 4: Deterministic Grid Tuner

**Files:**
- Create: `tools/early-stability/src/tuner.py`
- Create: `tools/early-stability/tests/test_tuner.py`

- [ ] **Step 1: Write tests for parameter search**
Verify grid search finds the minimum required `initial_energy` for a cell to survive given starvation parameters.

- [ ] **Step 2: Implement nested key parameter tuning**
Iterate values, run simulation, evaluate stability, aggregate empirical min/max ranges.

- [ ] **Step 3: Verify grid search deterministic ordering**
Ensure same seeds and ranges always produce identical results.

- [ ] **Step 4: Commit**
Command: `git add tools/early-stability/src/tuner.py && git commit -m "feat(stability): parameter tuning search"`

---

### Task 5: Result & Report Exporters

**Files:**
- Create: `tools/early-stability/src/result_writer.py`
- Create: `tools/early-stability/src/report_writer.py`
- Create: `tools/early-stability/tests/test_writers.py`

- [ ] **Step 1: Write tests for file output formats**
Verify JSON contents of `results.json`, `ranges.json`, and formatting of `REPORT.md`.

- [ ] **Step 2: Implement JSON and Markdown report generation**
Structure Markdown reports to display run parameters, stable/fragile status, and empirical ranges.

- [ ] **Step 3: Run pytest**
Confirm report file outputs write correctly to temporary directories.

- [ ] **Step 4: Commit**
Command: `git add tools/early-stability/src/res* && git commit -m "feat(stability): report and json writers"`

---

### Task 6: CLI Interface and Batch Runner

**Files:**
- Create: `tools/early-stability/src/cli.py`
- Create: `tools/early-stability/tests/test_cli.py`

- [ ] **Step 1: Write CLI parser tests**
Assert CLI commands (`evaluate`, `simulate`, `tune`, `batch`) route inputs and output folders correctly.

- [ ] **Step 2: Implement CLI commands using argparse**
Parse parameters, execute matching tasks, and output to path target.

- [ ] **Step 3: Implement batch directory execution**
Process all scenarios in a folder sorted alphabetically.

- [ ] **Step 4: Run CLI validation tests**
Ensure command executions write files without exceptions.

- [ ] **Step 5: Commit**
Command: `git add tools/early-stability/src/cli.py && git commit -m "feat(stability): cli entrypoint and batch execution"`

---

## 6. Acceptance Criteria

1.  **Standalone Execution**: The tool can run fully headless without imports from `alife-core` or external libraries except standard library `tomllib`/`json`/`argparse`.
2.  **Output Integrity**: Running commands creates `results.json`, `REPORT.md`, and `ranges.json` inside the requested `outputs/stability/<run_id>/` folder.
3.  **Strict Determinism**: Re-running the tool with the same seed, config, and parameters produces identical byte-for-byte outputs (excluding run timestamp and path references in logs).
4.  **Static Bounds Validation**: Static validation marks configs as `invalid` immediately for negative values, footprint mismatch, or initial capacity overrun.
5.  **Preservation of Source TOML**: The tool must never overwrite source configuration TOML files in-place; all candidates must be saved under the `outputs/stability/<run_id>/` directory.
