---
tags:
  - alife
  - implementation
  - tools
  - stability
  - reachability
---

# Mechanism Reachability

> Second calibration stage after [[docs/implementation/early-stability-parameter-tuning|Early Stability Parameter Tuning]].

---

# Purpose

`early-stability-parameter-tuning` answers:

```text
Can the world remain numerically and locally viable inside tested parameter ranges?
```

`mechanism-reachability` answers:

```text
Are planned mechanisms actually reachable, useful and not bypassed by cheaper paths?
```

This document defines a future tool/reporting mode for detecting whether planned mechanics can activate and matter before we design the Rust data model and class/module documentation.

---

# Why This Exists

A world can be stable while important mechanisms are never used.

Examples:

```text
passive_energy_income covers all costs -> uptake is irrelevant
heat_dissipation always exceeds heat_generated -> heat mechanics never matters
capacity_limit is huge -> export/storage pressure never matters
passive diffusion solves resource flow -> active transport is never selected
Genome Runtime has no reason to request a process -> process remains dead code
Feasibility rejects an action every time -> mechanism exists but is unreachable
```

We do not want to implement complex mechanisms that the world never uses.

---

# Position In Workflow

The calibration loop is:

```text
1. Run early-stability parameter tuning.
2. Accept stable / fragile / collapse / invalid ranges.
3. Run mechanism reachability.
4. If mechanisms are reachable and useful -> proceed to data model/class documentation.
5. If mechanisms are blocked or bypassed -> return to parameter tuning.
6. Repeat while there are meaningful parameter ranges to move.
7. If no tuning can make a mechanism useful -> document model gap before implementation.
```

`mechanism-reachability` is not a replacement for stability tuning. It depends on it.

---

# Required Inputs

Minimum input:

```text
scenario_id
config_hash
seed
tick_count
accepted_stability_ranges_ref
mechanism_registry
mechanism_scenarios
```

Mechanism registry entry:

```text
mechanism_id
status: now | future | estimate_only
required_inputs
required_materials
required_energy
required_capacity
required_environment_conditions
expected_effect
possible_blockers
bypass_risks
minimum_useful_activation_rate
required_scenarios
```

---

# Mechanism Event Counters

Each mechanism should expose observer-only counters:

```text
available_count
attempted_count
allowed_count
executed_count
blocked_count
effect_nonzero_count
cost_paid_count
bypass_detected_count
survivor_used_count
```

Derived rates:

```text
attempt_rate = attempted_count / max(1, available_count)
allow_rate = allowed_count / max(1, attempted_count)
execution_rate = executed_count / max(1, allowed_count)
effect_rate = effect_nonzero_count / max(1, executed_count)
block_rate = blocked_count / max(1, attempted_count)
bypass_rate = bypass_detected_count / max(1, available_count)
```

These counters are observer-only. Cells, Genome Runtime, Feasibility and Processes must not read them.

---

# Block Reasons

Minimum block reason vocabulary:

```text
not_configured
missing_resource
missing_material
missing_capability
insufficient_energy
insufficient_capacity
boundary_blocked
threshold_not_reached
cooldown_active
process_not_registered
feasibility_rejected
no_gradient_or_driver
competing_path_cheaper
effect_zero
tool_limited
future_only
```

The report must count block reasons per mechanism.

---

# Bypass Detection

A mechanism is bypassed when the world stays stable through another path that makes the mechanism unnecessary.

Common bypasses:

```text
passive_energy_income bypasses uptake/metabolism pressure
high capacity bypasses export/storage pressure
high dissipation bypasses heat handling
high waste sink bypasses waste management
passive permeability bypasses active transport
free resource availability bypasses foraging/sensing
static estimates bypass growth/division process logic
```

Bypass is not always bad. It is bad when it makes a planned mechanism irrelevant in all scenarios.

---

# Minimum Useful Activation

Default thresholds for a mechanism to be considered useful in a scenario:

