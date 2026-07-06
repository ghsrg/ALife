---
tags:
  - alife
  - mechanics
  - agent/router
---

# Resource -> Material

> Agent pre-flight card. Canon wins on conflict.

## Use When
- synthesis
- growth
- repair material production
- reverse decomposition checks

## Must Read
- [[docs/world/resources]]
- [[docs/world/materials]]
- [[docs/world/reactions]]
- [[docs/biology/processes]]
- [[docs/biology/process-capabilities]]
- [[docs/biology/feasibility]]
- [[docs/config/reactions_config]]
- [[docs/engine/chemistry]]

## Contract
- Resource does not become Material directly.
- Conversion requires an explicit registered process or reaction.
- Required capability, inputs, Energy, locality and capacity must pass.
- Inputs, Material output, waste and Heat are accounted.
- Material capability applies only in valid context.

## Checks
- no conversion rule -> reject
- missing precursor or capability -> reject
- insufficient Energy or capacity -> reject
- output and residuals are bounded and accounted
