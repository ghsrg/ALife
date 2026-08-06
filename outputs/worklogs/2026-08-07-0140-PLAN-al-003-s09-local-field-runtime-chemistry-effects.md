---
plan_id: AL-003-S09
status: proposed
date: 2026-08-07
scope: Local Field Runtime And Chemistry Effects
---

# AL-003-S09 Local Field Runtime And Chemistry Effects TDD Plan

## Delivery-Control Routing

Mode: `TDD_PLAN_REQUEST`

Route:

```text
delivery-control -> delivery-lint deterministic preflight -> roadmap-control -> writing-plans
```

Decision: create a reviewable TDD handoff plan only. Do not execute implementation in this planning pass.

Approval gate:

```text
OK EXECUTE AL-003-S09
```

## Slice Status

Roadmap status: `planned`.

Operational status: `ready-for-approval` after local merge of prerequisites.

Prerequisite state observed during planning:

- `main` was fast-forward merged from `codex/al-003-s08-physical-genome-precursors`.
- `AL-003-S08` contains `AL-003-S07`, so both prerequisite commits are now on local `main`.
- Existing unrelated dirty files remain in the working tree and must be preserved during execution.
- `canonical_test_world.toml` already contains a manifest-only Field section that must become executable through this slice.

## Goal

Add a bounded deterministic Core-owned local Field grid runtime and connect configured Field values to local chemistry/material effects only through explicit rules.

Fields must not directly credit `EnergyBuffer`, mutate Genome behavior, apply direct damage, move Cells, or act as hidden commands.

## Source Hierarchy

Authority order used for this plan:

1. `docs/PRINCIPLES.md`
2. `docs/world/fields.md`
3. `docs/world/field-semantics.md`
4. `docs/config/fields_config.md`
5. `docs/engine/scheduler.md`
6. `docs/engine/physics.md`
7. `docs/world/reactions.md`
8. `docs/world/materials.md`
9. `docs/world/resources.md`
10. `docs/delivery/roadmap.md`
11. `docs/delivery/status.md`
12. `docs/delivery/acceptance.md`
13. `outputs/worklogs/2026-08-06-2246-PLAN-al-003-s07-resource-derived-material-synthesis.md`
14. `outputs/worklogs/2026-08-07-0017-PLAN-al-003-s08-physical-genome-precursor-accounting.md`
15. `outputs/worklogs/2026-08-07-0008-REPORT-al-003-s07-resource-derived-material-synthesis.md`
16. `outputs/worklogs/2026-08-07-0057-REPORT-al-003-s08-physical-genome-precursor-accounting.md`

Implementation files inspected:

- `src/bootstrap/field_layers.rs`
- `src/bootstrap/generator_spec.rs`
- `src/bootstrap/manifest.rs`
- `src/bootstrap/resource_layers.rs`
- `src/core/config.rs`
- `src/core/resources.rs`
- `src/core/world.rs`
- `src/core/tick.rs`
- `src/core/reactions.rs`
- `src/core/cell_store.rs`
- `src/runner/config_parser.rs`
- `config/scenarios/demo/canonical_test_world.toml`
- `tests/bootstrap_preview.rs`
- `tests/bootstrap_rich_generators.rs`
- `tests/phase2g_heat_boundary_repair.rs`
- `tests/phase2g_tick_integration.rs`
- `tests/phase3f_canonical_test_world.rs`

## Current Code Facts

- Bootstrap Field helpers currently create manifest summaries only; they do not populate Core-readable spatial Field grids.
- `RuntimeConfig` has scheduler support for `field_update_ticks`, but no active Core Field runtime model is present.
- `ResourceGrid` already provides a bounded deterministic scalar grid pattern with typed layers, coordinate lookup, decay, diffusion, and stable per-layer indexing.
- Cell temperature and heat effects already exist as local Cell state.
- Reaction conditions support temperature ranges in `src/core/reactions.rs`.
- `canonical_test_world.toml` declares `[canonical_manifests.fields.temperature]` with `runtime_owner_slice = "AL-003-S09"`.
- Observer/UI already has several field-summary surfaces, but this slice is Core runtime first; projection expansion is only allowed if needed for source-backed test evidence.

## Deterministic Delivery-Lint Result

