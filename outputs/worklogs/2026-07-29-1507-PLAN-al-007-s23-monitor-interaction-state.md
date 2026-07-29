---
plan_id: AL-007-S23
status: proposed
created: 2026-07-29
---

# AL-007-S23 Monitor Interaction State TDD Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Do not use subagents unless the human explicitly authorizes delegated execution.

**Goal:** Make the Monitor shell interaction state match the final reference-driven layout behavior after `AL-007-S22`: runner status belongs in the top context surface, Layers & Filters is only map presentation, Stop looks and behaves like Stop, Data Panel is denser at `1280x720`, and Map-only fullscreen is view-only.

**Architecture:** Keep this slice UI-only unless an existing Runner command is already available. Move presentation/state composition between existing React components without changing Core simulation rules, Runner lifecycle contracts, ALIF payload semantics, or Observer projection truth. Any unsupported final-contract surface must render an explicit unavailable/stub state and be routed to later `AL-007-S24`/`AL-007-S25`.

**Tech Stack:** Vite, React, TypeScript, Vitest/RTL, Playwright, existing CSS tokens/components.

---

## Source-of-truth hierarchy

1. `docs/PRINCIPLES.md`
2. `docs/ui/control-center-design-spec.md`
3. `docs/ui/control-center-block.md`
4. `docs/implementation/implementation-plan-ui.md`
5. `docs/implementation/implementation-plan-runner.md`
6. `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`
7. Current implementation under `ui/control-center/src/`

## Files read

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/source-map.md`
- `docs/delivery/acceptance.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/control-center-block.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/implementation/implementation-plan-runner.md`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/RunBar.tsx`
- `ui/control-center/src/components/RunControls.tsx`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/components/BottomDataPanel.tsx`
- `ui/control-center/src/components/ConnectionPanel.tsx`
- `ui/control-center/tests/e2e/monitor.spec.ts`
- `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## Scope

### In scope

- Compress Data Panel visual density so the four analytics cards are usable at `1280x720` CSS px.
- Replace the misleading Stop glyph with a real stop square while preserving the existing `Stop live run` command path and disabled state.
- Move Runner connection/status details out of Layers & Filters and into the top Run/Data Context surface by consolidating the existing left context blocks.
- Remove Runner UI from Layers & Filters; that panel must contain only map presentation controls.
- Implement or complete Map-only fullscreen shell behavior: view-only, Map + eligible Focus overlay only, optional Data Panel bottom overlay at normal track height, no run controls while fullscreen.
- Preserve stable map geometry: layer toggles, runner status, Data Panel density changes, and fullscreen exit must not reset camera/selection unless the existing selected target becomes incompatible or disappears.

### Out of scope

- New Runner/Core commands.
- Changing simulation speed semantics beyond existing UI presentation.
- New Observer/ALIF fields.
- Source-backed chart/accounting expansion beyond the data already available in UI.
- Full final Level/Pin/Focus mechanics not directly required by this shell cleanup.
- Rewriting all Monitor visual styling.

## Assumptions

- The user's "runner in navigator" means the persistent top context area, specifically the Run & Data Context Bar, replacing/consolidating the current left run identity/scenario blocks. Putting HTTP endpoint details in the global workspace navigation would crowd the primary workspace tabs and is lower confidence.
- Data Panel compression is a visual-density fix, not a source-data rewrite. Current cards may still show unavailable/stub data where Observer contracts do not exist.
- Stop command behavior remains `controller.stopRun`; this slice changes glyph, labels, confirmation presentation if already available, and placement clarity.

## Open questions

- None blocking. If the user wants Runner status in the global navigation row instead of the Run/Data Context Bar, revise `AL-007-S23-AC03` before execution.

## LINT_RESULT

**Scope:** `AL-007-S23`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`
**Mode:** deterministic

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| WARN | DL006/DL008 | `docs/delivery/status.md` | `Next` listed `AL-006-S02` even though roadmap candidate order selects `AL-007-S23`; this can misroute execution. | Update `Next` to `AL-007-S23` once this plan exists. |
| WARN | DL008 | `docs/delivery/roadmap.md` | Candidate Next Work still listed already closed `AL-002-S17`, `AL-006-S01`, `AL-005-S02`, `AL-002-S18`. | Replace with still-open next candidates. |
| WARN | DL008 | `docs/delivery/status.md` and `docs/delivery/roadmap.md` | `AL-007-S22` notes still mention old `1366x862` threshold after the accepted `1280x720` CSS baseline. | Update S22 notes to match current layout contract. |

**Decision:** PASS_WITH_WARNINGS. The warnings are deterministic documentation sync issues and are safe to repair as part of this planning change.

## Agent Scenario Cards

### `AL-007-S23-AC01`: Data Panel density at laptop CSS baseline

**Intent:** The Data Panel remains useful at `1280x720` without stealing Map dominance or relying on panel-only scroll.

**Priority:** High

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-monitor-v3.png`

