# AL-007-S26 Monitor Selection Model And Map Interactions TDD Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Do not execute without explicit `OK EXECUTE AL-007-S26`.

**Goal:** Replace the current Cell-only Monitor selection with a typed Level-aware selection model and correct Map interactions for World block selection, Cell selection, multi-selection, and disappearing targets.

**Architecture:** Add a small `selectionModel` domain module that owns selection types, compatibility, world-block hit math, and target refresh rules. Keep `WorldViewer` as an interaction surface that emits semantic selection intents rather than mutating Cell-only state. Keep Focus, Pin, Inspector comparison, and Data Panel scope mostly out of this slice except for clearing/compatibility hooks needed to avoid incorrect current behavior.

**Tech Stack:** React, TypeScript, Zustand vanilla store, Vitest, Testing Library, Playwright smoke where needed.

---

## Plan Metadata

| Field | Value |
| --- | --- |
| Plan ID | `AL-007-S26` |
| Slice title | Monitor Selection Model And Map Interactions |
| Status | TDD plan proposal |
| Created | 2026-07-30 |
| Depends on | `AL-007-S24-Fix`, partial `AL-007-S25` monitor payload work |
| Confidence | medium-high |

## Delivery Control Result

**Route:** `delivery-control` -> deterministic `delivery-lint` -> `roadmap-control` as `TDD_PLAN_REQUEST`.

## LINT_RESULT

**Scope:** `AL-007-S26`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, UI Canon, current code  
**Mode:** deterministic

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| WARN | `DL001` | `AL-007-S26` | Plan ID is proposed in `outputs/worklogs/2026-07-30-2017-PLAN-monitor-selection-focus-and-data-scope-gaps.md`, but is not registered in `docs/delivery/roadmap.md`. | During execution, add roadmap/status/acceptance rows before code changes or treat this plan as proposed until approved. |
| WARN | `DL003` | `AL-007-S26-AC01..AC06` | Acceptance rows do not exist yet. | Add acceptance rows when plan is approved. |
| WARN | `DL008` | `AL-007-S25` / `AL-007-S26` | `AL-007-S25` is in-progress for Runner/Core contracts, while `AL-007-S26` is UI selection behavior. | Keep `AL-007-S26` UI-only except consuming existing projection data; do not add Core/Runner contracts here. |

**Decision:** `PASS_WITH_WARNINGS`

## Source-Of-Truth Hierarchy Used

1. `docs/PRINCIPLES.md`
2. `docs/INDEX.md`
3. `docs/ui/INDEX.md`
4. `docs/ui/control-center-design-spec.md`
5. `docs/ui/control-center-block.md`
6. `docs/delivery/roadmap.md`
7. `docs/delivery/status.md`
8. `outputs/worklogs/2026-07-30-2017-PLAN-monitor-selection-focus-and-data-scope-gaps.md`
9. Current implementation files listed below.

## Files Read

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/ui/INDEX.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/control-center-block.md`
- `docs/delivery/source-map.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `outputs/worklogs/2026-07-30-2017-PLAN-monitor-selection-focus-and-data-scope-gaps.md`
- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/app/appState.test.ts`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/components/LevelPanel.tsx`
- `ui/control-center/src/viewer/viewerHitTargets.ts`
- `ui/control-center/src/viewer/useViewerCamera.ts`
- `ui/control-center/src/projection/types.ts`

## Current Implementation Findings

- App state is Cell-only: `selectedCellId`, `selectedCell`, `selectCell`.
- Initial load auto-selects the first Cell.
- If selected Cell disappears on live frame update, app auto-selects the first available Cell.
- `WorldViewer` builds hit targets only for Cells.
- World Level is external UI state in `AppShell`, not part of selection compatibility logic.
- `WorldViewer` click on Cell always calls `onSelectCell(target.id)`.
- `Shift + click` and `Shift + drag-select` are not implemented.
- Pointer drag currently always means camera pan; there is no selection rectangle mode.
- Empty Map click clears Cell selection.
- `SelectedEntityFocusCard` renders whenever `selectedCell` exists; this is a Focus issue, but `AL-007-S26` should not implement final Focus. It may hide/remove auto-Focus only if needed to stop selection regression.

## Assumptions