```text
available_count > 0
attempted_count > 0
allowed_count > 0
executed_count > 0
effect_nonzero_count > 0
block_rate < 0.95 unless this is a blocked scenario
bypass_rate < 0.8 unless this is a bypass scenario
```

Recommended minimum activation targets:

```text
smoke scenario: executed at least once
positive scenario: execution_rate >= 0.05
stress scenario: block reasons are meaningful and varied
negative scenario: absence or blocking worsens result
```

These are diagnostic thresholds, not Canon.

---

# Required Scenario Types Per Mechanism

Every non-trivial mechanism needs:

```text
positive scenario
  mechanism helps survival, growth, stability or recovery

negative scenario
  absence or blocking of mechanism worsens result

blocked scenario
  mechanism is attempted but rejected for a clear reason

bypass scenario
  cheaper path can bypass mechanism; report must detect this
```

If a mechanism lacks these scenarios, it is not ready for implementation beyond a stub/interface.

---

# Initial Mechanism List

Phase 1 / early Phase 2 candidates:

```text
mandatory_energy_cost
passive_energy_income
energy_buffer_clamp
capacity_limit
heat_generation
heat_dissipation
waste_generation
waste_sink
stress_state
dormancy
death_by_energy
death_by_heat
death_by_waste
candidate_config_validation
```

Later mechanisms:

```text
resource_uptake
resource_export
material_synthesis
repair
degradation
division
genome_runtime
mutation
inheritance
joint_creation
joint_upkeep
joint_resource_transfer
signal_emit
signal_receive
signal_trace
specialization_profile
organism_view
selection_metrics
```

Mechanisms marked `future` or `estimate_only` must not be reported as runtime-proven.

---

# Output Artifacts

Recommended output path:

```text
outputs/reachability/<run_id>/
```

Artifacts:

```text
results.json
REPORT.md
mechanisms.json
scenarios.json
block-reasons.json
bypass.json
```

`results.json` minimum:

```text
run_id
config_hash
seed
tick_count
stability_ranges_ref
overall_result: pass | warning | fail | blocked
mechanism_count
passed_count
warning_count
failed_count
blocked_count
tool_limited_count
```

`mechanisms.json` row shape:

```text
mechanism_id
status
scenario_id
available_count
attempted_count
allowed_count
executed_count
blocked_count
effect_nonzero_count
bypass_detected_count
top_block_reason
reachability_result
notes
```

Reachability result values:

```text
pass
warning
fail
blocked
tool_limited
future_only
```

---

# Report Requirements

`REPORT.md` must include:

```text
run summary
linked stability tuning run
accepted parameter ranges used
mechanism table
failed mechanisms
blocked mechanisms
bypassed mechanisms
tool-limited mechanisms
recommended tuning adjustments
whether to proceed to data model design
```

Required decision line:

```text
Proceed to data model docs: yes | no | partial
```

If `no` or `partial`, report must say which parameter tuning group to revisit.

---

# Link Back To Parameter Tuning

When reachability fails, return to [[docs/implementation/early-stability-parameter-tuning|Early Stability Parameter Tuning]] with a concrete tuning request.

Example:

```text
Mechanism: resource_uptake
Failure: bypassed by passive_energy_income
Suggested tuning group: Energy Budget Sweep
Move: reduce passive_energy_income_placeholder or introduce resource scarcity scenario
Expected change: uptake attempted_count > 0 and effect_nonzero_count > 0
```

Stop the loop if:

```text
3 reachability/tuning cycles show no meaningful improvement
required mechanism needs unimplemented model behavior
only way to pass is to violate Canon
negative scenarios stop failing
```

---

# Semantic Links

- follows: [[docs/implementation/early-stability-parameter-tuning|Early Stability Parameter Tuning]]
- supports: [[docs/implementation/phase-1-design|Phase 1 Design]]
- prepares: [[docs/implementation/architecture|Architecture]]
- uses: [[docs/biology/action-process-registry|Action Process Registry]]
- uses: [[docs/engine/feasibility|Feasibility]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
