---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-B Projection Truthfulness And Render Scale Report

## Summary

Implemented viewer projection truth state, shared Cell render geometry and visible missing-resource / display-minimum indicators.

This slice does not change Runner, ALIF schema, Core physics, bootstrap, or Observer contracts.

## Changed Files

- `ui/control-center/src/viewer/renderGeometry.ts`
- `ui/control-center/src/viewer/renderGeometry.test.ts`
- `ui/control-center/src/components/viewerTruth.ts`
- `ui/control-center/src/components/viewerTruth.test.ts`
- `ui/control-center/src/components/ViewerTruthOverlay.tsx`
- `ui/control-center/src/components/ViewerTruthOverlay.test.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Verification

- `npm.cmd test` - PASS, 17 test files, 76 tests.
- `npm.cmd run build` - PASS, TypeScript build and Vite production build.
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` - PASS, 4 Playwright tests.
- `git diff --check` - PASS, no whitespace errors; Windows LF/CRLF warnings only for existing modified docs.
- Visual QA: inspected `ui/control-center/test-results/ui-1c-a/1366x768-dark.png`.

## Manual Check Now

- Open `http://127.0.0.1:5173/`.
- In fixture state, confirm the Viewer truth overlay says `Resources / Fixture grid`.
- Run a live scenario; when `ALIF/v2` frames arrive, confirm the overlay says `Resources / Missing projection`.
- If small live Cells are visible, confirm the overlay says `Cell size / Display minimum applied`.
- Confirm the Resource layer row says `Missing live projection` when live `resources` is empty.

## Notes

- `WorldViewer` hit targets and Pixi renderer now use the same `displayRadiusPx` model.
- Tiny Cells keep their physical radius in the projection model, but the UI applies a documented display minimum for visibility.
- The Resource layer no longer implies a live heatmap exists when `ALIF/v2` has no resource grid.
- During visual QA, the existing selected-focus card was found overlapping bottom stats at `1366x768`; the CSS and e2e acceptance guard now prevent that overlap.

## Unresolved Issues

- Live Resource grid still requires Runner/Observer projection work.
- Bootstrap/physics spacing is not changed by this UI slice.
- Semantic zoom remains in a later UI-1C slice.

## Next Recommended Slice

`UI-1C-C`: Google Maps-like Viewer zoom, pan and selection navigation on top of truthful scale/projection state.
