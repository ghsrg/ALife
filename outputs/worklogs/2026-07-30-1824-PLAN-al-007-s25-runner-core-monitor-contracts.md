# AL-007-S25 Runner And Core Monitor Contracts TDD Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use `superpowers:executing-plans` to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking. Do not use subagents unless the human explicitly asks for delegated execution.

**Goal:** Populate every Monitor Data Panel diagram with source-backed Runner/Core/Observer data, while keeping unavailable states explicit where a real contract still does not exist.

**Architecture:** Add typed Observer projection payloads for Monitor accounting, classification, organism, lineage, genome, and selected metric summaries; expose them through the Runner projection bundle; adapt them into UI view models and the existing UI RRD metric buffer. The UI remains a read-only consumer: it may aggregate displayed projection data and compact it into RRD samples, but it must not invent simulation truth.

**Tech Stack:** Rust Core/Observer/Viewer Server, React, TypeScript, Vite, Vitest, Testing Library, Playwright.

---

## Plan Metadata

| Field | Value |
| --- | --- |
| Plan ID | `AL-007-S25` |
| Slice title | Runner And Core Monitor Contracts |
| Status | TDD plan proposal |
| Created | 2026-07-30 |
| Depends on | `AL-007-S24`, `AL-004-S05` |
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
9. `docs/observer/classification-contract.md`
10. `docs/observer/classification-registry.md`
11. `docs/observer/behavior-profile-balance.md`
12. `docs/implementation/implementation-plan-ui.md`
13. Current implementation files listed below.

## Delivery Control Result

**Route:** `TDD_PLAN_REQUEST` through `delivery-control` -> `roadmap-control` with `test-driven-development` obligations.

## LINT_RESULT

**Scope:** `AL-007-S25`, `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/source-map.md`  
**Mode:** deterministic

| Severity | Rule | ID/Path | Finding | Required action |
| --- | --- | --- | --- | --- |
| WARN | `DL003` | `AL-007-S25-AC01..AC07` | Acceptance rows for the selected slice did not exist before planning. | Add proposed S25 acceptance rows mapped to this plan and expected implementation evidence. |
| WARN | `DL008` | `AL-007-S25` | Roadmap title is broad (`Runner And Core Monitor Contracts`), while the clarified requirement is Data Panel data for all diagrams. | Treat Data Panel contracts as the executable scope; do not expand into unrelated Runner/Core feature work. |

**Decision:** `PASS_WITH_WARNINGS`

## Current Implementation Findings

- `ui/control-center/src/app/monitorSurfaceModel.ts` currently has source-backed lifecycle and radius basics, but keeps role, organism, lineage, evolution, analytics, Energy Flow, Material Cycle, and accounting time chart unavailable.
- `ui/control-center/src/projection/types.ts` has `DebugClassificationProjection` and `DebugBalanceFindingProjection` as `unknown[]`; UI cannot safely render them as chart data.
- `src/viewer_server/api/projections.rs` returns live `visual_world`, coverage, warnings, empty classifications, and empty balance findings. It does not expose typed accounting, behavior profile, organism, lineage, genome, or monitor metric payloads.
- `src/observer/projection.rs` already exposes useful raw metrics from `MetricsSummary`, including resource decay, reaction amounts, heat, material degradation, boundary leakage, joint metrics, and integrated matter values. They are not yet packaged for final Monitor diagrams.
- `docs/ui/control-center-block.md` requires Data Panel content by Level:
  - World: Population Lifecycle; selected Matter Cycle/Energy Flow; time evolution.
  - Cells: observed primary roles with potential markers; Cell radius distribution.
  - Organisms: primary observed Behavior Profiles; Cell-count size bins.
  - Lineages: current population, history, genealogy, spatial footprint.
  - Evolution: Genome provenance, mutation history, diversity, carrier history.
  - Analytics: selected metric with complete provenance.
