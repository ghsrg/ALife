---
plan_id: AL-007-S37
status: executed
date: 2026-08-07
---

# Canonical Resource Layers End-To-End Contract Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Make every configured canonical resource layer visible, named, selectable, and source-backed from `canonical_test_world.toml` through Core snapshot, Observer projection, Runner HTTP JSON, and Control Center UI.
**Architecture:** Preserve Core resource mechanics as typed layer-indexed simulation state; add read-only resource identity metadata only at snapshot/projection/API/UI boundaries; keep layer toggles presentation-only.
**Tech Stack:** Rust Core/Observer/Viewer Server, Axum JSON projection API, TypeScript React Control Center, Vitest.

---

## Classification

Request type: `TDD_PLAN_REQUEST` for `AL-007-S37`.

Do not implement this plan in the planning turn. Execution requires explicit approval:

```text
OK EXECUTE AL-007-S37
```

## Source Hierarchy Reviewed

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/mechanics/INDEX.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `config/scenarios/demo/canonical_test_world.toml`
- `src/core/snapshot.rs`
- `src/core/world.rs`
- `src/observer/payloads.rs`
- `src/observer/projection.rs`
- `src/viewer_server/api/projections.rs`
- `ui/control-center/src/projection/types.ts`
- `ui/control-center/src/projection/debugProjectionAdapter.ts`
- `ui/control-center/src/app/appState.ts`
- `ui/control-center/src/app/layerDisplayModel.ts`
- `ui/control-center/src/components/LayerPanel.tsx`
- `ui/control-center/src/viewer/debugLayers.ts`

## Current Problem

`canonical_test_world.toml` declares 19 resource ids:

```text
amino_acid, short_peptide, sugar, long_sugar, fatty_acid, phospholipid,
sterol, fiber, resin, mineral_salt, silicate, phosphate, carbon_fuel,
storage_fat, reactive_solvent, catalyst_mineral, metal_ion, inert_waste,
nucleotide_precursor
```

Core currently snapshots all numeric resource layers, but the layer identity is lost before UI display:

- `CommittedSnapshot.resource_layers` has `layer_index`, totals, grid cells, but no `resource_id`.
- `ResourceLayerSummaryPayload` has no `resource_id`.
- `/projections/latest` emits no layer names.
- Control Center types/adapters cannot display names.
- `resourceLayersToGrid()` preserves arbitrary `layers[layerIndex]`, but also projects legacy `organic/mineral/energy` channels by `layerIndex % 3`.
- `activeResourceLayers` defaults to `[0, 1]`, so a canonical run can appear as only a small legacy subset even when the payload has more layers.
- Debug resource legend labels use modulo color channel text rather than canonical resource names.

## Non-Goals

- Do not add new resource mechanics.
- Do not change `canonical_test_world.toml` resource definitions.
- Do not add vector/flow field runtime behavior.
- Do not make `light`, `radiation`, `flow`, `pressure`, or `chemical_gradient` execute direct behavior.
- Do not introduce species IDs, cell roles, organs, predators, scripted behavior, or material ownership shortcuts.
- Do not replace the existing live frame stream.
- Do not make UI infer biological meaning from resource names.

## Domain Model Constraints

- Core resource grid remains the authoritative numeric SoA-like layer store.
- Resource layer identity is metadata derived from runtime config, not behavior authority.
- `ResourceLayerIndex` remains the simulation-side addressing primitive.
- Projection payloads may clone stable resource ids because they are bounded read-only observer data, not Tick hot-path logic.
- UI layer selection remains presentation-only and must not mutate Runner, Core, selection, tick, or live frame ownership.

## Acceptance Scenarios

### AL-007-S37-AC01: Core and Observer preserve canonical resource layer identity

Given `config/scenarios/demo/canonical_test_world.toml` declares 19 resource ids, when a `WorldState` is built and projected through `CommittedSnapshot` and `VisualWorldProjection`, then the projection exposes 19 resource layers with stable `layer_index`, `resource_type_id`, `resource_id`, totals, cells, and completeness.

### AL-007-S37-AC02: Runner JSON exposes the same layer contract

Given a live Runner debug projection bundle for `canonical_test_world`, when `/projections/latest` returns `visual_world.payload.resource_layers`, then every resource layer has `layer_index`, `resource_type_id`, `resource_id`, `width`, `height`, `total_amount`, `cells`, and `completeness`.

### AL-007-S37-AC03: Control Center lists and selects all canonical resource layers by name

Given the Control Center receives a debug projection with canonical resource layers, when the Monitor Layers panel and debug legend render, then all available resource layer rows are named by `resource_id`, search can match those names, and initial canonical layer visibility is not collapsed to only legacy organic/mineral/energy channels.

## TDD Tasks

### T01 RED: Core snapshot carries resource identity metadata

