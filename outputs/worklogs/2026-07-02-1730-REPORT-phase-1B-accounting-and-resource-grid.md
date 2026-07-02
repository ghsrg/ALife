# REPORT: Phase 1B Accounting And ResourceGrid

## Goal
Hardened Phase 1B accounting by replacing the single-layer ResourceGrid placeholder with a flat, indexed, deterministic multi-layer resource grid storage, separating external resources from internal cell inventory, and preserving current scenario behaviors.

## Scope
- **GridCoord:** Implemented `GridCoord` to represent integer grid coordinates.
- **ResourceConfig:** Created `ResourceConfig` representing layer distribution and decay, adding it to `RuntimeConfig` and removing `WorldConfig::optional_decay_rate`.
- **ResourceGrid:** Implemented flat multi-layer quantity storage in `ResourceGrid`, using world size and spatial grid size to dynamically allocate deterministic cells.
- **Decay:** Applied decay rate to all layers in `ResourceGrid::decay_or_passive_update()`.
- **Parser Mapping:** Enabled parser to map initial distributions into `ResourceConfig`, verifying length correctness of `resource_type_ids` and `initial_distribution`.
- **Snapshot Integration:** Exposed read-only `resource_layer_totals` in committed snapshots.

## Decisions
- Spatial decay applies uniformly to all cells in the grid on each tick.
- Internal Cell inventory (`initial_resource_amount` / `initial_material_amount`) is kept fully separate from the external `ResourceGrid` initial distribution layers.
- Avoided all heap allocation during `step()` execution by utilizing index loops.

## Scenario Results
```text
single_cell_survival,Stable,None,100,50.000,0.000,0.000,Alive
single_cell_starvation,Collapse,MandatoryCostUnpaid,1,1.000,0.000,0.000,Dead
single_cell_dormancy,Collapse,EnergyDepleted,2,0.000,0.000,0.000,Dead
single_cell_heat_death,Collapse,HeatLimitExceeded,3,50.000,15.000,0.000,Dead
single_cell_waste_death,Collapse,WasteLimitExceeded,3,50.000,0.000,15.000,Dead
single_cell_over_capacity,Collapse,CapacityExceeded,1,53.000,0.000,0.000,Dead
```

## Verification
- **Rust Formatting:** `cargo fmt --check` passes successfully.
- **Linter Lints:** `cargo clippy` passes successfully with no warnings.
- **Rust Integration Tests:** All 37 integration tests pass.
- **Python Verification:** `python -m pytest .\tools\early-stability` passes (93 tests).

## Open Questions
- Resource uptake/export and diffusion are intentionally left unimplemented for Phase 1B, keeping the world static except for resource decay.
