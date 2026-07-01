# Mechanism Reachability Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first `mechanism-reachability` tool mode that reports whether configured mechanisms are reachable, useful, blocked, bypassed, or tool-limited after stability parameter ranges have been calibrated.

**Architecture:** Implement this as an offline helper inside `tools/early-stability/`, not inside Rust `alife-core`. The first version consumes TOML scenario/config plus a mechanism registry TOML, runs deterministic observer-side analysis over the current micro simulator model, and writes JSON/Markdown artifacts under `outputs/reachability/<run_id>/`. If reachability fails, the report must point back to the relevant `early-stability-parameter-tuning` group.

**Tech Stack:** Python 3.11, stdlib `argparse/json/tomllib/os/hashlib`, pytest, existing `tools/early-stability` modules, TOML registry/scenarios, Markdown reports.

---

## Context

Read before implementation:

- `docs/implementation/mechanism-reachability.md`
- `docs/implementation/early-stability-parameter-tuning.md`
- `docs/implementation/early-stability-tool.md`
- `docs/config/stability_bounds.md`
- `tools/early-stability/src/micro_simulator.py`
- `tools/early-stability/src/config_loader.py`
- `tools/early-stability/src/result_writer.py`
- `tools/early-stability/src/report_writer.py`

Do not change Canon to make reachability pass.

Do not make cells, Genome Runtime, Feasibility or Processes read reachability counters. They are observer-only.

---

## File Structure

Create:

- `tools/early-stability/src/reachability.py`
  - Loads mechanism registry.
  - Evaluates mechanism counters from scenario/config and micro simulator history.
  - Detects block reasons and bypass risks.
  - Produces machine-readable reachability result objects.

- `tools/early-stability/src/reachability_writer.py`
  - Writes `results.json`, `mechanisms.json`, `block-reasons.json`, `bypass.json`, `REPORT.md`.

- `tools/early-stability/mechanisms/phase1.toml`
  - First mechanism registry for Phase 1 and estimate-only mechanisms.

- `tools/early-stability/tests/test_reachability.py`
  - Unit tests for reachability classification.

- `tools/early-stability/tests/test_reachability_cli.py`
  - CLI tests for `reachability` command and artifact output.

Modify:

- `tools/early-stability/src/cli.py`
  - Add `reachability` subcommand.

- `tools/early-stability/README.md`
  - Document `reachability` command and output artifacts.

- `docs/implementation/early-stability-tool.md`
  - Add reachability mode as second-stage helper.

Do not create generated `outputs/reachability/` artifacts in committed source unless explicitly requested.

---

## Data Contracts

Mechanism registry TOML:

```toml
[[mechanisms]]
mechanism_id = "mandatory_energy_cost"
status = "now"
required_inputs = ["cell.mandatory_cost_per_tick", "cell.initial_energy"]
expected_effect = "energy decreases when mandatory cost is paid"
possible_blockers = ["insufficient_energy", "invalid_config"]
bypass_risks = ["passive_energy_income_too_high"]
minimum_useful_activation_rate = 1.0
required_scenarios = ["single_cell_survival", "single_cell_starvation"]
```

Mechanism result dict:

```python
{
    "mechanism_id": "mandatory_energy_cost",
    "status": "now",
    "scenario_id": "single_cell_survival",
    "available_count": 100,
    "attempted_count": 100,
    "allowed_count": 100,
    "executed_count": 100,
    "blocked_count": 0,
    "effect_nonzero_count": 100,
    "bypass_detected_count": 0,
    "top_block_reason": "none",
    "reachability_result": "pass",
    "notes": "Mandatory cost was paid every tick."
}
```

Reachability result values:

```text
pass
warning
fail
blocked
tool_limited
future_only
```

---

### Task 1: Add Mechanism Registry Loader

**Files:**
- Create: `tools/early-stability/src/reachability.py`
- Create: `tools/early-stability/tests/test_reachability.py`

- [ ] **Step 1: Write failing tests for registry loading**

Add to `tools/early-stability/tests/test_reachability.py`:

```python
import pytest

from reachability import ReachabilityValidationError, load_mechanism_registry


VALID_REGISTRY = """
[[mechanisms]]
mechanism_id = "mandatory_energy_cost"
status = "now"
required_inputs = ["cell.mandatory_cost_per_tick", "cell.initial_energy"]
expected_effect = "energy decreases when mandatory cost is paid"
possible_blockers = ["insufficient_energy", "invalid_config"]
bypass_risks = ["passive_energy_income_too_high"]
minimum_useful_activation_rate = 1.0
required_scenarios = ["single_cell_survival", "single_cell_starvation"]
"""


def test_load_mechanism_registry_returns_mechanisms():
    mechanisms = load_mechanism_registry(VALID_REGISTRY)

    assert len(mechanisms) == 1
    assert mechanisms[0]["mechanism_id"] == "mandatory_energy_cost"
    assert mechanisms[0]["status"] == "now"
    assert mechanisms[0]["minimum_useful_activation_rate"] == 1.0


def test_registry_rejects_missing_mechanism_id():
    bad_registry = """
[[mechanisms]]
status = "now"
required_inputs = []
expected_effect = "missing id"
possible_blockers = []
bypass_risks = []
minimum_useful_activation_rate = 1.0
required_scenarios = []
"""
    with pytest.raises(ReachabilityValidationError):
        load_mechanism_registry(bad_registry)
```

