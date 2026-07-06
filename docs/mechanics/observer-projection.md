---
tags:
  - alife
  - mechanics
  - agent/router
---

# Committed State -> Observer Projection

> Agent pre-flight card. Canon wins on conflict.

## Use When
- viewer or rendering
- metrics and analytics
- OrganismView or lineage summaries
- debug traces

## Must Read
- [[docs/biology/organism]]
- [[docs/engine/rendering]]
- [[docs/engine/storage]]
- [[docs/implementation/architecture]]
- [[docs/world/tick-semantics]]

## Contract
- Observer reads committed simulation state.
- Viewer, analytics and debug data are projections, not authorities.
- OrganismView, lineage, fitness and summary labels are not behavior inputs.
- Sampling or dropping observer data cannot change simulation behavior.
- External adapters do not enter the simulation hot path.

## Checks
- core has no viewer dependency
- observer metrics are read-only
- UI actions do not mutate the active run
- full-state output is bounded or explicit debug mode
