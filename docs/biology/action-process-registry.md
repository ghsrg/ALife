---
tags:
  - alife
  - canon
  - area/biology
---

# action-process-registry.md

> Action / Process Registry — canonical source of process ids, Genome output bindings, Feasibility scope and process duration.

---

# Призначення

Цей документ є єдиним реєстром executable processes, planned actions, controlled reactions і Genome output bindings.

Інші документи можуть пояснювати concepts, але не повинні вводити independent executable process/output lists.

---

# Registry Fields

Кожен process entry має визначати:

```text
process_id
kind: passive | planned_action | controlled_reaction
status: now | future
duration: atomic | long_running
required_capabilities
required_inputs
energy_cost
material_cost
output/effect
feasibility_rules
failure_modes
```

Optional binding fields may include:

```text
allowed_genome_output
notes
```

Only entries with `status: now` are executable. `status: future` entries may exist for schema compatibility, but must not be accepted by Genome Runtime or Feasibility until explicitly enabled.

---

# Base Process Set

| process_id | kind | status | duration | allowed_genome_output | required_capabilities | required_inputs | energy_cost | material_cost | output/effect | feasibility_rules | failure_modes |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `mandatory_upkeep` | passive | now | atomic | none | stability-capable Materials | live Cell state | mandatory | maintenance resources if configured | maintains minimal stability | paid before planned actions | `cell_stressed`, `material_degraded`, `heat_generated` |
| `resource_uptake` | planned_action | now | atomic | `resource_uptake_priority` | Boundary permeability or transport capability | local Resource, free internal capacity | uptake cost if active | none unless carrier/process consumes Material | Resource enters Cell | Boundary allows passive or active_required; capacity exists | `rejected_no_effect`, `resource_wasted`, `heat_generated` |
| `resource_export` | planned_action | now | atomic | `resource_export_priority` | Boundary export capability or passive leak | internal Resource, local external capacity | export cost if active | none unless carrier/process consumes Material | Resource leaves Cell | Resource exists; Boundary/export rule allows it | `rejected_no_effect`, `resource_wasted`, `heat_generated` |
| `energy_conversion` | controlled_reaction | now | atomic | `energy_conversion_priority` | conversion-capable Material | Resource/Field potential | none or catalyst upkeep | consumed Resource, catalyst wear if configured | Energy Buffer and/or Heat | inputs exist; conversion pathway allowed | `rejected_no_effect`, `resource_wasted`, `heat_generated`, `cell_stressed` |
| `material_synthesis` | planned_action | now | atomic | `material_synthesis_priority` | synthesis-capable Material | precursor Resources, free capacity | synthesis cost | precursor Materials/Resources | new or repaired Material amount | inputs and capacity exist | `rejected_no_effect`, `resource_wasted`, `heat_generated`, `partial_progress_not_added` |
| `basic_repair` | planned_action | now | atomic | `repair_priority` | repair-capable Material | damaged Material, repair inputs | repair cost | repair Resources/Materials | lower damage / higher integrity | target exists; repair path allowed | `rejected_no_effect`, `resource_wasted`, `heat_generated`, `material_degraded` |
| `signal_emit` | planned_action | now | atomic | `signal_emit_priority` | signal-emitting Material | signal level, carrier/contact/joint/trace path | signal cost | trace Resource if emitted | scalar `signal_level` emitted | medium exists; output bounded | `rejected_no_effect`, `heat_generated` |
| `movement_request` | planned_action | now | atomic | `movement_priority` | movement-capable Material | direction/bias, local space | movement cost | wear if configured | movement intent for physics | space/collision rules allow resolution | `rejected_no_effect`, `heat_generated`, `material_degraded` |
| `division_preparation` | planned_action | now | long_running | `division_preparation_priority` | growth/division-capable Materials | resources, energy, capacity | per-progress cost | division prep Materials | paid division progress | no partial final result; progress rules apply | `rejected_no_effect`, `progress_paused`, `progress_decayed`, `partial_progress_not_added` |
| `genome_copying` | planned_action | now | long_running | `genome_copying_priority` | genome-copy-capable Material | genome carrier, copy precursors | per-progress cost | carrier Material/precursors | copied Genome carrier | carrier integrity and inputs valid | `rejected_no_effect`, `progress_paused`, `progress_decayed`, `resource_wasted`, `heat_generated` |
| `division_partition` | planned_action | now | atomic | `division_partition_priority` | division-capable Materials | completed prep, copied genome carrier | partition cost | partitioned Materials/Resources | two daughter cells | prep complete; carrier exists; space/capacity valid | `rejected_no_effect`, `material_degraded`, `cell_stressed` |
| `dormancy_shift` | planned_action | now | atomic | `dormancy_bias` | lifecycle regulation capability | lifecycle state | transition cost if configured | none | activity mode changes | lifecycle allows transition | `rejected_no_effect` |
| `internal_rebalance` | planned_action | now | atomic | `internal_rebalance_priority` | internal transport capability | internal Resources/Materials | rebalance cost if active | none | local internal redistribution | no hidden creation; capacity valid | `rejected_no_effect`, `heat_generated` |

Long-running synthesis or repair variants require separate registry entries with separate `process_id` values.

---

# Future-Compatible Processes

These may be referenced as future-compatible, but must not be treated as implemented base behavior unless enabled by this registry:

```text
HGT / genome integration
recombination
advanced_learning_plasticity
fast_signal_conduction
complex_joint_remodeling
specialized_structure_growth
long_range_sensing
multi_cell_coordinated_development
```

Future entries must use `status: future` and must not expose an allowed Genome output or Feasibility action until promoted to `status: now`.

---

# Rules

## Rule 1. Genome outputs are registered

Genome Runtime may output only priorities listed in this registry.

```text
Genome output -> ActionPlan candidate -> Feasibility -> Execution
```

Genome output names must not be invented in genetics documents without adding a registry entry. Future entries do not become allowed outputs until their status is `now`.

## Rule 2. Feasibility accepts registered actions

Feasibility Check accepts only registered planned actions or controlled reactions.

Passive processes do not require Genome output, but still require explicit rules.

Feasibility must reject `status: future` actions with `rejected_no_effect`.

## Rule 3. Long-running processes are registered

A process may be long-running only if this registry marks it as `long_running`.

If an action needs both atomic and long-running variants, they must be separate registry entries with separate `process_id` values.

## Rule 4. Registry is canonical

No document may introduce a new executable process or Genome output outside this registry.

## Rule 5. Failure modes are explicit

Each executable process must define `failure_modes`. If Feasibility rejects before execution, the result is `rejected_no_effect`. If failure happens during execution, the process rule must state the concrete consequences.

---

# Semantic Links

- registers: [[docs/biology/processes|Processes]]
- binds outputs from: [[docs/genetics/regulatory-interface|Regulatory Interface]]
- constrains: [[docs/genetics/regulatory-network|Regulatory Network]]
- used by: [[docs/biology/feasibility|Feasibility Check]]

# Пов'язані документи

- `biology/processes.md`
- `biology/feasibility.md`
- `biology/process-progress.md`
- `biology/process-capabilities.md`
- `genetics/regulatory-interface.md`
- `genetics/regulatory-network.md`
