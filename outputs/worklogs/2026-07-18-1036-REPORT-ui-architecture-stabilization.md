---
tags:
  - alife
  - worklog/report
  - ui
  - architecture
---

# UI Architecture Stabilization Report

## Summary

Executed `2026-07-17-1305-PLAN-ui-architecture-stabilization.md` on branch `codex/ui-architecture-stabilization`.

The Control Center keeps the current React, Zustand and PixiJS stack, but the main Monitor implementation now has clearer architecture boundaries around view-model derivation, Runner orchestration, workspace composition, viewer camera state, hit-target derivation, render planning, UI text and global styles.

## Done

- Added architecture boundary tests for Runner transport imports, Pixi imports and CSS import-hub structure.
- Extracted Monitor view-model derivation into `app/monitorViewModel.ts`.
- Extracted live Runner orchestration and stale-frame guards into `app/runnerController.ts`.
- Extracted Monitor composition into `components/MonitorWorkspace.tsx`.
- Extracted viewer camera gesture state into `viewer/useViewerCamera.ts`.
- Extracted viewer accessibility hit targets into `viewer/viewerHitTargets.ts`.
- Extracted pure render-plan generation into `viewer/worldRenderPlan.ts`; `worldRenderer.ts` now owns Pixi mounting and drawing only.
- Added a minimal `uiText.ts` registry for critical English UI labels.
- Split global styles into `styles/tokens.css`, `styles/layout.css` and `styles/components.css`, with `styles.css` as the import hub.
- Synced `runnerController.test.ts` with the current `ALIF/v2` projection fixture shape so the production build type gate passes.
- Fixed the selected focus card placement so UI-1C-A visual acceptance keeps the focus card above the stats strip at 1920x1080 and 1366x768.

## Changed Files

- `ui/control-center/src/architecture/architectureBoundaries.test.ts`
- `ui/control-center/src/app/monitorViewModel.ts`
- `ui/control-center/src/app/monitorViewModel.test.ts`
- `ui/control-center/src/app/runnerController.ts`
- `ui/control-center/src/app/runnerController.test.ts`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/uiText.ts`
- `ui/control-center/src/uiText.test.ts`
- `ui/control-center/src/viewer/useViewerCamera.ts`
- `ui/control-center/src/viewer/useViewerCamera.test.ts`
- `ui/control-center/src/viewer/viewerHitTargets.ts`
- `ui/control-center/src/viewer/viewerHitTargets.test.ts`
- `ui/control-center/src/viewer/worldRenderPlan.ts`
- `ui/control-center/src/viewer/worldRenderPlan.test.ts`
- `ui/control-center/src/viewer/worldRenderer.ts`
- `ui/control-center/src/viewer/worldRenderer.test.ts`
- `ui/control-center/src/styles.css`
- `ui/control-center/src/styles/tokens.css`
- `ui/control-center/src/styles/layout.css`
- `ui/control-center/src/styles/components.css`

## Verification

- `npm.cmd test`: PASS, 28 files, 124 tests.
- `npm.cmd run build`: PASS, TypeScript build and Vite production build completed.
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts`: PASS, 9 Playwright tests.
- `git diff --check`: PASS, no whitespace errors.

## User-Visible Checks

- The Monitor still opens with fixture data and selected Cell Inspector.
- The selected focus card stays above the bottom stats strip at 1920x1080 dark/light and 1366x768.
- Viewer zoom, reset, drag, reselection and empty-space unselect behavior remain covered by tests.
- The UI still exposes the same critical English labels, now sourced from `uiText`.

## Notes

- This slice intentionally did not change simulation semantics, projection payloads or command semantics.
- CSS split was kept mechanical; no new design system or component library was introduced.
- The next planned product slice remains `UI-1D: Start Demo, Export And Acceptance Hardening`.
