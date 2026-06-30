---
tags:
  - alife
  - config
  - area/config
---

# resources_config.md

> Конфігурація рухомих речовин світу.

---

# Призначення

`resources_config` описує ResourceType: речовини, які можуть рухатися, накопичуватися, займати об'єм, реагувати й бути поглинутими клітинами.

Space не є ResourceType.

---

# Мінімальна Схема

```yaml
resources:
  glucose:
    volume: 1.0
    diffusion_rate: 0.1
    energy_value: 4.0
    decay_rate: 0.0
    size_class: small
    phase: dissolved
    reactivity_class: stable
    polarity_tag: neutral
    permeability_class: small_nutrient
    tags: [energy_source]
```

---

# Канонічні правила

- Resource не дає функцію клітині напряму.
- Корисність/шкідливість Resource виникає через reactions, volume, Heat, Material degradation або Energy conversion.
- Немає окремої властивості `toxicity`.
- `energy_value` є потенціалом для Energy production, а не автоматичним поповненням Energy Buffer.
- Resource займає capacity.
- Resource не проходить через Boundary напряму; exchange визначається Resource physical traits і Boundary Material permeability rules.

---

# Validation

```text
id unique
volume >= 0
diffusion_rate >= 0
energy_value >= 0
decay_rate >= 0
known size_class
known phase
known reactivity_class
known permeability_class
known tags
```

---

# Semantic Links

- configures: [[docs/world/resources|Resources]]
- consumed by: [[docs/world/reactions|Reactions]]
- used by: [[docs/biology/cell|Cell]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]

# Пов'язані документи

- `world/resources.md`
- `world/reactions.md`
- `world/energy.md`
- `docs/examples/config-examples.md`
