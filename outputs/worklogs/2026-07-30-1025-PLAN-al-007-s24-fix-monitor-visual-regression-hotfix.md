---
tags:
  - alife
  - worklog/plan
  - ui
---

# AL-007-S24-Fix Monitor Visual Regression Hotfix TDD Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Use `superpowers:test-driven-development` before changing behavior. Do not add Runner/Core/Observer contracts in this slice.

**Goal:** Restore Monitor visual usability after AL-007-S24 while preserving source-backed truthfulness and S22/S23 layout invariants.

**Architecture:** Keep S24 source-backed models, but add compact presentation view models and CSS contracts so provenance/unavailable metadata is secondary, not the primary card/body content. Fix the broken interactive surfaces by replacing fragile native/select and long inline metadata patterns with bounded, testable UI primitives.

**Tech Stack:** React, TypeScript, Vite, Vitest, Testing Library, Playwright.

---

## Plan Metadata

| Field | Value |
| --- | --- |
| Plan ID | `AL-007-S24-Fix` |
| Slice title | Monitor Visual Regression Hotfix |
| Status | TDD plan proposal |
| Created | 2026-07-30 |
| Depends on | `AL-007-S24` |
| Blocks | `AL-007-S25` visual execution priority |
| Confidence | medium-high |

## ROUTED

`delivery-control` classification: `BUGFIX` + `STABILIZATION` + `TDD_PLAN_REQUEST`.

Required downstream discipline:

- `roadmap-control`: create acceptance scenarios and stable traceability for `AL-007-S24-Fix`.
- `systematic-debugging`: root-cause classification before implementation.
- `test-driven-development`: every UI behavior fix starts with RED tests.
- `verification-before-completion`: closure requires fresh Vitest/build/Playwright evidence.

Degraded note: the `superpowers:writing-plans` alias path was not present locally, but the local `writing-plans` skill was available and used as the planning discipline.

## LINT_RESULT

