---
tags:
  - alife
  - delivery/status
---

# Delivery Status

Operational dashboard for active delivery-control work. This file is not the
canonical roadmap and must not introduce new requirements. Canonical status,
confidence, dependencies, and scope live in `docs/delivery/roadmap.md`.

## Current Focus

| Plan ID | Status | Confidence | Source | Notes |
| --- | --- | --- | --- | --- |
| `AL-006-S03` | planned | high | `docs/delivery/roadmap.md` | Deterministic Parallelism: parallel execution preserving stable state hashes. |

## Operational Rules

- `Current Focus` is the single selected Plan ID for planning or execution.
- `planned` in `Current Focus` means a plan exists or is being reviewed, but
  implementation has not started.
- `in-progress` in `Current Focus` means execution is authorized and underway.
- `Next` mirrors selected `Candidate Next Work` rows with operational statuses.
- `planned-ready` means a concrete TDD plan exists, but the Plan ID is not the
  current focus and execution is not authorized.
- `ready-for-closure` means the next action is evidence review and closure
  verification, not implementation planning.
- `ready-to-plan` means dependencies are clear enough to draft a TDD plan.
- `blocked-dependency` means the row is not an incident; it is intentionally
  waiting for named prerequisite Plan IDs or decisions.
- `Dependency state` lists only active blockers or active dependencies; closed
  dependencies are omitted to avoid duplicating the roadmap.
- Non-next legacy rows in `docs/delivery/acceptance.md` stay untouched until
  their Plan ID becomes current or Candidate Next Work.

## Next

| Order | Plan ID | Operational status | Dependency state | Source | Notes |
| --- | --- | --- | --- | --- | --- |
| 1 | `AL-006-S02` | <kbd style="background-color: #ffd33d; color: #24292f; border: none;">ready-to-plan</kbd> | none | `docs/delivery/roadmap.md` | Hot Path Optimization And Dirty Regions: SoA, dirty region tracking, spatial index optimization. |
| 2 | `AL-005-S03` | <kbd style="background-color: #ffd33d; color: #24292f; border: none;">ready-to-plan</kbd> | none | `docs/delivery/roadmap.md` | Long-Run Evolution Scenario Suite: Replayable long-run evolution scenarios. |

## Recently Closed

