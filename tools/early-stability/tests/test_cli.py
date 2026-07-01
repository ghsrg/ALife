import os
import json
import pytest
import tomllib
from cli import main

VALID_SCENARIO_TOML = """
scenario_id = "test_survival"
seed = 42
tick_count = 10

[world]
size = [256.0, 256.0]
boundary_mode = "solid_wall"

[space]
spatial_grid_size = 8.0

[resources]
resource_type_ids = ["nutrient"]
initial_distribution = [10.0]
passive_energy_income_placeholder = 1.0

[cell]
initial_position = [128.0, 128.0]
radius = 1.0
initial_resources = { nutrient = 2.0 }
initial_materials = { material = 3.0 }
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 2.0
dormant_mandatory_cost_modifier = 0.1
capacity_limit = 10.0
minimum_viability_materials = ["material"]

[environment]
ambient_temperature = 25.0
heat_current = 0.0
heat_generated_per_tick = 0.1
heat_dissipation_rate = 0.2
heat_warning_threshold = 5.0
heat_death_threshold = 10.0
waste_current = 0.0
waste_generated_per_tick = 0.05
waste_sink_rate = 0.1
waste_warning_threshold = 2.0
waste_death_threshold = 5.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
critical_capacity_overrun = 5.0
"""

TUNING_TOML = """
[tuning]
max_iterations = 10
seeds = [42, 100]
objective = "map_stable_ranges"

[tuning.ranges]
"cell.initial_energy" = [5.0, 15.0, 5.0]
"""

@pytest.fixture
def setup_files(tmp_path):
    scenarios_dir = tmp_path / "scenarios"
    scenarios_dir.mkdir()
    
    scenario_path = scenarios_dir / "single_cell_survival.toml"
    scenario_path.write_text(VALID_SCENARIO_TOML, encoding="utf-8")
    
    tuning_dir = tmp_path / "tuning"
    tuning_dir.mkdir()
    tuning_path = tuning_dir / "single_cell.toml"
    tuning_path.write_text(TUNING_TOML, encoding="utf-8")
    
    out_dir = tmp_path / "out"
    
    return {
        "scenario_path": str(scenario_path),
        "tuning_path": str(tuning_path),
        "out_dir": str(out_dir),
        "scenarios_dir": str(scenarios_dir)
    }

def test_cli_evaluate(setup_files):
    args = [
        "evaluate",
        "--scenario", setup_files["scenario_path"],
        "--out", setup_files["out_dir"]
    ]
    main(args)
    
    # Assert output files exist
    results_json_path = os.path.join(setup_files["out_dir"], "results.json")
    report_md_path = os.path.join(setup_files["out_dir"], "REPORT.md")
    
    assert os.path.exists(results_json_path)
    assert os.path.exists(report_md_path)
    
    with open(results_json_path, "r") as f:
        res = json.load(f)
    assert res["scenario_id"] == "test_survival"
    assert res["survival_result"] == "stable"

def test_cli_simulate(setup_files):
    args = [
        "simulate",
        "--scenario", setup_files["scenario_path"],
        "--ticks", "5",
        "--out", setup_files["out_dir"]
    ]
    main(args)
    
    results_json_path = os.path.join(setup_files["out_dir"], "results.json")
    run_file_path = os.path.join(setup_files["out_dir"], "runs", "run_0001.json")
    
    assert os.path.exists(results_json_path)
    assert os.path.exists(run_file_path)

def test_cli_tune(setup_files):
    args = [
        "tune",
        "--scenario", setup_files["scenario_path"],
        "--tuning", setup_files["tuning_path"],
        "--out", setup_files["out_dir"]
    ]
    main(args)
    
    ranges_json_path = os.path.join(setup_files["out_dir"], "ranges.json")
    assert os.path.exists(ranges_json_path)

def test_cli_batch(setup_files):
    # Add a second scenario to batch run
    second_scenario_path = os.path.join(setup_files["scenarios_dir"], "another_scenario.toml")
    # This one will be invalid to check batch aggregates different results
    invalid_toml = "invalid toml data!!!"
    with open(second_scenario_path, "w") as f:
        f.write(invalid_toml)
        
    args = [
        "batch",
        "--scenarios", setup_files["scenarios_dir"],
        "--out", setup_files["out_dir"]
    ]
    main(args)
    
    results_json_path = os.path.join(setup_files["out_dir"], "results.json")
    report_md_path = os.path.join(setup_files["out_dir"], "REPORT.md")
    
    assert os.path.exists(results_json_path)
    assert os.path.exists(report_md_path)
    
    with open(results_json_path, "r") as f:
        res = json.load(f)
        
    assert "scenarios" in res
    assert len(res["scenarios"]) == 2
    # Alphabetical order: another_scenario.toml (invalid), then single_cell_survival.toml (stable)
    assert res["scenarios"][0]["scenario_id"] == "another_scenario.toml"
    assert res["scenarios"][0]["survival_result"] == "invalid"
    assert res["scenarios"][1]["scenario_id"] == "test_survival"
    assert res["scenarios"][1]["survival_result"] == "stable"

def test_cli_batch_on_real_scenarios(tmp_path):
    # Locate actual scenarios directory
    test_dir = os.path.dirname(os.path.abspath(__file__))
    scenarios_dir = os.path.join(test_dir, "..", "scenarios")
    
    out_dir = str(tmp_path / "out")
    
    args = [
        "batch",
        "--scenarios", scenarios_dir,
        "--out", out_dir
    ]
    main(args)
    
    results_json_path = os.path.join(out_dir, "results.json")
    report_md_path = os.path.join(out_dir, "REPORT.md")
    
    assert os.path.exists(results_json_path)
    assert os.path.exists(report_md_path)
    
    with open(results_json_path, "r") as f:
        res = json.load(f)
        
    assert "scenarios" in res
    assert len(res["scenarios"]) >= 5
    for scenario_res in res["scenarios"]:
        if scenario_res["file_name"] == "single_cell_over_capacity.toml":
            assert scenario_res["survival_result"] == "invalid"
        else:
            assert scenario_res["survival_result"] != "invalid", f"Scenario {scenario_res['file_name']} failed validation: {scenario_res.get('collapse_reason')}"


