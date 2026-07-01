import copy
import itertools
from micro_simulator import run_micro_simulation

class TuningValidationError(Exception):
    """Raised when a tuning configuration is structurally unsafe to execute."""
    pass

def set_nested_value(d: dict, key_str: str, value):
    """Sets a value in a nested dictionary using a dot-separated key string."""
    parts = key_str.split(".")
    curr = d
    for part in parts[:-1]:
        curr = curr.setdefault(part, {})
    curr[parts[-1]] = value

def generate_values(range_spec: list) -> list:
    """Generates candidate values from a range spec [start, end, step]."""
    if len(range_spec) < 3:
        return [range_spec[0]] if range_spec else []
    
    start, end, step = range_spec[0], range_spec[1], range_spec[2]
    if step <= 0 or start == end:
        return [start]
        
    values = []
    curr = start
    epsilon = step * 0.0001
    while curr <= end + epsilon:
        values.append(curr)
        curr += step
    return values

def path_exists(d: dict, key_str: str) -> bool:
    parts = key_str.split(".")
    curr = d
    for part in parts:
        if not isinstance(curr, dict) or part not in curr:
            return False
        curr = curr[part]
    return True

def validate_tuning_config(base_config: dict, tuning_config: dict) -> dict:
    tuning_sect = tuning_config.get("tuning", {})
    ranges_spec = tuning_sect.get("ranges", {})
    allowed_params = tuning_sect.get("allowed_parameters")

    if not isinstance(tuning_sect, dict):
        raise TuningValidationError("tuning must be a table")
    if not isinstance(ranges_spec, dict) or not ranges_spec:
        raise TuningValidationError("tuning.ranges must be a non-empty table")
    if allowed_params is None or not isinstance(allowed_params, list) or not allowed_params:
        raise TuningValidationError("tuning.allowed_parameters must be a non-empty list")

    allowed_set = set(allowed_params)
    range_set = set(ranges_spec.keys())

    missing_ranges = allowed_set - range_set
    if missing_ranges:
        raise TuningValidationError(f"allowed parameters missing ranges: {sorted(missing_ranges)}")

    extra_ranges = range_set - allowed_set
    if extra_ranges:
        raise TuningValidationError(f"ranges include parameters not allowed: {sorted(extra_ranges)}")

    for param in allowed_params:
        if not isinstance(param, str):
            raise TuningValidationError("allowed parameter names must be strings")
        if not path_exists(base_config, param) and not param.startswith("estimates."):
            raise TuningValidationError(f"parameter path does not exist in base config: {param}")

    for param, spec in ranges_spec.items():
        if not isinstance(spec, list) or len(spec) != 3:
            raise TuningValidationError(f"range for {param} must be [start, end, step]")
        start, end, step = spec
        if not all(isinstance(v, (int, float)) and not isinstance(v, bool) for v in [start, end, step]):
            raise TuningValidationError(f"range for {param} must contain numbers")
        if step <= 0:
            raise TuningValidationError(f"range step for {param} must be positive")
        if end < start:
            raise TuningValidationError(f"range end for {param} must be greater than or equal to start")

    return tuning_sect

