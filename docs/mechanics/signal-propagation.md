---
tags:
  - alife
  - mechanics
  - agent/router
---

# Signal -> Propagation -> Runtime Input

> Agent pre-flight card. Canon wins on conflict.

## Use When
- signal emission or sensing
- Joint signal
- Resource-like trace
- Runtime signal state

## Must Read
- [[docs/biology/communication]]
- [[docs/biology/joint]]
- [[docs/world/tick-semantics]]
- [[docs/genetics/genome-runtime]]
- [[docs/world/materials]]
- [[docs/biology/processes]]

## Contract
- Base signal is local scalar stimulus, not typed command.
- Active emission needs registered process and physical/material cost.
- Signal has carrier, decay and readable-from Tick.
- Signal emitted in Tick N is readable no earlier than Tick N+1.
- Receiver needs compatible sensing Material.
- Debug communication traces are observer-only.

## Checks
- semantic command signal -> reject
- carrierless or free active signal -> reject
- same-tick receiver feedback -> reject
- receiver without sensing basis gets unavailable/zero input