- [ ] **Step 2: Run tests and verify RED**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability.py -q
```

Expected:

```text
ModuleNotFoundError: No module named 'reachability'
```

- [ ] **Step 3: Implement registry loader**

Create `tools/early-stability/src/reachability.py`:

```python
import tomllib


class ReachabilityValidationError(Exception):
    pass


REQUIRED_MECHANISM_FIELDS = [
    "mechanism_id",
    "status",
    "required_inputs",
    "expected_effect",
    "possible_blockers",
    "bypass_risks",
    "minimum_useful_activation_rate",
    "required_scenarios",
]


VALID_STATUSES = {"now", "future", "estimate_only"}


def load_mechanism_registry(toml_str: str) -> list[dict]:
    try:
        data = tomllib.loads(toml_str)
    except tomllib.TOMLDecodeError as exc:
        raise ReachabilityValidationError(f"Invalid mechanism registry TOML: {exc}")

    mechanisms = data.get("mechanisms")
    if not isinstance(mechanisms, list) or not mechanisms:
        raise ReachabilityValidationError("registry must contain at least one [[mechanisms]] entry")

    seen_ids = set()
    for mechanism in mechanisms:
        if not isinstance(mechanism, dict):
            raise ReachabilityValidationError("mechanism entry must be a table")
        for field in REQUIRED_MECHANISM_FIELDS:
            if field not in mechanism:
                raise ReachabilityValidationError(f"mechanism missing required field: {field}")
        mechanism_id = mechanism["mechanism_id"]
        if not isinstance(mechanism_id, str) or not mechanism_id:
            raise ReachabilityValidationError("mechanism_id must be a non-empty string")
        if mechanism_id in seen_ids:
            raise ReachabilityValidationError(f"duplicate mechanism_id: {mechanism_id}")
        seen_ids.add(mechanism_id)
        if mechanism["status"] not in VALID_STATUSES:
            raise ReachabilityValidationError(f"invalid mechanism status: {mechanism['status']}")
        for list_field in ["required_inputs", "possible_blockers", "bypass_risks", "required_scenarios"]:
            if not isinstance(mechanism[list_field], list):
                raise ReachabilityValidationError(f"{mechanism_id}.{list_field} must be a list")
        rate = mechanism["minimum_useful_activation_rate"]
        if not isinstance(rate, (int, float)) or isinstance(rate, bool) or rate < 0.0 or rate > 1.0:
            raise ReachabilityValidationError(f"{mechanism_id}.minimum_useful_activation_rate must be 0.0..1.0")

    return mechanisms
```

- [ ] **Step 4: Run tests and verify GREEN**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability.py -q
```

Expected:

```text
2 passed
```

---

### Task 2: Add Phase 1 Mechanism Registry

**Files:**
- Create: `tools/early-stability/mechanisms/phase1.toml`
- Modify: `tools/early-stability/tests/test_reachability.py`

- [ ] **Step 1: Write failing test that bundled registry loads**

Add to `tools/early-stability/tests/test_reachability.py`:

```python
from pathlib import Path


def test_bundled_phase1_registry_loads():
    registry_path = Path(__file__).resolve().parents[1] / "mechanisms" / "phase1.toml"
    mechanisms = load_mechanism_registry(registry_path.read_text(encoding="utf-8"))

    ids = {m["mechanism_id"] for m in mechanisms}
    assert "mandatory_energy_cost" in ids
    assert "heat_dissipation" in ids
    assert "capacity_limit" in ids
```

- [ ] **Step 2: Run test and verify RED**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability.py::test_bundled_phase1_registry_loads -q
```

Expected:

```text
FileNotFoundError
```

- [ ] **Step 3: Create registry file**

Create `tools/early-stability/mechanisms/phase1.toml`:

```toml
[[mechanisms]]
mechanism_id = "mandatory_energy_cost"
status = "now"
required_inputs = ["cell.mandatory_cost_per_tick", "cell.initial_energy"]
expected_effect = "energy decreases when mandatory cost is paid"
possible_blockers = ["insufficient_energy", "mandatory_cost_unpaid", "invalid_config"]
bypass_risks = ["passive_energy_income_too_high"]
minimum_useful_activation_rate = 1.0
required_scenarios = ["single_cell_survival", "single_cell_starvation"]

[[mechanisms]]
mechanism_id = "passive_energy_income"
status = "now"
required_inputs = ["resources.passive_energy_income_placeholder"]
expected_effect = "energy can increase before mandatory cost"
possible_blockers = ["not_configured", "effect_zero"]
bypass_risks = ["bypasses_resource_uptake", "bypasses_metabolism_pressure"]
minimum_useful_activation_rate = 0.05
required_scenarios = ["single_cell_survival"]