- `AL-007-S26` is UI-only. It must not change Core, Runner, Observer contracts, simulation mechanics, or projection schema.
- First World block implementation uses `frame.resources` grid dimensions when present. If no resource grid exists, UI exposes one fallback full-world block with `completeness: unavailable`.
- Organisms, Lineages, Evolution, and Analytics selection can be represented in the typed model now, but only Cells and World block get active Map hit behavior in this slice unless source data already exists.
- Multi-selection supports compatible Map targets. For this slice, compatible means same `selection.kind` within the active Level.
- Focus final behavior is deferred to `AL-007-S27`; this slice only prevents selection logic from forcing Focus.

## Forbidden Scope

- Do not implement Focus overlay content beyond removing accidental auto-open behavior if necessary.
- Do not implement Pin baseline comparison.
- Do not implement Data Panel scoped chart aggregation.
- Do not add Runner/Core/Observer payloads.
- Do not infer organism, lineage, genome, or analytics source data that is unavailable.
- Do not change Map layout tracks, Layer presentation semantics, or Data Panel layout.
- Do not mark `AL-007-S26` done without closure verification.

## Agent Scenario Cards

### AL-007-S26-AC01: Typed Monitor selection replaces Cell-only state

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`, `outputs/worklogs/2026-07-30-2017-PLAN-monitor-selection-focus-and-data-scope-gaps.md`  
**Intent:** Selection can represent all final Monitor Levels without forcing Cell-only behavior.  
**Priority:** P0  
**Independent verification:** `selectionModel` and `appState` Vitest.

**Given** Monitor has an active Analysis Level  
**When** the user selects or clears a target  
**Then** app state stores a discriminated `currentSelection` with exact kind, target id/block/set, displayed tick, run id, and compatibility metadata, while legacy `selectedCell` remains a derived bridge only for existing Cell Inspector code.

**TDD obligation:** RED test must fail because `currentSelection` does not exist and app state auto-selects a Cell.

**Evidence:** `AL-007-S26-EV01`, `AL-007-S26-EV02`

### AL-007-S26-AC02: World Level selects World block, not Cell

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`  
**Intent:** World Level has its own spatial selection semantics.  
**Priority:** P0  
**Independent verification:** `selectionModel` unit tests and `WorldViewer` interaction tests.

**Given** active Level is `world` and the Map is clicked  
**When** the click lands in a World block  
**Then** selection becomes `world-block` with block coordinates and bounds; it does not select a Cell even if a Cell exists inside the block.

**TDD obligation:** RED test must fail because Cell hit targets currently select Cells at World Level.

**Evidence:** `AL-007-S26-EV03`, `AL-007-S26-EV04`

### AL-007-S26-AC03: Cells Level preserves single Cell selection without auto-Focus

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`  
**Intent:** Existing Cell selection remains usable, but selection alone does not open Focus.  
**Priority:** P0  
**Independent verification:** `WorldViewer`, `AppShell`, and `App` tests.

**Given** active Level is `cells`  
**When** the user single-clicks a Cell hit target  
**Then** selection becomes one `cell`; Inspector receives that Cell; Focus does not open from this action.

**TDD obligation:** RED test must fail because current Focus card renders immediately on selected Cell.

**Evidence:** `AL-007-S26-EV05`, `AL-007-S26-EV06`

### AL-007-S26-AC04: Shift interactions create compatible multi-selection without breaking pan/zoom

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`  
**Intent:** Multi-selection exists and does not conflict with normal Map navigation.  
**Priority:** P0  
**Independent verification:** `selectionModel`, `WorldViewer`, and app-state tests.

**Given** active Level has compatible Map targets  
**When** the user uses `Shift + click` or `Shift + drag-select`  
**Then** compatible targets are added/removed from a `selection-set`; normal click-drag without Shift still pans Map; wheel still zooms Map.

**TDD obligation:** RED test must fail because Shift interactions currently do not exist.

**Evidence:** `AL-007-S26-EV07`, `AL-007-S26-EV08`