**Scope:** `AL-007-S24-Fix`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/source-map.md`  
**Mode:** deterministic

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| WARN | `DL008` | `docs/delivery/status.md` | `Current Focus` is `AL-007-S25`, but the user reported blocking S24 visual regressions. | Add `AL-007-S24-Fix` as current planned hotfix before S25 execution. |
| WARN | `DL003` | `AL-007-S24-Fix-AC01..AC06` | Acceptance rows do not exist yet because the hotfix slice is new. | Add acceptance rows mapped to plan and expected evidence. |
| WARN | `DL012` | `outputs/worklogs/2026-07-29-1835-REPORT-al-007-s24-source-backed-monitor-surfaces.md` | S24 verification passed assertions, but acceptance did not constrain compact visual density of source metadata. | Hotfix must add regression assertions for density/overflow and visual priority. |

**Decision:** `PASS_WITH_WARNINGS`

The warnings justify creating a new regression hotfix slice. They do not require changing S24 source-backed semantics.

## Source-Of-Truth Hierarchy Used

1. `docs/PRINCIPLES.md`
2. `docs/INDEX.md`
3. `docs/ui/INDEX.md`
4. `docs/delivery/source-map.md`
5. `docs/delivery/roadmap.md`
6. `docs/delivery/status.md`
7. `docs/delivery/acceptance.md`
8. `docs/ui/control-center-design-spec.md`
9. `docs/ui/control-center-block.md`
10. `docs/ui/visualization.md`
11. `docs/ui/interaction.md`
12. `docs/ui/exploration.md`
13. `docs/ui/analytics.md`
14. `outputs/worklogs/2026-07-29-1835-REPORT-al-007-s24-source-backed-monitor-surfaces.md`
15. Current UI files listed below.

## Root-Cause Analysis

Observed symptoms from the reported screenshots and current code:

- `Layers & Filters` renders long source paths such as `CommittedSnapshot.heat` and verbose completeness text directly inside 262px rows, so rows wrap, push toggles, and make the panel unreadable.
- `BottomDataPanel` renders `Source`, `Completeness`, and `Unit` as large `<dl>` body content. That preserves truthfulness but destroys the chart-card visual hierarchy.
- `RunBar` uses a native `<select>` for scenario selection. In Windows/Chromium the opened dropdown can render white option backgrounds with very pale option text, outside reliable CSS control.
- `LevelPanel` still uses letters instead of the agreed icons and relies on vertical text density that can collide at the `1280x720` CSS baseline.
- Map selection has pieces of behavior, but no explicit visual contract/test that selected/search/focus states stay visible above resource/field layers and do not rely only on text labels.
- S24 tests checked source-backed semantics and basic layout, but did not assert compact text bounds, dropdown readability, metadata priority, or selected-map affordance visibility.

Hypothesis:

The root cause is not missing Runner/Core data. It is an S24 presentation-regression gap: source-backed metadata became primary visual content instead of compact provenance, and several dense controls lack bounded display models.

## Assumptions

- This slice is UI-only.
- It must not add real Energy Flow, Material Cycle, lineage, genome, or analytics contracts; those remain `AL-007-S25`.
- It may add UI presentation models, compact labels, custom listbox UI, icons, CSS tokens, component tests, and Playwright visual/layout assertions.
- It may update docs/delivery traceability for this hotfix, but must not rewrite UI Canon semantics.
- Native browser dropdown styling is insufficient for acceptance because the opened menu is OS/browser controlled; use an accessible custom scenario picker if guaranteed dark/readable popup is required.

## Open Questions

No blocking questions.

Non-blocking implementation choice: icons can be inline SVG components or CSS-mask/emoji-like glyphs. Prefer inline SVG to keep deterministic rendering and avoid external assets.

## Forbidden Scope

- Do not implement `AL-007-S25` Runner/Core/Observer contracts.
- Do not reintroduce fake chart values.
- Do not put Raw Data back into Data Panel.
- Do not add panel-only Data Panel scroll.
- Do not add native dropdown styling as the only fix if the opened menu remains browser-controlled.
- Do not let layer toggles mutate Data Context, selection, Tick, Inspector, or simulation state.
- Do not change map camera, Fit World, fullscreen, or Runner command semantics.
- Do not add new dependencies.

## File Map

### Create

- `ui/control-center/src/app/layerDisplayModel.ts`  
  Compact display names, secondary labels, tooltip/provenance strings, and row states for Field/Resource layer rows.

- `ui/control-center/src/app/layerDisplayModel.test.ts`  
  Unit tests for compact names, no full source path in primary row, bounded labels, and fallback names.

- `ui/control-center/src/components/ScenarioPicker.tsx`  
  Accessible compact custom listbox/popover for scenario selection in the Run/Data Context Bar.

- `ui/control-center/src/components/ScenarioPicker.test.tsx`  
  Component tests for dark/readable open state, keyboard/click selection, truncation, and selected item contrast hooks.

### Modify

- `ui/control-center/src/components/RunBar.tsx`  
  Replace native scenario `<select>` with `ScenarioPicker`; compact runner summary; remove remaining `Latency` label in favor of `FRAME AGE` if still present in UI.

- `ui/control-center/src/components/RunBar.test.tsx`  
  Add scenario picker readability and run bar density assertions.

- `ui/control-center/src/components/LevelPanel.tsx`  
  Replace letters with canonical icons and accessible labels.

- `ui/control-center/src/components/LevelPanel.test.tsx`  
  Add icon/order/active-state/compact-density tests.

- `ui/control-center/src/components/LayerPanel.tsx`  
  Use `layerDisplayModel`; show compact rows; move verbose provenance to `title`, details/expand affordance, or hidden accessible description.

- `ui/control-center/src/components/LayerPanel.test.tsx`  
  Add tests that primary rows do not show full projection paths, toggles remain visible, and only dynamic rows scroll.

- `ui/control-center/src/components/BottomDataPanel.tsx`  
  Render chart-card body first and provenance as compact chips/footer. Unavailable cards must be compact placeholders, not large metadata tables.

- `ui/control-center/src/components/BottomDataPanel.test.tsx`  
  Add tests for no large provenance body, compact unavailable placeholders, and card count/height-friendly content.

- `ui/control-center/src/components/WorldViewer.tsx`  
  Add explicit selected/search/highlight affordance classes/data attributes if missing.

- `ui/control-center/src/components/WorldViewer.test.tsx`  
  Add tests for selected cell affordance, search match affordance, no duplicate DOM selected ring, and foreground selection semantics.

- `ui/control-center/src/styles/components.css`  
  Add compact typography/density rules for RunBar, Layers, Data Panel, Level icons, scenario popover, and map selection affordances.

- `ui/control-center/tests/e2e/monitor.spec.ts`  
  Add visual-regression smoke at `1280x720` and `1920x1080`: no visible overlap in RunBar/Layers/Data Panel, readable scenario picker, selected Map affordance visible, no horizontal scroll.

- `docs/delivery/acceptance.md`  
  Add hotfix acceptance rows.

- `docs/delivery/roadmap.md`  
  Add hotfix row between S24 and S25; Candidate Next Work keeps S25 after hotfix.

- `docs/delivery/status.md`  
  Set hotfix as current planned focus; demote S25 to next/ready-to-plan until hotfix closes.

- `docs/delivery/worklog-ledger.md`  
  Add this plan row.

## Agent Scenario Cards

### AL-007-S24-Fix-AC01: Run/Data Context Bar remains readable and compact

**Source links:** `docs/ui/control-center-design-spec.md`, `docs/ui/control-center-block.md`, `docs/ui/interaction.md`  
**Intent:** Run state, runner status, scenario identity, controls, and metrics must remain readable without overlap at supported viewports.  
**Priority:** P0  
**Independent verification:** RunBar component tests plus Playwright at `1280x720` and `1920x1080`.

**Given** Monitor runs at the `1280x720` CSS baseline with connected Runner data and many scenario ids  
**When** the Run/Data Context Bar renders and the scenario menu opens  
**Then** scenario text is readable in dark theme, selected scenario is visible, no text overlays adjacent sections, and the bar retains its fixed height.

**TDD obligation:** Write failing tests that open the scenario selector and assert a readable custom listbox/popover before changing `RunBar`.

**Evidence:** `AL-007-S24-Fix-EV01`, `AL-007-S24-Fix-EV02`

### AL-007-S24-Fix-AC02: Layers & Filters dynamic rows are compact and source-backed

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/visualization.md`, `outputs/worklogs/2026-07-29-1835-REPORT-al-007-s24-source-backed-monitor-surfaces.md`  
**Intent:** S24 source-backed layer rows must not display full projection paths or verbose limitation text as primary row content.  
**Priority:** P0  
**Independent verification:** Layer display model tests, LayerPanel tests, Playwright layout smoke.

