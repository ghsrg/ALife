---
plan_id: AL-003-S07
status: partial
date: 2026-08-07
scope: Resource-Derived Material Synthesis
---

# AL-003-S07 Resource-Derived Material Synthesis Report

Worklogs are evidence, not source of truth.

## Purpose

Implement the approved TDD plan for Resource-derived Material Synthesis: derive `MaterialInstance` records from configured precursor Resources, validate material profiles/capabilities in TOML, execute material synthesis as an atomic resource/energy transaction, preserve fragment profile identity, and add a canonical test scenario for downstream completion work.

## Source Documents Read

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/mechanics/INDEX.md`
- `docs/mechanics/resource-material.md`
- `docs/mechanics/material-decomposition.md`
- `docs/mechanics/matter-accounting.md`
- `docs/mechanics/action-feasibility.md`
- `docs/world/resources.md`
- `docs/world/materials.md`
- `docs/world/reactions.md`
- `docs/config/reactions_config.md`
- `outputs/worklogs/2026-08-06-2246-PLAN-al-003-s07-resource-derived-material-synthesis.md`

## Changed Files Summary

- Added `src/core/material_instance.rs` with `MaterialProfile`, `MaterialCapabilityProfile`, `MaterialInstance`, synthesis transaction inventory/recipe/outcome, profile-preserving inactive fragments, explicit fragment conversion recipe, and stable fingerprints.
- Extended `src/core/config.rs` and `src/runner/config_parser.rs` with Resource `material_profile`, Resource `material_capabilities`, and Reaction `material_output` parsing/validation/hash participation.
- Added cell-owned material instance inventory to `src/core/cell_store.rs`.
- Updated `src/core/world.rs` so configured controlled `material_synthesis` reactions create resource-derived material instances and commit typed precursor debit, energy debit, byproduct Resource outputs, and Heat.
- Added `config/scenarios/demo/canonical_test_world.toml` with precursor resources, 7 material synthesis recipes, metabolism by-products, passive decay/degradation-style conversion declarations, nucleotide-like precursor declarations, and bootstrap field manifest.
- Updated neutral old literals in `src/bin/sweep_analyzer.rs` and `tests/phase2g_heat_boundary_repair.rs`.
- Added Phase 3F tests:
  - `tests/phase3f_resource_material_synthesis.rs`
  - `tests/phase3f_resource_material_config_parser.rs`
  - `tests/phase3f_canonical_test_world.rs`

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Evidence ID | Evidence | Status |
| --- | --- | --- | --- | --- | --- |
| `AL-003-S07` | Deterministic resource-derived profile/capability derivation and stable fingerprint | `AL-003-S07-AC01` | `AL-003-S07-EV01` | `cargo test --test phase3f_resource_material_synthesis` via focused combined run: 7 passed | covered |
| `AL-003-S07` | Atomic synthesis transaction: debit inputs/Energy, create MaterialInstance, emit Heat/waste, reject without mutation | `AL-003-S07-AC01` | `AL-003-S07-EV02` | `cargo test --test phase3f_resource_material_synthesis --test phase3f_resource_material_config_parser --test phase3f_canonical_test_world`: 13 passed total across Phase 3F tests | covered |
| `AL-003-S07` | Degradation creates profile-preserving inactive fragment; Resource recovery only through explicit conversion | `AL-003-S07-AC02` | `AL-003-S07-EV03` | `material_degradation_creates_profile_preserving_inactive_fragment` and `material_fragment_becomes_resources_only_through_explicit_conversion_recipe`: passed | covered |
| `AL-003-S07` | Parser validates resource profile bounds, capability keys, material-output recipes, invalid refs/accounting | `AL-003-S07-AC03` | `AL-003-S07-EV04` | `tests/phase3f_resource_material_config_parser.rs`: 5 passed | covered |
| `AL-003-S07` | Canonical test scenario resolves with full resource/material/reaction surface | `AL-003-S07-AC03` | `AL-003-S07-EV05` | `tests/phase3f_canonical_test_world.rs`: 1 passed | covered |
| `AL-003-S07` | Full workspace regression | all | `AL-003-S07-EV06` | `cargo test --workspace --all-targets` blocked by Windows PDB linker limit and disk full (`os error 112`) after compiling; no Rust test failure was observed before blocker | partial |

## Verification Commands

- `cargo test --test phase3f_resource_material_synthesis --test phase3f_resource_material_config_parser --test phase3f_canonical_test_world`
  - Result: pass; 13 tests passed.
- `cargo test --test phase2g_heat_boundary_repair`
  - Result: pass; 14 tests passed.
- `git diff --check`
  - Result: pass; only line-ending warnings.
- `rustfmt --check src/core/material_instance.rs src/core/cell_store.rs src/core/config.rs src/core/world.rs src/runner/config_parser.rs tests/phase3f_resource_material_synthesis.rs tests/phase3f_resource_material_config_parser.rs tests/phase3f_canonical_test_world.rs`
  - Result: pass.
- `cargo test --workspace --all-targets`
  - Result: blocked by environment: `LNK1318 Unexpected PDB error`, `LNK1140 limit exceeded for program database`, and `os error 112` not enough disk space. `cargo clean` removed 6.8 GiB in this worktree and focused tests were rerun successfully.

## Closure Result

`PARTIAL`.

The implemented S07 behavior has focused TDD coverage and the canonical test scenario resolves. Do not mark `AL-003-S07` as `done` until full workspace verification can run in an environment with enough disk space and without the current Windows PDB linker limit.

## Follow-Up

- Re-run `cargo test --workspace --all-targets` after freeing disk space or using a no-PDB build configuration.
- If the project accepts the focused evidence as sufficient for branch review, run closure again and then update `docs/delivery/status.md`.
