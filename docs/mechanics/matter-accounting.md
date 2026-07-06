---
tags:
  - alife
  - mechanics
  - agent/router
---

# Matter Accounting

> Agent pre-flight card. Canon wins on conflict.

## Use When
- reaction or synthesis
- repair or degradation
- death and remains
- division or configured loss

## Must Read
- [[docs/PRINCIPLES]]
- [[docs/world/resources]]
- [[docs/world/materials]]
- [[docs/world/reactions]]
- [[docs/config/reactions_config]]
- [[docs/engine/chemistry]]

## Contract
- Matter is not created or silently destroyed.
- Every input maps to product, retained material, residual/waste or explicit sink.
- Energy and Heat do not explain missing matter.
- Amounts remain non-negative and bounded.
- Simplified units still require explicit accounting.

## Checks
- product without input -> reject
- input without destination -> reject or explicit warning
- implicit sink -> reject
- before/after matter balance is traceable
