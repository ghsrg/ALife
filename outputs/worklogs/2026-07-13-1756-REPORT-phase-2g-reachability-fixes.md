---
tags:
  - alife
  - worklog/report
  - phase/2G
  - tdd
---

# Phase 2G Reachability Fixes Report

## Scope

Processed the analyzer report that showed inactive Phase 2G mechanisms in smoke outputs:

- repair had no successful executions;
- resource diffusion stayed at zero;
- material fragments were created but never converted;
- reaction heat was generated but not visible in peak temperature metrics.

## Changes

- Added deterministic typed resource diffusion execution in the tick pipeline and seeded the analyzer diffusion scenario with a non-uniform layer.
- Added explicit `MaterialFragment` conversion after at least one retention tick, with identity preserved before conversion.
- Connected passive reaction heat to local cell temperature and updated `sweep_analyzer` peak heat reporting from cell temperatures.
- Fixed repair feasibility and execution to consume the declared typed repair resource when generic resource is absent.
- Seeded analyzer `repair_viability` with boundary damage so the repair process is reachable in smoke.
- Added regression tests that run smoke analyzer and assert positive metrics for diffusion, fragment conversion, heat peak, and repair success.

## Verification

- `cargo test --test phase2g_resource_types resource_grid_diffuses_non_uniform_layer_and_conserves_without_decay -- --nocapture`
- `cargo test --test phase2g_tick_integration passive_reaction_heat_changes_local_cell_temperature -- --nocapture`
- `cargo test --test phase2g_tick_integration material_fragments_convert_to_resources_after_identity_retention_tick -- --nocapture`
- `cargo test --test phase2g_heat_boundary_repair repair_can_consume_declared_typed_resource_when_generic_resource_is_absent -- --nocapture`
- `cargo test --test phase2g_reachability_regression -- --nocapture`
- `cargo test --test phase2g_tick_integration -- --nocapture`
- `cargo test --test phase2g_heat_boundary_repair -- --nocapture`
- `cargo test --test phase2g_resource_types -- --nocapture`
- `cargo test --test phase2g_sweep_parser -- --nocapture`
- `cargo fmt --check`
- `cargo test --workspace --all-targets`

All verification commands passed.

## Smoke Evidence

Latest smoke run writes to `outputs/raw_data/smoke/`.

- `resource_type_decay_diffusion.csv`: `resource_diffused_amount` is positive for active diffusion rows.
- `fragment_decomposition_conversion.csv`: `fragment_created_amount` and `fragment_converted_amount` are positive for active rows.
- `local_heat_degradation.csv`: `heat_peak_temperature` reflects local reaction heat.
- `repair_viability.csv`: non-zero repair rows now have positive `repair_success_count`.

## Remaining Risk

The fixes make the core Phase 2G mechanisms reachable and regression-tested. The broader analyzer balance diagnostics still deserve a separate accounting/calibration pass; this report does not claim that legacy `BALANCE_ERROR` style diagnostics are fully normalized.
