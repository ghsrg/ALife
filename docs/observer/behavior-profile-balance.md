---
tags:
  - alife
  - observer
  - behavior-profile
  - balance
  - analyzer
---

# Behavior Profile Balance

Behavior Profile Balance is the Observer-level interpretation above mechanism coverage.

It answers a different question:

```text
not only: is the mechanism tested?
but also: do derived survival styles stay balanced under equal requirements?
```

Survival styles are not Canon organism types. They are observer-only labels inferred from measured process, material, cost and benefit patterns.

Cells cannot read survival style labels.

## Analysis Layers

The analyzer should separate three layers:

```text
Mechanism Coverage
  -> mechanism exists, activates and has metrics

Behavior Profile
  -> measured usage pattern across processes, Materials, Resources and state changes

Balance Finding
  -> comparison of profiles under equal requirements
```

`Mechanism Coverage` is a gate. `Behavior Profile` is a derived summary. `Balance Finding` is the final human-facing conclusion.

## Behavior Profile

A behavior profile is built from measured data, not from predefined species or organism classes.

Profile inputs:

```text
process_usage_profile
material_usage_profile
resource_usage_profile
energy_cost_profile
heat_waste_profile
movement_profile
growth_profile
repair_profile
survival_profile
environment_context
```

Initial observer-only style labels may include:

```text
storage-heavy
fast-growth
movement-foraging
repair-heavy
metabolism-efficient
heat-tolerant
waste-tolerant
dormancy-survival
balanced-generalist
```

These labels are fuzzy analysis labels. They must not become Core entities, config categories, Cell classes or Genome inputs.

## Equal Requirements

Balance comparison must only compare styles under explicitly equalized requirements.

Required comparison context:

```text
same initial energy
same initial materials/resources when relevant
same world size and boundary mode
same resource density or patch model
same mandatory cost
same tick count
same hazard level
same seed set or declared seed policy
same enabled mechanism set
same phase/runtime version
```

If requirements are not equalized, the report may describe observations but must not claim imbalance.

## Balance Finding

The analyzer should produce a finding when one behavior profile dominates another or fails to pay an expected trade-off.

Minimum finding fields:

```text
finding_id
compared_profiles
equal_requirements
result
evidence_metrics
dominance_rate
affected_scenarios
suspected_cause
recommendation
recommended_reruns
confidence
```

Allowed `result` values:

```text
balanced
not_balanced
inconclusive
insufficient_coverage
```

Example:

```text
compared_profiles:
  - storage-heavy
  - fast-growth

equal_requirements:
  - same initial energy
  - same resource density
  - same mandatory cost
  - same tick count
  - same hazard level

result:
  not_balanced

evidence:
  survival_ticks +41%
  collapse_rate -35%
  division_readiness -8%
  energy_cost similar

suspected_cause:
  storage has capacity benefit but weak upkeep/growth penalty

recommendation:
  increase storage material upkeep
  or add growth penalty from stored mass
  rerun Storage x Growth and Storage x Upkeep matrices
```

## Required Outputs

In addition to mechanism coverage artifacts, the analyzer should eventually produce:

```text
outputs/raw_data/behavior_profiles.csv
outputs/reports/behavior-profiles-<timestamp>.json
outputs/reports/behavior-profiles-<timestamp>.md
outputs/reports/balance-findings-<timestamp>.json
outputs/reports/balance-findings-<timestamp>.md
```

The current implementation may produce only mechanism coverage first. A report that lacks profile balance conclusions must state that it is coverage-only.

## Interpretation Rules

- A mechanism can be covered while the survival style using it is unbalanced.
- A style cannot be evaluated if its underlying mechanisms are not covered.
- A style label is only valid for the scenario family and equal requirements used in the comparison.
- A dominant style is not automatically a bug; it is a balance finding that needs evidence and recommended reruns.
- If parameter tuning cannot restore trade-offs, the report should recommend a mechanic change.

## Semantic Links

- base contract: [[docs/observer/observer-layer|Observer Layer]]
- mechanism gate: [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]]
- evolution interpretation: [[docs/evolution/selection|Selection]]
- adaptation interpretation: [[docs/evolution/adaptation|Adaptation]]
- source plan: [[outputs/worklogs/2026-07-04-1405-PLAN-sweep_scenario-eval-coverage_refactor|Sweep Scenario Eval Coverage Refactor]]