[[mechanisms]]
mechanism_id = "capacity_limit"
status = "now"
required_inputs = ["cell.capacity_limit", "cell.initial_resources", "cell.initial_materials"]
expected_effect = "invalid or stressed states appear when stored amount exceeds capacity"
possible_blockers = ["invalid_config", "insufficient_capacity"]
bypass_risks = ["capacity_too_high"]
minimum_useful_activation_rate = 0.05
required_scenarios = ["single_cell_survival", "single_cell_over_capacity"]

[[mechanisms]]
mechanism_id = "heat_generation"
status = "now"
required_inputs = ["environment.heat_generated_per_tick"]
expected_effect = "heat increases when generation exceeds dissipation"
possible_blockers = ["not_configured", "effect_zero"]
bypass_risks = ["heat_generated_too_low"]
minimum_useful_activation_rate = 0.05
required_scenarios = ["single_cell_survival", "single_cell_heat_stress"]

[[mechanisms]]
mechanism_id = "heat_dissipation"
status = "now"
required_inputs = ["environment.heat_dissipation_rate", "environment.heat_generated_per_tick"]
expected_effect = "heat decreases or remains bounded"
possible_blockers = ["not_configured", "effect_zero"]
bypass_risks = ["dissipation_too_high_bypasses_heat_pressure"]
minimum_useful_activation_rate = 0.05
required_scenarios = ["single_cell_survival", "single_cell_heat_stress"]

[[mechanisms]]
mechanism_id = "waste_generation"
status = "now"
required_inputs = ["environment.waste_generated_per_tick"]
expected_effect = "waste increases when generation exceeds sink"
possible_blockers = ["not_configured", "effect_zero"]
bypass_risks = ["waste_generated_too_low"]
minimum_useful_activation_rate = 0.05
required_scenarios = ["single_cell_survival", "waste_heat_balance"]

[[mechanisms]]
mechanism_id = "waste_sink"
status = "now"
required_inputs = ["environment.waste_sink_rate", "environment.waste_generated_per_tick"]
expected_effect = "waste decreases or remains bounded"
possible_blockers = ["not_configured", "effect_zero"]
bypass_risks = ["sink_too_high_bypasses_waste_pressure"]
minimum_useful_activation_rate = 0.05
required_scenarios = ["single_cell_survival", "waste_heat_balance"]

[[mechanisms]]
mechanism_id = "growth_estimate"
status = "estimate_only"
required_inputs = ["estimates.growth_cost_estimate"]
expected_effect = "tool-only growth budget estimate can be analyzed"
possible_blockers = ["tool_limited", "future_only"]
bypass_risks = ["not_consumed_by_micro_simulator"]
minimum_useful_activation_rate = 0.0
required_scenarios = ["single_cell_growth_budget"]

[[mechanisms]]
mechanism_id = "joint_upkeep_estimate"
status = "estimate_only"
required_inputs = ["estimates.joint_count_estimate", "estimates.joint_upkeep_cost"]
expected_effect = "tool-only joint upkeep budget estimate can be analyzed"
possible_blockers = ["tool_limited", "future_only"]
bypass_risks = ["not_consumed_by_micro_simulator"]
minimum_useful_activation_rate = 0.0
required_scenarios = ["joint_upkeep_budget"]
```

- [ ] **Step 4: Run test and verify GREEN**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability.py::test_bundled_phase1_registry_loads -q
```

Expected:

```text
1 passed
```

---

### Task 3: Implement Mechanism Evaluation For Current Micro Simulator

**Files:**
- Modify: `tools/early-stability/src/reachability.py`
- Modify: `tools/early-stability/tests/test_reachability.py`

- [ ] **Step 1: Write failing tests for pass, bypass and tool-limited results**

Add to `tools/early-stability/tests/test_reachability.py`:

