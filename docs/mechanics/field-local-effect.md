---
tags:
  - alife
  - mechanics
  - agent/router
---

# Field -> Local Effect

> Agent pre-flight card. Canon wins on conflict.

## Use When
- Heat, Light, Pressure or Radiation
- Flow or chemical gradient
- sensing or Field-driven reaction
- Field configuration

## Must Read
- [[docs/world/fields]]
- [[docs/world/field-semantics]]
- [[docs/world/physics]]
- [[docs/world/materials]]
- [[docs/config/fields_config]]

## Contract
- Field is locally sampled and is not a command.
- Effect requires Material, reaction, process or physics mediation.
- Propagation, decay, bounds and conservation/abstraction are explicit.
- Heat is not Energy Buffer.
- Derived debug/render fields are not behavior inputs.

## Checks
- direct Field -> behavior/Energy/damage -> reject
- missing local sampler or mediator -> reject
- dissipation/clamp is explicit
- effect respects Tick visibility
