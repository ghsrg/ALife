---
tags:
  - alife
  - worklog/report
  - delivery/al-007
---

# REPORT: AL-007-S20 Start Track Residual Visual Gap Disposition

## Purpose

Execute `AL-007-S20` as a Start-track residual UI hardening slice after the human disposition decisions:

- disabled future workspace tabs are visual debt and must stay visible, disabled, and explicit;
- Start full-screen belongs in Start now;
- simulation rate and visualization FPS display must be visible;
- unavailable workspace presentation must be improved.

Worklogs are evidence only, not source of truth.

## Plan ID

`AL-007-S20`

## Source Documents Read

- [[docs/PRINCIPLES]]
- [[docs/delivery/roadmap]]
- [[docs/delivery/status]]
- [[docs/delivery/acceptance]]
- [[docs/implementation/implementation-plan-ui]]
- [[docs/ui/principles]]
- [[docs/ui/quality]]
- [[docs/ui/visualization]]
- [[docs/ui/control-center-design-spec]]
- [[outputs/worklogs/2026-07-19-1937-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening]]

## Selected Slice

`Start Track Residual Visual Gap Disposition`

This was implemented as Start UI presentation and acceptance hardening only. No Core, Observer, storage, or Runner server behavior changed.

## Changed Files Summary

- `ui/control-center/src/runner/apiClient.ts`: mapped optional Runner `ticks_per_second` into `RunStatus.ticksPerSecond`.
- `ui/control-center/src/components/RunControls.tsx`: added visible simulation rate and Viewer FPS target display.
- `ui/control-center/src/components/AppShell.tsx`: kept future workspace tabs visible, disabled, and annotated with unavailable reasons.
- `ui/control-center/src/components/MonitorWorkspace.tsx`: added Start full-screen toggle on the monitor workspace.
- `ui/control-center/src/uiText.ts`: added Start residual UI labels.
- `ui/control-center/src/styles/layout.css` and `ui/control-center/src/styles/components.css`: adjusted header/toolbar layout for the new controls.
- `ui/control-center/src/runner/apiClient.test.ts`, `ui/control-center/src/App.test.tsx`, `ui/control-center/src/components/MonitorWorkspace.test.tsx`: added RED/GREEN coverage for status mapping, unavailable workspaces, rate/FPS display, and full-screen.
- `docs/delivery/acceptance.md`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/worklog-ledger.md`, `outputs/worklogs/index.md`: updated delivery closure evidence and Candidate Next Work.

## Verification Commands And Results

| Evidence ID | Command | Result |
| --- | --- | --- |
| `AL-007-S20-EV01-RED` | `npm.cmd test -- src/runner/apiClient.test.ts src/App.test.tsx src/components/MonitorWorkspace.test.tsx` | FAIL as expected after RED tests: missing `ticksPerSecond` mapping, missing Start full-screen button, and missing new UI labels. Initial sandbox-only config access failure was infrastructure, then rerun with escalation produced the expected RED failures. |
| `AL-007-S20-EV02-GREEN` | `npm.cmd test -- src/runner/apiClient.test.ts src/App.test.tsx src/components/MonitorWorkspace.test.tsx` | PASS: 3 files, 31 tests passed after implementation and test cleanup. |
| `AL-007-S20-EV03` | `npm.cmd test` | PASS: 28 files, 131 tests passed. |
| `AL-007-S20-EV04` | `npm.cmd run build` | PASS: `tsc -b && vite build` completed. |
| `AL-007-S20-EV05` | `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` | PASS: 9 Playwright tests passed, including 1024x768 Monitor smoke and selected UI-1C-A visual acceptance. |

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task ID(s) | Evidence ID(s) | Test/Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-007-S20` | Runner `ticks_per_second` is consumed as UI-visible simulation rate without treating rendering FPS as simulation truth. | `AL-007-S20-AC01` | `AL-007-S20-T01`, `AL-007-S20-T02` | `AL-007-S20-EV01-RED`, `AL-007-S20-EV02-GREEN`, `AL-007-S20-EV03` | RED/GREEN `apiClient` and `App` tests; full Vitest. | covered |
| `AL-007-S20` | Disabled future workspaces stay visible and disabled with explicit unavailable reasons. | `AL-007-S20-AC01` | `AL-007-S20-T03`, `AL-007-S20-T04` | `AL-007-S20-EV01-RED`, `AL-007-S20-EV02-GREEN`, `AL-007-S20-EV03`, `AL-007-S20-EV05` | RED/GREEN App test; full Vitest; Playwright visual regression suite. | covered |
| `AL-007-S20` | Start full-screen is available from the Monitor workspace without changing Core or projection semantics. | `AL-007-S20-AC01` | `AL-007-S20-T05`, `AL-007-S20-T06` | `AL-007-S20-EV01-RED`, `AL-007-S20-EV02-GREEN`, `AL-007-S20-EV04`, `AL-007-S20-EV05` | RED/GREEN MonitorWorkspace test; build; Playwright visual regression suite. | covered |
| `AL-007-S20` | Residual unavailable projection fields remain explicit and routed to later UI slices. | `AL-007-S20-AC01` | `AL-007-S20-T07` | `AL-007-S20-EV03`, `AL-007-S20-EV05` | Existing projection truth and visual acceptance tests remain green. | covered |
| `AL-007-S20` | Roadmap/status/acceptance reflect closure and Candidate Next Work is reviewed. | `AL-007-S20-AC01` | `AL-007-S20-T08` | `AL-007-S20-EV06` | Delivery docs updated in this closure pass. | covered |

## Deferred Scope

- Full-world screenshot metadata/legend remains deferred; current Start supports current viewport PNG and honest status.
- Live resource/material/process/contact/joint payloads remain Observer/UI-2 scope.
- Rich Cell/Resource/Material/Process inspectors remain `AL-007-S11`.
- Versioned projections, keyframes, stale/unavailable historical Tick handling, and bounded history remain `AL-007-S09`.
- Debug recovery diagnostics and multi-seed queues remain `AL-007-S14`.
- Balance/warning projections remain `AL-004-S05` and their UI remains `AL-007-S12`.
- Research workspaces remain `AL-007-S15` through `AL-007-S19`.

## Status Update Recommendation

Closure outcome: `PASS`.

Recommended updates applied:

- mark `AL-007-S20` as `done` in [[docs/delivery/roadmap]];
- move `AL-007-S20` to `Recently Closed` in [[docs/delivery/status]];
- remove `AL-007-S20` from `Candidate Next Work`;
- keep `AL-007-S09` as the next UI planning candidate;
- add this report to [[outputs/worklogs/index]] and [[docs/delivery/worklog-ledger]].