| Plan ID | Status | Evidence | Notes |
| --- | --- | --- | --- |
| `AL-007-S22` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-29-1147-REPORT-al-007-s22-monitor-layout-stabilization.md` | Monitor Layout Stabilization closed for fixed reference tracks, no Monitor/Data tabs, root scroll below `1366x862`, stable Map geometry, and overlay click regression coverage. Complete Map-only fullscreen shell remains routed to `AL-007-S23`. |
| `AL-007-S14` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-25-1555-REPORT-al-007-s14-debug-diagnostics-recovery-and-projection-cadence.md` | Debug Experiments, Diagnostics, Recovery, And Projection Cadence Optimization closed for adaptive grid stride streaming, lightweight HTTP debug payloads, and smooth real-time projection updates. |
| `AL-007-S13` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-25-1215-REPORT-al-007-s13-world-editor-and-scenario-runner.md` | World Editor And Scenario Runner closed for pre-run TOML config editor, scenario presets, validation diagnostics, SHA-256 config hashing, localStorage draft recovery, relaunch controls, and read-only execution safety. |
| `AL-004-S04` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-25-1150-REPORT-al-004-s04-organism-view-projection.md` | Observer-Only OrganismView Projection closed for OrganismViewPayload, OrganismViewProjection, graph connected-component BFS analysis, centroid, and read-only Observer boundary. |
| `AL-002-S18` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-25-1140-REPORT-al-002-s18-diverse-rich-world-and-closure.md` | Core-Bootstrap-Runner Closure Matrix And Diverse World Setup closed for diverse_rich_world.toml, patchy resource oases, specialized genome profiles, seed-driven diversity, and AL-002 block closure. |
| `AL-005-S02` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-25-1125-REPORT-al-005-s02-analytics-export-foundation.md` | Analytics Export Foundation closed for AnalyticsExportManifest, Population, Balance, Lineage, Environment datasets, JSON/CSV formatting, and export_to_dir verification. |
| `AL-006-S01` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-24-2305-REPORT-al-006-s01-benchmark-harness-target-scenarios.md` | Benchmark Harness And Target Scenarios closed for scale_20k_cells.toml, scale_40k_joints.toml, scale_scenarios_smoke, scale_benchmark_harness, and determinism verification. |
| `AL-002-S17` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-24-1925-REPORT-al-002-s17-phase-2-debt-closure.md` | Phase 2 Debt Closure closed JointRepair canonical disposition, RepairBoundary & material damage execution, boundary retention, and observer environment heat metrics. |
| `AL-007-S13` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-24-1238-REPORT-v3-ui-charts-docking-and-ecosystem-balance.md` | V3 Control Center layout alignment, charts docking under Map Viewer, live metric streams, and living ecosystem balance tuning. |
| `AL-007-S13` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-24-1115-REPORT-living-ecosystem-scenario-finalization.md` | Unified production scenario `living_ecosystem.toml` created with 4 resource layers, environmental fields, balanced cell energy, genome execution, and dynamic UI streaming. |
| `AL-007-S12` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-23-2150-REPORT-al-007-s12-balance-analytics.md` | UI-2D is closed for Matter Cycle accounting, Unaccounted difference badge, Energy utilization progress, Population lifecycle breakdown, Engineering Warnings list, searchable Raw Data grid, CSV export, and V3 Monitor tabs. |
| `AL-007-S11` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-23-1235-REPORT-al-007-s11-inspectors-search-filters-comparison.md` | UI-2C is closed for compact source-backed Cell inspector sections, debug projection loading/stale states, bounded resource legend, map search/filter over available Cell/resource/layer/field text, and local pinned Cell comparison. |
| `AL-002-S12` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-2112-REPORT-al-002-s12-bootstrap-preview-reports-calibration.md` | Bootstrap-4 preview/report/calibration is closed for shared preview API, bounded resource preview, stable manifest JSON, deterministic compact seed sweep, CLI `--bootstrap-preview`, and explicit manifest-only field warnings. |
| `AL-007-S21` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-2035-REPORT-al-007-s21-rich-world-visibility-repair.md` | Rich world visibility repair is closed: `demo_world_resource`, exact resource layer payload cells, per-Cell energy/material/internal/local resource details, multi-layer map overlay merge, Cell Inspector details, and map-first layout verification. Bootstrap fields remain manifest-only until Core has spatial field grids. |
| `AL-002-S11` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-1547-REPORT-al-002-s11-rich-spatial-generators-and-world-families.md` | Typed Bootstrap generator specs, deterministic prepared resource layers, `patchy_temperate_v1` manifest metadata, explicit field-grid warning, and rich Runner/Core smoke are closed. Full `cargo test` is blocked by local disk capacity during link, not by an assertion failure. |
| `AL-007-S10` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-1444-REPORT-al-007-s10-debug-visualization-mode-exact-layers.md` | Read-only Observer projection gateway, UI debug projection state, compact Debug Visualization Mode, disabled unsupported overlays, and map-first Monitor layout are closed. Exact per-cell material/resource grids and richer inspectors remain downstream. |
| `AL-004-S05` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-1331-REPORT-al-004-s05-visual-balance-coverage-warning-projections.md` | Typed Observer payload builders are closed for bounded visual world data, classification provenance, coverage statuses, warning dispositions, and balance findings. Per-Cell material/internal-resource payloads remain explicitly partial until Core/Observer exposes that snapshot data. |
| `AL-004-S03` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-1256-REPORT-al-004-s03-classification-registry-and-provenance.md` | Implemented early Observer classification baseline is closed for config loading, deterministic Cell role/behavior/archetype classifiers, mode/status/confidence/version/evidence/completeness fields, and read-only boundary. Concrete consumer classification payload/provenance is now covered by `AL-004-S05`; Research UI display remains downstream. |
| `AL-007-S09` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-1059-REPORT-al-007-s09-versioned-projections-keyframes-history.md` | UI-2A is closed for Data Context, projection source/version/completeness display, bounded client live history, frozen read-only inspection, stale context, Jump to Live, and explicit unavailable Tick/no-nearest-frame-substitution behavior. Full storage replay and exact debug layers remain later slices. |
| `AL-007-S20` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-22-0003-REPORT-al-007-s20-start-track-residual-visual-gap-disposition.md` | Start residual UI debt is closed for disabled workspace presentation, Start full-screen, visible simulation rate, visible Viewer FPS target, and explicit unavailable projection state without Core behavior changes. |
| `AL-005-S01` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-2320-REPORT-al-005-s01-run-metadata-and-storage-index.md` | Minimal file-backed SQLite run metadata/index is closed with run rows, artifact reference rows, explicit unavailable keyframes, file-delete test reset, and no Core storage authority. |
| `AL-002-S16` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-2221-REPORT-al-002-s16-runner-4-remote-viewer-acceptance-hardening.md` | Runner-4 remote viewer opt-in, CORS allowlist, stable HTTP errors, graceful shutdown state, reconnect latest-frame behavior, and status metadata are closed without Core mechanic changes. |
| `AL-004-S02` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-1312-REPORT-al-004-s02-versioned-projection-envelope.md` | Rust-only typed projection envelope, completeness/source vocabulary, Observer inventory mapping, and non-breaking `WorldFrameProjection v2` envelope wrapping are closed; storage/keyframes and generated schemas remain later slices. |
| `AL-003-S05` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-1159-REPORT-al-003-s05-lineage-event-log-and-replay.md` | Deterministic read-only lineage event log and replay summary are closed for founder Cells, Genome copying/mutation, division inheritance, death, and observer-boundary guards. |
| `AL-004-S01` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-21-1014-REPORT-al-004-s01-observer-contract-closure.md` | Static Observer contract inventory now covers current metric fields, coverage statuses, warning dispositions, and Runner live-frame field ownership without entering Core behavior. |
| `AL-003-S04` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1741-REPORT-al-003-s04-genome-copying-mutation-repair.md` | Material-backed Genome copying, deterministic bounded mutation, division copy gate, and conservative sweeper scenario are closed. |
| `AL-003-S03` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1513-REPORT-al-003-s03-scheduled-genome-runtime-cadence.md` | Scheduled Genome Runtime cadence, cached `ActionPlan` reuse, due-Tick trace emission, and no every-Tick recomputation are closed. |
| `AL-003-S02` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1244-REPORT-al-003-s02-genome-runtime-contract-output-coverage.md` | Genome Runtime contract, output disposition, deferred output rejection, capability masks, and minimal debug trace are closed. |
| `AL-001-S04` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-20-1154-REPORT-al-001-s04-delivery-control-baseline-refactor.md` | Delivery-control artifact responsibilities separated; current/next acceptance normalized to slice IDs. |
| `AL-007-S08` | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `outputs/worklogs/2026-07-19-1937-REPORT-al-007-ui-1d-start-demo-export-acceptance-hardening.md` | Start demo, screenshot export, and acceptance hardening are closed for the current UI-1D scope. |

## Control Drift Notes

| Area | Status | Notes |
| --- | --- | --- |
| Historical evidence | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | Historical worklog-derived roadmap rows live in `docs/delivery/worklog-ledger.md`; `docs/delivery/roadmap.md` keeps only a pointer. |
| Acceptance matrix | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | Current/next acceptance uses slice-level `AL-###-S##-AC##` IDs. Non-next legacy rows remain untouched until their Plan ID is selected. |
| Source map/control vocabulary | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | Delivery-control vocabulary distinguishes stage IDs (`AL-###`) from executable slice Plan IDs (`AL-###-S##`). |
| Stage status sync | <kbd style="background-color: #2ea44f; color: white; border: none;">done</kbd> | `AL-001` and `AL-003` stage statuses reflect closed slice rows with closure evidence. |
