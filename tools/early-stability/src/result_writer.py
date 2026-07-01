import os
import json

def write_results_json(output_dir: str, data: dict):
    """Writes results.json to the output directory."""
    os.makedirs(output_dir, exist_ok=True)
    filepath = os.path.join(output_dir, "results.json")
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)

def write_ranges_json(output_dir: str, ranges: list):
    """Writes ranges.json to the output directory."""
    os.makedirs(output_dir, exist_ok=True)
    filepath = os.path.join(output_dir, "ranges.json")
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(ranges, f, indent=2)

def write_run_detail_json(output_dir: str, run_index: int, run_data: dict):
    """Writes runs/run_XXXX.json to the output directory."""
    runs_dir = os.path.join(output_dir, "runs")
    os.makedirs(runs_dir, exist_ok=True)
    filepath = os.path.join(runs_dir, f"run_{run_index:04d}.json")
    with open(filepath, "w", encoding="utf-8") as f:
        json.dump(run_data, f, indent=2)
