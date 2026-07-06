---
tags:
  - alife
  - mechanics
  - agent/router
---

# Tick Transaction

> Agent pre-flight card. Canon wins on conflict.

## Use When
- scheduler or system order
- state visibility
- deltas, conflicts, commits
- process execution

## Must Read
- [[docs/world/tick]]
- [[docs/world/tick-semantics]]
- [[docs/engine/scheduler]]
- [[docs/implementation/architecture]]

## Contract
- Read stable committed snapshots.
- Decision writes ActionPlans only.
- Execution writes deltas, resolves conflicts, then commits.
- Physics/Lifecycle reads post-action state.
- Observer reads committed state only.
- No same-phase feedback or iteration-order behavior.

## Checks
- uncommitted state is not readable
- commit boundary is explicit
- competing requests use deterministic resolution
- same-tick signal feedback is rejected
