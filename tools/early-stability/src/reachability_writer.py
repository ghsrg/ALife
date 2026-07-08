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


import csv
from pathlib import Path


def write_mechanism_coverage_outputs(root_dir: str, timestamp: str, coverage: list[dict]) -> None:
    root = Path(root_dir)
    raw_dir = root / "raw_data"
    reports_dir = root / "reports"
    raw_dir.mkdir(parents=True, exist_ok=True)
    reports_dir.mkdir(parents=True, exist_ok=True)

    fields = [
        "mechanism_id",
        "category",
        "introduced_in_phase",
        "registered",
        "enabled",
        "activation_scenario",
        "isolated_test",
        "integration_test",
        "raw_metrics",
        "cost_metrics",
        "benefit_metrics",
        "balance_sweep",
        "status",
        "warning_codes",
    ]

    csv_path = raw_dir / "mechanism_coverage.csv"
    with csv_path.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        for row in coverage:
            serializable = {**row, "warning_codes": ",".join(row.get("warning_codes", []))}
            writer.writerow(serializable)

    json_path = reports_dir / f"mechanism-coverage-{timestamp}.json"
    write_json(str(json_path), coverage)

    lines = [
        f"# Mechanism Coverage Report: {timestamp}",
        "",
        "| Mechanism | Status | Warnings |",
        "| --- | --- | --- |",
    ]
    for row in coverage:
        warnings = ", ".join(row.get("warning_codes", [])) or "none"
        lines.append(f"| {row['mechanism_id']} | {row['status']} | {warnings} |")

    md_path = reports_dir / f"mechanism-coverage-{timestamp}.md"
    md_path.write_text("\n".join(lines) + "\n", encoding="utf-8")

    phase_mechanism_delta = raw_dir / "phase_mechanism_delta.csv"
    with phase_mechanism_delta.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["introduced_in_phase", "mechanism_id", "category", "status"])
        writer.writeheader()
        for row in coverage:
            writer.writerow(
                {
                    "introduced_in_phase": row["introduced_in_phase"],
                    "mechanism_id": row["mechanism_id"],
                    "category": row["category"],
                    "status": row["status"],
                }
            )

    phase_test_coverage_delta = raw_dir / "phase_test_coverage_delta.csv"
    with phase_test_coverage_delta.open("w", encoding="utf-8", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=["mechanism_id", "isolated_test", "integration_test", "balance_sweep"])
        writer.writeheader()
        for row in coverage:
            writer.writerow(
                {
                    "mechanism_id": row["mechanism_id"],
                    "isolated_test": row["isolated_test"],
                    "integration_test": row["integration_test"],
                    "balance_sweep": row["balance_sweep"],
                }
            )

    impact_lines = ["# Phase Balance Impact", ""]
    for row in coverage:
        impact_lines.append(f"- {row['mechanism_id']}: {row['status']}")
    (reports_dir / "phase_balance_impact.md").write_text("\n".join(impact_lines) + "\n", encoding="utf-8")

    rerun_lines = [f"# Recommended Reruns: {timestamp}", ""]
    for row in coverage:
        if row["status"] != "covered":
            rerun_lines.append(f"- {row['mechanism_id']}: add activation scenario and rerun related sweeps.")
    (reports_dir / f"recommended-reruns-{timestamp}.md").write_text(
        "\n".join(rerun_lines) + "\n",
        encoding="utf-8",
    )


