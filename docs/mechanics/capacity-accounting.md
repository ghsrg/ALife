---
tags:
  - alife
  - mechanics
  - agent/router
---

# Capacity Accounting

> Agent pre-flight card. Canon wins on conflict.

## Use When
- uptake, synthesis or repair
- growth or division
- initial Cell state
- internal rebalance or overflow

## Must Read
- [[docs/biology/cell]]
- [[docs/world/space]]
- [[docs/world/resources]]
- [[docs/world/materials]]
- [[docs/biology/feasibility]]
- [[docs/config/world_config]]

## Contract
- Resources, Materials, Genome carriers and fragments occupy capacity.
- Energy Buffer does not occupy volume directly.
- Total capacity is bounded by footprint and storage-capable Materials.
- Actions use post-mandatory free capacity.
- Overflow requires explicit rejection, export, pressure, growth or instability handling.

## Checks
- used capacity is fully accounted
- free capacity cannot be negative silently
- uptake/synthesis output fits
- initialization above capacity -> reject
