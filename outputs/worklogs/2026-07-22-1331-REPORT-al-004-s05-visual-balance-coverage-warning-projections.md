---
tags:
  - alife
  - worklog/report
  - delivery/closure
  - observer/projection
plan_id: AL-004-S05
status: done
---

# REPORT: AL-004-S05 Visual, Balance, Coverage, And Warning Projections

## Outcome

PASS for the bounded Observer payload slice.

This closes the Rust typed payload/building layer needed before UI Debug and analytics consumers render Observer-derived visual, classification, coverage, warning, and balance data. It does not claim UI rendering, storage export, full resource grids, or per-Cell material/internal-resource snapshots.

## Scope Checked

- Delivery plan: `outputs/worklogs/2026-07-22-1304-PLAN-al-004-s05-visual-balance-coverage-warning-projections.md`.
- Delivery docs: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`.
- Canon/contract docs: `docs/observer/observer-layer.md`, `docs/observer/projection-contract.md`, `docs/observer/mechanism-coverage.md`, `docs/observer/behavior-profile-balance.md`, `docs/observer/classification-contract.md`.
- Code/tests: `src/observer/payloads.rs`, `src/observer/projection.rs`, `src/observer/classifiers.rs`, `src/observer/contract.rs`, `src/core/snapshot.rs`, `src/core/summary.rs`, `tests/observer_projection_payloads.rs`, `tests/observer_contract_closure.rs`.
- Worklogs were used only as evidence, not as source of truth.

## Changed Files Summary

- Added `src/observer/payloads.rs` with typed payload structs for visual world data, classification provenance, mechanism coverage, warning disposition, and balance findings.
- Extended `src/observer/projection.rs` with builders for visual world, classification, coverage, warning, and balance payloads.
- Exported `observer::payloads` from `src/observer/mod.rs`.
- Added equality derives for classifier evidence structs so payload tests can assert exact provenance.
- Added `tests/observer_projection_payloads.rs` covering all AL-004-S05 acceptance rows and the Core boundary guard.

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task ID(s) | Evidence ID(s) | Test/Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-004-S05` | Visual world payload is bounded and source-backed. | `AL-004-S05-AC01` | `AL-004-S05-T01`-`T03` | `AL-004-S05-EV01` | `visual_world_projection_is_bounded_and_source_backed`; `src/observer/payloads.rs`; `src/observer/projection.rs` | covered |
| `AL-004-S05` | Classification payload preserves deterministic provenance. | `AL-004-S05-AC02` | `AL-004-S05-T04`-`T05` | `AL-004-S05-EV02` | `classification_projection_keeps_deterministic_provenance` | covered |
| `AL-004-S05` | Coverage and warning payloads use canonical statuses/codes and reject unknown values. | `AL-004-S05-AC03` | `AL-004-S05-T06`-`T07` | `AL-004-S05-EV03` | `coverage_projection_rejects_unknown_statuses`; `warning_projection_preserves_canonical_and_legacy_dispositions` | covered |
| `AL-004-S05` | Balance findings preserve evidence but suppress claims without equal requirements. | `AL-004-S05-AC04` | `AL-004-S05-T08` | `AL-004-S05-EV04` | `balance_projection_does_not_claim_balance_without_equal_requirements` | covered |
| `AL-004-S05` | Observer payloads remain read-only and out of Core behavior. | `AL-004-S05-AC05` | `AL-004-S05-T09`-`T11` | `AL-004-S05-EV05` | `observer_payloads_do_not_enter_core_behavior`; `observer_contract_closure` | covered |

## Verification

```text
cargo fmt --check
PASS

$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_projection_payloads
PASS: 6 tests passed, 0 failed

$env:CARGO_TARGET_DIR='target\codex-al004s05'; cargo test --test observer_contract_closure
PASS: 7 tests passed, 0 failed
```

Note: an earlier isolated-target test attempt timed out while compiling dependencies; the repeated focused tests completed successfully.

## Disposition

- `AL-004-S05` can move to `done`.
- `AL-007-S10` is unblocked for TDD planning as the next UI-2B Debug Visualization Mode and Exact Layers slice.
- `AL-005-S02` and `AL-005-S03` can now depend on typed Observer payloads for analytics/export planning, but they still need separate TDD plans.

## Remaining Debt

- Per-Cell material and internal-resource payload vectors are present but empty because `CommittedSnapshot` currently exposes Cell draw data, energy, lifecycle, heat/waste, and resource layer totals, not per-Cell material/resource breakdowns. This is explicit partial completeness, not fake visual data.
- Exact resource grid/cell sampling remains downstream for `AL-007-S10` and likely needs a follow-up Core/Observer snapshot source decision if UI must inspect exact per-cell materials.
- Classification payload covers concrete provenance fields over existing classifier results, but full registry-vs-config label normalization remains a later Registry v1 hardening concern.

