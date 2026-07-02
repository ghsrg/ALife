# REPORT: Phase 1C Resource Interaction Smoke

## Goal
Implemented deterministic local resource interaction smoke for Phase 1C.

## Scope
- **Snapshot Radius:** Fixed snapshot radius projection to return the actual stored Cell radius instead of a hardcoded 1.0.
- **ResourceInteractionConfig:** Introduced config structure and validation checks to verify rate correctness and layer indexing constraints.
- **Position-to-Grid Mapping:** Mapped cell center coordinate to grid coord using spatial grid size.
- **Local Uptake:** Implemented capacity-limited local resource uptake from ResourceGrid.
- **Metabolism:** Implemented simple metabolism converting aggregate resources to Energy, Heat and Waste.
- **Survival Scenario:** Added a deterministic survival scenario where a cell survives without passive energy income by consuming a nearby resource.
- **Collapse Scenario:** Added a deterministic collapse scenario when no resources are present.
- **Replay Determinism:** Confirmed that the new resource interaction is fully deterministic.
- **Old Behavior:** Preserved Phase 1A/1B scenario behaviors.

## Decisions
- Cell radius is fixed and does not yet affect uptake area footprint.
- Internal cell resources are represented as a single aggregate value.
- Resource interaction is disabled by default, keeping existing scenarios untouched.
- Cleaned up the unused variable `_used_capacity` inside the config parser.

## Scenario Results
```text
single_cell_survival,Stable,None,100,50.000,0.000,0.000,Alive
single_cell_starvation,Collapse,MandatoryCostUnpaid,1,1.000,0.000,0.000,Dead
single_cell_dormancy,Collapse,EnergyDepleted,2,0.000,0.000,0.000,Dead
single_cell_heat_death,Collapse,HeatLimitExceeded,3,50.000,15.000,0.000,Dead
single_cell_waste_death,Collapse,WasteLimitExceeded,3,50.000,0.000,15.000,Dead
single_cell_over_capacity,Collapse,CapacityExceeded,1,53.000,0.000,0.000,Dead
```

## New Resource Interaction Tests
- `cell_resource_consumption_is_limited_by_available_inventory` - passes
- `cell_collapses_without_local_resource_or_passive_income` - passes
- `cell_resource_uptake_is_limited_by_free_capacity` - passes
- `cell_survives_from_local_resource_without_passive_income` - passes
- `resource_grid_maps_position_to_clamped_grid_coord` - passes
- `resource_interaction_config_disabled_preserves_default_behavior` - passes
- `resource_interaction_is_deterministic_for_same_config_and_seed` - passes
- `runtime_config_rejects_enabled_interaction_with_missing_resource_layer` - passes
- `tick_metabolizes_internal_resource_into_energy_heat_and_waste` - passes
- `tick_uptakes_local_resource_into_cell_inventory` - passes
- `parser_maps_resource_interaction_block` - passes

## Verification
- `cargo fmt --check` - passes
- `cargo clippy --workspace --all-targets --all-features -- -D warnings` - passes
- `cargo test` - passes (all 41 tests)
- `python -m pytest .\tools\early-stability` - passes (93 tests)

## Open Questions
- Specific multi-resource metabolisms and active transport processes remain future work.
- Resource grid diffusion remains out of scope for Phase 1C.
