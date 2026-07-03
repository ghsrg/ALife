# REPORT: Phase 2B Material Stubs Resolution - Task 2

## Goal
Implement a 9-material inventory in `CellStore` mapping to capability checks, maintaining backward compatibility.

## Scope & Decisions
- **9-Material Vectors**: Replaced the general `materials: Vec<MaterialAmount>` vector in `CellStore` with 9 individual vectors: `boundary_materials`, `transport_materials`, `metabolic_materials`, `storage_materials`, `synthesis_materials`, `structural_materials`, `repair_materials`, `contractile_materials`, and `sensory_materials`.
- **Getters & Setters**: Added specific getters and setters/mutators for all 9 material types, and a `total_materials(index)` function summing all 9 amounts.
- **Backward Compatibility**:
  - `insert_initial` and `set_materials` distribute the legacy amount equally among all 9 materials, enabling all capabilities for cells populated by legacy tests or parser.
  - `material_amount(index)` returns `total_materials(index)`.
- **Capability Mapping**: Updated `has_capability` to query specific materials $> 0.0$ for each capability (e.g. `Metabolism` -> `metabolic_materials > 0.0`).
- **TDD verification**: Wrote `capability_depends_on_specific_material_amount` in `tests/phase2_materials_smoke.rs`, confirming that capabilities are resolved from specific materials.

## Files Changed
- `src/core/cell_store.rs`
- `tests/phase2_materials_smoke.rs`

## Verification
- **Rust Core Suite**: `cargo test` -> **PASS (78 tests)**.
- **Formatting and Lints**: `cargo fmt --check` and `cargo clippy --workspace --all-targets --all-features -- -D warnings` -> **PASS**.
- **Python Suite**: `python -m pytest .\tools\early-stability` -> **PASS (93 tests)**.

## Open Questions
- None.
