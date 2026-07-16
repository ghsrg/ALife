---
tags:
  - alife
  - worklog/report
  - ui
  - ui-1c
---

# UI-1C-A World-First Monitor Layout Report

## Summary

Implemented world-first Monitor layout, compact bottom stats, selected entity focus card and visual acceptance harness.

## Changed Files

- `ui/control-center/src/components/monitorStats.ts`
- `ui/control-center/src/components/monitorStats.test.ts`
- `ui/control-center/src/components/BottomStatsStrip.tsx`
- `ui/control-center/src/components/BottomStatsStrip.test.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.test.tsx`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/styles.css`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`
- `outputs/worklogs/2026-07-16-2051-PLAN-ui-1c-a-world-first-monitor-layout.md`
- `outputs/worklogs/index.md`

## Verification

- `npm.cmd test` - PASS, 68 tests
- `npm.cmd run build` - PASS
- `npm.cmd run e2e -- tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts` - PASS, 4 tests
- `git diff --check` - PASS, only LF/CRLF warning for `outputs/worklogs/index.md`

## Screenshots

- `ui/control-center/test-results/ui-1c-a/1920x1080-dark.png`
- `ui/control-center/test-results/ui-1c-a/1366x768-dark.png`
- `ui/control-center/test-results/ui-1c-a/1920x1080-light.png`

## TDD Notes

- `monitorStats`, `BottomStatsStrip`, `SelectedEntityFocusCard` and AppShell composition followed RED/GREEN TDD.
- The Playwright visual acceptance test was added after the CSS layout was implemented, so it did not produce an initial RED in this session. It still protects the UI-1C-A acceptance gate going forward.

## Unresolved Issues

- Live resource grid remains unavailable until Runner/Observer projection includes it.
- Semantic zoom and renderer detail remain in the next UI-1C slice.
- Existing fixture cells do not carry lifecycle, so the focus card and bottom stats correctly show lifecycle as `Unavailable`.

## Next Recommended Slice

`UI-1C-B`: projection truthfulness and renderer scale cleanup for live cells/resources.
