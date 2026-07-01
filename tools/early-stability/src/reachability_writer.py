import json
import os
from collections import Counter


def summarize_reachability(mechanism_results: list[dict]) -> dict:
    counts = Counter(r["reachability_result"] for r in mechanism_results)
    failed = counts["fail"] + counts["blocked"]
    warnings = counts["warning"]
    tool_limited = counts["tool_limited"] + counts["future_only"]

    if failed > 0:
        overall = "fail"
    elif tool_limited > 0:
        overall = "partial"
    elif warnings > 0:
        overall = "warning"
    else:
        overall = "pass"

    return {
        "overall_result": overall,
        "mechanism_count": len(mechanism_results),
        "passed_count": counts["pass"],
        "warning_count": warnings,
        "failed_count": counts["fail"],
        "blocked_count": counts["blocked"],
        "tool_limited_count": tool_limited,
    }


def write_json(path: str, data) -> None:
    with open(path, "w", encoding="utf-8") as f:
        json.dump(data, f, indent=2)


def write_reachability_outputs(
    output_dir: str,
    run_id: str,
    config_hash: str,
    seed: int,
    tick_count: int,
    stability_ranges_ref: str,
    mechanism_results: list[dict],
) -> None:
    os.makedirs(output_dir, exist_ok=True)
    summary = summarize_reachability(mechanism_results)
    results = {
        "run_id": run_id,
        "config_hash": config_hash,
        "seed": seed,
        "tick_count": tick_count,
        "stability_ranges_ref": stability_ranges_ref,
        **summary,
    }

    write_json(os.path.join(output_dir, "results.json"), results)
    write_json(os.path.join(output_dir, "mechanisms.json"), mechanism_results)

    block_reasons = Counter(
        r["top_block_reason"] for r in mechanism_results if r["top_block_reason"] != "none"
    )
    write_json(os.path.join(output_dir, "block-reasons.json"), dict(sorted(block_reasons.items())))

    bypass = [r for r in mechanism_results if r["bypass_detected_count"] > 0]
    write_json(os.path.join(output_dir, "bypass.json"), bypass)

    lines = [
        f"# Mechanism Reachability Report: {run_id}",
        "",
        "## Summary",
        f"* **Overall Result**: {summary['overall_result']}",
        f"* **Mechanisms**: {summary['mechanism_count']}",
        f"* **Pass**: {summary['passed_count']}",
        f"* **Warning**: {summary['warning_count']}",
        f"* **Fail**: {summary['failed_count']}",
        f"* **Blocked**: {summary['blocked_count']}",
        f"* **Tool Limited**: {summary['tool_limited_count']}",
        f"* **Stability Ranges Ref**: {stability_ranges_ref}",
        "",
        "## Mechanisms",
        "| Mechanism | Result | Block Reason | Executed | Effect Nonzero | Bypass | Notes |",
        "| --- | --- | --- | ---: | ---: | ---: | --- |",
    ]

    for r in mechanism_results:
        lines.append(
            f"| {r['mechanism_id']} | {r['reachability_result']} | {r['top_block_reason']} | "
            f"{r['executed_count']} | {r['effect_nonzero_count']} | {r['bypass_detected_count']} | "
            f"{r['notes']} |"
        )

    needs_tuning = [
        r for r in mechanism_results if r["reachability_result"] in {"warning", "fail", "blocked"}
    ]
    lines.extend(["", "## Feedback Loop"])
    if needs_tuning:
        lines.append("Return to parameter tuning before data model design.")
        lines.append("")
        lines.append("| Mechanism | Reason | Suggested Action |")
        lines.append("| --- | --- | --- |")
        for r in needs_tuning:
            lines.append(
                f"| {r['mechanism_id']} | {r['top_block_reason']} | "
                "Adjust relevant early-stability tuning group and rerun reachability. |"
            )
    else:
        lines.append("No parameter tuning loop required for currently evaluated mechanisms.")

    lines.extend([
        "",
        "## Decision",
        f"Proceed to data model docs: {'yes' if summary['overall_result'] == 'pass' else 'partial'}",
    ])

    with open(os.path.join(output_dir, "REPORT.md"), "w", encoding="utf-8") as f:
        f.write("\n".join(lines) + "\n")
