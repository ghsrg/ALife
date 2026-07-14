---
tags: [alife, docs/index, area/runner, audience/agent]
---
# Runner Index
> Navigation for Runner orchestration, execution modes, lifecycle, commands, Scenario resolution, projections, and Bootstrap.

## Reading Order
1. [[docs/runner/runner|Runner]]
2. [[docs/runner/execution-modes|Execution Modes]]
3. [[docs/runner/run-lifecycle|Run Lifecycle]]
4. [[docs/runner/command-contract|Command Contract]]
5. [[docs/runner/scenario-resolution|Scenario Resolution]]
6. [[docs/runner/projections|Runner Projections]]
7. [[docs/runner/bootstrap|Bootstrap]]

## Documents

### Runner
Defines Runner purpose, responsibilities, boundaries, shared control flow, and relation to Core.

### Execution Modes
Defines `run`, `serve`, `bootstrap`, `validate`, and future-compatible `resume` process behavior.

### Run Lifecycle
Defines Runner process states, active-run states, valid transitions, start atomicity, and cleanup.

### Command Contract
Defines transport-independent commands shared by CLI, HTTP, UI, tests, and batch tools.

### Scenario Resolution
Defines Scenario sources, normalization, overrides, reference resolution, canonicalization, hashing, and immutability.

### Runner Projections
Defines versioned read-only status, frame, event, summary, and Bootstrap manifest projections.

### Bootstrap
Defines deterministic preparation of complete Tick 0 state under reusable `src/bootstrap/`.

## Area Boundaries
```text
docs/runner  -> orchestration and preparation contracts
docs/config  -> Scenario and initialization schemas
docs/engine  -> Core runtime and technical constraints
docs/world   -> world laws and domain semantics
docs/ui      -> interaction and rendering behavior
docs/implementation -> implementation plans and worklogs
```

## Core Flow
```text
Scenario Source
  -> Scenario Resolution
  -> Bootstrap
  -> PreparedWorld
  -> Runner Lifecycle
  -> Simulation Core
  -> Committed Projections
```

## Implementation Links
- [[docs/implementation/architecture|Implementation Architecture]]
- [[docs/implementation/implementation-plan-runner|Runner Implementation Plan]]
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]

## Practical Usage
- [[docs/RUNNER_USAGE|Runner Usage Guide]]

## Invariants
```text
Canon defines behavior before implementation.
Runner contains no world mechanics.
Bootstrap prepares Tick 0 but executes no Tick.
CLI and UI use shared contracts.
Committed Core state remains authoritative.
```
