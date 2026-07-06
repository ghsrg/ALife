---
tags:
  - alife
  - mechanics
  - agent/router
---

# Config -> Runtime

> Agent pre-flight card. Canon wins on conflict.

## Use When
- TOML schema or loading
- Resource, Material, Field or Reaction config
- defaults, validation or bounds
- runtime config structures

## Must Read
- [[docs/config/world_config]]
- [[docs/config/resources_config]]
- [[docs/config/materials_config]]
- [[docs/config/reactions_config]]
- [[docs/config/fields_config]]
- [[docs/config/stability_bounds]]
- [[docs/implementation/architecture]]

## Contract
- TOML is parsed into validated runtime config before simulation.
- References use known ids only.
- Config selects parameters; it does not invent hidden mechanics.
- Invalid values fail; risky values warn explicitly.
- A run keeps its config hash, seed and schema version.

## Checks
- unknown id -> reject
- negative rate, amount or capacity -> reject
- missing required relation -> reject
- runtime receives normalized bounded values
