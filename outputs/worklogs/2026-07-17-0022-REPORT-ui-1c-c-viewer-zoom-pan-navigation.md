---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-C Viewer Zoom Pan Navigation Report

## Summary

Implemented the baseline Monitor World Viewer navigation slice: zoom, pan, fit, reset, aligned Pixi rendering and DOM hit targets.

## Changed Files

- `ui/control-center/src/viewer/viewerNavigation.ts`
- `ui/control-center/src/viewer/viewerNavigation.test.ts`
- `ui/control-center/src/viewer/renderGeometry.ts`
- `ui/control-center/src/viewer/renderGeometry.test.ts`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/viewer/worldRenderer.test.ts`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/monitor.spec.ts`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Verification

- `npm.cmd test` - PASS, 19 test files, 87 tests.
- `npm.cmd run build` - PASS, TypeScript build and Vite production build completed.
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` - PASS, 5 Playwright tests.
- `git diff --check` - PASS, no whitespace errors; Windows LF/CRLF warnings only.

## Implementation Notes

- `ViewerCamera` remains UI-only navigation state and does not mutate simulation projection data.
- `Fit` is allowed to scale below the interactive wheel zoom minimum when the world is larger than the viewport; this keeps the full world bounds visible.
- Browser acceptance found two pointer-capture issues that unit tests did not expose: navigation buttons and Cell hotspots were starting parent drag. Both now stop pointer/mouse propagation before their own click actions.
- Legacy tests and e2e locators were tightened to exact `World Viewer` labels because nested controls now also include "World Viewer" in accessible names.

## Manual Check Now

- Open `http://127.0.0.1:5173/`.
- Use mouse wheel over World View and confirm the map zooms around the cursor.
- Drag the World View and confirm Cells, selected ring and resource layer move together.
- Click `+`, `-`, `Fit` and `Reset`, and confirm the zoom percentage updates.
- Select a Cell after zoom/pan and confirm the Inspector updates.

## Unresolved Issues

- Navigation state is not persisted between reloads.
- Semantic zoom and richer Cell detail remain deferred to a later UI-1C slice.
- Multi-touch and kinetic panning are intentionally out of scope.

## Next Recommended Slice

`UI-1C-D`: Atmospheric Renderer, Selection Feedback And Semantic Detail.
