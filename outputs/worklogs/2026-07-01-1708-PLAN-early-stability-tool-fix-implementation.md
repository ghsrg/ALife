# Early Stability Tool Fix Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Fix the reviewed gaps in `tools/early-stability/` so tune outputs, reports, validation, CLI behavior and repository hygiene match `docs/implementation/early-stability-tool.md`.

**Architecture:** Keep the tool as a standalone Python offline helper outside `alife-core`. Preserve deterministic behavior, write artifacts under `outputs/stability/<run_id>/`, and do not mutate source configs automatically.

**Tech Stack:** Python 3.11, stdlib `tomllib/json/argparse`, pytest, TOML scenarios under `tools/early-stability/scenarios/`.

---

## Context

Read before implementation:

- `docs/implementation/early-stability-tool.md`
- `docs/implementation/phase-1-design.md`
- `outputs/worklogs/2026-07-01-1614-PLAN-early-stability-tool-review-fixes.md`

Current known issues:

- tune `REPORT.md` does not receive `ranges`, `runs`, `best_stable_metrics`, `base_config_path`;
- tune `runs/*.json` lacks `history` and final metrics;
- `evaluate` and `batch` have no optional micro simulation path;
- tuning config validation silently ignores bad/empty parameter sets;
- config validation is incomplete for required numeric fields;
- README has mojibake tree characters;
- tracked `outputs/worklogs` conflicts with `.gitignore` intent.

---

## File Structure

Modify:

- `tools/early-stability/src/cli.py`
- `tools/early-stability/src/tuner.py`
- `tools/early-stability/src/config_loader.py`
- `tools/early-stability/src/report_writer.py`
- `tools/early-stability/README.md`
- `tools/early-stability/pyproject.toml`
- `tools/early-stability/tests/test_cli.py`
- `tools/early-stability/tests/test_tuner.py`
- `tools/early-stability/tests/test_config_loader.py`
- `tools/early-stability/tests/test_writers.py`
- `.gitignore`

Create:

- `tools/early-stability/tests/helpers.py`
- `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-early-stability-tool-fixes.md`

Do not create or commit generated `outputs/stability/` artifacts.

---

### Task 1: Pass tune report data into `REPORT.md`

**Files:**
- Modify: `tools/early-stability/src/cli.py`
- Modify: `tools/early-stability/tests/test_cli.py`

- [ ] **Step 1: Add failing test for tune report contents**

Add a test to `tools/early-stability/tests/test_cli.py`:

```python
def test_cli_tune_report_contains_ranges_and_failures(setup_files):
    args = [
        "tune",
        "--scenario", setup_files["scenario_path"],
        "--tuning", setup_files["tuning_path"],
        "--out", setup_files["out_dir"],
    ]
    main(args)

    report_path = os.path.join(setup_files["out_dir"], "REPORT.md")
    assert os.path.exists(report_path)

    report = open(report_path, "r", encoding="utf-8").read()
    assert "Recommended Values Table" in report
    assert "Empirical Tested & Stable Ranges" in report
    assert "Parameter Sensitivity Ranking" in report
    assert "Failure Reasons Summary" in report
    assert "cell.initial_energy" in report
```

- [ ] **Step 2: Run the focused test and confirm failure**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_cli.py::test_cli_tune_report_contains_ranges_and_failures -q
```

Expected before fix: FAIL because report lacks parameter range details.

- [ ] **Step 3: Pass report data from `run_tune_mode()`**

In `tools/early-stability/src/cli.py`, update `report_data` in `run_tune_mode()` to include:

```python
report_data = {
    "run_id": base_config.get("scenario_id", "tuned_scenario"),
    "mode": "tune",
    "base_config_path": scenario_path,
    "iteration_count": len(runs),
    "stable_count": stable_count,
    "fragile_count": fragile_count,
    "collapse_count": collapse_count,
    "invalid_count": invalid_count,
    "parameter_ranges": ranges,
    "runs": runs,
    "best_stable_metrics": next(
        (
            {
                "final_energy": r.get("final_energy", 0.0),
                "final_heat": r.get("final_heat", 0.0),
                "final_waste": r.get("final_waste", 0.0),
            }
            for r in runs
            if r.get("survival_result") == "stable"
        ),
        None,
    ),
    "warnings": ["Tuning did not find any stable configurations."] if stable_count == 0 else [],
    "limits_of_evidence": ["Evaluated with deterministic parameter grid tuner."],
}
```

- [ ] **Step 4: Run test again**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_cli.py::test_cli_tune_report_contains_ranges_and_failures -q
```

