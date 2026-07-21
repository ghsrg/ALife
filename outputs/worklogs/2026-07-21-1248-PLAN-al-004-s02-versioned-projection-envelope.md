---
tags:
  - alife
  - worklog/plan
  - plan/AL-004-S02
  - area/observer
  - area/projection
---

# PLAN: AL-004-S02 Versioned Projection Envelope

## TDD_PLAN_PROPOSAL

Plan ID: `AL-004-S02`

Selected slice: Versioned Projection Envelope

Current roadmap status: `planned`

Goal: define and test a shared, versioned, read-only projection envelope for Observer/Runner/UI/storage/report consumers, then map the current `WorldFrameProjection v2` / ALIF v2 frame body into that envelope without changing Core behavior or Runner transport semantics.

Architecture: add a typed Rust Observer projection envelope module with canonical vocabulary for projection kind, entity kind, source, completeness, schema version, run/tick identity, config hash, engine version, and generation time. Existing projection payload structs remain body types; consumers can receive `EnvelopedProjection<T>` when they need shared metadata. Current ALIF v2 binary frame encoding remains a compact transport body and is tested as semantically mapped, not replaced.

Tech stack: Rust core/observer/runner modules, existing `WorldFrameProjection`, existing `Observer contract` inventory, integration tests under `tests/`, and delivery-control artifacts.

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/GLOSSARY.md`
3. `docs/observer/observer-layer.md`
4. `docs/observer/projection-contract.md`
5. `docs/mechanics/observer-projection.md`
6. `docs/runner/projections.md`
7. `docs/engine/storage.md`
8. `docs/ui/architecture.md`
9. `docs/implementation/implementation-phases.md`
10. `docs/delivery/roadmap.md`

Existing code/tests are implementation evidence only. Worklogs are historical/execution evidence, not source of truth.

## Files Read

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/delivery/source-map.md`
- `docs/PRINCIPLES.md`
- `docs/GLOSSARY.md`
- `docs/observer/observer-layer.md`
- `docs/observer/projection-contract.md`
- `docs/observer/mechanism-coverage.md`
- `docs/observer/classification-contract.md`
- `docs/observer/behavior-profile-balance.md`
- `docs/mechanics/observer-projection.md`
- `docs/runner/projections.md`
- `docs/engine/storage.md`
- `docs/ui/architecture.md`
- `docs/implementation/implementation-phases.md`
- `src/observer/contract.rs`
- `src/runner/projections.rs`
- `src/viewer_server/frame_encoder.rs`
- `tests/observer_contract_closure.rs`
- `tests/runner_projection_world_frame.rs`
- `tests/runner_frame_encoder.rs`
- `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md`
- `outputs/worklogs/2026-07-21-1159-REPORT-al-003-s05-lineage-event-log-and-replay.md`

## Domain Modeling Decisions

| Question | Decision |
| --- | --- |
| Stable identity | Projection identity is metadata, not a Core entity. Use typed enum/value objects, not new Core IDs. |
| State owner | Observer/Runner owns projection metadata at projection build time. Core owns committed state only. |
| Mutation authority | Projection envelope is read-only and must not feed Genome Runtime, Feasibility, Scheduler, lifecycle, or process selection. |
| Hot path category | Observer projection metadata, outside Tick hot path. |
| Storage | No SQLite/Parquet/files in this slice; storage/index remains `AL-005-S01`. |
| Schema export | Rust-only typed contract for this slice. Generated JSON/TOML/TypeScript schemas are deferred until a consumer slice proves the need. |
| ALIF v2 | Keep existing binary body layout. Add typed semantic envelope mapping around `WorldFrameProjection`, not a transport rewrite. |

## Assumptions