- [ ] Add failing `tests/phase3f_canonical_test_world.rs::canonical_test_world_snapshot_preserves_resource_layer_identity`.
- [ ] Assert `ScenarioDocument::resolve(Path::new("config/scenarios"), "demo/canonical_test_world")` returns 19 `resource_type_ids`.
- [ ] Build `WorldState::from_config(document.runtime_config.clone())`.
- [ ] Assert every `snapshot.resource_layers[index]` has matching `layer_index`, `resource_type_id`, `resource_id`, positive dimensions, and `width * height` cells.
- [ ] Run `cargo test canonical_test_world_snapshot_preserves_resource_layer_identity`; expected RED: `ResourceLayerSnapshot` has no `resource_type_id` and `resource_id`.

### T02 GREEN: Add snapshot metadata without changing Tick behavior

- [ ] Update `src/core/snapshot.rs` `ResourceLayerSnapshot` with `resource_type_id: u32` and `resource_id: String`.
- [ ] In `CommittedSnapshot::from_world`, populate metadata from `world.config().chemistry.resources.get(layer).map(|resource| resource.id.clone())`.
- [ ] Use `format!("resource_{layer}")` only as observer-metadata fallback for impossible mismatches.
- [ ] Update manual snapshot fixtures in `tests/observer_projection_payloads.rs`, `tests/organism_hulls_joints.rs`, and `tests/phenotype_traits.rs`.
- [ ] Run `cargo test canonical_test_world_snapshot_preserves_resource_layer_identity` and `cargo test observer_projection_payloads`; expected GREEN.

### T03 RED: Observer payload preserves resource identity

- [ ] Extend `tests/observer_projection_payloads.rs::visual_world_projection_is_bounded_and_source_backed`.
- [ ] Assert `payload.resource_layers[0].resource_type_id == 0`, `resource_id == "amino_acid"`, and the second layer uses a different id.
- [ ] Run `cargo test visual_world_projection_is_bounded_and_source_backed`; expected RED: `ResourceLayerSummaryPayload` has no metadata fields.

### T04 GREEN: Project metadata through Observer

- [ ] Add `resource_type_id: u32` and `resource_id: String` to `src/observer/payloads.rs::ResourceLayerSummaryPayload`.
- [ ] Map `resource_type_id: layer.resource_type_id` and `resource_id: layer.resource_id.clone()` in `src/observer/projection.rs`.
- [ ] Run `cargo test visual_world_projection_is_bounded_and_source_backed` and `cargo test observer_payloads_do_not_enter_core_behavior`; expected GREEN.

### T05 RED: Runner projection JSON exposes resource layer names

- [ ] Add `tests/runner_http_projections.rs::latest_projections_expose_named_canonical_resource_layers`.
- [ ] Start `"demo/canonical_test_world"` with request id `"run-canonical-resource-layer-contract-test"`.
- [ ] Assert `/projections/latest` returns 19 `visual_world.payload.resource_layers`.
- [ ] Assert layer 0 has `resource_type_id: 0`, `resource_id: "amino_acid"`.
- [ ] Assert layer 18 has `resource_type_id: 18`, `resource_id: "nucleotide_precursor"`.
- [ ] Run `cargo test latest_projections_expose_named_canonical_resource_layers`; expected RED: JSON has no resource metadata.

### T06 GREEN: Serialize resource metadata in Viewer Server

- [ ] Update `src/viewer_server/api/projections.rs` resource layer JSON with `"resource_type_id": layer.resource_type_id` and `"resource_id": layer.resource_id`.
- [ ] Run `cargo test latest_projections_expose_named_canonical_resource_layers` and `cargo test latest_projections_return_bounded_observer_payload_bundle`; expected GREEN.

### T07 RED: UI adapter and types require resource identity

- [ ] Update `ui/control-center/src/projection/debugProjectionAdapter.test.ts` wire fixture with `resource_type_id: 0` and `resource_id: "amino_acid"`.
- [ ] Assert normalized layer has `resourceTypeId: 0` and `resourceId: "amino_acid"`.
- [ ] Run `cd ui/control-center; npm test -- --run src/projection/debugProjectionAdapter.test.ts`; expected RED: normalized layer lacks metadata.

### T08 GREEN: Normalize resource identity in UI projection model

- [ ] Add `resourceTypeId: number` and `resourceId: string` to `DebugResourceLayer` in `ui/control-center/src/projection/types.ts`.
- [ ] Map `resourceTypeId: layer.resource_type_id ?? layer.layer_index` and `resourceId: layer.resource_id ?? \`resource_${layer.layer_index}\`` in `debugProjectionAdapter.ts`.
- [ ] Update existing UI fixtures that construct `DebugResourceLayer`.
- [ ] Run `cd ui/control-center; npm test -- --run src/projection/debugProjectionAdapter.test.ts`; expected GREEN.

### T09 RED: Layers panel displays canonical names and keeps toggles presentation-only

- [ ] Update `ui/control-center/src/app/layerDisplayModel.test.ts` to expect `buildResourceLayerDisplay({ resourceId: "nucleotide_precursor", layerIndex: 18, ... }).primaryLabel === "nucleotide_precursor"`.
- [ ] Update `ui/control-center/src/components/LayerPanel.test.tsx` fixture to include `amino_acid` and `nucleotide_precursor`.
- [ ] Assert labels `Resource layer amino_acid` and `Resource layer nucleotide_precursor` exist.
- [ ] Assert toggling `Resource layer amino_acid` only changes `activeResourceLayers`.
- [ ] Run `cd ui/control-center; npm test -- --run src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx`; expected RED: rows still use `Resource Layer N`.