Scope checked:

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- S07/S08 plan/report worklogs
- Local branch merge state

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| `WARN` | `DL007` | `AL-003-S07`, `AL-003-S08` | Reports still record environment-blocked full-suite verification, but prerequisite feature commits are merged locally and focused evidence exists. | Do not mark S07/S08 done in status from this plan; S09 may proceed as a planned dependent slice with focused prerequisite evidence. |
| `WARN` | `DL006` | `outputs/worklogs/index.md` | Worklog index has existing dirty edits from prior planning/merge state. | Add only the S09 plan link; avoid broad cleanup without separate approval. |
| `WARN` | `working tree` | root dirty files | Existing uncommitted UI/delivery/canonical-living-world changes predate this plan. | Preserve them; S09 execution should start from a clean dedicated worktree or explicitly approved dirty-root strategy. |

Decision: `PASS_WITH_WARNINGS` for TDD plan creation.

## Assumptions

1. `Field` values are scalar `f32` grid layers with explicit min/max clamps in S09. Vector/flow fields remain future work.
2. The first executable profile is `temperature` because the docs and current runtime already define heat/temperature semantics.
3. Bootstrap field generator summaries can become Core field-grid initializers without changing Bootstrap manifest semantics.
4. A Field effect is allowed to modify reaction/material parameters only through configured mechanism rules, not by direct global mutation.
5. The initial S09 local effect set should be narrow: local reaction rate multiplier, local material degradation multiplier, and optional local resource decay multiplier.
6. Genome Runtime may sample field-derived local inputs only in a later slice with material sensing basis unless this slice adds a read-only local sample API that is not wired to Genome decisions.

## Open Questions

| ID | Question | Default for implementation unless changed |
| --- | --- | --- |
| `AL-003-S09-Q01` | Should Field grids be typed by string ids or typed ids? | Use `FieldTypeId` plus stable string ids in config, mirroring Resource typed layers. |
| `AL-003-S09-Q02` | Should Bootstrap field manifests automatically initialize Core grids? | Yes, when a matching `[fields.<id>]` runtime config exists; otherwise preserve manifest-only warning behavior. |
| `AL-003-S09-Q03` | Should temperature Field overwrite Cell temperature? | No. It is sampled as local environment temperature / ambient input. Cell temperature changes through existing heat/reaction/transfer mechanisms. |
| `AL-003-S09-Q04` | Should Field effects be applied every Tick? | No. Use `scheduler.world.field_update_ticks` and elapsed-tick integration where propagation/decay is scheduled. |

## BDD Agent Scenario Cards

### `AL-003-S09-AC01-SC01` Field Config Parsing And Bounds

Source: `docs/world/fields.md`, `docs/config/fields_config.md`, `docs/delivery/acceptance.md`.

Given a scenario declares scalar Field runtime config with id, kind, initial value, min/max, diffusion, decay, effect profile, and conserved behavior.

When scenario resolution parses it.

Then RuntimeConfig contains deterministic bounded Field definitions, rejects invalid bounds/unknown profiles/non-finite values, and participates in config hash.

Acceptance: `AL-003-S09-AC01`.

TDD obligation: parser RED tests before adding config structs.

Evidence target: `AL-003-S09-E01`, `AL-003-S09-E02`.

### `AL-003-S09-AC01-SC02` Core Field Grid Is Local And Deterministic

Source: `docs/world/fields.md`, `docs/world/field-semantics.md`, `docs/engine/scheduler.md`.

Given a world has a scalar Field grid initialized from config or Bootstrap field layer.

When a cell samples the Field by position and two worlds run with the same seed/config.

Then sampled values are local to grid coordinates, clamped to configured bounds, and replay produces the same committed state and stable hash.

Acceptance: `AL-003-S09-AC01`.

TDD obligation: Core grid RED tests before `WorldState` integration.

Evidence target: `AL-003-S09-E03`, `AL-003-S09-E04`.

### `AL-003-S09-AC01-SC03` Field Effects Are Mechanism-Mediated

Source: `docs/world/field-semantics.md`, `docs/world/reactions.md`, `docs/world/materials.md`.

Given a reaction or material rule declares a local Field effect dependency.

When the local Field value is inside or outside the configured band.

