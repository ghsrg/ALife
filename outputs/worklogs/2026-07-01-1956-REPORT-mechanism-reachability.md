# REPORT: Mechanism Reachability

Date: 2026-07-01 19:56

## Goal

Implement the first observer-only `mechanism-reachability` mode according to `2026-07-01-1927-PLAN-mechanism-reachability.md`.

## Scope

- Added a mechanism registry loader and validator.
- Added Phase 1 mechanism registry.
- Added deterministic reachability evaluation over current micro simulator history.
- Added reachability JSON/Markdown artifact writer.
- Added `reachability` CLI subcommand.
- Added tests for registry validation, evaluation, writer output, feedback loop and CLI integration.
- Updated tool and implementation documentation.

## Files Changed

- `tools/early-stability/src/reachability.py`
- `tools/early-stability/src/reachability_writer.py`
- `tools/early-stability/src/cli.py`
- `tools/early-stability/mechanisms/phase1.toml`
- `tools/early-stability/tests/test_reachability.py`
- `tools/early-stability/tests/test_reachability_writer.py`
- `tools/early-stability/tests/test_reachability_cli.py`
- `tools/early-stability/README.md`
- `docs/implementation/early-stability-tool.md`
- `docs/implementation/mechanism-reachability.md`

## Verification

- RED verified for missing `reachability` module.
- GREEN verified for `test_reachability.py`.
- RED verified for missing `reachability_writer` module.
- GREEN verified for `test_reachability_writer.py`.
- RED verified for unknown `reachability` CLI command.
- GREEN verified for `test_reachability_cli.py`.
- Full `tools/early-stability` pytest suite passed before documentation/report finalization.
- CLI smoke created `outputs/reachability/phase1_smoke/` with all expected artifacts.
- Smoke result: 9 mechanisms, 5 pass, 2 warning, 0 fail, 0 blocked, 2 tool-limited, overall `partial`.
- Final repeated pytest after documentation/report changes could not be rerun inside this session because escalation hit the Codex usage limit and sandboxed local Python returned `Access is denied`.

## Interpretation

`mechanism-reachability` is now the second calibration stage after `early-stability-parameter-tuning`.

It does not prove advanced biology or future Rust runtime behavior. It checks whether mechanisms in the registry are currently observable, blocked, bypassed or tool-limited under the existing early stability micro simulator.

Warnings, blocked mechanisms or bypasses should send work back to parameter tuning before data model decisions are treated as settled.

The first smoke run correctly reports tuning feedback:

- `passive_energy_income`: warning, `competing_path_cheaper`;
- `capacity_limit`: warning, `capacity_too_high`;
- `growth_estimate`: tool-limited;
- `joint_upkeep_estimate`: tool-limited.

## Open Questions

- Future process-level mechanisms will need richer event counters after `alife-core` starts exposing process/feasibility events.
- Current Phase 1 reachability is intentionally limited to the early stability tool state: energy, capacity, heat, waste and estimate-only growth/joint placeholders.
