# AL-007-S24 Source-Backed Monitor Surfaces Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Do not use subagents unless the human explicitly asks for delegated execution.

**Goal:** Make Monitor Data Panel, chart cards, legends, selectors, and Layers & Filters truthful to available projections only, with explicit unavailable states for missing contracts.

**Architecture:** Add focused UI view-model modules that classify every displayed value as source-backed, derived from an approved UI buffer, or unavailable. Components render those view models and stop embedding hardcoded chart fallback numbers or heuristic biological labels. UI-only RRD buffers store compact metric samples for charts and trails without retaining full World frames.

**Tech Stack:** React, TypeScript, Vite, Vitest, Testing Library, Playwright.

---

## Plan Metadata

| Field | Value |
| --- | --- |
| Plan ID | `AL-007-S24` |
| Slice title | Source-Backed Monitor Surfaces |
| Status | TDD plan proposal |
| Created | 2026-07-29 |
| Depends on | `AL-007-S23` |
| Confidence | medium |

## Source-Of-Truth Hierarchy Used

1. `docs/PRINCIPLES.md`
2. `docs/delivery/roadmap.md`
3. `docs/delivery/status.md`
4. `docs/delivery/acceptance.md`
5. `docs/delivery/source-map.md`
6. `docs/ui/control-center-design-spec.md`
7. `docs/ui/control-center-block.md`
8. `docs/observer/projection-contract.md`
9. `docs/ui/analytics.md`
10. `docs/ui/visualization.md`
11. `docs/ui/exploration.md`
12. `docs/ui/interaction.md`
13. `docs/implementation/implementation-plan-ui.md`
14. Current UI implementation files listed below.

## LINT_RESULT

