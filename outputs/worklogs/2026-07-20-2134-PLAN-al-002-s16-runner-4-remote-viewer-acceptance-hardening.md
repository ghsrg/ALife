---
tags:
  - alife
  - worklog/plan
  - plan/AL-002-S16
  - area/runner
---

# PLAN: AL-002-S16 Runner-4 Remote Viewer And Acceptance Hardening

## TDD_PLAN_PROPOSAL

Plan ID: `AL-002-S16`

Selected slice: `Remote Viewer And Acceptance Hardening`

Roadmap status at planning time: `planned`

Requested route: `delivery-control` -> `roadmap-control` `TDD_PLAN_REQUEST`

Approval gate: do not execute until the human replies `OK EXECUTE AL-002-S16`.

## Goal

Close the Runner-4 hardening slice without changing simulation mechanics:

- remote viewer mode is explicit opt-in and uses a validated origin policy;
- HTTP validation, Scenario, Bootstrap, and command failures expose stable Runner error categories;
- service shutdown is graceful and observable;
- reconnecting WebSocket clients receive current status and the latest available frame without retrying commands;
- terminal status, collapse reason, and status timing/performance metadata remain available through Runner projections.

## Files And Docs Read

Source of truth:

- `docs/PRINCIPLES.md`
- `docs/INDEX.md`
- `docs/runner/INDEX.md`
- `docs/runner/runner.md`
- `docs/runner/execution-modes.md`
- `docs/runner/run-lifecycle.md`
- `docs/runner/command-contract.md`
- `docs/runner/projections.md`
- `docs/implementation/implementation-plan-runner.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`

Evidence only:

- `outputs/worklogs/2026-07-13-1325-PLAN-runner-phase-4-hardening.md`
- `outputs/worklogs/2026-07-14-1816-REPORT-runner-phase-1-headless.md`
- `outputs/worklogs/2026-07-14-2005-REPORT-runner-phase-2-http-api.md`
- `outputs/worklogs/2026-07-14-2158-REPORT-runner-phase-3-ws-stream.md`
- `outputs/worklogs/2026-07-15-1740-REPORT-runner-debug-snapshot-cadence.md`

Code and tests inspected:

- `src/runner/server_config.rs`
- `src/viewer_server/mod.rs`
- `src/viewer_server/api/mod.rs`
- `src/viewer_server/api/info.rs`
- `src/viewer_server/api/run.rs`
- `src/viewer_server/api/stream.rs`
- `src/viewer_server/state.rs`
- `src/bin/runner.rs`
- `tests/runner_server_config.rs`
- `tests/runner_http_info.rs`
- `tests/runner_http_run_control.rs`
- `tests/runner_ws_stream.rs`
- `tests/runner_serve_smoke.rs`
- `tests/runner_binary_serve.rs`

## Source-Of-Truth Hierarchy

1. `docs/PRINCIPLES.md`
2. Runner canon under `docs/runner/`
3. `docs/implementation/implementation-plan-runner.md`
4. Delivery plan/status under `docs/delivery/`
5. Existing code and tests as current implementation evidence
6. Worklogs as historical evidence only

## Current Implementation Evidence

Observed implemented pieces:

- `ServerConfig` has `bind_host`, `port`, `allow_remote_viewer`, and `target_broadcast_fps`.
- Default server config is local-only: `127.0.0.1:8080`, `allow_remote_viewer=false`.
- `viewer_server::create_app` applies local-only CORS headers for localhost/127.0.0.1 viewer origins.
- `/stream` sends an initial status text message on connect.
- `/run/start`, `/run/pause`, `/run/resume`, `/run/step`, `/run/stop`, and `/run/status` exist.
- Basic HTTP error bodies include `ok`, `category`, `message`, and `current_state`.

Observed gaps:

- `allowed_origins` is not represented in `ServerConfig`.
- `server/info` currently reports `allow_remote_viewer=false` from a hardcoded value, not from effective config.
- CORS policy is not configured from server config and has no validated remote-mode allowlist.
- HTTP error bodies do not include command identity, scenario identity when available, or concrete current process/run state.
- Service mode uses plain `axum::serve(...).await`; no graceful shutdown path was observed.
- WebSocket reconnect gets current status, but no latest frame is sent immediately on reconnect.
- `/run/status` lacks status performance metadata such as `ticks_per_second`.
- Terminal/collapse status retention is partial: `terminal_reason` exists, but collapse/error detail and final-state broadcast coverage need tests.

