import os
import copy

def set_nested_value(d: dict, key_str: str, value):
    """Sets a value in a nested dictionary using a dot-separated key string."""
    parts = key_str.split(".")
    curr = d
    for part in parts[:-1]:
        curr = curr.setdefault(part, {})
    curr[parts[-1]] = value

def serialize_val(v):
    """Serializes a Python value into TOML-compatible format."""
    if isinstance(v, bool):
        return "true" if v else "false"
    elif isinstance(v, (int, float)):
        return str(v)
    elif isinstance(v, str):
        return f'"{v}"'
    elif isinstance(v, list):
        list_items = [serialize_val(x) for x in v]
        return "[" + ", ".join(list_items) + "]"
    elif isinstance(v, dict):
        inline_parts = []
        for ik, iv in sorted(v.items()):
            inline_parts.append(f"{ik} = {serialize_val(iv)}")
        return "{ " + ", ".join(inline_parts) + " }"
    else:
        return str(v)

def dict_to_toml(d: dict) -> str:
    """Converts a dictionary to a standard TOML string representation."""
    lines = []
    
    # 1. Root level simple values
    for k, v in sorted(d.items()):
        if not isinstance(v, dict):
            lines.append(f"{k} = {serialize_val(v)}")
            
    # 2. Subtables
    for k, v in sorted(d.items()):
        if isinstance(v, dict):
            lines.append("")
            lines.append(f"[{k}]")
            for subk, subv in sorted(v.items()):
                lines.append(f"{subk} = {serialize_val(subv)}")
                
    return "\n".join(lines) + "\n"

def write_recommended_toml(output_dir: str, base_config: dict, name: str, params: dict):
    """Modifies base_config with params and writes it to recommended-configs/<name>.toml."""
    os.makedirs(os.path.join(output_dir, "recommended-configs"), exist_ok=True)
    
    config_copy = copy.deepcopy(base_config)
    for k, v in params.items():
        set_nested_value(config_copy, k, v)
        
    toml_str = dict_to_toml(config_copy)
    filepath = os.path.join(output_dir, "recommended-configs", f"{name}.toml")
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(toml_str)

def write_report_markdown(output_dir: str, report_data: dict):
    """Generates REPORT.md containing the summary and warnings in the output directory."""
    os.makedirs(output_dir, exist_ok=True)
    
    run_id = report_data.get("run_id", "unknown")
    mode = report_data.get("mode", "unknown")
    base_config_path = report_data.get("base_config_path", "unknown")
    iter_count = report_data.get("iteration_count", 0)
    
    stable = report_data.get("stable_count", 0)
    fragile = report_data.get("fragile_count", 0)
    collapse = report_data.get("collapse_count", 0)
    invalid = report_data.get("invalid_count", 0)
    
    # 1. Best stable candidate metrics
    best_stable_metrics = report_data.get("best_stable_metrics", None)
    best_stable_str = ""
    if best_stable_metrics:
        final_energy = best_stable_metrics.get("final_energy", 0.0)
        final_heat = best_stable_metrics.get("final_heat", 0.0)
        final_waste = best_stable_metrics.get("final_waste", 0.0)
        best_stable_str = f"""
## Best Stable Candidate Details
* **Final Energy**: {final_energy}
* **Final Heat**: {final_heat}
* **Final Waste**: {final_waste}
"""

    # 2. Recommended values table
    parameter_ranges = report_data.get("parameter_ranges", [])
    rec_lines = [
        "## Recommended Values Table",
        "",
        "| Parameter ID | Recommended Value |",
        "| --- | --- |"
    ]
    has_rec = False
    for r in parameter_ranges:
        rec_val = r.get("recommended", None)
        if rec_val is not None:
            has_rec = True
            rec_lines.append(f"| {r['parameter_id']} | {rec_val} |")
    if not has_rec:
        rec_lines.append("| None | N/A |")
    rec_table_str = "\n".join(rec_lines)

    # 3. Empirical ranges table
    range_lines = [
        "## Empirical Tested & Stable Ranges",
        "",
        "| Parameter | Tested Min | Tested Max | Stable Min | Stable Max | Recommended | Confidence |",
        "| --- | --- | --- | --- | --- | --- | --- |"
    ]
    for r in parameter_ranges:
        t_min = r.get("tested_min", "N/A")
        t_max = r.get("tested_max", "N/A")
        s_min = r.get("stable_min") if r.get("stable_min") is not None else "N/A"
        s_max = r.get("stable_max") if r.get("stable_max") is not None else "N/A"
        rec = r.get("recommended") if r.get("recommended") is not None else "N/A"
        conf = r.get("confidence", "none")
        range_lines.append(f"| {r['parameter_id']} | {t_min} | {t_max} | {s_min} | {s_max} | {rec} | {conf} |")
    range_table_str = "\n".join(range_lines)

    # 4. Sensitivity parameter rank
    sensitivity_list = []
    for r in parameter_ranges:
        t_min = r.get("tested_min", 0.0)
        t_max = r.get("tested_max", 0.0)
        s_min = r.get("stable_min", None)
        s_max = r.get("stable_max", None)
        
        tested_range = t_max - t_min
        if tested_range > 0:
            if s_max is not None and s_min is not None:
                stable_range = s_max - s_min
                ratio = stable_range / tested_range
                score = 1.0 - ratio
            else:
                score = 1.0
        else:
            score = 0.0
            
        sensitivity_list.append((r["parameter_id"], score))
        
    sensitivity_list.sort(key=lambda x: x[1], reverse=True)
    sens_lines = [
        "## Parameter Sensitivity Ranking",
        "",
        "| Rank | Parameter | Sensitivity Score | Note |",
        "| --- | --- | --- | --- |"
    ]
    for idx, (param_id, score) in enumerate(sensitivity_list, 1):
        note = "Extremely sensitive (no stable range)" if score == 1.0 else "Varying sensitivity"
        sens_lines.append(f"| {idx} | {param_id} | {score:.2f} | {note} |")
    sens_table_str = "\n".join(sens_lines)

    # 5. Failure reasons list
    runs = report_data.get("runs", [])
    unique_collapse_reasons = set()
    for run in runs:
        reason = run.get("collapse_reason", "none")
        if reason and reason != "none":
            unique_collapse_reasons.add(reason)
            
    failure_lines = [
        "## Failure Reasons Summary",
        ""
    ]
    if unique_collapse_reasons:
        for r in sorted(unique_collapse_reasons):
            failure_lines.append(f"* {r}")
    else:
        failure_lines.append("* None encountered")
    failure_reasons_str = "\n".join(failure_lines)

    # Warnings and limits
    warnings = report_data.get("warnings", [])
    limits = report_data.get("limits_of_evidence", [])
    warnings_str = "\n".join([f"* {w}" for w in warnings]) if warnings else "* None"
    limits_str = "\n".join([f"* {l}" for l in limits]) if limits else "* None"
    
    report_content = f"""# Stability Report: {run_id}

* **Mode**: {mode}
* **Base Config**: {base_config_path}
* **Iteration Count**: {iter_count}

## Summary of Candidates
* **Stable**: {stable}
* **Fragile**: {fragile}
* **Collapse**: {collapse}
* **Invalid**: {invalid}
{best_stable_str}
{rec_table_str}

{range_table_str}

{sens_table_str}

{failure_reasons_str}

## Warnings
{warnings_str}

## Limits of Evidence
{limits_str}
"""
    filepath = os.path.join(output_dir, "REPORT.md")
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(report_content)
