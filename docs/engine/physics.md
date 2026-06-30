---
tags:
  - alife
  - engine
  - area/engine
---

# physics.md

> Physics system — локальна геометрія, contact, movement, pressure і Joint mechanics.

---

# Призначення

Physics відповідає на питання де щось існує, що поруч, що контактує, як рухається і які механічні обмеження діють.

Space semantics описані в `world/space.md`.

---

# Канонічні правила

- Physics не знає hardcoded biology roles.
- Movement і shape changes мають physical constraints.
- Joint mechanics застосовуються як локальні constraints/forces.
- Pressure/contact можуть бути inputs для клітин тільки через material basis.
- Heat/temperature effects застосовуються локально згідно з поточною спрощеною моделлю.
- Field effects, including Heat/temperature, follow `world/field-semantics.md`.
- Locality queries use the `uniform_spatial_grid` contract from `world/space.md` in the first implementation.
- Boundary behavior follows explicit world `boundary_mode`, with `solid_wall` as the default.

---

# Мінімальні Обов'язки

```text
spatial indexing
contact detection
boundary/world limits
movement integration
collision/pressure estimates
joint distance/force constraints
heat/contact transfer estimates
local neighborhood queries
```

---

# Заборонено

Не вводити:

- teleport movement;
- distant Joint creation;
- species-specific collision rules;
- organism body controller;
- physics shortcuts that create biological outcomes directly.

---

# Semantic Links

- implements: [[docs/world/physics|Physics]]
- resolves: [[docs/world/space|Space]]
- resolves contact for: [[docs/biology/cell|Cell]]
- resolves mechanics for: [[docs/biology/joint|Joint]]

# Пов'язані документи

- `world/space.md`
- `world/physics.md`
- `biology/joint.md`
- `biology/cell.md`
