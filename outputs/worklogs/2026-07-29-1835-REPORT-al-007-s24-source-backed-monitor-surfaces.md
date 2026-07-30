---
tags:
  - alife
  - worklog/report
  - ui
---

# AL-007-S24 Source-Backed Monitor Surfaces Report

## Summary

Status: done.

AL-007-S24 replaced fabricated Monitor Data Panel and Layers & Filters surfaces with source-backed view models and explicit unavailable states. The slice stayed UI-only: no Runner/Core/Observer behavior contract was added.

## Implemented Scope

- Added `monitorSurfaceModel` for source-backed/unavailable Data Panel cards, selectors, legends, provenance, and Level-specific surfaces.
- Added UI-only `rrdMetricHistory` for compact metric/trail history:
  - newest 100 samples retained consecutively;
  - older samples collapsed by 10x windows;
  - numeric and point samples aggregate by mean;
  - retained samples capped at 1000.
- Updated app state with `monitorMetricHistory` and `monitorAccountingTarget` without increasing full `frameHistory`.
- Replaced Bottom Data Panel hardcoded fallback chart logic with the source-backed model.
- Replaced Layers & Filters tabs/color mode/fallback presets with canonical grouped controls:
  - Fields;
  - Resources;
  - Cell Energy;
  - Structure;
  - Selection.
- Kept layer toggles presentation-only.
- Updated selected Playwright acceptance to use the UI package `cwd` and S24 canonical expectations.

## Acceptance Matrix

| Acceptance ID | Result | Evidence |
| --- | --- | --- |
| `AL-007-S24-AC01` | pass | `monitorSurfaceModel.test.ts`, `BottomDataPanel.test.tsx`: no fake Monitor values, unavailable Energy Flow, no heuristic role inference. |
| `AL-007-S24-AC02` | pass | `rrdMetricHistory.test.ts`, `appState.test.ts`: RRD retention, mean aggregation, point aggregation, max bounds, full frame history remains bounded. |
| `AL-007-S24-AC03` | pass | `LayerPanel.test.tsx`, selected Playwright: canonical grouped Layers & Filters, no tabs/color mode/fake presets/runner controls. |
| `AL-007-S24-AC04` | pass | `monitorSurfaceModel.test.ts`, `BottomDataPanel.test.tsx`: Level-aware Data Panel card sets with unavailable states for missing contracts. |
| `AL-007-S24-AC05` | pass | `monitorSurfaceModel.test.ts`, `BottomDataPanel.test.tsx`: Energy default, Resource selector options only from source-backed layers, visible source/completeness/unit/reasons. |
| `AL-007-S24-AC06` | pass | `MonitorWorkspace.test.tsx`, `WorldViewer.test.tsx`, selected Playwright: layout, fullscreen, Fit World, page scroll, and layer side-effect invariants preserved. |

## TDD Evidence

| Evidence ID | Result |
| --- | --- |
| `AL-007-S24-EV01` | RED: `npm.cmd --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts --run` failed because the model module did not exist. |
| `AL-007-S24-EV02` | GREEN: `npm.cmd --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run` passed, 6 tests. |
| `AL-007-S24-EV03` | RED: `npm.cmd --prefix ui/control-center test -- src/app/rrdMetricHistory.test.ts --run` failed because the RRD module did not exist. |
| `AL-007-S24-EV04` | GREEN: `npm.cmd --prefix ui/control-center test -- src/app/rrdMetricHistory.test.ts src/app/appState.test.ts src/app/monitorSurfaceModel.test.ts --run` passed, 28 tests. |
| `AL-007-S24-EV05` | RED: `npm.cmd --prefix ui/control-center test -- src/components/LayerPanel.test.tsx --run` failed against old tabs/color mode/fallback layer presets. |
| `AL-007-S24-EV06` | GREEN: `npm.cmd --prefix ui/control-center test -- src/components/LayerPanel.test.tsx src/app/appState.test.ts --run` passed, 26 tests. |
| `AL-007-S24-EV07` | RED: Level-aware Data Panel tests failed before the Data Panel used active Level/model output. |
| `AL-007-S24-EV08` | GREEN: Level-aware model/component checks passed in focused Data Panel/model runs. |
| `AL-007-S24-EV09` | RED: selector/provenance tests failed against mixed `RESOURCE CYCLE (ENERGY & MATTER)` and incomplete provenance. |
| `AL-007-S24-EV10` | GREEN: selector/provenance tests passed after model-driven rendering. |
| `AL-007-S24-EV11` | GREEN: focused regression set passed: `npm.cmd --prefix ui/control-center test -- src/components/MonitorWorkspace.test.tsx src/components/WorldViewer.test.tsx src/components/LayerPanel.test.tsx src/components/BottomDataPanel.test.tsx --run`, 36 tests. |
| `AL-007-S24-EV12` | GREEN: full Vitest, build, and selected Playwright acceptance passed; command details below. |

## Final Verification

| Command | Result |
| --- | --- |
| `npm.cmd test -- --run` from `ui/control-center` | pass: 53 test files, 221 tests. Required escalation because sandboxed esbuild could not read `vite.config.ts`. |
| `npm.cmd run build` from `ui/control-center` | pass: TypeScript build and Vite production build. Warning only: main chunk is larger than 500 kB after minification. |
| `npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` from `ui/control-center` | pass: 16/16 browser tests. |

## Notes

- The failed `npm --prefix ui/control-center exec playwright ...` form was a command-shape issue: Playwright ran from the repository root and discovered unrelated `.agents/.../*.spec.ts` files. The verified form is `npm.cmd exec -- playwright ...` with `cwd=ui/control-center`.
- Energy Flow, Material Cycle, lineage, genome, and richer analytics surfaces remain intentionally unavailable until Runner/Core/Observer contracts exist.
- Next implementation slice should be `AL-007-S25` for Runner/Core Monitor contracts.