**Given** source-backed Field/Resource layers with long `sourceMetric.sourcePath` and completeness reasons  
**When** Layers & Filters renders  
**Then** each row shows compact swatch/name/toggle/gradient, full provenance is available only as secondary detail, the toggle remains visible, and only the dynamic Fields/Resources list scrolls.

**TDD obligation:** Add RED tests that fail because current rows expose long `CommittedSnapshot...` paths directly and wrap the 262px panel.

**Evidence:** `AL-007-S24-Fix-EV03`, `AL-007-S24-Fix-EV04`

### AL-007-S24-Fix-AC03: Data Panel preserves chart-card visual hierarchy

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/analytics.md`, `outputs/worklogs/2026-07-29-1835-REPORT-al-007-s24-source-backed-monitor-surfaces.md`  
**Intent:** Truthfulness metadata must be visible but secondary; charts/placeholders remain the primary visual surface.  
**Priority:** P0  
**Independent verification:** BottomDataPanel tests plus Playwright compact panel geometry.

**Given** World Data Panel has available Population Lifecycle and unavailable Energy Flow/time-series cards  
**When** the panel renders at `1280x720`  
**Then** cards show compact chart/placeholder bodies, provenance chips/footer, short unavailable reasons, no large `Source/Completeness` table body, and no card content exceeds the Data Panel track.

**TDD obligation:** Add RED tests that fail because current cards render `Source` and `Completeness` as large primary content.

**Evidence:** `AL-007-S24-Fix-EV05`, `AL-007-S24-Fix-EV06`

### AL-007-S24-Fix-AC04: Level Panel uses canonical icons without density regression

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/control-center-design-spec.md`  
**Intent:** Level Panel must match the agreed icon-based research-lens presentation and stay readable at the compact baseline.  
**Priority:** P1  
**Independent verification:** LevelPanel tests and Playwright geometry check.

