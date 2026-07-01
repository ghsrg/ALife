import pytest
from micro_simulator import run_micro_simulation

@pytest.fixture
def base_config():
    return {
        "scenario_id": "test_scenario",
        "seed": 42,
        "tick_count": 10,
        "world": {},
        "space": {},
        "resources": {
            "passive_energy_income_placeholder": 1.0
        },
        "cell": {
            "initial_energy": 50.0,
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

def test_successful_survival(base_config):
    # Cell should survive all 10 ticks and remain stable
    history, result, reason = run_micro_simulation(base_config)
    assert len(history) == 10
    assert result == "stable"
    assert reason == "none"
    assert history[-1]["state"] == "alive"

def test_energy_depletion(base_config):
    # initial energy is low, no passive income -> energy depletes to <= 0
    base_config["cell"]["initial_energy"] = 4.0
    base_config["resources"]["passive_energy_income_placeholder"] = 0.0
    base_config["cell"]["mandatory_cost_per_tick"] = 2.0
    base_config["lifecycle"]["dormancy_allowed"] = False # prevent dormancy to force energy depletion death
    
    # Tick 1: 4 - 2 = 2
    # Tick 2: 2 - 2 = 0 (<=0) -> dead
    history, result, reason = run_micro_simulation(base_config)
    assert result == "collapse"
    assert reason == "energy_depleted"
    assert len(history) == 2
    assert history[-1]["state"] == "dead"


def test_heat_death(base_config):
    # heat generation is very high, dissipation is low, exceeds death threshold
    base_config["environment"]["heat_generated_per_tick"] = 4.0
    base_config["environment"]["heat_dissipation_rate"] = 1.0
    base_config["environment"]["heat_death_threshold"] = 5.0
    
    # Tick 1: heat = 0 + 4 - 1 = 3 (warning > 5?)
    # Tick 2: heat = 3 + 4 - 1 = 6 (> death 5) -> dead
    history, result, reason = run_micro_simulation(base_config)
    assert result == "collapse"
    assert reason == "heat_limit_exceeded"
    assert len(history) == 2
    assert history[-1]["state"] == "dead"

def test_waste_death(base_config):
    # waste exceeds death threshold
    base_config["environment"]["waste_generated_per_tick"] = 3.0
    base_config["environment"]["waste_sink_rate"] = 1.0
    base_config["environment"]["waste_death_threshold"] = 3.5
    
    # Tick 1: waste = 0 + 3 - 1 = 2
    # Tick 2: waste = 2 + 3 - 1 = 4 (> death 3.5) -> dead
    history, result, reason = run_micro_simulation(base_config)
    assert result == "collapse"
    assert reason == "waste_limit_exceeded"
    assert len(history) == 2
    assert history[-1]["state"] == "dead"

def test_transition_to_stressed_and_warning(base_config):
    # energy falls below stress threshold, triggering fragile state
    base_config["cell"]["initial_energy"] = 11.0
    base_config["resources"]["passive_energy_income_placeholder"] = 0.0
    base_config["cell"]["mandatory_cost_per_tick"] = 2.0
    base_config["lifecycle"]["stress_energy_threshold"] = 10.0
    
    # Tick 1: energy = 11 - 2 = 9 (< 10) -> stressed
    # Survives 10 ticks (since energy is 9, 7, 5, 3, 1, then goes dormant/dead? Wait, we need it to survive but be fragile.
    # Let's give it passive income of 2.0 and mandatory cost of 2.0, with energy at 8.0 (< 10).
    # Then it survives but stays at energy 8.0, which is stressed.
    base_config["cell"]["initial_energy"] = 8.0
    base_config["resources"]["passive_energy_income_placeholder"] = 2.0
    base_config["cell"]["mandatory_cost_per_tick"] = 2.0
    
    history, result, reason = run_micro_simulation(base_config)
    assert result == "fragile"
    assert reason == "none"
    assert history[-1]["state"] == "stressed"

def test_dormancy_transitions(base_config):
    # cell fails to pay mandatory cost, but dormancy is allowed and dormant cost is paid
    base_config["tick_count"] = 5
    base_config["cell"]["initial_energy"] = 3.0
    base_config["resources"]["passive_energy_income_placeholder"] = 0.0
    base_config["cell"]["mandatory_cost_per_tick"] = 4.0
    base_config["cell"]["dormant_mandatory_cost_modifier"] = 0.1 # dormant cost = 0.4
    base_config["lifecycle"]["dormancy_allowed"] = True
    
    # Tick 1: energy = 3. E_before_cost = 3. Cost = 4. Cannot pay.
    # Can pay dormant cost? E_before_cost (3) >= 0.4. Yes! Go dormant.
    # energy_next = 3 - 0.4 = 2.6. State: dormant.
    # Tick 2: energy = 2.6. State: dormant. Cost = 0.4. Can pay.
    # energy_next = 2.6 - 0.4 = 2.2. State: dormant.
    history, result, reason = run_micro_simulation(base_config)
    assert result == "fragile"  # because warning triggered (dormancy is a stress/warning indicator)
    assert reason == "none"
    assert history[0]["state"] == "dormant"
    assert history[1]["state"] == "dormant"

