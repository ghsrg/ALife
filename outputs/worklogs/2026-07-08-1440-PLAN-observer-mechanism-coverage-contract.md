# Observer Mechanism Coverage Contract Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the first Observer-backed mechanism coverage layer for `tools/early-stability`, so registered mechanisms can produce coverage statuses, warning codes and machine-readable artifacts.

**Architecture:** Keep this outside Core. The Python tool reads the current adapter registry from `tools/early-stability/mechanisms/*.toml`, normalizes it into Observer coverage records, checks scenario/metric coverage, and writes CSV/JSON/Markdown artifacts. This is an Observer diagnostic path; it must not mutate accepted configs or simulation behavior.

**Tech Stack:** Python 3.11, pytest, stdlib `csv`, `json`, `dataclasses`, `pathlib`, existing `tools/early-stability` modules.

**Important:** Do not commit automatically. Leave changes unstaged unless the owner explicitly asks for git actions.

---

## Required Reading

- [[docs/observer/observer-layer|Observer Layer]]
- [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]]
- [[docs/mechanics/observer-projection|Committed State -> Observer Projection]]
- [[outputs/worklogs/2026-07-04-1405-PLAN-sweep_scenario-eval-coverage_refactor|Sweep Scenario Eval Coverage Refactor]]
- [[docs/implementation/mechanism-reachability|Mechanism Reachability]]

## File Structure

- Modify: `tools/early-stability/src/reachability.py`
  - Keep current reachability evaluators.
  - Add coverage normalization and coverage evaluation helpers.
- Modify: `tools/early-stability/src/reachability_writer.py`
  - Add `write_mechanism_coverage_outputs(...)`.
- Modify: `tools/early-stability/src/cli.py`
  - Add or extend a CLI path for coverage artifact export after reachability runs.
- Modify: `tools/early-stability/tests/test_reachability.py`
  - Add unit tests for coverage normalization and warning generation.
- Modify: `tools/early-stability/tests/test_reachability_writer.py`
  - Add artifact writer tests for CSV/JSON/Markdown.
- Modify: `tools/early-stability/tests/test_reachability_cli.py`
  - Add CLI-level smoke test for coverage output generation.
- Modify: `tools/early-stability/mechanisms/phase1.toml`
  - Add adapter fields only if tests require them and backward-compatible defaults are not enough.

## Task 1: Normalize Mechanism Coverage Records

**Files:**
- Modify: `tools/early-stability/src/reachability.py`
- Modify: `tools/early-stability/tests/test_reachability.py`

- [ ] **Step 1: Write the failing normalization test**

Add this test to `tools/early-stability/tests/test_reachability.py`:

```python
def test_build_coverage_records_normalizes_registry_defaults():
    mechanisms = load_mechanism_registry(VALID_REGISTRY)

    records = build_coverage_records(mechanisms)

    assert records == [
        {
            "mechanism_id": "mandatory_energy_cost",
            "category": "uncategorized",
            "introduced_in_phase": "unknown",
            "registered": True,
            "enabled": True,
            "activation_scenario": "single_cell_survival",
            "isolated_test": "",
            "integration_test": "",
            "raw_metrics": "",
            "cost_metrics": "",
            "benefit_metrics": "",
            "balance_sweep": "",
            "status": "missing_metrics",
            "warning_codes": ["METRIC_MISSING"],
        }
    ]
```

- [ ] **Step 2: Run the test and verify RED**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability.py::test_build_coverage_records_normalizes_registry_defaults -q
```

Expected: FAIL with `NameError` or import error for `build_coverage_records`.

- [ ] **Step 3: Implement minimal normalization**

In `tools/early-stability/src/reachability.py`, add:

```python
COVERAGE_STATUSES = {
    "covered",
    "partially_covered",
    "registered_but_disabled",
    "not_activated",
    "missing_scenario",
    "missing_metrics",
    "missing_balance_test",
}


def _first_or_empty(values: list[str]) -> str:
    return values[0] if values else ""


def build_coverage_records(mechanisms: list[dict]) -> list[dict]:
    records = []
    for mechanism in mechanisms:
        enabled = mechanism["status"] == "now"
        scenario = _first_or_empty(mechanism.get("required_scenarios", []))
        status = "missing_metrics" if enabled and scenario else "registered_but_disabled"
        warnings = ["METRIC_MISSING"] if status == "missing_metrics" else []
        records.append(
            {
                "mechanism_id": mechanism["mechanism_id"],
                "category": mechanism.get("category", "uncategorized"),
                "introduced_in_phase": mechanism.get("introduced_in_phase", "unknown"),
                "registered": True,
                "enabled": enabled,
                "activation_scenario": scenario,
                "isolated_test": mechanism.get("isolated_test", ""),
                "integration_test": mechanism.get("integration_test", ""),
                "raw_metrics": mechanism.get("raw_metrics", ""),
                "cost_metrics": mechanism.get("cost_metrics", ""),
                "benefit_metrics": mechanism.get("benefit_metrics", ""),
                "balance_sweep": mechanism.get("balance_sweep", ""),
                "status": status,
                "warning_codes": warnings,
            }
        )
    return records
