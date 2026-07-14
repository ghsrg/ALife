---
tags:
  - alife
  - implementation-report
  - bootstrap
  - phase/bootstrap-1
  - tdd
---

# Bootstrap-1 Foundation Report

## Source Plan

Implemented from:

```text
outputs/worklogs/2026-07-14-1635-PLAN-bootstrap-1-foundation.md
```

Branch:

```text
codex/bootstrap-1-foundation
```

## Summary

Bootstrap-1 foundation is implemented as an application-level deterministic boundary:

```text
ScenarioDocument -> Bootstrap::prepare -> PreparedWorld + BootstrapManifest
```

The implementation intentionally does not rewrite Core world construction. Current Core still constructs Tick 0 from `RuntimeConfig` through `TickExecutor::new`. Bootstrap-1 wraps that current contract in `PreparedWorld`, validates it, derives deterministic seed domains, computes stable hashes, and produces manifest summaries.

This unblocks Runner-1 from depending on `PreparedWorld` instead of direct TOML -> `RuntimeConfig` -> Core startup.

## Files Added

```text
src/bootstrap/mod.rs
src/bootstrap/manifest.rs
src/bootstrap/prepared.rs
src/bootstrap/seed_domains.rs
src/bootstrap/resource_layers.rs
src/bootstrap/field_layers.rs
src/bootstrap/cell_placement.rs
src/bootstrap/starter_state.rs
src/bootstrap/viability.rs
src/runner/scenario_doc.rs

tests/bootstrap_scenario_doc.rs
tests/bootstrap_seed_domains.rs
tests/bootstrap_prepared_world.rs
tests/bootstrap_resource_layers.rs
tests/bootstrap_cell_placement.rs
tests/bootstrap_starter_state.rs
tests/bootstrap_viability.rs
tests/bootstrap_integration.rs

config/scenarios/bootstrap/minimal_viable_world.toml
```

## Files Modified

```text
src/lib.rs
src/runner/mod.rs
```

## Implemented Contracts

### ScenarioDocument

Implemented:

- `ScenarioSource::Path`
- `ScenarioSource::Inline`
- immutable `ScenarioDocument`
- `scenario_hash_v1`
- stable FNV-1a hash over Bootstrap-1 canonical source
- typed resolution errors with stable codes:
  - `SCENARIO_LOAD_FAILED`
  - `SCENARIO_PARSE_FAILED`

Constraint:

Bootstrap-1 canonicalization is intentionally narrow. It normalizes blank lines, comments, and spacing around `=`. It does not yet perform full TOML semantic canonicalization for arbitrary table/key ordering.

### PreparedWorld And BootstrapManifest

Implemented:

- `PreparedWorld`
- `PreparedStateHash`
- `BootstrapManifest`
- generator versions
- sorted seed domain records
- world summary
- resource summary
- field summary
- cell summary
- viability report

`PreparedWorld.runtime_config` is currently the compatibility bridge into existing Core.

### Seed Domains

Implemented deterministic seed domains:

```text
world.layout
resources.layers
fields.layers
cells.placement
cells.starter_state
genome.initialization
viability.audit
```

Implemented deterministic `SplitMix64` helper for Bootstrap generators.

No `thread_rng`, system entropy, `DefaultHasher`, or unordered map iteration is used for seed derivation.

### Resource And Field Helpers

Implemented:

- uniform resource layer summary
- patches resource layer summary
- constant field layer summary
- stable error code for invalid resource/field layer input:
  - `BOOTSTRAP_INVALID_RESOURCE_LAYER`

These are constrained Bootstrap helpers and summaries. They do not yet inject rich spatial resource maps into Core.

### Cell Placement Helpers

Implemented:

- explicit placement
- grid placement
- near-resource placement
- stable placement errors:
  - `BOOTSTRAP_CELL_OUT_OF_BOUNDS`
  - `BOOTSTRAP_CELL_PLACEMENT_IMPOSSIBLE`