**Scope:** `AL-007-S24`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/source-map.md`  
**Mode:** deterministic

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| WARN | `DL008` | `docs/delivery/status.md` | `Current Focus` still named `AL-006-S03`, while `AL-007-S24` is requested, unblocked, and listed as Candidate Next Work. | Move operational Current Focus to `AL-007-S24` while this plan is reviewed. |
| WARN | `DL003` | `AL-007-S24-AC01..AC05` | Acceptance rows for the selected slice did not exist before planning. | Add proposed S24 acceptance rows mapped to this plan and expected implementation evidence. |

**Decision:** `PASS_WITH_WARNINGS`

The warnings are traceability gaps, not behavior blockers. This plan includes the required deterministic sync.

## Current Implementation Findings

These are characterization inputs for RED tests, not implementation instructions:

- `ui/control-center/src/components/BottomDataPanel.tsx` currently renders hardcoded fallback percentages such as `65.2%`, `23.1%`, `8.7%`, and `3.0%`.
- `BottomDataPanel.tsx` mixes "Energy & Matter" in one cycle card even though Energy Flow lacks an approved Core/Observer accounting contract.
- `BottomDataPanel.tsx` builds time charts from `state.frameHistory.slice(-30)` instead of the agreed UI RRD buffer.
- `BottomDataPanel.tsx` infers "Metabolic", "Transport", and "Structural" from Cell energy/radius heuristics. S24 must not fabricate classifications from raw geometry.
- `ui/control-center/src/components/LayerPanel.tsx` still renders `SCENE`/`DATA` tabs and `COLOR MODE`. Canon says Layers & Filters is one vertical list grouped as `Fields | Resources | Cell Energy | Structure | Selection`.
- `LayerPanel.tsx` includes fallback resource labels such as `Nutrient / Organic`, `Mineral`, and `Energy` when debug projections are absent. S24 must show unavailable state instead of pretending a source-backed layer exists.
- `ui/control-center/src/app/appState.ts` currently caps full frame history with `FRAME_HISTORY_LIMIT = 12`; S24 needs a separate compact metric RRD buffer, not a larger full-frame store.
- `ui/control-center/src/projection/types.ts` has projection availability and completeness concepts, but no UI-owned Energy Flow, Material Cycle, lineage, or genome accounting truth.

## Assumptions

- S24 is UI-only. It may add UI types, view models, tests, and component rendering, but it must not add Runner/Core/Observer API behavior.
- Missing contracts remain explicit `unavailable` surfaces. In particular, Energy Flow must not be approximated from resource `energy` values.
- Source-backed means: value came from the displayed projection payload, a documented Observer projection, or a UI RRD buffer fed by those values.
- The UI RRD buffer stores metric/trail samples only. It does not replace bounded live frame history and does not retain full World frames.
- If an existing projection field is typed as `unknown[]`, UI may display only an unavailable/provenance state until the payload is typed or safely narrowed.
- Single Cell/Organism selection updates Inspector/Focus; it does not rebuild Data Panel distribution charts. Multi-selection narrows compatible Data Panel aggregates.

## Open Questions

No blocking question. The main grey zone is Analytics cross-filter behavior; this plan keeps it minimal: chart interactions may highlight only source-backed identity sets and otherwise show disabled/unavailable state. Rich Analytics filtering can be a later slice.

## Forbidden Scope

- Do not change Core simulation mechanics.
- Do not change Runner command contracts.
- Do not add Observer accounting payloads in this slice.
- Do not estimate Energy Flow in UI.
- Do not reintroduce Monitor/Data tabs or Data Panel-only scroll.
- Do not move Raw Data back into Data Panel.
- Do not fabricate Cell roles, organism behavior, lineage, genome, or energy accounting from visual heuristics.
- Do not resize Map tracks when toggling layers, levels, selection, or Data Panel cards.
- Do not add a new charting dependency unless the existing CSS/SVG approach cannot pass acceptance.

## File Map

### Create

- `ui/control-center/src/app/monitorSurfaceModel.ts`  
  Owns source-backed/unavailable display model for Data Panel cards, chart legends, selectors, and provenance chips.

- `ui/control-center/src/app/monitorSurfaceModel.test.ts`  
  Unit tests for source-backed value classification, unavailable fallbacks, per-Level Data Panel card selection, and no fabricated classifications.

- `ui/control-center/src/app/rrdMetricHistory.ts`  
  UI-only compact metric/trail history with 100 newest consecutive samples, older 10x decimation tiers, max 1000 samples, and mean aggregation for collapsed intervals.

- `ui/control-center/src/app/rrdMetricHistory.test.ts`  
  Unit tests for RRD retention, mean aggregation, tick labels, and max sample bounds.

### Modify

- `ui/control-center/src/components/BottomDataPanel.tsx`  
  Render `monitorSurfaceModel` instead of inline hardcoded chart logic.

- `ui/control-center/src/components/BottomDataPanel.test.tsx`  
  Add component-level acceptance checks for no fake values, unavailable cards, selectors, legends, provenance, and Level-specific cards.

- `ui/control-center/src/components/LayerPanel.tsx`  
  Remove tabs/color mode, render canonical grouped source-backed layer controls, and keep layer state presentation-only.

- `ui/control-center/src/components/LayerPanel.test.tsx`  
  Add tests for canonical groups, absence of forbidden controls, dynamic-list scroll ownership, unavailable projection messaging, and layer toggle side-effect boundaries.

- `ui/control-center/src/app/appState.ts`  
  Add minimal state for analysis level / source-backed surface selection / UI RRD metrics if no existing state already owns it.

- `ui/control-center/src/app/appState.test.ts`  
  Add tests proving RRD metrics do not expand full frame history and layer toggles do not mutate Data Context, Tick, selection, or Inspector inputs.

- `ui/control-center/src/projection/types.ts`  
  Add UI-facing types only if needed to represent data-state/provenance; do not claim new backend contracts.

- `ui/control-center/tests/e2e/monitor.spec.ts`  
  Add S24 smoke checks for `1280x720` and `1920x1080`: source-backed/unavailable surfaces are visible, Data Panel remains compact, and Map remains dominant.

- `docs/delivery/acceptance.md`  
  Keep S24 acceptance rows in the current/next matrix.

- `docs/delivery/status.md`  
  Keep operational Current Focus aligned to S24 while the plan is reviewed/executed.

- `docs/delivery/roadmap.md`  
  Update S24 evidence only after implementation and closure verification, not during this plan proposal.

## Agent Scenario Cards

### AL-007-S24-AC01: Data Panel never renders fake Monitor values

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/control-center-design-spec.md`, `docs/observer/projection-contract.md`  
**Intent:** Data Panel values must be truthful to displayed projections or explicitly unavailable.  
**Priority:** P0  
**Independent verification:** Vitest component/model tests plus Playwright visible text checks.

