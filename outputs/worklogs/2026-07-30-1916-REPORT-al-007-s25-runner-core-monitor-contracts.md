# AL-007-S25 Runner And Core Monitor Contracts Report

Status: in-progress

## Scope executed

- Added a typed `MonitorDataPanelProjection/v1` payload to the Runner projection bundle.
- Added source-backed World population lifecycle and Resource Cycle payloads.
- Added explicit unavailable descriptors for Material Cycle, Energy Flow, classification, organisms, lineages, evolution, and analytics contracts that are not yet source-backed in `/projections/latest`.
- Added UI monitor projection adapter and attached optional `monitor` payload to the existing debug projection bundle.
- Extended UI RRD history to support mean-collapsed multi-series samples and source-backed Monitor resource accounting samples.
- Updated Data Panel model so World `Resource Cycle` and `Resource Distribution Over Time` consume `monitor.payload.world.resourceCycle` + UI RRD instead of `visualWorld.resourceLayers`.
- Preserved Cells lifecycle as compact stacked progress bar and kept role/behavior charts unavailable when typed source rows are absent.

## Important constraints preserved

- UI does not infer roles from Cell energy, radius, materials, or `roleHint`.
- UI does not estimate Energy Flow from resource energy values.
- Resource layer toggles remain Map presentation controls, not Data Panel source filters.
- Available Monitor subsections are not listed in `completeness.missing_fields`.

## Verification

```powershell
cargo test --test observer_monitor_payloads -- --nocapture
cargo test --test runner_monitor_projections -- --nocapture
npm.cmd test -- src/app/rrdMetricHistory.test.ts src/app/appState.test.ts src/app/monitorSurfaceModel.test.ts src/projection/monitorProjectionAdapter.test.ts src/projection/debugProjectionAdapter.test.ts src/components/BottomDataPanel.test.tsx --run
npm.cmd run build
```

Result:

- Rust monitor payload tests: 2 passed.
- Rust runner monitor projection test: 1 passed.
- Targeted UI tests: 6 files, 46 tests passed.
- Production build: passed.

## Remaining work before closure

AL-007-S25 must not be marked done yet.

Open source-backed contracts:

- Cells observed primary roles and potential role markers from typed Observer classification rows.
- Organism behavior profiles and size bins from `OrganismViewProjection` / behavior profile projections.
- Lineage current population, history, genealogy, and spatial footprint.
- Evolution genome provenance, mutation history, diversity, and carrier history.
- Analytics selected metric descriptor with definition, unit, aggregation, interval, sampling, completeness, and source/classifier provenance.
- Material Cycle and Energy Flow accounting from Core/Observer metrics rather than UI estimates.

## Next recommended slice

Continue AL-007-S25 as smaller sub-slices:

1. `AL-007-S25A`: Cells classification payload rows and Data Panel role bars.
2. `AL-007-S25B`: OrganismView size bins and behavior profile distribution.
3. `AL-007-S25C`: Lineage/Evolution source projections.
4. `AL-007-S25D`: Material/Energy accounting contracts.