- `Needs Review`: `docs/observer/projection-contract.md` lists `full`, `bounded`, `sampled`, `partial`, `debug_selected`; roadmap also requires unavailable/stale semantics. This plan adds explicit `unavailable` and `stale` states to the typed completeness vocabulary because UI-2A needs them to avoid showing absent historical/live data as complete.
- `Needs Review`: `run_id` is optional in early in-process projections because current `WorldFrameProjection::from_committed_snapshot` can be built from a snapshot without a Runner active-run context. Runner-backed envelopes must provide it.
- `Needs Review`: `config_hash` and `engine_version` are optional in bare tests/in-process fixtures but required for Runner-backed live/recorded envelopes.
- `Needs Review`: `generated_at` is represented as `u64` Unix milliseconds to avoid adding a time dependency and to match existing frame metadata.

## Forbidden Scope

- Do not change Core simulation behavior.
- Do not add projection data as a Genome Runtime, Feasibility, Scheduler, lifecycle, mutation, selection, or Process input.
- Do not implement durable storage, SQLite metadata, Parquet exports, historical keyframes, or seek/replay storage.
- Do not implement UI-2A, UI projection compatibility UI, keyframe browser, inspectors, or frontend state changes.
- Do not replace ALIF v2 binary frame encoding or change `/stream` transport compatibility in this slice.
- Do not implement full `OrganismView`, classification, behavior balance, coverage, or lineage visualization payloads; only define envelope vocabulary and minimal typed wrapping support.

## BDD Agent Scenario Cards

### `AL-004-S02-AC01`: Canonical Projection Envelope Vocabulary

Source links: `docs/observer/projection-contract.md`, `docs/observer/observer-layer.md`, `docs/mechanics/observer-projection.md`.

Priority: P1

Intent: all projection consumers need one typed vocabulary for schema, projection kind, source, completeness, and entity references.

Given a projection payload is produced for any Observer consumer  
When the payload is wrapped in the shared projection envelope  
Then the envelope exposes `schema_version`, `projection_kind`, `run_id`, `tick`, `config_hash`, `engine_version`, `source`, `completeness`, and `generated_at_unix_ms` through read-only typed fields.

TDD obligation: add RED tests for missing `observer::projection_envelope` module and vocabulary before implementation.

Evidence IDs: `AL-004-S02-EV01`, `AL-004-S02-EV02`.

### `AL-004-S02-AC02`: Current Runner Frame Maps Into Envelope Without ALIF Rewrite

Source links: `docs/runner/projections.md`, `docs/observer/projection-contract.md`, `src/runner/projections.rs`, `src/viewer_server/frame_encoder.rs`.

Priority: P1

Intent: current live frame consumers should gain shared metadata semantics without breaking the compact binary frame body.

Given a `WorldFrameProjection v2` exists for a committed snapshot  
When Runner context supplies run/config/engine/source/completeness metadata  
Then the frame can be wrapped as `ProjectionKind::Frame` with matching tick/schema metadata, and existing ALIF v2 encode/decode roundtrip remains unchanged.

TDD obligation: add RED tests proving no envelope mapping API exists and GREEN tests proving ALIF bytes remain compatible.

Evidence IDs: `AL-004-S02-EV03`, `AL-004-S02-EV04`.

### `AL-004-S02-AC03`: Completeness, Source, And Unavailable/Stale Semantics

Source links: `docs/observer/projection-contract.md`, `docs/ui/architecture.md`, `docs/engine/storage.md`.

Priority: P1

Intent: UI and storage must distinguish live, recorded, debug, fixture, sampled, partial, stale, and unavailable data instead of silently treating missing data as truth.

Given a projection cannot provide full data for a requested consumer context  
When the envelope is built  
Then `completeness` records the explicit state and optional missing fields/reason without mutating or fabricating payload data.

TDD obligation: add RED tests for unavailable/stale/partial completeness constructors and their immutable reason/missing-field data.

Evidence IDs: `AL-004-S02-EV05`, `AL-004-S02-EV06`.

### `AL-004-S02-AC04`: Projection-Kind Coverage For Downstream Slices

Source links: `docs/observer/projection-contract.md`, `docs/observer/mechanism-coverage.md`, `docs/observer/classification-contract.md`, `src/observer/contract.rs`.