**Given** a Monitor frame with no Energy Flow, Material Cycle, typed classification, lineage, or genome accounting projection  
**When** Data Panel renders World/Cells/Organisms/Lineages/Evolution/Analytics surfaces  
**Then** it shows source-backed counts/values only where data exists and renders explicit unavailable cards with reason/provenance everywhere else.

**TDD obligation:** Write failing tests that catch hardcoded fallback numbers and heuristic labels before changing `BottomDataPanel.tsx`.

**Evidence:** `AL-007-S24-EV01`, `AL-007-S24-EV02`

### AL-007-S24-AC02: Time-series charts use one UI RRD metric buffer

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/analytics.md`, `docs/observer/projection-contract.md`  
**Intent:** Data Panel time charts and trails must use bounded compact metric history, not recent full-frame slicing.  
**Priority:** P0  
**Independent verification:** Pure unit tests for RRD compaction and component tests for chart source labels.

**Given** more than 1000 tick samples arrive from displayed projections  
**When** UI records chart metrics  
**Then** the newest 100 samples remain consecutive, older samples are decimated by 10x tiers, collapsed numeric intervals store mean values, the total sample count stays at or below 1000, and chart axes expose actual tick positions/sampling density.

**TDD obligation:** Implement `rrdMetricHistory` only after tests fail for retention, mean aggregation, and max bounds.

**Evidence:** `AL-007-S24-EV03`, `AL-007-S24-EV04`

### AL-007-S24-AC03: Layers & Filters is canonical and source-backed

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/visualization.md`, `docs/ui/interaction.md`  
**Intent:** Layers & Filters controls Map presentation only and no longer contains tabs, primary color mode, fake layer presets, or runner/debug controls.  
**Priority:** P0  
**Independent verification:** LayerPanel unit tests and e2e layout/presentation checks.

**Given** Monitor has available or unavailable Field/Resource projections  
**When** Layers & Filters renders  
**Then** it shows one vertical grouped list `Fields | Resources | Cell Energy | Structure | Selection`, source-backed rows with swatch/toggle/gradient/provenance where available, and unavailable messages where missing.

**TDD obligation:** Write failing tests that assert absence of `SCENE`, `DATA`, `COLOR MODE`, and fallback resource names before refactoring the panel.

**Evidence:** `AL-007-S24-EV05`, `AL-007-S24-EV06`

