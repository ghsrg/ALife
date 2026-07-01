# REPORT: Early Stability Candidate Revalidation

Date: 2026-07-01 19:09

## Goal

Ensure `tools/early-stability` revalidates every mutated tuning candidate before running the micro simulator.

## Changes

- Added candidate revalidation in `tools/early-stability/src/tuner.py`.
- Added `validate_config_dict()` in `tools/early-stability/src/config_loader.py` so parsed config dicts can be validated after mutation.
- Invalid mutated candidates now produce:
  - `survival_result = invalid`
  - `collapse_reason = invalid_config`
  - `history = []`
- Added `invalid_count` to tune `results.json` metrics summary in `tools/early-stability/src/cli.py`.
- Added regression tests:
  - tuner does not call micro simulator for invalid mutated candidates;
  - CLI tune `results.json` includes `invalid_count`.
- Updated manuals:
  - `docs/implementation/early-stability-tool.md`
  - `docs/implementation/early-stability-parameter-tuning.md`

## Verification

Focused RED/GREEN tests:

```powershell
python -m pytest .\tools\early-stability\tests\test_tuner.py::test_tuning_revalidates_mutated_candidate_before_simulation -q
python -m pytest .\tools\early-stability\tests\test_cli.py::test_cli_tune_results_include_invalid_count_for_invalid_candidates -q
```

Full suite:

```powershell
python -m pytest .\tools\early-stability
```

Result:

```text
76 passed in 0.59s
```

Capacity revalidation sweep:

```powershell
python .\tools\early-stability\src\cli.py tune --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --tuning .\tools\early-stability\tuning\group3_capacity.toml --out .\outputs\stability\group3_capacity_revalidated
```

Result:

```json
{
  "stable_count": 639,
  "fragile_count": 0,
  "collapse_count": 0,
  "invalid_count": 90
}
```

## Interpretation

Previous capacity/material fragile candidates were actually invalid mutated configs. The tool now separates invalid config boundaries from valid-but-fragile runtime behavior.

This makes future min/max parameter ranges more useful for:

- future UI slider bounds;
- config warnings before simulation starts;
- Rust model/data-structure documentation;
- boundary test fixtures.