- The resource “evaporation” concern must be handled as visible accounting: explicit decay/sink/metabolism/material conversion are allowed only when surfaced; unclassified loss must be visible and testable.

## Assumptions

- This slice may add Observer/Runner projection contracts and UI consumers. It must not change simulation mechanics or rebalance scenarios.
- “All diagrams” means all required Data Panel cards for every Level, not every future Research workspace.
- If Core lacks a required raw value, implementation must expose an unavailable card with exact missing source, not infer it in UI.
- Energy Flow remains separate from Resource/Material matter accounting. UI must not estimate Energy from `resource.energy_value`.
- Resource disappearance is acceptable only if it is accounted as explicit decay/sink/metabolism/material conversion; `unclassified_loss` must be displayed as a warning/diagnostic input.
- RRD history remains UI-owned compact history fed from source-backed projections; it is not a full frame store.
- Single Cell/Organism selection stays Inspector/Focus detail. Data Panel distribution narrowing applies only to compatible multi-selection or World block scope.

## Open Questions

No blocker for planning. During execution, if an exact Core source for a chart segment is missing, implement the typed unavailable state first and stop before inventing a substitute.

## Forbidden Scope

- Do not change Core lifecycle, resource, reaction, genome, joint, or organism mechanics.
- Do not tune `living_ecosystem` resource decay/sink values in this slice.
- Do not add UI heuristic classifications from energy/radius/material amounts unless they come from typed Observer classification payloads with provenance.
- Do not merge Resource, Material, and Energy into one synthetic accounting chart.
- Do not add persistent storage replay, full keyframe loading, or long-run analytics exports.
- Do not add new charting dependencies unless existing SVG/CSS cannot pass acceptance.
- Do not mark `AL-007-S25` done without closure verification.

## File Map

### Create

- `src/observer/monitor_payloads.rs`  
  Typed Rust payload structs for Monitor Data Panel cards: accounting cycle/time inputs, role distributions, organism size/behavior distributions, lineage summaries, genome summaries, analytics metric descriptors, and unavailable source descriptors.

- `tests/observer_monitor_payloads.rs`  
  Rust contract tests for payload accounting, classification mapping, completeness, and no hidden resource loss.

- `tests/runner_monitor_projections.rs`  
  HTTP projection bundle tests proving `/projections/latest` exposes Monitor payloads with stable schema keys and source/completeness.

- `ui/control-center/src/projection/monitorProjectionAdapter.ts`  
  Parses Runner Monitor payloads into typed UI data with validation and unavailable states.

- `ui/control-center/src/projection/monitorProjectionAdapter.test.ts`  
  Unit tests for payload parsing, missing fields, source labels, and no heuristic fallback.

- `ui/control-center/src/app/monitorAccountingModel.ts`  
  Builds World Resource/Material/Energy cycle cards and accounting time inputs from typed Monitor projections plus UI RRD.

- `ui/control-center/src/app/monitorLevelDataModel.ts`  
  Builds Cells, Organisms, Lineages, Evolution, and Analytics card data from typed Monitor projections.

- `ui/control-center/src/app/monitorLevelDataModel.test.ts`  
  Unit tests for every Level card data contract.

### Modify

- `src/observer/mod.rs`  
  Export Monitor payload builders.

- `src/observer/projection.rs`  
  Build Monitor payloads from `CommittedSnapshot`, `MetricsSummary`, classification results, lineage summaries, and genome summaries where available.

- `src/viewer_server/api/projections.rs`  
  Add `monitor` section to `ControlCenterProjectionBundle/v1` without breaking existing `visual_world` consumers.

- `src/observer/payloads.rs`  
  Reuse or route shared payload types if existing module ownership makes a separate `monitor_payloads.rs` too fragmented.

- `ui/control-center/src/projection/types.ts`  
  Add typed Monitor projection interfaces; replace `unknown[]` usage only where a source-backed schema exists.

