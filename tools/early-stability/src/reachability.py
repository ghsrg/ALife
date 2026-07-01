import tomllib


class ReachabilityValidationError(Exception):
    """Raised when a mechanism registry cannot be used."""


REQUIRED_MECHANISM_FIELDS = [
    "mechanism_id",
    "status",
    "required_inputs",
    "expected_effect",
    "possible_blockers",
    "bypass_risks",
    "minimum_useful_activation_rate",
    "required_scenarios",
]

VALID_STATUSES = {"now", "future", "estimate_only"}


def load_mechanism_registry(toml_str: str) -> list[dict]:
    try:
        data = tomllib.loads(toml_str)
    except tomllib.TOMLDecodeError as exc:
        raise ReachabilityValidationError(f"Invalid mechanism registry TOML: {exc}") from exc

    mechanisms = data.get("mechanisms")
    if not isinstance(mechanisms, list) or not mechanisms:
        raise ReachabilityValidationError("registry must contain at least one [[mechanisms]] entry")

    seen_ids = set()
    for mechanism in mechanisms:
        if not isinstance(mechanism, dict):
            raise ReachabilityValidationError("mechanism entry must be a table")
        for field in REQUIRED_MECHANISM_FIELDS:
            if field not in mechanism:
                raise ReachabilityValidationError(f"mechanism missing required field: {field}")

        mechanism_id = mechanism["mechanism_id"]
        if not isinstance(mechanism_id, str) or not mechanism_id:
            raise ReachabilityValidationError("mechanism_id must be a non-empty string")
        if mechanism_id in seen_ids:
            raise ReachabilityValidationError(f"duplicate mechanism_id: {mechanism_id}")
        seen_ids.add(mechanism_id)

        if mechanism["status"] not in VALID_STATUSES:
            raise ReachabilityValidationError(f"invalid mechanism status: {mechanism['status']}")

        for list_field in ["required_inputs", "possible_blockers", "bypass_risks", "required_scenarios"]:
            if not isinstance(mechanism[list_field], list):
                raise ReachabilityValidationError(f"{mechanism_id}.{list_field} must be a list")

        rate = mechanism["minimum_useful_activation_rate"]
        if not isinstance(rate, (int, float)) or isinstance(rate, bool) or rate < 0.0 or rate > 1.0:
            raise ReachabilityValidationError(
                f"{mechanism_id}.minimum_useful_activation_rate must be 0.0..1.0"
            )

    return mechanisms


def get_nested(config: dict, path: str, default=None):
    curr = config
    for part in path.split("."):
        if not isinstance(curr, dict) or part not in curr:
            return default
        curr = curr[part]
    return curr


def has_required_inputs(config: dict, mechanism: dict) -> bool:
    return all(get_nested(config, path, None) is not None for path in mechanism["required_inputs"])


def _sum_amounts(value) -> float:
    if isinstance(value, dict):
        return sum(value.values())
    if isinstance(value, list):
        return sum(value)
    return 0.0


def base_result(
    mechanism: dict,
    config: dict,
    history: list,
    scenario_result: str,
    collapse_reason: str,
) -> dict:
    available = 1 if has_required_inputs(config, mechanism) else 0
    return {
        "mechanism_id": mechanism["mechanism_id"],
        "status": mechanism["status"],
        "scenario_id": config.get("scenario_id", "unknown"),
        "available_count": available,
        "attempted_count": 0,
        "allowed_count": 0,
        "executed_count": 0,
        "blocked_count": 0,
        "effect_nonzero_count": 0,
        "bypass_detected_count": 0,
        "top_block_reason": "none",
        "reachability_result": "fail" if available else "blocked",
        "notes": "",
    }


def evaluate_mandatory_energy_cost(
    result: dict,
    config: dict,
    history: list,
    scenario_result: str,
    collapse_reason: str,
) -> dict:
    ticks = len(history)
    result["available_count"] = ticks
    result["attempted_count"] = ticks

    if collapse_reason in {"mandatory_cost_unpaid", "energy_depleted"}:
        result["blocked_count"] = 1
        result["top_block_reason"] = collapse_reason
        result["reachability_result"] = "blocked"
        result["notes"] = "Mandatory cost pressure is reachable and can block survival."
        return result

    result["allowed_count"] = ticks
    result["executed_count"] = ticks
    result["effect_nonzero_count"] = ticks
    result["reachability_result"] = "pass" if ticks > 0 else "fail"
    result["notes"] = "Mandatory cost was evaluated over simulation history."
    return result


