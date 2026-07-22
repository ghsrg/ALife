---
tags:
  - alife
  - worklog/report
  - delivery/closure
  - ui
  - debug-visualization
plan_id: AL-007-S10
status: done
---

# REPORT: AL-007-S10 Debug Visualization Mode And Exact Layers

## Outcome

PASS for the bounded UI-2B debug visualization slice.

This closes the first UI consumer path for the `AL-004-S05` Observer projection payloads: Runner/Viewer Server exposes a read-only latest projection bundle, Control Center stores normalized debug projection state beside the ALIF v2 frame stream, and Monitor renders a compact data-bound Debug Visualization Mode with explicit partial/unavailable states.

The slice also repairs the current Monitor layout so the Viewer remains the central working surface. It does not claim full material-rich worlds, exact per-cell material breakdowns, spatial index/contact/process overlays, or analytics dashboards.

## Scope Checked

- Delivery plan: `outputs/worklogs/2026-07-22-1354-PLAN-al-007-s10-debug-visualization-mode-exact-layers.md`.
- Delivery docs: `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`.
- Canon/contract docs: `docs/observer/projection-contract.md`, `docs/ui/control-center-design-spec.md`, `docs/ui/visualization.md`, `docs/ui/architecture.md`, `docs/ui/quality.md`.
- Code/tests: `src/observer/payloads.rs`, `src/observer/projection.rs`, `src/viewer_server/api/mod.rs`, `src/viewer_server/api/projections.rs`, `tests/runner_http_projections.rs`, `tests/observer_projection_payloads.rs`, `ui/control-center/src/projection/*`, `ui/control-center/src/runner/apiClient.ts`, `ui/control-center/src/app/*`, `ui/control-center/src/components/MonitorWorkspace.tsx`, `ui/control-center/src/components/WorldViewer.tsx`, `ui/control-center/src/viewer/debugLayers.ts`, `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts`.
- Worklogs were used only as evidence, not as source of truth.

## Changed Files Summary

- Added `GET /projections/latest` as a read-only Runner/Viewer Server projection gateway.
- Added typed `DebugProjectionBundle` normalization in Control Center.
- Added UI store/client wiring so debug projections update from live frames without replacing ALIF v2 rendering.
- Added Debug Visualization Mode controls for exact/smooth mode, projection category visibility, unavailable overlays, projection tick/source, missing projection warnings, and sampled resource/field labels.
- Added a stale debug projection guard so an older async `/projections/latest` response cannot overwrite debug state for a newer live frame.
- Moved selected focus context out of the persistent canvas overlay and tightened Monitor layout so the Viewer dominates the central surface.
- Added Rust, TypeScript, component, and Playwright coverage for projection availability and map-first layout.

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task ID(s) | Evidence ID(s) | Test/Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-007-S10` | Read-only latest Observer debug projection gateway. | `AL-007-S10-AC01` | `AL-007-S10-T01`-`T02` | `AL-007-S10-EV01`, `AL-007-S10-EV02` | `tests/runner_http_projections.rs`; `tests/observer_projection_payloads.rs` | covered |
| `AL-007-S10` | UI model preserves available, partial, and unavailable projection state. | `AL-007-S10-AC02` | `AL-007-S10-T03`-`T04` | `AL-007-S10-EV03`, `AL-007-S10-EV04` | `ui/control-center/src/projection/debugProjectionAdapter.test.ts`; `apiClient.test.ts`; `appState.test.ts` | covered |
| `AL-007-S10` | Exact/smooth debug layer planning and legends stay data-bound. | `AL-007-S10-AC03` | `AL-007-S10-T05`-`T06` | `AL-007-S10-EV05`, `AL-007-S10-EV06` | `ui/control-center/src/viewer/debugLayers.test.ts`; `WorldViewer.test.tsx` | covered with explicit projection limits |
| `AL-007-S10` | Debug overlays expose supported data and disable unsupported overlays with reasons. | `AL-007-S10-AC04` | `AL-007-S10-T07`-`T08` | `AL-007-S10-EV07`, `AL-007-S10-EV08` | `WorldViewer.test.tsx`; `MonitorWorkspace.test.tsx`; `src/viewer/debugLayers.ts` | covered with unavailable overlays |
| `AL-007-S10` | Monitor is map-first at target viewports. | `AL-007-S10-AC05` | `AL-007-S10-T09`-`T10` | `AL-007-S10-EV09`, `AL-007-S10-EV10` | `ui/control-center/tests/e2e/monitor.spec.ts`; `ui/control-center/tests/e2e/ui-1c-a-visual.spec.ts` | covered |
| `AL-007-S10` | Start/Monitor behavior remains usable after debug UI is added. | `AL-007-S10-AC06` | `AL-007-S10-T10`-`T12` | `AL-007-S10-EV10`-`AL-007-S10-EV12` | full Vitest, production build, selected Playwright e2e | covered |

## Verification

```text
cargo fmt --check
PASS
```

```text
cargo test --test runner_http_projections --test observer_projection_payloads
PASS: observer_projection_payloads 6 passed; runner_http_projections 2 passed
```

```text
npm test
PASS: 31 test files passed; 153 tests passed
```

```text
npm run build
PASS: tsc -b and vite build completed
```

```text
npm run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts
PASS: 9 Playwright tests passed
```

## Disposition

- `AL-007-S10` can move to `done`.
- `AL-002-S11` remains the recommended next implementation slice if the priority is to make worlds visually richer with deterministic Bootstrap world families.
- `AL-007-S11` is now ready to plan as the direct UI follow-up for richer inspectors/search/filtering over the same projection/control model.

## Remaining Debt

- Exact per-cell material and internal-resource values are still unavailable because `CommittedSnapshot` does not expose those fields. UI keeps this partial instead of inventing material composition.
- Spatial index, contact/collision bounds, process/event overlays, and exact per-cell resource grids remain disabled/unavailable overlays until Core/Observer exposes source-backed payloads.
- Debug layer rendering is currently a compact projection audit/control surface over the existing Viewer; full heatmap/grid drawing and richer chart/raw-data surfaces remain downstream in `AL-007-S11` and `AL-007-S12`.
- Rich visual diversity still depends on `AL-002-S11` Bootstrap generators/world families.