Expected: PASS.

---

### Task 2: Store tune history and final metrics in `runs/*.json`

**Files:**
- Modify: `tools/early-stability/src/tuner.py`
- Modify: `tools/early-stability/tests/test_tuner.py`

- [ ] **Step 1: Add failing test for run history**

Add to `tools/early-stability/tests/test_tuner.py`:

```python
def test_tune_runs_include_history_and_final_metrics(base_config, tuning_config):
    runs, ranges, profiles = run_tuning(base_config, tuning_config)

    first = runs[0]
    assert "history" in first
    assert isinstance(first["history"], list)
    assert "final_energy" in first
    assert "final_heat" in first
    assert "final_waste" in first
    assert "final_state" in first
```

- [ ] **Step 2: Run focused test and confirm failure**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_tuner.py::test_tune_runs_include_history_and_final_metrics -q
```

Expected before fix: FAIL.

- [ ] **Step 3: Add history/final metrics to `run_record`**

In `tools/early-stability/src/tuner.py`, replace `run_record` construction with:

```python
final_step = history[-1] if history else {}
run_record = {
    "parameters": copy.deepcopy(candidate),
    "seed": seed,
    "survival_result": result,
    "collapse_reason": reason,
    "history": history,
    "final_energy": final_step.get("energy", 0.0),
    "final_heat": final_step.get("heat", 0.0),
    "final_waste": final_step.get("waste", 0.0),
    "final_state": final_step.get("state", "unknown"),
}
```

Keep downstream final metric calculations using the same `final_step`.

- [ ] **Step 4: Run focused test**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_tuner.py::test_tune_runs_include_history_and_final_metrics -q
```

Expected: PASS.

---

### Task 3: Validate tuning config and parameter paths

**Files:**
- Modify: `tools/early-stability/src/tuner.py`
- Modify: `tools/early-stability/tests/test_tuner.py`

- [ ] **Step 1: Add validation helpers**

Add to `tools/early-stability/src/tuner.py`:

```python
class TuningValidationError(Exception):
    pass


def path_exists(d: dict, key_str: str) -> bool:
    parts = key_str.split(".")
    curr = d
    for part in parts:
        if not isinstance(curr, dict) or part not in curr:
            return False
        curr = curr[part]
    return True


def validate_tuning_config(base_config: dict, tuning_config: dict) -> dict:
    tuning_sect = tuning_config.get("tuning", {})
    ranges_spec = tuning_sect.get("ranges", {})
    allowed_params = tuning_sect.get("allowed_parameters")

    if not isinstance(ranges_spec, dict) or not ranges_spec:
        raise TuningValidationError("tuning.ranges must be a non-empty table")

    if allowed_params is None or not isinstance(allowed_params, list) or not allowed_params:
        raise TuningValidationError("tuning.allowed_parameters must be a non-empty list")

    allowed_set = set(allowed_params)
    range_set = set(ranges_spec.keys())

    missing_ranges = allowed_set - range_set
    if missing_ranges:
        raise TuningValidationError(f"allowed parameters missing ranges: {sorted(missing_ranges)}")

    extra_ranges = range_set - allowed_set
    if extra_ranges:
        raise TuningValidationError(f"ranges include parameters not allowed: {sorted(extra_ranges)}")

    for param in allowed_params:
        if not path_exists(base_config, param) and not param.startswith("estimates."):
            raise TuningValidationError(f"parameter path does not exist in base config: {param}")

    for param, spec in ranges_spec.items():
        if not isinstance(spec, list) or len(spec) != 3:
            raise TuningValidationError(f"range for {param} must be [start, end, step]")
        start, end, step = spec
        if not all(isinstance(v, (int, float)) for v in [start, end, step]):
            raise TuningValidationError(f"range for {param} must contain numbers")
        if step <= 0:
            raise TuningValidationError(f"range step for {param} must be positive")

    return tuning_sect
```

