---
status: done
created: 2026-07-29
---

# Fit World Camera Fix Report

## Purpose

Fix the Monitor Map `Fit` behavior reported after `AL-007-S23`: `Fit` drew a smaller centered rectangle instead of restoring the full world to the visible Map viewport.

## Root cause

The viewer used two coordinate systems at the same time:

- `projectCellForRender()` and `worldRenderer.drawBounds()` already project world coordinates into the measured canvas viewport.
- `fitCameraToWorld()` also computed a world-to-viewport scale and centered margin as if the renderer still drew in raw world units.

That double scaling produced a smaller world rectangle centered inside the Map.

## Changed files

- `ui/control-center/src/viewer/viewerNavigation.ts`
  - changed `fitCameraToWorld()` to restore the baseline projected camera `{ x: 0, y: 0, scale: 1 }`.
  - removed the obsolete fit margin constant.
- `ui/control-center/src/viewer/viewerNavigation.test.ts`
  - added/updated behavior coverage for fitting the already projected world to the full Map viewport.
  - updated baseline scale label expectation.
- `ui/control-center/src/components/WorldViewer.test.tsx`
  - updated viewer camera, zoom, pan, and hit-target expectations to the corrected baseline camera.

## TDD evidence

| Evidence | Result |
| --- | --- |
| RED: `npm.cmd test -- src/viewer/viewerNavigation.test.ts --run` | Failed as expected: received `{ scale: 0.46, x: 24, y: 116 }`, expected `{ scale: 1, x: 0, y: 0 }`. |
| GREEN focused: same command after fix | Passed: 6/6. |
| Viewer regression: `npm.cmd test -- src/components/WorldViewer.test.tsx src/viewer/worldRenderPlan.test.ts src/viewer/renderGeometry.test.ts src/viewer/viewerHitTargets.test.ts --run` | Passed: 31/31. |
| Full Vitest: `npm.cmd test -- --run` | Passed: 51 files, 204 tests. |
| Build: `npm.cmd run build` | Passed; existing Vite chunk-size warning remains. |
| E2E: `npm.cmd run e2e -- ui-1c-a-visual.spec.ts monitor.spec.ts` | Passed: 16/16. |

## Notes

- No Core, Runner, projection, or data contract changes.
- This fix intentionally treats `Fit` as camera reset to the renderer's projected full-world baseline.