```

- [ ] **Step 4: Run the test and verify GREEN**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability.py::test_build_coverage_records_normalizes_registry_defaults -q
```

Expected: PASS.

## Task 2: Evaluate Coverage From Reachability Results

**Files:**
- Modify: `tools/early-stability/src/reachability.py`
- Modify: `tools/early-stability/tests/test_reachability.py`

- [ ] **Step 1: Write the failing coverage evaluation tests**

Add:

```python
def test_evaluate_coverage_marks_passed_mechanism_as_partially_covered_without_balance():
    records = build_coverage_records(load_mechanism_registry(VALID_REGISTRY))
    reachability_results = [
        {
            "mechanism_id": "mandatory_energy_cost",
            "reachability_result": "pass",
            "executed_count": 3,
            "effect_nonzero_count": 3,
        }
    ]

    coverage = evaluate_coverage_records(records, reachability_results)

    assert coverage[0]["status"] == "partially_covered"
    assert coverage[0]["warning_codes"] == ["MECHANIC_TRADEOFF_MISSING"]


def test_evaluate_coverage_flags_registered_mechanism_without_reachability_result():
    records = build_coverage_records(load_mechanism_registry(VALID_REGISTRY))

    coverage = evaluate_coverage_records(records, [])

    assert coverage[0]["status"] == "not_activated"
    assert coverage[0]["warning_codes"] == ["UNTESTED_REGISTERED_MECHANISM"]
```

- [ ] **Step 2: Run tests and verify RED**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability.py::test_evaluate_coverage_marks_passed_mechanism_as_partially_covered_without_balance tools/early-stability/tests/test_reachability.py::test_evaluate_coverage_flags_registered_mechanism_without_reachability_result -q
```

Expected: FAIL because `evaluate_coverage_records` does not exist.

- [ ] **Step 3: Implement minimal coverage evaluation**

Add:

```python
def evaluate_coverage_records(records: list[dict], reachability_results: list[dict]) -> list[dict]:
    by_id = {item["mechanism_id"]: item for item in reachability_results}
    evaluated = []
    for record in records:
        updated = {**record}
        result = by_id.get(record["mechanism_id"])
        if not record["enabled"]:
            updated["status"] = "registered_but_disabled"
            updated["warning_codes"] = []
        elif result is None:
            updated["status"] = "not_activated"
            updated["warning_codes"] = ["UNTESTED_REGISTERED_MECHANISM"]
        elif result["reachability_result"] == "pass" and result.get("effect_nonzero_count", 0) > 0:
            has_balance = bool(updated["balance_sweep"])
            updated["status"] = "covered" if has_balance else "partially_covered"
            updated["warning_codes"] = [] if has_balance else ["MECHANIC_TRADEOFF_MISSING"]
        elif result["reachability_result"] in {"blocked", "fail"}:
            updated["status"] = "not_activated"
            updated["warning_codes"] = ["SCENARIO_MECHANISM_NOT_ACTIVATED"]
        else:
            updated["status"] = "partially_covered"
            updated["warning_codes"] = ["METRIC_MISSING"]
        evaluated.append(updated)
    return evaluated
```

- [ ] **Step 4: Run tests and verify GREEN**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability.py -q
```

Expected: all reachability tests PASS.

## Task 3: Write Coverage Artifacts

**Files:**
- Modify: `tools/early-stability/src/reachability_writer.py`
- Modify: `tools/early-stability/tests/test_reachability_writer.py`

- [ ] **Step 1: Write failing writer test**

Add:

```python
def test_write_mechanism_coverage_outputs_writes_csv_json_and_markdown(tmp_path):
    coverage = [
        {
            "mechanism_id": "mandatory_energy_cost",
            "category": "lifecycle",
            "introduced_in_phase": "phase1",
            "registered": True,
            "enabled": True,
            "activation_scenario": "single_cell_survival",
            "isolated_test": "tests/test.py::test_cost",
            "integration_test": "",
            "raw_metrics": "energy_delta",
            "cost_metrics": "energy_spent",
            "benefit_metrics": "",
            "balance_sweep": "",
            "status": "partially_covered",
            "warning_codes": ["MECHANIC_TRADEOFF_MISSING"],
        }
    ]

    write_mechanism_coverage_outputs(str(tmp_path), "2026-07-08-1440", coverage)

    assert (tmp_path / "raw_data" / "mechanism_coverage.csv").exists()
    assert (tmp_path / "reports" / "mechanism-coverage-2026-07-08-1440.json").exists()
    report = tmp_path / "reports" / "mechanism-coverage-2026-07-08-1440.md"
    assert report.exists()
    assert "mandatory_energy_cost" in report.read_text(encoding="utf-8")
    assert "MECHANIC_TRADEOFF_MISSING" in report.read_text(encoding="utf-8")
```

