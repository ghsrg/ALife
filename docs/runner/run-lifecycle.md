---
tags: [alife, canon, area/runner, audience/agent]
---
# Run Lifecycle
> Defines authoritative Runner process and active-run states.

## State Separation
```text
Runner Process: Starting | Ready | ShuttingDown | Failed
Active Run:     Idle | Preparing | Running | Paused | Stopping | Completed | Failed
```
A ready Runner may have no active World.

## Runner Process
```text
Starting -> Ready | Failed
Ready -> ShuttingDown
```
- `Starting`: initializes adapters, configuration, and services.
- `Ready`: accepts supported commands.
- `ShuttingDown`: rejects new starts and terminates the active run safely.
- `Failed`: cannot safely accept commands; process restart is required.

## Active Run
```text
Idle -> Preparing
Preparing -> Running | Failed | Idle
Running -> Paused | Stopping | Completed | Failed
Paused -> Running | Stopping | Completed | Failed
Stopping -> Completed | Failed
Completed -> Idle
Failed -> Idle
```
- `Idle`: no authoritative World is active.
- `Preparing`: resolves and validates Scenario, invokes Bootstrap, validates `PreparedWorld`, and starts Core.
- `Running`: Core executes scheduled Ticks.
- `Paused`: Core does not advance automatically.
- `Stopping`: Runner completes safe termination.
- `Completed`: run ended normally.
- `Failed`: preparation or execution could not continue safely.

No Tick may execute in `Preparing`.

`StepRun` may execute exactly one committed Tick while state remains `Paused`.

No new Tick may begin after stop becomes effective.

## Start Atomicity
A run becomes active only after:
1. Scenario resolution succeeds;
2. validation succeeds;
3. Bootstrap succeeds;
4. `PreparedWorld` validation succeeds;
5. Core accepts the prepared state.

Earlier failure must not leave a partial World.

## Command Validity
- `StartRun`: `Idle`;
- `PauseRun`: `Running`;
- `ResumeRun`: `Paused`;
- `StepRun`: `Paused`;
- `StopRun`: `Preparing`, `Running`, or `Paused`;
- status queries: every non-process-failed state.

Invalid commands return a state conflict without changing state.

## Completion And Cleanup
Run identity, final status, manifest, and committed outputs remain available in `Completed` or `Failed`.

Transition to `Idle` releases the authoritative World but may retain persisted records and summaries.

## Invariants
```text
At most one active run exists.
Preparing never executes a Tick.
Paused never advances automatically.
Step commits at most one Tick.
Failed preparation never exposes a partial World.
State transitions are explicit and observable.
```

## Semantic Links
- [[docs/runner/runner|Runner]]
- [[docs/runner/execution-modes|Execution Modes]]
- [[docs/runner/command-contract|Command Contract]]
- [[docs/runner/bootstrap|Bootstrap]]
