# REPORT: Phase 1D Sustained Viability Gates

## Goal
Closed Phase 1 with deterministic sustained-viability gates and observer-only run metrics.

## Scope
- Added run-level viability metrics.
- Added min/max Energy tracking across configured runs.
- Added final internal/external Resource metrics.
- Added final capacity metrics.
- Added observer-only growth_readiness metric.
- Added 1_000 tick sustained viability test.
- Added resource exhaustion collapse test.
- Added long-run Heat/Waste collapse tests.
- Preserved Phase 1A/1B/1C behavior.

## Decisions
- Cell radius remains fixed.
- No active growth or division was implemented.
- `growth_readiness` is observer-only and cannot affect behavior.
- Internal Resources remain aggregate in Phase 1D.
- Phase 2 will own Process Registry, Feasibility and real growth/division mechanics.

## Scenario Results
```text
single_cell_survival,Stable,None,100,50.000,0.000,0.000,Alive
single_cell_starvation,Collapse,MandatoryCostUnpaid,1,1.000,0.000,0.000,Dead
single_cell_dormancy,Collapse,EnergyDepleted,2,0.000,0.000,0.000,Dead
single_cell_heat_death,Collapse,HeatLimitExceeded,3,50.000,15.000,0.000,Dead
single_cell_waste_death,Collapse,WasteLimitExceeded,3,50.000,0.000,15.000,Dead
single_cell_over_capacity,Collapse,CapacityExceeded,1,53.000,0.000,0.000,Dead
```

## New Sustained Viability Tests
- `cell_collapses_when_local_resources_are_exhausted` - passes
- `cell_store_exposes_capacity_limit_for_observer_summary` - passes
- `run_summary_reports_resource_capacity_and_growth_readiness_metrics` - passes
- `cell_remains_stable_for_1000_ticks_on_local_resource_loop` - passes
- `run_until_configured_tick_tracks_energy_range_across_ticks` - passes
- `sustained_metabolism_collapses_when_heat_has_no_sufficient_sink` - passes
- `sustained_metabolism_collapses_when_waste_has_no_sufficient_sink` - passes

## Verification
- `cargo fmt --check` - passes
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` - passes
- `cargo test` - passes (all 48 tests)
- `python -m pytest .\tools\early-stability` - passes (93 tests)

## Open Questions
- Real growth remains Phase 2+.
- Division remains Phase 2+.
- Process Registry and Feasibility remain Phase 2+.
- Multi-resource internal inventories remain future work.

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
