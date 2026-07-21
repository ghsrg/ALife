---
tags:
  - alife
  - worklog/report
  - plan/AL-002-S16
  - area/runner
---

# REPORT: AL-002-S16 Runner-4 Remote Viewer And Acceptance Hardening

## Purpose

Close `AL-002-S16` by hardening Runner service-mode behavior for remote viewer opt-in, config-driven CORS, stable HTTP errors, graceful shutdown state, WebSocket reconnect latest-frame delivery, and status metadata without changing Core simulation mechanics.

Worklogs are evidence only, not source of truth.

## Source Documents Read

- `docs/PRINCIPLES.md`
- `docs/runner/execution-modes.md`
- `docs/runner/run-lifecycle.md`
- `docs/runner/command-contract.md`
- `docs/runner/projections.md`
- `docs/implementation/implementation-plan-runner.md`
- `docs/delivery/roadmap.md`
- `docs/delivery/status.md`
- `docs/delivery/acceptance.md`
- `outputs/worklogs/2026-07-20-2134-PLAN-al-002-s16-runner-4-remote-viewer-acceptance-hardening.md`

## Changed Files

- `src/runner/server_config.rs`: adds `allowed_origins` and remote-viewer validation.
- `src/viewer_server/mod.rs`: adds config-aware CORS and `create_app_with_config`.
- `src/viewer_server/api/info.rs`: reports effective server config.
- `src/viewer_server/api/run.rs`: adds stable error DTO fields and status timing/collapse metadata.
- `src/viewer_server/api/stream.rs`: sends latest cached frame after reconnect status when available.
- `src/viewer_server/state.rs`: stores effective server config, latest frame, timing metadata, and graceful shutdown state helper.
- `src/bin/runner.rs`: wires service mode to effective config and Ctrl+C graceful shutdown.
- `tests/runner_server_config.rs`, `tests/runner_http_info.rs`, `tests/runner_http_run_control.rs`, `tests/runner_ws_reconnect.rs`, `tests/runner_graceful_shutdown.rs`: coverage for Runner-4 hardening.
- Delivery files: roadmap/status/acceptance/ledger/worklog index.

## Verification

| Evidence ID | Command | Result |
| --- | --- | --- |
| `AL-002-S16-EV01` | `cargo test --target-dir target/codex-al002s16 runner_server_config` | RED: compile failed on missing `allowed_origins` and `ServerConfig::validate`. |
| `AL-002-S16-EV02` | `cargo test --target-dir target/codex-al002s16 --test runner_server_config` | PASS: 6 passed, 0 failed. |
| `AL-002-S16-EV03` | `cargo test --target-dir target/codex-al002s16 --test runner_http_run_control` after adding hardening tests | RED/GREEN cycle exposed mutex lifetime deadlock in state-conflict paths; fixed before closure. |
| `AL-002-S16-EV04` | `cargo test --target-dir target/codex-al002s16 --test runner_http_run_control` | PASS: 9 passed, 0 failed. |
| `AL-002-S16-EV05` | `cargo test --target-dir target/codex-al002s16 --test runner_graceful_shutdown` | PASS: 2 passed, 0 failed. |
| `AL-002-S16-EV06` | `cargo test --target-dir target/codex-al002s16 --test runner_binary_serve` | PASS: 1 passed, 0 failed. |
| `AL-002-S16-EV07` | `cargo test --target-dir target/codex-al002s16 --test runner_ws_reconnect` | PASS: 2 passed, 0 failed. |
| `AL-002-S16-EV08` | `cargo test --target-dir target/codex-al002s16 --test runner_ws_stream` | PASS: 7 passed, 0 failed. |
| `AL-002-S16-EV09` | `cargo test --target-dir target/codex-al002s16 --test runner_http_info` | PASS: 7 passed, 0 failed. |
| `AL-002-S16-EV10` | `cargo test --target-dir target/codex-al002s16 --test runner_serve_smoke` | PASS: 2 passed, 0 failed. |
| `AL-002-S16-EV11` | `cargo test --target-dir target/codex-al002s16 --test runner_server_config --test runner_http_info --test runner_http_run_control --test runner_ws_stream --test runner_ws_reconnect --test runner_graceful_shutdown --test runner_serve_smoke --test runner_binary_serve` | PASS: 36 passed, 0 failed across Runner-4 focused regression. |
| `AL-002-S16-EV12` | `cargo fmt --check`; `git diff --check`; deterministic path/ID traceability over AL-002-S16 code, tests, delivery docs, and report. | PASS: format and diff checks exited 0; all AL-002-S16 closure paths exist. `git diff --check` reported line-ending warnings only. |

Full workspace `cargo test` was intentionally not run to avoid unnecessary target growth. The focused regression uses `target/codex-al002s16`.

## Coverage Matrix

| Plan ID | Requirement | Scenario ID | Task ID(s) | Evidence ID(s) | Test/Evidence | Status |
| --- | --- | --- | --- | --- | --- | --- |
| `AL-002-S16` | Remote viewer opt-in and origin policy | `AL-002-S16-AC01` | `AL-002-S16-T01`, `AL-002-S16-T02` | `AL-002-S16-EV01`, `AL-002-S16-EV02`, `AL-002-S16-EV09` | server config and HTTP info/CORS tests | covered |
| `AL-002-S16` | Stable HTTP validation and bootstrap errors | `AL-002-S16-AC02` | `AL-002-S16-T03`, `AL-002-S16-T04` | `AL-002-S16-EV03`, `AL-002-S16-EV04` | run-control error contract tests | covered |
| `AL-002-S16` | Graceful service shutdown | `AL-002-S16-AC03` | `AL-002-S16-T05`, `AL-002-S16-T06` | `AL-002-S16-EV05`, `AL-002-S16-EV06` | internal shutdown state tests and binary serve smoke | covered |
| `AL-002-S16` | Reconnect receives current state and latest frame | `AL-002-S16-AC04` | `AL-002-S16-T07`, `AL-002-S16-T08` | `AL-002-S16-EV07`, `AL-002-S16-EV08` | reconnect and existing WS stream tests | covered |
| `AL-002-S16` | Terminal status, collapse reason, and status metrics | `AL-002-S16-AC05` | `AL-002-S16-T09`, `AL-002-S16-T10` | `AL-002-S16-EV04`, `AL-002-S16-EV09`, `AL-002-S16-EV11` | status JSON, WS status, and focused regression | covered |
| `AL-002-S16` | Runner-4 closure verification | `AL-002-S16-AC06` | `AL-002-S16-T11`, `AL-002-S16-T12` | `AL-002-S16-EV11`, `AL-002-S16-EV12` | delivery closure updates and final verification | covered |

## Status Update

- `AL-002-S16`: `done`, `high`.
- `Current Focus`: none selected after closure.
- `Candidate Next Work`: reviewed and updated. `AL-005-S01` is now the recommended next planning slice because `AL-004-S02` and `AL-002-S16` are closed and UI-2A still needs storage/keyframe ownership.
- `AL-007-S09`: remains blocked by active `AL-005-S01` only.

## Follow-Up Slices

- `AL-005-S01`: run metadata, storage index, and keyframe/history ownership.
- `AL-002-S11` and `AL-002-S12`: Bootstrap rich generators and preview/report path before World Editor.
- `AL-002-S18`: final AL-002 closure matrix after remaining Bootstrap/debt slices close.

## Notes

- No Core simulation behavior was changed.
- Server keeps only the latest frame for reconnect; it does not add server-side history or seek.
- `Needs Review`: graceful shutdown is covered by internal state helper and binary serve smoke; OS signal end-to-end WS close sequencing is intentionally not over-specified in this slice.
