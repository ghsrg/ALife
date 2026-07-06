---
tags:
  - alife
  - mechanics
  - agent/router
---

# Space -> Local Interaction

> Agent pre-flight card. Canon wins on conflict.

## Use When
- sensing, contact or uptake
- movement and collision
- Joint creation
- Resource, Field or trace lookup
- world boundaries

## Must Read
- [[docs/world/space]]
- [[docs/world/physics]]
- [[docs/config/world_config]]
- [[docs/engine/physics]]

## Contract
- Every interaction has spatial context.
- Proximity uses an explicit radius, contact rule, Field or material channel.
- Cells read local samples, not global maps.
- One spatial-grid contract resolves base locality.
- Boundary mode explicitly covers Cells, Resources and Fields.

## Checks
- no distance-free interaction
- radius/contact rule is defined
- target is inside valid locality
- boundary behavior is explicit
