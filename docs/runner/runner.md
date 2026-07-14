---
tags: [alife, canon, area/runner, audience/agent]
---
# Runner
> Application authority that prepares, starts, controls, and exposes one ALife simulation run.
## Purpose
Runner connects CLI, HTTP, UI, tests, and batch tools to the deterministic simulation Core through shared application contracts.
## Position
```text
CLI / HTTP / UI / Tests
          |
          v
        Runner
   +------+------+------+
   |             |      |
Scenario     Bootstrap Lifecycle
Resolution
          |
          v
     Simulation Core
          |
          v
Committed Outputs -> Viewer / Storage / Analysis
```
## Responsibilities
Runner must:
- accept normalized application commands;
- resolve and validate a Scenario;
- invoke Bootstrap for a new World;
- start Core only from a valid prepared state;
- control pause, resume, step, stop, and completion;
- expose run status and committed projections;
- preserve run identity, seed, config hash, and schema versions;
- reject failures without partially starting a run.
## Boundaries
Runner must not:
- define world laws or simulation mechanics;
- generate Cells, Resources, Fields, Materials, or Genomes directly;
- mutate World state outside Core contracts;
- depend on UI or rendering state;
- expose internal `WorldState` as a public contract;
- allow adapters into the simulation hot path.
Bootstrap belongs to the Runner subsystem conceptually but is implemented as reusable application code under `src/bootstrap/`.

## Shared Control Contract
CLI and UI adapt their inputs into the same Runner commands.
```text
CLI arguments  ----\
                    -> Application Command -> Runner
HTTP / UI input ----/
```
Adapter-specific values resolve before shared logic. The same Scenario, seed, and options must produce the same prepared World regardless of adapter.

## Process And Run Separation
Runner may be ready without an active World.
```text
Runner Process: Ready
Active Run:     Idle
```
Service mode without a Scenario is valid. A later command may resolve a Scenario, invoke Bootstrap, and start Core.
Only one authoritative active run is supported unless a future contract enables multiple runs.

## New Run Contract
```text
Scenario Source
  -> Scenario Document
  -> Validation And Resolution
  -> Bootstrap
  -> Prepared World
  -> Core Start
  -> Committed Outputs
```
No Tick may execute before Bootstrap and prepared-state validation succeed.

## Output Contract
External consumers receive versioned, read-only projections from committed state. Viewer, storage, and analysis are consumers, not authorities.
Heatmaps, interpolation, filtering, and presentation state must not modify the run.

## Failure Contract
Resolution, validation, or Bootstrap failure must leave no partially active run.
Each failure exposes a stable category, actionable message, affected command or Scenario identity, and current Runner/run state.

## Invariants
```text
Core does not depend on Runner.
Runner does not contain world mechanics.
Bootstrap prepares Tick 0 but does not execute a Tick.
CLI and UI use the same application contracts.
Committed Core state is the source of truth.
At most one authoritative run is active.
```

## Semantic Links
- [[docs/runner/scenario-resolution|Scenario Resolution]]
- [[docs/runner/bootstrap|Bootstrap]]
- [[docs/runner/run-lifecycle|Run Lifecycle]]
- [[docs/runner/command-contract|Command Contract]]
- [[docs/runner/projections|Runner Projections]]
