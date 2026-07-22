---
tags:
  - alife
  - worklog/plan
  - delivery/al-002-s12
---

# PLAN: AL-002-S12 Bootstrap Preview, Reports, And Calibration

## Status

approved-for-execution

## Goal

Expose Bootstrap manifests and compact preview/calibration reports for humans and batch tools without starting Core, mutating `WorldState`, or pretending manifest-only field generators are spatial Core field grids.

## Source Of Truth

- `docs/runner/bootstrap.md`
- `docs/runner/projections.md`
- `docs/runner/scenario-resolution.md`
- `docs/implementation/implementation-plan-bootstrap.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `outputs/worklogs/2026-07-22-1547-REPORT-al-002-s11-rich-spatial-generators-and-world-families.md`

## Acceptance

| Acceptance ID | Outcome |
| --- | --- |
| `AL-002-S12-AC01` | Bootstrap preview prepares Tick 0 and returns manifest/preview data without starting Core or executing ticks. |
| `AL-002-S12-AC02` | Manifest export is stable and human-readable for the same scenario/seed. |
| `AL-002-S12-AC03` | Resource preview is bounded by a configured cell cap and includes layer dimensions/totals/ranges. |
| `AL-002-S12-AC04` | Field generators remain explicit manifest summaries with `BOOTSTRAP_FIELD_LAYER_NOT_CORE_INTEGRATED`; no spatial field grid is claimed. |
| `AL-002-S12-AC05` | Seed-sweep calibration report is deterministic, compact, and does not execute Core ticks. |
| `AL-002-S12-AC06` | Runner CLI exposes the shared preview path as JSON without changing normal run/serve/list behavior. |

## TDD Tasks

- `AL-002-S12-T01`: RED tests for shared preview manifest/resource/field warning behavior.
- `AL-002-S12-T02`: GREEN implementation under `src/bootstrap/preview.rs`.
- `AL-002-S12-T03`: RED tests for seed-sweep calibration limits.
- `AL-002-S12-T04`: GREEN seed-sweep report implementation.
- `AL-002-S12-T05`: RED CLI test for `runner --bootstrap-preview <scenario>`.
- `AL-002-S12-T06`: GREEN CLI integration over the shared preview API.
- `AL-002-S12-T07`: Refactor and verify targeted Rust suite.
- `AL-002-S12-T08`: Update delivery acceptance/status/roadmap/ledger/index and create closure report.

## Forbidden Scope

- Do not implement Core spatial `FieldGrid`.
- Do not add runtime field mechanics or Tick behavior.
- Do not make UI World Editor changes.
- Do not create large unbounded artifacts during tests.
- Do not use preview/calibration output as simulation input.

## Verification

- `cargo fmt --check`
- `cargo test --test bootstrap_preview --test runner_bootstrap_preview_cli --test bootstrap_rich_generators --test bootstrap_integration --test runner_scenario_loader`
- `git diff --check`