```python
from reachability import evaluate_mechanisms


def sample_config():
    return {
        "scenario_id": "single_cell_survival",
        "tick_count": 3,
        "resources": {"passive_energy_income_placeholder": 5.0},
        "cell": {
            "initial_energy": 20.0,
            "mandatory_cost_per_tick": 2.0,
            "capacity_limit": 50.0,
            "initial_resources": {"water": 2.0, "nutrient": 1.0},
            "initial_materials": {"cell_wall": 5.0},
        },
        "environment": {
            "heat_generated_per_tick": 0.1,
            "heat_dissipation_rate": 0.2,
            "waste_generated_per_tick": 0.05,
            "waste_sink_rate": 0.1,
        },
        "estimates": {
            "growth_cost_estimate": 10.0,
            "joint_count_estimate": 0,
            "joint_upkeep_cost": 0.0,
        },
    }


def test_evaluate_mandatory_energy_cost_passes_when_energy_changes():
    mechanisms = load_mechanism_registry(VALID_REGISTRY)
    history = [
        {"tick": 1, "state": "alive", "energy": 23.0, "heat": 0.0, "waste": 0.0},
        {"tick": 2, "state": "alive", "energy": 26.0, "heat": 0.0, "waste": 0.0},
        {"tick": 3, "state": "alive", "energy": 29.0, "heat": 0.0, "waste": 0.0},
    ]

    results = evaluate_mechanisms(sample_config(), mechanisms, history, "stable", "none")

    mandatory = results[0]
    assert mandatory["mechanism_id"] == "mandatory_energy_cost"
    assert mandatory["reachability_result"] == "pass"
    assert mandatory["executed_count"] == 3


def test_estimate_only_mechanism_is_tool_limited():
    registry = """
[[mechanisms]]
mechanism_id = "growth_estimate"
status = "estimate_only"
required_inputs = ["estimates.growth_cost_estimate"]
expected_effect = "estimate only"
possible_blockers = ["tool_limited"]
bypass_risks = ["not_consumed_by_micro_simulator"]
minimum_useful_activation_rate = 0.0
required_scenarios = ["single_cell_growth_budget"]
"""
    mechanisms = load_mechanism_registry(registry)

    results = evaluate_mechanisms(sample_config(), mechanisms, [], "stable", "none")

    assert results[0]["reachability_result"] == "tool_limited"
    assert results[0]["top_block_reason"] == "tool_limited"
```

- [ ] **Step 2: Run tests and verify RED**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability.py -q
```

Expected:

```text
ImportError: cannot import name 'evaluate_mechanisms'
```

- [ ] **Step 3: Implement evaluation helpers**

Add to `tools/early-stability/src/reachability.py`:

```python
def get_nested(config: dict, path: str, default=None):
    curr = config
    for part in path.split("."):
        if not isinstance(curr, dict) or part not in curr:
            return default
        curr = curr[part]
    return curr


def has_required_inputs(config: dict, mechanism: dict) -> bool:
    return all(get_nested(config, path, None) is not None for path in mechanism["required_inputs"])


def base_result(mechanism: dict, config: dict, history: list, scenario_result: str, collapse_reason: str) -> dict:
    available = 1 if has_required_inputs(config, mechanism) else 0
    return {
        "mechanism_id": mechanism["mechanism_id"],
        "status": mechanism["status"],
        "scenario_id": config.get("scenario_id", "unknown"),
        "available_count": available,
        "attempted_count": 0,
        "allowed_count": 0,
        "executed_count": 0,
        "blocked_count": 0,
        "effect_nonzero_count": 0,
        "bypass_detected_count": 0,
        "top_block_reason": "none",
        "reachability_result": "fail" if available else "blocked",
        "notes": "",
    }


def evaluate_mandatory_energy_cost(result: dict, config: dict, history: list, scenario_result: str, collapse_reason: str):
    ticks = len(history)
    result["available_count"] = ticks
    result["attempted_count"] = ticks
    if collapse_reason in {"mandatory_cost_unpaid", "energy_depleted"}:
        result["blocked_count"] = 1
        result["top_block_reason"] = collapse_reason
        result["reachability_result"] = "blocked"
        result["notes"] = "Mandatory cost pressure is reachable and can block survival."
        return result
    result["allowed_count"] = ticks
    result["executed_count"] = ticks
    result["effect_nonzero_count"] = ticks
    result["reachability_result"] = "pass" if ticks > 0 else "fail"
    result["notes"] = "Mandatory cost was evaluated over simulation history."
    return result


def evaluate_passive_energy_income(result: dict, config: dict, history: list):
    income = get_nested(config, "resources.passive_energy_income_placeholder", 0.0)
    result["available_count"] = 1
    if income <= 0:
        result["blocked_count"] = 1
        result["top_block_reason"] = "not_configured"
        result["reachability_result"] = "blocked"
        result["notes"] = "Passive energy income is zero."
        return result
    result["attempted_count"] = len(history)
    result["allowed_count"] = len(history)
    result["executed_count"] = len(history)
    result["effect_nonzero_count"] = len(history)
    mandatory = get_nested(config, "cell.mandatory_cost_per_tick", 0.0)
    if income > mandatory:
        result["bypass_detected_count"] = 1
        result["reachability_result"] = "warning"
        result["top_block_reason"] = "competing_path_cheaper"
        result["notes"] = "Passive income may bypass future uptake/metabolism pressure."
    else:
        result["reachability_result"] = "pass"
        result["notes"] = "Passive income exists but does not fully dominate mandatory cost."
    return result


