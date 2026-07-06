---
tags:
  - alife
  - mechanics
  - agent/router
---

# Deterministic Execution

> Agent pre-flight card. Canon wins on conflict.

## Use When
- RNG or mutation
- parallel execution
- entity iteration
- scheduler cadence
- replay tests

## Must Read
- [[docs/world/tick-semantics]]
- [[docs/engine/scheduler]]
- [[docs/implementation/architecture]]
- [[docs/implementation/implementation-phases]]

## Contract
- Same seed, config and binary produce the same result.
- RNG source and consumption order are explicit.
- Stable ids do not create hidden biological priority.
- Parallel partitions and reductions are deterministic.
- Optimization must preserve semantic visibility and outcomes.

## Checks
- unordered iteration cannot affect behavior
- replay hash matches
- cadence defines snapshot and commit boundaries
- conflict resolution is stable