Priority: P1

Intent: downstream Observer/UI/storage work must share one declared projection-kind vocabulary before implementing richer payloads.

Given the Observer contract inventory is inspected  
When projection kinds are enumerated  
Then frame, entity, inspector, metrics, coverage, balance, classification, lineage, OrganismView, and debug trace kinds are present and read-only, with follow-up payload implementation still owned by their existing Plan IDs.

TDD obligation: add RED test for missing projection-kind coverage and GREEN it through typed vocabulary plus contract inventory mapping.

Evidence IDs: `AL-004-S02-EV07`, `AL-004-S02-EV08`.

### `AL-004-S02-AC05`: Observer Boundary And Rust-Only Schema Disposition

Source links: `docs/PRINCIPLES.md`, `docs/observer/observer-layer.md`, `docs/mechanics/observer-projection.md`, `docs/implementation/implementation-phases.md`.

Priority: P1

Intent: the envelope must remain an Observer/API contract and must not become a behavior input or hidden generated-schema commitment.

Given Core and Genome Runtime sources are scanned  
When the projection envelope is introduced  
Then Core behavior modules do not import/use projection envelope read models, and the contract declares `RustTypedContractOnly` for this slice.

TDD obligation: add source-boundary tests and a schema disposition test before closure.

Evidence IDs: `AL-004-S02-EV09`, `AL-004-S02-EV10`.

### `AL-004-S02-AC06`: Delivery Closure And Handoff

Source links: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`.

Priority: P1

Intent: closure should unblock `AL-005-S01` planning and reduce `AL-007-S09` dependency risk without pretending storage/UI payloads are implemented.

Given envelope tests and focused regression pass  
When delivery closure runs  
Then roadmap/status/acceptance/ledger/report mark `AL-004-S02` according to evidence, while storage remains `AL-005-S01`, Runner hardening remains `AL-002-S16`, and UI-2A remains blocked until those dependencies are selected or explicitly staged.

TDD obligation: no production behavior; closure-verification after implementation tests.

Evidence IDs: `AL-004-S02-EV11`, `AL-004-S02-EV12`.

## Proposed File Plan

Create:

- `src/observer/projection_envelope.rs`: typed envelope, schema version, projection kind, entity kind, source, completeness, schema export disposition, generic `EnvelopedProjection<T>`.
- `tests/projection_envelope_contract.rs`: TDD tests for vocabulary, frame wrapping, completeness, contract inventory, read-only boundary, and schema disposition.

Modify:

- `src/observer/mod.rs`: expose `projection_envelope`.
- `src/observer/contract.rs`: include projection envelope field/kind coverage in the existing static Observer inventory where useful.
- `src/runner/projections.rs`: add non-breaking helper(s) for wrapping `WorldFrameProjection` with `ProjectionEnvelope`.
- `tests/observer_contract_closure.rs`: only if needed to keep AL-004-S01 inventory tests aligned with the new envelope fields.
- `tests/runner_projection_world_frame.rs`: only if the frame helper belongs with existing Runner projection tests; prefer the dedicated new test file for envelope behavior.
- Delivery closure after execution: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, `outputs/worklogs/index.md`.

Do not create a new Canon doc in this slice unless implementation discovers a blocking ambiguity. If the typed vocabulary reveals a semantic gap, record `Needs Review` in the closure report instead of silently expanding requirements.

## Numbered TDD Tasks

### `AL-004-S02-T01`: RED for `AL-004-S02-AC01`

- Add `tests/projection_envelope_contract.rs`.
- Write failing test `projection_envelope_declares_required_metadata_vocabulary`.
- Expected RED: `alife::observer::projection_envelope` does not exist.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract projection_envelope_declares_required_metadata_vocabulary
```

- Capture result as `AL-004-S02-EV01`.

### `AL-004-S02-T02`: GREEN for `AL-004-S02-AC01`

