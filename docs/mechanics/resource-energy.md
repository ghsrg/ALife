---
tags:
  - alife
  - mechanics
  - agent/router
---

# Resource / Field -> Energy Buffer

> Agent pre-flight card. Canon wins on conflict.

## Use When
- energy conversion
- metabolism-like process
- photosensitive conversion
- mandatory or planned Energy accounting

## Must Read
- [[docs/world/resources]]
- [[docs/world/energy]]
- [[docs/world/reactions]]
- [[docs/world/field-semantics]]
- [[docs/biology/process-capabilities]]
- [[docs/biology/feasibility]]
- [[docs/config/reactions_config]]

## Contract
- Energy Buffer is local Cell state, not matter.
- Resource or Field potential needs explicit controlled conversion.
- Compatible Material/capability and Feasibility are required.
- Products, waste and Heat are accounted separately.
- Energy Buffer cannot be transferred between independent Cells.

## Checks
- free Energy -> reject
- passive release charging Cell directly -> reject
- Energy above material-defined capacity -> reject or explicit handling
- consumed matter has explicit products/residuals
