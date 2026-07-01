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
    iter_count = report_data.get("iteration_count", 0)
    
    stable = report_data.get("stable_count", 0)
    fragile = report_data.get("fragile_count", 0)
    collapse = report_data.get("collapse_count", 0)
    invalid = report_data.get("invalid_count", 0)
    
    warnings = report_data.get("warnings", [])
    limits = report_data.get("limits_of_evidence", [])
    
    # Format lists
    warnings_str = "\n".join([f"* {w}" for w in warnings]) if warnings else "* None"
    limits_str = "\n".join([f"* {l}" for l in limits]) if limits else "* None"
    
    report_content = f"""# Stability Report: {run_id}

* **Mode**: {mode}
* **Iteration Count**: {iter_count}

## Summary of Candidates
* **Stable**: {stable}
* **Fragile**: {fragile}
* **Collapse**: {collapse}
* **Invalid**: {invalid}

## Warnings
{warnings_str}

## Limits of Evidence
{limits_str}
"""
    filepath = os.path.join(output_dir, "REPORT.md")
    with open(filepath, "w", encoding="utf-8") as f:
        f.write(report_content)
