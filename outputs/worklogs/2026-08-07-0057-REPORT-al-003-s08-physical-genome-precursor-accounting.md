---
plan_id: AL-003-S08
status: implemented-with-environment-blocker
date: 2026-08-07
scope: Physical Genome Precursor Accounting
branch: codex/al-003-s08-physical-genome-precursors
base: stacked-on-uncommitted-AL-003-S07-worktree
---

# AL-003-S08 Physical Genome Precursor Accounting Report

## Summary

Implemented physical Genome precursor accounting on top of the uncommitted `AL-003-S07` Resource-derived Material Synthesis worktree.

Genome copying and recombination can now consume configured typed precursor Resources and Energy atomically after feasibility succeeds. Failed attempts preserve Resource, Energy, Genome, carrier, progress, and lineage state. Dead-cell decomposition now accounts Genome carrier matter as passive MaterialFragments instead of leaving carrier amount undecomposed or silently converting it to Resource.

This branch is stacked on `AL-003-S07`; review/merge should preserve that dependency order.

## Implemented

- Added `GenomePhysicalAccountingConfig`.
- Added `GenomeCopyingAccountingRule`.
- Added `GenomeRecombinationAccountingRule`.
- Added parser support for `[genome_physical_accounting.copying]`.
- Added parser support for `[genome_physical_accounting.recombination]`.
- Added validation for unknown precursor/waste Resource ids.
- Added validation for positive precursor requirements.
- Added config-hash participation for physical Genome accounting.
- Updated Genome copying feasibility to check typed precursor Resources when a physical accounting rule is configured.
- Updated Genome copying execution to debit typed Resources, emit configured waste outputs, and add copied carrier amount without using the generic Resource pool.
- Kept legacy Genome copying behavior when no physical accounting rule is configured.
- Updated Genome recombination feasibility to use configured Energy and typed precursor requirements.
- Updated Genome recombination execution to go through feasibility before debit/recombine.
- Added configured typed Resource debit and waste outputs for recombination.
- Kept existing deterministic recombination operator semantics.
- Added passive Genome carrier fragment accounting during dead-cell decomposition.
- Added `GENOME_CARRIER_FRAGMENT_MATERIAL_TYPE_ID` as a passive fragment identity, not an active capability.
- Updated `canonical_test_world.toml` with actual `genome_physical_accounting` runtime config and manifest declarations.
- Updated canonical scenario test coverage.
- Repaired stale Phase 3A ActionPlan expectation for existing `GenomeRecombination` output.
- Repaired a Phase 2G fixture initializer that still lacked S07 fields.

## Acceptance Mapping

| Acceptance ID | Evidence |
| --- | --- |
| `AL-003-S08-AC01` | `tests/phase3g_genome_precursors.rs` covers parser/config validation, copy precursor debit, copy rejection atomicity, recombination precursor debit, recombination rejection atomicity, config hash, and Genome carrier decomposition accounting. |
| `AL-003-S07-AC03` | `tests/phase3f_canonical_test_world.rs` confirms canonical test scenario resolves with S07 material synthesis and S08 genome precursor accounting surfaces. |

## Verification

Passed:

```powershell
cargo test --test phase3g_genome_precursors
```

Result: 9 passed.

```powershell
cargo test --test phase3g_genome_precursors --test phase3f_canonical_test_world --test phase3f_resource_material_synthesis --test phase3f_resource_material_config_parser --test phase3c_genome_copying --test phase3e_recombination
```

Result: 30 passed.

```powershell
cargo test --test phase3a_genome_bootstrap --test phase3a_tick_integration --test phase3a_action_plan --test runner_scenario_loader
```

Result: 24 passed.

```powershell
cargo test --workspace --lib
```

Result: 1 passed.

```powershell
rustfmt --check src/core/config.rs src/core/world.rs src/core/materials.rs src/runner/config_parser.rs tests/phase3a_action_plan.rs tests/phase3f_canonical_test_world.rs tests/phase3g_genome_precursors.rs
```

Result: passed.

```powershell
git diff --check
```

Result: passed with CRLF warnings only.

Blocked:

```powershell
cargo test --workspace --all-targets
```

First run exposed and allowed repair of `tests/phase2g_determinism.rs` fixture fields. After repair, the command is blocked by the known Windows linker/PDB and disk-space environment issue:

- `LNK1318: Unexpected PDB error; LIMIT (12)`
- `LNK1140: limit exceeded for program database; link with /PDB:NONE`
- `LNK1180: insufficient disk space to complete link`

Focused tests above did not expose Rust assertion failures.

Also blocked by the same linker issue:

```powershell
cargo test --test phase2g_determinism
```

## Files Changed By AL-003-S08

- `src/core/config.rs`
- `src/core/materials.rs`
- `src/core/world.rs`
- `src/runner/config_parser.rs`
- `tests/phase3g_genome_precursors.rs`
- `tests/phase3f_canonical_test_world.rs`
- `tests/phase3a_action_plan.rs`
- `tests/phase2g_determinism.rs`
- `config/scenarios/demo/canonical_test_world.toml`
- `outputs/worklogs/2026-08-07-0057-REPORT-al-003-s08-physical-genome-precursor-accounting.md`

Stacked `AL-003-S07` files are still present in the same worktree and branch diff.

## Known Constraints

- This is not ready to mark `done` until closure verification runs after the S07/S08 stack is reviewable.
- Full workspace all-target verification remains environment-blocked on Windows linker/PDB/disk limits.
- The implementation intentionally does not add active `AL-003-S09` Field runtime.
- The implementation intentionally does not change mutation/recombination algorithms beyond physical accounting gates.

## Next Step

Run closure verification on the stacked S07/S08 branch, or first separate/commit S07 and then rebase this S08 branch onto the accepted S07 base.