- [ ] **Step 2: Use validation in `run_tuning()`**

In `run_tuning()`, replace direct section extraction with:

```python
tuning_sect = validate_tuning_config(base_config, tuning_config)
max_iterations = tuning_sect.get("max_iterations", 100)
seeds = tuning_sect.get("seeds", [42])
objective = tuning_sect.get("objective", "map_stable_ranges")
ranges_spec = tuning_sect.get("ranges", {})
allowed_params = tuning_sect.get("allowed_parameters")
```

Keep the deterministic `sorted_params = sorted(ranges_spec.keys())`.

- [ ] **Step 3: Add tests for invalid tuning config**

Add to `tools/early-stability/tests/test_tuner.py`:

```python
import pytest
from tuner import TuningValidationError


def test_tuning_rejects_allowed_parameter_without_range(base_config, tuning_config):
    tuning_config["tuning"]["allowed_parameters"] = ["cell.initial_energy", "cell.energy_capacity"]
    with pytest.raises(TuningValidationError):
        run_tuning(base_config, tuning_config)


def test_tuning_rejects_range_not_in_allowed_parameters(base_config, tuning_config):
    tuning_config["tuning"]["allowed_parameters"] = ["cell.initial_energy"]
    tuning_config["tuning"]["ranges"]["cell.energy_capacity"] = [50.0, 100.0, 25.0]
    with pytest.raises(TuningValidationError):
        run_tuning(base_config, tuning_config)


def test_tuning_rejects_unknown_non_estimate_path(base_config, tuning_config):
    tuning_config["tuning"]["allowed_parameters"] = ["cell.unknown_field"]
    tuning_config["tuning"]["ranges"] = {"cell.unknown_field": [1.0, 2.0, 1.0]}
    with pytest.raises(TuningValidationError):
        run_tuning(base_config, tuning_config)
```

- [ ] **Step 4: Run tuner tests**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_tuner.py -q
```

Expected: PASS.

---

### Task 4: Add optional simulation for evaluate and batch

**Files:**
- Modify: `tools/early-stability/src/cli.py`
- Modify: `tools/early-stability/tests/test_cli.py`

- [ ] **Step 1: Add result precedence helper**

Add to `tools/early-stability/src/cli.py`:

```python
RESULT_RANK = {
    "stable": 0,
    "fragile": 1,
    "collapse": 2,
    "invalid": 3,
}


def worse_result(static_result: str, static_reason: str, sim_result: str, sim_reason: str) -> tuple[str, str]:
    if RESULT_RANK[sim_result] > RESULT_RANK[static_result]:
        return sim_result, sim_reason
    return static_result, static_reason
```

- [ ] **Step 2: Add CLI flags**

Update parser:

```python
eval_parser.add_argument("--with-simulation", action="store_true")
batch_parser.add_argument("--with-simulation", action="store_true")
```

Update dispatch:

```python
run_evaluate_mode(args.scenario, args.out, args.with_simulation)
run_batch_mode(args.scenarios, args.out, args.with_simulation)
```

Update function signatures:

```python
def run_evaluate_mode(scenario_path: str, out_dir: str, with_simulation: bool = False):
def run_batch_mode(scenarios_dir: str, out_dir: str, with_simulation: bool = False):
```

- [ ] **Step 3: Run micro simulation when requested**

In `run_evaluate_mode()`, after static result:

```python
history = []
if with_simulation and result in ("stable", "fragile"):
    history, sim_result, sim_reason = run_micro_simulation(config)
    result, reason = worse_result(result, reason, sim_result, sim_reason)
