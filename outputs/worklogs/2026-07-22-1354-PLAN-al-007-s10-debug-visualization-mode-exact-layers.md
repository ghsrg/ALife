---
tags:
  - alife
  - worklog/plan
  - delivery/tdd
  - ui
  - debug-visualization
---

# AL-007-S10 Debug Visualization Mode And Exact Layers TDD Plan

Plan ID: `AL-007-S10`
Status at planning: `planned`
Legacy refs: `UI-2B`, `Visual A`, `Visual C`

## Goal

Expose the Observer projection payloads created by `AL-004-S05` in Control Center without faking unavailable data, and repair the current Monitor UX so the map is again the central working surface.

## Architecture

Add a read-only projection gateway path from Runner/Viewer Server to UI for typed Observer projection payloads, separate from the existing ALIF v2 binary frame stream. UI should merge those payloads into the existing projection/data-context model and render Debug Visualization Mode as Viewer layers with explicit exact/smooth state, legends, warnings, and unavailable states.

The UX patch is in scope because the current debug controls and context panels are already harming the primary Viewer hierarchy. It must reorganize existing controls and new debug controls into compact map-adjacent surfaces, not add another large persistent panel.

## Source Of Truth

- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `docs/implementation/implementation-plan-ui.md`
- `docs/ui/control-center-design-spec.md`
- `docs/ui/visualization.md`
- `docs/ui/architecture.md`
- `docs/ui/quality.md`
- `docs/observer/projection-contract.md`
- `src/observer/payloads.rs`
- `src/observer/projection.rs`
- `tests/observer_projection_payloads.rs`
- `src/viewer_server/state.rs`
- `src/viewer_server/api/mod.rs`
- `src/viewer_server/api/stream.rs`
- `ui/control-center/src/projection/types.ts`
- `ui/control-center/src/components/MonitorWorkspace.tsx`
- `ui/control-center/src/components/WorldViewer.tsx`
- `ui/control-center/src/styles/layout.css`
- `ui/control-center/src/styles/components.css`

Worklogs used only as evidence:

- `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md`
- `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md`

## Assumptions

- The existing ALIF v2 binary stream remains the live frame path for fast Cell rendering.
- `AL-007-S10` may add a bounded read-only JSON projection endpoint or equivalent side channel, but must not make full-world JSON the normal Viewer path.
- Per-Cell materials/internal resources are still partial until Core/Observer exposes those snapshot fields. UI must show that as partial/unavailable, not as zero or invented composition.
- `Needs Review`: exact endpoint naming should be decided during RED tests. Candidate names are `/projections/latest` or `/debug/projections/latest`; the final name should match existing viewer-server route style.

## Forbidden Scope

- No Core Tick behavior changes.
- No UI-side inference of materials, resources, classifications, contacts, or processes when the projection does not provide them.
- No full UI-2C inspector/search/filter/entity comparison scope.
- No UI-2D analytics dashboards, raw data grid, CSV/JSON debug export, or balance charts.
- No Bootstrap rich generator work.
- No new dependency unless a testable need appears and is approved.

## BDD Agent Scenario Cards

### `AL-007-S10-AC01` Projection Gateway

Given a live Runner has a committed snapshot and Observer can build `AL-004-S05` payloads,
When Control Center requests the latest debug projections,
Then the response exposes bounded `VisualWorldProjection`, coverage, warning, classification, and balance payload metadata with schema/completeness/provenance, without returning mutable `WorldState` or replacing the ALIF v2 frame stream.

TDD obligation: Rust endpoint/state tests first; prove no Core behavior dependency on UI projection payloads.

### `AL-007-S10-AC02` UI Projection Model

Given the UI receives a live ALIF frame plus optional Observer debug projections,
When projection payloads are complete, partial, or unavailable,
Then the UI model preserves projection kind, tick/run, completeness, source metrics, missing fields, and limitation text instead of converting missing values to zero or placeholder visuals.

TDD obligation: TypeScript adapter/model tests first.

### `AL-007-S10-AC03` Exact And Smooth Layers

Given Debug Visualization Mode is enabled,
When the user switches Resource/Field display between `Exact` and `Smooth`,
Then the Viewer labels the interpolation mode, shows a legend, keeps tooltips/inspector values tied to actual sampled data, and marks missing layer grids as unavailable.

TDD obligation: render-plan/unit tests for layer state before drawing changes.

### `AL-007-S10-AC04` Debug Overlays

Given debug projection fields are present or absent,
When the user toggles debug overlays,
Then grid coordinates, cell/entity bounds, projection ids, frame timing, missing projection warnings, and supported debug overlays are visible only when data-bound; unsupported overlays remain disabled with explicit reason.

TDD obligation: component tests for enabled/disabled overlay state and visible provenance.

### `AL-007-S10-AC05` Map-First UX Patch