### AL-007-S26-AC05: Level changes clear incompatible selection with visible reason

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`  
**Intent:** Selection semantics stay truthful when Analysis Level changes.  
**Priority:** P1  
**Independent verification:** `selectionModel`, `appState`, and `AppShell` tests.

**Given** current selection is incompatible with a new Level  
**When** the user changes Level  
**Then** selection clears and a transient selection reason is exposed; compatible selection may remain only when the Canon allows it.

**TDD obligation:** RED test must fail because Level changes currently do not interact with selection.

**Evidence:** `AL-007-S26-EV09`, `AL-007-S26-EV10`

### AL-007-S26-AC06: Live target disappearance clears selection instead of auto-selecting another target

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`  
**Intent:** UI does not silently switch user context when a live target disappears.  
**Priority:** P0  
**Independent verification:** `appState` tests.

**Given** an unpinned live selection follows a target  
**When** a new displayed frame no longer contains that target  
**Then** selection clears, a transient reason is recorded, and no first available Cell is auto-selected. `Dead` lifecycle remains a valid selected state while the target still exists.

**TDD obligation:** RED test must fail because current app state auto-selects first available Cell.

**Evidence:** `AL-007-S26-EV11`, `AL-007-S26-EV12`

## File Map

### Create

- `ui/control-center/src/app/selectionModel.ts`  
  Selection discriminated unions, compatibility helpers, world block derivation, point/rectangle hit helpers, refresh rules.

- `ui/control-center/src/app/selectionModel.test.ts`  
  Unit tests for typed selections, world block derivation, compatibility, multi-selection add/remove, and disappearance handling.

### Modify

- `ui/control-center/src/app/appState.ts`  
  Add `currentSelection`, `selectionNotice`, selection actions, derived Cell bridge, and live refresh behavior.

- `ui/control-center/src/app/appState.test.ts`  
  Replace/extend Cell-only tests with selection model coverage.

- `ui/control-center/src/components/AppShell.tsx`  
  Pass `activeLevel` and typed selection actions into Monitor.

- `ui/control-center/src/components/MonitorWorkspace.tsx`  
  Replace `onSelectCell` with typed selection intents; pass Level/selection into `WorldViewer`.

- `ui/control-center/src/components/WorldViewer.tsx`  
  Add Level-aware click semantics, world block click handling, Shift-click, Shift-drag selection rectangle, and no-Focus single click behavior.

- `ui/control-center/src/components/WorldViewer.test.tsx`  
  Add interaction tests for World block, Cells single selection, Shift-click, Shift-drag, empty click, pan, and zoom.

- `ui/control-center/src/components/SelectedEntityFocusCard.tsx`  
  Stop rendering as implicit Focus. Either remove from `MonitorWorkspace` or gate behind future explicit Focus state with default closed.

- `ui/control-center/src/components/InspectorPanel.tsx` and/or `CellInspector.tsx`  
  Keep existing Cell detail through derived `selectedCell`; show selection notice if needed. Full World total Inspector is deferred to `AL-007-S28`.

- `ui/control-center/src/projection/types.ts`  
  Add reusable `WorldBlockSelectionBounds` type only if needed by multiple modules.

- `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`  
  Register `AL-007-S26` when execution is approved.

## Numbered TDD Tasks

### AL-007-S26-T01: RED for `AL-007-S26-AC01`

- [ ] Add `ui/control-center/src/app/selectionModel.test.ts`.
- [ ] Test `none`, `cell`, `world-block`, and `selection-set` selection constructors.
- [ ] Test that a Cell selection stores `{ kind: 'cell', cellId, runId, tick }`.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts --run
```

- [ ] Expected RED: module or constructors do not exist.
- [ ] Capture as `AL-007-S26-EV01`.

### AL-007-S26-T02: GREEN for `AL-007-S26-AC01`

- [ ] Create `selectionModel.ts` with minimal types and constructors.
- [ ] Add app state fields:
  - `currentSelection`
  - `selectionNotice`
  - `selectMonitorTarget`
  - `clearSelection`
- [ ] Keep `selectedCellId` and `selectedCell` as derived compatibility bridge for current Inspector code.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts --run
```

- [ ] Expected GREEN.
- [ ] Capture as `AL-007-S26-EV02`.

### AL-007-S26-T03: RED for `AL-007-S26-AC02`

