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
| None selected | `idle` | `high` | `docs/delivery/roadmap.md` | No active execution focus selected after `AL-002-S16` closure. |

## Operational Rules

- `Current Focus` is the single selected Plan ID for planning or execution.
- `planned` in `Current Focus` means a plan exists or is being reviewed, but
  implementation has not started.
- `in-progress` in `Current Focus` means execution is authorized and underway.
- `Next` mirrors selected `Candidate Next Work` rows with operational statuses.
- `planned-ready` means a concrete TDD plan exists, but the Plan ID is not the
  current focus and execution is not authorized.
- `ready-to-plan` means dependencies are clear enough to draft a TDD plan.
- `blocked-dependency` means the row is not an incident; it is intentionally
  waiting for named prerequisite Plan IDs or decisions.
- `Dependency state` lists only active blockers or active dependencies; closed
  dependencies are omitted to avoid duplicating the roadmap.
- Non-next legacy rows in `docs/delivery/acceptance.md` stay untouched until
  their Plan ID becomes current or Candidate Next Work.

## Next

| Order | Plan ID | Operational status | Dependency state | Source | Notes |
| --- | --- | --- | --- | --- | --- |
| 1 | `AL-005-S01` | <kbd style="background-color: #ffd33d; color: #24292f; border: none;">ready-to-plan</kbd> | none | `docs/delivery/roadmap.md` | Run metadata and storage index should be planned before UI-2A keyframe/history work depends on persisted run semantics. |
| 2 | `AL-002-S11` | <kbd style="background-color: #ffd33d; color: #24292f; border: none;">ready-to-plan</kbd> | none | `docs/delivery/roadmap.md` | Bootstrap rich generators/world families should precede Bootstrap preview/report and World Editor work. |
| 3 | `AL-007-S09` | <kbd style="background-color: #d73a49; color: white; border: none;">blocked-dependency</kbd> | Active blocker: `AL-005-S01`. | `docs/delivery/roadmap.md` | Not a critical incident. It is shown here to keep UI-2A blocked until run metadata/keyframe assumptions are explicit. |

## Recently Closed

| Plan ID | Status | Evidence | Notes |
| --- | --- | --- | --- |
| `AL-002-S16` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-2221-REPORT-al-002-s16-runner-4-remote-viewer-acceptance-hardening.md` | Runner-4 remote viewer opt-in, CORS allowlist, stable HTTP errors, graceful shutdown state, reconnect latest-frame behavior, and status metadata are closed without Core mechanic changes. |
| `AL-004-S02` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-1312-REPORT-al-004-s02-versioned-projection-envelope.md` | Rust-only typed projection envelope, completeness/source vocabulary, Observer inventory mapping, and non-breaking `WorldFrameProjection v2` envelope wrapping are closed; storage/keyframes and generated schemas remain later slices. |
| `AL-003-S05` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-1159-REPORT-al-003-s05-lineage-event-log-and-replay.md` | Deterministic read-only lineage event log and replay summary are closed for founder Cells, Genome copying/mutation, division inheritance, death, and observer-boundary guards. |
| `AL-004-S01` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md` | Static Observer contract inventory now covers current metric fields, coverage statuses, warning dispositions, and Runner live-frame field ownership without entering Core behavior. |
| `AL-003-S04` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1741-REPORT-al-003-s04-genome-copying-mutation-repair.md` | Material-backed Genome copying, deterministic bounded mutation, division copy gate, and conservative sweeper scenario are closed. |
| `AL-003-S03` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1513-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md` | Scheduled Genome Runtime cadence, cached `ActionPlan` reuse, due-Tick trace emission, and no every-Tick recomputation are closed. |
| `AL-003-S02` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1244-REPORT-al-003-s02-genome-runtime-contract-output-coverage.md` | Genome Runtime contract, output disposition, deferred output rejection, capability masks, and minimal debug trace are closed. |
| `AL-001-S04` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1154-REPORT-al-001-s04-delivery-control-baseline-refactor.md` | Delivery-control artifact responsibilities separated; current/next acceptance normalized to slice IDs. |
| `AL-007-S08` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-19-1937-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md` | Start demo, screenshot export, and acceptance hardening are closed for the current UI-1D scope. |

## Control Drift Notes

| Area | Status | Notes |
| --- | --- | --- |
| Historical evidence | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | Historical worklog-derived roadmap rows live in `docs/delivery/worklog-ledger.md`; `docs/delivery/roadmap.md` keeps only a pointer. |
| Acceptance matrix | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | Current/next acceptance uses slice-level `AL-###-S##-AC##` IDs. Non-next legacy rows remain untouched until their Plan ID is selected. |
| Source map/control vocabulary | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | Delivery-control vocabulary distinguishes stage IDs (`AL-###`) from executable slice Plan IDs (`AL-###-S##`). |
| Stage status sync | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `AL-001` and `AL-003` stage statuses reflect closed slice rows with closure evidence. |