```

Include final energy/heat/waste in `metrics_summary` when `history` exists.

In `run_batch_mode()`, apply the same logic per scenario.

- [ ] **Step 4: Add tests**

Add to `tools/early-stability/tests/test_cli.py`:

```python
def test_cli_evaluate_with_simulation_can_downgrade_static_stable(setup_files):
    args = [
        "evaluate",
        "--scenario", setup_files["scenario_path"],
        "--out", setup_files["out_dir"],
        "--with-simulation",
    ]
    main(args)

    results_json_path = os.path.join(setup_files["out_dir"], "results.json")
    assert os.path.exists(results_json_path)
    res = json.load(open(results_json_path, "r", encoding="utf-8"))
    assert res["survival_result"] in ["stable", "fragile", "collapse", "invalid"]
```

Add a second scenario in the test where static passes but simulation collapses by setting:

```toml
initial_energy = 3.0
mandatory_cost_per_tick = 1.0
passive_energy_income_placeholder = 0.0
tick_count = 10
```

Expected with `--with-simulation`: `collapse`.

- [ ] **Step 5: Run CLI tests**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_cli.py -q
```

Expected: PASS.

---

### Task 5: Expand config validation

**Files:**
- Modify: `tools/early-stability/src/config_loader.py`
- Modify: `tools/early-stability/tests/test_config_loader.py`

- [ ] **Step 1: Add validation helper functions**

Add to `config_loader.py`:

```python
def require_number(config: dict, path: str, non_negative: bool = True):
    curr = config
    for part in path.split("."):
        if not isinstance(curr, dict) or part not in curr:
            raise ValidationError(f"Missing required numeric field: {path}")
        curr = curr[part]
    if not isinstance(curr, (int, float)):
        raise ValidationError(f"{path} must be a number")
    if non_negative and curr < 0:
        raise ValidationError(f"{path} cannot be negative")
    return curr


def require_bool(config: dict, path: str):
    curr = config
    for part in path.split("."):
        if not isinstance(curr, dict) or part not in curr:
            raise ValidationError(f"Missing required boolean field: {path}")
        curr = curr[part]
    if not isinstance(curr, bool):
        raise ValidationError(f"{path} must be a boolean")
    return curr
```

- [ ] **Step 2: Validate required fields**

In `load_and_validate_config()`, after root checks, validate:

```python
require_number(config, "tick_count")
if config["tick_count"] <= 0:
    raise ValidationError("tick_count must be positive")
require_number(config, "space.spatial_grid_size")
if config["space"]["spatial_grid_size"] <= 0:
    raise ValidationError("space.spatial_grid_size must be positive")
require_number(config, "resources.passive_energy_income_placeholder")
require_number(config, "environment.heat_current")
require_number(config, "environment.heat_generated_per_tick")
require_number(config, "environment.heat_dissipation_rate")
require_number(config, "environment.heat_warning_threshold")
require_number(config, "environment.heat_death_threshold")
require_number(config, "environment.waste_current")
require_number(config, "environment.waste_generated_per_tick")
require_number(config, "environment.waste_sink_rate")
require_number(config, "environment.waste_warning_threshold")
require_number(config, "environment.waste_death_threshold")
require_number(config, "lifecycle.stress_energy_threshold")
require_number(config, "lifecycle.critical_capacity_overrun")
require_bool(config, "lifecycle.dormancy_allowed")
```

Validate boundary mode:

```python
if config["world"].get("boundary_mode") not in {"solid_wall", "wrapped", "open"}:
    raise ValidationError("world.boundary_mode must be solid_wall, wrapped, or open")
```

Validate estimates if present:

```python
for path in [
    "estimates.growth_cost_estimate",
    "estimates.division_cost_estimate",
    "estimates.resource_regeneration_or_inflow",
    "estimates.population_space_limit",
    "estimates.joint_count_estimate",
    "estimates.joint_upkeep_cost",
]:
    if "estimates" in config:
        require_number(config, path)
```

