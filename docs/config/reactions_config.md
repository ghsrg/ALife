# reactions_config.md

> **Reactions Config — конфігурація passive і controlled reaction rules**

---

# Призначення

`reactions_config.md` описує формат balanced reaction rules.

Reaction semantics описані в `world/reactions.md`.

---

# Що Reactions Config НЕ є

Не описує:

- hardcoded biology;
- toxicity shortcuts;
- direct kill commands;
- automatic Energy Buffer charging.

---

# Basic Schema

```yaml
reactions:
  - id: "nutrient_oxidation_A"
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

# Passive Reaction Example

```yaml
reactions:
  - id: "organic_waste_decay"
    type: "decay"
    inputs:
      organic_waste: 1.0
    products:
      mineral_A: 0.4
      inert_waste: 0.4
    heat_release: 0.02
    rate: 0.01
    probability: 1.0
```

---

# Controlled Reaction Example

Controlled reaction requires ActionPlan and Feasibility Check.

```yaml
reactions:
  - id: "cell_energy_conversion_A"
    type: "controlled"
    inputs:
      nutrient_A: 1.0
    catalyst:
      material_id: "energy_conversion_polymer"
      min_amount: 0.2
    energy_release: 0.8
    heat_release: 0.1
    rate: 0.25
    probability: 0.9
```

---

# Validation

Validation must check:

- reaction ids are unique;
- referenced Resources exist;
- referenced Materials exist;
- rates are per Tick;
- probabilities are normalized;
- outputs do not create matter without inputs unless explicit source rule exists;
- no `toxicity: true` shortcut.

---

# Rules

## Rule 1. Config defines reaction rules

New reactions are added by config, not by hardcoded biological cases.

## Rule 2. Reactions reference existing types

Reaction inputs, outputs and catalysts must reference valid Resource/Material IDs.

## Rule 3. No magical Energy

Energy release is not automatic Energy Buffer.

---

# Пов'язані документи

- `world/reactions.md`
- `engine/chemistry.md`
- `config/resources_config.md`
- `config/materials_config.md`
- `config/stability_bounds.md`

