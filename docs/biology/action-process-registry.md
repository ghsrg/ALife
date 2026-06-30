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
implementation_level: base | future_compatible | later
duration: atomic | long_running
required_capabilities
allowed_genome_output
feasibility_scope
resource_cost
energy_cost
material_cost
result_state
failure_behavior
```

---

# Base Process Set

| process_id | kind | duration | allowed_genome_output | Notes |
| --- | --- | --- | --- | --- |
| `mandatory_upkeep` | passive | atomic | none | Paid before planned action Feasibility. |
| `resource_uptake` | planned_action | atomic | `resource_uptake_priority` | Uses local Resource and Boundary/transport capability. |
| `resource_export` | planned_action | atomic | `resource_export_priority` | Exports Resource or waste. |
| `energy_conversion` | controlled_reaction | atomic | `energy_conversion_priority` | Converts Resource/Field potential through Material/process. |
| `material_synthesis` | planned_action | atomic or long_running | `material_synthesis_priority` | Large synthesis must be marked long_running. |
| `basic_repair` | planned_action | atomic or long_running | `repair_priority` | Repairs Materials/Boundary through explicit cost. |
| `signal_emit` | planned_action | atomic | `signal_emit_priority` | Active signal has cost and carrier. |
| `movement_request` | planned_action | atomic | `movement_priority` | Physics resolves movement/collision. |
| `division_preparation` | planned_action | long_running | `division_preparation_priority` | Accumulates paid progress before partition. |
| `genome_copying` | planned_action | long_running | `genome_copying_priority` | Requires physical carrier synthesis. |
| `division_partition` | planned_action | atomic | `division_partition_priority` | Requires completed preparation and Feasibility. |
| `dormancy_shift` | planned_action | atomic | `dormancy_bias` | Changes activity mode if lifecycle allows. |
| `internal_rebalance` | planned_action | atomic | `internal_rebalance_priority` | Local redistribution without hidden creation. |

---

# Future-Compatible Processes

These may be referenced as future-compatible, but must not be treated as implemented base behavior unless enabled by this registry:

```text
joint_creation
joint_repair
joint_strengthening
hgt_fragment_uptake
genome_integration
recombination
advanced_learning_plasticity
specialized_structure_growth
fast_signal_conduction
```

---

# Rules

## Rule 1. Genome outputs are registered

Genome Runtime may output only priorities listed in this registry.

```text
Genome output -> ActionPlan candidate -> Feasibility -> Execution
```

Genome output names must not be invented in genetics documents without adding a registry entry.

## Rule 2. Feasibility accepts registered actions

Feasibility Check accepts only registered planned actions or controlled reactions.

Passive processes do not require Genome output, but still require explicit rules.

## Rule 3. Long-running processes are registered

A process may be long-running only if this registry marks it as `long_running`.

## Rule 4. Registry is canonical

No document may introduce a new executable process or Genome output outside this registry.

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
