# performance.md

> Performance — обмеження й оптимізації без зміни семантики симуляції.

---

# Призначення

Performance описує, як зберігати стабільність виконання при зростанні кількості cells, joints, resources і traces.

---

# Канонічні правила

- Determinism важливіший за швидкість.
- Optimization не змінює Canon behavior.
- Spatial indexing потрібен для locality.
- Trace/debug мають бути sampled або configurable.
- Scheduler budgets не повинні мовчки пропускати mandatory semantics.

---

# Мінімальні Напрями

```text
spatial index
dirty regions
bounded traces
configurable debug sampling
deterministic parallel reductions
resource field chunking
profile-guided budgets
```

---

# Заборонено

Не вводити:

- nondeterministic updates by default;
- silent action drops due to budget;
- observer metrics in hot path as behavior inputs;
- optimization-only behavior differences.

---

# Пов'язані документи

- `engine/scheduler.md`
- `world/tick-semantics.md`
- `docs/config/stability_bounds.md`