- [ ] **Step 3: Add tests**

Add parametrized tests:

```python
@pytest.mark.parametrize("path", [
    "tick_count",
    "space.spatial_grid_size",
    "resources.passive_energy_income_placeholder",
    "environment.heat_generated_per_tick",
    "environment.waste_sink_rate",
    "lifecycle.stress_energy_threshold",
    "lifecycle.critical_capacity_overrun",
])
def test_negative_required_numeric_fields_rejected(path):
    toml_str = mutate_valid_toml_number(VALID_TOML, path, -1.0)
    with pytest.raises(ValidationError):
        load_and_validate_config(toml_str)
```

If no helper exists yet, implement `mutate_valid_toml_number()` in Task 8 test helper and use it here.

- [ ] **Step 4: Run config loader tests**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_config_loader.py -q
```

Expected: PASS.

---

### Task 6: Clean README encoding and add CLI install entrypoint

**Files:**
- Modify: `tools/early-stability/README.md`
- Modify: `tools/early-stability/pyproject.toml`

- [ ] **Step 1: Replace mojibake tree with ASCII**

In README, replace the directory structure block with:

```text
tools/early-stability/
  src/
    cli.py                  # Entrypoint, subcommands, and batch runner
    config_loader.py        # Validates config format, types, and capacity limits
    micro_simulator.py      # Runs tick loops tracking energy, heat, and waste
    report_writer.py        # Generates REPORT.md with sensitivity rankings
    result_writer.py        # Writes JSON outputs
    static_calculator.py    # Performs static bounds evaluations
    tuner.py                # Deterministic grid-search tuning engine
  scenarios/
  tuning/
  tests/
  pyproject.toml
  README.md