**Given** all six Analysis Levels render in the fixed Level track  
**When** Monitor is displayed at `1280x720` and `1920x1080`  
**Then** the Level Panel uses icons in the canonical order, active state is visible, accessible level names remain available, and labels/icons do not overlap.

**TDD obligation:** Add RED tests that fail while `LevelPanel` still renders letters `W/C/O/L/E/A`.

**Evidence:** `AL-007-S24-Fix-EV07`, `AL-007-S24-Fix-EV08`

### AL-007-S24-Fix-AC05: Map selection affordances are explicit and visible

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/visualization.md`, `docs/ui/exploration.md`  
**Intent:** Selection/search/highlight states on Map must have explicit visual rules and stay above Resource/Field layers.  
**Priority:** P1  
**Independent verification:** WorldViewer component tests and Playwright selected-cell smoke.

**Given** Resource/Field layers are active and a Cell is selected or matched by search  
**When** the Map renders  
**Then** selected targets expose a visible semantic affordance, search matches have a distinct affordance, DOM hit targets do not draw a conflicting second selection ring, and selected Cells remain the foreground over resource backgrounds.

**TDD obligation:** Add RED tests for selected/search affordance class/data attributes and Playwright checks for visible selected label/marker before changing map presentation code.

**Evidence:** `AL-007-S24-Fix-EV09`, `AL-007-S24-Fix-EV10`

### AL-007-S24-Fix-AC06: Hotfix preserves S22/S23/S24 invariants

**Source links:** `docs/ui/control-center-block.md`, `outputs/worklogs/2026-07-29-1545-REPORT-al-007-s23-monitor-interaction-state.md`, `outputs/worklogs/2026-07-29-1835-REPORT-al-007-s24-source-backed-monitor-surfaces.md`  
**Intent:** Visual cleanup must not undo layout stabilization or source-backed truthfulness.  
**Priority:** P0 regression  
**Independent verification:** Focused component tests, full Vitest, build, selected Playwright.

**Given** the hotfix changes dense UI surfaces  
**When** the user toggles levels/layers, opens scenario picker, selects cells, uses Fit World, and switches fullscreen  
**Then** Map remains dominant, no horizontal page scroll appears, source-backed/unavailable states remain honest, and layer controls remain presentation-only.

**TDD obligation:** Add regression tests only for changed surfaces; keep existing S22/S23/S24 tests green.

**Evidence:** `AL-007-S24-Fix-EV11`, `AL-007-S24-Fix-EV12`

## TDD Task Plan

### AL-007-S24-Fix-T01: RED for Run/Data Context readability and custom scenario picker

**Files:**
- Modify: `ui/control-center/src/components/RunBar.test.tsx`
- Create: `ui/control-center/src/components/ScenarioPicker.test.tsx`
- Read: `ui/control-center/src/components/RunBar.tsx`

- [ ] Add a failing test that opens the scenario selector and expects `role="listbox"` with dark/readable item classes, selected item state, and no native `<select>` in `monitor-run-track`.
- [ ] Add a failing test that long scenario ids are truncated in the trigger but available through `title` or accessible label.
- [ ] Add a failing test that RunBar does not render `LATENCY` and uses `FRAME AGE` as the metric label.
- [ ] Run:

```powershell
npm.cmd test -- src/components/RunBar.test.tsx src/components/ScenarioPicker.test.tsx --run
```

- [ ] Expected RED: `ScenarioPicker` module is missing and `RunBar` still uses native `<select>`/`LATENCY`.
- [ ] Capture failure as `AL-007-S24-Fix-EV01`.

### AL-007-S24-Fix-T02: GREEN for Run/Data Context readability and custom scenario picker

**Files:**
- Create: `ui/control-center/src/components/ScenarioPicker.tsx`
- Modify: `ui/control-center/src/components/RunBar.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/components/RunBar.test.tsx`

- [ ] Implement `ScenarioPicker` as a button + popover listbox with roving simple click/keyboard selection and bounded item text.
- [ ] Replace RunBar native scenario `<select>` with `ScenarioPicker`.
- [ ] Compact runner summary so endpoint/API/data state cannot push scenario/control sections.
- [ ] Rename the metric label `LATENCY` to `FRAME AGE`; keep value unavailable if no contract exists.
- [ ] Run:

```powershell
npm.cmd test -- src/components/RunBar.test.tsx src/components/ScenarioPicker.test.tsx --run
```

- [ ] Expected GREEN: RunBar/scenario picker tests pass.
- [ ] Capture pass as `AL-007-S24-Fix-EV02`.

### AL-007-S24-Fix-T03: RED for compact Layers & Filters row model

**Files:**
- Create: `ui/control-center/src/app/layerDisplayModel.test.ts`
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`