### AL-007-S24-AC04: Data Panel follows Level and analysis scope

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/exploration.md`, `docs/ui/analytics.md`  
**Intent:** Data Panel content is controlled by active Level plus analysis scope plus optional Pin, with no tabs.  
**Priority:** P1  
**Independent verification:** Model and component tests for each Level.

**Given** the user switches among World, Cells, Organisms, Lineages, Evolution, and Analytics Levels  
**When** Data Panel renders  
**Then** it selects the required card set for that Level, narrows only on compatible multi-selection, keeps single-entity details in Inspector/Focus, and renders unavailable states for missing lineage/genome/analytics contracts.

**TDD obligation:** Add failing tests for each Level card set before wiring level-aware model output.

**Evidence:** `AL-007-S24-EV07`, `AL-007-S24-EV08`

### AL-007-S24-AC05: Chart selectors, legends, and provenance are explicit

**Source links:** `docs/ui/control-center-block.md`, `docs/observer/projection-contract.md`, `docs/ui/analytics.md`  
**Intent:** Every chart-like surface must state what is selected, what source produced it, what unit/aggregation is used, and what is unavailable.  
**Priority:** P1  
**Independent verification:** Component tests for selectors/provenance plus Playwright smoke.

**Given** World Data Panel defaults to Energy accounting but Energy Flow is not source-backed  
**When** the user sees or changes chart target selectors  
**Then** Energy shows unavailable accounting, Resource/Material require a second explicit type selector when source types exist, legends match rendered series, and source/completeness/provenance are visible.

**TDD obligation:** Write failing tests that reject mixed Resource/Material/Energy title text and missing provenance before changing chart rendering.

**Evidence:** `AL-007-S24-EV09`, `AL-007-S24-EV10`

### AL-007-S24-AC06: S24 preserves S22/S23 layout and interaction invariants

**Source links:** `docs/ui/control-center-block.md`, `outputs/worklogs/2026-07-29-1545-REPORT-al-007-s23-monitor-interaction-state.md`, `outputs/worklogs/2026-07-29-1727-REPORT-fit-world-camera-fix.md`  
**Intent:** Source-backed surfaces must not regress Map-first layout, Fit World, camera stability, or layer-only presentation behavior.  
**Priority:** P0 regression  
**Independent verification:** Existing and added Vitest/Playwright checks.

**Given** S24 changes Data Panel and Layers & Filters rendering  
**When** the user toggles layers, switches Levels, selects entities, enters/exits fullscreen, and presses Fit World  
**Then** Map remains dominant at `1280x720` and `1920x1080`, layer changes do not alter simulation/data context, and Fit World still maximizes the whole World inside the Map viewport.

**TDD obligation:** Add regression tests only for changed paths; keep existing S22/S23 tests green throughout.

**Evidence:** `AL-007-S24-EV11`, `AL-007-S24-EV12`

## TDD Task Plan

### AL-007-S24-T01: RED for AC01 source-backed Data Panel model

**Files:**
- Create: `ui/control-center/src/app/monitorSurfaceModel.test.ts`
- Read: `ui/control-center/src/components/BottomDataPanel.tsx`

- [ ] Add failing tests proving `buildMonitorSurfaceModel(state)` returns `unavailable` for Energy Flow when no accounting projection exists.
- [ ] Add failing tests proving no card label/value contains hardcoded fallback values `65.2%`, `23.1%`, `8.7%`, `3.0%`, or `1270 amu`.
- [ ] Add failing tests proving role/behavior charts do not infer classifications from Cell `energy` or `radius`.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts --run
```

- [ ] Expected RED: module or assertions fail because the model does not exist and current component still owns fake inline values.
- [ ] Capture failure as `AL-007-S24-EV01`.

### AL-007-S24-T02: GREEN for AC01 source-backed Data Panel model

**Files:**
- Create: `ui/control-center/src/app/monitorSurfaceModel.ts`
- Modify: `ui/control-center/src/components/BottomDataPanel.tsx`
- Modify: `ui/control-center/src/components/BottomDataPanel.test.tsx`

- [ ] Implement `MonitorDataState = 'available' | 'partial' | 'unavailable'`.
- [ ] Implement `MonitorSurfaceCard` view-model records with title, level, state, source, completeness, reason, series, and display rows.
- [ ] Move World Population Lifecycle into the model using only displayed frame Cell lifecycle fields or explicit unavailable state.
- [ ] Replace inline cycle/time/role fallback logic in `BottomDataPanel.tsx` with model rendering.
- [ ] Render unavailable cards with visible reason and source requirement.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

- [ ] Expected GREEN: tests pass and fake fallback values are absent.
- [ ] Capture pass as `AL-007-S24-EV02`.

### AL-007-S24-T03: RED for AC02 UI RRD metric history

**Files:**
- Create: `ui/control-center/src/app/rrdMetricHistory.test.ts`

