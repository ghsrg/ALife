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
| `None selected` | `blocked` | `high` | `docs/delivery/roadmap.md` | Select one Candidate Next Work item before starting the next execution slice. |

## In Progress

| Plan ID | Status | Confidence | Source | Notes |
| --- | --- | --- | --- | --- |
| `AL-003-S02` | `planned` | `medium` | `docs/delivery/roadmap.md` | Candidate next: Genome Runtime contract before scheduler/cadence work. |
| `AL-004-S01` | `planned` | `medium` | `docs/delivery/roadmap.md` | Candidate next: Observer vocabulary/source/ownership matrix. |
| `AL-004-S02` | `planned` | `medium` | `docs/delivery/roadmap.md` | Candidate next after `AL-004-S01`; blocks richer projection-dependent UI. |

## Blocked

| Plan ID | Blocker | Source | Notes |
| --- | --- | --- | --- |
| `AL-007-S09` | Requires projection/keyframe contract decisions from `AL-004-S02` and Runner hardening context from `AL-002-S16`. | `docs/delivery/roadmap.md` | Do not start UI-2A until dependencies are planned or explicitly accepted as staged. |

## Ready Next

| Order | Plan ID | Dependency note | Source |
| --- | --- | --- | --- |
| 1 | `AL-003-S02` | Follows `AL-003-S01`; should absorb Genome runtime debt before scheduler/cadence. | `docs/delivery/roadmap.md` |
| 2 | `AL-004-S01` | Needed before versioned projection envelope. | `docs/delivery/roadmap.md` |
| 3 | `AL-004-S02` | Depends on `AL-004-S01`; blocks `AL-007-S09`. | `docs/delivery/roadmap.md` |
| 4 | `AL-002-S16` | Runner hardening remains a dependency for reconnect/remote UI behavior. | `docs/delivery/roadmap.md` |

## Recently Closed

| Plan ID | Status | Evidence | Notes |
| --- | --- | --- | --- |
| `AL-001-S04` | `done` | `outputs/worklogs/2026-07-20-1154-REPORT-al-001-s04-delivery-control-baseline-refactor.md` | Delivery-control artifact responsibilities separated; current/next acceptance normalized to slice IDs. |
| `AL-007-S08` | `done` | `outputs/worklogs/2026-07-19-1937-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md` | Start demo, screenshot export, and acceptance hardening are closed for the current UI-1D scope. |

## Control Drift Notes

| Area | Status | Notes |
| --- | --- | --- |
| Historical evidence | `done` | Historical worklog-derived roadmap rows live in `docs/delivery/worklog-ledger.md`; `docs/delivery/roadmap.md` keeps only a pointer. |
| Acceptance matrix | `done` | Current/next acceptance uses slice-level `AL-###-S##-AC##` IDs. Non-next legacy rows remain untouched until their Plan ID is selected. |
| Source map/control vocabulary | `done` | Delivery-control vocabulary distinguishes stage IDs (`AL-###`) from executable slice Plan IDs (`AL-###-S##`). |