def evaluate_capacity_limit(result: dict, config: dict):
    resources = get_nested(config, "cell.initial_resources", {})
    materials = get_nested(config, "cell.initial_materials", {})
    capacity = get_nested(config, "cell.capacity_limit", 0.0)
    used = sum(resources.values()) + sum(materials.values())
    result["available_count"] = 1
    result["attempted_count"] = 1
    if used > capacity:
        result["blocked_count"] = 1
        result["top_block_reason"] = "insufficient_capacity"
        result["reachability_result"] = "blocked"
        result["notes"] = "Initial stored amount exceeds capacity."
    elif capacity > used * 5:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["bypass_detected_count"] = 1
        result["top_block_reason"] = "capacity_too_high"
        result["reachability_result"] = "warning"
        result["notes"] = "Capacity is high enough to bypass storage pressure."
    else:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["effect_nonzero_count"] = 1
        result["reachability_result"] = "pass"
        result["notes"] = "Capacity limit is configured and near enough to matter."
    return result


def evaluate_heat_or_waste(result: dict, config: dict, generated_path: str, sink_path: str, label: str):
    generated = get_nested(config, generated_path, 0.0)
    sink = get_nested(config, sink_path, 0.0)
    result["available_count"] = 1
    result["attempted_count"] = 1
    if generated <= 0:
        result["blocked_count"] = 1
        result["top_block_reason"] = "effect_zero"
        result["reachability_result"] = "blocked"
        result["notes"] = f"{label} generation is zero."
    elif sink > generated * 3:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["bypass_detected_count"] = 1
        result["top_block_reason"] = "competing_path_cheaper"
        result["reachability_result"] = "warning"
        result["notes"] = f"{label} sink may be high enough to bypass pressure."
    else:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["effect_nonzero_count"] = 1
        result["reachability_result"] = "pass"
        result["notes"] = f"{label} generation/sink pressure is reachable."
    return result


def evaluate_mechanisms(config: dict, mechanisms: list[dict], history: list, scenario_result: str, collapse_reason: str) -> list[dict]:
    results = []
    for mechanism in mechanisms:
        result = base_result(mechanism, config, history, scenario_result, collapse_reason)
        mechanism_id = mechanism["mechanism_id"]

        if mechanism["status"] == "future":
            result["reachability_result"] = "future_only"
            result["top_block_reason"] = "future_only"
            result["notes"] = "Mechanism is declared future-only."
        elif mechanism["status"] == "estimate_only":
            result["reachability_result"] = "tool_limited"
            result["top_block_reason"] = "tool_limited"
            result["notes"] = "Estimate-only mechanism is not consumed by current micro simulator."
        elif mechanism_id == "mandatory_energy_cost":
            result = evaluate_mandatory_energy_cost(result, config, history, scenario_result, collapse_reason)
        elif mechanism_id == "passive_energy_income":
            result = evaluate_passive_energy_income(result, config, history)
        elif mechanism_id == "capacity_limit":
            result = evaluate_capacity_limit(result, config)
        elif mechanism_id in {"heat_generation", "heat_dissipation"}:
            result = evaluate_heat_or_waste(
                result,
                config,
                "environment.heat_generated_per_tick",
                "environment.heat_dissipation_rate",
                "heat",
            )
        elif mechanism_id in {"waste_generation", "waste_sink"}:
            result = evaluate_heat_or_waste(
                result,
                config,
                "environment.waste_generated_per_tick",
                "environment.waste_sink_rate",
                "waste",
            )
        else:
            result["reachability_result"] = "tool_limited"
            result["top_block_reason"] = "tool_limited"
            result["notes"] = "No evaluator exists for this mechanism yet."

        results.append(result)

    return results
```

- [ ] **Step 4: Run tests and verify GREEN**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability.py -q
```

Expected:

```text
5 passed
```

---

### Task 4: Add Reachability Writers

**Files:**
- Create: `tools/early-stability/src/reachability_writer.py`
- Create: `tools/early-stability/tests/test_reachability_writer.py`

- [ ] **Step 1: Write failing writer tests**

Create `tools/early-stability/tests/test_reachability_writer.py`:

```python
import json

from reachability_writer import summarize_reachability, write_reachability_outputs


def sample_results():
    return [
        {
            "mechanism_id": "mandatory_energy_cost",
            "status": "now",
            "scenario_id": "single_cell_survival",
            "available_count": 100,
            "attempted_count": 100,
            "allowed_count": 100,
            "executed_count": 100,
            "blocked_count": 0,
            "effect_nonzero_count": 100,
            "bypass_detected_count": 0,
            "top_block_reason": "none",
            "reachability_result": "pass",
            "notes": "ok",
        },
        {
            "mechanism_id": "growth_estimate",
            "status": "estimate_only",
            "scenario_id": "single_cell_growth_budget",
            "available_count": 1,
            "attempted_count": 0,
            "allowed_count": 0,
            "executed_count": 0,
            "blocked_count": 0,
            "effect_nonzero_count": 0,
            "bypass_detected_count": 0,
            "top_block_reason": "tool_limited",
            "reachability_result": "tool_limited",
            "notes": "estimate only",
        },
    ]


def test_summarize_reachability_counts_results():
    summary = summarize_reachability(sample_results())

    assert summary["mechanism_count"] == 2
    assert summary["passed_count"] == 1
    assert summary["tool_limited_count"] == 1
    assert summary["overall_result"] == "partial"


def test_write_reachability_outputs(tmp_path):
    write_reachability_outputs(
        str(tmp_path),
        "run-1",
        "hash-1",
        42,
        100,
        "outputs/stability/example",
        sample_results(),
    )

    assert (tmp_path / "results.json").exists()
    assert (tmp_path / "mechanisms.json").exists()
    assert (tmp_path / "block-reasons.json").exists()
    assert (tmp_path / "bypass.json").exists()
    assert (tmp_path / "REPORT.md").exists()

    results = json.loads((tmp_path / "results.json").read_text(encoding="utf-8"))
    assert results["overall_result"] == "partial"
    assert results["tool_limited_count"] == 1
```