- [ ] Add failing test: after 150 sequential samples, newest 100 ticks are consecutive.
- [ ] Add failing test: after 1500 samples, total retained samples are `<= 1000`.
- [ ] Add failing test: collapsed numeric windows store mean value, start tick, end tick, and source sample count.
- [ ] Add failing test: trail samples collapse to mean `(x, y)` and expose aggregation metadata.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/rrdMetricHistory.test.ts --run
```

- [ ] Expected RED: module missing.
- [ ] Capture failure as `AL-007-S24-EV03`.

### AL-007-S24-T04: GREEN for AC02 UI RRD metric history

**Files:**
- Create: `ui/control-center/src/app/rrdMetricHistory.ts`
- Modify: `ui/control-center/src/app/appState.ts`
- Modify: `ui/control-center/src/app/appState.test.ts`
- Modify: `ui/control-center/src/app/monitorSurfaceModel.ts`

- [ ] Implement `appendRrdSample(history, sample, options)` with default max samples `1000`, newest consecutive count `100`, and decimation factor `10`.
- [ ] Implement `appendRrdPointSample(history, sample, options)` for trail-style `(x, y)` mean aggregation.
- [ ] Store chart metric history separately from full `frameHistory`; do not increase `FRAME_HISTORY_LIMIT`.
- [ ] Feed Data Panel time-series cards from RRD samples and expose tick range/sampling density labels.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/rrdMetricHistory.test.ts src/app/appState.test.ts src/app/monitorSurfaceModel.test.ts --run
```

- [ ] Expected GREEN: RRD behavior passes and frame history remains bounded.
- [ ] Capture pass as `AL-007-S24-EV04`.

### AL-007-S24-T05: RED for AC03 canonical Layers & Filters

**Files:**
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`

- [ ] Add failing test asserting rendered panel has groups `Fields`, `Resources`, `Cell Energy`, `Structure`, `Selection`.
- [ ] Add failing test asserting `SCENE`, `DATA`, and `COLOR MODE` are absent.
- [ ] Add failing test asserting dynamic Fields/Resources rows are inside the only local scroll container.
- [ ] Add failing test asserting fallback resource labels are not rendered when source projections are unavailable.
- [ ] Add failing test asserting toggling a layer does not mutate Data Context, Tick, selection, or Inspector-relevant state.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/components/LayerPanel.test.tsx --run
```

- [ ] Expected RED: current `LayerPanel.tsx` still renders tabs/color mode/fallback rows.
- [ ] Capture failure as `AL-007-S24-EV05`.

### AL-007-S24-T06: GREEN for AC03 canonical Layers & Filters

**Files:**
- Modify: `ui/control-center/src/components/LayerPanel.tsx`
- Modify: `ui/control-center/src/components/LayerPanel.test.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/app/appState.ts`

