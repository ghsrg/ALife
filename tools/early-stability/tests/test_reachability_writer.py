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
