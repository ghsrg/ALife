# Monitor Selection, Focus, And Data Scope Gap Plan

Status: proposal

## Source of truth checked

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/ui/INDEX.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/control-center-block.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/components/LevelPanel.tsx`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/app/monitorSurfaceModel.ts`

## Audit result

The final Monitor behavior described in `control-center-design-spec.md` and
`control-center-block.md` is not fully implemented.

Implemented or partially implemented:

- Fixed Monitor layout, Level/Layers/Map/Inspector/Data Panel tracks.
- Empty Map click clears current Cell selection.
- Click-drag pans Map.
- Wheel zooms Map.
- Fit World exists.
- Basic single Cell selection exists.
- Cells lifecycle card is a compact stacked progress bar.
- Data Panel has no tabs.
- Layers/Filters dynamic Fields/Resources list has local scroll.
- Runner status is no longer in Layers.
- `MonitorDataPanelProjection/v1` exists with World lifecycle/resource accounting and explicit unavailable sections.

Not fully implemented:

- Selection model is still Cell-only.
- `World` Level still selects Cells through hit targets; it does not select a canonical World block.
- Multi-selection does not exist.
- `Shift + click` and `Shift + drag-select` do not exist.
- Level-specific selection semantics are not implemented.
- Incompatible selection is not cleared on Level change with an explicit reason.
- If selected live target disappears, app auto-selects the first Cell; expected behavior is clear selection and show a temporary reason.
- Focus opens immediately on any selected Cell; expected behavior is single click selects Inspector only, double click opens/closes Focus.
- Focus is Cell-only and not Level-bound.
- Focus has no close/Escape behavior and no explicit open state.
- Focus is not the final rich `413 x 399` overlay model for World block, Cell, Organism, Lineage, or Evolution contexts.
- Inspector is Cell-only and does not show World total without selection.
- Pin is local inside `CellInspector`; it is not app-wide `entity/selection set + Data Context + Tick + completeness`.
- Pin does not clear current selection after pinning.
- Data Panel does not use `analysis scope` from World block or multi-selection.
- Data Panel does not compare pinned baseline vs current selection as a same-scale overlay/contour.
- Data Panel distribution charts still ignore selection scope.
- Resource/Material explicit type selector is not implemented.
- Cell Energy `Cells/Heatmap`, Joints, and Trail controls are placeholders or UI-only labels.
- Lineages, Evolution, Analytics levels are disabled in `LevelPanel`.
- Organisms Level is disabled despite being part of the final Monitor Level model.
- Map highlighting for Lineage/Evolution/Analytics contexts is not implemented.
- Material Cycle and Energy Flow remain unavailable pending Core/Observer contracts.
- Cells roles, Organism behavior profiles, Lineage, Evolution, and Analytics cards remain unavailable pending Observer/Runner contracts.

## Required behavior summary

### Selection

- Single click selects according to active Level:
  - World: one World block / projection grid cell.
  - Cells: one Cell.
  - Organisms: one observer-side OrganismView.
  - Lineages: click Cell selects its lineage id.
  - Evolution: click Cell selects its Genome.
  - Analytics: chart/bar/segment selects analytical subset; Map highlight only.
- Empty Map click clears current selection and returns Inspector to World total.
- Click-drag without modifier pans Map.
- Wheel scroll zooms Map.
- `Shift + click` toggles one compatible target in the current selection set.
- `Shift + drag-select` draws a selection rectangle and adds all intersecting compatible targets.
- Multi-selection is Inspector/Data Panel only and never opens Focus.
- Live unpinned selection follows the same entity/set and updates each displayed Tick.
- Pause freezes displayed context.
- Dead is still valid selected state.
- Disappeared target clears selection, closes Focus, and shows a temporary reason.

### Focus

- Focus is explicitly opened/closed.
- Single click does not open Focus.
- Double click on the already selected compatible target toggles Focus.
- Focus closes on `Escape`, close button, incompatible Level change, disappeared target, and new run after Stop.
- Multi-selection never opens Focus.
- Analytics Level has no default Focus.
- Focus content is Level-bound and source/projection-backed; no invented anatomy or fake semantics.

### Data Panel scope

- Data Panel content is `active Level + analysis scope + optional Pin`.
- No selection at World Level means World total.
- One World block selection scopes World cards to that block.
- Multi-selection scopes compatible aggregates/distributions/histograms.
- Single Cell/Organism selection does not rebuild distribution charts; detail stays in Inspector/Focus.
- Pin compares pinned baseline vs current selection only when compatible, as an overlay/contour on the same chart scale.

## Proposed delivery breakdown

