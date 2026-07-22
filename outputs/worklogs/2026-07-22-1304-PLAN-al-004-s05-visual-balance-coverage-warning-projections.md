---
tags:
  - alife
  - worklog/plan
  - delivery/tdd
  - observer/projection
plan_id: AL-004-S05
status: planned
---

# PLAN: AL-004-S05 Visual, Balance, Coverage, And Warning Projections

## TDD_PLAN_PROPOSAL

Plan ID: `AL-004-S05`

Selected slice: `Visual, Balance, Coverage, And Warning Projections`

Current roadmap status: `planned`

## Goal

Build bounded, typed, read-only Observer projection payloads for visual world data, mechanism coverage, behavior balance, warnings, and classification provenance so UI Debug and later analytics consume explicit Observer truth instead of inventing values from live frame fragments.

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/observer/observer-layer.md`
3. `docs/observer/projection-contract.md`
4. `docs/observer/mechanism-coverage.md`
5. `docs/observer/behavior-profile-balance.md`
6. `docs/observer/classification-contract.md`
7. `docs/delivery/roadmap.md`
8. Existing Rust contracts and tests under `src/observer/`, `src/runner/`, `src/core/summary.rs`, and `tests/`

Worklogs are historical evidence only. The direct predecessor closure is `outputs/worklogs/2026-07-22-1256-REPORT-al-004-s03-classification-registry-and-provenance.md`.

## Files Likely To Change

- Create: `src/observer/payloads.rs`
  - Typed projection payload structs, source/provenance records, warning/coverage/balance/classification payload models, and bounded visual layer payload models.
- Modify: `src/observer/mod.rs`
  - Export the new payload module.
- Modify: `src/observer/contract.rs`
  - Register AL-004-S05-owned Observer fields and consumer surfaces for concrete payloads.
- Modify: `src/observer/projection.rs`
  - Add pure builder functions from `CommittedSnapshot`, `MetricsSummary`, and existing classifier/balance results into payload structs.
- Modify: `src/observer/projection_envelope.rs`
  - Reuse existing `ProjectionKind`; add only helper constructors if tests require them. Do not change `WorldFrameProjection` binary compatibility.
- Test: `tests/observer_projection_payloads.rs`
  - New focused TDD coverage for all acceptance scenarios.
- Possibly modify: `tests/observer_contract_closure.rs`, `tests/projection_envelope_contract.rs`
  - Extend inventory checks to include AL-004-S05 fields without weakening AL-004-S01/S02 assertions.

## Rust Domain Model Decisions

- These payloads are observer-only value objects, not Core entities and not mutable simulation state.
- No new stable Core identity is introduced. Payload IDs such as `classification_id`, `finding_id`, or `coverage_projection_id` are artifact/projection identifiers.
- Builders read committed snapshots, metrics summaries, classifier outputs, and analyzer-like records. They must not borrow or mutate `WorldState`.
- Payloads should use owned values (`String`, `Vec`) at the projection boundary; hot-path Core storage must not depend on them.
- Full resource grid dumps are not the default. Visual layer payloads must be bounded or explicitly marked `debug_selected`/`sampled`.

## Assumptions

- `AL-004-S05` should not implement UI rendering; `AL-007-S10` consumes these projections.
- `AL-004-S05` should not implement OrganismView structure; `AL-004-S04` remains separate.
- Existing `BalanceFinding` can be reused but needs projection wrapping/provenance, not a new balance algorithm.
- Existing classification results can be adapted into payloads, but missing `registry_version`, `classification_id`, source metric/projection references, and limitation text must be added at the projection layer.
- Needs Review: exact live transport shape for these payloads may remain Rust-only until `AL-007-S10` chooses UI transport integration.

## BDD Agent Scenario Cards

### AL-004-S05-AC01: Visual World Payload Is Bounded And Source-Backed

Source links:
- `docs/observer/projection-contract.md`
- `docs/observer/observer-layer.md`
- `src/core/snapshot.rs`
- `src/core/summary.rs`

Given a committed snapshot with Cells, heat/waste, and resource layer totals,
When the Observer builds a visual world projection,
Then the payload exposes Cell draw data, lifecycle, energy, resource layer summaries, field summaries, source metric references, and explicit completeness without exposing mutable `WorldState`.

TDD obligation: failing Rust test first for `VisualWorldProjection` envelope metadata, bounded completeness, and source metrics.

Evidence: `AL-004-S05-EV01`, `AL-004-S05-EV02`.

### AL-004-S05-AC02: Classification Projection Preserves Provenance And Limitations

Source links:
- `docs/observer/classification-contract.md`
- `docs/observer/classification-registry.md`
- `outputs/worklogs/2026-07-22-1256-REPORT-al-004-s03-classification-registry-and-provenance.md`

Given an existing classifier result for a Cell or OrganismView interval,
When the Observer converts it to `ClassificationProjection`,
Then the payload includes `classification_id`, `dimension_id`, `entity_type`, `entity_id`, tick interval, mode, labels, confidence, status, evidence summary, `registry_version`, `classifier_version`, source projection/metric references, data completeness, and limitation text.

TDD obligation: failing Rust test first for deterministic IDs and missing-data limitation text.

Evidence: `AL-004-S05-EV03`, `AL-004-S05-EV04`.

### AL-004-S05-AC03: Coverage And Warning Projections Use Canonical Statuses/Codes

Source links:
- `docs/observer/mechanism-coverage.md`
- `docs/observer/observer-layer.md`
- `src/observer/contract.rs`

Given mechanism coverage rows or warning codes,
When the Observer builds coverage/warning projections,
Then statuses and warning codes come from the canonical Observer inventory, unknown codes are not silently accepted, and every warning includes source, severity/disposition, affected mechanism or projection, and recommended rerun context where available.

TDD obligation: failing Rust tests first for accepted canonical codes, legacy disposition, and rejected unknown code behavior.

Evidence: `AL-004-S05-EV05`, `AL-004-S05-EV06`.

### AL-004-S05-AC04: Balance Finding Projection Preserves Equal-Requirements Context

Source links:
- `docs/observer/behavior-profile-balance.md`
- `src/observer/balance.rs`
- `tests/phase2_observer_balance.rs`

Given a `BalanceFinding`,
When it is wrapped as `BalanceFindingProjection`,
Then compared profiles, equal-requirements state, result, evidence metrics, dominance rate, source scenario/report, recommendation, reruns, confidence, and incompleteness are explicit.

TDD obligation: failing Rust test first for projection payload fields and no balance claim when equal requirements are false.

Evidence: `AL-004-S05-EV07`, `AL-004-S05-EV08`.

### AL-004-S05-AC05: Observer Payloads Cannot Enter Core Behavior

Source links:
- `docs/mechanics/observer-projection.md`
- `docs/observer/observer-layer.md`
- `docs/implementation/implementation-phases.md`

Given the new payload module and builders,
When dependency-boundary tests inspect Core behavior sources,
Then Core Tick, Genome Runtime, Feasibility, Process selection, and `WorldState` do not import or depend on Observer payloads.

TDD obligation: failing boundary test first, then keep it green through implementation.

Evidence: `AL-004-S05-EV09`, `AL-004-S05-EV10`.

## Numbered TDD Tasks

### AL-004-S05-T01: RED for AL-004-S05-AC01 visual payload

- Add `tests/observer_projection_payloads.rs`.
- Write a failing test named `visual_world_projection_is_bounded_and_source_backed`.
- Expected failure: `VisualWorldProjection` and builder do not exist.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads visual_world_projection_is_bounded_and_source_backed
```