- [ ] Remove `SCENE`/`DATA` tabs and `COLOR MODE`.
- [ ] Render grouped sections in the canonical order.
- [ ] Render Field/Resource rows only from available projection metadata.
- [ ] Render explicit unavailable rows/messages when projection metadata is missing.
- [ ] Keep Cell Energy, Joints, and Trail controls level-compatible and presentation-only.
- [ ] Keep layer toggle state isolated to Map presentation state.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/components/LayerPanel.test.tsx src/app/appState.test.ts --run
```

- [ ] Expected GREEN: canonical panel rendering and side-effect boundaries pass.
- [ ] Capture pass as `AL-007-S24-EV06`.

### AL-007-S24-T07: RED for AC04 Level-aware Data Panel cards

**Files:**
- Modify: `ui/control-center/src/app/monitorSurfaceModel.test.ts`
- Modify: `ui/control-center/src/components/BottomDataPanel.test.tsx`

- [ ] Add failing tests for World card set: Population Lifecycle, selected Matter Cycle/Energy Flow, time evolution.
- [ ] Add failing tests for Cells card set: observed primary roles with Potential markers and Cell radius distribution; unavailable if classification projection is absent.
- [ ] Add failing tests for Organisms card set: behavior profile distribution and Cell-count size bins; unavailable if organism projection is absent.
- [ ] Add failing tests for Lineages, Evolution, and Analytics card sets with explicit unavailable/provenance states when contracts are missing.
- [ ] Add failing tests proving single Cell/Organism selection does not rebuild distributions, while compatible multi-selection can narrow aggregates.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

- [ ] Expected RED: current Data Panel is always World Analytics and does not follow active Level.
- [ ] Capture failure as `AL-007-S24-EV07`.

### AL-007-S24-T08: GREEN for AC04 Level-aware Data Panel cards

**Files:**
- Modify: `ui/control-center/src/app/monitorSurfaceModel.ts`
- Modify: `ui/control-center/src/components/BottomDataPanel.tsx`
- Modify: `ui/control-center/src/app/appState.ts`
- Modify: `ui/control-center/src/components/LevelPanel.tsx`

- [ ] Wire existing Level Panel state into Data Panel model; if no app-level state exists, add a minimal `activeLevel` store field.
- [ ] Implement per-Level card selection in the model.
- [ ] Render unavailable states for unimplemented lineage/genome/analytics source contracts.
- [ ] Keep single-entity detail in Inspector/Focus; keep Data Panel scoped to baseline or compatible multi-selection.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx src/components/LevelPanel.test.tsx --run
```

- [ ] Expected GREEN: Level-aware card sets pass.
- [ ] Capture pass as `AL-007-S24-EV08`.

### AL-007-S24-T09: RED for AC05 selectors, legends, and provenance

**Files:**
- Modify: `ui/control-center/src/app/monitorSurfaceModel.test.ts`
- Modify: `ui/control-center/src/components/BottomDataPanel.test.tsx`

- [ ] Add failing test proving World accounting target defaults to `Energy` for a new run.
- [ ] Add failing test proving Energy Flow shows unavailable without an accounting projection.
- [ ] Add failing test proving Resource/Material target requires a second selector for one source-backed registry type.
- [ ] Add failing test proving chart titles do not mix `RESOURCE CYCLE (ENERGY & MATTER)`.
- [ ] Add failing test proving every chart-like card exposes source, completeness, unit or unavailable reason, and legend labels matching rendered series.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

- [ ] Expected RED: current Data Panel has mixed title text and incomplete provenance.
- [ ] Capture failure as `AL-007-S24-EV09`.

### AL-007-S24-T10: GREEN for AC05 selectors, legends, and provenance

**Files:**
- Modify: `ui/control-center/src/app/monitorSurfaceModel.ts`
- Modify: `ui/control-center/src/components/BottomDataPanel.tsx`
- Modify: `ui/control-center/src/styles/components.css`
- Modify: `ui/control-center/src/app/appState.ts`

- [ ] Add run-scoped Data Panel accounting target state: `Energy | Resource | Material`.
- [ ] Default target to `Energy` on new run; retain explicit user choice for the run.
- [ ] Add Resource/Material type selector only when source-backed registry options exist.
- [ ] Render Energy accounting as unavailable until a source-backed projection exists.
- [ ] Render chart provenance chips and legends from the model.
- [ ] Run:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

- [ ] Expected GREEN: selector/provenance tests pass.
- [ ] Capture pass as `AL-007-S24-EV10`.

### AL-007-S24-T11: RED/GREEN regression for AC06 layout and interaction invariants

**Files:**
- Modify: `ui/control-center/tests/e2e/monitor.spec.ts`
- Modify: `ui/control-center/src/components/MonitorWorkspace.test.tsx`
- Modify: `ui/control-center/src/components/WorldViewer.test.tsx`

- [ ] Add failing or characterization e2e checks for `1280x720` and `1920x1080`: Map remains dominant, Data Panel does not create horizontal page scroll, and Layers/Filters is the only local dynamic scroll.
- [ ] Add regression check that Fit World still maximizes the whole World inside the Map viewport after layer/level/Data Panel interactions.
- [ ] Add regression check that layer toggles do not mutate selection, displayed Tick, data context, or Inspector text.
- [ ] Run the focused test set and confirm RED only where missing coverage is intentional:

