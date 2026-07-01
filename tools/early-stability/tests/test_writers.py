import os
import json
import pytest
import tomllib
from result_writer import write_results_json, write_ranges_json, write_run_detail_json
from report_writer import write_report_markdown, write_recommended_toml

def test_write_results_json(tmp_path):
    output_dir = str(tmp_path)
    data = {
        "scenario_id": "test_scenario",
        "config_hash": "abc123hash",
        "seed": 42,
        "tick_count": 100,
        "survival_result": "stable",
        "collapse_reason": "none",
        "metrics_summary": {
            "final_energy": 50.0,
            "warnings_triggered": False
        }
    }
    
    write_results_json(output_dir, data)
    
    filepath = os.path.join(output_dir, "results.json")
    assert os.path.exists(filepath)
    with open(filepath, "r") as f:
        loaded = json.load(f)
    assert loaded == data

def test_write_ranges_json(tmp_path):
    output_dir = str(tmp_path)
    ranges = [
        {
            "parameter_id": "cell.initial_energy",
            "tested_min": 5.0,
            "tested_max": 50.0,
            "stable_min": 20.0,
            "stable_max": 50.0,
            "recommended": 35.0,
            "confidence": "high",
            "notes": "Values below 20.0 lead to collapse"
        }
    ]
    
    write_ranges_json(output_dir, ranges)
    
    filepath = os.path.join(output_dir, "ranges.json")
    assert os.path.exists(filepath)
    with open(filepath, "r") as f:
        loaded = json.load(f)
    assert loaded == ranges

def test_write_run_detail_json(tmp_path):
    output_dir = str(tmp_path)
    run_data = {
        "parameters": {"cell.initial_energy": 10.0},
        "seed": 42,
        "survival_result": "stable",
        "collapse_reason": "none"
    }
    
    write_run_detail_json(output_dir, 1, run_data)
    
    filepath = os.path.join(output_dir, "runs", "run_0001.json")
    assert os.path.exists(filepath)
    with open(filepath, "r") as f:
        loaded = json.load(f)
    assert loaded == run_data

def test_write_recommended_toml(tmp_path):
    output_dir = str(tmp_path)
    base_config = {
        "scenario_id": "test_scenario",
        "seed": 42,
        "cell": {
            "initial_energy": 10.0,
            "radius": 1.0
        }
    }
    params = {
        "cell.initial_energy": 25.0
    }
    
    write_recommended_toml(output_dir, base_config, "best_stable", params)
    
    filepath = os.path.join(output_dir, "recommended-configs", "best_stable.toml")
    assert os.path.exists(filepath)
    with open(filepath, "rb") as f:
        loaded = tomllib.load(f)
    assert loaded["cell"]["initial_energy"] == 25.0
    assert loaded["cell"]["radius"] == 1.0
    assert loaded["scenario_id"] == "test_scenario"

def test_write_report_markdown(tmp_path):
    output_dir = str(tmp_path)
    report_data = {
        "run_id": "run_9999",
        "mode": "tune",
        "iteration_count": 12,
        "stable_count": 4,
        "fragile_count": 2,
        "collapse_count": 6,
        "invalid_count": 0,
        "warnings": ["Heat dissipation is close to generation limit"],
        "limits_of_evidence": ["Evaluated with simple deterministic micro-sim only"]
    }
    
    write_report_markdown(output_dir, report_data)
    
    filepath = os.path.join(output_dir, "REPORT.md")
    assert os.path.exists(filepath)
    with open(filepath, "r") as f:
        content = f.read()
        
    assert "# run_9999" in content or "run_9999" in content
    assert "mode" in content.lower()
    assert "iteration count" in content.lower() or "iteration" in content.lower()
    assert "stable" in content.lower()
    assert "fragile" in content.lower()
    assert "warnings" in content.lower()
    assert "limits of evidence" in content.lower()
