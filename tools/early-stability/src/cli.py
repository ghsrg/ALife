import argparse
import sys
import os
import hashlib
import tomllib
import json

from config_loader import load_and_validate_config, ValidationError
from static_calculator import evaluate_static_bounds
from micro_simulator import run_micro_simulation
from tuner import run_tuning
from result_writer import write_results_json, write_ranges_json, write_run_detail_json
from report_writer import write_report_markdown, write_recommended_toml
from reachability import load_mechanism_registry, evaluate_mechanisms, ReachabilityValidationError
from reachability_writer import write_reachability_outputs

RESULT_RANK = {
    "stable": 0,
    "fragile": 1,
    "collapse": 2,
    "invalid": 3,
}

def worse_result(static_result: str, static_reason: str, sim_result: str, sim_reason: str) -> tuple[str, str]:
    if RESULT_RANK[sim_result] > RESULT_RANK[static_result]:
        return sim_result, sim_reason
    return static_result, static_reason

def final_metrics_from_history(history: list) -> dict:
    final_step = history[-1] if history else {}
    return {
        "final_energy": final_step.get("energy", 0.0),
        "final_heat": final_step.get("heat", 0.0),
        "final_waste": final_step.get("waste", 0.0),
        "final_state": final_step.get("state", "unknown"),
    }

def run_evaluate_mode(scenario_path: str, out_dir: str, with_simulation: bool = False):
    try:
        with open(scenario_path, "r", encoding="utf-8") as f:
            toml_str = f.read()
    except Exception as e:
        print(f"Error reading scenario file: {e}")
        sys.exit(1)

    config_hash = hashlib.sha256(toml_str.encode('utf-8')).hexdigest()

    try:
        config = load_and_validate_config(toml_str)
        result, reason = evaluate_static_bounds(config)
        history = []
        if with_simulation and result in ("stable", "fragile"):
            history, sim_result, sim_reason = run_micro_simulation(config)
            result, reason = worse_result(result, reason, sim_result, sim_reason)
    except ValidationError as e:
        result = "invalid"
        reason = "invalid_config"
        config = {
            "scenario_id": os.path.basename(scenario_path),
            "seed": 0,
            "tick_count": 0
        }
        history = []

    metrics_summary = {
        "warnings_triggered": (result == "fragile")
    }
    if history:
        metrics_summary.update(final_metrics_from_history(history))

    results_data = {
        "scenario_id": config.get("scenario_id", os.path.basename(scenario_path)),
        "config_hash": config_hash,
        "seed": config.get("seed", 0),
        "tick_count": config.get("tick_count", 0),
        "survival_result": result,
        "collapse_reason": reason,
        "metrics_summary": metrics_summary
    }
    
    write_results_json(out_dir, results_data)
    
    report_data = {
        "run_id": config.get("scenario_id", os.path.basename(scenario_path)),
        "mode": "evaluate",
        "iteration_count": 1,
        "stable_count": 1 if result == "stable" else 0,
        "fragile_count": 1 if result == "fragile" else 0,
        "collapse_count": 1 if result == "collapse" else 0,
        "invalid_count": 1 if result == "invalid" else 0,
        "warnings": [f"Evaluation returned collapse due to {reason}"] if result == "collapse" else [],
        "limits_of_evidence": [
            "Evaluated with static budget calculator and micro simulation."
            if with_simulation else
            "Evaluated with static budget calculator only."
        ]
    }
    write_report_markdown(out_dir, report_data)

def run_simulate_mode(scenario_path: str, ticks: int, out_dir: str):
    try:
        with open(scenario_path, "r", encoding="utf-8") as f:
            toml_str = f.read()
    except Exception as e:
        print(f"Error reading scenario file: {e}")
        sys.exit(1)

    config_hash = hashlib.sha256(toml_str.encode('utf-8')).hexdigest()

    try:
        config = load_and_validate_config(toml_str)
        if ticks is not None:
            config["tick_count"] = ticks
        history, result, reason = run_micro_simulation(config)
    except ValidationError as e:
        result = "invalid"
        reason = "invalid_config"
        config = {
            "scenario_id": os.path.basename(scenario_path),
            "seed": 0,
            "tick_count": 0
        }
        history = []

    final_energy = history[-1].get("energy", 0.0) if history else 0.0
    final_heat = history[-1].get("heat", 0.0) if history else 0.0
    final_waste = history[-1].get("waste", 0.0) if history else 0.0

    results_data = {
        "scenario_id": config.get("scenario_id", os.path.basename(scenario_path)),
        "config_hash": config_hash,
        "seed": config.get("seed", 0),
        "tick_count": config.get("tick_count", 0),
        "survival_result": result,
        "collapse_reason": reason,
        "metrics_summary": {
            "final_energy": final_energy,
            "final_heat": final_heat,
            "final_waste": final_waste,
            "warnings_triggered": (result == "fragile")
        }
    }
    
    write_results_json(out_dir, results_data)
    
    run_detail = {
        "parameters": {},
        "seed": config.get("seed", 0),
        "survival_result": result,
        "collapse_reason": reason,
        "history": history
    }
    write_run_detail_json(out_dir, 1, run_detail)
    
    report_data = {
        "run_id": config.get("scenario_id", os.path.basename(scenario_path)),
        "mode": "simulate",
        "iteration_count": 1,
        "stable_count": 1 if result == "stable" else 0,
        "fragile_count": 1 if result == "fragile" else 0,
        "collapse_count": 1 if result == "collapse" else 0,
        "invalid_count": 1 if result == "invalid" else 0,
        "warnings": [f"Simulation collapsed: {reason}"] if result == "collapse" else [],
        "limits_of_evidence": ["Evaluated with micro simulation headless runner."]
    }
    write_report_markdown(out_dir, report_data)