**Given** the Monitor is opened at `1280x720` CSS px with the existing live or fixture projection,
**When** the bottom Data Panel renders World Analytics,
**Then** all four chart cards fit inside the fixed Data Panel track with compact borders/headers/labels, no horizontal page scroll, no Data Panel-only vertical scrollbar, and the Map remains the dominant center surface.

**TDD obligation:** Add Playwright geometry assertions before CSS/component changes.

**Evidence:** `AL-007-S23-EV01`, `AL-007-S23-EV02`

### `AL-007-S23-AC02`: Stop control visual semantics

**Intent:** Stop is visually unambiguous and no longer resembles previous-frame/back.

**Priority:** High

**Source links:** `docs/ui/control-center-block.md`, `docs/implementation/implementation-plan-runner.md`

**Given** a run can be stopped,
**When** the Run/Data Context Bar displays the Stop control,
**Then** the button uses a square stop glyph, keeps `aria-label="Stop live run"`, keeps the existing stop command path, and does not use previous-frame/back glyphs.

**TDD obligation:** Add RTL assertions for visible glyph/title/aria plus existing command callback.

**Evidence:** `AL-007-S23-EV03`, `AL-007-S23-EV04`

### `AL-007-S23-AC03`: Runner status belongs in Run/Data Context, not Layers

**Intent:** Layers & Filters controls only Map presentation; runner connectivity/status is part of persistent run context.

**Priority:** High

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/control-center-design-spec.md`, `docs/implementation/implementation-plan-runner.md`

**Given** the Runner is connected, connecting, disconnected, or using fixture fallback,
**When** the Monitor shell renders,
**Then** Runner status, endpoint/API/data state, scenario selector, and reconnect action are available in the Run/Data Context Bar compactly, while Layers & Filters has no `Runner:` section or connection details.

**TDD obligation:** Add RTL tests for component placement and Playwright visibility tests at `1280x720`.

**Evidence:** `AL-007-S23-EV05`, `AL-007-S23-EV06`

### `AL-007-S23-AC04`: Layers & Filters remains map-presentation only

**Intent:** Layer toggles do not change Data Context, Runner state, or Map geometry.

**Priority:** Medium

**Source links:** `docs/ui/control-center-block.md`

**Given** a user toggles resource/field/overlay controls,
**When** the panel state changes,
**Then** only Map presentation changes; selected entity, Inspector current data, Data Panel scope, displayed Tick, and camera geometry remain stable.

**TDD obligation:** Add component/state regression tests and one Playwright geometry test.

**Evidence:** `AL-007-S23-EV07`, `AL-007-S23-EV08`

### `AL-007-S23-AC05`: Map-only fullscreen shell

**Intent:** Fullscreen is purely a viewing mode and does not become a second run-control surface.

**Priority:** High

**Source links:** `docs/ui/control-center-block.md`

**Given** the user enters Map fullscreen,
**When** fullscreen is active,
**Then** only Map and eligible Focus overlay consume the viewport; global nav, run controls, Level, Layers, and Inspector are hidden; current viewport/camera/selection/pin are preserved; the Data Panel can be raised from the bottom at its normal Monitor height without resizing Map; leaving fullscreen restores the normal Monitor layout.

**TDD obligation:** Add Playwright fullscreen mode assertions before implementation.

**Evidence:** `AL-007-S23-EV09`, `AL-007-S23-EV10`

## Implementation file map

- Modify `ui/control-center/src/components/RunBar.tsx`: consolidate left context blocks, add compact Runner connection summary, scenario selector, reconnect action, and real Stop glyph.
- Modify `ui/control-center/src/components/LayerPanel.tsx`: remove `ConnectionPanel` details and runner props that become unnecessary; keep only layer/filter controls.
- Modify `ui/control-center/src/components/AppShell.tsx`: route runner props to `RunBar`; wire fullscreen shell state if it is not already centralized.
- Modify `ui/control-center/src/components/MonitorWorkspace.tsx`: complete fullscreen state trigger/exit and optional Data Panel overlay integration if not already available.
- Modify `ui/control-center/src/components/BottomDataPanel.tsx`: compact card headers/labels and reduce duplicate frames while keeping current source-backed/unavailable semantics.
- Modify `ui/control-center/src/styles/components.css`: compact data card density, top context runner styles, fullscreen shell styles, and removal of layer-embedded connection spacing.
- Modify `ui/control-center/src/styles/layout.css`: fullscreen shell grid/overflow guards if required.
- Modify/add tests:
  - `ui/control-center/src/components/RunBar.test.tsx`
  - `ui/control-center/src/components/LayerPanel.test.tsx`
  - `ui/control-center/src/components/BottomDataPanel.test.tsx`
  - `ui/control-center/src/components/MonitorWorkspace.test.tsx`
  - `ui/control-center/tests/e2e/monitor.spec.ts`
  - `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