- [ ] Add failing tests that `CommittedSnapshot.heat`, `CommittedSnapshot.waste`, and long completeness reasons are not primary row text.
- [ ] Add failing tests that compact rows expose swatch/name/toggle/gradient and full provenance through title/details only.
- [ ] Add failing test that `Cell Energy`, `Joints`, and `Trail` are absent outside `Cells`/`Organisms` when active Level is World.
- [ ] Run:

```powershell
npm.cmd test -- src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx --run
```

- [ ] Expected RED: display model missing and current LayerPanel exposes verbose source text.
- [ ] Capture failure as `AL-007-S24-Fix-EV03`.

### AL-007-S24-Fix-T04: GREEN for compact Layers & Filters row model

**Files:**
- Create: `ui/control-center/src/app/layerDisplayModel.ts`
- Modify: `ui/control-center/src/components/LayerPanel.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`

- [ ] Implement compact layer display model with:
  - primary label from semantic id or fallback `Field N` / `Resource Layer N`;
  - secondary short status such as `bounded`, `partial`, or formatted total;
  - full provenance string for `title`/details.
- [ ] Use the model in LayerPanel rows.
- [ ] Keep only Fields/Resources inside `data-testid="layers-dynamic-scroll"`.
- [ ] Gate `Cell Energy`, `Joints`, and `Trail` controls by active Level; if LayerPanel does not yet receive active Level, add a prop from AppShell.
- [ ] Run:

```powershell
npm.cmd test -- src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx src/app/appState.test.ts --run
```

- [ ] Expected GREEN: compact layer rows and side-effect boundaries pass.
- [ ] Capture pass as `AL-007-S24-Fix-EV04`.

### AL-007-S24-Fix-T05: RED for compact Data Panel chart-card hierarchy

**Files:**
- Modify: `ui/control-center/src/components/BottomDataPanel.test.tsx`
- Modify: `ui/control-center/src/app/monitorSurfaceModel.test.ts`

- [ ] Add failing tests that `.monitor-card-provenance` is rendered as compact metadata chips/footer and not the primary body.
- [ ] Add failing tests that unavailable cards render a compact placeholder body with short reason and source badge.
- [ ] Add failing tests that Population Lifecycle keeps a visible stacked/series representation rather than only metadata rows.
- [ ] Run:

```powershell
npm.cmd test -- src/components/BottomDataPanel.test.tsx src/app/monitorSurfaceModel.test.ts --run
```

- [ ] Expected RED: current BottomDataPanel renders large `Source`/`Completeness` body content.
- [ ] Capture failure as `AL-007-S24-Fix-EV05`.

### AL-007-S24-Fix-T06: GREEN for compact Data Panel chart-card hierarchy

**Files:**
- Modify: `ui/control-center/src/components/BottomDataPanel.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/components/BottomDataPanel.test.tsx`

- [ ] Render card body in this order: chart/placeholder content, rows/legend, compact provenance footer.
- [ ] Keep source/completeness/unit visible as chips with truncated text and full `title`.
- [ ] Render unavailable cards as chart placeholders with a short reason; do not expand metadata.
- [ ] Keep accounting selector compact and inside the Data Panel header without increasing track height.
- [ ] Run:

