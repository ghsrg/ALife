---
tags:
  - alife
  - implementation-plan
  - bootstrap
  - phase/bootstrap-1
  - tdd
---

# Bootstrap-1 Foundation Plan

> **For agentic workers:** це execution-grade TDD план. Не починай Runner-1 implementation, доки Bootstrap-1 acceptance gate не проходить. Bootstrap-1 є prerequisite для `outputs/worklogs/2026-07-12-1700-PLAN-runner-phase-1-headless.md`.

## Goal

Побудувати мінімальний, deterministic, constrained Bootstrap module, який перетворює immutable `ScenarioDocument` у `PreparedWorld` для Tick 0 і `BootstrapManifest`.

Bootstrap-1 не робить “красиву мапу світу”. Він створює достатньо контрольований стартовий світ, щоб Runner міг запускати Core без direct TOML-to-Core shortcut.

## Canon Sources

Перед implementation прочитати:

```text
docs/PRINCIPLES.md
docs/runner/INDEX.md
docs/runner/scenario-resolution.md
docs/runner/bootstrap.md
docs/runner/run-lifecycle.md
docs/runner/command-contract.md
docs/implementation/implementation-plan-bootstrap.md
docs/implementation/implementation-plan-runner.md
docs/mechanics/INDEX.md
docs/config/INDEX.md
docs/world/INDEX.md
docs/biology/INDEX.md
docs/genetics/INDEX.md
```

## Non-Goals

- No seasons.
- No temperature cycles.
- No catastrophes.
- No adaptive generation based on runtime results.
- No scripted survival behavior.
- No direct CLI/HTTP/UI dependency.
- No mutation of `WorldState` after Core starts.

Richer maps, seasonal inputs, temperature regions, disasters, and world families belong to later Bootstrap phases behind the same contract.

## Current Code Assumptions To Verify First

Expected existing modules:

```text
src/core/config.rs
src/core/world.rs
src/core/tick.rs
src/core/genome.rs
src/core/genome_bootstrap.rs
src/runner/config_parser.rs
```

If names differ, adapt the plan to existing APIs without changing the public Bootstrap contract.

Do not add new dependencies for Bootstrap-1 unless there is a strong reason. Prefer stable, local deterministic code.

## Contract

Bootstrap-1 implements this flow:

```text
ScenarioSource
  -> Scenario Resolution
  -> ScenarioDocument
  -> Bootstrap::prepare
  -> PreparedWorld
  -> BootstrapManifest
  -> Runner StartRun
  -> Core TickExecutor
```

Required properties:

```text
same ScenarioDocument + same seed -> same PreparedWorld hash
same ScenarioDocument + same seed -> same BootstrapManifest except timing-free metadata
changing one seed domain perturbs only that generated domain
Bootstrap executes zero Ticks
PreparedWorld is validated before Runner can start Core
```

## Public Types

### ScenarioDocument

Bootstrap-1 may wrap the existing `RuntimeConfig` to avoid rewriting all config parsing in this slice:

```rust
pub struct ScenarioDocument {
    pub id: String,
    pub schema_version: u32,
    pub scenario_hash: ScenarioHash,
    pub runtime_config: RuntimeConfig,
    pub canonical_source: String,
}
```

Rules:

- `scenario_hash` is computed from canonical normalized content, not path, request id, raw TOML bytes, or UI state.
- Use a versioned hash function name such as `scenario_hash_v1`.
- Do not use `DefaultHasher`; it is not a stable replay contract.
- Bootstrap-1 can use a local stable FNV-1a 64-bit implementation over normalized canonical text.
- `canonical_source` must be deterministic for semantically identical input according to the supported Bootstrap-1 normalization rules.

### PreparedWorld

Bootstrap-1 should stay compatible with current Core construction:

```rust
pub struct PreparedWorld {
    pub runtime_config: RuntimeConfig,
    pub manifest: BootstrapManifest,
    pub prepared_state_hash: PreparedStateHash,
}
```

This is intentionally minimal. Later phases may replace the inner representation with a richer Tick 0 state object, but Runner must keep depending on `PreparedWorld`, not on raw config internals.

### BootstrapManifest

Minimum fields:

```rust
pub struct BootstrapManifest {
    pub schema_version: u32,
    pub scenario_hash: ScenarioHash,
    pub prepared_state_hash: PreparedStateHash,
    pub root_seed: u64,
    pub generator_versions: Vec<GeneratorVersion>,
    pub seed_domains: Vec<SeedDomainRecord>,
    pub world_summary: WorldSummary,
    pub resource_summary: Vec<ResourceLayerSummary>,
    pub field_summary: Vec<FieldLayerSummary>,
    pub cell_summary: CellSummary,
    pub viability: ViabilityReport,
    pub warnings: Vec<BootstrapWarning>,
}
```