Then only the registered reaction/material rule changes its bounded rate/degradation modifier, and no unrelated process observes a hidden behavior command.

Acceptance: `AL-003-S09-AC01`.

TDD obligation: tests for reaction/material effects must assert no direct Energy/Genome mutation.

Evidence target: `AL-003-S09-E05`, `AL-003-S09-E06`.

### `AL-003-S09-AC01-SC04` Field Scheduler Integrates Elapsed Ticks

Source: `docs/engine/scheduler.md`.

Given `scheduler.world.field_update_ticks = N`.

When Field propagation/decay runs after N ticks.

Then the update integrates elapsed ticks deterministically instead of silently skipping physical effect.

Acceptance: `AL-003-S09-AC01`.

TDD obligation: scheduler cadence RED tests before TickExecutor integration.

Evidence target: `AL-003-S09-E07`, `AL-003-S09-E08`.

### `AL-003-S09-AC01-SC05` Canonical Test World Executes Field Runtime Surface

Source: `config/scenarios/demo/canonical_test_world.toml`, `docs/delivery/acceptance.md`.

Given `canonical_test_world.toml` declares resource-derived materials, physical Genome precursor accounting, and `temperature` Field runtime ownership.

When scenario resolution and a short deterministic Tick run execute.

Then the scenario has a Core Field grid for temperature, local chemistry/material effects are bounded and source-backed, and S07/S08 material/genome surfaces still pass.

Acceptance: `AL-003-S09-AC01`.

TDD obligation: canonical scenario RED/GREEN integration tests after unit grid/parser tests pass.

Evidence target: `AL-003-S09-E09`, `AL-003-S09-E10`.

## Acceptance Mapping

| Acceptance ID | Scenario cards | Primary evidence |
| --- | --- | --- |
| `AL-003-S09-AC01` | `SC01`, `SC02`, `SC03`, `SC04`, `SC05` | `tests/phase3h_local_fields.rs`, `tests/phase3f_canonical_test_world.rs`, `tests/scheduler_world_cadence.rs`, `tests/runner_scenario_loader.rs` |

## Proposed File Map

| File | Action | Responsibility |
| --- | --- | --- |
| `src/core/fields.rs` | create | Core scalar Field ids, config, grid storage, local sampling, clamp/decay/diffusion primitives. |
| `src/core/mod.rs` | modify | Export `fields` module. |
| `src/core/config.rs` | modify | Runtime Field config structs and config hash participation. |
| `src/core/world.rs` | modify | Own Field grid state, expose read-only local samples, initialize from config/prepared fields. |
| `src/core/tick.rs` | modify | Schedule field update cadence and apply bounded local effect hooks. |
| `src/core/reactions.rs` | modify | Add optional Field condition/effect gate to reaction matching or execution context. |
| `src/bootstrap/prepared.rs` | modify | Carry prepared field layers when available. |
| `src/bootstrap/generator_spec.rs` | modify | Preserve existing field specs and provide Core-ready initial layers. |
| `src/bootstrap/field_layers.rs` | modify | Generate bounded scalar layer values, not only summary min/max. |
| `src/runner/config_parser.rs` | modify | Parse `[fields.<id>]` runtime config and validate effect profiles/bounds. |
| `config/scenarios/demo/canonical_test_world.toml` | modify | Promote manifest-only temperature Field declaration to executable runtime Field config while preserving canonical manifest notes. |
| `tests/phase3h_local_fields.rs` | create | Main S09 parser/grid/effect/replay test suite. |
| `tests/phase3f_canonical_test_world.rs` | modify | Assert canonical scenario resolves and exercises S09 Field runtime surface. |
| `tests/scheduler_world_cadence.rs` | modify | Add field update elapsed-tick cadence coverage. |

## TDD Tasks

### `AL-003-S09-T01` RED: Parser rejects invalid Field runtime config

Add `tests/phase3h_local_fields.rs` with failing tests for:

- valid scalar `temperature` Field config parses;
- `min_value > max_value` rejects;
- `initial_value` outside bounds rejects;
- non-finite diffusion/decay rejects;
- unknown `effect_profile` rejects;
- config hash changes when Field bounds/effect profile change.

Expected initial failure: `RuntimeConfig` has no Field runtime config model.