- `ui/control-center/src/projection/debugProjectionAdapter.ts`  
  Preserve current debug payload behavior and attach optional Monitor payload state.

- `ui/control-center/src/app/appState.ts`  
  Store latest Monitor projection and record source-backed RRD metric samples.

- `ui/control-center/src/app/rrdMetricHistory.ts`  
  Extend sample shape only if required for multi-series chart metadata; preserve S24 retention rules.

- `ui/control-center/src/app/monitorSurfaceModel.ts`  
  Consume `monitorAccountingModel` and `monitorLevelDataModel`; keep unavailable cards when required payloads are absent.

- `ui/control-center/src/components/BottomDataPanel.tsx`  
  Render real chart bodies for source-backed cards: lifecycle stacked bar, cycle share, stacked time series, role bars with potential markers, histograms, genealogy/footprint summaries, and analytics metric card.

- `ui/control-center/src/components/BottomDataPanel.test.tsx`  
  Component acceptance tests for all Level diagrams and unavailable states.

- `ui/control-center/tests/e2e/monitor.spec.ts`  
  Smoke test live `living_ecosystem`/fixture Monitor shows source-backed Data Panel cards without overlap at supported viewport.

- `docs/delivery/acceptance.md`  
  Add `AL-007-S25-AC01..AC07`.

- `docs/delivery/status.md`  
  After human approval, optionally move `AL-007-S25` from `ready-to-plan` to `planned-ready`; execution moves it to `in-progress`.

## Agent Scenario Cards

### AL-007-S25-AC01: Monitor projection bundle exposes typed Data Panel payloads

**Source links:** `docs/observer/projection-contract.md`, `docs/ui/control-center-block.md`, `docs/delivery/roadmap.md`  
**Intent:** Runner gives UI one typed read-only source for every Data Panel card.  
**Priority:** P0  
**Independent verification:** Rust projection unit tests and HTTP projection tests.

**Given** a live committed snapshot and metrics summary  
**When** `/projections/latest` returns `ControlCenterProjectionBundle/v1`  
**Then** it includes a `monitor` section with schema version, tick, source, completeness, and typed subsections for World, Cells, Organisms, Lineages, Evolution, and Analytics, with explicit unavailable descriptors for missing contracts.

**TDD obligation:** RED test must fail because `monitor` is absent before adding any payload builder.

**Evidence:** `AL-007-S25-EV01`, `AL-007-S25-EV02`

### AL-007-S25-AC02: World accounting diagrams are source-backed and conservation-aware

**Source links:** `docs/PRINCIPLES.md`, `docs/ui/control-center-block.md`, `docs/world/reactions.md`, `docs/observer/projection-contract.md`  
**Intent:** Resource/Material/Energy charts show where matter/energy went and expose explicit sinks/unclassified loss.  
**Priority:** P0  
**Independent verification:** Rust accounting tests plus UI model tests.

**Given** a run where resources diffuse, decay, enter Cells, become Materials/fragments, feed Energy Buffer, generate Heat, or go to explicit sinks  
**When** World Data Panel renders `Resource`, `Material`, or `Energy` target  
**Then** the cycle card shows source-backed location shares and absolute totals, the time chart uses RRD samples, explicit sink/decay is visible, and any `unclassified_loss` is shown as a warning/provenance row instead of disappearing silently.

**TDD obligation:** RED tests must catch a resource total drop without an explicit sink/decay/unclassified-loss field.

**Evidence:** `AL-007-S25-EV03`, `AL-007-S25-EV04`

### AL-007-S25-AC03: UI RRD feeds all Data Panel time diagrams from source-backed samples

**Source links:** `docs/ui/control-center-block.md`, `ui/control-center/src/app/rrdMetricHistory.ts`  
**Intent:** Every time chart uses the existing RRD principle and does not slice full frames.  
**Priority:** P0  
**Independent verification:** Vitest RRD/model tests.