Rules:

- No wall-clock timestamps in deterministic manifest fields.
- Warnings are deterministic and sorted by stable code.
- Generator versions are explicit strings, for example `resource_layers.uniform.v1`.

## Proposed File Structure

```text
src/runner/scenario_doc.rs      [NEW] ScenarioSource, ScenarioDocument, ScenarioHash
src/bootstrap/mod.rs            [NEW] Bootstrap facade and BootstrapError
src/bootstrap/prepared.rs       [NEW] PreparedWorld, PreparedStateHash
src/bootstrap/manifest.rs       [NEW] BootstrapManifest and summaries
src/bootstrap/seed_domains.rs   [NEW] stable seed derivation and domain RNG helpers
src/bootstrap/resource_layers.rs [NEW] uniform and patches resource generation
src/bootstrap/field_layers.rs   [NEW] constant field generation
src/bootstrap/cell_placement.rs [NEW] explicit/grid/near-resource placement
src/bootstrap/starter_state.rs  [NEW] starter energy/material/resource/genome assignment
src/bootstrap/viability.rs      [NEW] constrained viability envelope validation
src/lib.rs                      [MODIFY] export bootstrap and runner scenario_doc
src/runner/mod.rs               [MODIFY] export scenario_doc

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

## TDD Tasks

### Task 0: Map Existing APIs

- [ ] Inspect current `RuntimeConfig`, `WorldConfig`, `TickExecutor`, `WorldState`, and genome bootstrap APIs.
- [ ] Record any mismatch directly in this plan before implementation.
- [ ] Confirm whether `TickExecutor` currently accepts only `RuntimeConfig` or already has a richer world/start-state constructor.

Expected outcome:

```text
Bootstrap-1 adapts to existing Core without changing Core hot-path semantics.
```

### Task 1: ScenarioDocument Boundary

Tests first:

```text
cargo test --test bootstrap_scenario_doc
```

Test cases:

- [ ] same scenario content loaded from two paths produces same `scenario_hash`;
- [ ] different `seed` produces different `scenario_hash`;
- [ ] changing whitespace or key order within Bootstrap-1 supported normalization does not change `scenario_hash`;
- [ ] `ScenarioDocument` preserves `runtime_config` needed by current Core;
- [ ] invalid TOML/config returns a typed resolution/validation error and does not call Bootstrap.

Implementation:

- [ ] Add `src/runner/scenario_doc.rs`.
- [ ] Add `ScenarioSource::{Path, Inline, Id}` if current runner needs all three; otherwise implement `Path` and leave `Id/Inline` as explicit future variants only if tests cover them.
- [ ] Add `ScenarioDocument::from_runtime_config(id, runtime_config, canonical_source)`.
- [ ] Add stable `scenario_hash_v1(canonical_source: &str) -> ScenarioHash`.
- [ ] Add deterministic normalization for Bootstrap-1 supported scenario fields.

Acceptance:

```text
ScenarioDocument is immutable after construction.
scenario_hash is stable across processes.
No hash uses std::collections::hash_map::DefaultHasher.
```

### Task 2: PreparedWorld And Manifest Skeleton

Tests first:

```text
cargo test --test bootstrap_prepared_world
```

Test cases:

- [ ] `Bootstrap::prepare(document)` returns `PreparedWorld`;
- [ ] `PreparedWorld.runtime_config` can be used to construct existing Core executor;
- [ ] `BootstrapManifest.scenario_hash` equals `ScenarioDocument.scenario_hash`;
- [ ] `prepared_state_hash` is stable for same input;
- [ ] manifest contains generator versions and seed domain records;
- [ ] Bootstrap does not execute a Tick: committed tick remains zero before Runner starts Core.

Implementation:

- [ ] Add `src/bootstrap/mod.rs`.
- [ ] Add `src/bootstrap/prepared.rs`.
- [ ] Add `src/bootstrap/manifest.rs`.
- [ ] Implement `prepare(document: &ScenarioDocument) -> Result<PreparedWorld, BootstrapError>`.
- [ ] Compute `prepared_state_hash_v1` from deterministic prepared state summary, not from memory addresses or debug formatting of unordered maps.

Acceptance:

```text
PreparedWorld is the only object Runner-1 may pass into Core start.
Runner plans must not call config_parser -> TickExecutor directly.
```

### Task 3: Seed Domains

Tests first:

```text
cargo test --test bootstrap_seed_domains
```

Seed domains:

```text
world.layout
resources.layers
fields.layers
cells.placement
cells.starter_state
genome.initialization
viability.audit
```

Test cases:

- [ ] same root seed + same domain label produces same domain seed;
- [ ] different domain labels produce different domain seeds;
- [ ] changing `resources.layers` domain does not change `cells.placement` generated sequence;
- [ ] domain records are sorted in manifest;
- [ ] seed derivation is stable across processes.

Implementation:

- [ ] Add `src/bootstrap/seed_domains.rs`.
- [ ] Use local deterministic split/derive function, for example stable FNV-1a over `root_seed || scenario_hash || domain_label`, then a small deterministic PRNG already available in the project or a local xorshift/splitmix implementation.
- [ ] Document generator version `seed_domains.v1`.

Acceptance:

```text
No global random source.
No thread_rng.
No non-deterministic system entropy.
```

### Task 4: Resource And Field Layers

Tests first:

```text
cargo test --test bootstrap_resource_layers
```

Bootstrap-1 supported resource generators:

```text
uniform
patches
```

Bootstrap-1 supported field generators:

```text
constant
```

Test cases:

- [ ] uniform resource generator produces exact expected total for a small grid;
- [ ] patches generator respects min/max amount and bounds;
- [ ] same seed produces same layer summary;
- [ ] changing resource seed domain changes resource layer summary but not cell placement domain output;
- [ ] constant field generator reports min=max=value;
- [ ] invalid resource/material/field id fails with typed `BootstrapError`.

Implementation:

- [ ] Add `src/bootstrap/resource_layers.rs`.
- [ ] Add `src/bootstrap/field_layers.rs`.
- [ ] For Bootstrap-1, generated layers may be represented as summaries/config transformations if current Core cannot ingest spatial layers yet.
- [ ] Do not silently drop unsupported generator fields; return a validation error or deterministic warning depending on Canon severity.

Acceptance:

```text
Resource and field layer summaries are in BootstrapManifest.
Spatial detail may be minimal, but constraints and totals are explicit.
```

### Task 5: Cell Placement

Tests first:

```text
cargo test --test bootstrap_cell_placement
```

Supported placement strategies:

```text
explicit
grid
near_resource
```

Test cases:

- [ ] explicit placement preserves declared coordinates and validates bounds;
- [ ] grid placement respects capacity and minimum spacing;
- [ ] near-resource placement is deterministic and bounded;
- [ ] impossible placement returns a typed error with stable code;
- [ ] all starting cell IDs are stable and sorted;
- [ ] placed cell count equals manifest `cell_summary.initial_cells`.

Implementation:

- [ ] Add `src/bootstrap/cell_placement.rs`.
- [ ] Keep placement independent from future organism roles or scripted behavior.
- [ ] Do not introduce species IDs, cell classes, organs, predators, brains, or fixed behavior roles.

Acceptance:

```text
Cells are placed by scenario constraints and seed domains only.
No biological shortcut is encoded in placement.
```

### Task 6: Starter State And Genome Bridge

Tests first:

```text
cargo test --test bootstrap_starter_state
```

Starter state includes:

```text
energy
resource inventory
material inventory
genome state
```

Test cases:

- [ ] starter energy is within configured min/max;
- [ ] starter inventories respect known resource/material ids;
- [ ] same seed gives same per-cell starter state;
- [ ] genome template assignment uses existing Phase 3A genome bootstrap module when configured;
- [ ] missing genome template fails or warns according to explicit scenario policy;
- [ ] starter state changes do not execute metabolism, repair, synthesis, growth, or joint creation.

Implementation:

- [ ] Add `src/bootstrap/starter_state.rs`.
- [ ] Call existing `core::genome_bootstrap` functions instead of duplicating genome mutation logic.
- [ ] Record genome generator version and template ids in `BootstrapManifest`.

Acceptance:

```text
Bootstrap assigns initial genome state but does not define runtime genome mechanics.
```

### Task 7: Viability Envelope

Tests first:

```text
cargo test --test bootstrap_viability
```

Minimum envelope checks:

```text
world dimensions > 0
initial cell count within capacity
all cells within bounds
minimum spacing satisfied where requested
starter energy within allowed range
starter resources/materials are known and non-negative
resource totals within configured min/max
required genome templates resolved
Core construction smoke check possible
```

Test cases:

- [ ] viable minimal scenario returns `ViabilityStatus::Pass`;
- [ ] marginal scenario returns deterministic warnings;
- [ ] impossible scenario returns typed error and no `PreparedWorld`;
- [ ] warnings are sorted by stable code;
- [ ] viability audit does not execute Tick.

Implementation:

- [ ] Add `src/bootstrap/viability.rs`.
- [ ] Add `ViabilityReport { status, checks, warnings }`.
- [ ] Use explicit warning/error codes, for example `BOOTSTRAP_LOW_START_ENERGY`, `BOOTSTRAP_CELL_CAPACITY_EXCEEDED`.

Acceptance:

```text
Bootstrap can reject worlds that are structurally impossible.
Bootstrap may warn about worlds that are likely weak but still valid.
```

### Task 8: End-To-End Bootstrap Pipeline

Tests first:

```text
cargo test --test bootstrap_integration
```

Test cases:

- [ ] `minimal_viable_world.toml` resolves to `ScenarioDocument`;
- [ ] Bootstrap prepares a stable `PreparedWorld`;
- [ ] `TickExecutor` can be constructed from `PreparedWorld.runtime_config` or the current equivalent Core entrypoint;
- [ ] a short smoke run advances ticks after Runner/Core starts, not during Bootstrap;
- [ ] same seed produces same final smoke summary;
- [ ] debug/manifest generation does not change final smoke result.

Implementation:

- [ ] Add `config/scenarios/bootstrap/minimal_viable_world.toml`.
- [ ] Add integration test using current `TickExecutor`.
- [ ] Keep smoke window short and deterministic.

Acceptance:

```text
Bootstrap-1 unblocks Runner-1 headless start.
Runner-1 can depend on PreparedWorld without knowing generation details.
```

### Task 9: Update Runner Integration Points

Files:

```text
docs/implementation/implementation-plan-runner.md
outputs/worklogs/2026-07-12-1700-PLAN-runner-phase-1-headless.md
```

Implementation requirements:

- [ ] Runner-1 references this Bootstrap-1 plan as a prerequisite.
- [ ] Runner-1 starts Core only from `PreparedWorld`.
- [ ] `PrepareScenario` returns `BootstrapManifest` and does not execute a Tick.
- [ ] `StartRun` runs Scenario Resolution, Bootstrap, PreparedWorld validation, then Core start.
- [ ] Direct TOML-to-`RuntimeConfig` startup snippets are marked superseded or rewritten.

## Minimal Scenario Shape

The exact TOML schema must follow existing config parser conventions. If Bootstrap-specific config blocks are added, keep them narrow:

```toml
[bootstrap]
schema_version = 1
root_seed = 42

