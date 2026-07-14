---
tags: [alife, canon, area/runner, audience/agent]
---
# Runner Execution Modes
> Defines how the Runner process starts and how a simulation run is initiated.

## Modes
The Runner supports:
- `run`: prepare and execute one Scenario;
- `serve`: start without requiring an active World;
- `bootstrap`: prepare and validate Tick 0 without starting Core;
- `validate`: validate Scenario structure and references without generating a World;
- `resume`: restore a persisted run state; future-compatible.

## Run Mode
```text
Start -> Resolve -> Validate -> Bootstrap -> Core -> Execute -> Exit
```
Run mode requires a Scenario source. Bootstrap failure prevents Core startup.

Completion or failure terminates the process unless keep-alive behavior is explicitly configured.

## Service Mode
```text
Start -> Runner Ready -> Active Run Idle -> Accept Commands
```
Service mode does not require a Scenario at startup.

CLI, HTTP, or UI adapters may submit later commands through the shared Runner contract.

The process remains available after a run completes unless shutdown is requested.

## Bootstrap Mode
```text
Scenario -> Resolve -> Validate -> Bootstrap
         -> PreparedWorld Validation -> Manifest -> Exit
```
Bootstrap mode must not execute Tick 0.

It may emit a prepared-state artifact only through an explicit serialization contract.

Its result must equal the preparation stage used by `run` and `serve`.

## Validation Mode
Validation checks:
- schema version and document structure;
- known identifiers and references;
- required relations;
- numeric ranges and stability bounds;
- adapter-independent normalization.

It must not perform stochastic generation or create `WorldState`.

## Resume Mode
```text
Checkpoint -> Compatibility Validation -> Core Restore -> Runner Control
```
Resume restores persisted authoritative state instead of creating a new World.

It must not invoke Bootstrap unless a future migration contract requires it.

## Adapter Equivalence
Execution mode selects process behavior, not simulation behavior.

Equivalent CLI and UI commands must use the same resolution, Bootstrap, validation, and Core startup paths.

## Concurrency
One Runner process supports at most one authoritative active run.

A start request while another run is preparing, running, paused, or stopping must be rejected.

Parallel experiments require separate Runner processes until a multi-run contract exists.

## Invariants
```text
New World -> Bootstrap is mandatory.
Resume -> Bootstrap is skipped.
Validate -> no World generation.
Bootstrap -> no Tick execution.
Serve -> Scenario is optional at startup.
Run -> Scenario is required at startup.
```

## Semantic Links
- [[docs/runner/runner|Runner]]
- [[docs/runner/run-lifecycle|Run Lifecycle]]
- [[docs/runner/command-contract|Command Contract]]
- [[docs/runner/scenario-resolution|Scenario Resolution]]
- [[docs/runner/bootstrap|Bootstrap]]
