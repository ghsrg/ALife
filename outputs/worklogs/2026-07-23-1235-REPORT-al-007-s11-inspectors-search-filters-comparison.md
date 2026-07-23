---
plan_id: AL-007-S11
type: report
status: done
---

# AL-007-S11 Inspectors, Search, Filters, And Entity Comparison Report

## Summary

Implemented the UI-2C source-backed inspector/search/compare slice without changing Core behavior, Runner ALIF v2 live-frame payloads, or Observer projection contracts.

## Source Documents Read

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/ui/visualization.md`
- `docs/implementation/implementation-plan-ui.md`
- `outputs/worklogs/2026-07-22-1444-REPORT-al-007-s10-debug-visualization-mode-exact-layers.md`
- `outputs/worklogs/2026-07-22-2035-REPORT-al-007-s21-rich-world-visibility-repair.md`

Worklogs were used as evidence only, not as source of truth.

## Completed

- Added explicit debug projection `loading` and `stale` states so live frames do not flicker through false "missing resource projection" states while Observer debug projections are still loading or behind the live tick.
- Prevented stale debug projections from enriching newer live frames.
- Bounded Debug Visualization Mode resource legend to the first 8 layers with total/hidden counts and scroll-limited overlay layout.
- Added compact grouped Cell Inspector sections for materials, internal resources, and local external resources.
- Added map search/filter UI over source-backed Cell ids, role/lifecycle text, material/resource text, resource layer/channel text, and field text.
- Added explicit unsupported search note for process/contact/history because those payloads are not source-backed yet.
- Added local pinned Cell comparison for selected-vs-pinned id, energy, and material overlap.

## Evidence

- `ui/control-center/src/projection/types.ts`
- `ui/control-center/src/app/runnerController.ts`
- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/viewer/debugLayers.ts`
- `ui/control-center/src/components/viewerTruth.ts`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/styles/components.css`
- `ui/control-center/src/app/runnerController.test.ts`
- `ui/control-center/src/viewer/debugLayers.test.ts`
- `ui/control-center/src/components/viewerTruth.test.ts`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/CellInspector.test.tsx`

## Verification

- Passed: `npm.cmd test -- --run src/app/runnerController.test.ts src/viewer/debugLayers.test.ts src/components/viewerTruth.test.ts src/components/CellInspector.test.tsx src/components/WorldViewer.test.tsx src/app/monitorViewModel.test.ts`
- Passed: `npm.cmd run build`
- Not completed: full `npm.cmd test -- --run` exceeded the local 5 minute timeout twice without useful failure output. Focused AL-007-S11 coverage and production build passed.

## Coverage Matrix

| Plan ID | Requirement | Acceptance ID | Evidence | Status |
| --- | --- | --- | --- | --- |
| `AL-007-S11` | Source-backed Cell Inspector groups material/internal/local resource details without unbounded panel growth. | `AL-007-S11-AC01` | `CellInspector.tsx`, `CellInspector.test.tsx`, focused Vitest | covered |
| `AL-007-S11` | Resource projection loading/stale states are explicit and avoid false missing-resource flicker. | `AL-007-S11-AC02` | `types.ts`, `runnerController.ts`, `viewerTruth.ts`, `runnerController.test.ts`, `viewerTruth.test.ts`, focused Vitest | covered |
| `AL-007-S11` | Resource legend/tooltip surfaces are bounded and cannot expand the map. | `AL-007-S11-AC03` | `debugLayers.ts`, `WorldViewer.tsx`, `components.css`, `debugLayers.test.ts`, `WorldViewer.test.tsx`, focused Vitest | covered |
| `AL-007-S11` | Search/filter finds and highlights source-backed Cell/resource/layer/field data; unsupported process/contact/history remains explicit. | `AL-007-S11-AC04` | `WorldViewer.tsx`, `WorldViewer.test.tsx`, focused Vitest | covered |
| `AL-007-S11` | Selected Cell can be pinned and compared against the current Cell using source-backed fields. | `AL-007-S11-AC05` | `CellInspector.tsx`, `CellInspector.test.tsx`, focused Vitest | covered |

## Deferred

- Dedicated Resource, Material, Field, and Process inspector workspaces remain downstream.
- Multi-select, rectangle selection, persisted selection sets, recent entities, and follow mode remain downstream.
- Process/contact/history search remains unsupported until source-backed payloads exist.
- Research workflow/reporting views remain downstream.

## Status Recommendation

Mark `AL-007-S11` as `done` with `high` confidence. Keep the full UI test-suite timeout as verification debt, not functional scope debt, because focused AL-007-S11 tests and production build passed.
