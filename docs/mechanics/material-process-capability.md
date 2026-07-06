---
tags:
  - alife
  - mechanics
  - agent/router
---

# Material -> Capability -> Process

> Agent pre-flight card. Canon wins on conflict.

## Use When
- adding Material properties
- enabling a process family
- deriving Cell capability
- checking specialization mechanics

## Must Read
- [[docs/world/materials]]
- [[docs/biology/process-capabilities]]
- [[docs/biology/processes]]
- [[docs/biology/feasibility]]
- [[docs/config/materials_config]]

## Contract
- Materials define physical and functional capabilities.
- Capability enables a process family; it is not the process itself.
- Genome may prioritize only available registered processes.
- Capability level may change efficiency, cost, rate or failure risk.
- Capability cannot bypass inputs, Energy, space, lifecycle or locality.

## Checks
- process without material mechanism -> reject
- Genome-created capability -> reject
- capability id/property is configured and bounded
- context supports the capability
