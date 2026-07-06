---
tags:
  - alife
  - mechanics
  - agent/router
---

# Division -> Partition -> Inheritance

> Agent pre-flight card. Canon wins on conflict.

## Use When
- division preparation
- Genome copying or mutation
- daughter creation
- state partition and validation

## Must Read
- [[docs/biology/lifecycle]]
- [[docs/biology/division-partition]]
- [[docs/genetics/inheritance]]
- [[docs/genetics/mutation]]
- [[docs/biology/process-progress]]
- [[docs/biology/joint]]

## Contract
- Division is physical partition, not object cloning.
- Genome information needs a physical copied carrier before partition.
- Resources, Materials, Energy and MaterialState are partitioned with explicit costs/losses.
- RuntimeState and ProcessProgress are not copied by default.
- Joints are not duplicated or preserved by default.
- Daughter viability is evaluated, not guaranteed.

## Checks
- pre-partition failure creates no daughter
- copied Genome has matter source and Energy cost
- partition conserves accounted state
- daughter capacity and references validate
