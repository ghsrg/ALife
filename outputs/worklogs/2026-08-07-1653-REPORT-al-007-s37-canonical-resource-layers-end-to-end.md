---
plan_id: AL-007-S37
status: done
date: 2026-08-07
---

# Canonical Resource Layers End-To-End Contract Report

## Outcome

`AL-007-S37` is implemented as a cross-layer contract fix. `canonical_test_world.toml` resource layers now retain source-backed identity from Core snapshot through Observer projection, Runner `/projections/latest`, and Control Center presentation.

No resource mechanics, field mechanics, synthesis reactions, diffusion/decay behavior, or cell movement behavior were changed.

## Acceptance Coverage

| Acceptance | Result | Evidence |
| --- | --- | --- |
| `AL-007-S37-AC01` | Passed | `CommittedSnapshot` now carries `resource_type_id` and `resource_id`; Observer `ResourceLayerSummaryPayload` preserves them; `tests/phase3f_canonical_test_world.rs`; `tests/observer_projection_payloads.rs`. |
| `AL-007-S37-AC02` | Passed | Runner `/projections/latest` serializes named canonical resource layers; `tests/runner_http_projections.rs`. |
| `AL-007-S37-AC03` | Passed | Control Center adapter/model/panel/viewer use source-backed resource names, auto-enable all live canonical resource layers once per run, and keep toggles presentation-only; Vitest focused suite. |

## Implementation Notes

- Added read-only resource identity metadata to `ResourceLayerSnapshot`.
- Propagated resource identity through `VisualWorldProjection` payloads and Runner JSON.
- Updated Control Center `DebugResourceLayer` contract with `resourceTypeId` and `resourceId`.
- Changed resource layer labels/search/legend text to use source-backed IDs instead of legacy channel labels.
- Auto-initialized live resource layer selection to all available canonical layers for a new run, preserving manual toggles afterward.
- Updated tests and fixtures for named resource layer contract coverage.

## Verification

RED evidence captured during execution:

- `cargo test canonical_test_world_snapshot_preserves_resource_layer_identity` failed before implementation because `ResourceLayerSnapshot` did not expose `resource_type_id` or `resource_id`.
- `cargo test --test observer_projection_payloads visual_world_projection_is_bounded_and_source_backed` failed before Observer payload propagation because `ResourceLayerSummaryPayload` did not expose resource identity.
- `cargo test --test runner_http_projections latest_projections_expose_named_canonical_resource_layers` failed before Runner JSON propagation because `resource_type_id` was absent.
- Focused Vitest suite failed before UI changes because adapter/model/panel/viewer state still collapsed resources to legacy layer/channel labels and only auto-enabled legacy layers.

GREEN evidence:

- `cargo fmt --check` passed.
- `git diff --check` passed.
- `cargo test --test phase3f_canonical_test_world canonical_test_world_snapshot_preserves_resource_layer_identity` passed: 1/1.
- `cargo test --test observer_projection_payloads` passed: 6/6.
- `cargo test --test runner_http_projections latest_projections_expose_named_canonical_resource_layers` passed: 1/1.
- `npm.cmd test -- --run src/projection/debugProjectionAdapter.test.ts src/app/layerDisplayModel.test.ts src/components/LayerPanel.test.tsx src/app/appState.test.ts src/viewer/debugLayers.test.ts src/components/WorldViewer.test.tsx src/viewer/visualEffectsToggles.test.ts` passed: 7 files, 75 tests.
- `npm.cmd run build` passed.

## Known Constraints And Warnings

- Full `cargo test` was not completed in this worktree because local Windows linking hit disk/PDB limits (`no space on device`, `LNK1180`, `LNK1140`) during broad test builds. Focused Rust tests covering this slice passed.
- Vitest/build required unsandboxed execution in this `.worktrees` path because Vite/esbuild dependency scanning hit sandbox path traversal restrictions.
- Existing React `act(...)` warnings remain in `WorldViewer.test.tsx`; focused tests still passed.
- Vite emitted an existing chunk-size warning for `assets/index-COUB8I3d.js` at 576.26 kB after minification.
- Generated `target/` in this isolated worktree was removed during execution to recover from local disk pressure; it is rebuildable.