[bootstrap.resources]
generator = "uniform"

[bootstrap.fields]
temperature = { generator = "constant", value = 0.5 }

[bootstrap.cells]
placement = "grid"
initial_count = 8
minimum_spacing = 2

[bootstrap.starter_state]
energy_min = 0.8
energy_max = 1.0
genome_template = "balanced"
```

If current config parser cannot accept new blocks yet, Bootstrap-1 may keep this as a scenario-side extension only after parser tests are added. Do not let unknown config silently pass if the project parser currently rejects unknown fields.

## Acceptance Gate

Bootstrap-1 is complete when:

```text
cargo test --test bootstrap_scenario_doc       -> PASS
cargo test --test bootstrap_seed_domains       -> PASS
cargo test --test bootstrap_prepared_world     -> PASS
cargo test --test bootstrap_resource_layers    -> PASS
cargo test --test bootstrap_cell_placement     -> PASS
cargo test --test bootstrap_starter_state      -> PASS
cargo test --test bootstrap_viability          -> PASS
cargo test --test bootstrap_integration        -> PASS
cargo test --workspace                         -> PASS
```

Manual smoke:

```text
cargo run --bin runner -- --debug --progress-interval-ms 200 config/scenarios/bootstrap/minimal_viable_world.toml
```

Expected:

```text
Runner resolves ScenarioDocument.
Bootstrap prepares PreparedWorld and BootstrapManifest.
No Tick is executed during Bootstrap.
Runner starts Core from PreparedWorld.
Debug table shows elapsed time, committed tick, ticks/sec, cell count, and state.
```

## Open Design Notes For Later Bootstrap Phases

Do not solve these in Bootstrap-1:

- rich terrain;
- seasonal resource changes;
- temperature bands changing over time;
- catastrophes;
- seed sweeps for world quality;
- preview maps;
- experiment calibration.

These require either richer initial-condition generators or real Core mechanics. They must not be hidden as Bootstrap shortcuts that fake life outcomes.