```powershell
npm.cmd test -- src/components/BottomDataPanel.test.tsx src/app/monitorSurfaceModel.test.ts --run
```

- [ ] Expected GREEN: Data Panel component/model tests pass.
- [ ] Capture pass as `AL-007-S24-Fix-EV06`.

### AL-007-S24-Fix-T07: RED for Level Panel icons and density

**Files:**
- Create or modify: `ui/control-center/src/components/LevelPanel.test.tsx`

- [ ] Add failing tests that Level Panel does not render letters `W`, `C`, `O`, `L`, `E`, `A` as the primary glyphs.
- [ ] Add failing tests for canonical icon order and accessible names: World, Cells, Organisms, Lineages, Evolution, Analytics.
- [ ] Add failing test that active level has an icon/track active state independent of text label.
- [ ] Run:

```powershell
npm.cmd test -- src/components/LevelPanel.test.tsx --run
```

- [ ] Expected RED: current LevelPanel uses letters and may lack a test file.
- [ ] Capture failure as `AL-007-S24-Fix-EV07`.

### AL-007-S24-Fix-T08: GREEN for Level Panel icons and density

**Files:**
- Modify: `ui/control-center/src/components/LevelPanel.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/components/LevelPanel.test.tsx`

- [ ] Replace letters with inline SVG icons for globe, dotted ring, connected spheres, graph, DNA, and bar chart.
- [ ] Keep level text as compact label and accessible name.
- [ ] Ensure disabled/available states remain clear.
- [ ] Run:

```powershell
npm.cmd test -- src/components/LevelPanel.test.tsx src/components/AppShell.test.tsx --run
```

- [ ] If `AppShell.test.tsx` does not exist, run `npm.cmd test -- src/App.test.tsx src/components/LevelPanel.test.tsx --run`.
- [ ] Expected GREEN: Level icon tests and shell smoke pass.
- [ ] Capture pass as `AL-007-S24-Fix-EV08`.

### AL-007-S24-Fix-T09: RED for Map selection/search affordance rules

**Files:**
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/tests/e2e/monitor.spec.ts`

- [ ] Add failing tests that selected targets expose an explicit selected affordance data attribute/class or visible selected label.
- [ ] Add failing tests that search-matched targets expose a distinct `search-match` affordance while non-matches are not exposed to accessibility during search.
- [ ] Add failing Playwright smoke that selecting a cell with active resource layers leaves selected detail visible over the map.
- [ ] Run:

```powershell
npm.cmd test -- src/components/WorldViewer.test.tsx --run
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts
```

- [ ] Expected RED only where current visible affordance contract is insufficient.
- [ ] Capture failure as `AL-007-S24-Fix-EV09`.

### AL-007-S24-Fix-T10: GREEN for Map selection/search affordance rules

**Files:**
- Modify: `ui/control-center/src/components/WorldViewer.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`
- Modify: `ui/control-center/tests/e2e/monitor.spec.ts`

- [ ] Add explicit selected/search/hit affordance classes or data attributes in DOM overlay without drawing a conflicting second selected ring.
- [ ] Ensure selected/search labels and focus styles sit above resource/field backgrounds.
- [ ] Keep Cell foreground renderer behavior intact; do not alter world physics/rendered committed values.
- [ ] Run:

```powershell
npm.cmd test -- src/components/WorldViewer.test.tsx --run
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts
```

- [ ] Expected GREEN: WorldViewer tests and selected Monitor e2e pass.
- [ ] Capture pass as `AL-007-S24-Fix-EV10`.

### AL-007-S24-Fix-T11: RED/GREEN viewport regression coverage

**Files:**
- Modify: `ui/control-center/tests/e2e/monitor.spec.ts`
- Modify: `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`

- [ ] Add e2e checks at `1280x720` and `1920x1080`:
  - no horizontal document scroll;
  - RunBar fixed height and readable scenario picker;
  - Layers dynamic rows do not cover Level label and toggles are visible;
  - Data Panel cards fit track height with compact metadata;
  - selected Map affordance is visible.
- [ ] Run selected e2e:

```powershell
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