def evaluate_passive_energy_income(result: dict, config: dict, history: list) -> dict:
    income = get_nested(config, "resources.passive_energy_income_placeholder", 0.0)
    result["available_count"] = 1

    if income <= 0:
        result["blocked_count"] = 1
        result["top_block_reason"] = "not_configured"
        result["reachability_result"] = "blocked"
        result["notes"] = "Passive energy income is zero."
        return result

    result["attempted_count"] = len(history)
    result["allowed_count"] = len(history)
    result["executed_count"] = len(history)
    result["effect_nonzero_count"] = len(history)
    mandatory = get_nested(config, "cell.mandatory_cost_per_tick", 0.0)
    if income > mandatory:
        result["bypass_detected_count"] = 1
        result["reachability_result"] = "warning"
        result["top_block_reason"] = "competing_path_cheaper"
        result["notes"] = "Passive income may bypass future uptake/metabolism pressure."
    else:
        result["reachability_result"] = "pass"
        result["notes"] = "Passive income exists but does not fully dominate mandatory cost."
    return result


def evaluate_capacity_limit(result: dict, config: dict) -> dict:
    resources = get_nested(config, "cell.initial_resources", {})
    materials = get_nested(config, "cell.initial_materials", {})
    capacity = get_nested(config, "cell.capacity_limit", 0.0)
    used = _sum_amounts(resources) + _sum_amounts(materials)

    result["available_count"] = 1
    result["attempted_count"] = 1
    if used > capacity:
        result["blocked_count"] = 1
        result["top_block_reason"] = "insufficient_capacity"
        result["reachability_result"] = "blocked"
        result["notes"] = "Initial stored amount exceeds capacity."
    elif used > 0 and capacity > used * 5:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["bypass_detected_count"] = 1
        result["top_block_reason"] = "capacity_too_high"
        result["reachability_result"] = "warning"
        result["notes"] = "Capacity limit may be too high to constrain storage."
    else:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["effect_nonzero_count"] = 1
        result["reachability_result"] = "pass"
        result["notes"] = "Capacity limit is present and can constrain stored amount."
    return result


def evaluate_heat_or_waste(result: dict, config: dict, gen_path: str, sink_path: str, label: str) -> dict:
    generated = get_nested(config, gen_path, 0.0)
    sink = get_nested(config, sink_path, 0.0)
    result["available_count"] = 1

    if generated <= 0 and sink <= 0:
        result["blocked_count"] = 1
        result["top_block_reason"] = "not_configured"
        result["reachability_result"] = "blocked"
        result["notes"] = f"{label} generation/sink are not configured."
    elif generated <= 0:
        result["blocked_count"] = 1
        result["top_block_reason"] = "effect_zero"
        result["reachability_result"] = "blocked"
        result["notes"] = f"{label} generation is zero."
    elif sink > generated * 3:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["bypass_detected_count"] = 1
        result["top_block_reason"] = "competing_path_cheaper"
        result["reachability_result"] = "warning"
        result["notes"] = f"{label} sink may be high enough to bypass pressure."
    else:
        result["allowed_count"] = 1
        result["executed_count"] = 1
        result["effect_nonzero_count"] = 1
        result["reachability_result"] = "pass"
        result["notes"] = f"{label} generation/sink pressure is reachable."
    return result


def evaluate_mechanisms(
    config: dict,
    mechanisms: list[dict],
    history: list,
    scenario_result: str,
    collapse_reason: str,
) -> list[dict]:
    results = []
    for mechanism in mechanisms:
        result = base_result(mechanism, config, history, scenario_result, collapse_reason)
        mechanism_id = mechanism["mechanism_id"]

        if mechanism["status"] == "future":
            result["reachability_result"] = "future_only"
            result["top_block_reason"] = "future_only"
            result["notes"] = "Mechanism is declared future-only."
        elif mechanism["status"] == "estimate_only":
            result["reachability_result"] = "tool_limited"
            result["top_block_reason"] = "tool_limited"
            result["notes"] = "Estimate-only mechanism is not consumed by current micro simulator."
        elif mechanism_id == "mandatory_energy_cost":
            result = evaluate_mandatory_energy_cost(result, config, history, scenario_result, collapse_reason)
        elif mechanism_id == "passive_energy_income":
            result = evaluate_passive_energy_income(result, config, history)
        elif mechanism_id == "capacity_limit":
            result = evaluate_capacity_limit(result, config)
        elif mechanism_id in {"heat_generation", "heat_dissipation"}:
            result = evaluate_heat_or_waste(
                result,
                config,
                "environment.heat_generated_per_tick",
                "environment.heat_dissipation_rate",
                "heat",
            )
        elif mechanism_id in {"waste_generation", "waste_sink"}:
            result = evaluate_heat_or_waste(
                result,
                config,
                "environment.waste_generated_per_tick",
                "environment.waste_sink_rate",
                "waste",
            )
        else:
            result["reachability_result"] = "tool_limited"
            result["top_block_reason"] = "tool_limited"
            result["notes"] = "No evaluator exists for this mechanism yet."

        results.append(result)

    return results