- Add `src/observer/projection_envelope.rs` with:
  - `ProjectionSchemaVersion`
  - `ProjectionKind`
  - `ProjectionEntityKind`
  - `ProjectionSource`
  - `ProjectionCompleteness`
  - `ProjectionCompletenessState`
  - `ProjectionEnvelope`
  - `EnvelopedProjection<T>`
  - `SchemaExportDisposition`
- Add `pub mod projection_envelope;` in `src/observer/mod.rs`.
- Keep fields immutable through constructors/accessors or read-only public value objects; no mutable global registry.
- Run the same test and capture pass as `AL-004-S02-EV02`.

### `AL-004-S02-T03`: RED for `AL-004-S02-AC02`

- Add failing tests:
  - `world_frame_projection_wraps_with_runner_envelope_metadata`
  - `world_frame_envelope_does_not_change_alif_v2_binary_body`
- Expected RED: `WorldFrameProjection` has no envelope mapping helper/context type.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract world_frame_projection_wraps_with_runner_envelope_metadata world_frame_envelope_does_not_change_alif_v2_binary_body
```

- Capture result as `AL-004-S02-EV03`.

### `AL-004-S02-T04`: GREEN for `AL-004-S02-AC02`

- Add `ProjectionBuildContext` or equivalent small value object in `src/observer/projection_envelope.rs` for:
  - `run_id`
  - `config_hash`
  - `engine_version`
  - `source`
  - `completeness`
  - `generated_at_unix_ms`
- Add a non-breaking `WorldFrameProjection::into_enveloped(context)` or `as_enveloped(context)` helper.
- Ensure `WorldFrameProjection::SCHEMA_VERSION` maps to `ProjectionSchemaVersion`.
- Do not add envelope fields to the ALIF v2 binary body.
- Run the same tests and existing `tests/runner_frame_encoder.rs`; capture pass as `AL-004-S02-EV04`.

### `AL-004-S02-T05`: RED for `AL-004-S02-AC03`

- Add failing tests:
  - `projection_completeness_records_partial_missing_fields`
  - `projection_completeness_records_stale_and_unavailable_reasons`
- Expected RED: completeness constructors/states do not exist.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract projection_completeness_records_partial_missing_fields projection_completeness_records_stale_and_unavailable_reasons
```

- Capture result as `AL-004-S02-EV05`.

### `AL-004-S02-T06`: GREEN for `AL-004-S02-AC03`

- Implement deterministic completeness constructors:
  - `ProjectionCompleteness::full()`
  - `bounded(reason)`
  - `sampled(reason)`
  - `partial(missing_fields, reason)`
  - `debug_selected(reason)`
  - `stale(reason)`
  - `unavailable(reason)`
- Keep missing fields as sorted/deterministic `Vec<&'static str>` or equivalent stable value list.
- Run the same tests and capture pass as `AL-004-S02-EV06`.

### `AL-004-S02-T07`: RED for `AL-004-S02-AC04`

- Add failing tests:
  - `projection_kind_vocabulary_covers_planned_observer_projection_kinds`
  - `observer_contract_maps_projection_envelope_fields_to_al_004_s02`
- Expected RED: projection kind vocabulary and inventory mapping are incomplete.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract projection_kind_vocabulary_covers_planned_observer_projection_kinds observer_contract_maps_projection_envelope_fields_to_al_004_s02
```

- Capture result as `AL-004-S02-EV07`.

### `AL-004-S02-T08`: GREEN for `AL-004-S02-AC04`

- Add every planned kind from `docs/observer/projection-contract.md` to `ProjectionKind`.
- Add entity vocabulary sufficient for `World`, `Cell`, `Resource`, `Material`, `Field`, `Process`, `Joint`, `Genome`, `Lineage`, `OrganismView`, `Run`, and Observer analytical artifacts.
- Extend `src/observer/contract.rs` only for envelope-level field mapping, not full payload schemas.
- Run the same tests and existing `observer_contract_closure` regression; capture pass as `AL-004-S02-EV08`.

### `AL-004-S02-T09`: RED for `AL-004-S02-AC05`

- Add failing/characterization tests:
  - `projection_envelope_declares_rust_typed_contract_only_schema_disposition`
  - `projection_envelope_does_not_enter_core_behavior_inputs`
- Expected RED may be missing schema disposition API or missing boundary guard.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract projection_envelope_declares_rust_typed_contract_only_schema_disposition projection_envelope_does_not_enter_core_behavior_inputs
```

