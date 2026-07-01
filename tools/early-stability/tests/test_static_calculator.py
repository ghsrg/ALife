import pytest
from static_calculator import evaluate_static_bounds

@pytest.fixture
def base_config():
    return {
        "scenario_id": "test_scenario",
        "seed": 42,
        "tick_count": 100,
        "world": {},
        "space": {},
        "resources": {
            "passive_energy_income_placeholder": 1.0
        },
        "cell": {
            "initial_energy": 10.0,
            "mandatory_cost_per_tick": 2.0,
            "capacity_limit": 10.0,
            "initial_resources": {"nutrient_A": 2.0},
            "initial_materials": {"material_A": 3.0}
        },
        "environment": {
            "heat_generated_per_tick": 0.1,
            "heat_dissipation_rate": 0.2,
            "waste_generated_per_tick": 0.05,
            "waste_sink_rate": 0.1
        },
        "lifecycle": {}
    }

def test_stable_config(base_config):
    # Balanced stable config should return stable and none
    result, reason = evaluate_static_bounds(base_config)
    assert result == "stable"
    assert reason == "none"

def test_mandatory_cost_starvation(base_config):
    # initial_energy (10) + passive_energy_income_placeholder (0) < mandatory_cost_per_tick (15)
    base_config["cell"]["initial_energy"] = 10.0
    base_config["resources"]["passive_energy_income_placeholder"] = 0.0
    base_config["cell"]["mandatory_cost_per_tick"] = 15.0
    
    result, reason = evaluate_static_bounds(base_config)
    assert result == "collapse"
    assert reason == "mandatory_cost_unpaid"

def test_mandatory_cost_starvation_alternate_key(base_config):
    # Support mandatory_cost key as well
    base_config["cell"]["initial_energy"] = 5.0
    base_config["resources"]["passive_energy_income_placeholder"] = 0.0
    if "mandatory_cost_per_tick" in base_config["cell"]:
        del base_config["cell"]["mandatory_cost_per_tick"]
    base_config["cell"]["mandatory_cost"] = 10.0
    
    result, reason = evaluate_static_bounds(base_config)
    assert result == "collapse"
    assert reason == "mandatory_cost_unpaid"

def test_initial_capacity_exceeded(base_config):
    # initial_resources (6) + initial_materials (5) = 11 > capacity_limit (10)
    base_config["cell"]["capacity_limit"] = 10.0
    base_config["cell"]["initial_resources"] = {"nutrient_A": 6.0}
    base_config["cell"]["initial_materials"] = {"material_A": 5.0}
    
    result, reason = evaluate_static_bounds(base_config)
    assert result == "collapse"
    assert reason == "capacity_exceeded"

def test_unbounded_heat_accumulation(base_config):
    # heat_generated_per_tick (0.3) > heat_dissipation_rate (0.2)
    base_config["environment"]["heat_generated_per_tick"] = 0.3
    base_config["environment"]["heat_dissipation_rate"] = 0.2
    
    result, reason = evaluate_static_bounds(base_config)
    assert result == "collapse"
    assert reason == "heat_limit_exceeded"

def test_unbounded_waste_accumulation(base_config):
    # waste_generated_per_tick (0.15) > waste_sink_rate (0.1)
    base_config["environment"]["waste_generated_per_tick"] = 0.15
    base_config["environment"]["waste_sink_rate"] = 0.1
    
    result, reason = evaluate_static_bounds(base_config)
    assert result == "collapse"
    assert reason == "waste_limit_exceeded"
