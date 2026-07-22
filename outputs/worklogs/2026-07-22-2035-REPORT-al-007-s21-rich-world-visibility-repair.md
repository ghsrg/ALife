---
tags:
  - alife
  - worklog/report
  - delivery/al-007-s21
---

# AL-007-S21 Rich World Visibility Repair

## Status

done

## Scope

Repair the rich-world visibility gap exposed after `AL-002-S11` and `AL-007-S10`: generated resource layers and source-backed Cell details existed in Core/Observer paths, but the Control Center still rendered a mostly empty live map and an under-informative Cell Inspector.

## Changes

- Added `config/scenarios/demo/demo_world_resource.toml` as a manually inspectable demo scenario with three typed resource layers, manifest-only field summaries, and multiple Cells with varied materials/resources/energy capacities.
- Expanded `CommittedSnapshot` and Observer visual payloads with exact resource layer cells, per-Cell energy capacity, material slots, internal resources, and local external resources for every current resource layer.
- Extended the Runner projection gateway JSON for the new source-backed visual-world fields.
- Normalized the richer debug projection payload in the UI and merged available debug projections into the visible live frame.
- Rendered multiple resource layers as map overlay channels instead of showing only the first layer.
- Expanded Cell Inspector output for raw/capacity energy, position, radius, materials, internal resources, and local external resources.
- Tightened Monitor layout CSS so the map remains first-viewport dominant after the debug projection repair.

## Notes

- Bootstrap field generators remain manifest-only. They are exposed as field summaries/warnings, but not drawn as spatial map grids because Core does not yet own a spatial Field grid model.
- The ALIF v2 stream remains the live transport for frame timing/cells; richer resource/material details are merged from the read-only `/projections/latest` Observer payload.
- Resource overlay currently maps visible resource layers into existing organic/mineral/energy channels. Richer per-type legends and selectable resource inspectors belong to `AL-007-S11`/`AL-007-S12`.
- This repair does not add World Editor, storage replay, long-run analytics, or new simulation mechanics.

## Verification

- `cargo fmt --check`
- `cargo test --test observer_rich_world_projection --test runner_http_projections --test bootstrap_integration --test bootstrap_rich_generators --test observer_projection_payloads`
- `npm.cmd test -- --run src/projection/debugProjectionAdapter.test.ts src/app/appState.test.ts src/components/CellInspector.test.tsx src/components/WorldViewer.test.tsx src/viewer/debugLayers.test.ts`
- `npm.cmd test -- --run src/app/appState.test.ts`
- `npm.cmd run build`
- `npm.cmd run e2e -- monitor.spec.ts ui-1c-a-visual.spec.ts`
