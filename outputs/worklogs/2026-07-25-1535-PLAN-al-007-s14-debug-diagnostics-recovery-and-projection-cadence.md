# TDD Plan: AL-007-S14 Debug Experiments, Fixtures, Diagnostics, And Projection Cadence Optimization

## Context
When running large scenarios (e.g. `diverse_rich_world.toml` with 160x160 world grid = 204,800 JSON cell objects across 8 resource layers), fetching un-downsampled JSON projections on every tick during fast live execution clogs the HTTP server thread, causing GET `/api/projections/debug/latest` requests to hang until the simulation is paused.

## Objectives
1. **Projection Streaming Cadence & Downsampling (`src/observer/` & `src/viewer_server/`)**:
   - Implement spatial downsampling (grid stride) and tick cadence caching for `VisualWorldProjection` in `src/observer/projection.rs` and `src/viewer_server/api/projections.rs`.
   - Provide lightweight live streaming payloads for large worlds (reducing payload size from ~500 KB+ to < 15 KB).
   - Ensure full un-downsampled grid payloads are served immediately when paused or stepping.
2. **Diagnostics & Stale Recovery (`ui/control-center/`)**:
   - Add Diagnostic & Recovery panel in `WorldViewer.tsx` / `AppShell.tsx` displaying stream latency, projection cadence, and an explicit **Manual Refresh / Recovery** control.
3. **Automated Tests**:
   - Rust integration test `tests/runner_projection_cadence.rs` for sub-millisecond debug projection streaming under live runs.
   - Vitest component tests in `ui/control-center/` for cadence adapters and recovery controls.

## Verification Plan
- `cargo test --test runner_projection_cadence`
- `npx vitest run` in `ui/control-center`
- `npm run build` in `ui/control-center`