- [ ] **Step 2: Run test and verify RED**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability_writer.py::test_write_mechanism_coverage_outputs_writes_csv_json_and_markdown -q
```

Expected: FAIL because writer does not exist.

- [ ] **Step 3: Implement writer**

Add to `tools/early-stability/src/reachability_writer.py`:

```python
import csv
from pathlib import Path


def write_mechanism_coverage_outputs(root_dir: str, timestamp: str, coverage: list[dict]) -> None:
    root = Path(root_dir)
    raw_dir = root / "raw_data"
    reports_dir = root / "reports"
    raw_dir.mkdir(parents=True, exist_ok=True)
    reports_dir.mkdir(parents=True, exist_ok=True)

    fields = [
        "mechanism_id",
        "category",
        "introduced_in_phase",
        "registered",
        "enabled",
        "activation_scenario",
        "isolated_test",
        "integration_test",
        "raw_metrics",
        "cost_metrics",
        "benefit_metrics",
        "balance_sweep",
        "status",
        "warning_codes",
    ]

    csv_path = raw_dir / "mechanism_coverage.csv"
    with csv_path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        for row in coverage:
            serializable = {**row, "warning_codes": ",".join(row.get("warning_codes", []))}
            writer.writerow(serializable)

    json_path = reports_dir / f"mechanism-coverage-{timestamp}.json"
    write_json(str(json_path), coverage)

    lines = [
        f"# Mechanism Coverage Report: {timestamp}",
        "",
        "| Mechanism | Status | Warnings |",
        "| --- | --- | --- |",
    ]
    for row in coverage:
        warnings = ", ".join(row.get("warning_codes", [])) or "none"
        lines.append(f"| {row['mechanism_id']} | {row['status']} | {warnings} |")

    md_path = reports_dir / f"mechanism-coverage-{timestamp}.md"
    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")
```

- [ ] **Step 4: Run writer tests and verify GREEN**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability_writer.py -q
```

Expected: all writer tests PASS.

## Task 4: Add CLI Coverage Export

**Files:**
- Modify: `tools/early-stability/src/cli.py`
- Modify: `tools/early-stability/tests/test_reachability_cli.py`

- [ ] **Step 1: Write failing CLI smoke test**

Add a test that invokes the existing reachability command with coverage export enabled. If the current CLI uses a different helper pattern, adapt only the invocation wrapper, not the assertions:

```python
def test_reachability_cli_writes_mechanism_coverage_outputs(tmp_path):
    result = run_cli(
        [
            "reachability",
            "--scenario",
            "tools/early-stability/scenarios/single_cell_survival.toml",
            "--mechanisms",
            "tools/early-stability/mechanisms/phase1.toml",
            "--output-dir",
            str(tmp_path),
            "--coverage",
            "--timestamp",
            "2026-07-08-1440",
        ]
    )

    assert result.exit_code == 0
    assert (tmp_path / "raw_data" / "mechanism_coverage.csv").exists()
    assert (tmp_path / "reports" / "mechanism-coverage-2026-07-08-1440.json").exists()
```

- [ ] **Step 2: Run test and verify RED**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability_cli.py::test_reachability_cli_writes_mechanism_coverage_outputs -q
```

Expected: FAIL because `--coverage` or `--timestamp` is not supported.

- [ ] **Step 3: Implement CLI integration**

In `tools/early-stability/src/cli.py`, wire the new functions:

```python
from reachability import build_coverage_records, evaluate_coverage_records
from reachability_writer import write_mechanism_coverage_outputs
```

Add arguments to the reachability parser:

```python
reachability_parser.add_argument("--coverage", action="store_true")
reachability_parser.add_argument("--timestamp", default=None)
```

After reachability results are produced:

```python
if args.coverage:
    timestamp = args.timestamp or run_id
    records = build_coverage_records(mechanisms)
    coverage = evaluate_coverage_records(records, mechanism_results)
    write_mechanism_coverage_outputs(args.output_dir, timestamp, coverage)
```

- [ ] **Step 4: Run CLI test and verify GREEN**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability_cli.py -q
```

Expected: all reachability CLI tests PASS.

## Task 5: Phase Delta And Recommended Rerun Placeholders

**Files:**
- Modify: `tools/early-stability/src/reachability_writer.py`
- Modify: `tools/early-stability/tests/test_reachability_writer.py`

