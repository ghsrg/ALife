# config-examples.md

> Приклади конфігів. Вони ілюстративні й не є окремим schema authority.

---

# Minimal World

```yaml
world:
  id: "smoke_world"
  seed: 42

space:
  type: "2d"
  width: 256
  height: 256
  boundary: "wrapped"
  spatial_grid_size: 8

time:
  tick_duration: 1.0
  max_ticks: 1000
```

---

# Minimal Resource

```yaml
resources:
  - id: "nutrient_A"
    density: 1.0
    volume_per_unit: 1.0
    diffusion_rate: 0.30
    stability: 0.80
    decay_rate: 0.001
    energy_value: 0.50
```

---

# Minimal Material

```yaml
materials:
  - id: "boundary_gel_A"
    density: 1.0
    volume_per_unit: 1.0
    stability: 0.80
    synthesis_cost:
      resources:
        nutrient_A: 1.0
      energy: 0.2
    physical:
      strength: 0.45
      elasticity: 0.70
      permeability: 0.30
      heat_resistance: 0.50
    functional:
      boundary_support: 0.85
      storage_capacity: 0.10
      energy_conversion_efficiency: 0.00
      signal_sensitivity: 0.10
      signal_conductivity: 0.05
      joint_affinity: 0.25
```

---

# Minimal Field

```yaml
fields:
  - id: "light"
    type: "scalar"
    default_value: 0.5
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "uniform"
    temporal_mode: "constant"
    diffusion: 0.0
    decay_rate: 0.0
```

---

# Minimal Reaction

```yaml
reactions:
  - id: "nutrient_energy_conversion"
    type: "controlled"
    inputs:
      resources:
        nutrient_A: 1.0
    required_capabilities:
      energy_conversion_efficiency: 0.1
    outputs:
      energy_delta: 0.5
      resources:
        inert_waste: 0.2
    heat_release: 0.05
```