```powershell
npm --prefix ui/control-center test -- src/components/MonitorWorkspace.test.tsx src/components/WorldViewer.test.tsx --run
npm --prefix ui/control-center exec playwright test tests/e2e/monitor.spec.ts
```

- [ ] Implement only minimal test-facing adjustments if the new S24 UI changes break layout or interaction invariants.
- [ ] Re-run the same commands until GREEN.
- [ ] Capture results as `AL-007-S24-EV11`.

### AL-007-S24-T12: REFACTOR, full verification, and report

**Files:**
- Modify: `docs/delivery/status.md`
- Modify: `docs/delivery/roadmap.md`
- Create: `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s24-source-backed-monitor-surfaces.md`

- [ ] Refactor only after all focused tests are green.
- [ ] Run full UI verification:

```powershell
npm --prefix ui/control-center test -- --run
npm --prefix ui/control-center run build
```

- [ ] Run selected browser acceptance if local runner/browser prerequisites are available:

```powershell
npm --prefix ui/control-center exec playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

- [ ] Update `docs/delivery/roadmap.md` S24 evidence only after verification.
- [ ] Update `docs/delivery/status.md` to move S24 out of Current Focus only after closure verification.
- [ ] Create closure report with AC/Task/Evidence matrix.
- [ ] Capture final verification as `AL-007-S24-EV12`.

## Verification Commands

Use these commands during execution:

```powershell
npm --prefix ui/control-center test -- src/app/monitorSurfaceModel.test.ts --run
npm --prefix ui/control-center test -- src/app/rrdMetricHistory.test.ts --run
npm --prefix ui/control-center test -- src/components/BottomDataPanel.test.ts src/components/LayerPanel.test.ts --run
npm --prefix ui/control-center test -- src/app/appState.test.ts src/components/MonitorWorkspace.test.tsx src/components/WorldViewer.test.tsx --run
npm --prefix ui/control-center test -- --run
npm --prefix ui/control-center run build
npm --prefix ui/control-center exec playwright test tests/e2e/monitor.spec.ts tests/e2e/ui-1c-a-visual.spec.ts
```

## Evidence Matrix

| Evidence ID | Scenario | Required evidence |
| --- | --- | --- |
| `AL-007-S24-EV01` | `AL-007-S24-AC01` | RED output for source-backed Data Panel tests. |
| `AL-007-S24-EV02` | `AL-007-S24-AC01` | GREEN output for source-backed Data Panel model and component tests. |
| `AL-007-S24-EV03` | `AL-007-S24-AC02` | RED output for RRD metric history tests. |
| `AL-007-S24-EV04` | `AL-007-S24-AC02` | GREEN output for RRD metric history and app state tests. |
| `AL-007-S24-EV05` | `AL-007-S24-AC03` | RED output for canonical Layers & Filters tests. |
| `AL-007-S24-EV06` | `AL-007-S24-AC03` | GREEN output for canonical Layers & Filters tests. |
| `AL-007-S24-EV07` | `AL-007-S24-AC04` | RED output for Level-aware Data Panel tests. |
| `AL-007-S24-EV08` | `AL-007-S24-AC04` | GREEN output for Level-aware Data Panel tests. |
| `AL-007-S24-EV09` | `AL-007-S24-AC05` | RED output for selector/legend/provenance tests. |
| `AL-007-S24-EV10` | `AL-007-S24-AC05` | GREEN output for selector/legend/provenance tests. |
| `AL-007-S24-EV11` | `AL-007-S24-AC06` | Focused regression/e2e output for layout, Fit World, and side-effect boundaries. |
| `AL-007-S24-EV12` | all | Full UI test/build and selected Playwright acceptance output. |

## Approval Gate

Reply `OK EXECUTE AL-007-S24` to authorize implementation of this TDD plan.

Reply `CHANGE AL-007-S24` with corrections to revise the plan.