- [ ] Add `deriveWorldBlockAtPoint` tests:
  - `frame.resources` grid `2 x 2`, world `200 x 100`, click `(150, 75)` returns block `(1, 1)` bounds `{ x: 100, y: 50, width: 100, height: 50 }`.
  - empty resources returns one fallback block covering the full World and `completeness: 'unavailable'`.
- [ ] Add `WorldViewer` test: with `activeLevel="world"`, click a map/block location calls `onSelectTarget({ kind: 'world-block', ... })`, not `onSelectCell`.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/components/WorldViewer.test.tsx --run
```

- [ ] Expected RED: world block selection is absent.
- [ ] Capture as `AL-007-S26-EV03`.

### AL-007-S26-T04: GREEN for `AL-007-S26-AC02`

- [ ] Implement world block derivation from `frame.resources`.
- [ ] Add `activeLevel` prop to `WorldViewer`.
- [ ] Add typed `onSelectTarget` prop.
- [ ] At World Level:
  - Empty/body click selects `world-block` at click position.
  - Cell hit target click also resolves to the containing World block, not Cell.
- [ ] Keep existing Cell selection behavior for Cells Level.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/components/WorldViewer.test.tsx src/App.test.tsx --run
```

- [ ] Expected GREEN.
- [ ] Capture as `AL-007-S26-EV04`.

### AL-007-S26-T05: RED for `AL-007-S26-AC03`

- [ ] Add test that at `activeLevel="cells"`, single-clicking a Cell selects `{ kind: 'cell' }`.
- [ ] Add `AppShell`/`MonitorWorkspace` test that a selected Cell does not render `Selected entity focus` by default.
- [ ] Run:

```powershell
npm.cmd test -- src/components/WorldViewer.test.tsx src/App.test.tsx --run
```

- [ ] Expected RED: current auto Focus card renders on selected Cell.
- [ ] Capture as `AL-007-S26-EV05`.

### AL-007-S26-T06: GREEN for `AL-007-S26-AC03`

- [ ] Remove implicit `SelectedEntityFocusCard` rendering from `MonitorWorkspace`, or gate it behind an explicit future `focusState` that defaults closed.
- [ ] Preserve Inspector Cell detail through `state.selectedCell`.
- [ ] Run:

```powershell
npm.cmd test -- src/components/WorldViewer.test.tsx src/App.test.tsx src/components/CellInspector.test.tsx --run
```

- [ ] Expected GREEN.
- [ ] Capture as `AL-007-S26-EV06`.

### AL-007-S26-T07: RED for `AL-007-S26-AC04`

- [ ] Add `selectionModel` tests for:
  - `toggleSelectionSetMember` adds compatible target.
  - same target toggles off.
  - incompatible target replaces or clears with reason, according to helper contract.
- [ ] Add `WorldViewer` tests:
  - `Shift + click` on two Cells creates/toggles set intents.
  - `Shift + drag-select` emits rectangle selection intent.
  - normal drag still pans and does not select.
  - wheel still zooms.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/components/WorldViewer.test.tsx --run
```

- [ ] Expected RED: Shift multi-selection is absent.
- [ ] Capture as `AL-007-S26-EV07`.

### AL-007-S26-T08: GREEN for `AL-007-S26-AC04`

- [ ] Implement `selection-set` helpers in `selectionModel.ts`.
- [ ] Implement Shift-click event path in `WorldViewer`.
- [ ] Implement Shift-drag rectangle state:
  - pointer down with Shift starts rectangle;
  - pointer move updates visible rectangle;
  - pointer up emits compatible target ids intersecting rectangle;
  - non-Shift drag keeps camera pan behavior.
- [ ] Add a minimal `.selection-rectangle` visual affordance.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/components/WorldViewer.test.tsx --run
```

- [ ] Expected GREEN.
- [ ] Capture as `AL-007-S26-EV08`.

### AL-007-S26-T09: RED for `AL-007-S26-AC05`

- [ ] Add `selectionModel` tests for `isSelectionCompatibleWithLevel`.
- [ ] Add `AppShell` or `appState` test:
  - Select Cell at Cells Level.
  - Change Level to World.
  - Selection clears and `selectionNotice` contains incompatible level reason.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts src/App.test.tsx --run