Evidence ID: `AL-003-S09-E01`.

### `AL-003-S09-T02` GREEN: Add Field config model and parser validation

Implement minimal config/parser support:

- `FieldTypeId`
- `FieldKind::Scalar`
- `FieldEffectProfile::{Temperature, Light, Pressure, Radiation, ChemicalGradient, Flow}`
- `FieldConservedBehavior::{Conserved, Dissipated, Clamped, Derived, Abstracted}`
- `FieldRuntimeConfig`
- `RuntimeConfig.fields`
- parser support for `[fields.<id>]`
- deterministic sorting by field id before assigning typed ids
- config hash participation.

Keep field runtime disabled when no `[fields]` section exists.

Evidence ID: `AL-003-S09-E02`.

### `AL-003-S09-T03` RED: Core Field grid samples local bounded values

Add failing Core tests proving:

- a scalar Field grid maps `Position` to local `GridCoord`;
- sample returns only the local cell value, not global average;
- setting a value clamps to min/max;
- diffusion never escapes bounds;
- same seed/config/layers produce same sampled values.

Expected initial failure: no Core Field grid type exists.

Evidence ID: `AL-003-S09-E03`.

### `AL-003-S09-T04` GREEN: Implement `src/core/fields.rs`

Implement a minimal scalar grid modeled after `ResourceGrid`:

- width/height from `WorldSize` and `spatial_grid_size`;
- one value vector per Field layer;
- typed id lookup;
- `coord_for_position`;
- `sample_at_position`;
- `set_value_at`;
- `decay_elapsed`;
- `diffuse_layer`;
- explicit `FieldGridError`.

Do not add vector/flow mechanics yet.

Evidence ID: `AL-003-S09-E04`.

### `AL-003-S09-T05` RED: WorldState owns Field grid without affecting Genome or Energy directly

Add failing tests proving:

- `WorldState::from_config` initializes fields from runtime config;
- local field sampling is read-only;
- stepping a world with fields but no registered effect does not change Energy, Genome outputs, Genome ids, action plans, or Cell movement;
- stable replay hash includes committed field values.

Expected initial failure: `WorldState` has no field grid ownership.

Evidence ID: `AL-003-S09-E05`.

### `AL-003-S09-T06` GREEN: Integrate Field grid into `WorldState`

Add Field grid ownership and read APIs:

- `WorldState::fields()`;
- `WorldState::fields_mut_for_commit()`;
- `WorldState::local_field_sample(cell_idx, field_id)`;
- initialization from `RuntimeConfig.fields`;
- stable-state hash participation for committed field values.

Do not wire Field samples into Genome Runtime in this task.

Evidence ID: `AL-003-S09-E06`.

### `AL-003-S09-T07` RED: Field scheduler integrates elapsed ticks

Add failing cadence tests:

- with `field_update_ticks = 5`, Field decay applies once at tick 5 using elapsed 5;
- same total elapsed ticks produce same final values as repeated deterministic updates where mathematically equivalent;
- observer/projection cadence does not alter field state.

Expected initial failure: `TickExecutor` does not update Field grids.

Evidence ID: `AL-003-S09-E07`.

### `AL-003-S09-T08` GREEN: Add scheduled Field propagation

Modify `TickExecutor::step` to:

- detect due field update through `scheduler.world.field_update_ticks`;
- call Field grid decay/diffusion with elapsed ticks;
- record a lightweight metric if an existing metrics slot can support it without broad Observer contract changes;
- preserve deterministic ordering by FieldTypeId/layer index.

Evidence ID: `AL-003-S09-E08`.

### `AL-003-S09-T09` RED: Field-gated chemistry/material effects are explicit

Add failing tests for a configured local temperature effect:

- a passive or controlled reaction with a temperature band executes only when local sampled Field value matches;
- reaction `heat_output` still changes local Cell temperature through existing heat capacity rules;
- Field never directly credits `EnergyBuffer`;
- Field never directly mutates Genome state;
- material degradation multiplier is bounded and applies only to configured material degradation rule.

Expected initial failure: reaction/material execution does not read Field runtime samples.

Evidence ID: `AL-003-S09-E09`.

### `AL-003-S09-T10` GREEN: Implement minimal field-mediated effect hooks

