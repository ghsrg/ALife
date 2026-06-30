---
tags:
  - alife
  - canon
  - area/biology
---

# cell.md

> `Cell` — базова локальна одиниця життя у світі.

---

# Призначення

`Cell` є обмеженим фізичним об'ємом, у якому поєднуються Resources, Materials, Energy Buffer, Genome Runtime, local signals, lifecycle і cell processes.

Клітина не є готовим типом організму, органом, тканиною, нейроном, сенсором, хижаком, рослиною або твариною. Такі ролі можуть виникати лише як emergent стани через матеріали, регуляцію, Joint, signals і selection.

---

# Мінімальний Стан

```text
Cell
├── id
├── position
├── physical_state
├── resources
├── materials
├── energy_buffer
├── genome_state
├── epigenetic_state
├── runtime_state
├── joints
├── local_inputs
├── lifecycle_state
└── process_progress
```

Функціональні стани `active`, `stalled`, `dormant`, `inert`, `decomposing` і `persistent_remains` описані в `biology/cell-state.md`.

---

# Канонічні правила

- Клітина читає тільки локальні inputs.
- `id` є технічним і не визначає species або behavior.
- Energy Buffer є локальним станом клітини, не Resource і не Material.
- Energy Buffer не передається напряму між незалежними клітинами.
- Boundary є агрегованою властивістю Materials, а не магічною оболонкою.
- Внутрішній об'єм обмежує Resources, Materials, Genome fragments і internal fragments.
- Genome регулює process priorities, але не виконує фізичну роботу.
- Materials визначають capabilities; Genome не може обійти відсутність матеріальної основи.
- Клітина не має глобального знання про світ, organism identity або “своїх/чужих”.
- Смерть є втратою мінімальної життєздатності, а не HP event.

---

# Capacity

```text
used_capacity =
resources_volume
+ materials_volume
+ genome_volume
+ internal_fragments_volume

free_capacity =
total_capacity - used_capacity
```

Якщо `free_capacity` недостатній, клітина не може нормально uptake, synthesize, repair, grow або divide.

Energy Buffer не займає об'єм напряму, але його capacity визначається Materials, які об'єм займають.

У 2D base model `volume_capacity` є abstract internal capacity unit, not SI volume.

Starting relation:

```text
cell_area = π * radius²

volume_capacity =
  base_capacity_per_area
  * cell_area
  * storage_material_modifier
```

Storage-capable Materials may increase effective capacity.

Non-storage Materials may occupy capacity without increasing storage.

If stored amount exceeds capacity, lifecycle/physics rules must handle rejection/export, pressure/stress, growth, instability or division preparation.

---

# Tick Loop

```text
1. Read local environment.
2. Read internal state.
3. Run Genome Runtime.
4. Build planned actions.
5. Run Feasibility Check.
6. Execute allowed actions.
7. Update lifecycle and persistent state.
```

Деталі порядку належать `world/tick.md`, `world/tick-semantics.md` і `engine/scheduler.md`.

---

# Заборонено

Не вводити:

- hardcoded cell roles;
- species_hash or species_id behavior;
- direct organism-level command;
- direct Energy Buffer transport;
- global world knowledge;
- HP;
- process without materials/resources/energy when required;
- magic detox або poison damage поза reaction/material model.

---

# Open Questions

- Мінімальний набір fields для першої `Cell` entity.
- Чи Boundary у першій реалізації буде агрегованою властивістю Materials або окремим component.
- Який мінімальний `runtime_state` потрібен для signal accumulation і cooldowns.
- Які metrics зберігати для debug без впливу на behavior.

---

# Semantic Links

- contains: [[docs/world/resources|Resources]]
- contains: [[docs/world/materials|Materials]]
- stores: [[docs/world/energy|Energy Buffer]]
- bounded by: [[docs/biology/membrane|Boundary]]
- regulated by: [[docs/biology/genome|Genome]]
- connected through: [[docs/biology/joint|Joint]]
- progresses through: [[docs/biology/lifecycle|Lifecycle]]

# Пов'язані документи

- `biology/cell-state.md`
- `biology/feasibility.md`
- `biology/processes.md`
- `biology/process-progress.md`
- `biology/process-capabilities.md`
- `biology/lifecycle.md`
- `biology/joint.md`
- `biology/communication.md`
- `biology/specialization.md`
- `genetics/genome-runtime.md`
- `world/energy.md`
- `world/materials.md`
- `world/resources.md`
- `world/space.md`