Capture failure as `AL-004-S05-EV01`.

### AL-004-S05-T02: GREEN for AL-004-S05-AC01

- Create `src/observer/payloads.rs`.
- Add value objects for `VisualWorldProjection`, `VisualCellPayload`, `ResourceLayerSummaryPayload`, `FieldSummaryPayload`, and `ProjectionSourceMetricRef`.
- Add a pure builder in `src/observer/projection.rs` that reads `CommittedSnapshot` and optional `MetricsSummary`.
- Export module from `src/observer/mod.rs`.
- Run the AC01 focused test and capture pass as `AL-004-S05-EV02`.

### AL-004-S05-T03: RED for AL-004-S05-AC02 classification payload

- Add a failing test named `classification_projection_preserves_provenance_and_limitations`.
- Construct a classifier result from existing test fixtures and require deterministic `classification_id`, `registry_version`, source metric references, and limitation text.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads classification_projection_preserves_provenance_and_limitations
```

Capture failure as `AL-004-S05-EV03`.

### AL-004-S05-T04: GREEN for AL-004-S05-AC02

- Add `ClassificationProjectionPayload` and `ClassificationEvidenceSummary`.
- Add a builder that adapts existing `ClassificationResult` without changing classifier semantics.
- Include `registry_version` from loaded registry metadata or an explicit builder parameter.
- Include limitation text for missing source metrics, unavailable registry labels, or insufficient data.
- Run AC02 focused test and capture pass as `AL-004-S05-EV04`.

### AL-004-S05-T05: RED for AL-004-S05-AC03 coverage/warning payloads

- Add failing tests:
  - `coverage_projection_accepts_only_canonical_statuses`
  - `warning_projection_rejects_unknown_warning_codes`
- Require canonical and legacy warning dispositions from `src/observer/contract.rs`.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads coverage_projection_accepts_only_canonical_statuses warning_projection_rejects_unknown_warning_codes
```

