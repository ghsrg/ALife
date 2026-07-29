---
tags:
  - alife
  - worklog/report
  - ui
  - control-center
  - delivery/al-007-s22
---

# AL-007-S22 Monitor Layout Stabilization Report

## Purpose

Close `AL-007-S22` by stabilizing the Monitor layout against the final Control Center references without changing Runner/Core simulation behavior.

Worklogs are evidence, not source of truth. Canon remains in `docs/ui/control-center-design-spec.md` and `docs/ui/control-center-block.md`.

## Plan

- Plan ID: `AL-007-S22`
- Plan file: `outputs/worklogs/2026-07-29-PLAN-al-007-s22-monitor-layout-stabilization.md`
- Selected slice: `AL-007-S22` Monitor Layout Stabilization

## Sources Read

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/ui/INDEX.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/control-center-block.md`
- `docs/delivery/roadmap.md`
- `outputs/worklogs/2026-07-29-PLAN-al-007-s22-monitor-layout-stabilization.md`

## Changed Files

- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/BottomDataPanel.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/components/GlobalNavigation.tsx`
- `ui/control-center/src/components/RunBar.tsx`
- `ui/control-center/src/components/LevelPanel.tsx`
- `ui/control-center/src/components/InspectorPanel.tsx`
- `ui/control-center/src/styles/tokens.css`
- `ui/control-center/src/styles/layout.css`
- `ui/control-center/src/styles/components.css`
- `ui/control-center/src/uiText.ts`
- UI unit and e2e tests under `ui/control-center/src/**/*.test.tsx` and `ui/control-center/tests/e2e/*.spec.ts`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `outputs/worklogs/index.md`

## Implementation Summary

- Added stable Monitor track anchors for Navigation, Run Bar, Level, Layers, Map, Inspector, and Data Panel.
- Removed the old `MonitorWorkspace` tabs (`Map Viewer`, `Analytics`, `Raw Data`).
- Removed Data Panel tab/collapse controls and kept one context-driven analytics surface.
- Removed layout-changing Layers collapse from normal Monitor shell composition.
- Added canonical layout tokens for the Monitor minimum height and fixed track sizing.
- Changed below-minimum behavior to root/page vertical scroll instead of panel collapse or zero-sized tracks.
- Removed the separate `Reset World Viewer navigation` control; `Fit World Viewer` remains.
- Fixed Focus/truth/hit-target pointer layering so overlays do not block Map target selection and projection notices remain dismissible.

## Verification Commands

| Evidence ID | Command | Result |
|---|---|---|
| `AL-007-S22-EV01` | `npm.cmd test -- --run src/components/MonitorWorkspace.test.tsx src/components/BottomDataPanel.test.tsx src/App.test.tsx` before GREEN | RED observed: missing `monitor-map-track` and existing Data Panel tabs. |
| `AL-007-S22-EV02` | `npm.cmd test` | PASS: 49 test files, 200 tests. |
| `AL-007-S22-EV03` | `npm.cmd run build` | PASS: TypeScript and Vite production build completed. Vite emitted existing bundle-size warning for `index-*.js` over 500 kB. |
| `AL-007-S22-EV04` | `npm.cmd run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts` | PASS: 11 Playwright tests. |
| `AL-007-S22-EV05` | Playwright screenshots from `ui-1c-a-visual.spec.ts` | Captured `test-results/ui-1c-a/1920x1080-dark.png`, `1366x862-dark.png`, `1920x1080-light.png`, `1920x1080-semantic-detail.png`, and `1366x862-navigation.png`. |

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task IDs | Evidence IDs | Status |
|---|---|---|---|---|---|
| `AL-007-S22` | Stable reference tracks at `1920x1080` and no zero-size Map/panels after layout changes. | `AL-007-S22-AC01` | `AL-007-S22-T01`, `AL-007-S22-T02` | `AL-007-S22-EV01`, `AL-007-S22-EV02`, `AL-007-S22-EV04`, `AL-007-S22-EV05` | covered |
| `AL-007-S22` | Compact viewport `1366x862`; below threshold uses root/page scroll without collapsing fixed tracks. | `AL-007-S22-AC02` | `AL-007-S22-T03` | `AL-007-S22-EV04`, `AL-007-S22-EV05` | covered |
| `AL-007-S22` | Fullscreen shell groundwork keeps Map/Fit/overlay semantics in scope without normal chrome layout regressions. | `AL-007-S22-AC03` | `AL-007-S22-T04` | `AL-007-S22-EV02`, `AL-007-S22-EV04` | partial |

## Deviations

- `AL-007-S22-AC03` remains partial: the existing DOM fullscreen entry remains, and `Reset` was removed, but a complete Map-only fullscreen shell with optional Data overlay is deferred. This is a consolidated follow-up inside `AL-007-S23` because it depends on explicit interaction state and Focus/Data overlay behavior.
- Run Bar content still contains older control/metric labels (`CONFIG`, `POPULATION`, `LATENCY`) because S22 was limited to layout stabilization. Final Run Bar semantics belong to `AL-007-S23` and `AL-007-S25`.
- Data Panel chart semantics remain provisional; S22 only removed tabs/collapse and stabilized the surface. Source-backed chart truth belongs to `AL-007-S24`.

## Delivery Lint Result

## LINT_RESULT

**Scope:** `AL-007-S22`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `outputs/worklogs/index.md`, report and plan IDs.  
**Mode:** deterministic

| Severity | Rule | ID/Path | Finding | Required action |
|---|---|---|---|---|
| WARN | `DL007` | `AL-007-S22-AC03` | Fullscreen acceptance is only partially covered by this stabilization pass. | Keep `AL-007-S22` closure partial for AC03 and route full interaction shell to `AL-007-S23`. |

**Semantic proposals:** none.

**Remediation proposal:** applied deterministic traceability updates in this report pass.

| Fix ID | Severity | Rules | Files | Applied change | Risk |
|---|---|---|---|---|---|
| `F01` | WARN | `DL007` | `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `outputs/worklogs/index.md` | Add S22 evidence, mark S22 done with partial fullscreen note, move Candidate Next Work to S23. | Low |

**Decision:** PASS_WITH_WARNINGS

## Status Update Recommendation

- Mark `AL-007-S22` as `done` with the fullscreen caveat noted.
- Make `AL-007-S23` the first Candidate Next Work.

## Follow-Up

- `AL-007-S23`: implement final Monitor interaction state, including complete Map fullscreen composition, Level/Layer selection semantics, Focus open/close, Pin behavior, and final Run Bar cleanup.
- `AL-007-S24`: replace provisional Data Panel charts with source-backed context-driven surfaces and unavailable states.
