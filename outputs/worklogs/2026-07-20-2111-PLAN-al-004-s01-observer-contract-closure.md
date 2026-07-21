---
tags:
  - alife
  - worklog/plan
  - plan/AL-004-S01
---

# PLAN: AL-004-S01 Observer Contract Closure

## TDD_PLAN_PROPOSAL

Plan ID: `AL-004-S01`

Selected slice: Observer Contract Closure

Current roadmap status: `in-progress`

Goal: close the shared Observer vocabulary/source/ownership matrix so Core, `sweep_analyzer`, Runner projections, reports, and UI-facing names use one auditable Observer contract before `AL-003-S05` lineage replay and `AL-004-S02` projection envelope work.

Architecture: add a small Observer contract inventory in Rust that declares canonical Observer fields, their source owner, consumer surface, projection readiness, and warning/status vocabulary. Then make existing Observer feature extraction, sweep/analyzer output expectations, and Runner frame projection names test against that inventory. This slice must not introduce new projection envelope behavior, mutate Core state, add organism authority, or change simulation mechanics.

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/observer/observer-layer.md`
3. `docs/observer/mechanism-coverage.md`
4. `docs/observer/projection-contract.md`
5. `docs/observer/behavior-profile-balance.md`
6. `docs/observer/classification-contract.md`
7. `docs/mechanics/observer-projection.md`
8. `docs/implementation/implementation-phases.md`
9. Existing code/tests as evidence: `src/core/summary.rs`, `src/observer/*`, `src/bin/sweep_analyzer.rs`, `src/runner/projections.rs`, `tests/*observer*`, `tests/*sweep*`, `tests/runner_projection_world_frame.rs`.

Worklogs are evidence only, not source of truth.

## Files Read

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/delivery/source-map.md`
- `docs/INDEX.md`
- `docs/PRINCIPLES.md`
- `docs/observer/INDEX.md`
- `docs/observer/observer-layer.md`
- `docs/observer/mechanism-coverage.md`
- `docs/observer/projection-contract.md`
- `docs/observer/behavior-profile-balance.md`
- `docs/observer/classification-contract.md`
- `docs/mechanics/observer-projection.md`
- `docs/implementation/implementation-phases.md`
- `src/core/summary.rs`
- `src/observer/projection.rs`
- `src/runner/projections.rs`
- `src/bin/sweep_analyzer.rs`
- `tests/phase2_sweep_observer_outputs.rs`
- `tests/phase2g_observer_outputs.rs`
- `tests/phase2h_observer_outputs.rs`
- `tests/runner_projection_world_frame.rs`

## Assumptions

- The closure artifact should be executable Rust code plus tests, not only a Markdown matrix.
- `src/observer/contract.rs` can be added as the canonical implementation-level inventory for currently exposed Observer names.
- Existing `MetricsSummary` remains Core-owned raw metric evidence. Observer owns feature names, coverage statuses, warning codes, consumer mapping, and projection readiness.
- `WorldFrameProjection` remains Runner-owned live frame shape until `AL-004-S02`; this slice only maps its fields to Observer vocabulary/provenance.
- `Needs Review`: whether `ENVIRONMENT_DOMINATED_RESULT`, `LOW_INFORMATION_SWEEP`, `BALANCE_ERROR`, and material-profile warning strings should become stable Observer warning codes or legacy analyzer-only warnings. This plan starts by inventorying them and failing on unregistered active warnings; execution should decide whether to register or rename them.

## Forbidden Scope

- Do not implement the `AL-004-S02` projection envelope.
- Do not add new UI behavior or UI projection kinds.
- Do not implement lineage replay, OrganismView projection, storage, Parquet, SQLite, or long-run experiments.
- Do not feed Observer labels, warnings, classifications, or coverage status into Core behavior, Genome Runtime, Feasibility, Scheduler, or process selection.
- Do not rewrite `sweep_analyzer` as a new analysis engine.
- Do not claim behavior-profile balance if only mechanism coverage is verified.

## BDD Agent Scenario Cards

### `AL-004-S01-AC01`: Shared Observer Vocabulary Inventory

Source links: `docs/observer/observer-layer.md`, `docs/observer/mechanism-coverage.md`, `docs/observer/projection-contract.md`, `src/core/summary.rs`, `src/observer/projection.rs`.

Priority: P1

Intent: every Observer-visible metric/feature/warning/status has one canonical name and owner before downstream projections depend on it.

Given Core exposes `MetricsSummary`, Observer derives features, and `sweep_analyzer` writes CSV/report fields  
When a test compares implemented fields against the Observer contract inventory  
Then every active Observer-facing field is classified by `field_id`, source owner, consumer surface, readiness, and provenance expectation.

TDD obligation: add a failing inventory coverage test before adding the inventory or wiring code.

Evidence IDs: `AL-004-S01-EV01`, `AL-004-S01-EV02`.

### `AL-004-S01-AC02`: Mechanism Coverage Status And Warning Vocabulary

Source links: `docs/observer/mechanism-coverage.md`, `docs/observer/observer-layer.md`, `src/bin/sweep_analyzer.rs`, `tests/phase2_sweep_observer_outputs.rs`.

Priority: P1

Intent: analyzer warnings/statuses must be stable and auditable; legacy or analyzer-only strings must be explicitly classified, not silently treated as canonical Observer terms.

Given mechanism coverage docs define allowed statuses and warning codes  
When tests scan active analyzer warning/status strings and Observer contract definitions  
Then all active canonical warnings/statuses are registered, and any legacy analyzer-only warning has an explicit `legacy_analyzer_warning` disposition.

TDD obligation: add RED tests that fail on currently unregistered active analyzer warnings.

Evidence IDs: `AL-004-S01-EV03`, `AL-004-S01-EV04`.

### `AL-004-S01-AC03`: Runner Projection Field Ownership

Source links: `docs/observer/projection-contract.md`, `docs/runner/projections.md`, `src/runner/projections.rs`, `tests/runner_projection_world_frame.rs`.

Priority: P1

Intent: current live frame fields must have explicit ownership/provenance without pretending the full versioned projection envelope exists.

Given Runner currently emits `WorldFrameProjection v2` with `committed_tick`, `projection_sequence`, `heat`, `waste`, and `cells`  
When tests compare Runner frame fields with the Observer contract inventory  
Then each field is mapped to source owner `runner_live_frame` or `core_committed_snapshot`, and fields not yet envelope-compliant are marked as `AL-004-S02` follow-up rather than silently canonical.

TDD obligation: add RED test for runner projection field mapping before implementing inventory rows.

Evidence IDs: `AL-004-S01-EV05`, `AL-004-S01-EV06`.

### `AL-004-S01-AC04`: Observer Boundary Non-Authority Guard

Source links: `docs/observer/observer-layer.md`, `docs/mechanics/observer-projection.md`, `docs/PRINCIPLES.md`, existing Core tests.

Priority: P1

Intent: closing Observer contracts must not change simulation behavior.

Given Observer reads committed state and may produce reports/projections  
When the new contract inventory and tests are added  
Then Core tick behavior, feasibility, Genome Runtime, and Runner frame generation remain independent of Observer labels/warnings.

TDD obligation: add characterization tests or extend existing tests to prove contract inventory is read-only and does not enter `TickExecutor`, `WorldState`, `ActionPlan`, or `GenomeRuntimeInputs`.

Evidence IDs: `AL-004-S01-EV07`, `AL-004-S01-EV08`.

### `AL-004-S01-AC05`: Delivery Closure And Handoff

Source links: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`.

Priority: P1

Intent: `AL-004-S01` closure must unblock `AL-003-S05` and `AL-004-S02` with explicit remaining gaps.

Given contract inventory tests pass and delivery docs are updated  
When closure verification runs  
Then roadmap/status/acceptance/ledger identify `AL-004-S01` as closed or partial, and any unclosed projection envelope work remains in `AL-004-S02`.

TDD obligation: no production behavior; use delivery-lint and closure-verification after implementation tests.

Evidence IDs: `AL-004-S01-EV09`, `AL-004-S01-EV10`.

## Proposed File Plan

Create:

- `src/observer/contract.rs`: canonical implementation-level Observer vocabulary inventory.
- `tests/observer_contract_closure.rs`: contract inventory, warning/status, Runner field mapping, and boundary tests.

Modify:

- `src/observer/mod.rs`: expose `contract`.
- `src/observer/projection.rs`: use contract field IDs where feature extraction currently emits raw strings.
- `src/bin/sweep_analyzer.rs`: optionally expose active warning/status vocabulary through a small function if tests cannot inspect strings cleanly.
- `tests/phase2g_observer_outputs.rs`, `tests/phase2h_observer_outputs.rs`: update expectations only when inventory field IDs intentionally replace duplicated literal strings.
- `tests/phase2_sweep_observer_outputs.rs`: add/adjust coverage for analyzer warning/status fields.
- `tests/runner_projection_world_frame.rs`: add mapping assertion; do not change frame payload in this slice unless tests expose a direct naming/provenance bug.
- `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, `outputs/worklogs/index.md`: update only during execution/closure.

Do not create a new Canon doc unless execution discovers an unavoidable semantic gap. If that happens, stop with `Needs Review` rather than silently changing requirements.

## Numbered TDD Tasks

### `AL-004-S01-T01`: RED for `AL-004-S01-AC01`

- Add `tests/observer_contract_closure.rs`.
- Write failing test `observer_contract_covers_metrics_summary_feature_fields`.
- Expected RED: `alife::observer::contract` module or inventory APIs do not exist, or feature fields are missing from the inventory.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure observer_contract_covers_metrics_summary_feature_fields
```

- Capture result as `AL-004-S01-EV01`.

### `AL-004-S01-T02`: GREEN for `AL-004-S01-AC01`

- Add `src/observer/contract.rs` with:
  - `ObserverFieldId`
  - `ObserverSourceOwner`
  - `ObserverConsumerSurface`
  - `ObserverReadiness`
  - `ObserverFieldSpec`
  - `observer_field_specs()`
  - `observer_field_by_id()`
- Register current `metrics_summary_features()` names, including reaction, resource, fragment, repair, joint, heat, and material degradation fields.
- Expose module through `src/observer/mod.rs`.
- Run the same test and capture pass as `AL-004-S01-EV02`.

### `AL-004-S01-T03`: RED for `AL-004-S01-AC02`

- Add failing tests:
  - `observer_contract_declares_allowed_coverage_statuses`
  - `active_sweep_analyzer_warning_codes_are_registered_or_marked_legacy`
- Expected RED: active warning/status vocabulary is not exposed or not registered.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure observer_contract_declares_allowed_coverage_statuses active_sweep_analyzer_warning_codes_are_registered_or_marked_legacy
```

- Capture result as `AL-004-S01-EV03`.

### `AL-004-S01-T04`: GREEN for `AL-004-S01-AC02`

- Extend `src/observer/contract.rs` with:
  - `CoverageStatus`
  - `ObserverWarningCode`
  - `WarningDisposition`
  - `coverage_status_specs()`
  - `warning_code_specs()`
- Register doc-defined canonical codes from `docs/observer/mechanism-coverage.md`.
- Classify existing analyzer-only strings as `legacy_analyzer_warning` if they are not Canon warning codes.
- If needed, expose a minimal `sweep_analyzer_warning_codes_for_contract()` helper without changing analyzer output semantics.
- Run the same tests and capture pass as `AL-004-S01-EV04`.

### `AL-004-S01-T05`: RED for `AL-004-S01-AC03`

- Add failing test `runner_world_frame_fields_have_observer_contract_mapping`.
- Assert current `WorldFrameProjection` fields map to either Core committed snapshot source or Runner live-frame metadata source.
- Expected RED: field mapping does not exist.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure runner_world_frame_fields_have_observer_contract_mapping
```

- Capture result as `AL-004-S01-EV05`.

### `AL-004-S01-T06`: GREEN for `AL-004-S01-AC03`

- Add Runner frame field specs to `src/observer/contract.rs`:
  - `schema_version`
  - `committed_tick`
  - `projection_sequence`
  - `wall_clock_generated_at_ms`
  - `previous_committed_tick`
  - `heat`
  - `waste`
  - `cells`
  - `cells.id`
  - `cells.x`
  - `cells.y`
  - `cells.radius`
  - `cells.energy`
  - `cells.lifecycle`
- Mark missing full envelope fields as `follow_up_plan_id = AL-004-S02`, not as implemented.
- Run the same test and capture pass as `AL-004-S01-EV06`.

### `AL-004-S01-T07`: RED for `AL-004-S01-AC04`

- Add failing/characterization tests:
  - `observer_contract_is_static_and_read_only`
  - `observer_contract_does_not_enter_genome_runtime_inputs`
  - `observer_contract_does_not_change_runner_frame_projection_hash`
- Expected RED may be a missing helper or missing invariant assertion, not a behavior failure.
- Run:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure observer_contract_is_static_and_read_only observer_contract_does_not_enter_genome_runtime_inputs observer_contract_does_not_change_runner_frame_projection_hash
```

- Capture result as `AL-004-S01-EV07`.

### `AL-004-S01-T08`: GREEN for `AL-004-S01-AC04`

- Implement minimal static inventory with no references from Core hot-path modules to Observer contract.
- If tests need dependency checks, keep them in tests via source scanning or compile-time module boundaries, not runtime behavior.
- Run the same tests and existing regression targets:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure --test phase2g_observer_outputs --test phase2h_observer_outputs --test phase2_sweep_observer_outputs --test runner_projection_world_frame
```

- Capture pass as `AL-004-S01-EV08`.

### `AL-004-S01-T09`: REFACTOR/Docs for `AL-004-S01-AC05`

- Refactor duplicated string literals in Observer tests only after all tests are green.
- Update delivery files after successful implementation verification:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
  - `outputs/worklogs/index.md`
- Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-004-s01-observer-contract-closure.md`.
- Run deterministic delivery lint and `git diff --check`.
- Capture as `AL-004-S01-EV09`.

### `AL-004-S01-T10`: Closure Verification

- Run targeted verification:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure --test phase2_observer_config --test phase2_observer_role_classifier --test phase2_observer_behavior_classifier --test phase2_observer_balance --test phase2g_observer_outputs --test phase2h_observer_outputs --test phase2_sweep_observer_outputs --test runner_projection_world_frame --test scheduler_observer_cadence
```

- Attempt broader verification if practical:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test
```

- If full `cargo test` times out, record timeout explicitly and rely only on targeted evidence.
- Run closure-verification and capture `AL-004-S01-EV10`.

## Verification Commands

Primary:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test observer_contract_closure
```

Regression:

```powershell
$env:CARGO_TARGET_DIR='target\codex-al004s01'; cargo test --test phase2_observer_config --test phase2_observer_role_classifier --test phase2_observer_behavior_classifier --test phase2_observer_balance --test phase2g_observer_outputs --test phase2h_observer_outputs --test phase2_sweep_observer_outputs --test runner_projection_world_frame --test scheduler_observer_cadence
```

Delivery:

```powershell
git diff --check
```

## Open Questions

1. `Needs Review`: should currently active non-Canon analyzer strings such as `ENVIRONMENT_DOMINATED_RESULT`, `LOW_INFORMATION_SWEEP`, `BALANCE_ERROR`, `LOCAL_INTERACTION_NOT_ACTIVATED`, and `PROFILE_EFFECT_FLAT` become stable Observer warning codes, or remain legacy analyzer warnings with explicit disposition?
2. `Needs Review`: should `heat_peak_temperature` remain the Observer feature name for `MetricsSummary.heat`, or should this slice introduce an alias/mapping while preserving backward compatibility?
3. `Needs Review`: should the inventory be purely Rust (`src/observer/contract.rs`) or also exported later as JSON/TOML? This plan keeps export out of scope unless tests show downstream consumers need it now.

## Approval Gate

Reply `OK EXECUTE AL-004-S01` to authorize execution of this TDD plan.

Reply `CHANGE AL-004-S01` with corrections to revise the plan.

Generic `OK` approves the plan content only. It does not authorize execution.