```

- [ ] Expected RED: Level changes currently do not clear incompatible selection.
- [ ] Capture as `AL-007-S26-EV09`.

### AL-007-S26-T10: GREEN for `AL-007-S26-AC05`

- [ ] Add `setActiveMonitorLevel` or `applyLevelChange` action that owns selection compatibility.
- [ ] Move active Level state into `AppState` or keep AppShell state but call store action before changing UI Level. Prefer moving into `AppState` to avoid split-brain.
- [ ] Show transient `selectionNotice` in Monitor context strip or Inspector placeholder.
- [ ] Run:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts src/App.test.tsx --run
```

- [ ] Expected GREEN.
- [ ] Capture as `AL-007-S26-EV10`.

### AL-007-S26-T11: RED for `AL-007-S26-AC06`

- [ ] Update existing `appState.test.ts` behavior:
  - selected live Cell disappears from next live frame;
  - expect `currentSelection.kind === 'none'`;
  - expect `selectedCell === null`;
  - expect no auto-selected first Cell;
  - expect `selectionNotice` reason.
- [ ] Add test that a selected Cell with `lifecycle`/`lifecycleState` dead still remains selected while present.
- [ ] Run:

```powershell
npm.cmd test -- src/app/appState.test.ts --run
```

- [ ] Expected RED: current code auto-selects first available Cell.
- [ ] Capture as `AL-007-S26-EV11`.

### AL-007-S26-T12: GREEN for `AL-007-S26-AC06`

- [ ] Replace `selectCellForFrame` fallback behavior:
  - explicit empty selection stays empty;
  - selected target present stays selected;
  - selected target missing clears selection with notice;
  - no previous explicit selection may optionally remain `none`; do not auto-select on live frames.
- [ ] Preserve fixture startup compatibility only if tests require it; prefer no automatic selection unless the fixture-specific test is updated to click explicitly.
- [ ] Run:

```powershell
npm.cmd test -- src/app/appState.test.ts src/components/CellInspector.test.tsx src/App.test.tsx --run
```

- [ ] Expected GREEN.
- [ ] Capture as `AL-007-S26-EV12`.

### AL-007-S26-T13: Integration and visual regression smoke

- [ ] Run targeted UI suite:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts src/components/WorldViewer.test.tsx src/components/MonitorWorkspace.test.tsx src/App.test.tsx --run
```

- [ ] Run production build:

```powershell
npm.cmd run build
```

- [ ] If existing Playwright Monitor smoke is stable locally, run:

```powershell
npx.cmd playwright test tests/e2e/monitor.spec.ts --project=chromium
```

- [ ] Capture as `AL-007-S26-EV13`.

### AL-007-S26-T14: Docs/status/report preparation

- [ ] If execution was approved, add `AL-007-S26` row to `docs/delivery/roadmap.md`.
- [ ] Add `AL-007-S26-AC01..AC06` rows to `docs/delivery/acceptance.md`.
- [ ] Update `docs/delivery/status.md` Current Focus only if the user wants `AL-007-S26` to supersede current `AL-007-S25` focus. Otherwise keep `AL-007-S25` current and list `AL-007-S26` as planned/next.
- [ ] Add worklog ledger row for this plan.
- [ ] Create execution report after verification:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s26-monitor-selection-model-and-map-interactions.md
```

- [ ] Do not mark done until closure verification.

## Verification Commands

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts src/components/WorldViewer.test.tsx src/components/MonitorWorkspace.test.tsx src/App.test.tsx --run
npm.cmd run build
npx.cmd playwright test tests/e2e/monitor.spec.ts --project=chromium
```

## Open Questions

No blocker for planning.

Execution assumptions to confirm implicitly by `OK EXECUTE AL-007-S26`:

- Use `frame.resources` grid as first World block grid source.
- Make no automatic initial Cell selection in Monitor unless a test is intentionally kept for fixture compatibility.
- Keep Organisms/Lineages/Evolution/Analytics typed in selection model but inactive/unavailable if no source-backed hit targets exist.
- Treat Focus behavior beyond disabling auto-Focus as `AL-007-S27`.

## Approval Gate

Reply `OK EXECUTE AL-007-S26` to authorize execution of this TDD plan.

Reply `CHANGE AL-007-S26` with corrections to revise the plan.