def run_tune_mode(scenario_path: str, tuning_path: str, out_dir: str):
    try:
        with open(scenario_path, "r", encoding="utf-8") as f:
            scenario_toml = f.read()
        base_config = load_and_validate_config(scenario_toml)
    except Exception as e:
        print(f"Error reading scenario or base config is invalid: {e}")
        sys.exit(1)

    try:
        with open(tuning_path, "rb") as f:
            tuning_config = tomllib.load(f)
    except Exception as e:
        print(f"Error reading tuning config: {e}")
        sys.exit(1)

    runs, ranges, profiles = run_tuning(base_config, tuning_config)

    for idx, run_record in enumerate(runs, 1):
        write_run_detail_json(out_dir, idx, run_record)
        
    write_ranges_json(out_dir, ranges)

    for profile_name, params in profiles.items():
        if params is not None:
            write_recommended_toml(out_dir, base_config, profile_name, params)


    stable_count = sum(1 for r in runs if r["survival_result"] == "stable")
    fragile_count = sum(1 for r in runs if r["survival_result"] == "fragile")
    collapse_count = sum(1 for r in runs if r["survival_result"] == "collapse")
    invalid_count = sum(1 for r in runs if r["survival_result"] == "invalid")
    
    overall_result = "stable" if stable_count > 0 else "collapse"

    results_data = {
        "scenario_id": base_config.get("scenario_id", "tuned_scenario"),
        "config_hash": hashlib.sha256(scenario_toml.encode('utf-8')).hexdigest(),
        "seed": base_config.get("seed", 0),
        "tick_count": base_config.get("tick_count", 0),
        "survival_result": overall_result,
        "collapse_reason": "none" if stable_count > 0 else "all_candidates_collapsed",
        "metrics_summary": {
            "total_iterations": len(runs),
            "stable_count": stable_count,
            "fragile_count": fragile_count,
            "collapse_count": collapse_count,
            "invalid_count": invalid_count
        }
    }
    write_results_json(out_dir, results_data)

    report_data = {
        "run_id": base_config.get("scenario_id", "tuned_scenario"),
        "mode": "tune",
        "base_config_path": scenario_path,
        "iteration_count": len(runs),
        "stable_count": stable_count,
        "fragile_count": fragile_count,
        "collapse_count": collapse_count,
        "invalid_count": invalid_count,
        "parameter_ranges": ranges,
        "runs": runs,
        "best_stable_metrics": next(
            (
                {
                    "final_energy": r.get("final_energy", 0.0),
                    "final_heat": r.get("final_heat", 0.0),
                    "final_waste": r.get("final_waste", 0.0),
                }
                for r in runs
                if r.get("survival_result") == "stable"
            ),
            None,
        ),
        "warnings": ["Tuning did not find any stable configurations."] if stable_count == 0 else [],
        "limits_of_evidence": ["Evaluated with deterministic parameter grid tuner."]
    }
    write_report_markdown(out_dir, report_data)

def run_reachability_mode(
    scenario_path: str,
    mechanisms_path: str,
    stability_ranges_ref: str,
    out_dir: str,
):
    try:
        with open(scenario_path, "r", encoding="utf-8") as f:
            toml_str = f.read()
        config_hash = hashlib.sha256(toml_str.encode("utf-8")).hexdigest()
        config = load_and_validate_config(toml_str)
        history, result, reason = run_micro_simulation(config)
    except ValidationError as e:
        print(f"Scenario config is invalid: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading scenario file: {e}")
        sys.exit(1)

    try:
        with open(mechanisms_path, "r", encoding="utf-8") as f:
            mechanisms = load_mechanism_registry(f.read())
    except ReachabilityValidationError as e:
        print(f"Mechanism registry is invalid: {e}")
        sys.exit(1)
    except Exception as e:
        print(f"Error reading mechanism registry: {e}")
        sys.exit(1)

    mechanism_results = evaluate_mechanisms(config, mechanisms, history, result, reason)
    write_reachability_outputs(
        out_dir,
        config.get("scenario_id", "reachability"),
        config_hash,
        config.get("seed", 0),
        config.get("tick_count", 0),
        stability_ranges_ref,
        mechanism_results,
    )

