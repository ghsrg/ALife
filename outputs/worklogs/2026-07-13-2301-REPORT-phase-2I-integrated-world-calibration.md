---
tags:
  - alife
  - phase-2I
  - integrated-world-calibration
  - report
---

# Phase 2I Integrated World Calibration Report

## Scope

Executed the Phase 2I TDD plan for integrated world calibration.

The goal was not ecosystem balance. The implemented gate establishes one reproducible baseline world where resources, reactions, heat, repair, fragments, joints, and lifecycle all operate together without false analyzer warnings or untracked matter movement.

## Implemented Changes

- Added integrated matter accounting across world resources, typed/internal cell resources, cell materials, material fragments, joint materials, and explicit sinks.
- Fixed typed/generic resource accounting paths in division, decomposition, growth, synthesis, metabolism, joint creation, joint transfer, and local contact transfer.
- Added repair resource sink accounting and made material type decay create material damage state so repair has a real target.
- Added no-op suppression coverage for repair-without-damage and duplicate joint creation.
- Added joint resource transfer audit metrics: gross, net, endpoint final amounts, and backflow.
- Added world scenario configs:
  - `config/scenarios/world/world_baseline_stable.toml`
  - `config/scenarios/world/world_mechanism_showcase.toml`
  - `config/scenarios/world/world_stress_regression.toml`
- Added integrated world 10k gate tests for baseline survival, deterministic replay, and deterministic stress failure.
- Added `scenario_path` support to `sweep_analyzer` presets so analyzer smoke/full can run the calibrated world config directly.
- Added `integrated_world_baseline.csv` to smoke/full analyzer outputs.
- Added analyzer CSV columns:
  - `integrated_matter_before`
  - `integrated_matter_after`
  - `integrated_matter_unclassified_loss`
  - `integrated_matter_unclassified_gain`
- Made analyzer warning logic scenario-aware for integrated world activity and integrated matter accounting.
- Updated the Phase 2H joint creation test expectation for distant cells: no contact candidate is not a rejected joint creation attempt.

## Gate Results

`world_baseline_stable.toml` 10k gate:

- survives 10,000 ticks with bounded population;
- deterministic replay matches;
- has active diffusion, repair, fragments, joint creation, and joint degradation;
- includes death/decomposition activity;
- ends with clean integrated matter accounting within the Phase 2I f32 tolerance.

Analyzer smoke:

- generated `outputs/raw_data/smoke/integrated_world_baseline.csv`;
- all integrated rows have `warning_codes=none`;
- raw data includes integrated matter columns and mechanism activity metrics.

## Verification

Passed:

- `cargo fmt --check`
- `cargo test --test phase2i_accounting --test phase2i_noop_candidates --test phase2i_analyzer_warnings --test phase2i_joint_transfer_audit --test phase2i_world_configs --test phase2i_integrated_world`
- `cargo test --test phase2g_heat_boundary_repair --test phase2g_tick_integration --test phase2h_joint_channels --test phase2h_joint_lifecycle --test phase2h_reachability`
- `cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer_smoke.toml`
- `cargo test --workspace --all-targets`

## Notes

- The integrated analyzer smoke uses 100 ticks for speed; the full 10k integrated-world gate is covered by `tests/phase2i_integrated_world.rs`.
- Analyzer integrated matter warnings use a looser per-run max tolerance than the 10k final gate because smoke reports max per-tick f32 deltas during decomposition on a large resource grid.
- The pre-existing modified file `outputs/worklogs/2026-07-13-1756-REPORT-phase-2g-reachability-fixes.md` was not part of this Phase 2I implementation.