- Capture result as `AL-004-S02-EV09`.

### `AL-004-S02-T10`: GREEN for `AL-004-S02-AC05`

- Add `SchemaExportDisposition::RustTypedContractOnly`.
- Add source-scan/API-boundary guard covering `src/core/genome.rs`, `src/core/tick.rs`, `src/core/world.rs`, `src/core/process.rs`, and `src/core/stable_state_hash.rs`.
- Guard should allow Runner/Observer imports but reject Core behavior modules importing `projection_envelope`.
- Run the same tests and capture pass as `AL-004-S02-EV10`.

### `AL-004-S02-T11`: REFACTOR and Focused Regression

- Refactor only after `projection_envelope_contract` passes.
- Keep envelope code outside Core hot path.
- Run focused regression:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract --test observer_contract_closure --test runner_projection_world_frame --test runner_frame_encoder --test runner_projection_sampler --test runner_tick_broadcast --test runner_ws_stream --test phase3d_lineage_replay
```

- Capture result as `AL-004-S02-EV11`.

### `AL-004-S02-T12`: Delivery Closure

- Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-004-s02-versioned-projection-envelope.md`.
- Update:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
  - `outputs/worklogs/index.md`
- Review `Candidate Next Work` in the same pass.
- Run:

```powershell
git diff --check
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo fmt --check
```

- Avoid full `cargo test` unless explicitly selected; use the focused regression above to avoid unnecessary target growth.
- Capture closure as `AL-004-S02-EV12`.

## Verification Commands

Primary:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract
```

Focused regression:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo test --test projection_envelope_contract --test observer_contract_closure --test runner_projection_world_frame --test runner_frame_encoder --test runner_projection_sampler --test runner_tick_broadcast --test runner_ws_stream --test phase3d_lineage_replay
```

Delivery:

```powershell
git diff --check
$env:CARGO_TARGET_DIR='target\codex-al004s02'; cargo fmt --check
```

## Open Questions

1. `Needs Review`: should `unavailable` and `stale` be added to `docs/observer/projection-contract.md` after implementation, since roadmap/UI needs them but the current doc only lists five completeness states?
2. `Needs Review`: should generated JSON/TOML/TypeScript schemas be pulled into `AL-004-S02`, or is Rust-only typed contract enough until `AL-007-S09` asks for frontend compatibility? This plan chooses Rust-only for scope control.
3. `Needs Review`: should bare in-process test projections permit missing `run_id/config_hash/engine_version`, or should every envelope require Runner/run metadata immediately? This plan permits optional values for fixture/in-process projections but requires Runner-backed tests to supply them.
4. `Needs Review`: should `generated_at_unix_ms` stay wall-clock metadata, or should offline/replay projections support deterministic artifact timestamps from storage metadata? This plan only preserves metadata; storage semantics remain `AL-005-S01`.

## Status Update Recommendation

- Set `docs/delivery/status.md` `Current Focus` to `AL-004-S02` with status `planned`.
- Mark `AL-004-S02` in `Next` as `planned-ready`.
- Keep `docs/delivery/roadmap.md` status for `AL-004-S02` as `planned` until execution/closure.
- Keep `AL-002-S16` and `AL-002-S11` in `Next`.
- Keep `AL-007-S09` blocked by active dependencies `AL-004-S02`, `AL-002-S16`, and `AL-005-S01`.

## Approval Gate

Reply `OK EXECUTE AL-004-S02` to authorize execution of this TDD plan.

Reply `CHANGE AL-004-S02` with corrections to revise the plan.

Generic `OK` approves the plan content only. It does not authorize execution.