## Assumptions And Open Questions

- Assumption: `allowed_origins` is required because the implementation plan explicitly names it for Runner-4 remote viewer mode.
- Assumption: remote mode may bind to `0.0.0.0` only when `allow_remote_viewer=true`; a non-local bind with `allow_remote_viewer=false` should fail config validation.
- Assumption: reconnect must send at most the latest available committed frame, not a server-side replay/history buffer. This preserves the implementation-plan rule that clients own history.
- Needs Review: exact shutdown API shape should be internal Runner/server infrastructure, not a public Runner command, because `docs/runner/command-contract.md` has no `Shutdown` command.
- Needs Review: collapse reason source should reuse current Core/Runner terminal diagnostics if available; do not invent new collapse semantics in this slice.

## BDD Agent Scenario Cards

### AL-002-S16-AC01: Remote Viewer Opt-In And Origin Policy

Source links:

- `docs/runner/execution-modes.md`
- `docs/implementation/implementation-plan-runner.md`
- `docs/delivery/roadmap.md`

Intent: local mode remains safe by default, while remote viewer mode is explicit and bounded by configured origins.

Priority: high

Given local default server config with `allow_remote_viewer=false`
When the viewer server is built and queried with local and non-local origins
Then local viewer origins are allowed, non-local origins are not allowed, and `/server/info` reports `allow_remote_viewer=false`.

Given remote viewer config with `allow_remote_viewer=true`, a non-local bind host, and `allowed_origins`
When CORS preflight or GET/POST requests arrive from configured and unconfigured origins
Then only configured origins receive CORS headers, and `/server/info` reports the effective remote-viewer mode.

TDD obligation: write config and HTTP/CORS tests before adding `allowed_origins` or refactoring `create_app`.

Evidence IDs: `AL-002-S16-EV01`, `AL-002-S16-EV02`

### AL-002-S16-AC02: Stable HTTP Validation And Bootstrap Errors

Source links:

- `docs/runner/runner.md`
- `docs/runner/command-contract.md`
- `docs/runner/run-lifecycle.md`

Intent: HTTP adapters expose Runner errors in canonical categories without leaking partial run state.

Priority: high

Given an unknown Scenario, invalid Scenario input, invalid command state, or Bootstrap failure
When HTTP requests are sent through `/run/start` or control endpoints
Then the response includes stable `category`, actionable `message`, command identity, scenario identity when available, and current process/run state.

And failed start/prepare leaves no partial active run.

TDD obligation: add failing HTTP tests that assert exact stable fields and state after failure.

Evidence IDs: `AL-002-S16-EV03`, `AL-002-S16-EV04`

### AL-002-S16-AC03: Graceful Service Shutdown

Source links:

- `docs/runner/run-lifecycle.md`
- `docs/runner/execution-modes.md`
- `docs/implementation/implementation-plan-runner.md`

Intent: service mode rejects new starts during shutdown, stops active Core work safely, broadcasts terminal status, and closes WS clients.

Priority: high

Given Runner service mode is ready with or without an active run
When shutdown is requested by the server runtime
Then process state becomes `shutting_down`, new starts are rejected, active run termination reaches a safe commit boundary, and WebSocket clients are closed or receive terminal status before closure.

TDD obligation: first add internal server-state tests for shutdown behavior, then add the smallest binary/service smoke test that can run reliably in CI.

Evidence IDs: `AL-002-S16-EV05`, `AL-002-S16-EV06`

### AL-002-S16-AC04: Reconnect Receives Current State And Latest Frame

Source links:

- `docs/runner/projections.md`
- `docs/implementation/implementation-plan-runner.md`

Intent: reconnect is observational only and never repeats start/pause/resume/step commands.

Priority: high

Given a run has started and at least one frame has been committed
When a WebSocket client disconnects and reconnects to `/stream`
Then the new connection receives current status and the latest available frame without sending any command request.

Given no frame has been committed yet
When a client connects
Then the client receives current status and no fabricated frame.

TDD obligation: add reconnect tests that observe messages only and do not call control endpoints after reconnect.

Evidence IDs: `AL-002-S16-EV07`, `AL-002-S16-EV08`

### AL-002-S16-AC05: Terminal Status, Collapse Reason, And Status Metrics

Source links:

- `docs/runner/run-lifecycle.md`
- `docs/runner/projections.md`
- `docs/implementation/implementation-plan-runner.md`

Intent: `/run/status` and WS status expose enough read-only metadata for UI and diagnostics after run completion or failure.

