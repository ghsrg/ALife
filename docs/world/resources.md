---
tags:
  - alife
  - canon
  - area/world
---

# resources.md

> `Resource` — рухома речовина світу.

---

# Призначення

Resource описує речовину, яка може існувати в середовищі або всередині клітини, займати об'єм, рухатися, реагувати, бути поглинутою або виведеною.

Space не є Resource.

MaterialFragment не є Resource, доки explicit degradation/reaction/conversion rule не перетворить його на Resource units.

---

# Канонічні правила

- Resource не дає функцію напряму; функціональність виникає через Materials і processes.
- Resource може мати `energy_value`, але Energy Buffer поповнюється тільки через reaction/process.
- Resource займає capacity.
- Resource може бути корисним, нейтральним або шкідливим лише через reactions, Heat, volume, Material degradation або clogging.
- Немає окремої властивості `toxicity`.
- Resource movement підпорядковується Space/locality/diffusion/transport rules.
- Living Cell не може silently absorb MaterialFragment as ordinary Resource.

---

# Мінімальні Властивості

```text
id
volume
diffusion_rate
energy_value
decay_rate
reactivity profile
permeability constraints
tags
```

---

# Заборонено

Не вводити:

- direct function from Resource;
- poison shortcut;
- free Energy from Resource;
- Resource without volume when capacity matters;
- global Resource access by cell.
- silent consumption of MaterialFragment without explicit breakdown/conversion process.

---

# Semantic Links

- stored inside: [[docs/biology/cell|Cell]]
- constrained by: [[docs/world/space|Space]]
- converted by: [[docs/world/reactions|Reactions]]
- can become: [[docs/world/materials|Materials]]
- can feed: [[docs/world/energy|Energy Buffer]]
- configured by: [[docs/config/resources_config|Resources Config]]

# Пов'язані документи

- `world/reactions.md`
- `world/energy.md`
- `world/materials.md`
- `docs/config/resources_config.md`
