---
tags:
  - alife
  - worklog/report
  - phase/2E
  - material-profiles
---

# Phase 2E Material Profile Coverage Report

## Status

Phase 2E material profiles are now mechanism-measurable.

Implemented scope:

- numeric material composition and capability levels in core;
- material-effect config and TOML parsing;
- material strength effects for uptake, metabolism, storage capacity, growth and contractility;
- sensory observer/debug metric;
- explicit repair placeholder/tool-limited diagnostics;
- canonical profile and baseline configs under `config/`;
- analyzer artifacts for material profile summary and coverage;
- observer regression for transport-rich observed role evidence.

Not claimed:

- full chemical material-specific balance;
- full repair/damage model;
- full boundary retention/leakage model;
- Genome-driven material profile evolution.

## Core Changes

- Added `src/core/materials.rs` with canonical 9 material slots and deterministic `MaterialComposition`.
- Added `CellStore` helpers:
  - `material_amount_for_slot`;
  - `material_composition`;
  - `capability_level`;
  - effective capacity/free capacity with storage bonus.
- Added `MaterialEffectConfig` to `RuntimeConfig` and `config_hash`.
- Added `[material_effects]` TOML parsing.
- Applied material effects:
  - transport material scales resource uptake above baseline;
  - metabolic material scales conversion above baseline;
  - storage material increases effective capacity;
  - structural material scales growth output;
  - contractile material scales deterministic displacement;
  - sensory material increases `sensory_input_accumulated`;
  - repair material sets explicit placeholder/tool-limited diagnostics.

Compatibility note: process execution treats any positive material amount below `1.0` as baseline strength for legacy fixtures, while raw `capability_level` remains the actual material amount for analysis.

## Configs Added

- `config/material_profiles/phase2_profiles.toml`
- `config/scenarios/material_profiles/material_profile_baseline.toml`
- `config/scenarios/material_profiles/material_profile_negative_controls.toml`
- `config/analyzer/material_profile_sweeps.toml`

The baseline uses 10 cells, moderate resource flow, nonzero upkeep, explicit heat/waste sinks, and equal non-material conditions. It is not copied from Phase 2D probe configs.

## Analyzer Evidence

Generated:

- `outputs/raw_data/material_profile_summary.csv`
- `outputs/raw_data/material_profile_coverage.csv`
- `outputs/reports/material-profile-coverage-1783720357.md`

Observed material-profile signals:

- `transport_rich` has higher `resource_absorbed`;
- `metabolic_rich` has higher `energy_produced`;
- `storage_rich` reports higher capacity support;
- `structural_rich` has higher growth output;
- `contractile_rich` reports contractile activation basis;
- `sensory_rich` has higher `sensory_input_accumulated`;
- `repair_rich` is explicit `TOOL_LIMITED_REPAIR`;
- `boundary_rich` is explicit `TOOL_LIMITED_BOUNDARY_RETENTION`.

## Phase 2D Gate

Rechecked after Phase 2E changes:

- `decomposition_viability`: `warning_codes=none`; `time_to_decomposed` changes from 19 ticks at rate 1.0 to 3 ticks at rate 5.0; released-per-tick increases from 1.1751 to 5.8752.
- `division_viability`: all rows survive to end after division-specific heat envelope calibration; `divisions_count > 0`, `births_count > 0`, and `energy_spent_division > 0`.

Known warning:

- `BALANCE_ERROR` can still appear in division-style analyzer rows as an accounting-audit signal. It does not block Phase 2E material-profile claims because those claims do not depend on the failed accounting term.

## Verification

Passed:

```powershell
cargo test --test phase2_material_profile_gating --test phase2_material_profile_effects --test phase2_material_profile_analyzer --test phase2_observer_role_classifier
cargo test --test phase2_sweep_parser --test phase2_sweep_outputs --test phase2_sweep_warnings
cargo run --bin sweep_analyzer -- config/analyzer/material_profile_sweeps.toml
cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml
cargo fmt --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-targets
```

## Phase Gate

Phase 2E can proceed as material-profile coverage complete for current Phase 2 mechanics.

Phase 2F can start with this caveat: repair and boundary retention are intentionally tool-limited placeholders, so any Phase 2F scenario that depends on real damage repair or selective boundary leakage must first implement those mechanisms.
