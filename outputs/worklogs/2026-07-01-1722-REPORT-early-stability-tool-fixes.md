# REPORT: Early Stability Tool Fixes

Date: 2026-07-01 17:22

## Goal

Execute `outputs/worklogs/2026-07-01-1708-PLAN-early-stability-tool-fix-implementation.md` with TDD-oriented changes for `tools/early-stability/`.

## Changes

- Added tests for tune report sections, tune run history/final metrics, strict tuning config validation, `evaluate --with-simulation`, and stricter scenario validation.
- Updated `tuner.py`:
  - added `TuningValidationError`;
  - requires explicit `allowed_parameters`;
  - rejects missing ranges, extra ranges, invalid ranges, and unknown non-`estimates.*` paths;
  - writes `history`, `final_energy`, `final_heat`, `final_waste`, and `final_state` into each run record.
- Updated `cli.py`:
  - tune report now receives `base_config_path`, `parameter_ranges`, `runs`, and `best_stable_metrics`;
  - added `--with-simulation` to `evaluate` and `batch`;
  - optional simulation keeps the worse result between static and micro simulation.
- Updated `config_loader.py`:
  - added required numeric/boolean field validation for Phase 1 fields;
  - validates `world.boundary_mode`;
  - validates optional `[estimates]` fields when the section exists.
- Added `tools/early-stability/tests/helpers.py` for TOML mutation/serialization in tests.
- Rewrote `tools/early-stability/README.md` in clean ASCII, documented install, console script, `--with-simulation`, strict tuning config, and generated outputs.
- Updated `tools/early-stability/pyproject.toml` with `early-stability` console script and explicit setuptools `src` module mapping.
- Updated `.gitignore` policy:
  - track `outputs/worklogs/*.md`;
  - ignore generated `outputs/stability/`;
  - ignore `.idea/`.

## Verification

Passed:

```powershell
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' -m pytest .\tools\early-stability\tests\test_tuner.py::test_tune_runs_include_history_and_final_metrics -q
```

Result:

```text
.                                                                        [100%]
```

Passed:

```powershell
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' -m pytest .\tools\early-stability
```

Result:

```text
74 passed in 2.55s
```

Passed CLI smoke commands:

```powershell
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' .\tools\early-stability\src\cli.py evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\review_evaluate
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' .\tools\early-stability\src\cli.py evaluate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --out .\outputs\stability\review_evaluate_sim --with-simulation
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' .\tools\early-stability\src\cli.py simulate --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --ticks 20 --out .\outputs\stability\review_simulate
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' .\tools\early-stability\src\cli.py tune --scenario .\tools\early-stability\scenarios\single_cell_survival.toml --tuning .\tools\early-stability\tuning\single_cell.toml --out .\outputs\stability\review_tune
& 'C:\Users\korsr\AppData\Local\Programs\Python\Python311\python.exe' .\tools\early-stability\src\cli.py batch --scenarios .\tools\early-stability\scenarios --out .\outputs\stability\review_batch --with-simulation
```

Verified key artifacts:

```text
outputs/stability/review_tune/results.json
outputs/stability/review_tune/ranges.json
outputs/stability/review_tune/REPORT.md
outputs/stability/review_tune/runs/run_0001.json
outputs/stability/review_tune/recommended-configs/fragile_edge.toml
```

The current tuning grid produced `stable_count: 0`, `fragile_count: 240`, and `collapse_count: 60`, so `best_stable.toml` was correctly not generated for this scenario/range.

`review_tune/REPORT.md` contains:

- `Recommended Values Table`
- `Empirical Tested & Stable Ranges`
- `Parameter Sensitivity Ranking`
- `Failure Reasons Summary`
- tuned parameter rows such as `cell.initial_energy`

`review_tune/runs/run_0001.json` contains:

- `history`
- `final_energy`
- `final_heat`
- `final_waste`
- `final_state`

## Remaining Risk

- Existing unrelated files outside this task were not modified: `docs/implementation/architecture.md`, `Cargo.toml`, `Cargo.lock`, `src/`, older untracked worklogs.
