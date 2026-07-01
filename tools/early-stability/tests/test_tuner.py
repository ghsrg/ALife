import pytest
from unittest.mock import patch
from tuner import TuningValidationError, run_tuning

@pytest.fixture
def base_config():
    return {
        "scenario_id": "test_scenario",
        "seed": 42,
        "tick_count": 5,
        "world": {},
        "space": {},
        "resources": {
            "passive_energy_income_placeholder": 1.0
        },
        "cell": {
            "initial_energy": 10.0,
            "energy_capacity": 100.0,
            "mandatory_cost_per_tick": 2.0,
            "dormant_mandatory_cost_modifier": 0.1,
            "capacity_limit": 10.0,
            "initial_resources": {"nutrient_A": 2.0},
            "initial_materials": {"material_A": 3.0}
        },
        "environment": {
            "ambient_temperature": 25.0,
            "heat_current": 0.0,
            "heat_generated_per_tick": 0.1,
            "heat_dissipation_rate": 0.2,
            "heat_warning_threshold": 5.0,
            "heat_death_threshold": 10.0,
            "waste_current": 0.0,
            "waste_generated_per_tick": 0.05,
            "waste_sink_rate": 0.1,
            "waste_warning_threshold": 2.0,
            "waste_death_threshold": 5.0
        },
        "lifecycle": {
            "stress_energy_threshold": 10.0,
            "dormancy_allowed": True,
            "critical_capacity_overrun": 5.0
        }
    }

@pytest.fixture
def tuning_config():
    return {
        "tuning": {
            "max_iterations": 100,
            "seeds": [42, 100],
            "objective": "map_stable_ranges",
            "allowed_parameters": ["cell.initial_energy"],
            "ranges": {
                "cell.initial_energy": [5.0, 15.0, 5.0]  # values: 5.0, 10.0, 15.0
            }
        }
    }

def test_map_stable_ranges(base_config, tuning_config):
    # Map stable ranges will evaluate all candidates: 5.0, 10.0, 15.0
    runs, ranges, profiles = run_tuning(base_config, tuning_config)
    
    # 3 candidates * 2 seeds = 6 runs
    assert len(runs) == 6
    assert len(ranges) == 1
    
    param_range = ranges[0]
    assert param_range["parameter_id"] == "cell.initial_energy"
    assert param_range["tested_min"] == 5.0
    assert param_range["tested_max"] == 15.0
    assert param_range["stable_min"] == 15.0
    assert param_range["stable_max"] == 15.0

def test_find_first_stable(base_config, tuning_config):
    tuning_config["tuning"]["objective"] = "find_first_stable"
    runs, ranges, profiles = run_tuning(base_config, tuning_config)
    assert len(runs) == 6
    assert ranges[0]["stable_min"] == 15.0

def test_max_iterations(base_config, tuning_config):
    # Set max_iterations to 1
    tuning_config["tuning"]["max_iterations"] = 1
    runs, ranges, profiles = run_tuning(base_config, tuning_config)
    
    # Only 1 candidate configuration is evaluated.
    # 1 candidate * 2 seeds = 2 runs.
    assert len(runs) == 2
    assert ranges[0]["tested_min"] == 5.0
    assert ranges[0]["tested_max"] == 5.0
    assert ranges[0]["stable_min"] is None

def test_multiple_seeds_handling(base_config, tuning_config):
    def mock_simulate(config):
        if config["seed"] == 42:
            return [], "stable", "none"
        else:
            return [], "collapse", "energy_depleted"
            
    with patch("tuner.run_micro_simulation", side_effect=mock_simulate):
        runs, ranges, profiles = run_tuning(base_config, tuning_config)
        assert ranges[0]["stable_min"] is None
        assert ranges[0]["stable_max"] is None

def test_tuning_rejects_range_not_in_allowed_parameters(base_config, tuning_config):
    tuning_config["tuning"]["allowed_parameters"] = ["cell.initial_energy"]
    tuning_config["tuning"]["ranges"]["environment.heat_dissipation_rate"] = [0.1, 0.3, 0.1]

    with pytest.raises(TuningValidationError):
        run_tuning(base_config, tuning_config)

def test_tune_runs_include_history_and_final_metrics(base_config, tuning_config):
    runs, ranges, profiles = run_tuning(base_config, tuning_config)

    first = runs[0]
    assert "history" in first
    assert isinstance(first["history"], list)
    assert "final_energy" in first
    assert "final_heat" in first
    assert "final_waste" in first
    assert "final_state" in first

def test_tuning_rejects_allowed_parameter_without_range(base_config, tuning_config):
    tuning_config["tuning"]["allowed_parameters"] = ["cell.initial_energy", "cell.energy_capacity"]

    with pytest.raises(TuningValidationError):
        run_tuning(base_config, tuning_config)

def test_tuning_rejects_unknown_non_estimate_path(base_config, tuning_config):
    tuning_config["tuning"]["allowed_parameters"] = ["cell.unknown_field"]
    tuning_config["tuning"]["ranges"] = {"cell.unknown_field": [1.0, 2.0, 1.0]}

    with pytest.raises(TuningValidationError):
        run_tuning(base_config, tuning_config)

def test_find_conservative_stable(base_config, tuning_config):
    tuning_config["tuning"]["objective"] = "find_conservative_stable"
    # We vary cell.initial_energy: 5.0, 10.0, 15.0.
    # 15.0 has higher energy margin than 10.0 (and 10.0 is fragile anyway).
    # So 15.0 is conservative stable.
    runs, ranges, profiles = run_tuning(base_config, tuning_config)
    
    assert profiles["conservative_stable"] == {"cell.initial_energy": 15.0}

def test_candidate_profiles(base_config, tuning_config):
    # Set range to vary initial_energy: 10.0 and 15.0
    tuning_config["tuning"]["ranges"]["cell.initial_energy"] = [10.0, 15.0, 5.0]
    runs, ranges, profiles = run_tuning(base_config, tuning_config)
    
    # 15.0 is stable. 10.0 is fragile.
    # So:
    # best_stable: 15.0
    # conservative_stable: 15.0
    # fragile_edge: 10.0 (survived but triggered warnings/stressed)
    assert profiles["best_stable"] == {"cell.initial_energy": 15.0}
    assert profiles["conservative_stable"] == {"cell.initial_energy": 15.0}
    assert profiles["fragile_edge"] == {"cell.initial_energy": 10.0}