### T10 GREEN: Display resource names without changing simulation truth

- [ ] Update `ui/control-center/src/app/layerDisplayModel.ts` so resource `primaryLabel` is `layer.resourceId`.
- [ ] Include `Layer ${layer.layerIndex}`, `resource_type_id ${layer.resourceTypeId}`, and completeness in provenance.
- [ ] Update `LayerPanel.tsx` accessible labels to `Resource layer ${display.primaryLabel}`.
- [ ] Keep `toggleResourceLayer(layer.layerIndex)` unchanged.
- [ ] Run `cd ui/control-center; npm test -- --run src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx`; expected GREEN.

### T11 RED: Canonical resource layers are not initially collapsed to legacy channels

- [ ] Add `ui/control-center/src/app/appState.test.ts::auto-enables all source-backed resource layers for a new live canonical projection`.
- [ ] Fixture should provide 19 `resourceLayers` with `resourceTypeId`, `resourceId`, dimensions, totals, cells, and completeness.
- [ ] Assert `activeResourceLayers` becomes `[0, 1, ..., 18]`.
- [ ] Assert `frame.resources[0][0].layers?.[18]` exists.
- [ ] Run `cd ui/control-center; npm test -- --run src/app/appState.test.ts`; expected RED: state keeps default `[0, 1]`.

### T12 GREEN: Auto-initialize layer selection per live run, preserve manual toggles

- [ ] Add `resourceLayerSelectionInitializedForRunId: string | null` to `AppState`.
- [ ] Initialize it to `null`.
- [ ] In `setDebugProjections`, when projections are available and contain resource layers for the current live run, set `activeResourceLayers` to all available indices only when `resourceLayerSelectionInitializedForRunId !== debugProjections.runId`.
- [ ] Preserve manual toggles on repeated projections for the same run.
- [ ] Run `cd ui/control-center; npm test -- --run src/app/appState.test.ts src/viewer/visualEffectsToggles.test.ts`; expected GREEN.

### T13 RED: Debug legend and search use resource ids

- [ ] Update `ui/control-center/src/viewer/debugLayers.test.ts` to expect `channelLabel === "amino_acid"` and legend text contains `amino_acid`.
- [ ] Update `ui/control-center/src/components/WorldViewer.test.tsx` resource search fixture to include `resourceId: "nucleotide_precursor"`.
- [ ] Assert searching `nucleotide` matches the resource row.
- [ ] Run `cd ui/control-center; npm test -- --run src/viewer/debugLayers.test.ts src/components/WorldViewer.test.tsx`; expected RED: debug legend/search still use `Layer N` and modulo color labels.

### T14 GREEN: Resource names drive debug legend/search text

- [ ] Update `ui/control-center/src/viewer/debugLayers.ts`:
  - `channelLabel` uses `layer.resourceId`.
  - `legendLabel` uses `${layer.resourceId} total ${formatAmount(layer.totalAmount)}`.
  - `resourceChannelColor(layer.layerIndex)` remains visual palette only.
  - Keep `DEBUG_RESOURCE_LEGEND_LIMIT` unless layout tests prove it blocks available resource layer rows.
- [ ] Update `WorldViewer.tsx::matchesResourceLayerSearch()` to include `resourceId`, `layerIndex`, and totals.
- [ ] Run `cd ui/control-center; npm test -- --run src/viewer/debugLayers.test.ts src/components/WorldViewer.test.tsx`; expected GREEN.

### T15 Integration verification

- [ ] Run focused Rust verification:

```powershell
cargo test canonical_test_world_snapshot_preserves_resource_layer_identity latest_projections_expose_named_canonical_resource_layers observer_projection_payloads
```

- [ ] Run focused UI verification:

```powershell
cd ui/control-center
npm test -- --run src/projection/debugProjectionAdapter.test.ts src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx src/app/appState.test.ts src/viewer/debugLayers.test.ts src/components/WorldViewer.test.tsx
```

- [ ] Run UI build:

```powershell
cd ui/control-center
npm run build
```

- [ ] If local environment allows, run full Rust suite:

```powershell
cargo test
```

- [ ] If full Rust suite fails on known unrelated `runner_binary_serve` behavior, record it in the report as unrelated existing failure and include focused pass evidence.

## Required Report After Execution

Create:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-al-007-s37-canonical-resource-layers-end-to-end.md
```

The report must include acceptance coverage for `AL-007-S37-AC01` through `AL-007-S37-AC03`, RED/GREEN command evidence, changed Core/Observer/Runner/UI files, explicit confirmation that no resource mechanics or field behavior changed, and known unrelated test failures if any.

## Approval Gate

This plan is ready for execution only after explicit user approval:

```text
OK EXECUTE AL-007-S37
```
