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
  boundary_mode: closed
  tick:
    dt: 1.0
    max_ticks: null
  space:
    cell_radius_min: 0.5
    cell_radius_max: 8.0
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
base_capacity_per_area > 0
max_cells > 0
known boundary_mode
known dimensions
```

---

# Пов'язані документи

- `world/space.md`
- `world/tick.md`
- `world/tick-semantics.md`
- `engine/scheduler.md`
- `docs/examples/config-examples.md`
