---
tags:
  - alife
  - canon
  - area/world
---

# space.md

> `Space` — простір, локальність і межі світу.

---

# Призначення

Space відповідає на питання:

```text
де щось існує?
як світ поділений?
які межі?
як визначається локальність?
що означає "поруч"?
як працюють координати?
```

Space не є `ResourceType`. Це обмеження й структура існування об'єктів.

---

# Канонічні правила

- У першій моделі світ 2D, але схема не повинна блокувати future 3D.
- Кожна Cell, Joint, Resource, Field value, Trace або Fragment має spatial context.
- Locality визначає sensing, contact, diffusion, transfer, Joint creation і physics.
- Resource/Field/Trace можуть мати різні spatial representations, але мають бути прив'язані до координат або region.
- Boundary mode світу має бути явним.

---

# Мінімальна Модель

```text
WorldSpace
├── dimensions
├── width
├── height
├── coordinate_system
├── boundary_mode
├── spatial_partition
└── neighborhood_radius rules
```

---

# 2D Footprint And Capacity

У 2D base model `volume_capacity` є internal simulation capacity unit, not SI volume.

It must be bounded by physical footprint and storage-capable Materials.

```text
2D radius -> physical footprint
Materials -> storage structure
volume_capacity -> bounded internal capacity
```

Starting footprint:

```text
cell_area = π * radius²
```

Starting capacity placeholder:

```text
volume_capacity =
  base_capacity_per_area
  * cell_area
  * storage_material_modifier
```

This is a placeholder contract, not final physics.

`cell_area` is not automatically equal to `volume_capacity`.

A small Cell cannot contain unlimited matter.

---

# Locality

`nearby` не повинно бути неявним. Для кожного механізму потрібен radius або contact rule:

```text
sensing_radius
uptake_radius
contact_radius
joint_creation_distance
diffusion_neighborhood
physics_collision_radius
```

---

# Заборонено

Не вводити:

- distance-free interaction;
- global scan as cell sensing;
- ResourceType named Space;
- teleport movement;
- hidden infinite world;
- implicit boundary behavior.

---

# Open Questions

- Який стартовий spatial partition потрібен: grid, chunks або інший index.
- Які default radii входять у першу конфігурацію.
- Як boundary mode впливає на Resources, Fields і Cells.

---

# Semantic Links

- constrains locality of: [[docs/biology/cell|Cell]]
- constrains movement of: [[docs/world/resources|Resources]]
- constrains geometry of: [[docs/biology/joint|Joint]]
- implemented by: [[docs/engine/physics|Engine Physics]]
- configured by: [[docs/config/world_config|World Config]]

# Пов'язані документи

- `world/physics.md`
- `world/resources.md`
- `world/fields.md`
- `docs/config/world_config.md`
- `engine/physics.md`
- `engine/performance.md`
