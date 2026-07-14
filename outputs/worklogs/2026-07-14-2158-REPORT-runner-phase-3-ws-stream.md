# Runner Phase 3 WS Stream — Implementation Report

Date: 2026-07-14 21:58

Plan: `outputs/worklogs/2026-07-13-1245-PLAN-runner-phase-3-ws-stream.md`

Branch: `codex/runner-3-ws-stream`

Base: `codex/runner-2-http-api`

## Result

Implemented Runner Phase 3 push-only WebSocket streaming foundation.

This phase adds:

- canonical `WorldFrameProjection v1` from committed snapshots;
- binary ALIF frame encoder/decoder;
- non-blocking broadcast hub based on `tokio::sync::broadcast`;
- tick-loop frame broadcast after committed ticks;
- `/stream` WebSocket endpoint;
- initial and command-triggered text status messages;
- binary `runner --serve` smoke coverage for `/stream`.

No server-side stream history, seek, replay, multi-tick step command, or UI behavior was added.

## Commits

- `01b2739 docs(runner): sync phase 3 WS plan with Canon`
- `c6bc8e2 feat(runner): add world frame projection and ALIF encoder`
- `ea1e6ca feat(viewer-server): add WebSocket broadcaster`
- `06e48c6 feat(viewer-server): broadcast frames from tick loop`
- `d272dd4 feat(viewer-server): add push-only WebSocket stream`
- `5b72741 feat(viewer-server): broadcast run status over WebSocket`
- `05996a3 test(runner): cover WebSocket stream in serve mode`

## Implemented contracts

### Frame projection

`runner::projections::WorldFrameProjection` is built from committed Core state only.

Projected fields:

- `schema_version`
- `committed_tick`
- `heat`
- `waste`
- visible cells:
  - `id`
  - `x`
  - `y`
  - `radius`
  - `energy`
  - `lifecycle`

Lifecycle is encoded as stable numeric values:

- `Alive = 0`
- `Stressed = 1`
- `Dormant = 2`
- `Dead = 3`

### ALIF frame

Binary frame format:

- magic: `ALIF`
- version: `1`
- schema version
- committed tick
- heat
- waste
- cell count
- fixed-size cell records

The test decoder is intentionally local to `viewer_server::frame_encoder` and is used to lock format behavior.

### Broadcast hub

`viewer_server::broadcaster::Broadcaster` wraps `tokio::sync::broadcast`.

Message variants:

- `Frame(Vec<u8>)`
- `Status(String)`

Slow clients are isolated by independent receivers. Lagged receivers skip missed messages and continue.

### Tick-loop integration

The runner tick loop now:

- advances Core through the existing command/run state path;
- reads committed snapshots;
- projects them to `WorldFrameProjection`;
- encodes ALIF frames;
- sends frames through the broadcaster without holding the Core/Runner lock while awaiting client IO.

Broadcast rate is controlled by `target_broadcast_fps` from `config/server.toml`.

### WebSocket endpoint

`GET /stream`:

- upgrades to WebSocket;
- sends initial status text JSON immediately;
- forwards binary ALIF frame messages;
- forwards status text messages;
- ignores client input except close/error;
- never mutates Runner/Core state.

Status text JSON includes Canon fields:

- `type`
- `process_state`
- `active_run_state`
- `run_id`
- `committed_tick`
- `scenario_id`
- `scenario_hash`
- `effective_seed`
- `terminal_reason`

### Status broadcasts

Run-control HTTP commands now broadcast status after state changes:

- `POST /run/start`
- `POST /run/pause`
- `POST /run/resume`
- `POST /run/step`
- `POST /run/stop`

`/run/start` broadcasts `running` before spawning the tick loop so short scenarios do not skip the observable running state.

## TDD checkpoints

Added tests:

- `tests/runner_projection_world_frame.rs`
- `tests/runner_frame_encoder.rs`
- `tests/runner_broadcaster.rs`
- `tests/runner_tick_broadcast.rs`
- `tests/runner_ws_stream.rs`

Extended:

- `tests/runner_binary_serve.rs`

RED checkpoints were observed for:

- missing projection/encoder modules;
- missing broadcaster module;
- missing `new_app_state(..., target_broadcast_fps)` and broadcaster fields;
- `/stream` returning HTTP 404 before endpoint implementation;
- missing command-triggered WS status broadcasts.

The serve-mode WS smoke test was verification-only because `/stream` was already implemented by that point.

## Verification

Passed:

```text
cargo fmt --check
```

Passed:

```text
CARGO_PROFILE_TEST_DEBUG=0 cargo test --test runner_frame_encoder --test runner_projection_world_frame --test runner_broadcaster --test runner_tick_broadcast --test runner_ws_stream --test runner_http_run_control --test runner_http_info --test runner_http_scenarios --test runner_serve_smoke --test runner_binary_serve --test runner_server_config --test runner_viewer_server_state
```

Passed:

```text
CARGO_PROFILE_TEST_DEBUG=0 cargo test --workspace
```

Notes:

- A first full workspace run timed out after 10 minutes while tests were still progressing and had no visible test failure.
- The second full workspace run completed successfully with a longer timeout.
- The workspace test output contains an expected negative-path `sweep_analyzer` subprocess exit code `1`; the owning test passes and the final `cargo test --workspace` exit code was `0`.
- `CARGO_PROFILE_TEST_DEBUG=0` was used because earlier Windows runs hit linker/PDB/disk-space failures.

## Constraints intentionally preserved

- No server-side frame history.
- No seek/replay protocol.
- No multi-tick advancement command.
- No UI-specific state mutation.
- `POST /run/step` semantics remain exactly one committed tick and paused-only.
- WebSocket clients are read-only observers.

## Next step

Runner Phase 3 is sufficient to unblock early UI work that needs a live read-only stream.

Runner Phase 4 is still useful for UI quality and resilience, but not required to start a minimal UI:

- client-side frame decoding and state interpolation;
- reconnect behavior;
- visible degraded/offline status;
- richer viewer diagnostics;
- more deliberate browser-facing protocol polish.