Priority: high

Given a run completes, is stopped, or fails with a terminal reason
When `/run/status` is queried or a client connects to `/stream`
Then run identity, committed tick, scenario hash, effective seed, active-run state, terminal reason/collapse reason, and timing metadata remain available.

Given a run is active
When `/run/status` is queried
Then `ticks_per_second` is present as bounded status metadata, with deterministic simulation semantics unaffected by wall-clock measurement.

TDD obligation: add status JSON tests before extending status state and projection DTOs.

Evidence IDs: `AL-002-S16-EV09`, `AL-002-S16-EV10`

### AL-002-S16-AC06: Runner-4 Closure Verification

Source links:

- `docs/delivery/roadmap.md`
- `docs/delivery/acceptance.md`

Intent: closure evidence proves the slice without relying on the old Runner-4 plan as canonical requirements.

Priority: medium

Given all Runner-4 hardening tests pass
When closure verification is performed
Then the roadmap, status, acceptance matrix, and worklog report identify closed coverage and remaining deferred work, if any.

TDD obligation: no status moves to `done` until closure verification and a REPORT exist.

Evidence IDs: `AL-002-S16-EV11`, `AL-002-S16-EV12`

## Numbered TDD Tasks

### AL-002-S16-T01: RED for `AL-002-S16-AC01`

- Add failing tests for default local-only config, configured `allowed_origins`, rejected remote origins, and `/server/info` effective `allow_remote_viewer`.
- Candidate files: `tests/runner_server_config.rs`, `tests/runner_http_info.rs`, new `tests/runner_remote_viewer_cors.rs`.
- Run:
  - `cargo test --target-dir target/codex-al002s16 runner_server_config`
  - `cargo test --target-dir target/codex-al002s16 runner_http_info`
- Capture expected failures as `AL-002-S16-EV01`.

### AL-002-S16-T02: GREEN for `AL-002-S16-AC01`

- Add `allowed_origins` to `ServerConfig` and `config/server.toml`.
- Add config validation for unsafe remote binding combinations.
- Refactor viewer app creation so CORS and `/server/info` use effective `ServerConfig`.
- Keep local default behavior backward compatible.
- Re-run T01 commands and capture passes as `AL-002-S16-EV02`.

### AL-002-S16-T03: RED for `AL-002-S16-AC02`

- Add failing HTTP tests for unknown scenario, invalid JSON/body, state conflict, and Bootstrap failure shape.
- Assert stable fields: `ok=false`, `category`, `message`, `command`, `scenario_id` when available, `process_state`, and `active_run_state`.
- Candidate files: `tests/runner_http_run_control.rs`, new `tests/runner_http_error_contract.rs`.
- Run:
  - `cargo test --target-dir target/codex-al002s16 runner_http_error_contract`
- Capture expected failures as `AL-002-S16-EV03`.

### AL-002-S16-T04: GREEN for `AL-002-S16-AC02`

- Introduce a single HTTP error response DTO/mapping for Runner adapter errors.
- Map Scenario, Bootstrap, Core, and state-conflict failures to canonical categories.
- Ensure failed start leaves active run non-active and status query remains valid.
- Re-run T03 command and existing run-control tests; capture passes as `AL-002-S16-EV04`.

### AL-002-S16-T05: RED for `AL-002-S16-AC03`

- Add failing tests for server shutdown state transition, new-start rejection while shutting down, active tick-loop stop request, and WS terminal/close behavior.
- Prefer deterministic state/service tests before OS-signal smoke.
- Candidate files: new `tests/runner_graceful_shutdown.rs`, `tests/runner_binary_serve.rs`.
- Run:
  - `cargo test --target-dir target/codex-al002s16 runner_graceful_shutdown`
- Capture expected failures as `AL-002-S16-EV05`.

### AL-002-S16-T06: GREEN for `AL-002-S16-AC03`

- Add internal graceful-shutdown path for service mode without adding a public simulation command unless canon is updated.
- Wire `src/bin/runner.rs` service mode to graceful shutdown.
- Broadcast terminal status and close or stop WS streams without blocking Core.
- Re-run T05 command and `runner_binary_serve`; capture passes as `AL-002-S16-EV06`.

### AL-002-S16-T07: RED for `AL-002-S16-AC04`