def run_batch_mode(scenarios_dir: str, out_dir: str, with_simulation: bool = False):
    if not os.path.isdir(scenarios_dir):
        print(f"Scenarios directory does not exist: {scenarios_dir}")
        sys.exit(1)
        
    filenames = sorted([f for f in os.listdir(scenarios_dir) if f.endswith(".toml")])
    
    batch_results = []
    stable_count = 0
    fragile_count = 0
    collapse_count = 0
    invalid_count = 0
    
    for fname in filenames:
        path = os.path.join(scenarios_dir, fname)
        try:
            with open(path, "r", encoding="utf-8") as f:
                toml_str = f.read()
            config = load_and_validate_config(toml_str)
            result, reason = evaluate_static_bounds(config)
            if with_simulation and result in ("stable", "fragile"):
                history, sim_result, sim_reason = run_micro_simulation(config)
                result, reason = worse_result(result, reason, sim_result, sim_reason)
            scenario_id = config.get("scenario_id", fname)
        except Exception as e:
            result = "invalid"
            reason = "invalid_config"
            scenario_id = fname
            
        if result == "stable":
            stable_count += 1
        elif result == "fragile":
            fragile_count += 1
        elif result == "collapse":
            collapse_count += 1
        elif result == "invalid":
            invalid_count += 1
            
        batch_results.append({
            "scenario_id": scenario_id,
            "file_name": fname,
            "survival_result": result,
            "collapse_reason": reason
        })
        
    results_data = {
        "scenarios": batch_results,
        "summary": {
            "stable": stable_count,
            "fragile": fragile_count,
            "collapse": collapse_count,
            "invalid": invalid_count
        }
    }
    
    write_results_json(out_dir, results_data)
    
    report_lines = [
        "# Batch Evaluation Report",
        "",
        "## Summary of Scenarios",
        f"* **Stable**: {stable_count}",
        f"* **Fragile**: {fragile_count}",
        f"* **Collapse**: {collapse_count}",
        f"* **Invalid**: {invalid_count}",
        "",
        "## Scenario List",
        "| Scenario ID | Filename | Result | Reason |",
        "| --- | --- | --- | --- |"
    ]
    for r in batch_results:
        report_lines.append(f"| {r['scenario_id']} | {r['file_name']} | {r['survival_result']} | {r['collapse_reason']} |")
        
    report_content = "\n".join(report_lines) + "\n"
    
    os.makedirs(out_dir, exist_ok=True)
    filepath = os.path.join(out_dir, "REPORT.md")
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(report_content)

def main(argv=None):
    parser = argparse.ArgumentParser(description="Early Stability Tool for ALife Simulation")
    subparsers = parser.add_subparsers(dest="command", required=True)

    # evaluate
    eval_parser = subparsers.add_parser("evaluate")
    eval_parser.add_argument("--scenario", required=True)
    eval_parser.add_argument("--out", required=True)
    eval_parser.add_argument("--with-simulation", action="store_true")

    # simulate
    sim_parser = subparsers.add_parser("simulate")
    sim_parser.add_argument("--scenario", required=True)
    sim_parser.add_argument("--ticks", type=int, default=None)
    sim_parser.add_argument("--out", required=True)

    # tune
    tune_parser = subparsers.add_parser("tune")
    tune_parser.add_argument("--scenario", required=True)
    tune_parser.add_argument("--tuning", required=True)
    tune_parser.add_argument("--out", required=True)

    # batch
    batch_parser = subparsers.add_parser("batch")
    batch_parser.add_argument("--scenarios", required=True)
    batch_parser.add_argument("--out", required=True)
    batch_parser.add_argument("--with-simulation", action="store_true")

    # reachability
    reach_parser = subparsers.add_parser("reachability")
    reach_parser.add_argument("--scenario", required=True)
    reach_parser.add_argument("--mechanisms", required=True)
    reach_parser.add_argument("--stability-ranges-ref", required=True)
    reach_parser.add_argument("--out", required=True)

    args = parser.parse_args(argv)

    if args.command == "evaluate":
        run_evaluate_mode(args.scenario, args.out, args.with_simulation)
    elif args.command == "simulate":
        run_simulate_mode(args.scenario, args.ticks, args.out)
    elif args.command == "tune":
        run_tune_mode(args.scenario, args.tuning, args.out)
    elif args.command == "batch":
        run_batch_mode(args.scenarios, args.out, args.with_simulation)
    elif args.command == "reachability":
        run_reachability_mode(
            args.scenario,
            args.mechanisms,
            args.stability_ranges_ref,
            args.out,
        )

if __name__ == "__main__":
    main()
