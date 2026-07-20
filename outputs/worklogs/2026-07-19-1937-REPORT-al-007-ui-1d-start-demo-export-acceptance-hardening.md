---
tags:
  - alife
  - worklog/report
  - delivery/al-007
---

# REPORT: AL-007 UI-1D Start Demo Export Acceptance Hardening

## Purpose

Execute `AL-007` / `UI-1D` as a Start-scope demo, screenshot export, and acceptance hardening slice while preserving Runner and Observer ownership boundaries.

Worklogs are evidence only, not source of truth.

## Plan ID

`AL-007`

## Source Documents Read

- [[docs/PRINCIPLES]]
- [[docs/delivery/control]]
- [[docs/delivery/roadmap]]
- [[docs/delivery/status]]
- [[docs/delivery/acceptance]]
- [[docs/delivery/scenario-cards]]
- [[docs/delivery/execution-handoff-al-007]]
- [[docs/implementation/implementation-plan-ui]]
- [[docs/implementation/implementation-plan-runner]]
- [[docs/observer/projection-contract]]
- [[docs/observer/observer-layer]]
- [[outputs/worklogs/2026-07-19-1605-PLAN-al-007-ui-1d-start-demo-export-acceptance-hardening]]

## Dependency Pre-Check

| Dependency | Decision | Evidence |
| --- | --- | --- |
| `AL-002 Runner` | UI consumes current status and `ALIF v2` frames only; no Runner command or protocol behavior was added. | [[docs/implementation/implementation-plan-runner]], [[docs/delivery/execution-handoff-al-007]] |
| `AL-005 Observer` | UI renders existing read-only projections; missing live fields remain explicit. | [[docs/observer/projection-contract]], [[docs/observer/observer-layer]] |

## Changed Files Summary

- `ui/control-center/src/App.test.tsx`: added Start demo provenance, live provenance, and Start-scope screenshot status tests.
- `ui/control-center/src/app/monitorViewModel.ts`: added Start demo provenance labels derived from existing fixture/live state.
- `ui/control-center/src/components/MonitorWorkspace.tsx`: rendered compact Start demo provenance in the viewer toolbar.
- `ui/control-center/src/components/MonitorWorkspace.test.tsx`: added configurable renderer export mock and PNG-unavailable test.
- `ui/control-center/src/components/AppShell.tsx`: changed screenshot export status copy to Start-scope wording.
- `ui/control-center/src/uiText.ts`: added Start demo and Start screenshot text constants.
- `ui/control-center/tests/e2e/monitor.spec.ts`: narrowed existing UI-1A text selector and aligned export status assertion.

## Verification Commands And Results

| Evidence ID | Command | Result |
| --- | --- | --- |
| `AL-007-EV02-RED`, `AL-007-EV03-RED` | `npm.cmd test -- src/App.test.tsx` | FAIL as expected: `Start demo` provenance was absent before implementation. |
| `AL-007-EV02-GREEN`, `AL-007-EV03-GREEN` | `npm.cmd test -- src/App.test.tsx` | PASS: 15 tests passed after Start demo provenance implementation. |
| `AL-007-EV04-RED` | `npm.cmd test -- src/components/MonitorWorkspace.test.tsx` | FAIL as expected: export mock returned PNG instead of `null`. |
| `AL-007-EV04-GREEN` | `npm.cmd test -- src/components/MonitorWorkspace.test.tsx` | PASS: 2 tests passed after configurable renderer mock. |
| `AL-007-EV05-RED` | `npm.cmd test -- src/App.test.tsx` | FAIL as expected: status was `PNG ready (29 bytes)` before Start-scope copy. |
| `AL-007-EV02`, `AL-007-EV04` | `npm.cmd test` | PASS: 28 files, 128 tests passed. |
| `AL-007-EV05` | `npm.cmd run build` | PASS: `tsc -b && vite build` completed. |
| `AL-007-EV03`, `AL-007-EV06` | `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` | PASS: 9 Playwright tests passed. |
| Runner conditional check | `cargo test --test runner_ws_stream --test runner_http_run_control --test runner_frame_encoder` | Not run: Runner code and protocol behavior were not touched. |
| Delivery lint | deterministic wikilink/ID check over `AL-007` closure scope | PASS: local wikilinks resolved; `AL-007` AC/T/EV IDs mapped through scenario, handoff, plan, and report. |

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task IDs | Evidence IDs | Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-007` | Dependency pre-check respects Runner, Observer, and projection boundaries. | `AL-007-AC01` | `AL-007-T01` | `AL-007-EV01` | Dependency pre-check table; no Runner/Observer source changes. | covered |
| `AL-007` | Start demo path is coherent and does not infer unavailable projection data. | `AL-007-AC02` | `AL-007-T02`, `AL-007-T03` | `AL-007-EV02`, `AL-007-EV03` | RED/GREEN App tests; full Vitest; Playwright monitor/e2e. | covered |
| `AL-007` | Screenshot export works within Start scope and reports unavailable state. | `AL-007-AC03` | `AL-007-T04`, `AL-007-T05` | `AL-007-EV04`, `AL-007-EV05` | RED/GREEN MonitorWorkspace and App tests; build. | covered |
| `AL-007` | Acceptance hardening preserves UI-1C behavior and defers Debug/Research scope. | `AL-007-AC04` | `AL-007-T06`, `AL-007-T07` | `AL-007-EV05`, `AL-007-EV06` | Full Vitest, production build, selected Playwright UI-1C-A visual acceptance. | covered |

## Deferred Scope

- Debug CSV/JSON/diagnostic export remains deferred.
- Research reports and experiment export remain deferred.
- Genome, lineage, and OrganismView deep UI remain deferred.
- Runner command/protocol hardening remains owned by `AL-002`.
- Observer projection expansion remains owned by `AL-005`.

## Status Update Recommendation

Closure outcome: `PASS`.

Recommended updates:

- mark `AL-007` as `done-evidenced` in [[docs/delivery/roadmap]] and [[docs/delivery/status]];
- add this report to [[outputs/worklogs/index]] and [[docs/delivery/worklog-ledger]];
- preserve legacy `UI-1D` alias and all historical worklog names.