- Add failing reconnect tests: connect, start run, observe frame, disconnect, reconnect, assert initial status plus latest available frame.
- Add a no-frame case: idle/pre-frame reconnect must not fabricate a frame.
- Candidate file: new `tests/runner_ws_reconnect.rs` or extension of `tests/runner_ws_stream.rs`.
- Run:
  - `cargo test --target-dir target/codex-al002s16 runner_ws_reconnect`
- Capture expected failures as `AL-002-S16-EV07`.

### AL-002-S16-T08: GREEN for `AL-002-S16-AC04`

- Store only the latest available encoded frame or projection reference in server state.
- Send current status first and latest frame second on connect when available.
- Do not add unbounded server-side frame history.
- Re-run T07 and existing `runner_ws_stream`; capture passes as `AL-002-S16-EV08`.

### AL-002-S16-T09: RED for `AL-002-S16-AC05`

- Add failing tests for `/run/status` fields after running, stopped/completed, and failed/collapsed states.
- Assert status includes `ticks_per_second` and terminal/collapse reason fields with stable null/non-null behavior.
- Candidate files: new `tests/runner_run_status_hardening.rs`, `tests/runner_http_run_control.rs`.
- Run:
  - `cargo test --target-dir target/codex-al002s16 runner_run_status_hardening`
- Capture expected failures as `AL-002-S16-EV09`.

### AL-002-S16-T10: GREEN for `AL-002-S16-AC05`

- Extend shared status state and status JSON/WS text with timing metadata.
- Preserve final status summary after stop/completion/failure until explicit Idle reset path.
- Reuse existing Core/Runner diagnostics for collapse/terminal reason; do not invent mechanics.
- Re-run T09 and status/WS tests; capture passes as `AL-002-S16-EV10`.

### AL-002-S16-T11: REFACTOR And Compatibility Sweep

- Remove duplicate status-label/error mapping where practical.
- Keep public API additions backward compatible for existing UI and tests.
- Run focused compatibility commands:
  - `cargo test --target-dir target/codex-al002s16 runner_server_config`
  - `cargo test --target-dir target/codex-al002s16 runner_http_info`
  - `cargo test --target-dir target/codex-al002s16 runner_http_run_control`
  - `cargo test --target-dir target/codex-al002s16 runner_ws_stream`
  - `cargo test --target-dir target/codex-al002s16 runner_serve_smoke`
  - `cargo test --target-dir target/codex-al002s16 runner_binary_serve`
- Capture pass/fail summary as `AL-002-S16-EV11`.

### AL-002-S16-T12: Closure Docs And Report

- Create a REPORT worklog for `AL-002-S16` only after verification.
- Update `docs/delivery/roadmap.md`, `docs/delivery/status.md`, `docs/delivery/acceptance.md`, `docs/delivery/worklog-ledger.md`, and `outputs/worklogs/index.md`.
- Do not mark `AL-002-S16` done without closure verification.
- Capture delivery-control closure evidence as `AL-002-S16-EV12`.

## Verification Commands

Primary focused commands:

```text
cargo test --target-dir target/codex-al002s16 runner_server_config
cargo test --target-dir target/codex-al002s16 runner_http_info
cargo test --target-dir target/codex-al002s16 runner_http_run_control
cargo test --target-dir target/codex-al002s16 runner_ws_stream
cargo test --target-dir target/codex-al002s16 runner_serve_smoke
cargo test --target-dir target/codex-al002s16 runner_binary_serve
```

Expected new focused commands:

```text
cargo test --target-dir target/codex-al002s16 runner_remote_viewer_cors
cargo test --target-dir target/codex-al002s16 runner_http_error_contract
cargo test --target-dir target/codex-al002s16 runner_graceful_shutdown
cargo test --target-dir target/codex-al002s16 runner_ws_reconnect
cargo test --target-dir target/codex-al002s16 runner_run_status_hardening
```

Final closure command:

```text
cargo test --target-dir target/codex-al002s16
```

## Forbidden Scope

- Do not change Core simulation mechanics.
- Do not change world laws, Genome behavior, Bootstrap semantics, or Observer classification.
- Do not implement UI reconnect UX in this slice.
- Do not add server-side frame history beyond the latest available frame.
- Do not expose internal `WorldState` through HTTP or WS.
- Do not treat the old Runner-4 worklog as canonical requirements when it conflicts with `docs/runner/`.
- Do not move `AL-002-S16` to `done` without closure verification and a REPORT.

## Approval Request

Reply `OK EXECUTE AL-002-S16` to authorize execution of this TDD plan.

Reply `CHANGE AL-002-S16` with corrections to revise the plan.
