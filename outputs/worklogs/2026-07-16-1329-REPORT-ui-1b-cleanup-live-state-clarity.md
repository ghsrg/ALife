---
tags:
  - alife
  - report
  - ui
  - control-center
  - ui-1b-cleanup
---

# REPORT: UI-1B Cleanup Live State Clarity

## Summary

Implemented the `UI-1B-Cleanup` bridge slice from
`outputs/worklogs/2026-07-16-1233-PLAN-ui-1b-cleanup-live-state-clarity.md`.

The cleanup does not reopen `UI-1B` scope. It clarifies existing UI state around
Runner connection, displayed data source, reconnect behavior, unavailable live
resources, and the single-tick step command.

## Changed Files

- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/app/appState.test.ts`
- `ui/control-center/src/components/ConnectionPanel.tsx`
- `ui/control-center/src/components/ConnectionPanel.test.tsx`
- `ui/control-center/src/components/RunControls.tsx`
- `ui/control-center/src/components/RunControls.test.tsx`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/App.test.tsx`
- `ui/control-center/src/styles.css`
- `docs/implementation/implementation-plan-ui.md`
- `outputs/worklogs/index.md`

## Behavior Changes

- Added derived `MonitorDataState` classification:
  - `fixture-offline`
  - `fixture-idle`
  - `live-waiting`
  - `live`
  - `stale-live`
- Connection panel now separates:
  - Runner connection state;
  - displayed data state;
  - live resource projection availability.
- Connection panel now exposes `Reconnect` as an explicit UI action.
- Reconnect retries Runner bootstrap and stream connection without sending run
  commands.
- Monitor toolbar now explains fixture fallback and waiting states:
  - `Fixture Tick 128 - Runner idle`
  - `Waiting for live frame - Fixture Tick 128`
  - `Live Tick N`
  - `Stale Live Tick N - disconnected`
- Run controls now label the step command as `Step 1` with accessible name
  `Step one committed tick`.
- Canonical UI implementation plan now records `UI-1B-Cleanup` as a bridge
  before `UI-1C`.

## Tests

Added or updated coverage for:

- monitor data state derivation;
- connection/data/resource status labels;
- reconnect button behavior;
- reconnect after failed bootstrap without repeating run commands;
- single-tick step label;
- connected idle fixture fallback explanation;
- active run waiting-for-live-frame explanation.

## Verification

Commands run from `ui/control-center`:

```powershell
npm.cmd test -- src/app/appState.test.ts src/components/ConnectionPanel.test.tsx src/components/RunControls.test.tsx src/App.test.tsx
npm.cmd test
npm.cmd run build
```

Results:

- targeted tests: 4 files passed, 31 tests passed;
- full UI tests: 11 files passed, 60 tests passed;
- production build passed.

Command run from repo root:

```powershell
git diff --check
```

Result:

- passed with no whitespace errors;
- Git reported LF-to-CRLF warnings for touched docs files.

## Deviations

- The reconnect test expects `listScenarios` and `getRunStatus` to be called on
  both bootstrap attempts. The current bootstrap uses `Promise.all`, so sibling
  requests still start when `getServerInfo` rejects. This matches current
  implementation semantics and does not repeat run commands.
- No live resource grid projection was added. The UI now states that resources
  are not streamed in `ALIF v2`.

## Remaining Work Before UI-1C

- Define or defer the Runner projection contract for live resources.
- Reconcile live visual radius/render scale with projection truthfulness.
- Implement semantic zoom and richer Cell Inspector only in `UI-1C` or later.
- Add final Start demo hardening after `UI-1C` scope is complete.

