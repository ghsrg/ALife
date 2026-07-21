---
tags:
  - alife
  - delivery/status
---

# Delivery Status

Operational dashboard for active delivery-control work. This file is not the
canonical roadmap and must not introduce new requirements. Canonical status,
confidence, dependencies, and scope live in `docs/delivery/roadmap.md`.

## Current Focus

| Plan ID | Status | Confidence | Source | Notes |
| --- | --- | --- | --- | --- |
| None selected | `idle` | `high` | `docs/delivery/roadmap.md` | No active Plan ID is selected. `AL-004-S01` is closed; prepared plans remain in `Ready Next` until explicitly selected for execution. |

## Operational Rules

- `Current Focus` is the single selected Plan ID for planning or execution.
- `planned` in `Current Focus` means a plan exists or is being reviewed, but
  implementation has not started.
- `Planning` contains only selected Plan IDs with a concrete plan artifact.
- `Planning` uses only an explicit `None selected` empty-state row when `Current Focus` is `None selected`.
- `Ready Next` contains candidates from `Candidate Next Work` that are not yet
  selected and should not be treated as active work.
- `Blocked` appears after `Ready Next` and names the dependency or decision that
  prevents execution.
- Non-next legacy rows in `docs/delivery/acceptance.md` stay untouched until
  their Plan ID becomes current or Candidate Next Work.

## Planning

| Plan ID | Status | Confidence | Source | Notes |
| --- | --- | --- | --- | --- |
| None selected | `idle` | `high` | `docs/delivery/roadmap.md` | Select a Plan ID from `Ready Next` before planning or execution resumes. |

## Ready Next

| Order | Plan ID | Dependency note | Source |
| --- | --- | --- | --- |
| 1 | `AL-003-S05` | Observer-facing evidence boundary is now closed by `AL-004-S01`; lineage event log/replay can be planned. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md` |
| 2 | `AL-004-S02` | Versioned projection envelope can be planned on top of the closed Observer inventory, including top-level projection/entity/source/completeness vocabulary and Rust-only vs generated-schema decision. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md` |
| 3 | `AL-002-S16` | Runner hardening TDD plan exists and remains ready for execution before reconnect/remote UI behavior depends on it. | `docs/delivery/roadmap.md`, `outputs/worklogs/2026-07-20-2134-PLAN-al-002-s16-runner-4-remote-viewer-acceptance-hardening.md` |

## Blocked

| Plan ID | Blocker | Source | Notes |
| --- | --- | --- | --- |
| `AL-007-S09` | Requires projection/keyframe contract decisions from `AL-004-S02` and Runner hardening context from `AL-002-S16`. | `docs/delivery/roadmap.md` | Do not start UI-2A until dependencies are planned or explicitly accepted as staged. |

## Recently Closed

| Plan ID | Status | Evidence | Notes |
| --- | --- | --- | --- |
| `AL-004-S01` | `done` | `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md` | Static Observer contract inventory now covers current metric fields, coverage statuses, warning dispositions, and Runner live-frame field ownership without entering Core behavior. |
| `AL-003-S04` | `done` | `outputs/worklogs/2026-07-20-1741-REPORT-al-003-s04-genome-copying-mutation-repair.md` | Material-backed Genome copying, deterministic bounded mutation, division copy gate, and conservative sweeper scenario are closed. |
| `AL-003-S03` | `done` | `outputs/worklogs/2026-07-20-1513-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md` | Scheduled Genome Runtime cadence, cached `ActionPlan` reuse, due-Tick trace emission, and no every-Tick recomputation are closed. |
| `AL-003-S02` | `done` | `outputs/worklogs/2026-07-20-1244-REPORT-al-003-s02-genome-runtime-contract-output-coverage.md` | Genome Runtime contract, output disposition, deferred output rejection, capability masks, and minimal debug trace are closed. |
| `AL-001-S04` | `done` | `outputs/worklogs/2026-07-20-1154-REPORT-al-001-s04-delivery-control-baseline-refactor.md` | Delivery-control artifact responsibilities separated; current/next acceptance normalized to slice IDs. |
| `AL-007-S08` | `done` | `outputs/worklogs/2026-07-19-1937-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md` | Start demo, screenshot export, and acceptance hardening are closed for the current UI-1D scope. |

## Control Drift Notes

| Area | Status | Notes |
| --- | --- | --- |
| Historical evidence | `done` | Historical worklog-derived roadmap rows live in `docs/delivery/worklog-ledger.md`; `docs/delivery/roadmap.md` keeps only a pointer. |
| Acceptance matrix | `done` | Current/next acceptance uses slice-level `AL-###-S##-AC##` IDs. Non-next legacy rows remain untouched until their Plan ID is selected. |
| Source map/control vocabulary | `done` | Delivery-control vocabulary distinguishes stage IDs (`AL-###`) from executable slice Plan IDs (`AL-###-S##`). |
| Stage status sync | `done` | `AL-001` stage status now reflects that all current `AL-001-S01` through `AL-001-S04` slices are closed. |
