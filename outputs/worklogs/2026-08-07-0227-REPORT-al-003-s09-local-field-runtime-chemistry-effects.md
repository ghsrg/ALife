---
plan_id: AL-003-S09
status: implemented
date: 2026-08-07
scope: Local Field Runtime And Chemistry Effects
---

# AL-003-S09 Local Field Runtime And Chemistry Effects Report

## Summary

Implemented Core-owned scalar Field runtime support for `Resource-derived Material Synthesis` testing:

- added typed `FieldTypeId`, `FieldValue`, `FieldRuntimeConfig`, and deterministic `FieldGrid`;
- parsed `[fields.<id>]` runtime configs with bounded scalar values, effect profiles, and conserved behavior;
- initialized World-owned Field grids from runtime config or matching bootstrap field generators;
- added scheduled Field decay/diffusion through `scheduler.world.field_update_ticks`;
- added explicit local `field_condition` gates for material synthesis reactions;
- added explicit local `field_degradation` multipliers for configured material decay rules;
- included committed Field values and Field config in stable hash/config hash surfaces;
- promoted `canonical_test_world.toml` from manifest-only Field notes to executable `temperature` runtime config.

No Field path directly credits Energy, mutates Genome state, moves Cells, or acts as a hidden behavior command.

## TDD Evidence

RED evidence observed during execution:

- `cargo test --test phase3h_local_fields` initially failed because `core::fields`, `FieldTypeId`, `FieldValue`, `RuntimeConfig.fields`, and `WorldState::local_field_sample` did not exist.
- `field_grid_preserves_per_layer_bounds_from_runtime_configs` failed with `left: FieldValue(50.0), right: FieldValue(10.0)`, proving global bounds were incorrectly applied across layers.
- `field_condition_scales_configured_material_degradation_without_energy_or_genome_effects` failed with boundary material `0.9` instead of `0.8`, proving material degradation ignored the configured Field multiplier.
- `bootstrap_field_spec_initializes_matching_core_field_layer` failed with `FieldValue(10.0)` instead of `FieldValue(50.0)`, proving bootstrap Field specs did not initialize Core Field layers.

GREEN evidence:

- `tests/phase3h_local_fields.rs` now covers parser validation, local sampling/clamping, per-layer bounds, read-only world sampling, scheduled decay, reaction gating, material degradation scaling, and bootstrap-backed initialization.
- `tests/scheduler_world_cadence.rs` covers elapsed-tick Field updates.
- `tests/phase3f_canonical_test_world.rs` covers executable canonical `temperature` Field runtime alongside S07/S08 surfaces.

## Verification

Passed:

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --test phase3h_local_fields --test phase3f_canonical_test_world --test scheduler_world_cadence --test phase3f_resource_material_synthesis --test phase3g_genome_precursors --test runner_scenario_loader --test bootstrap_preview --test bootstrap_rich_generators --test phase2g_tick_integration --test phase2g_determinism --test phase2g_heat_boundary_repair
```

Result: 64 focused/regression tests passed.

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --workspace --lib
```

Result: 1 library test passed.

```powershell
$env:RUSTFLAGS='-C debuginfo=0'; cargo test --workspace --all-targets
```

Result: full workspace all-target test run passed. It required `RUSTFLAGS='-C debuginfo=0'` because the default Windows MSVC PDB/debug build previously hit disk/PDB limits.

```powershell
rustfmt --edition 2024 --check src/core/fields.rs src/core/config.rs src/core/world.rs src/core/tick.rs src/runner/config_parser.rs src/bootstrap/mod.rs tests/phase3h_local_fields.rs tests/phase3f_canonical_test_world.rs tests/scheduler_world_cadence.rs tests/phase2_process_smoke.rs tests/phase2_reflex_smoke.rs tests/phase2g_determinism.rs tests/phase2g_heat_boundary_repair.rs tests/phase2g_tick_integration.rs src/bin/sweep_analyzer.rs
```

Result: scoped S09 formatting passed.

```powershell
git diff --check
```

Result: passed with CRLF warnings only.

## Notes

- `cargo fmt --check` for the whole repository still reports pre-existing formatting diffs in unrelated files. Those formatting-only diffs were intentionally not included in this slice.
- Two stale all-target tests were repaired to match current fallback synthesis semantics:
  - `tests/phase2_process_smoke.rs`
  - `tests/phase2_reflex_smoke.rs`

