import copy
import itertools
from micro_simulator import run_micro_simulation

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

def run_tuning(base_config: dict, tuning_config: dict) -> tuple[list, list]:
    """
    Performs deterministic grid search parameter optimization.
    
    Returns:
        tuple[list, list]: (runs, ranges)
    """
    tuning_sect = tuning_config.get("tuning", {})
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
    stable_candidates = []
    evaluated_candidates = []

    for candidate in candidates:
        evaluated_candidates.append(candidate)
        candidate_stable = True
        
        # Test candidate across all seeds
        for seed in seeds:
            config_copy = copy.deepcopy(base_config)
            config_copy["seed"] = seed
            
            # Set nested parameters
            for param, val in candidate.items():
                set_nested_value(config_copy, param, val)
                
            # Run simulation
            _, result, reason = run_micro_simulation(config_copy)
            
            # Record run
            runs.append({
                "parameters": candidate,
                "seed": seed,
                "survival_result": result,
                "collapse_reason": reason
            })
            
            if result != "stable":
                candidate_stable = False
                
        if candidate_stable:
            stable_candidates.append(candidate)
            if objective == "find_first_stable":
                break

    # Compute tested and stable ranges for each parameter
    parameter_ranges = []
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

    return runs, parameter_ranges