Implement only the mechanisms required by `T09`:

- extend reaction config with optional `field_condition = { field_id, min, max }`;
- pass local Field samples into reaction matching context;
- add optional material degradation multiplier config keyed by Field id and bounded min/max range;
- ensure all effects are clamped and local.

Do not add photosynthesis, movement flow, radiation mutation, or Genome sensing.

Evidence ID: `AL-003-S09-E10`.

### `AL-003-S09-T11` RED/GREEN: Bootstrap Field layers become Core initial Field grids

Add tests proving:

- existing `[[bootstrap.fields]]` specs can produce Core initial Field layer values when a matching runtime Field config exists;
- manifest-only warnings remain when runtime Field config is absent;
- `canonical_test_world.toml` resolves with executable `temperature` Field grid.

Then minimally extend Bootstrap prepared state to carry bounded field layers into `WorldState`.

Evidence ID: `AL-003-S09-E11`.

### `AL-003-S09-T12` RED/GREEN: Canonical scenario S07/S08/S09 integration

Update `tests/phase3f_canonical_test_world.rs` or add `tests/phase3h_canonical_fields.rs` to verify:

- `canonical_test_world.toml` has resource-derived material synthesis;
- physical Genome precursor accounting still parses;
- `temperature` Field grid is executable, bounded, and local;
- registered Field effects affect only reaction/material mechanisms;
- no direct Energy/Genome command side effects occur in a short Tick run.

Evidence ID: `AL-003-S09-E12`.

### `AL-003-S09-T13` REFACTOR: Keep Field runtime narrow and deterministic

After tests pass:

- centralize Field id lookup and bounds validation;
- remove duplicate parser normalization helpers if introduced;
- keep `ResourceGrid` and `FieldGrid` separate because Fields are not matter;
- add comments only where they prevent future Energy/Genome shortcuts;
- avoid UI/Observer expansion unless required by tests.

Evidence ID: `AL-003-S09-E13`.

### `AL-003-S09-T14` REGRESSION: Focused and broader verification

Run focused checks:

```powershell
cargo test --test phase3h_local_fields
cargo test --test phase3f_canonical_test_world
cargo test --test scheduler_world_cadence
cargo test --test phase3f_resource_material_synthesis --test phase3g_genome_precursors
```

Run broader checks:

```powershell
cargo test --test runner_scenario_loader
cargo test --test bootstrap_preview --test bootstrap_rich_generators
cargo test --workspace --lib
rustfmt --check src/core/fields.rs src/core/config.rs src/core/world.rs src/core/tick.rs src/core/reactions.rs src/runner/config_parser.rs tests/phase3h_local_fields.rs tests/phase3f_canonical_test_world.rs tests/scheduler_world_cadence.rs
git diff --check
```

Attempt full verification if the Windows linker/PDB/disk environment allows:

```powershell
cargo test --workspace --all-targets
```

Evidence ID: `AL-003-S09-E14`.

## Forbidden Scope

- Do not implement photosynthesis or any Field-to-Energy shortcut.
- Do not wire Field values directly into Genome Runtime decisions without material sensing basis and explicit downstream acceptance.
- Do not implement radiation-driven mutation in S09.
- Do not implement flow-driven movement in S09.
- Do not add species IDs, cell classes, organs, predators, brains, or scripted behaviors.
- Do not make Bootstrap field manifests claim Core runtime availability unless matching runtime Field config exists.
- Do not refactor UI rendering or Observer contracts except for narrow source-backed evidence, if needed.
- Do not mark S07/S08/S09 as done from this plan; closure verification is separate.

## Verification Strategy

Use RED/GREEN at each task boundary. Preserve the first failing output for each RED evidence id in the implementation report. Focus on narrow Rust tests first because full workspace `cargo test --workspace --all-targets` has known Windows linker/PDB/disk blockers in recent reports.

If full verification is blocked by environment:

- record exact linker/disk error;
- keep focused Rust test evidence;
- do not mark `AL-003-S09` done;
- route to closure verification for status recommendation.

## Handoff

Reply:

```text
OK EXECUTE AL-003-S09
```

to authorize implementation of this TDD plan.

Reply:

```text
CHANGE AL-003-S09 ...
```

to revise scope before execution.
