---
tags:
  - alife
  - mechanics
  - agent/router
---

# Snapshot -> Replay

> Agent pre-flight card. Canon wins on conflict.

## Use When
- save/load
- snapshot schema
- event log
- replay or migration

## Must Read
- [[docs/engine/serialization]]
- [[docs/engine/storage]]
- [[docs/engine/ecs]]
- [[docs/world/tick-semantics]]
- [[docs/implementation/architecture]]

## Contract
- Snapshot is created from committed state.
- Snapshot contains schema version, config hash, seed, tick and RNG state.
- ProcessProgress and behavior-relevant runtime state are preserved.
- Save/load does not repair or reinterpret state silently.
- Observer and viewer state are not required for continuation.

## Checks
- save/load round trip preserves behavior
- replay from snapshot is deterministic
- incompatible schema needs explicit migration
- storage I/O is outside the behavior-critical hot path
