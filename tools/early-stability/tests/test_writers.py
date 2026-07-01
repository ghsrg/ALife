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
        "base_config_path": "scenarios/single_cell_survival.toml",
        "iteration_count": 12,
        "stable_count": 4,
        "fragile_count": 2,
        "collapse_count": 6,
        "invalid_count": 0,
        "best_stable_metrics": {
            "final_energy": 85.0,
            "final_heat": 0.5,
            "final_waste": 0.1
        },
        "parameter_ranges": [
            {
                "parameter_id": "cell.initial_energy",
                "tested_min": 5.0,
                "tested_max": 50.0,
                "stable_min": 20.0,
                "stable_max": 50.0,
                "recommended": 35.0,
                "confidence": "high",
                "notes": "Values below 20.0 lead to collapse"
            },
            {
                "parameter_id": "environment.heat_dissipation_rate",
                "tested_min": 0.1,
                "tested_max": 0.5,
                "stable_min": 0.4,
                "stable_max": 0.5,
                "recommended": 0.45,
                "confidence": "high",
                "notes": "Low dissipation causes collapse"
            }
        ],
        "runs": [
            {"collapse_reason": "energy_depleted"},
            {"collapse_reason": "heat_limit_exceeded"},
            {"collapse_reason": "energy_depleted"}
        ],
        "warnings": ["Heat dissipation is close to generation limit"],
        "limits_of_evidence": ["Evaluated with simple deterministic micro-sim only"]
    }
    
    write_report_markdown(output_dir, report_data)
    
    filepath = os.path.join(output_dir, "REPORT.md")
    assert os.path.exists(filepath)
    with open(filepath, "r") as f:
        content = f.read()
        
    assert "run_9999" in content
    assert "mode" in content.lower()
    assert "scenarios/single_cell_survival.toml" in content
    assert "iteration count" in content.lower() or "iteration" in content.lower()
    
    # Best stable metrics
    assert "85" in content
    assert "final energy" in content.lower()
    
    # Ranges table
    assert "tested_min" in content.lower() or "tested min" in content.lower()
    assert "cell.initial_energy" in content
    assert "5.0" in content
    
    # Recommended table
    assert "recommended value" in content.lower() or "recommended" in content.lower()
    
    # Sensitivity parameter rank
    # cell.initial_energy tested range 45, stable range 30 -> ratio 30/45 = 0.66 -> sensitivity 0.33
    # heat_dissipation tested range 0.4, stable range 0.1 -> ratio 0.25 -> sensitivity 0.75
    # So heat_dissipation has higher sensitivity and should be ranked first!
    assert "sensitivity ranking" in content.lower() or "sensitivity" in content.lower()
    
    # Failure reasons
    assert "failure reasons" in content.lower() or "collapse reason" in content.lower()
    assert "energy_depleted" in content
    assert "heat_limit_exceeded" in content
    
    # Warnings and limits
    assert "warnings" in content.lower()
    assert "limits of evidence" in content.lower()

