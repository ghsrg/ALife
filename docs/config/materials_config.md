---
tags:
  - alife
  - config
  - area/config
---

# materials_config.md

> Конфігурація структурних і функціональних матеріалів.

---

# Призначення

`materials_config` описує MaterialType: те, з чого складається клітина, Boundary, Joint і внутрішні функціональні стани.

Material надає capabilities, але не є готовою поведінкою.

---

# Мінімальна Схема

```yaml
materials:
  boundary_polymer:
    volume: 1.0
    stability: 0.8
    strength: 0.7
    energy_capacity: 0.0
    boundary:
      default_permeability: blocked
      permeability_by_resource_class:
        tiny_neutral: passive
        small_nutrient: active_required
        structural_precursor: active_required
        waste_small: passive
        large_fragment: blocked
        reactive: blocked
      permeability_by_resource_id:
        water_like_A: passive
        nutrient_A: active_required
        waste_A: passive
        mineral_A: active_required
        genetic_fragment_A: blocked
    capabilities:
      boundary: 1.0
      joint_affinity: 0.2
    signal:
      signal_sensitivity: 0.0
      signal_storage: 0.0
      signal_conductivity: 0.0
    decay_rate: 0.001
```

---

# Канонічні правила

- Materials визначають capabilities для processes.
- Genome може регулювати synthesis/repair/use Material, але не створює capability без Material.
- Boundary і Joint мають матеріальну основу.
- Stateful Materials можуть підтримувати signal-plastic behavior.
- Materials займають capacity і можуть деградувати.
- External MaterialFragment retains material identity but does not provide active cell capabilities outside Cell/Joint context.
- Boundary Material rules define what Resource classes or ids are `blocked`, `passive` or `active_required`.
- Boundary Material default permeability is `blocked`.
- Signal-plastic Materials use `signal_sensitivity`, `signal_storage` and `signal_conductivity`; they do not create hardcoded neurons.

---

# Validation

```text
id unique
volume >= 0
0 <= stability
0 <= strength
energy_capacity >= 0
known boundary.default_permeability
known boundary.permeability_by_resource_class values
known boundary.permeability_by_resource_id values
0 <= signal.signal_sensitivity
0 <= signal.signal_storage
0 <= signal.signal_conductivity
decay_rate >= 0
known capabilities
capability values within bounds
known fragment behavior if material can exist as external remains
```

---

# Semantic Links

- configures: [[docs/world/materials|Materials]]
- enables: [[docs/biology/process-capabilities|Process Capabilities]]
- shapes: [[docs/biology/membrane|Boundary]]
- shapes: [[docs/biology/joint|Joint]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]

# Пов'язані документи

- `world/materials.md`
- `biology/process-capabilities.md`
- `biology/cell.md`
- `biology/joint.md`
- `docs/examples/config-examples.md`
