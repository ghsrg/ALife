from pathlib import Path

import pytest

from reachability import (
    ReachabilityValidationError,
    evaluate_mechanisms,
    load_mechanism_registry,
)


VALID_REGISTRY = """
[[mechanisms]]
mechanism_id = "mandatory_energy_cost"
status = "now"
required_inputs = ["cell.mandatory_cost_per_tick", "cell.initial_energy"]
expected_effect = "energy decreases when mandatory cost is paid"
possible_blockers = ["insufficient_energy", "invalid_config"]
bypass_risks = ["passive_energy_income_too_high"]
minimum_useful_activation_rate = 1.0
required_scenarios = ["single_cell_survival", "single_cell_starvation"]
"""


def sample_config():
    return {
        "scenario_id": "single_cell_survival",
        "tick_count": 3,
        "resources": {"passive_energy_income_placeholder": 5.0},
        "cell": {
            "initial_energy": 20.0,
            "mandatory_cost_per_tick": 2.0,
            "capacity_limit": 50.0,
            "initial_resources": {"water": 2.0, "nutrient": 1.0},
            "initial_materials": {"cell_wall": 5.0},
        },
        "environment": {
            "heat_generated_per_tick": 0.1,
            "heat_dissipation_rate": 0.2,
            "waste_generated_per_tick": 0.05,
            "waste_sink_rate": 0.1,
        },
        "estimates": {
            "growth_cost_estimate": 10.0,
            "joint_count_estimate": 0,
            "joint_upkeep_cost": 0.0,
        },
    }


def test_load_mechanism_registry_returns_mechanisms():
    mechanisms = load_mechanism_registry(VALID_REGISTRY)

    assert len(mechanisms) == 1
    assert mechanisms[0]["mechanism_id"] == "mandatory_energy_cost"
    assert mechanisms[0]["status"] == "now"
    assert mechanisms[0]["minimum_useful_activation_rate"] == 1.0


def test_registry_rejects_missing_mechanism_id():
    bad_registry = """
[[mechanisms]]
status = "now"
required_inputs = []
expected_effect = "missing id"
possible_blockers = []
bypass_risks = []
minimum_useful_activation_rate = 1.0
required_scenarios = []
"""
    with pytest.raises(ReachabilityValidationError):
        load_mechanism_registry(bad_registry)


def test_bundled_phase1_registry_loads():
    registry_path = Path(__file__).resolve().parents[1] / "mechanisms" / "phase1.toml"
    mechanisms = load_mechanism_registry(registry_path.read_text(encoding="utf-8"))

    ids = {m["mechanism_id"] for m in mechanisms}
    assert "mandatory_energy_cost" in ids
    assert "heat_dissipation" in ids
    assert "capacity_limit" in ids


def test_evaluate_mandatory_energy_cost_passes_when_energy_changes():
    mechanisms = load_mechanism_registry(VALID_REGISTRY)
    history = [
        {"tick": 1, "state": "alive", "energy": 23.0, "heat": 0.0, "waste": 0.0},
        {"tick": 2, "state": "alive", "energy": 26.0, "heat": 0.0, "waste": 0.0},
        {"tick": 3, "state": "alive", "energy": 29.0, "heat": 0.0, "waste": 0.0},
    ]

    results = evaluate_mechanisms(sample_config(), mechanisms, history, "stable", "none")

    mandatory = results[0]
    assert mandatory["mechanism_id"] == "mandatory_energy_cost"
    assert mandatory["reachability_result"] == "pass"
    assert mandatory["executed_count"] == 3


def test_estimate_only_mechanism_is_tool_limited():
    registry = """
[[mechanisms]]
mechanism_id = "growth_estimate"
status = "estimate_only"
required_inputs = ["estimates.growth_cost_estimate"]
expected_effect = "estimate only"
possible_blockers = ["tool_limited"]
bypass_risks = ["not_consumed_by_micro_simulator"]
minimum_useful_activation_rate = 0.0
required_scenarios = ["single_cell_growth_budget"]
"""
    mechanisms = load_mechanism_registry(registry)

    results = evaluate_mechanisms(sample_config(), mechanisms, [], "stable", "none")

    assert results[0]["reachability_result"] == "tool_limited"
    assert results[0]["top_block_reason"] == "tool_limited"
