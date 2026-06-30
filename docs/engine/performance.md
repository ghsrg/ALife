---
tags:
  - alife
  - engine
  - area/engine
---

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
- Technology stack and practical performance choices are defined in `engine/technology-stack.md`.
- Headless simulation is the source of truth; rendering is a read-only projection.

---

# Target Scale

Initial architecture target:

```text
world: 512 x 512 su
cells: 20k target
joints: 20k-40k
resources: 4-8 types
fields: 3-5 layers
signals/traces: 1 scalar trace layer or sparse trace
rendered speed: 30+ ticks/sec
headless speed: 100+ ticks/sec
```

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
scheduled Genome Runtime cadence
scheduled diffusion / trace cadence
SoA memory layout
fixed-point accounting for conserved amounts
```

Expected hot spots:

```text
Cell hot loop
Genome Runtime
Feasibility and conflict resolution
Resource / Field / Trace diffusion
Spatial locality queries
Joint graph updates
Observer/debug trace output
```

---

# Заборонено

Не вводити:

- nondeterministic updates by default;
- silent action drops due to budget;
- observer metrics in hot path as behavior inputs;
- optimization-only behavior differences;
- untyped float accounting for Energy, Resources, Materials, costs or capacity.

---

# Semantic Links

- optimizes execution of: [[docs/engine/scheduler|Scheduler]]
- constrained by: [[docs/engine/technology-stack|Technology Stack]]
- optimizes fields from: [[docs/world/fields|Fields]]
- optimizes many: [[docs/biology/cell|Cells]]
- must preserve: [[docs/world/tick-semantics|Tick Semantics]]

# Пов'язані документи

- `engine/scheduler.md`
- `engine/technology-stack.md`
- `world/tick-semantics.md`
- `docs/config/stability_bounds.md`