Capture failure as `AL-004-S05-EV05`.

### AL-004-S05-T06: GREEN for AL-004-S05-AC03

- Add `CoverageProjectionPayload`, `CoverageMechanismRecordPayload`, `WarningProjectionPayload`, and typed validation errors.
- Reuse `coverage_status_specs()` and `warning_code_specs()` as the only accepted vocabulary.
- Preserve legacy warnings as explicit legacy disposition, not canonical truth.
- Run AC03 focused tests and capture pass as `AL-004-S05-EV06`.

### AL-004-S05-T07: RED for AL-004-S05-AC04 balance payload

- Add a failing test named `balance_projection_preserves_equal_requirements_context`.
- Use existing `evaluate_balance` output and require source scenario/report, equal-requirements field, recommendation, reruns, confidence, and projection completeness.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads balance_projection_preserves_equal_requirements_context
```

Capture failure as `AL-004-S05-EV07`.

### AL-004-S05-T08: GREEN for AL-004-S05-AC04

- Add `BehaviorProfileProjectionPayload` and `BalanceFindingProjectionPayload`.
- Add a builder that refuses or marks `inconclusive` when equal requirements are false.
- Do not change the current balance algorithm beyond projection conversion.
- Run AC04 focused test and capture pass as `AL-004-S05-EV08`.

### AL-004-S05-T09: RED for AL-004-S05-AC05 observer/Core boundary

- Add or extend a boundary test named `observer_payloads_do_not_enter_core_behavior_inputs`.
- Scan `src/core/genome.rs`, `src/core/tick.rs`, `src/core/world.rs`, `src/core/process.rs`, and `src/core/stable_state_hash.rs` for `observer::payloads`, `VisualWorldProjection`, `ClassificationProjectionPayload`, `CoverageProjectionPayload`, and `BalanceFindingProjectionPayload`.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads observer_payloads_do_not_enter_core_behavior_inputs
```

Capture failure or initial pass as `AL-004-S05-EV09`; if it initially passes before implementation, keep it as a characterization guard.

### AL-004-S05-T10: REFACTOR and contract inventory

- Extend `ObserverConsumerSurface` only if needed for concrete payload ownership.
- Add AL-004-S05 follow-up field mappings to `observer_field_specs()`.
- Keep payload structs small and serializable where downstream report/UI consumers need JSON later.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_contract_closure --test projection_envelope_contract --test observer_projection_payloads
```

Capture pass as `AL-004-S05-EV10`.

### AL-004-S05-T11: Full focused regression and docs/report prep

- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo fmt --check
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test phase2_observer_config --test phase2_observer_role_classifier --test phase2_observer_behavior_classifier --test phase2_observer_archetypes --test phase2_observer_balance --test phase2g_observer_outputs --test phase2h_observer_outputs --test observer_contract_closure --test projection_envelope_contract --test observer_projection_payloads
```

- Create a closure report only after implementation and verification.
- Update roadmap/status only through closure verification.

## Verification Commands

Primary:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_contract_closure --test projection_envelope_contract
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo fmt --check
```

Focused regression:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test phase2_observer_config --test phase2_observer_role_classifier --test phase2_observer_behavior_classifier --test phase2_observer_archetypes --test phase2_observer_balance --test phase2g_observer_outputs --test phase2h_observer_outputs
```

## Forbidden Scope

- Do not implement UI Debug rendering; that belongs to `AL-007-S10`.
- Do not implement OrganismView structure; that belongs to `AL-004-S04`.
- Do not add Core behavior dependencies on Observer payloads.
- Do not change ALIF v2 binary frame compatibility.
- Do not persist projection payloads by default; durable analytics export belongs to `AL-005-S02`.
- Do not add new biological shortcuts, species IDs, organs, Cell classes, or behavior labels as Core concepts.
- Do not run broad sweep matrices or generate large output folders as part of this slice.

## Approval Gate

Reply `OK EXECUTE AL-004-S05` to authorize execution of this TDD plan.

Reply `CHANGE AL-004-S05` with corrections to revise the plan.