No species IDs, roles, organs, predators, brains, or scripted behavior were added.

### Starter State And Genome Bridge

Implemented:

- starter energy range helper
- `assign_initial_genomes`
- bridge to existing `core::genome_bootstrap::instantiate_initial_genome`
- stable missing template error:
  - `BOOTSTRAP_UNKNOWN_GENOME_TEMPLATE`

Bootstrap assigns initial genome state through existing genome bootstrap logic; it does not define runtime genome mechanics.

### Viability Envelope

Implemented checks:

- world dimensions positive
- at least one initial cell
- starter energy within capacity
- cells within world bounds
- low starter energy warning

Stable failures/warnings:

```text
BOOTSTRAP_NO_INITIAL_CELLS
CELLS_WITHIN_WORLD_BOUNDS
BOOTSTRAP_LOW_START_ENERGY
```

## TDD Evidence

Each module was driven by failing tests first:

```text
bootstrap_scenario_doc      RED: missing runner::scenario_doc
bootstrap_seed_domains      RED: missing alife::bootstrap
bootstrap_prepared_world    RED: missing alife::bootstrap
bootstrap_resource_layers   RED: missing Result/error contract
bootstrap_cell_placement    RED: missing grid/near_resource placement
bootstrap_starter_state     RED: missing genome assignment bridge
bootstrap_viability         RED: low-energy warning not implemented
bootstrap_integration       RED: missing minimal_viable_world.toml
```

Then implementation was added until each test passed.

## Verification

Bootstrap acceptance gate:

```text
cargo test --test bootstrap_scenario_doc       -> 5 passed
cargo test --test bootstrap_seed_domains       -> 4 passed
cargo test --test bootstrap_prepared_world     -> 5 passed
cargo test --test bootstrap_resource_layers    -> 4 passed
cargo test --test bootstrap_cell_placement     -> 4 passed
cargo test --test bootstrap_starter_state      -> 3 passed
cargo test --test bootstrap_viability          -> 3 passed
cargo test --test bootstrap_integration        -> 2 passed
```

Workspace verification:

```text
cargo fmt --check                             -> pass
cargo test --workspace                        -> pass
cargo clippy --workspace --all-targets -- -D warnings -> pass
git diff --check                              -> pass
```

Notes:

- `cargo test --workspace` prints expected stderr from a negative-path `sweep_analyzer` validation test, but the command exits with code 0.
- `git diff --check` prints Windows LF -> CRLF warnings for touched files, but no whitespace errors.

## Manual Smoke

Not run:

```text
cargo run --bin runner -- --debug --progress-interval-ms 200 config/scenarios/bootstrap/minimal_viable_world.toml
```

Reason:

Runner binary/debug CLI is planned for Runner-1 and is not part of Bootstrap-1 implementation yet.

Equivalent Bootstrap/Core smoke is covered by:

```text
cargo test --test bootstrap_integration
```

It verifies:

- scenario file resolves to `ScenarioDocument`;
- Bootstrap prepares stable `PreparedWorld`;
- Core starts from `PreparedWorld.runtime_config`;
- Tick advances only after Core start;
- same seed gives same short smoke summary.

## Known Limits

- Full Scenario Resolution Canon is not complete. Bootstrap-1 implements a minimal `ScenarioDocument` wrapper around the existing parser.
- Full TOML semantic canonicalization is not implemented.
- Resource/field generators currently produce deterministic summaries/helpers, not full rich spatial maps.
- `PreparedWorld` still carries `RuntimeConfig` as compatibility bridge. This is intentional for Bootstrap-1.
- Runner command integration is not implemented in this slice; Runner-1 should consume these contracts next.

## Result

Bootstrap-1 foundation is ready for Runner-1 integration:

```text
Runner StartRun -> ScenarioDocument -> Bootstrap::prepare -> PreparedWorld -> Core start
```
