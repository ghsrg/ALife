---
tags:
  - alife
  - canon
  - area/world
---

# energy.md

> `Energy` — локальний буфер здатності клітини виконувати активну роботу.

---

# Призначення

Energy Buffer є локальним станом клітини. Він не є Resource і не є Material.

---

# Канонічні правила

- Energy Buffer is a local state of a Cell.
- Energy Buffer не є Resource, Material або transferable substance.
- Energy Buffer не займає internal volume напряму.
- Energy capacity задається storage-capable Materials, структурою та internal organization клітини.
- Energy Buffer не передається напряму між незалежними клітинами.
- Energy Buffer може partition-итися під час division як локальний стан parent cell.
- Energy production потребує Resource/Field + Material/process/reaction.
- Active processes потребують Energy.
- Mandatory costs are paid before planned action Feasibility.
- Якщо Energy недостатньо для planned actions як набору, planned actions не виконуються в цьому Tick.
- Mandatory costs обробляються окремо.
- Excess/inefficiency може створювати Heat.
- Heat не є Energy Buffer.

---

# Мінімальні Властивості

```text
energy_current
energy_capacity
mandatory_costs
planned_action_costs
production_rate
consumption_rate
heat_side_effect
```

`energy_capacity` визначається Materials клітини.

---

# Mandatory Costs

Mandatory costs are the cost of remaining an alive cell in current local conditions.

They may include:

```text
minimal boundary stability
basic internal stability
Genome Runtime baseline cost if configured
other required existence costs
```

Planned actions may use only Energy, Resources, Materials, volume and process capacity that remain after mandatory costs.

```text
available_for_planned_actions =
  committed_state_after_mandatory_costs
```

If mandatory costs cannot be paid, planned actions are skipped or rejected for that Tick. The cell may become stalled, damaged, degrading, dormant or inert according to lifecycle rules.

---

# Заборонено

Не вводити:

- treating Energy Buffer as Resource;
- treating Energy Buffer as Material;
- direct Energy transfer through Joint;
- free Energy from Field;
- action execution without sufficient Energy;
- hidden priority from action iteration order.

---

# Semantic Links

- powers: [[docs/biology/processes|Processes]]
- gates: [[docs/biology/feasibility|Feasibility Check]]
- stored in: [[docs/biology/cell|Cell]]
- produced through: [[docs/world/reactions|Reactions]]
- capacity depends on: [[docs/world/materials|Materials]]

# Пов'язані документи

- `biology/processes.md`
- `biology/feasibility.md`
- `biology/division-partition.md`
- `world/reactions.md`
- `world/physics.md`