**Given** more than 1000 Monitor metric samples arrive from live projections  
**When** Data Panel records World accounting, lineage history, evolution history, and analytics metric samples  
**Then** the newest 100 samples remain consecutive, older tiers are 10x decimated, collapsed numeric intervals store mean values, and tooltips/axis metadata expose actual Tick intervals.

**TDD obligation:** RED tests must fail if code reads `frameHistory.slice(...)` for chart data or exceeds 1000 retained metric samples.

**Evidence:** `AL-007-S25-EV05`, `AL-007-S25-EV06`

### AL-007-S25-AC04: Cells and Organisms diagrams use Observer classification payloads

**Source links:** `docs/ui/control-center-block.md`, `docs/observer/classification-contract.md`, `docs/observer/behavior-profile-balance.md`  
**Intent:** Role and behavior charts are explainable Observer outputs, not UI heuristics.  
**Priority:** P0  
**Independent verification:** Rust classification payload tests and UI model/component tests.

**Given** classification and behavior profile projections contain labels, counts, confidence, classifier version, and evidence summaries  
**When** Cells or Organisms Level is active  
**Then** Cells shows observed primary roles as bars with potential-role markers, Organisms shows observed behavior profile distribution and organism-size bins, and all labels include source/completeness/classifier provenance.

**TDD obligation:** RED tests must fail if roles are inferred from Cell energy/radius/material amounts instead of typed classification payloads.

**Evidence:** `AL-007-S25-EV07`, `AL-007-S25-EV08`

### AL-007-S25-AC05: Lineages and Evolution diagrams use lineage/genome source data or explicit unavailable states

**Source links:** `docs/ui/control-center-block.md`, `docs/observer/projection-contract.md`, `docs/delivery/roadmap.md`  
**Intent:** Lineage and Evolution cards become useful where source data exists and truthful where it does not.  
**Priority:** P1  
**Independent verification:** Rust projection tests and UI model tests.

**Given** lineage event log and genome runtime summaries are available for the current run  
**When** Lineages or Evolution Level is active  
**Then** Data Panel shows current lineage population, history, compact genealogy, spatial footprint, genome provenance, mutation history, diversity, and carrier history from source-backed payloads; missing subsections remain unavailable with exact missing contract names.

**TDD obligation:** RED tests must prove unavailable states render for missing lineage/genome fields before adding any partial renderer.

**Evidence:** `AL-007-S25-EV09`, `AL-007-S25-EV10`

