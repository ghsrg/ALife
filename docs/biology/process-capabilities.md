---
tags:
  - alife
  - canon
  - area/biology
---

# process-capabilities.md

> **Process Capabilities — як Materials дозволяють процеси**

---

# Призначення

`process-capabilities.md` описує шар між Materials, Genome outputs, Feasibility Check і ProcessExecution.

---

# Canon Principle

```text
No process may execute without a material mechanism.
Material properties define what process capabilities are available.
Genome outputs only prioritize available capabilities.
```

---

# Capability Is Not Boolean

Capability is normalized level, not only true/false.

Low capability can mean:

- lower efficiency;
- higher energy cost;
- lower max intensity;
- higher failure risk;
- higher degradation risk.

If capability is `0.0`, corresponding process cannot execute even with high Genome priority.

---

# Runtime Flow

```text
cell materials
    ↓
derived capabilities
    ↓
allowed process set
    ↓
Genome priority can modulate only enabled processes
    ↓
Feasibility Check
    ↓
ProcessExecution
```

---

# Minimum Capability Matrix

| Material property | Enables process | Example process | Also requires |
| --- | --- | --- | --- |
| `boundary_support` | boundary upkeep | maintain boundary | Energy + boundary material |
| `permeability` | uptake/export | resource intake/export | local Resource + Energy optional |
| `energy_conversion_efficiency` | energy conversion | convert resource to Energy Buffer | Resource with energy_value |
| `repair_support` | repair | restore damaged material | repair Resources + Energy |
| `movement_support` | movement | movement impulse | Energy + space + physics |
| `signal_sensitivity` | sensing | read local signal | local signal/field |
| `signal_conductivity` | signal transfer | internal/joint signal | Energy optional |
| `joint_affinity` | joint creation/upkeep | create/maintain Joint | compatible material + neighbor |
| `resource_transport_support` | transport | move Resources internally/via Joint | Resource + Energy optional |
| `storage_capacity` | storage | hold Resource/Material | free volume |
| `field_sensitivity` | field sensing/conversion | react to light/heat/etc. | local Field + material |
| `structural_strength` | resist pressure/damage | passive resistance | no active cost, but degradation risk |

---

# Capability And Feasibility

Capability enables a process family.

Feasibility Check still verifies:

- Energy;
- Resources;
- Materials;
- Space;
- local conditions;
- lifecycle state;
- conflict resolution.

---

# Rules

## Rule 1. Genome cannot create capability

Genome can prioritize only enabled capabilities.

## Rule 2. Capability cannot bypass cost

Capability does not bypass Energy, Resources, Space or Lifecycle.

## Rule 3. Materials define function

Function comes from material properties, not cell class.

---

# Semantic Links

- enabled by: [[docs/world/materials|Materials]]
- allow: [[docs/biology/processes|Processes]]
- prioritized by: [[docs/biology/genome|Genome]]
- checked by: [[docs/biology/feasibility|Feasibility Check]]

# Пов'язані документи

- `world/materials.md`
- `biology/processes.md`
- `biology/feasibility.md`
- `genetics/regulatory-interface.md`
- `config/materials_config.md`