```

- [ ] **Step 2: Add console script**

Add to `tools/early-stability/pyproject.toml`:

```toml
[project.scripts]
early-stability = "cli:main"
```

- [ ] **Step 3: Update README commands**

Add:

```powershell
python -m pip install -e .\tools\early-stability[dev]
early-stability evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\manual_evaluate
```

Keep direct script examples as fallback.

- [ ] **Step 4: Verify README no mojibake**

Run:

```powershell
rg "в”" .\tools\early-stability\README.md
```

Expected: no output.

---

### Task 7: Decide and fix tracked outputs policy

**Files:**
- Modify: `.gitignore`
- Possibly untrack or explicitly allow `outputs/worklogs/*.md`

- [ ] **Step 1: Choose policy**

Use this project policy:

```text
Track worklogs because they are decision history.
Ignore generated stability artifacts.
```

- [ ] **Step 2: Update `.gitignore`**

Replace broad `outputs/` rule with:

```gitignore
outputs/*
!outputs/worklogs/
!outputs/worklogs/*.md
outputs/stability/
__pycache__/
.pytest_cache/
.venv/
*.pyc
.idea/
```

- [ ] **Step 3: Verify tracked outputs**

Run:

```powershell
git ls-files outputs
git status --short
```

Expected:

- worklogs may remain tracked;
- generated `outputs/stability/` must not appear;
- `.idea/` must not appear after `.gitignore` update.

---

### Task 8: Refactor duplicated test TOML helpers

**Files:**
- Create: `tools/early-stability/tests/helpers.py`
- Modify: `tools/early-stability/tests/test_config_loader.py`

- [ ] **Step 1: Create helper**

Create `tools/early-stability/tests/helpers.py`:

```python
import tomllib


def serialize_value(value):
    if isinstance(value, bool):
        return str(value).lower()
    if isinstance(value, str):
        return f'"{value}"'
    if isinstance(value, list):
        return "[" + ", ".join(serialize_value(v) for v in value) + "]"
    if isinstance(value, dict):
        parts = [f"{k} = {serialize_value(v)}" for k, v in value.items()]
        return "{ " + ", ".join(parts) + " }"
    return str(value)


def dict_to_toml(data: dict) -> str:
    lines = []
    for key, value in data.items():
        if not isinstance(value, dict):
            lines.append(f"{key} = {serialize_value(value)}")
    for key, value in data.items():
        if isinstance(value, dict):
            lines.append("")
            lines.append(f"[{key}]")
            for sub_key, sub_value in value.items():
                lines.append(f"{sub_key} = {serialize_value(sub_value)}")
    return "\n".join(lines) + "\n"


def mutate_toml(base_toml: str, path: str, value):
    data = tomllib.loads(base_toml)
    parts = path.split(".")
    curr = data
    for part in parts[:-1]:
        curr = curr[part]
    curr[parts[-1]] = value
    return dict_to_toml(data)
```

- [ ] **Step 2: Replace duplicated local serializers**

In `test_config_loader.py`, import:

```python
from helpers import mutate_toml
```

Replace repeated `dict_to_toml()` local functions with:

```python
toml_str = mutate_toml(VALID_TOML, "cell.capacity_limit", 10.0)
```

For multi-field changes, parse once, mutate dict, then use `dict_to_toml` from helper.

- [ ] **Step 3: Run config tests**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_config_loader.py -q
```

Expected: PASS.

---

### Task 9: Full verification and report

**Files:**
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-early-stability-tool-fixes.md`

- [ ] **Step 1: Run full tests**

Run:

```powershell
python -m pytest .\tools\early-stability
```

Expected: all tests pass.

- [ ] **Step 2: Run CLI smoke commands**

Run:

```powershell
python .\tools\early-stability\src\cli.py evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\review_evaluate
python .\tools\early-stability\src\cli.py evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\review_evaluate_sim --with-simulation
python .\tools\early-stability\src\cli.py simulate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --ticks 20 --out .\outputs\stability\review_simulate
python .\tools\early-stability\src\cli.py tune --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --tuning .\tools\early-stability\tuning\single_cell.toml --out .\outputs\stability\review_tune
python .\tools\early-stability\src\cli.py batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\review_batch --with-simulation
```

Expected files:

```text
outputs/stability/review_tune/results.json
outputs/stability/review_tune/ranges.json
outputs/stability/review_tune/REPORT.md
outputs/stability/review_tune/runs/run_0001.json
outputs/stability/review_tune/recommended-configs/best_stable.toml
```

- [ ] **Step 3: Inspect artifacts**

Run:

```powershell
Select-String -Path .\outputs\stability\review_tune\REPORT.md -Pattern "Recommended Values Table","Empirical Tested","Parameter Sensitivity","Failure Reasons"
Get-Content -Raw .\outputs\stability\review_tune\runs\run_0001.json
```

Expected:

- report contains all searched sections;
- run json contains `history`, `final_energy`, `final_heat`, `final_waste`, `final_state`.

- [ ] **Step 4: Check git hygiene**

Run:

```powershell
git status --short
git ls-files outputs
```

Expected:

- no generated `outputs/stability/` tracked;
- `.idea/` not shown;
- worklogs policy is explicit through `.gitignore`.

- [ ] **Step 5: Create final report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-early-stability-tool-fixes.md` with:

```markdown
# REPORT: Early Stability Tool Fixes

Date: YYYY-MM-DD HH:MM

## Goal
Fix review findings for early stability tool.

## Files Changed
- ...

## Verification
- `python -m pytest .\tools\early-stability`: ...
- CLI smoke commands: ...

## Remaining Risks
- ...
```

---

## Acceptance Criteria

Implementation is acceptable when:

- tune report includes ranges, recommended values, sensitivity and failure reasons;
- tune `runs/*.json` includes history and final metrics;
- `evaluate --with-simulation` and `batch --with-simulation` work;
- invalid tuning configs fail loudly;
- config validation covers required Phase 1 numeric fields;
- README has no mojibake and documents install/CLI usage;
- `.gitignore` clearly tracks worklogs but ignores generated stability outputs;
- `python -m pytest .\tools\early-stability` passes;
- generated stability artifacts are not committed.
