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
- Space locality у першій реалізації вирішується через один uniform spatial grid contract.
- Cells є entities з position/radius і spatial grid index.
- Resources і local traces можуть бути grid або sparse-grid quantities.
- Fields sampled locally from grid або function layer.

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

Starting spatial partition:

```text
spatial_partition = uniform_spatial_grid

Cells      -> entity + spatial grid index
Joints     -> entity links between cells
Resources  -> grid / sparse grid
Fields     -> grid or function layer
Traces     -> grid / sparse grid
```

Chunks are future-compatible optimization for large worlds, streaming or loading. They are not part of the base spatial semantics.

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

Starting radii:

```text
cell_radius_default = 1.0 su
cell_radius_min = 0.5 su
cell_radius_max = 3.0 su
sense_radius = 4.0 su
uptake_radius = 1.5 su
contact_radius = cell_radius_a + cell_radius_b
joint_creation_radius = contact_radius + 0.5 su
signal_radius = 4.0 su
```

Starting grid size:

```text
spatial_grid_size = 8.0 su
```

Grid size має бути більшим за typical interaction radius, але не настільки великим, щоб одна grid cell містила забагато Cells.

---

# Boundary Modes

Boundary mode має явно описувати поведінку Cells, Resources і Fields.

Default:

```text
boundary_mode = solid_wall
```

`solid_wall` простіший для debug і не створює teleport-like ефектів на краях світу.

```text
wrapped:
  Cells, Resources and Fields wrap around world edges.

solid_wall:
  Cells collide/stop at boundary.
  Resources reflect, accumulate or stop according to resource rule.
  Fields are clamped or follow configured edge values.

open:
  Cells cannot leave unless explicit outflow/removal rule exists.
  Resources may leave and be removed by outflow.
  Fields may use external/ambient boundary values.
```

Invariant:

```text
Space locality is resolved through one spatial grid contract.
Cells are entities indexed by position.
Resources/traces are grid or sparse-grid quantities.
Fields are sampled locally.
Boundary mode must explicitly define behavior for Cells, Resources and Fields.
```

---

# Заборонено

Не вводити:

- distance-free interaction;
- global scan as cell sensing;
- ResourceType named Space;
- teleport movement;
- hidden infinite world;
- implicit boundary behavior;
- chunks/streaming as required base semantics.

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
