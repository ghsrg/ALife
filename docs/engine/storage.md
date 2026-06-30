---
tags:
  - alife
  - engine
  - area/engine
---

# storage.md

> Storage — файли, логи, traces і результати експериментів.

---

# Призначення

Storage визначає, що і як записується для replay, audit, debug і research metrics.

---

# Канонічні правила

- Storage не керує симуляцією.
- Logs/traces мають бути bounded або configurable.
- Experiment output має містити config hash і seed.
- Data needed for replay належить snapshot/serialization, а не ad hoc logs.

---

# Мінімальні Артефакти

```text
snapshot
config copy/hash
run metadata
event trace
metrics summary
debug sampled traces
```

---

# Заборонено

Не вводити:

- behavior input from previous logs unless explicitly loading snapshot;
- unbounded trace by default;
- hidden mutable global storage.

---

# Semantic Links

- stores snapshots from: [[docs/engine/serialization|Serialization]]
- stores: [[docs/engine/ecs|ECS]]
- supports analysis of: [[docs/evolution/population-dynamics|Population Dynamics]]

# Пов'язані документи

- `engine/serialization.md`
- `engine/rendering.md`
- `docs/config/stability_bounds.md`
