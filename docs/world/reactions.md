# reactions.md

> **Reactions — універсальна семантика перетворень Resources, Materials, Heat і Energy release**

---

# Призначення

`reactions.md` описує reaction semantics.

Це не повний науковий chemistry module. Це contract для passive і controlled reactions.

---

# Reaction Contract

Reaction happens if:

- inputs are present;
- conditions are satisfied;
- catalyst/material exists if required;
- rate/probability allows it;
- location/locality allows interaction.

Example:

```yaml
reaction:
  id: "nutrient_oxidation_A"
  type: "controlled"
  inputs:
    nutrient_A: 1.0
    oxidizer_A: 0.5
  conditions:
    heat:
      min: 0.2
      max: 0.8
  catalyst:
    material_id: "conversion_polymer_A"
    min_amount: 0.2
  products:
    waste_A: 0.6
  energy_release: 0.8
  heat_release: 0.1
  rate: 0.25
  probability: 0.8
```

---

# Reaction Types

- `passive`
- `controlled`
- `degradation`
- `decay`
- `synthesis`
- `conversion`

## Passive reactions

Environment-driven.

Do not require Genome priority.

Examples:

- organic_waste decays into mineral Resource;
- unstable Resource breaks down under heat;
- reactive Resources produce blocking waste.

## Controlled reactions

Cell process-driven.

Require:

- Genome/ActionPlan priority;
- Feasibility Check;
- compatible Material/capability;
- cost and local conditions.

Examples:

- Energy conversion;
- Material synthesis;
- repair material production;
- signal substance production.

---

# Energy Release

`reaction.energy_release` does not automatically charge any cell's Energy Buffer.

Energy Buffer increases only through controlled conversion process with compatible Material and passed Feasibility Check.

Passive reaction may release Heat or products, but not magical cell Energy.

---

# Reaction Accounting Contract

Reactions may simplify chemistry, but they must not silently create or destroy matter.

Resources and Materials are accounted in simulation amount units, not strict SI mass units.

Every reaction must explicitly describe where input matter goes:

```text
inputs
  -> products
  -> retained/internalized material
  -> residual/waste
  -> configured sink/loss
```

`energy_output` represents released or captured energy potential from input Resources/Materials.

It does not replace material outputs and does not explain missing matter.

Configured loss is allowed only when explicitly modeled as:

```text
outflow
degradation sink
evaporation-like removal
radiation-like escape
scenario-defined sink
```

Hidden disappearance is not allowed.

Validation should:

```text
warn when input/output accounting is not balanced
warn when part of input matter has no explicit destination
fail when reaction creates products without inputs
fail when unknown Resources/Materials are used
fail when core invariants are violated
```

Invariant:

```text
Energy is not matter.
Reaction products must have material sources.
Configured loss must be explicit.
Unaccounted Resources or Materials are invalid or at least a validation warning.
```

---

# Poisoning-by-Reaction

There is no `toxicity` field.

Harmful effects come from:

- bad products occupying volume;
- blocking transport;
- material damage;
- local condition changes;
- reactions with useful Resources;
- Heat release;
- acidity-like or pressure-like derived effects if modeled.

---

# Rules

## Rule 1. No hidden effects

No Resource may produce Energy, damage, poisoning, synthesis or degradation effect without explicit reaction rule or material/process mechanism.

## Rule 2. Controlled reactions require process

Controlled reactions require ActionPlan and Feasibility Check.

## Rule 3. Passive reactions are environment-driven

Passive reactions do not require Genome decision.

## Rule 4. Reactions are local

Reaction candidates come from Locality Contract in `space.md`.

## Rule 5. Material accounting is explicit

Reaction inputs must have explicit products, retained state, residual/waste, or configured sink/loss.

---

# Пов'язані документи

- `world/resources.md`
- `world/materials.md`
- `world/energy.md`
- `world/field-semantics.md`
- `biology/processes.md`
- `engine/chemistry.md`
- `config/reactions_config.md`