- [ ] Implement only minimal CSS/component changes needed to turn RED to GREEN.
- [ ] Capture result as `AL-007-S24-Fix-EV11`.

### AL-007-S24-Fix-T12: Full verification and closure handoff

**Files:**
- Modify: `docs/delivery/roadmap.md`
- Modify: `docs/delivery/status.md`
- Create later during execution: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s24-fix-monitor-visual-regression-hotfix.md`

- [ ] Run focused regression:

```powershell
npm.cmd test -- src/components/RunBar.test.tsx src/components/ScenarioPicker.test.tsx src/components/LayerPanel.test.tsx src/components/BottomDataPanel.test.tsx src/components/LevelPanel.test.tsx src/components/WorldViewer.test.tsx --run
```

- [ ] Run full UI tests:

```powershell
npm.cmd test -- --run
```

- [ ] Run build:

```powershell
npm.cmd run build
```

- [ ] Run selected Playwright from `ui/control-center` cwd:

```powershell
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

- [ ] Update roadmap/status only after closure verification.
- [ ] Create closure report with AC/Task/Evidence matrix.
- [ ] Capture final verification as `AL-007-S24-Fix-EV12`.

## Verification Commands

Run these from `ui/control-center` unless explicitly stated otherwise:

```powershell
npm.cmd test -- src/components/RunBar.test.tsx src/components/ScenarioPicker.test.tsx --run
npm.cmd test -- src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx --run
npm.cmd test -- src/components/BottomDataPanel.test.tsx src/app/monitorSurfaceModel.test.ts --run
npm.cmd test -- src/components/LevelPanel.test.tsx --run
npm.cmd test -- src/components/WorldViewer.test.tsx --run
npm.cmd test -- --run
npm.cmd run build
npm.cmd exec -- playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

Note: if sandboxed Vite/Vitest cannot read `vite.config.ts`, rerun the same npm command with approved filesystem escalation. The previous S24 closure verified this as a sandbox command-shape issue, not a test assertion issue.

## Evidence Matrix

| Evidence ID | Scenario | Required evidence |
| --- | --- | --- |
| `AL-007-S24-Fix-EV01` | `AL-007-S24-Fix-AC01` | RED RunBar/ScenarioPicker output. |
| `AL-007-S24-Fix-EV02` | `AL-007-S24-Fix-AC01` | GREEN RunBar/ScenarioPicker output. |
| `AL-007-S24-Fix-EV03` | `AL-007-S24-Fix-AC02` | RED Layer display model/LayerPanel output. |
| `AL-007-S24-Fix-EV04` | `AL-007-S24-Fix-AC02` | GREEN Layer display model/LayerPanel output. |
| `AL-007-S24-Fix-EV05` | `AL-007-S24-Fix-AC03` | RED BottomDataPanel compact hierarchy output. |
| `AL-007-S24-Fix-EV06` | `AL-007-S24-Fix-AC03` | GREEN BottomDataPanel compact hierarchy output. |
| `AL-007-S24-Fix-EV07` | `AL-007-S24-Fix-AC04` | RED LevelPanel icon output. |
| `AL-007-S24-Fix-EV08` | `AL-007-S24-Fix-AC04` | GREEN LevelPanel icon output. |
| `AL-007-S24-Fix-EV09` | `AL-007-S24-Fix-AC05` | RED WorldViewer/e2e selected-map affordance output. |
| `AL-007-S24-Fix-EV10` | `AL-007-S24-Fix-AC05` | GREEN WorldViewer/e2e selected-map affordance output. |
| `AL-007-S24-Fix-EV11` | `AL-007-S24-Fix-AC06` | Selected viewport regression Playwright output. |
| `AL-007-S24-Fix-EV12` | all | Full Vitest, build, and selected Playwright output plus closure report. |

## Approval Gate

Reply `OK EXECUTE AL-007-S24-Fix` to authorize implementation of this TDD plan.

Reply `CHANGE AL-007-S24-Fix` with corrections to revise the plan.
