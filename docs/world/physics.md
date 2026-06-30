---
tags:
  - alife
  - canon
  - area/world
---

# physics.md

> `Physics` — концептуальні фізичні обмеження світу.

---

# Призначення

Цей файл описує доменні фізичні принципи світу. Технічна реалізація живе в `engine/physics.md`.

---

# Канонічні правила

- Об'єкти існують у Space і мають локальність.
- Interaction потребує proximity, contact, Field або матеріальний канал.
- Movement, pressure, collision і deformation мають physical constraints.
- Joint є матеріальним механічним/транспортним/сигнальним зв'язком.
- Heat не є Energy Buffer.
- На поточному етапі клітина має `temperature`, а Heat впливає локально на суміжні клітини/Joint без обов'язкового глобального environmental heat field.
- Heat transfer/dissipation і Material heat tolerance описані в `world/field-semantics.md`.
- Physics не знає cell roles, species або organism behavior.

---

# Мінімальні Поняття

```text
position
velocity
mass
volume
cell_area
volume_capacity
shape/radius
contact
pressure
temperature
heat_capacity
joint constraint
boundary/world limits
```

---

# Заборонено

Не вводити:

- distant physical action without carrier;
- teleportation;
- hardcoded biological movement;
- organism-level body controller;
- HP-like physical damage;
- species-specific physics.

---

# Semantic Links

- constrains: [[docs/world/space|Space]]
- constrains contact for: [[docs/biology/cell|Cell]]
- constrains mechanics of: [[docs/biology/joint|Joint]]
- constrains boundary behavior of: [[docs/biology/membrane|Boundary]]
- implemented by: [[docs/engine/physics|Engine Physics]]

# Пов'язані документи

- `world/space.md`
- `world/energy.md`
- `biology/joint.md`
- `engine/physics.md`
