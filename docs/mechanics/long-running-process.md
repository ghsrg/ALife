---
tags:
  - alife
  - mechanics
  - agent/router
---

# Long-Running Process

> Agent pre-flight card. Canon wins on conflict.

## Use When
- genome copying
- division preparation
- large synthesis or repair
- persistent work across Ticks

## Must Read
- [[docs/biology/action-process-registry]]
- [[docs/biology/process-progress]]
- [[docs/biology/processes]]
- [[docs/biology/feasibility]]
- [[docs/world/tick-semantics]]

## Contract
- Only registry-marked long-running processes use ProcessProgress.
- Each progress step passes Feasibility and pays explicit cost.
- Progress is persistent paid work, not partial final output.
- Pause, decay, cancel and completion rules are explicit.
- Final output appears only after completion validation and commit.

## Checks
- rejected action increases progress -> fail
- progress without owner/process/target -> reject
- partial final product before completion -> reject
- division inheritance of progress needs explicit rule
