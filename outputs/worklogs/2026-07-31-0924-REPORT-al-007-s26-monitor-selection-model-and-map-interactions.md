# AL-007-S26 Monitor Selection Model And Map Interactions Execution Report

## Status

Implementation progressed, but closure is blocked by local approval/usage limit for escalated test commands. Do not mark `AL-007-S26` done until the verification commands below are rerun successfully.

## Implemented Scope

- Added typed Monitor selection model:
  - `none`
  - `cell`
  - `world-block`
  - compatible `selection-set`
- Added World block derivation from resource grid geometry with full-world fallback when no resource grid exists.
- Added selection compatibility helper for Analysis Level changes.
- Added store-level `currentSelection`, `selectionNotice`, `selectMonitorTarget`, and `clearSelection`.
- Kept `selectedCellId` / `selectedCell` only as a compatibility bridge for current Cell Inspector code.
- Changed disappearing target behavior:
  - selected target present: remains selected and updates to displayed frame context;
  - selected target missing: clears selection and records a notice;
  - no silent first-available Cell auto-selection.
- Wired `activeLevel`, `currentSelection`, and `onSelectTarget` from `AppShell` through `MonitorWorkspace` to `WorldViewer`.
- World Level Map interactions:
  - Cell hit click selects containing World block, not Cell;
  - empty Map click selects a World block.
- Cells Level Map interactions:
  - single Cell click selects Cell;
  - `Shift + click` toggles compatible Cell selection-set;
  - `Shift + mouse drag` creates a Cell selection-set;
  - normal drag remains pan through the existing camera path.
- Removed implicit Focus card rendering on Cell selection.
- Added a minimal `.selection-rectangle` visual affordance.
- Exposed selection notices in the Inspector empty state.

## Verification Completed Before Blocker

- `npm.cmd test -- src/app/selectionModel.test.ts src/components/WorldViewer.test.tsx --run`
  - Passed: 33 tests after Shift-click implementation.
- `npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts --run`
  - Passed: 32 tests after compatibility and disappearing-target changes.
- `npm.cmd test -- src/app/appState.test.ts --run`
  - Passed: 24 tests after `selectMonitorTarget`.
- `npm.cmd test -- src/App.test.tsx --run`
  - Passed: 20 tests after AppShell Level compatibility and no-auto-selection expectation changes.
- `npm.cmd test -- src/components/WorldViewer.test.tsx --run`
  - Passed: 27 tests after World empty-click and Shift-drag behavior.

## Verification Blocker

After the final patches for `clearSelection`, visible `selectionNotice`, and selection type narrowing, escalated command approval failed with:

```text
You've hit your usage limit. Upgrade to Pro, visit settings/usage to purchase more credits or try again at Aug 5th, 2026 10:01 AM.
```

Because `vite.config.ts` test startup needs filesystem access outside the restricted sandbox in this environment, non-escalated Vitest startup is not currently usable here. The slice must remain `in-progress` until verification is rerun.

## Required Closure Verification

Run from `ui/control-center`:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts src/components/WorldViewer.test.tsx src/App.test.tsx --run
npm.cmd run build
```

Optional if the local browser environment is ready:

```powershell
npx.cmd playwright test tests/e2e/monitor.spec.ts --project=chromium
```

## Files Changed

- `ui/control-center/src/app/selectionModel.ts`
- `ui/control-center/src/app/selectionModel.test.ts`
- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/app/appState.test.ts`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/InspectorPanel.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/styles/components.css`
- `ui/control-center/src/App.test.tsx`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/delivery/worklog-ledger.md`

## Follow-Up

- Rerun closure verification when command approval/usage is available.
- If verification passes, run closure verification and only then move `AL-007-S26` from `in-progress` to `done`.
- If TypeScript build fails, first inspect the narrowed selection model types around `createSelectionSet` call sites.