- [ ] **Step 2: Run tests and verify RED**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability_writer.py -q
```

Expected:

```text
ModuleNotFoundError: No module named 'reachability_writer'
```

- [ ] **Step 3: Implement writer**

Create `tools/early-stability/src/reachability_writer.py`:

```python
import json
import os
from collections import Counter


def summarize_reachability(mechanism_results: list[dict]) -> dict:
    counts = Counter(r["reachability_result"] for r in mechanism_results)
    failed = counts["fail"] + counts["blocked"]
    warnings = counts["warning"]
    tool_limited = counts["tool_limited"] + counts["future_only"]
    if failed > 0:
        overall = "fail"
    elif tool_limited > 0:
        overall = "partial"
    elif warnings > 0:
        overall = "warning"
    else:
        overall = "pass"
    return {
        "overall_result": overall,
        "mechanism_count": len(mechanism_results),
        "passed_count": counts["pass"],
        "warning_count": counts["warning"],
        "failed_count": counts["fail"],
        "blocked_count": counts["blocked"],
        "tool_limited_count": counts["tool_limited"] + counts["future_only"],
    }


def write_json(path: str, data):
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)


def write_reachability_outputs(
    output_dir: str,
    run_id: str,
    config_hash: str,
    seed: int,
    tick_count: int,
    stability_ranges_ref: str,
    mechanism_results: list[dict],
):
    os.makedirs(output_dir, exist_ok=True)
    summary = summarize_reachability(mechanism_results)
    results = {
        "run_id": run_id,
        "config_hash": config_hash,
        "seed": seed,
        "tick_count": tick_count,
        "stability_ranges_ref": stability_ranges_ref,
        **summary,
    }
    write_json(os.path.join(output_dir, "results.json"), results)
    write_json(os.path.join(output_dir, "mechanisms.json"), mechanism_results)

    block_reasons = Counter(r["top_block_reason"] for r in mechanism_results if r["top_block_reason"] != "none")
    write_json(os.path.join(output_dir, "block-reasons.json"), dict(sorted(block_reasons.items())))

    bypass = [r for r in mechanism_results if r["bypass_detected_count"] > 0]
    write_json(os.path.join(output_dir, "bypass.json"), bypass)

    lines = [
        f"# Mechanism Reachability Report: {run_id}",
        "",
        "## Summary",
        f"* **Overall Result**: {summary['overall_result']}",
        f"* **Mechanisms**: {summary['mechanism_count']}",
        f"* **Pass**: {summary['passed_count']}",
        f"* **Warning**: {summary['warning_count']}",
        f"* **Fail**: {summary['failed_count']}",
        f"* **Blocked**: {summary['blocked_count']}",
        f"* **Tool Limited**: {summary['tool_limited_count']}",
        f"* **Stability Ranges Ref**: {stability_ranges_ref}",
        "",
        "## Mechanisms",
        "| Mechanism | Result | Block Reason | Executed | Effect Nonzero | Bypass | Notes |",
        "| --- | --- | --- | ---: | ---: | ---: | --- |",
    ]
    for r in mechanism_results:
        lines.append(
            f"| {r['mechanism_id']} | {r['reachability_result']} | {r['top_block_reason']} | "
            f"{r['executed_count']} | {r['effect_nonzero_count']} | {r['bypass_detected_count']} | {r['notes']} |"
        )
    lines.extend([
        "",
        "## Decision",
        f"Proceed to data model docs: {'yes' if summary['overall_result'] == 'pass' else 'partial'}",
    ])
    with open(os.path.join(output_dir, "REPORT.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
```

- [ ] **Step 4: Run tests and verify GREEN**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability_writer.py -q
```

Expected:

```text
2 passed
```

---

### Task 5: Add `reachability` CLI Command

**Files:**
- Modify: `tools/early-stability/src/cli.py`
- Create: `tools/early-stability/tests/test_reachability_cli.py`

- [ ] **Step 1: Write failing CLI test**

Create `tools/early-stability/tests/test_reachability_cli.py`:

```python
import json
import os
from pathlib import Path

from cli import main
from test_cli import VALID_SCENARIO_TOML
from test_reachability import VALID_REGISTRY


def test_cli_reachability_writes_artifacts(tmp_path):
    scenario = tmp_path / "scenario.toml"
    scenario.write_text(VALID_SCENARIO_TOML, encoding="utf-8")
    registry = tmp_path / "mechanisms.toml"
    registry.write_text(VALID_REGISTRY, encoding="utf-8")
    out_dir = tmp_path / "reachability"

    main([
        "reachability",
        "--scenario", str(scenario),
        "--mechanisms", str(registry),
        "--stability-ranges-ref", "outputs/stability/test",
        "--out", str(out_dir),
    ])

    assert (out_dir / "results.json").exists()
    assert (out_dir / "mechanisms.json").exists()
    assert (out_dir / "REPORT.md").exists()

    results = json.loads((out_dir / "results.json").read_text(encoding="utf-8"))
    assert results["stability_ranges_ref"] == "outputs/stability/test"
    assert results["mechanism_count"] == 1
```

- [ ] **Step 2: Run test and verify RED**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability_cli.py -q
```

Expected:

```text
argparse exits because reachability command is unknown
```

- [ ] **Step 3: Implement CLI mode**

Modify `tools/early-stability/src/cli.py` imports:

```python
from reachability import load_mechanism_registry, evaluate_mechanisms, ReachabilityValidationError
from reachability_writer import write_reachability_outputs
```

Add function:

```python
def run_reachability_mode(scenario_path: str, mechanisms_path: str, stability_ranges_ref: str, out_dir: str):
    try:
        with open(scenario_path, "r", encoding="utf-8") as f:
            toml_str = f.read()
        config_hash = hashlib.sha256(toml_str.encode("utf-8")).hexdigest()
        config = load_and_validate_config(toml_str)
        history, result, reason = run_micro_simulation(config)
    except ValidationError as e:
        print(f"Scenario config is invalid: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading scenario file: {e}")
        sys.exit(1)

    try:
        with open(mechanisms_path, "r", encoding="utf-8") as f:
            mechanisms = load_mechanism_registry(f.read())
    except ReachabilityValidationError as e:
        print(f"Mechanism registry is invalid: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading mechanism registry: {e}")
        sys.exit(1)

    mechanism_results = evaluate_mechanisms(config, mechanisms, history, result, reason)
    write_reachability_outputs(
        out_dir,
        config.get("scenario_id", "reachability"),
        config_hash,
        config.get("seed", 0),
        config.get("tick_count", 0),
        stability_ranges_ref,
        mechanism_results,
    )
```

Add parser:

```python
reach_parser = subparsers.add_parser("reachability")
reach_parser.add_argument("--scenario", required=True)
reach_parser.add_argument("--mechanisms", required=True)
reach_parser.add_argument("--stability-ranges-ref", required=True)
reach_parser.add_argument("--out", required=True)
```

Add dispatch:

```python
elif args.command == "reachability":
    run_reachability_mode(args.scenario, args.mechanisms, args.stability_ranges_ref, args.out)
```

- [ ] **Step 4: Run test and verify GREEN**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability_cli.py -q
```

Expected:

```text
1 passed
```

---

### Task 6: Add Feedback Loop Report Rules

**Files:**
- Modify: `tools/early-stability/src/reachability_writer.py`
- Modify: `tools/early-stability/tests/test_reachability_writer.py`

- [ ] **Step 1: Write failing test for tuning feedback**

Add to `tools/early-stability/tests/test_reachability_writer.py`:

```python
def test_report_points_back_to_parameter_tuning_when_bypass_exists(tmp_path):
    results = sample_results()
    results[0]["reachability_result"] = "warning"
    results[0]["bypass_detected_count"] = 1
    results[0]["top_block_reason"] = "competing_path_cheaper"
    results[0]["notes"] = "Passive income may bypass uptake pressure."

    write_reachability_outputs(
        str(tmp_path),
        "run-2",
        "hash-2",
        42,
        100,
        "outputs/stability/example",
        results,
    )

    report = (tmp_path / "REPORT.md").read_text(encoding="utf-8")
    assert "Return to parameter tuning" in report
    assert "competing_path_cheaper" in report
```

- [ ] **Step 2: Run test and verify RED**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability_writer.py::test_report_points_back_to_parameter_tuning_when_bypass_exists -q
```

Expected:

```text
AssertionError: 'Return to parameter tuning' not in report
```

- [ ] **Step 3: Add feedback section to report writer**

In `write_reachability_outputs()`, before `## Decision`, add:

```python
    needs_tuning = [r for r in mechanism_results if r["reachability_result"] in {"warning", "fail", "blocked"}]
    lines.extend(["", "## Feedback Loop"])
    if needs_tuning:
        lines.append("Return to parameter tuning before data model design.")
        lines.append("")
        lines.append("| Mechanism | Reason | Suggested Action |")
        lines.append("| --- | --- | --- |")
        for r in needs_tuning:
            lines.append(
                f"| {r['mechanism_id']} | {r['top_block_reason']} | "
                "Adjust relevant early-stability tuning group and rerun reachability. |"
            )
    else:
        lines.append("No parameter tuning loop required for currently evaluated mechanisms.")
```

- [ ] **Step 4: Run tests and verify GREEN**

Run:

```powershell
python -m pytest .\tools\early-stability\tests\test_reachability_writer.py -q
```

Expected:

```text
3 passed
```

---

### Task 7: Documentation Updates

**Files:**
- Modify: `tools/early-stability/README.md`
- Modify: `docs/implementation/early-stability-tool.md`
- Modify: `docs/implementation/mechanism-reachability.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-mechanism-reachability.md`

- [ ] **Step 1: Update README command section**

Add this section to `tools/early-stability/README.md`:

```markdown
### reachability

Runs observer-only mechanism reachability analysis after stability tuning.

```powershell
early-stability reachability --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --mechanisms .\tools\early-stability\mechanisms\phase1.toml --stability-ranges-ref .\outputs\stability\group3_capacity_revalidated --out .\outputs\reachability\phase1_smoke
```

Use this after `tune`, not before it. If reachability reports bypassed or blocked mechanisms, return to parameter tuning and adjust the relevant tuning group.
```

- [ ] **Step 2: Update implementation docs**

In `docs/implementation/early-stability-tool.md`, add `reachability` as a related second-stage mode and link to:

```markdown
[[docs/implementation/mechanism-reachability|Mechanism Reachability]]
```

In `docs/implementation/mechanism-reachability.md`, add actual CLI command once implemented:

```powershell
python .\tools\early-stability\src\cli.py reachability --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --mechanisms .\tools\early-stability\mechanisms\phase1.toml --stability-ranges-ref .\outputs\stability\group3_capacity_revalidated --out .\outputs\reachability\phase1_smoke
```

- [ ] **Step 3: Create worklog report**

Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-mechanism-reachability.md`:

```markdown
# REPORT: Mechanism Reachability

Date: YYYY-MM-DD HH:MM

## Goal
Implement first observer-only mechanism reachability mode.

## Files Changed
- ...

## Verification
- `python -m pytest .\tools\early-stability`: ...
- CLI reachability smoke: ...

## Interpretation
- Which mechanisms passed.
- Which mechanisms were warning/fail/tool-limited.
- Whether to proceed to data model docs.
- Which parameter tuning group to revisit.
```

---

### Task 8: Full Verification

**Files:**
- No source edits unless verification fails.

- [ ] **Step 1: Run full tests**

Run:

```powershell
python -m pytest .\tools\early-stability
```

Expected:

```text
all tests pass
```

- [ ] **Step 2: Run reachability smoke**

Run:

```powershell
python .\tools\early-stability\src\cli.py reachability --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --mechanisms .\tools\early-stability\mechanisms\phase1.toml --stability-ranges-ref .\outputs\stability\group3_capacity_revalidated --out .\outputs\reachability\phase1_smoke
```

Expected artifacts:

```text
outputs/reachability/phase1_smoke/results.json
outputs/reachability/phase1_smoke/mechanisms.json
outputs/reachability/phase1_smoke/block-reasons.json
outputs/reachability/phase1_smoke/bypass.json
outputs/reachability/phase1_smoke/REPORT.md
```

- [ ] **Step 3: Inspect report for decision line**

Run:

```powershell
Select-String -Path .\outputs\reachability\phase1_smoke\REPORT.md -Pattern "Proceed to data model docs","Return to parameter tuning"
```

Expected:

```text
At least one decision or feedback-loop line is present.
```

- [ ] **Step 4: Check git hygiene**

Run:

```powershell
git status --short
```

Expected:

```text
Generated outputs/reachability artifacts are ignored or left untracked only if explicitly allowed.
Source changes and worklog report are visible.
```

---

## Acceptance Criteria

Implementation is complete when:

- mechanism registry loads and validates;
- `phase1.toml` registry exists;
- `reachability` CLI command writes all required artifacts;
- mechanism results distinguish `pass`, `warning`, `fail`, `blocked`, `tool_limited`, `future_only`;
- report includes feedback loop to parameter tuning;
- estimate-only mechanisms are not reported as high-confidence runtime mechanisms;
- invalid or bypassed mechanisms do not silently pass;
- full `tools/early-stability` test suite passes;
- documentation explains that reachability follows parameter tuning and can send work back to tuning.

## Self-Review

Spec coverage:

- Linked loop with `early-stability-parameter-tuning`: covered in Tasks 6-7.
- Reachability counters and block reasons: covered in Tasks 3-4.
- Mechanism registry: covered in Tasks 1-2.
- CLI/artifacts: covered in Tasks 4-5 and Task 8.
- Instruction/manual: covered in Task 7 and `docs/implementation/mechanism-reachability.md`.

Placeholder scan:

- Placeholder scan completed; all steps contain concrete files, commands, expected results and code snippets where needed.
