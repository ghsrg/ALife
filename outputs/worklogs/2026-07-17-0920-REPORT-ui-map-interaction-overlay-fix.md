---
tags:
  - alife
  - worklog/report
  - ui
  - bugfix
---

# UI Map Interaction and Overlay Fix Report

## Summary

Fixed a Control Center interaction bug where World Viewer navigation could also trigger browser/page behavior, and projection notice overlays could not be dismissed.

## Done

- Isolated World Viewer wheel zoom with a non-passive native wheel listener.
- Prevented browser text selection and page gesture leakage during map drag/pan.
- Added dismissible projection notice overlay with an explicit close button.
- Added empty World Viewer surface click dismissal for projection notices.
- Kept navigation controls and Cell selection from accidentally dismissing notices.
- Added a separate Cell interaction radius so small visually emphasized Cells remain easy to select after map movement.
- Aligned DOM hit targets with the measured Pixi canvas viewport instead of the world coordinate size.
- Raised maximum Viewer zoom to `1200%` so small physically adjacent Cells can be inspected without presentation-minimum overlap dominating.
- Removed the visible DOM selected ring so selected Cell feedback is drawn once by the Pixi renderer.
- Added empty-map selection clearing so the selected focus card and Inspector state can be dismissed.
- Reduced the 1366px World Viewer minimum height from 430px to 390px to keep bottom stats from overlapping the viewer.

## User-Visible Checks

- Scroll over the map zooms the map without scrolling the page.
- Dragging the map pans it without selecting text.
- Projection warning cards can be closed with the `x` button.
- Clicking empty map space closes projection warning cards.
- Cell clicks still select Cells instead of dismissing overlay by accident.
- After dragging the map, Cells still expose pointer hit targets instead of behaving like empty map surface.
- Cell hit targets align with the rendered Cells after zoom/pan.
- The visible zoom control can reach `1200%`.
- Empty map clicks clear the selected Cell and hide the lower selected focus card.
- Reselecting a Cell after clearing selection restores the focus card and Inspector state.

## Verification

- `npm.cmd test` passed: 20 files, 103 tests.
- `npm.cmd run build` passed.
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` passed: 9 tests.
- `git diff --check` passed.
