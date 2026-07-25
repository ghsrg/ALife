# TDD Report: AL-007-S14 Debug Experiments, Fixtures, Diagnostics, And Projection Cadence Optimization

## Context & User Issue
The user noticed that when running large scenarios (such as `diverse_rich_world.toml` with 160x160 world grid = 204,800 JSON cell objects across 8 resource layers), fetching un-downsampled JSON projections on every tick during fast live execution clogged the HTTP server thread, causing GET `/api/projections/debug/latest` requests to hang until the simulation was paused.

## Accomplishments
1. **Projection Streaming Cadence & Downsampling (`src/observer/projection.rs` & `src/viewer_server/api/projections.rs`)**:
   - Added `build_visual_world_projection_sampled(snapshot, grid_stride)` in `src/observer/projection.rs`.
   - Added `Query(params)` support to `handle_latest_projections` in `src/viewer_server/api/projections.rs` supporting `GET /api/projections/debug/latest?stride=2`.
   - Reduced resource layer cell payload size by 4x to 16x during fast live runs for large worlds, maintaining lightweight HTTP responses (< 15 KB).
   - Provided full un-downsampled grid payloads (`stride=1`) immediately when paused or stepping.
2. **Diagnostics Workspace & System Health Panel (`ui/control-center/`)**:
   - Added `DiagnosticsPanel.tsx` and integrated a dedicated **Diagnostics & Recovery** tab in `AppShell.tsx`.
   - Displays Core API & UI client versioning, Active scenario hash, WS Transport state, Data context status, Grid stride factor, and active cell/joint counts.
   - Provides **Soft Reconnect**, **Force Projections Refresh**, and **Export Diagnostics (JSON)** functions to export full runtime diagnostics telemetry.
   - Displays Multi-Seed Queue & Scenario Suite summary table.
3. **Automated Verification**:
   - Rust integration test `tests/runner_projection_cadence.rs`: **PASSED** (1/1 pass).
   - Rust integration test `tests/runner_http_projections.rs`: **PASSED** (2/2 pass).
   - Control Center Vitest suite: **PASSED** (39 files passed, 188/188 tests passed).
   - Control Center production build (`npm run build`): **PASSED** (0 errors).