## TDD tasks

### `AL-007-S23-T01`: RED for `AL-007-S23-AC01`

- [ ] Add a Playwright test in `ui/control-center/tests/e2e/monitor.spec.ts` named `AL-007-S23 keeps Data Panel compact at 1280x720`.
- [ ] Assert at viewport `1280x720`:
  - `monitor-data-track` height equals the adaptive `187px` track.
  - four `.v3-chart-card` elements are visible.
  - each card bounding box is inside `monitor-data-track`.
  - `document.documentElement.scrollWidth <= document.documentElement.clientWidth`.
  - `.cc-data-panel` has no panel-only vertical scrollbar.
- [ ] Run:

```powershell
Set-Location ui/control-center
npm.cmd run e2e -- monitor.spec.ts --grep "AL-007-S23 keeps Data Panel compact"
```

- [ ] Capture expected failure as `AL-007-S23-EV01`.

### `AL-007-S23-T02`: GREEN for `AL-007-S23-AC01`

- [ ] Reduce Data Panel internal density in `BottomDataPanel.tsx` and `components.css` only as much as needed:
  - remove duplicate decorative frames where they consume chart area;
  - shrink card header height and title line-height;
  - use compact legends;
  - preserve visible labels/counts required by the current data contract.
- [ ] Run the same Playwright command and capture pass as `AL-007-S23-EV02`.

### `AL-007-S23-T03`: RED for `AL-007-S23-AC02`

- [ ] Add or update `ui/control-center/src/components/RunBar.test.tsx`.
- [ ] Test that the Stop button:
  - is found by role/name `Stop live run`;
  - has visible text or accessible glyph `■`;
  - does not contain `◄`, `◀`, `|◄`, or `◄|`;
  - invokes `onStop` when enabled.
- [ ] Run:

```powershell
Set-Location ui/control-center
npm.cmd test -- src/components/RunBar.test.tsx --run
```

- [ ] Capture expected failure as `AL-007-S23-EV03`.

### `AL-007-S23-T04`: GREEN for `AL-007-S23-AC02`

- [ ] Replace the Stop button glyph/title in `RunBar.tsx` with a square stop symbol while preserving `aria-label="Stop live run"` and `onClick={onStop}`.
- [ ] Run the focused RunBar test and capture pass as `AL-007-S23-EV04`.

### `AL-007-S23-T05`: RED for `AL-007-S23-AC03`

- [ ] Add tests that fail on current placement:
  - `RunBar.test.tsx`: Run/Data Context Bar renders `Runner: Connected`, endpoint/API/data state, scenario selector, and reconnect action when props are supplied.
  - `LayerPanel.test.tsx`: Layers & Filters does not render `Runner:` or `Reconnect`.
  - `monitor.spec.ts`: at `1280x720`, runner status is visible in the top context surface and absent from Layers.
- [ ] Run:

```powershell
Set-Location ui/control-center
npm.cmd test -- src/components/RunBar.test.tsx src/components/LayerPanel.test.tsx --run
npm.cmd run e2e -- monitor.spec.ts --grep "runner status"
```

- [ ] Capture expected failures as `AL-007-S23-EV05`.

### `AL-007-S23-T06`: GREEN for `AL-007-S23-AC03`

