---
tags:
  - alife
  - mechanics
  - agent/router
---

# Material -> Fragment -> Resource / Remains

> Agent pre-flight card. Canon wins on conflict.

## Use When
- Material decay
- Cell death or remains
- Joint degradation
- external digestion or breakdown

## Must Read
- [[docs/world/materials]]
- [[docs/world/reactions]]
- [[docs/biology/cell-state]]
- [[docs/biology/lifecycle]]
- [[docs/biology/joint]]
- [[docs/config/reactions_config]]

## Contract
- External Material becomes MaterialFragment, not ordinary Resource.
- Fragment keeps material identity and passive physical properties.
- Fragment becomes Resource only through explicit degradation/reaction/conversion.
- Decay depends on Material and local conditions.
- Stable remains may persist as structure or obstacle.

## Checks
- instant disappearance -> reject
- silent fragment uptake -> reject
- active Cell capability outside living context -> reject
- decomposition outputs and Heat are accounted