### AL-007-S26: Monitor Selection Model And Map Interactions

Goal: replace Cell-only app selection with a typed Level-aware selection model.

Acceptance:

- `AL-007-S26-AC01`: App state stores `currentSelection` as a discriminated union: `none`, `world-block`, `cell`, `organism`, `lineage`, `genome`, `analytics-subset`, or `selection-set`.
- `AL-007-S26-AC02`: WorldViewer supports single click, empty click, click-drag pan, wheel zoom, `Shift + click`, and `Shift + drag-select` without conflict.
- `AL-007-S26-AC03`: World Level click selects a World block, not a Cell.
- `AL-007-S26-AC04`: Level change clears incompatible selection with a visible transient reason.
- `AL-007-S26-AC05`: Live target disappearance clears selection; it does not auto-select the first available Cell.

Primary files:

- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/app/selectionModel.ts`
- `ui/control-center/src/app/selectionModel.test.ts`
- `ui/control-center/src/components/AppShell.tsx`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/components/WorldViewer.test.tsx`
- `ui/control-center/src/viewer/viewerHitTargets.ts`

Verification:

```powershell
npm.cmd test -- src/app/selectionModel.test.ts src/app/appState.test.ts src/components/WorldViewer.test.tsx src/App.test.tsx --run
npm.cmd run build
```

### AL-007-S27: Focus Overlay State And Level-Bound Focus Content

Goal: make Focus explicit, double-click driven, closeable, and Level-bound.

Acceptance:

- `AL-007-S27-AC01`: Single click selects but does not open Focus.
- `AL-007-S27-AC02`: Double click on selected compatible target opens/closes Focus.
- `AL-007-S27-AC03`: Escape and close button close Focus.
- `AL-007-S27-AC04`: Multi-selection and Analytics Level do not open Focus.
- `AL-007-S27-AC05`: Focus supports World block and Cell states first; Organism/Lineage/Evolution render exact unavailable or source-backed content without fake visuals.

Primary files:

- `ui/control-center/src/app/focusModel.ts`
- `ui/control-center/src/app/focusModel.test.ts`
- `ui/control-center/src/components/FocusPanel.tsx`
- `ui/control-center/src/components/FocusPanel.test.tsx`
- `ui/control-center/src/components/SelectedEntityFocusCard.tsx` replaced or removed.
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`

Verification:

```powershell
npm.cmd test -- src/app/focusModel.test.ts src/components/FocusPanel.test.tsx src/components/WorldViewer.test.tsx src/App.test.tsx --run
npm.cmd run build
```

### AL-007-S28: Inspector, Pin, And Selection Comparison

Goal: make Inspector and Pin follow the final contract.

Acceptance:

- `AL-007-S28-AC01`: Inspector without selection shows World total from source-backed projections.
- `AL-007-S28-AC02`: Inspector with single selection shows compatible detail.
- `AL-007-S28-AC03`: Inspector with multi-selection shows aggregate selection-set values.
- `AL-007-S28-AC04`: Pin stores `entity/selection set + Data Context + Tick + completeness`.
- `AL-007-S28-AC05`: Pressing Pin stores baseline and clears current selection.
- `AL-007-S28-AC06`: Pin persists when current target disappears; Stop followed by new run clears Pin, selection, and Focus.

Primary files:

- `ui/control-center/src/app/pinModel.ts`
- `ui/control-center/src/app/pinModel.test.ts`
- `ui/control-center/src/components/InspectorPanel.tsx`
- `ui/control-center/src/components/CellInspector.tsx`
- `ui/control-center/src/components/InspectorPanel.test.tsx`
- `ui/control-center/src/app/appState.ts`

Verification:

```powershell
npm.cmd test -- src/app/pinModel.test.ts src/app/appState.test.ts src/components/InspectorPanel.test.tsx src/components/CellInspector.test.tsx --run
npm.cmd run build
```

### AL-007-S29: Data Panel Analysis Scope

Goal: make Data Panel consume `active Level + analysis scope + optional Pin`.

Acceptance:

- `AL-007-S29-AC01`: World total is default Data Panel scope.
- `AL-007-S29-AC02`: World block selection scopes World cards to that block.
- `AL-007-S29-AC03`: Multi-selection scopes compatible distributions/histograms.
- `AL-007-S29-AC04`: Single Cell/Organism selection does not rebuild distribution charts.
- `AL-007-S29-AC05`: Compatible Pin comparison overlays current chart; no duplicate chart card.
- `AL-007-S29-AC06`: Resource/Material target requires explicit registry type selector unless a validated combined contract exists.

Primary files:

- `ui/control-center/src/app/monitorSurfaceModel.ts`
- `ui/control-center/src/app/monitorSurfaceModel.test.ts`
- `ui/control-center/src/app/analysisScopeModel.ts`
- `ui/control-center/src/app/analysisScopeModel.test.ts`
- `ui/control-center/src/components/BottomDataPanel.tsx`
- `ui/control-center/src/components/BottomDataPanel.test.tsx`

Verification:

```powershell
npm.cmd test -- src/app/analysisScopeModel.test.ts src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
npm.cmd run build
```

### AL-007-S25A: Cells Classification Payloads

Goal: populate Cells observed primary roles and potential markers from Observer classification rows.

Acceptance:

- `AL-007-S25A-AC01`: Runner Monitor payload exposes typed Cell observed role rows.
- `AL-007-S25A-AC02`: Runner Monitor payload exposes typed potential role marker rows.
- `AL-007-S25A-AC03`: UI renders role bars with potential markers only from typed payloads.
- `AL-007-S25A-AC04`: UI remains unavailable when typed classification rows are absent.

Primary files:

- `src/observer/monitor_payloads.rs`
- `src/viewer_server/api/projections.rs`
- `tests/observer_monitor_payloads.rs`
- `tests/runner_monitor_projections.rs`
- `ui/control-center/src/projection/monitorProjectionAdapter.ts`
- `ui/control-center/src/app/monitorSurfaceModel.ts`

Verification:

```powershell
cargo test --test observer_monitor_payloads -- --nocapture
cargo test --test runner_monitor_projections -- --nocapture
npm.cmd test -- src/projection/monitorProjectionAdapter.test.ts src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

