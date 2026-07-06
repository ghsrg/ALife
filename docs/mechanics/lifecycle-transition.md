---
tags:
  - alife
  - mechanics
  - agent/router
---

# Cell State -> Lifecycle Transition

> Agent pre-flight card. Canon wins on conflict.

## Use When
- upkeep and stress
- dormancy
- death or decomposition
- Cell state labels

## Must Read
- [[docs/biology/cell]]
- [[docs/biology/cell-state]]
- [[docs/biology/lifecycle]]
- [[docs/world/energy]]
- [[docs/biology/membrane]]
- [[docs/world/materials]]

## Contract
- No HP or abstract viability score controls death.
- Living continuity depends on Boundary, Materials, Energy path, Genome and maintenance.
- Mandatory costs precede planned actions.
- Dormancy reduces activity but not all costs or degradation.
- Death produces inert/decomposing physical state and remains.
- Observer state labels cannot control behavior.

## Checks
- instant death without physical cause -> reject
- unpaid mandatory cost is handled explicitly
- dead Cell does not disappear
- transition reason is traceable
