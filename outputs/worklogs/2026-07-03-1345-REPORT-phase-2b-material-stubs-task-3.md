# REPORT: Phase 2B Material Stubs Resolution - Task 3

## Goal
Update configuration and TOML parsing logic to support the 9 specific material fields, while ensuring backward compatibility with legacy scenarios.

## Scope & Decisions
- **Config Fields**: Replaced `initial_material_amount` with the 9 specific material amounts inside `CellInitialConfig` in `src/core/config.rs`.
- **Config Hashing**: Updated `RuntimeConfig::config_hash` to include all 9 material fields (for both single and multi-cell configurations) to guarantee determinism.
- **TOML Parsing & Compatibility**:
  - Implemented `parse_materials_inventory` in `src/runner/config_parser.rs`.
  - Maps specific parsed material keys (e.g. `boundary`, `transport`, `metabolic`) to their respective fields.
  - Automatically maps common aliases (e.g. `membrane`/`envelope` to `boundary`, `pump` to `transport`, `metabolism`/`converter` to `metabolic`, `vacuolar` to `storage`, `producer` to `synthesis`, `skeleton`/`wall`/`cell_wall` to `structural`, `motor` to `contractile`, `receptor` to `sensory`).
  - If a legacy config uses non-specific keys but has a non-zero sum, the total amount is distributed equally among all 9 materials so that all capabilities are preserved.
- **Test Alignment**: Updated all manual configuration instantiations and initial cell states in integration tests to use the new fields.
- **TDD verification**: Wrote `test_legacy_config_backward_compatibility` and `test_specific_named_materials_parsing` in `tests/phase2_materials_smoke.rs`.

## Files Changed
- `src/core/config.rs`
- `src/runner/config_parser.rs`
- `src/core/world.rs`
- `src/core/cell_store.rs`
- `tests/phase1_accounting.rs`
- `tests/phase1_config_validation.rs`
- `tests/phase1_core_smoke.rs`
- `tests/phase1_determinism.rs`
- `tests/phase1_resource_grid.rs`
- `tests/phase1_resource_interaction.rs`
- `tests/phase1_sustained_viability.rs`
- `tests/phase2_core_smoke.rs`
- `tests/phase2_process_smoke.rs`
- `tests/phase2_reachability.rs`
- `tests/phase2_materials_smoke.rs`

## Verification
- **Rust Core Suite**: `cargo test` -> **PASS (80 tests)**.
- **Formatting and Lints**: `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` -> **PASS**.
- **Python Suite**: `python -m pytest .\tools\early-stability` -> **PASS (93 tests)**.

## Open Questions
- None.
