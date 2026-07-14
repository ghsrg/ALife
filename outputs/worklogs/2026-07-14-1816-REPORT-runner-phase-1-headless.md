---
tags:
  - alife
  - implementation-report
  - runner
  - phase/runner-1
  - tdd
---

# Runner Phase 1 Headless Report

## Source Plan

Implemented from:

```text
outputs/worklogs/2026-07-12-1700-PLAN-runner-phase-1-headless.md
```

Branch:

```text
codex/runner-1-headless
```

## Summary

Runner-1 headless foundation is implemented with the Canon path:

```text
ScenarioSource -> ScenarioDocument -> Bootstrap::prepare -> PreparedWorld -> TickExecutor
```

Runner now has:

- shared command contract types;
- Canon lifecycle states;
- scenario discovery and loading;
- headless `RunEngine`;
- committed snapshot ring buffer;
- `--debug` terminal progress table;
- `--progress-interval-ms`;
- `src/bin/runner.rs` binary.

## Files Added

```text
src/bin/runner.rs
src/runner/commands.rs
src/runner/engine.rs
src/runner/lifecycle.rs
src/runner/progress.rs
src/runner/ring_buffer.rs
src/runner/scenario.rs

tests/runner_bootstrap.rs
tests/runner_commands.rs
tests/runner_headless_e2e.rs
tests/runner_lifecycle.rs
tests/runner_progress.rs
tests/runner_ring_buffer.rs
tests/runner_scenario_doc.rs
tests/runner_scenario_loader.rs
tests/runner_state_machine.rs
```

## Files Modified

```text
src/runner/mod.rs
```

## Implemented Contracts

### Lifecycle

Implemented:

```text
RunnerProcessState: Starting | Ready | ShuttingDown | Failed
ActiveRunState: Idle | Preparing | Running | Paused | Stopping | Completed | Failed
```

Covered transitions:

- process `Starting -> Ready -> ShuttingDown`;
- active run `Idle -> Preparing -> Running`;
- `Running -> Paused -> Running`;
- `Preparing/Running/Paused -> Stopping -> Completed`;
- `Completed/Failed -> Idle`;
- invalid transitions return state conflict.

### Commands

Implemented shared command types:

```text
ValidateScenario
PrepareScenario
StartRun
PauseRun
ResumeRun
StepRun
StopRun
GetRunStatus
```

`StepRun` contract:

```text
valid only while Paused
executes exactly one Tick
does not mean multi-tick advancement
```

### Scenario Discovery And Resolution

Implemented:

- recursive TOML discovery under `config/scenarios`;
- stable sorting by scenario id/path;
- discovery reads `scenario_id` without forcing full Core validation of every file;
- selected scenario loading resolves full `ScenarioDocument`.

Reason for discovery behavior:

Some existing scenario TOML files are discoverable but currently fail full parser validation because they use older chemistry fields. Discovery should not fail the whole list because one scenario is invalid; actual run preparation still validates the selected scenario.

### Bootstrap Integration

`RunEngine::prepare_from_document` calls:

```text
Bootstrap::prepare(document) -> PreparedWorld
TickExecutor::new(prepared.runtime_config)
```

The engine records the initial committed snapshot at Tick 0 and does not execute a Tick during preparation.

### Headless Engine

Implemented:

- `RunEngineConfig`;
- `RunEngine`;
- start/pause/resume/stop;
- one-tick paused step;
- running tick loop;
- run until configured tick;
- snapshot ring buffer;
- scenario hash tracking.

### CLI

Implemented:

```text
cargo run --bin runner -- --list
cargo run --bin runner -- bootstrap_minimal_viable_world
cargo run --bin runner -- --debug --progress-interval-ms 200 bootstrap_minimal_viable_world
```

`--serve` is explicitly rejected as Runner-2 future work.

### Debug Progress

Implemented:

- default interval `2000 ms`;
- `--progress-interval-ms <N>`;
- table fields:
  - elapsed seconds;
  - tick/max;
  - ticks/sec;
  - cell count;
  - alive/dead counts;
  - heat;
  - waste;
  - state.

Debug output is observer-only and reads committed snapshots.

## TDD Evidence

RED failures observed before implementation:

```text
runner_lifecycle       -> missing runner::lifecycle
runner_commands        -> missing runner::commands
runner_ring_buffer     -> missing runner::ring_buffer
runner_scenario_loader -> missing runner::scenario
runner_bootstrap       -> missing runner::engine/scenario
runner_progress        -> missing runner::progress
cargo build --bin runner -> no bin target named runner
```

Additional RED/fix:

```text
runner_scenario_loader initially failed because discovery forced full parse of an older invalid scenario file.
Fixed by making discovery read only scenario_id and deferring full validation to selected scenario loading.

runner_progress initially failed because padded tick formatting did not contain "10/20".
Fixed by rendering tick progress as a contiguous "{tick}/{max_ticks}" value.
```

## Verification

Targeted Runner-1 acceptance:

```text
cargo test --test runner_ring_buffer       -> 2 passed
cargo test --test runner_state_machine     -> 3 passed
cargo test --test runner_scenario_loader   -> 3 passed
cargo test --test runner_headless_e2e      -> 3 passed
cargo test --test runner_progress          -> 2 passed
cargo test --test runner_lifecycle         -> 3 passed
cargo test --test runner_commands          -> 3 passed
cargo test --test runner_scenario_doc      -> 1 passed
cargo test --test runner_bootstrap         -> 2 passed
cargo build --bin runner                   -> pass
cargo build --bin sweep_analyzer           -> pass
```

Manual CLI smoke:

```text
cargo run --bin runner -- --list
cargo run --bin runner -- bootstrap_minimal_viable_world
cargo run --bin runner -- --debug --progress-interval-ms 200 bootstrap_minimal_viable_world
```

Workspace verification:

```text
cargo fmt --check                                      -> pass
cargo test --workspace                                 -> pass
cargo clippy --workspace --all-targets -- -D warnings  -> pass
```

Notes:

- The first `cargo test --workspace` run with 300s timeout was interrupted by timeout, not by a test failure.
- Re-run with 600s timeout completed successfully.
- `cargo test --workspace` prints expected stderr from a negative-path `sweep_analyzer` validation test, but the command exits with code 0.

## Known Limits

- HTTP/WS server is not implemented; that is Runner-2/Runner-3.
- `--serve` is intentionally rejected with a Runner-2 message.
- Scenario discovery lists scenario ids even if a file may fail full validation later; run start still validates selected scenario through `ScenarioDocument`.
- `RunEngine` is intentionally single-process/single-run and synchronous for Runner-1.
- Runner status projection module is not split into a separate file yet; debug progress uses committed snapshots directly.

## Result

Runner-1 can start and run a Bootstrap-prepared world headlessly:

```text
cargo run --bin runner -- bootstrap_minimal_viable_world
```

And it can show early terminal observability:

```text
cargo run --bin runner -- --debug --progress-interval-ms 200 bootstrap_minimal_viable_world
```