Given the Monitor opens at 1024x768 and 1920x1080,
When a run is live or a Cell is selected,
Then the Viewer remains the dominant central surface, persistent large overlays do not cover the canvas, map tools are compact icon-style controls, Full Screen and PNG live in the Viewer toolbar, theme moves under Settings, and data context/run controls do not become competing dashboard panels.

TDD obligation: component/e2e visual layout assertions before CSS/component changes.

### `AL-007-S10-AC06` Start Regression Safety

Given the existing Start/Monitor flow,
When Debug Visualization Mode is off,
Then Start behavior, live/frozen context, selection, run controls, screenshot export, FPS/sim-rate display, and unavailable workspace presentation continue to work.

TDD obligation: run existing Start tests and selected Playwright tests after each UI-green step.

## Proposed File Plan

Rust:

- Modify `src/viewer_server/api/mod.rs` to register the projection route.
- Create or modify `src/viewer_server/api/projections.rs` for read-only latest projection response.
- Modify `src/viewer_server/state.rs` only if state needs to cache the latest typed projection payload alongside the binary frame.
- Modify `src/runner/projections.rs` or `src/observer/projection.rs` only if an adapter is needed to bundle existing payloads; do not change Core behavior.
- Add `tests/runner_http_projections.rs` or equivalent viewer-server route test.

UI model/client:

- Extend `ui/control-center/src/projection/types.ts` with Observer projection payload types.
- Add `ui/control-center/src/projection/debugProjectionAdapter.ts` and tests.
- Add or extend `ui/control-center/src/runner/apiClient.ts` for the read-only projection request.
- Extend `ui/control-center/src/app/appState.ts` and `monitorViewModel.ts` with debug projection state.

UI rendering:

- Add `ui/control-center/src/viewer/debugLayers.ts` and tests for exact/smooth/legend/availability render planning.
- Extend `ui/control-center/src/viewer/worldRenderPlan.ts` and `worldRenderer.ts` for resource/field layers and debug overlays.
- Modify `ui/control-center/src/components/WorldViewer.tsx` for compact map toolbar, debug layer controls, legend, and tooltip/value affordances.
- Modify `ui/control-center/src/components/MonitorWorkspace.tsx`, `LayerPanel.tsx`, and `CellInspector.tsx` only to reduce duplication and route debug/selection context to the right surface.
- Modify `ui/control-center/src/components/AppShell.tsx` and `uiText.ts` for Settings/theme placement and stable labels.
- Modify `ui/control-center/src/styles/layout.css` and `ui/control-center/src/styles/components.css` for map-first layout.
- Extend `ui/control-center/tests/e2e/monitor.spec.ts` and `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` for layout regression coverage.

Delivery docs:

- Update `docs/delivery/status.md` during execution start/closure.
- Update `docs/delivery/roadmap.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, and `outputs/worklogs/index.md` during closure.
- Create a `REPORT` worklog after verification.

## Numbered TDD Tasks

### `AL-007-S10-T01`: RED for `AL-007-S10-AC01`

- [ ] Add a failing Rust route/state test proving latest Observer debug projections are exposed by a read-only viewer-server endpoint.
- [ ] The test must assert schema/projection kind/completeness/provenance fields and that the route is unavailable/empty before an active frame exists.
- [ ] Run `cargo test --test runner_http_projections`.
- [ ] Capture expected failure as `AL-007-S10-EV01`.

### `AL-007-S10-T02`: GREEN for `AL-007-S10-AC01`

- [ ] Implement the minimal read-only projection endpoint and state wiring.
- [ ] Reuse existing `AL-004-S05` payload builders; do not duplicate payload semantics in viewer-server.
- [ ] Run `cargo test --test runner_http_projections --test observer_projection_payloads`.
- [ ] Capture pass as `AL-007-S10-EV02`.

### `AL-007-S10-T03`: RED for `AL-007-S10-AC02`

- [ ] Add failing UI tests for parsing/normalizing `VisualWorldProjection`, `CoverageProjection`, `WarningProjection`, `ClassificationProjection`, and `BalanceFindingProjection` payload states.
- [ ] Tests must include partial Cell material/internal-resource fields and unavailable projection payloads.
- [ ] Run `npm test -- debugProjectionAdapter`.
- [ ] Capture expected failure as `AL-007-S10-EV03`.

### `AL-007-S10-T04`: GREEN for `AL-007-S10-AC02`

- [ ] Implement typed UI projection models and the read-only API client call.
- [ ] Merge debug projection state into the app store without replacing `WorldFrame`.
- [ ] Run `npm test -- debugProjectionAdapter apiClient appState`.
- [ ] Capture pass as `AL-007-S10-EV04`.

### `AL-007-S10-T05`: RED for `AL-007-S10-AC03`

- [ ] Add failing viewer render-plan tests for exact resource/field layer state, smooth interpolation labels, legend metadata, sampled tooltip values, and unavailable layer state.
- [ ] Run `npm test -- debugLayers worldRenderPlan`.
- [ ] Capture expected failure as `AL-007-S10-EV05`.

### `AL-007-S10-T06`: GREEN for `AL-007-S10-AC03`

- [ ] Implement exact/smooth resource and field render planning.
- [ ] Render only values present in projection payloads; show explicit partial/unavailable labels for missing grids.
- [ ] Run `npm test -- debugLayers worldRenderPlan worldRenderer`.
- [ ] Capture pass as `AL-007-S10-EV06`.

### `AL-007-S10-T07`: RED for `AL-007-S10-AC04`

- [ ] Add failing component tests for Debug Visualization Mode controls: active mode label, enabled data-bound overlays, disabled unsupported overlays with reason, projection ids, frame timing, and missing projection warnings.
- [ ] Run `npm test -- MonitorWorkspace WorldViewer`.
- [ ] Capture expected failure as `AL-007-S10-EV07`.

### `AL-007-S10-T08`: GREEN for `AL-007-S10-AC04`

- [ ] Implement compact debug layer controls and map legend/tooltip surfaces.
- [ ] Keep unsupported overlays visible but disabled when they are canonical future capabilities without live payloads.
- [ ] Run `npm test -- MonitorWorkspace WorldViewer ViewerTruthOverlay`.
- [ ] Capture pass as `AL-007-S10-EV08`.

### `AL-007-S10-T09`: RED for `AL-007-S10-AC05`

- [ ] Add failing component/e2e layout assertions for map-first Monitor: Viewer bbox dominates center area, selected focus card does not persistently cover canvas, top context is compact, map toolbar contains Full Screen and PNG, and theme is no longer a top-level competing action.
- [ ] Include 1024x768 and 1920x1080 Playwright checks.
- [ ] Run `npm run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts`.
- [ ] Capture expected failure as `AL-007-S10-EV09`.

### `AL-007-S10-T10`: GREEN for `AL-007-S10-AC05`

- [ ] Refactor Monitor layout to prioritize the central Viewer.
- [ ] Move theme into Settings or a settings popover; keep future workspaces disabled and visible.
- [ ] Convert Viewer tools to compact icon-style controls with accessible labels/tooltips.
- [ ] Move persistent selected summary out of the canvas overlay; use Inspector or a compact non-overlapping selection strip.
- [ ] Run `npm test` and `npm run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts`.
- [ ] Capture pass/screenshots as `AL-007-S10-EV10`.

### `AL-007-S10-T11`: REFACTOR for `AL-007-S10`

- [ ] Remove duplicated projection-state display paths and keep source/completeness text centralized.
- [ ] Keep renderer helpers focused; do not turn `WorldViewer.tsx` into the projection business-logic owner.
- [ ] Run `npm test` after refactor.
- [ ] Capture pass as `AL-007-S10-EV11`.

### `AL-007-S10-T12`: Full Verification And Report

- [ ] Run `cargo test --test runner_http_projections --test observer_projection_payloads`.
- [ ] Run `npm test`.
- [ ] Run `npm run build`.
- [ ] Run `npm run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts`.
- [ ] Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s10-debug-visualization-mode-exact-layers.md`.
- [ ] Update roadmap/status/acceptance/ledger/index and review `Candidate Next Work`.
- [ ] Capture final evidence as `AL-007-S10-EV12`.

## Verification Commands

```powershell
cargo test --test runner_http_projections --test observer_projection_payloads
```

```powershell
Set-Location ui/control-center
npm test
```

```powershell
Set-Location ui/control-center
npm run build
```

```powershell
Set-Location ui/control-center
npm run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts
```

## Acceptance Gate

`AL-007-S10` can be reported as complete only when:

- UI shows all currently created `AL-004-S05` projection payload categories that are available through the read-only projection gateway.
- Missing/partial projection data is explicit and never presented as zero or complete.
- Debug exact/smooth layer mode is visible and auditable.
- Monitor layout is map-first at 1024x768 and 1920x1080.
- Start behavior remains intact when Debug Visualization Mode is off.
- Closure report contains command outputs and screenshot/e2e evidence.

## Open Questions

- `Needs Review`: exact endpoint path and response envelope naming should be finalized during the first RED/GREEN task, after checking existing route naming conventions.
- `Needs Review`: whether coverage/warning/classification/balance payloads should be fetched on demand or bundled into one latest-debug response. Default recommendation: one bounded latest-debug response for this slice, because it minimizes UI request churn and keeps AL-006 performance work separate.

## Approval Gate

Reply `OK EXECUTE AL-007-S10` to authorize execution of this TDD plan.

Reply `CHANGE AL-007-S10` with corrections to revise the plan.
