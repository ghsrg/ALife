---
tags:
  - alife
  - worklog/report
  - phase/2d
  - sweep-analyzer
---

# Phase 2D Division Sweep Calibration Report

## Summary

`division_viability` was recalibrated from "mechanism activates but every run collapses" to a clean survival probe for Phase 2D division reachability.

The core division lifecycle was not rewritten. Changes are limited to analyzer preset support, warning classification, and the analyzer scenario configuration.

## Changes

- Added analyzer scenario preset fields:
  - `division_energy_cost`
  - `max_uptake_per_tick`
  - `metabolism_resource_per_tick`
- Mapped these fields into `RuntimeConfig` in `sweep_analyzer`.
- Updated `division_viability` warning logic:
  - do not emit `LOW_INFORMATION_SWEEP` when the scenario has a survived run with division successes, births, and non-zero division energy cost.
- Recalibrated `[scenarios.division_viability]`:
  - `decay_rate = 0.0`
  - `max_uptake_per_tick = 0.55`
  - `metabolism_resource_per_tick = 0.5`
  - `division_energy_cost = 10.0`
  - capacity and environmental thresholds raised only inside this scenario preset.
- Added tests for:
  - parser support for new preset fields;
  - division-specific warning behavior;
  - a real analyzer sweep with at least one clean dividing survivor.

## Acceptance Evidence

From `outputs/raw_data/division_viability.csv` after running the full analyzer:

```text
survived_to_end = true
divisions_count = 15
births_count = 15
division_attempts = 15
division_successes = 15
energy_spent_division = 150.0000
death_reason = none
warning_codes = BALANCE_ERROR
```

`LOW_INFORMATION_SWEEP` is no longer present for `division_viability`.

## Remaining Caveat

`BALANCE_ERROR` remains in the division CSV. This is separate from the requested division sweep calibration and appears tied to analyzer accounting under repeated growth/division, not to division activation or survival. It should be handled as a separate accounting-audit task if needed.

## Verification

```powershell
cargo fmt
cargo test --test phase2_sweep_parser --test phase2_sweep_outputs --test phase2_sweep_warnings
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
cargo test --workspace --all-targets
```

Results:

- targeted sweep tests passed;
- full analyzer completed successfully;
- workspace tests passed.

## Phase Gate

Phase 2D division is still reachability-complete.

Phase 2D division sweep is now clean enough to prove:

- division activates;
- births are recorded;
- at least one analyzer condition survives to configured tick end;
- division energy cost is non-zero and observed;
- `LOW_INFORMATION_SWEEP` no longer hides the signal.

This does not claim full population-level reproductive balance. It only clears the minimal Phase 2D gate before deeper Phase 2E material-profile effects.
