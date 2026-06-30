# field-semantics.md

> Field Effect Contract — як Fields стають локальними впливами без командної поведінки.

---

# Призначення

`field-semantics.md` визначає спільний contract для Light, Heat, Pressure, Radiation, Flow, Chemical gradients та майбутніх Fields.

Field не є командою і не виконує поведінку напряму.

---

# Field Effect Contract

Для кожного Field або derived field-like effect потрібно явно описати:

```text
origin
propagation / decay
local sampling
effect mechanism
bounds
conserved or abstracted behavior
```

Cell, Material або Resource реагує на Field тільки через:

```text
material capability
reaction
physics rule
process
```

Field не може напряму створити Energy Buffer, damage, mutation, movement або behavior.

---

# Categories

## External Field

Глобальна або просторова умова світу.

## Local State

Стан Cell, Material, Resource patch або environment patch.

## Derived Field

Поле, обчислене з локальних станів для rendering, debug або analytics.

## Behavior Input

Локально sampled значення, яке проходить через material capability або process mechanism.

---

# Concrete Profiles

## Heat

Heat is a local physical effect represented through local temperature state and explicit transfer/dissipation rules.

Base fields:

```text
temperature
heat_capacity
heat_generated
heat_transfer_rate
heat_dissipation_rate
material_heat_tolerance
```

`temperature` is a local state of a Cell, Material, Resource patch or environment patch.

Reaction `heat_output` changes local temperature through `heat_capacity`.

```text
higher heat_capacity -> same heat output causes smaller temperature change
lower heat_capacity  -> same heat output causes larger temperature change
```

Heat may transfer only through explicit local mechanisms:

```text
contact
nearby environment patch
Joint with heat_transfer capability
```

If no global HeatField or full thermodynamic environment is modeled, local dissipation is allowed as explicit sink:

```text
temperature -> ambient_temperature by heat_dissipation_rate
```

This is a simplification representing heat escaping into unresolved environment.

Heat damage works only through Material tolerance/degradation thresholds:

```text
if local temperature exceeds material tolerance:
  material degradation risk increases
```

Heat is not Energy Buffer.

## Light

Light has intensity, propagation/occlusion, absorption and bounds.

Light can support Energy conversion only through photosensitive Material and explicit process/reaction.

Light does not directly create Energy Buffer.

## Pressure

Pressure arises from collision, crowding, flow or Joint/mechanical constraints.

Pressure affects cells through structural strength, elasticity, Boundary tolerance, Material degradation or physics rules.

Pressure does not directly damage HP because HP does not exist.

## Radiation

Radiation may affect mutation risk, Material degradation or Genome carrier damage only through explicit damage/mutation/degradation rules.

## Chemical Gradient

Chemical gradient is either Resource distribution or a derived Field.

Cells read only local samples through sensing Material.

## Flow

Flow changes movement or Resource transport only through physics rules.

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

## Rule 5. Field accounting is explicit

Every Field must define whether it is conserved, dissipated, clamped, derived or abstracted.

---

# Invariants

```text
Field effects require explicit mechanism and material/process/physics/reaction mediation.
Heat is not Energy Buffer.
Heat damage is Material degradation.
Heat transfer is local and explicit.
Heat dissipation must be configured, not hidden.
Reaction heat_output changes local temperature through heat_capacity.
```

---

# Пов'язані документи

- `world/fields.md`
- `world/energy.md`
- `world/physics.md`
- `world/reactions.md`
- `config/fields_config.md`
