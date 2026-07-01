import pytest
from unittest.mock import patch
from tuner import run_tuning

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
            "ranges": {
                "cell.initial_energy": [5.0, 15.0, 5.0]  # values: 5.0, 10.0, 15.0
            }
        }
    }

def test_map_stable_ranges(base_config, tuning_config):
    # Map stable ranges will evaluate all candidates: 5.0, 10.0, 15.0
    runs, ranges = run_tuning(base_config, tuning_config)
    
    # 3 candidates * 2 seeds = 6 runs
    assert len(runs) == 6
    assert len(ranges) == 1
    
    param_range = ranges[0]
    assert param_range["parameter_id"] == "cell.initial_energy"
    assert param_range["tested_min"] == 5.0
    assert param_range["tested_max"] == 15.0
    # Let's verify stable bounds:
    # 5.0: with cost=2 and income=1.
    # Start: 5. Tick 1: 5+1-2=4. Tick 2: 4+1-2=3. Tick 3: 3+1-2=2. Tick 4: 2+1-2=1. Tick 5: 1+1-2=0 (dead).
    # So 5.0 is collapse (not stable).
    # 10.0: Start: 10. Tick 1: 10+1-2=9. Tick 2: 9+1-2=8. Tick 3: 8+1-2=7. Tick 4: 7+1-2=6. Tick 5: 6+1-2=5.
    # Since 5 < 10 (stress threshold), it survives but is stressed (fragile).
    # Wait, is fragile considered "stable"?
    # The requirement: "If a candidate is stable across all seeds, save it as a recommended configuration."
    # Wait, does stable mean survival_result == "stable" or survival_result != "collapse" (i.e. survives)?
    # Let's check early-stability-tool.md: "If all seeds achieve stable result, mark parameter values as stable."
    # Wait, the survival_result values are: "stable", "fragile", "collapse", "invalid".
    # So if it says "achieve stable result", does it mean "stable" or "not collapse"?
    # In general, "stable" means survival_result is exactly "stable" (no warning triggered).
    # Let's check: 15.0: Start: 15.
    # Tick 1: 15+1-2=14. Tick 2: 14+1-2=13. Tick 3: 13+1-2=12. Tick 4: 12+1-2=11. Tick 5: 11+1-2=10.
    # All ticks are >= 10.0, so no warning triggered. Result is "stable"!
    # So only 15.0 should be stable.
    # Therefore, stable_min = 15.0, stable_max = 15.0.
    assert param_range["stable_min"] == 15.0
    assert param_range["stable_max"] == 15.0

def test_find_first_stable(base_config, tuning_config):
    tuning_config["tuning"]["objective"] = "find_first_stable"
    # Candidates are evaluated in order of initial_energy: 5.0, 10.0, 15.0.
    # 5.0 -> collapse
    # 10.0 -> fragile (not stable)
    # 15.0 -> stable. Once 15.0 is found, it is stable across all seeds, so we stop.
    runs, ranges = run_tuning(base_config, tuning_config)
    # We should have runs for 5.0, 10.0, and 15.0
    # So 3 candidates * 2 seeds = 6 runs total.
    assert len(runs) == 6
    assert ranges[0]["stable_min"] == 15.0

def test_max_iterations(base_config, tuning_config):
    # Set max_iterations to 1
    tuning_config["tuning"]["max_iterations"] = 1
    runs, ranges = run_tuning(base_config, tuning_config)
    
    # Only 1 candidate configuration is evaluated.
    # 1 candidate * 2 seeds = 2 runs.
    assert len(runs) == 2
    assert ranges[0]["tested_min"] == 5.0
    assert ranges[0]["tested_max"] == 5.0
    assert ranges[0]["stable_min"] is None

def test_multiple_seeds_handling(base_config, tuning_config):
    # We patch run_micro_simulation to return different results depending on the seed.
    # If seed is 42 -> stable, if seed is 100 -> collapse.
    # In this case, the candidate should NOT be marked stable because it failed on seed 100.
    
    def mock_simulate(config):
        if config["seed"] == 42:
            return [], "stable", "none"
        else:
            return [], "collapse", "energy_depleted"
            
    with patch("tuner.run_micro_simulation", side_effect=mock_simulate):
        runs, ranges = run_tuning(base_config, tuning_config)
        
        # Verify that even if it passed on seed 42, because it failed on seed 100, stable_min/max is None.
        assert ranges[0]["stable_min"] is None
        assert ranges[0]["stable_max"] is None
