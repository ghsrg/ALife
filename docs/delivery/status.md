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
| none | n/a | n/a | n/a | No active Plan ID selected after `AL-003-S04` closure. |

## Operational Rules

- `Current Focus` is the single selected Plan ID for planning or execution.
- `planned` in `Current Focus` means a plan exists or is being reviewed, but
  implementation has not started.
- `Planning` contains only selected Plan IDs with a concrete plan artifact.
- `Ready Next` contains candidates from `Candidate Next Work` that are not yet
  selected and should not be treated as active work.
- `Blocked` appears after `Ready Next` and names the dependency or decision that
  prevents execution.
- Non-next legacy rows in `docs/delivery/acceptance.md` stay untouched until
  their Plan ID becomes current or Candidate Next Work.

## Planning

| Plan ID | Status | Confidence | Source | Notes |
| --- | --- | --- | --- | --- |
| none | n/a | n/a | n/a | No selected plan is waiting for execution. |

## Ready Next

| Order | Plan ID | Dependency note | Source |
| --- | --- | --- | --- |
| 1 | `AL-004-S01` | Needed before versioned projection envelope and before `AL-003-S05` lineage Observer boundary. | `docs/delivery/roadmap.md` |
| 2 | `AL-002-S16` | Runner hardening remains a dependency for reconnect/remote UI behavior. | `docs/delivery/roadmap.md` |

## Blocked

| Plan ID | Blocker | Source | Notes |
| --- | --- | --- | --- |
| `AL-003-S05` | Requires `AL-004-S01` Observer vocabulary/source/ownership boundary. | `docs/delivery/roadmap.md` | `AL-003-S04` is closed; do not plan lineage replay before the Observer evidence boundary is explicit. |
| `AL-004-S02` | Requires `AL-004-S01`. | `docs/delivery/roadmap.md` | Blocks `AL-007-S09` and richer Debug/Research UI projection work. |
| `AL-007-S09` | Requires projection/keyframe contract decisions from `AL-004-S02` and Runner hardening context from `AL-002-S16`. | `docs/delivery/roadmap.md` | Do not start UI-2A until dependencies are planned or explicitly accepted as staged. |

## Recently Closed

| Plan ID | Status | Evidence | Notes |
| --- | --- | --- | --- |
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
