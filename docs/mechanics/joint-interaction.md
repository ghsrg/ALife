---
tags:
  - alife
  - mechanics
  - agent/router
---

# Joint Interaction

> Agent pre-flight card. Canon wins on conflict.

## Use When
- Joint creation or repair
- mechanical connection
- Resource or Heat transfer
- division, death or Joint break

## Must Read
- [[docs/biology/joint]]
- [[docs/biology/membrane]]
- [[docs/world/materials]]
- [[docs/world/resources]]
- [[docs/world/physics]]
- [[docs/biology/lifecycle]]
- [[docs/world/tick-semantics]]

## Contract
- Joint is local, material and paid.
- Channels are explicit: mechanical, Resource, signal, Heat.
- Base Resource transfer is passive and capacity-bounded.
- Energy Buffer and Genome are not transferred.
- Division does not duplicate Joints.
- Death disables living channels; remaining material degrades physically.

## Checks
- distant or material-free Joint -> reject
- Resource transfer without channel -> reject
- direct Energy/Genome transfer -> reject
- creation/break/repair respects locality and accounting