def run_tuning(base_config: dict, tuning_config: dict) -> tuple[list, list, dict]:
    """
    Performs deterministic grid search parameter optimization.
    
    Returns:
        tuple[list, list, dict]: (runs, ranges, profiles)
    """
    tuning_sect = validate_tuning_config(base_config, tuning_config)
    max_iterations = tuning_sect.get("max_iterations", 100)
    seeds = tuning_sect.get("seeds", [42])
    objective = tuning_sect.get("objective", "map_stable_ranges")
    ranges_spec = tuning_sect.get("ranges", {})

    sorted_params = sorted(ranges_spec.keys())
    param_values = [generate_values(ranges_spec[p]) for p in sorted_params]
    
    # Generate Cartesian product
    grid = list(itertools.product(*param_values))
    
    candidates = []
    for val_tuple in grid:
        candidate = {}
        for i, param in enumerate(sorted_params):
            candidate[param] = val_tuple[i]
        candidates.append(candidate)

    # Restrict to max_iterations
    candidates = candidates[:max_iterations]

    runs = []
    evaluated_candidates = []
    candidate_evals = []

    for candidate in candidates:
        evaluated_candidates.append(candidate)
        
        all_seeds_stable = True
        all_seeds_survived = True
        
        final_energies = []
        final_heats = []
        final_wastes = []
        safety_scores = []
        
        candidate_runs = []
        
        for seed in seeds:
            config_copy = copy.deepcopy(base_config)
            config_copy["seed"] = seed
            
            # Set nested parameters
            for param, val in candidate.items():
                set_nested_value(config_copy, param, val)
                
            # Run simulation
            history, result, reason = run_micro_simulation(config_copy)
            
            final_step = history[-1] if history else {}
            run_record = {
                "parameters": copy.deepcopy(candidate),
                "seed": seed,
                "survival_result": result,
                "collapse_reason": reason,
                "history": history,
                "final_energy": final_step.get("energy", 0.0),
                "final_heat": final_step.get("heat", 0.0),
                "final_waste": final_step.get("waste", 0.0),
                "final_state": final_step.get("state", "unknown")
            }
            runs.append(run_record)
            candidate_runs.append(run_record)
            
            if result != "stable":
                all_seeds_stable = False
            if result == "collapse" or result == "invalid":
                all_seeds_survived = False
                
            final_energies.append(final_step.get("energy", 0.0))
            final_heats.append(final_step.get("heat", 0.0))
            final_wastes.append(final_step.get("waste", 0.0))
            
            # Track safety margin: E - stress_threshold, warning - H, warning - W
            stress_energy_threshold = config_copy.get("lifecycle", {}).get("stress_energy_threshold", 0.0)
            heat_warning_threshold = config_copy.get("environment", {}).get("heat_warning_threshold", 0.0)
            waste_warning_threshold = config_copy.get("environment", {}).get("waste_warning_threshold", 0.0)
            
            min_energy_margin = min((step["energy"] - stress_energy_threshold) for step in history) if history else 0.0
            min_heat_margin = min((heat_warning_threshold - step["heat"]) for step in history) if history else 0.0
            min_waste_margin = min((waste_warning_threshold - step["waste"]) for step in history) if history else 0.0
            
            run_safety_score = min_energy_margin + min_heat_margin + min_waste_margin
            safety_scores.append(run_safety_score)
            
        avg_final_energy = sum(final_energies) / len(final_energies) if final_energies else 0.0
        avg_final_heat = sum(final_heats) / len(final_heats) if final_heats else 0.0
        avg_final_waste = sum(final_wastes) / len(final_wastes) if final_wastes else 0.0
        avg_safety_score = sum(safety_scores) / len(safety_scores) if safety_scores else 0.0
        
        is_fragile_candidate = any(r["survival_result"] == "fragile" for r in candidate_runs)
        
        candidate_evals.append({
            "parameters": candidate,
            "all_seeds_stable": all_seeds_stable,
            "all_seeds_survived": all_seeds_survived,
            "avg_final_energy": avg_final_energy,
            "avg_final_heat": avg_final_heat,
            "avg_final_waste": avg_final_waste,
            "avg_safety_score": avg_safety_score,
            "is_fragile_candidate": is_fragile_candidate
        })
        
        if all_seeds_stable and objective == "find_first_stable":
            break

    # Compute tested and stable ranges for each parameter
    parameter_ranges = []
    stable_candidates = [ce["parameters"] for ce in candidate_evals if ce["all_seeds_stable"]]
    
    for param in sorted_params:
        tested_vals = [c[param] for c in evaluated_candidates]
        stable_vals = [c[param] for c in stable_candidates]
        
        tested_min = min(tested_vals) if tested_vals else None
        tested_max = max(tested_vals) if tested_vals else None
        stable_min = min(stable_vals) if stable_vals else None
        stable_max = max(stable_vals) if stable_vals else None
        
        if stable_min is not None:
            recommended = (stable_min + stable_max) / 2.0
            confidence = "high"
            notes = f"Stable range identified between {stable_min} and {stable_max}"
        else:
            recommended = None
            confidence = "none"
            notes = "No stable configuration found in the tested range"
            
        parameter_ranges.append({
            "parameter_id": param,
            "tested_min": tested_min,
            "tested_max": tested_max,
            "stable_min": stable_min,
            "stable_max": stable_max,
            "recommended": recommended,
            "confidence": confidence,
            "notes": notes
        })

    # Profile selection
    best_stable = None
    conservative_stable = None
    fragile_edge = None

    stable_evals = [ce for ce in candidate_evals if ce["all_seeds_stable"]]
    if stable_evals:
        # best_stable: highest final energy / lowest heat & waste
        # Score = avg_final_energy - avg_final_heat - avg_final_waste
        stable_evals_best = sorted(stable_evals, key=lambda ce: ce["avg_final_energy"] - ce["avg_final_heat"] - ce["avg_final_waste"], reverse=True)
        best_stable = stable_evals_best[0]["parameters"]
        
        # conservative_stable: stable candidate maximizing safety margins (avg_safety_score)
        stable_evals_cons = sorted(stable_evals, key=lambda ce: ce["avg_safety_score"], reverse=True)
        conservative_stable = stable_evals_cons[0]["parameters"]

    # fragile_edge: survived (stable/fragile) but triggered warning thresholds, closest to collapse (lowest safety margin)
    survived_fragile_evals = [ce for ce in candidate_evals if ce["all_seeds_survived"] and ce["is_fragile_candidate"]]
    if survived_fragile_evals:
        survived_fragile_evals_sorted = sorted(survived_fragile_evals, key=lambda ce: ce["avg_safety_score"])
        fragile_edge = survived_fragile_evals_sorted[0]["parameters"]

    profiles = {
        "best_stable": best_stable,
        "conservative_stable": conservative_stable,
        "fragile_edge": fragile_edge
    }

    return runs, parameter_ranges, profiles