### AL-007-S25B: Organisms, Lineages, Evolution, Analytics Contracts

Goal: activate remaining Level cards from source-backed Observer/Runner projections.

Acceptance:

- Organisms: behavior profiles and size bins from OrganismView/behavior projections.
- Lineages: current population, history, genealogy, spatial footprint.
- Evolution: genome provenance, mutation history, diversity, carrier history.
- Analytics: selected metric descriptor with full provenance.

Primary files:

- `src/observer/monitor_payloads.rs`
- `src/observer/organism_view.rs`
- `src/observer/evolution_suite.rs`
- `src/viewer_server/api/projections.rs`
- `ui/control-center/src/projection/monitorProjectionAdapter.ts`
- `ui/control-center/src/app/monitorSurfaceModel.ts`

Verification:

```powershell
cargo test --test observer_monitor_payloads -- --nocapture
cargo test --test runner_monitor_projections -- --nocapture
npm.cmd test -- src/projection/monitorProjectionAdapter.test.ts src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

### AL-007-S25C: Material And Energy Accounting Contracts

Goal: make Material Cycle and Energy Flow source-backed and conservation-aware.

Acceptance:

- Material Cycle keeps Resources, Materials, MaterialFragments, decomposing Cells, and sinks distinct.
- Energy Flow uses Core/Observer accounting; UI never estimates it from resource values.
- Unclassified loss remains visible when source accounting is incomplete.
- Time charts consume UI RRD source-backed samples.

Primary files:

- `src/core/snapshot.rs`
- `src/observer/projection.rs`
- `src/observer/monitor_payloads.rs`
- `tests/observer_monitor_payloads.rs`
- `ui/control-center/src/app/rrdMetricHistory.ts`
- `ui/control-center/src/app/monitorSurfaceModel.ts`

Verification:

```powershell
cargo test --test observer_monitor_payloads -- --nocapture
npm.cmd test -- src/app/rrdMetricHistory.test.ts src/app/monitorSurfaceModel.test.ts src/components/BottomDataPanel.test.tsx --run
```

## Recommended order

1. `AL-007-S26`: selection model first. This unlocks World block, multi-selection, and correct target lifecycle.
2. `AL-007-S27`: Focus behavior. This removes the current wrong auto-Focus behavior.
3. `AL-007-S28`: Inspector/Pin. This stabilizes comparison semantics before chart overlays.
4. `AL-007-S29`: Data Panel analysis scope. This needs S26/S28 to avoid fake scoping.
5. Continue `AL-007-S25A/B/C`: source-backed data contracts for the actual charts.

## Open decisions

- Exact World block grid source: use current `VisualWorldProjection.resourceLayers` grid geometry first, or introduce a dedicated shared projection grid descriptor.
- Whether disabled Levels should become clickable now with unavailable content, or stay disabled until their source-backed Monitor contracts exist.
- Whether Organisms Level selection should initially select only connected-component `OrganismView`, or also support single-cell organism candidates as explicit OrganismView rows.
