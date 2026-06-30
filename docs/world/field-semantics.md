# field-semantics.md

> **Field Semantics — як Fields стають локальними впливами без командної поведінки**

---

# Призначення

`field-semantics.md` визначає спільні правила для Light, Heat, Pressure, Radiation, Flow та майбутніх Fields.

Fields are not commands.

---

# Field Categories

## External Field

Глобальна або просторова умова світу.

## Local State

Стан клітини, матеріалу, ресурсу або environment patch.

## Derived Field

Поле, обчислене з локальних станів для rendering, debug або analytics.

## Behavior Input

Локально sampled значення, яке проходить через material capability або process mechanism.

---

# Field Effect Contract

A Field may affect a cell only if:

1. the cell locally samples it;
2. the cell has a Material or process capable of responding to it;
3. the effect passes through Feasibility, Process, Physics or Reaction rules.

---

# Examples

## Light

```text
LightField exists in world
    ↓
cell samples local light
    ↓
light-sensitive Material can use it
    ↓
energy conversion / signal / degradation modifier
```

## Heat

```text
reaction produces heat locally
    ↓
local temperature changes
    ↓
heat transfers by contact / Joint
    ↓
optional HeatField derived for visualization
```

Environment Heat transfer is optional explicit model, not automatic global command.

## Radiation

RadiationField can affect mutation risk or degradation only through explicit material/genome damage rules.

## Pressure

Pressure may be derived from crowding, collision or flow and affect boundary damage, Joint stress or movement resistance.

---

# Config Semantics

Recommended config block:

```yaml
field_semantics:
  direct_behavior_effects_allowed: false
  local_sampling_required: true
  material_capability_required: true
```

---

# Rules

## Rule 1. Field is not command

Field cannot directly execute behavior.

## Rule 2. Local sampling required

Cells read local sampled Field values, not global Field maps.

## Rule 3. Material mediation required

Field effect needs compatible Material, process, reaction or physics rule.

## Rule 4. Derived fields are read-only

Derived fields for rendering/debug do not automatically become behavior inputs.

---

# Пов'язані документи

- `world/fields.md`
- `world/energy.md`
- `world/physics.md`
- `world/reactions.md`
- `config/fields_config.md`