- [ ] Change `RunBarProps` to receive connection props currently sent to `LayerPanel`.
- [ ] Render a compact Runner status cluster in the Run/Data Context Bar by consolidating the current run identity/scenario blocks.
- [ ] Remove `ConnectionPanel` from `LayerPanel.tsx`.
- [ ] Keep scenario selection and reconnect available through the top context surface.
- [ ] Run focused tests and e2e; capture pass as `AL-007-S23-EV06`.

### `AL-007-S23-T07`: RED for `AL-007-S23-AC04`

- [ ] Add regression tests for layer toggles:
  - toggling resource/field rows does not call scenario/run/reconnect handlers;
  - toggling layer rows does not clear selected Cell;
  - e2e bounding boxes for World Viewer and Inspector stay unchanged after a layer toggle.
- [ ] Run:

```powershell
Set-Location ui/control-center
npm.cmd test -- src/components/LayerPanel.test.tsx src/app/appState.test.ts --run
npm.cmd run e2e -- monitor.spec.ts --grep "layer toggles keep monitor geometry"
```

- [ ] Capture expected failure as `AL-007-S23-EV07`.

### `AL-007-S23-T08`: GREEN for `AL-007-S23-AC04`

- [ ] Make layer control state local/UI presentation state only, using existing store actions where available.
- [ ] Remove any remaining layer-panel side effects that touch runner/scenario/data context.
- [ ] Run focused tests and capture pass as `AL-007-S23-EV08`.

### `AL-007-S23-T09`: RED for `AL-007-S23-AC05`

- [ ] Add Playwright tests for fullscreen:
  - entering fullscreen hides nav/run/level/layers/inspector;
  - Map fills available viewport;
  - run controls are not visible;
  - optional Data Panel overlay can be raised and uses normal Data Panel height;
  - exit restores normal layout and camera/selection.
- [ ] Run:

```powershell
Set-Location ui/control-center
npm.cmd run e2e -- monitor.spec.ts --grep "Map-only fullscreen"
```

- [ ] Capture expected failure as `AL-007-S23-EV09`.

### `AL-007-S23-T10`: GREEN for `AL-007-S23-AC05`

- [ ] Implement the minimal fullscreen shell state and CSS needed to satisfy `AL-007-S23-AC05`.
- [ ] Do not add run controls to fullscreen.
- [ ] Preserve existing camera/selection state objects on enter/exit.
- [ ] Run fullscreen e2e and capture pass as `AL-007-S23-EV10`.

### `AL-007-S23-T11`: REFACTOR and visual acceptance sweep

- [ ] Remove duplicated prop plumbing and dead CSS left by moving Runner status.
- [ ] Keep files small; do not split large components unless the split directly reduces S23 risk.
- [ ] Run:

```powershell
Set-Location ui/control-center
npm.cmd test -- --run
npm.cmd run build
npm.cmd run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts
```

- [ ] Capture final suite evidence as `AL-007-S23-EV11`.

### `AL-007-S23-T12`: Closure report and delivery sync

- [ ] Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s23-monitor-interaction-state.md`.
- [ ] Update:
  - `docs/delivery/roadmap.md`
  - `docs/delivery/status.md`
  - `docs/delivery/acceptance.md`
  - `docs/delivery/worklog-ledger.md`
- [ ] Review `Candidate Next Work` in the same pass.
- [ ] Run deterministic delivery lint/checks over changed delivery docs.
- [ ] Capture delivery sync as `AL-007-S23-EV12`.

## Verification commands

```powershell
Set-Location ui/control-center
npm.cmd test -- --run
npm.cmd run build
npm.cmd run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts
```

Optional live smoke when Runner is already running:

```powershell
Set-Location ui/control-center
npm.cmd run e2e -- live-runner.spec.ts
```

## Completion criteria

`AL-007-S23` can move to closure only when:

- all `AL-007-S23-AC01..AC05` scenarios have RED and GREEN evidence;
- Data Panel is compact at `1280x720` without hiding Map;
- Stop is visually a stop command;
- Runner connection/status is absent from Layers and present in the top context surface;
- fullscreen is view-only and preserves state on exit;
- all required verification commands pass or have documented environmental blockers;
- closure verification confirms coverage before any `done` status update.

## Approval gate

Reply `OK EXECUTE AL-007-S23` to authorize execution of this TDD plan.

Reply `CHANGE AL-007-S23` with corrections to revise the plan.
