---
tags:
  - alife
  - canon
  - area/genetics
---

# regulatory-interface.md

> **Regulatory Interface — межа між Genome Runtime, ActionPlan і Feasibility**

---

# Призначення

`regulatory-interface.md` визначає, що може читати Genome Runtime і що означає його output.

Genome output не є дією у світі.

---

# Core Rule

```text
Genome output != world action.
Genome output = regulatory intent / priority / modulation.
```

Genome Runtime never mutates world state directly.

All outputs pass through:

```text
Genome outputs
    ↓
priority normalization
    ↓
ActionPlan candidates
    ↓
Feasibility Check
    ↓
Execution / rejection
```

---

# Allowed Inputs

Inputs are local and normalized:

- local_energy_level;
- local_resource_levels;
- internal_resource_levels;
- material_state;
- boundary_state;
- damage/stress signals;
- local_field_values;
- local_signal_inputs;
- crowding/pressure;
- lifecycle_state;
- epigenetic_state.

---

# Forbidden Inputs

Forbidden:

- global population;
- species_id;
- fitness_score;
- absolute world map;
- target coordinates;
- neighbor genome;
- organism command;
- observer metrics.

---

# Output Scale

Genome output value:

```text
-1.0 .. +1.0
```

Meaning:

- `-1.0` suppress;
- `0.0` neutral;
- `+1.0` strongly prioritize.

High priority does not guarantee execution.

Allowed Genome outputs are defined in `biology/action-process-registry.md`.

Genome Runtime may output only registered priorities.

---

# Runtime Model

Direct Regulatory Graph may be bounded recurrent graph.

Runtime steps are fixed and small, recommended `3..8`.

State resets each Tick unless stored in explicit RuntimeState or EpigeneticState.

Allowed activation functions:

- `linear_clamped`;
- `sigmoid`;
- `threshold`.

Arbitrary functions in config/code are forbidden because they become hidden scripting language.

---

# Runtime Cost

Runtime cost should include:

```text
runtime_cost =
base_cost
+ node_count_cost
+ edge_count_cost
+ runtime_step_cost
```

Base cost can belong to mandatory existence cost for cells with active Genome Runtime. Complexity must have cost to prevent free unlimited graphs.

---

# Rules

## Rule 1. Runtime is regulatory

Genome Runtime computes regulatory outputs only.

## Rule 2. Outputs become candidates

Outputs become ActionPlan candidates, not world mutations.

## Rule 3. Feasibility is mandatory

Every candidate action passes Feasibility Check.

## Rule 4. Memory is explicit

Runtime memory requires explicit RuntimeState or EpigeneticState.

---

# Semantic Links

- translates outputs of: [[docs/genetics/genome-runtime|Genome Runtime]]
- binds to: [[docs/biology/action-process-registry|Action Process Registry]]
- cannot bypass: [[docs/biology/feasibility|Feasibility Check]]
- modulates: [[docs/biology/processes|Processes]]

# Пов'язані документи

- `biology/genome.md`
- `genetics/genome-runtime.md`
- `genetics/genome-representation.md`
- `genetics/regulatory-network.md`
- `biology/feasibility.md`
