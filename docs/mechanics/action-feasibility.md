---
tags:
  - alife
  - mechanics
  - agent/router
---

# ActionPlan -> Feasibility -> Execution

> Agent pre-flight card. Canon wins on conflict.

## Use When
- planned action execution
- controlled reaction
- action conflicts
- rejection diagnostics

## Must Read
- [[docs/biology/action-process-registry]]
- [[docs/biology/feasibility]]
- [[docs/biology/process-capabilities]]
- [[docs/world/energy]]
- [[docs/world/tick-semantics]]
- [[docs/biology/lifecycle]]

## Contract
- Feasibility accepts registered current actions only.
- It reads post-mandatory committed state and does not mutate it.
- Energy, Resources, Materials, capacity, space, locality and lifecycle are checked.
- Rejection has explicit reasons, no cost and no output.
- Allowed actions proceed to deterministic conflict resolution and execution.

## Checks
- status future/unregistered -> reject
- insufficient requirement -> reject
- rejected action changes state/progress -> fail
- hidden per-process permission bypass -> fail
