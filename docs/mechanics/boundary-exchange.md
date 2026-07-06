---
tags:
  - alife
  - mechanics
  - agent/router
---

# Boundary -> Resource Exchange

> Agent pre-flight card. Canon wins on conflict.

## Use When
- Resource uptake or export
- passive diffusion or leakage
- Boundary damage
- active transport

## Must Read
- [[docs/biology/membrane]]
- [[docs/biology/feasibility]]
- [[docs/biology/processes]]
- [[docs/biology/process-capabilities]]
- [[docs/world/resources]]
- [[docs/world/space]]

## Contract
- Default permeability is blocked.
- Exchange rule is blocked, passive or active_required.
- Passive exchange follows physical compatibility and gradient.
- Active exchange requires registered process, capability, Energy and Feasibility.
- Target capacity and Boundary integrity constrain transfer.

## Checks
- no permeability rule -> blocked
- Resource outside locality -> reject
- active transport without capability/Energy -> reject
- transfer amount is bounded by source, rate and target capacity
