---
tags:
  - alife
  - config
  - area/config
---

# world_config.md

> Конфігурація геометрії, часу, seed і базових меж світу.

---

# Призначення

`world_config` задає умови симуляції, які не належать конкретному Resource, Material, Field або Reaction.

---

# Мінімальна Схема

```yaml
world:
  seed: 1
  dimensions: 2
  size:
    width: 256
    height: 256
  boundary_mode: solid_wall
  tick:
    dt: 1.0
    max_ticks: null
  space:
    spatial_partition: uniform_spatial_grid
    spatial_grid_size: 8.0
    cell_radius_default: 1.0
    cell_radius_min: 0.5
    cell_radius_max: 3.0
    sense_radius: 4.0
    uptake_radius: 1.5
    joint_creation_extra_distance: 0.5
    signal_radius: 4.0
    base_capacity_per_area: 3.0
    max_cells: 10000
```

---

# Канонічні правила

- `seed` має забезпечувати відтворюваність.
- `dimensions` стартово `2`; схема не повинна блокувати future 3D.
- Space є обмеженням локальності й місткості, але не `ResourceType`.
- `dt` є одиницею модельного часу, а не optimization scheduler step.
- Boundary mode визначає поведінку країв світу.
- Default `boundary_mode` is `solid_wall`.
- Base spatial partition is `uniform_spatial_grid`.
- Cells are entities indexed by position; Resources/traces are grid or sparse-grid quantities; Fields are grid or function layer.
- `spatial_grid_size` має бути більшим за typical interaction radius, але не настільки великим, щоб одна grid cell містила забагато Cells.
- У 2D `volume_capacity` є abstract internal capacity, bounded by radius, footprint and storage-capable Materials.
- Starting footprint: `cell_area = π * radius²`.
- Starting capacity uses `base_capacity_per_area * cell_area * storage_material_modifier`.

---

# Validation

Перевіряти:

```text
width > 0
height > 0
dt > 0
cell_radius_min > 0
cell_radius_max >= cell_radius_min
cell_radius_default within min/max
spatial_grid_size > 0
sense_radius > 0
uptake_radius > 0
signal_radius > 0
known spatial_partition
base_capacity_per_area > 0
max_cells > 0
known boundary_mode: solid_wall | wrapped | open
known dimensions
```

---

# Semantic Links

- configures: [[docs/world/space|Space]]
- configures: [[docs/world/tick|Tick]]
- configures environment for: [[docs/world/physics|Physics]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]

# Пов'язані документи

- `world/space.md`
- `world/tick.md`
- `world/tick-semantics.md`
- `engine/scheduler.md`
- `docs/examples/config-examples.md`
