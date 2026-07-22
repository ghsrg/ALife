---
tags:
  - alife
  - worklog/report
  - delivery/al-007-s09
---

# REPORT: AL-007-S09 Versioned Projections, Keyframes, And Historical Data

## Purpose

Close `AL-007-S09` for the UI-2A slice: make Control Center distinguish live,
fixture, frozen, stale, and unavailable projection contexts without treating
missing historical ticks as complete state.

Worklogs are evidence only, not source of truth.

## Source Documents Read

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/ui/architecture.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`
- `docs/ui/control-center-design-spec.md`
- `src/observer/projection_envelope.rs`
- `src/storage/mod.rs`

## Changed Files Summary

- Added UI projection context model and tests:
  - `ui/control-center/src/projection/projectionContext.ts`
  - `ui/control-center/src/projection/projectionContext.test.ts`
- Extended UI projection types with historical source and completeness labels:
  - `ui/control-center/src/projection/types.ts`
- Added bounded in-memory frame history and read-only context transitions:
  - `ui/control-center/src/app/appState.ts`
  - `ui/control-center/src/app/appState.test.ts`
- Added Monitor Data Context panel, freeze/history/live controls, and styles:
  - `ui/control-center/src/components/MonitorWorkspace.tsx`
  - `ui/control-center/src/components/MonitorWorkspace.test.tsx`
  - `ui/control-center/src/components/AppShell.tsx`
  - `ui/control-center/src/styles/components.css`
  - `ui/control-center/src/uiText.ts`
- Updated delivery control artifacts:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
  - `outputs/worklogs/index.md`

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task IDs | Evidence ID | Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-007-S09` | Projection context shows source, version, completeness, run, and tick. | `AL-007-S09-AC01` | `AL-007-S09-T01`, `AL-007-S09-T02` | `AL-007-S09-EV01` | `projectionContext.test.ts`, `MonitorWorkspace.test.tsx`, full `npm.cmd test` | covered |
| `AL-007-S09` | Bounded client live history can freeze a frame while live state advances and then jump back to latest live. | `AL-007-S09-AC02` | `AL-007-S09-T03`, `AL-007-S09-T04` | `AL-007-S09-EV02` | `appState.test.ts`, `MonitorWorkspace.test.tsx` | covered |
| `AL-007-S09` | Unavailable historical ticks do not substitute a nearby frame. | `AL-007-S09-AC03` | `AL-007-S09-T05`, `AL-007-S09-T06` | `AL-007-S09-EV03` | `projectionContext.test.ts`, `appState.test.ts`, `MonitorWorkspace.test.tsx` | covered |
| `AL-007-S09` | Stale live context is explicit and read-only after disconnect, then restores on reconnect. | `AL-007-S09-AC04` | `AL-007-S09-T07`, `AL-007-S09-T08` | `AL-007-S09-EV04` | `appState.test.ts`, full `npm.cmd test` | covered |
| `AL-007-S09` | Start/Monitor acceptance remains usable after the new context UI is added. | `AL-007-S09-AC05` | `AL-007-S09-T09` | `AL-007-S09-EV05` | `npm.cmd run build`, selected Playwright Monitor/UI-1C-A e2e | covered |

## Verification

- `npm.cmd test -- src/projection/projectionContext.test.ts`
  - RED: failed because `projectionContext` did not exist.
  - GREEN: 3 tests passed.
- `npm.cmd test -- src/app/appState.test.ts`
  - RED: failed for missing `freezeCurrentFrame`/`selectHistoryTick`.
  - GREEN: 16 tests passed.
  - RED: failed because disconnected live context stayed `live`.
  - GREEN: 17 tests passed.
- `npm.cmd test -- src/components/MonitorWorkspace.test.tsx`
  - RED: failed because `Data Context` UI was missing.
  - GREEN: 4 tests passed.
- `npm.cmd test`
  - 29 test files passed.
  - 138 tests passed.
- `npm.cmd run build`
  - TypeScript build and Vite production build passed.
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts`
  - 9 Playwright tests passed.

## Status Update Recommendation

Mark `AL-007-S09` as `done` with `high` confidence.

## Deferred Scope

- Full storage replay and keyframe payload loading remain out of scope.
- Live resource/material/joint expansion remains downstream Observer/projection work.
- Exact debug layers remain `AL-007-S10`.
- Balance/warning projections remain `AL-004-S05` and analytics UI remains
  `AL-007-S12`.
- Rich entity/resource/material/process inspectors remain `AL-007-S11`.

## Candidate Next Work Review

`AL-007-S09` was removed from Candidate Next Work after closure. `AL-007-S10`
was kept as the next UI-2B slice but is blocked by active dependency
`AL-004-S05`. A circular dependency between `AL-007-S10` and `AL-006-S04` was
removed by treating viewer/projection throughput as downstream performance work,
not a blocker for the first exact-layer UX plan.