### AL-007-S25-AC06: Analytics card is selected-metric driven with full provenance

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/analytics.md`, `docs/observer/projection-contract.md`  
**Intent:** Analytics Level shows one selected metric with definition and provenance, not raw data.  
**Priority:** P1  
**Independent verification:** UI model/component tests.

**Given** a selected metric descriptor exists with definition, unit, aggregation, interval, sample count, completeness, and source/classifier version where applicable  
**When** Analytics Level renders  
**Then** Data Panel shows the metric value/trend and its complete provenance; if no metric is selected, it shows an unavailable prompt without fake defaults.

**TDD obligation:** RED tests must fail when selected metric metadata is missing but a chart still renders as available.

**Evidence:** `AL-007-S25-EV11`, `AL-007-S25-EV12`

### AL-007-S25-AC07: Final Data Panel remains compact and layout-safe

**Source links:** `docs/ui/control-center-block.md`, `docs/ui/control-center-design-spec.md`, `docs/delivery/acceptance.md`  
**Intent:** Adding real data must not regress S22/S23/S24 layout constraints.  
**Priority:** P0  
**Independent verification:** Playwright visual/layout smoke plus component tests.

**Given** Monitor runs at `1280x720` CSS viewport and `1920x1080` display-scale scenarios  
**When** Data Panel renders all supported Level diagrams  
**Then** Map remains dominant, Data Panel cards stay compact, no Data Panel-local scrollbar appears, root/page scroll owns overflow below supported height, and unavailable/provenance chips remain secondary.

**TDD obligation:** RED e2e/component tests must capture card count/height/scroll regressions before chart renderers are expanded.

**Evidence:** `AL-007-S25-EV13`, `AL-007-S25-EV14`

## Numbered TDD Tasks

### AL-007-S25-T01: RED for `AL-007-S25-AC01`

- [ ] Add `tests/runner_monitor_projections.rs` expecting `/projections/latest` to include `monitor.schema_version = "MonitorDataPanelProjection/v1"` and Level subsections.
- [ ] Run `cargo test runner_monitor_projections -- --nocapture`.
- [ ] Expected RED: response has no `monitor` section.
- [ ] Capture as `AL-007-S25-EV01`.

### AL-007-S25-T02: GREEN for `AL-007-S25-AC01`

- [ ] Create minimal `src/observer/monitor_payloads.rs` with typed unavailable-first payload structs.
- [ ] Wire `src/viewer_server/api/projections.rs` to include `monitor`.
- [ ] Run `cargo test runner_monitor_projections -- --nocapture`.
- [ ] Expected GREEN: `monitor` exists with schema/source/completeness and unavailable descriptors.
- [ ] Capture as `AL-007-S25-EV02`.

### AL-007-S25-T03: RED for `AL-007-S25-AC02`

- [ ] Add `tests/observer_monitor_payloads.rs` for World accounting payload with resource/material/energy sections.
- [ ] Include a fixture where resource amount drops; assert the payload reports explicit `decay`, `sink`, `metabolism`, `material_conversion`, or `unclassified_loss`.
- [ ] Run `cargo test observer_monitor_payloads -- --nocapture`.
- [ ] Expected RED: accounting payload fields are absent.
- [ ] Capture as `AL-007-S25-EV03`.

### AL-007-S25-T04: GREEN for `AL-007-S25-AC02`

- [ ] Build World accounting payload from `CommittedSnapshot` and `MetricsSummary`.
- [ ] Keep Energy Flow unavailable until produced/stored/spent/heat/loss fields are source-backed enough; expose exact unavailable reason if incomplete.
- [ ] Run `cargo test observer_monitor_payloads -- --nocapture`.
- [ ] Capture as `AL-007-S25-EV04`.

### AL-007-S25-T05: RED for `AL-007-S25-AC03`

- [ ] Add Vitest coverage proving Monitor time charts consume RRD metric samples, not `frameHistory.slice(...)`.
- [ ] Add RRD test for multi-series World accounting samples with tick interval metadata.
- [ ] Run `npm.cmd test -- src/app/rrdMetricHistory.test.ts src/app/monitorAccountingModel.test.ts --run`.
- [ ] Expected RED: model/tests do not exist or chart source is unavailable.
- [ ] Capture as `AL-007-S25-EV05`.

### AL-007-S25-T06: GREEN for `AL-007-S25-AC03`

- [ ] Extend UI RRD sample metadata only as needed for Data Panel multi-series charts.
- [ ] Feed RRD from Monitor projection adapter in `appState`.
- [ ] Run the same Vitest command.
- [ ] Capture as `AL-007-S25-EV06`.

### AL-007-S25-T07: RED for `AL-007-S25-AC04`

- [ ] Add Rust tests for classification/behavior payload rows with observed counts, potential markers, confidence, classifier version, and evidence.
- [ ] Add UI model tests that reject energy/radius/material role heuristics.
- [ ] Run targeted Rust and Vitest commands.
- [ ] Expected RED: classification rows are empty/unknown and UI returns unavailable.
- [ ] Capture as `AL-007-S25-EV07`.

### AL-007-S25-T08: GREEN for `AL-007-S25-AC04`

- [ ] Type classification payloads in Rust and TypeScript.
- [ ] Map Cells observed roles/potential markers and Organism behavior/size bins into Data Panel cards.
- [ ] Preserve unavailable state if source classification is absent.
- [ ] Run targeted Rust and Vitest commands.
- [ ] Capture as `AL-007-S25-EV08`.

### AL-007-S25-T09: RED for `AL-007-S25-AC05`

- [ ] Add tests for lineage/genome payload availability and missing-field unavailable states.
- [ ] Run `cargo test observer_monitor_payloads -- --nocapture` and relevant Vitest model tests.
- [ ] Expected RED: Lineage/Evolution cards remain unavailable with broad reasons.
- [ ] Capture as `AL-007-S25-EV09`.

### AL-007-S25-T10: GREEN for `AL-007-S25-AC05`

- [ ] Expose available lineage event/log summary and genome runtime summary data through Monitor payloads.
- [ ] Render source-backed Lineages/Evolution cards where available and exact unavailable cards where incomplete.
- [ ] Run targeted Rust and Vitest commands.
- [ ] Capture as `AL-007-S25-EV10`.

### AL-007-S25-T11: RED for `AL-007-S25-AC06`

- [ ] Add UI tests for selected Analytics metric descriptor, value/trend, and required provenance fields.
- [ ] Run `npm.cmd test -- src/app/monitorLevelDataModel.test.ts src/components/BottomDataPanel.test.tsx --run`.
- [ ] Expected RED: selected metric model does not exist or is unavailable.
- [ ] Capture as `AL-007-S25-EV11`.

### AL-007-S25-T12: GREEN for `AL-007-S25-AC06`

- [ ] Implement selected metric model using typed Monitor projection or explicit unavailable descriptor.
- [ ] Render Analytics card with definition, unit, aggregation, interval, sampling/completeness, and source/classifier version.
- [ ] Run the same Vitest command.
- [ ] Capture as `AL-007-S25-EV12`.

### AL-007-S25-T13: RED for `AL-007-S25-AC07`

- [ ] Add Playwright checks for Data Panel compactness at supported viewport, no local Data Panel scrollbar, and Map dominance after switching all Levels.
- [ ] Run `npx playwright test tests/e2e/monitor.spec.ts --project=chromium`.
- [ ] Expected RED: new all-level checks fail before final chart renderers/layout polish.
- [ ] Capture as `AL-007-S25-EV13`.

### AL-007-S25-T14: GREEN/REFACTOR for `AL-007-S25-AC07`

- [ ] Polish chart renderers in `BottomDataPanel.tsx` and CSS without changing payload semantics.
- [ ] Preserve S22/S23/S24 invariants: fixed tracks, root overflow, no fake values, layer presentation-only behavior.
- [ ] Run targeted Vitest, production build, selected Playwright, and targeted Rust tests.
- [ ] Capture as `AL-007-S25-EV14`.

### AL-007-S25-T15: Docs/status/report preparation

- [ ] Update `docs/delivery/acceptance.md` evidence links if file names differ from this plan.
- [ ] Create `outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s25-runner-core-monitor-contracts.md` after verification.
- [ ] Do not mark roadmap/status done until `closure-verification` confirms evidence coverage.

## Verification Commands

```powershell
cargo test observer_monitor_payloads -- --nocapture
cargo test runner_monitor_projections -- --nocapture
npm.cmd test -- src/projection/monitorProjectionAdapter.test.ts src/app/monitorAccountingModel.test.ts src/app/monitorLevelDataModel.test.ts src/components/BottomDataPanel.test.tsx --run
npm.cmd run build
npx.cmd playwright test tests/e2e/monitor.spec.ts --project=chromium
```

If Windows locks `target/debug/runner.exe`, record the blocked command and process state; do not silently skip Rust verification.

## Approval Gate

Reply `OK EXECUTE AL-007-S25` to authorize execution of this TDD plan.

Reply `CHANGE AL-007-S25` with corrections to revise the plan.
