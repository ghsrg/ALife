---
tags:
  - alife
  - mechanics
  - agent/router
---

# Genome -> ActionPlan

> Agent pre-flight card. Canon wins on conflict.

## Use When
- Genome Runtime
- regulatory inputs or outputs
- ActionPlan generation
- epigenetic or runtime state

## Must Read
- [[docs/biology/genome]]
- [[docs/genetics/genome-runtime]]
- [[docs/genetics/regulatory-interface]]
- [[docs/genetics/regulatory-network]]
- [[docs/biology/action-process-registry]]
- [[docs/biology/process-capabilities]]

## Contract
- Runtime reads normalized local snapshot inputs.
- Material/capability masks apply before executable intent.
- Genome output is bounded priority, not world mutation.
- Outputs bind only to registered current processes.
- ActionPlan is committed before Feasibility.
- Memory exists only in explicit RuntimeState or EpigeneticState.

## Checks
- global/observer input -> reject
- unregistered output -> reject
- direct process execution from Genome -> reject
- ActionPlan is deterministic for same state and seed
