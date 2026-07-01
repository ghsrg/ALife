import json

from cli import main
from test_cli import VALID_SCENARIO_TOML
from test_reachability import VALID_REGISTRY


def test_cli_reachability_writes_artifacts(tmp_path):
    scenario = tmp_path / "scenario.toml"
    scenario.write_text(VALID_SCENARIO_TOML, encoding="utf-8")
    registry = tmp_path / "mechanisms.toml"
    registry.write_text(VALID_REGISTRY, encoding="utf-8")
    out_dir = tmp_path / "reachability"

    main([
        "reachability",
        "--scenario", str(scenario),
        "--mechanisms", str(registry),
        "--stability-ranges-ref", "outputs/stability/test",
        "--out", str(out_dir),
    ])

    assert (out_dir / "results.json").exists()
    assert (out_dir / "mechanisms.json").exists()
    assert (out_dir / "REPORT.md").exists()

    results = json.loads((out_dir / "results.json").read_text(encoding="utf-8"))
    assert results["stability_ranges_ref"] == "outputs/stability/test"
    assert results["mechanism_count"] == 1