- [ ] **Step 1: Write failing test for phase files**

Add:

```python
def test_write_mechanism_coverage_outputs_writes_phase_delta_and_reruns(tmp_path):
    coverage = [
        {
            "mechanism_id": "storage_material",
            "category": "material",
            "introduced_in_phase": "phase2",
            "registered": True,
            "enabled": True,
            "activation_scenario": "",
            "isolated_test": "",
            "integration_test": "",
            "raw_metrics": "",
            "cost_metrics": "",
            "benefit_metrics": "",
            "balance_sweep": "",
            "status": "not_activated",
            "warning_codes": ["UNTESTED_REGISTERED_MECHANISM"],
        }
    ]

    write_mechanism_coverage_outputs(str(tmp_path), "2026-07-08-1440", coverage)

    assert (tmp_path / "raw_data" / "phase_mechanism_delta.csv").exists()
    assert (tmp_path / "raw_data" / "phase_test_coverage_delta.csv").exists()
    assert (tmp_path / "reports" / "phase_balance_impact.md").exists()
    reruns = tmp_path / "reports" / "recommended-reruns-2026-07-08-1440.md"
    assert reruns.exists()
    assert "storage_material" in reruns.read_text(encoding="utf-8")
```

- [ ] **Step 2: Run test and verify RED**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability_writer.py::test_write_mechanism_coverage_outputs_writes_phase_delta_and_reruns -q
```

Expected: FAIL because phase delta files are not written.

- [ ] **Step 3: Implement minimal phase/rerun outputs**

Extend `write_mechanism_coverage_outputs(...)`:

```python
    phase_mechanism_delta = raw_dir / "phase_mechanism_delta.csv"
    with phase_mechanism_delta.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["introduced_in_phase", "mechanism_id", "category", "status"])
        writer.writeheader()
        for row in coverage:
            writer.writerow(
                {
                    "introduced_in_phase": row["introduced_in_phase"],
                    "mechanism_id": row["mechanism_id"],
                    "category": row["category"],
                    "status": row["status"],
                }
            )

    phase_test_coverage_delta = raw_dir / "phase_test_coverage_delta.csv"
    with phase_test_coverage_delta.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["mechanism_id", "isolated_test", "integration_test", "balance_sweep"])
        writer.writeheader()
        for row in coverage:
            writer.writerow(
                {
                    "mechanism_id": row["mechanism_id"],
                    "isolated_test": row["isolated_test"],
                    "integration_test": row["integration_test"],
                    "balance_sweep": row["balance_sweep"],
                }
            )

    impact_lines = ["# Phase Balance Impact", ""]
    for row in coverage:
        impact_lines.append(f"- {row['mechanism_id']}: {row['status']}")
    (reports_dir / "phase_balance_impact.md").write_text("\n".join(impact_lines) + "\n", encoding="utf-8")

    rerun_lines = [f"# Recommended Reruns: {timestamp}", ""]
    for row in coverage:
        if row["status"] != "covered":
            rerun_lines.append(f"- {row['mechanism_id']}: add activation scenario and rerun related sweeps.")
    (reports_dir / f"recommended-reruns-{timestamp}.md").write_text(
        "\n".join(rerun_lines) + "\n",
        encoding="utf-8",
    )
```

- [ ] **Step 4: Run writer tests and verify GREEN**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability_writer.py -q
```

Expected: all writer tests PASS.

## Task 6: Full Verification

**Files:**
- No new files beyond previous tasks.

- [ ] **Step 1: Run focused Python tests**

Run:

```bash
python -m pytest tools/early-stability/tests/test_reachability.py tools/early-stability/tests/test_reachability_writer.py tools/early-stability/tests/test_reachability_cli.py -q
```

Expected: PASS.

- [ ] **Step 2: Run all early-stability tests**

Run:

```bash
python -m pytest tools/early-stability -q
```

Expected: PASS.

- [ ] **Step 3: Run Rust tests to check no Core regression**

Run:

```bash
cargo test
```

Expected: PASS.

- [ ] **Step 4: Write implementation report**

Create:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-observer-mechanism-coverage-contract.md
```

Report must include:

```text
tests run
artifacts produced
new warning/status behavior
remaining limitations
whether Phase 2 coverage can proceed
```

## Self-Review

Coverage against spec:

- Observer docs require registry-driven coverage: Task 1 and Task 2.
- Coverage artifacts required by contract: Task 3 and Task 5.
- CLI handoff for agents: Task 4.
- Verification and report: Task 6.

Known scope boundary:

- This plan does not implement full UI Observer or live projection service.
- This plan does not require Rust Core to export registries yet; current TOML registry remains the adapter source until Core export exists.
