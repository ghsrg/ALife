import json

from reachability_writer import (
    summarize_reachability,
    write_reachability_outputs,
    write_mechanism_coverage_outputs,
)


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


