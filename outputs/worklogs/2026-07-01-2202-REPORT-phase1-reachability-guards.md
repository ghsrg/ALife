# REPORT: Phase 1 Reachability Guards

Date: 2026-07-01 22:02

## Goal

Add reachability coverage for Phase 1 mechanisms that already exist in the current micro simulator but were not represented in the mechanism registry/evaluator.

## Scope

Added reachability coverage for:

- `energy_buffer_clamp`
- `stress_state`
- `dormancy`
- `death_by_energy`
- `death_by_heat`
- `death_by_waste`
- `candidate_config_validation`

No new physics was added to `micro_simulator.py`.

## Files Changed

- `tools/early-stability/src/reachability.py`
- `tools/early-stability/mechanisms/phase1.toml`
- `tools/early-stability/tests/test_reachability.py`
- `tools/early-stability/scenarios/single_cell_dormancy.toml`
- `tools/early-stability/scenarios/single_cell_heat_death.toml`
- `tools/early-stability/scenarios/single_cell_waste_death.toml`

## Verification

- RED verified: new tests failed because mechanisms were missing from registry or evaluated as `tool_limited`.
- GREEN verified: `python -m pytest .\tools\early-stability\tests\test_reachability.py -q` passed.
- Full suite verified: `python -m pytest .\tools\early-stability -q` passed.
- CLI smoke verified for:
  - `single_cell_dormancy`
  - `single_cell_heat_death`
  - `single_cell_waste_death`

## Smoke Results

`single_cell_dormancy`:

- `dormancy`: pass, observed 1 matching tick.
- `death_by_energy`: pass, observed `energy_depleted`.
- Overall report is `fail` because negative scenario intentionally blocks mandatory/passive energy paths.

`single_cell_heat_death`:

- `death_by_heat`: pass, observed `heat_limit_exceeded`.

`single_cell_waste_death`:

- `death_by_waste`: pass, observed `waste_limit_exceeded`.

## Interpretation

The Phase 1 reachability registry now covers the current micro simulator state guards and death paths.

Negative scenarios may still produce overall `fail` because the current writer treats any blocked mechanism as failing the whole run. That is acceptable for now, but a later improvement should distinguish:

- expected negative scenario block;
- unexpected mechanism block;
- target mechanism pass.

## Next Step

Rerun parameter tuning/reachability. If core positive scenarios stay warning-free and the new guard/death scenarios trigger their target mechanisms, move to Phase 1 data model and structure documentation.
