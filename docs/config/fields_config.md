---
tags:
  - alife
  - config
  - area/config
---

# fields_config.md

> Конфігурація полів і зовнішніх впливів.

---

# Призначення

`fields_config` описує FieldType: просторові впливи, які можуть читатися клітинами або впливати на Resources, Materials, Heat чи movement.

Field не є Resource і не зберігається в клітині як речовина.

---

# Мінімальна Схема

```yaml
fields:
  light:
    kind: scalar
    initial_value: 1.0
    diffusion_rate: 0.0
    decay_rate: 0.0
    min_value: 0.0
    max_value: 1.0
    effect_profile: light
    conserved_behavior: abstracted
```

---

# Канонічні правила

- Field може бути input для Genome Runtime лише за наявності матеріальної основи sensing.
- Field не створює Energy Buffer напряму; потрібен Material/process.
- Heat може моделюватися як field-like world state, але клітина має власну `temperature`.
- На поточному етапі temperature клітини діє на суміжні клітини/Joint локально і не обов'язково змінює глобальне середовище.
- Кожен Field має effect profile згідно з `world/field-semantics.md`.
- Field effect не є командою і не створює результат без Material/process/reaction/physics mechanism.

---

# Validation

```text
id unique
known kind
min_value <= initial_value <= max_value
diffusion_rate >= 0
decay_rate >= 0
max_value >= min_value
known effect_profile
known conserved_behavior
explicit bounds
```

---

# Semantic Links

- configures: [[docs/world/fields|Fields]]
- interpreted by: [[docs/world/field-semantics|Field Semantics]]
- sensed by: [[docs/biology/cell|Cell]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]

# Пов'язані документи

- `world/fields.md`
- `world/energy.md`
- `world/field-semantics.md`
- `docs/examples/config-examples.md`
