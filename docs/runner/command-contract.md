---
tags: [alife, canon, area/runner, audience/agent]
---
# Runner Command Contract
> Shared application commands for CLI, HTTP, UI, tests, and batch adapters.

## Rules
- adapters translate external input into Runner commands;
- commands are transport-independent;
- every command returns explicit success or failure;
- invalid commands do not change state;
- adapters do not call Core or Bootstrap directly.

## Commands

### ValidateScenario
Input: `scenario_document`.

Returns `scenario_hash`, `schema_version`, and warnings.

Validates without generating a World.

### PrepareScenario
Input: `scenario_document`, optional `seed_override`.

Returns `bootstrap_manifest`, warnings, and optional prepared-state reference.

Bootstraps without starting Core.

### StartRun
Input:
```text
scenario_document
seed_override?
run_options
request_id?
```
Returns `run_id`, effective seed, Scenario hash, Bootstrap manifest, and `Running` state.

### PauseRun
Input: `run_id`.

Returns `Paused` state and committed Tick. Pause takes effect at a safe commit boundary.

### ResumeRun
Input: `run_id`.

Returns `Running` state and current committed Tick.

### StepRun
Input: `run_id`.

Executes exactly one committed Tick and returns to `Paused`.

### StopRun
Input: `run_id` and optional reason.

Returns `Stopping` or `Completed` with the last committed Tick.

### GetRunStatus
Returns process state, run identity, active-run state, committed Tick, effective seed, Scenario hash, and terminal reason.

It never mutates state.

## Identity And Duplication
`run_id` identifies one authoritative run.

A supplied `request_id` may detect duplicate start requests. Duplicate handling must never create multiple runs.

## Errors
Stable categories:
```text
invalid_command
state_conflict
scenario_error
bootstrap_error
core_error
run_not_found
unsupported_operation
```
Each error includes category, actionable message, command identity, and current state.

## Invariants
```text
CLI and UI issue equivalent commands.
One command produces at most one state transition.
StartRun is atomic.
StepRun commits at most one Tick.
Query commands never mutate state.
```

## Semantic Links
- [[docs/runner/runner|Runner]]
- [[docs/runner/execution-modes|Execution Modes]]
- [[docs/runner/run-lifecycle|Run Lifecycle]]
- [[docs/runner/scenario-resolution|Scenario Resolution]]
- [[docs/runner/bootstrap|Bootstrap]]
