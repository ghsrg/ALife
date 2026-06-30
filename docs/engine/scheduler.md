---
tags:
  - alife
  - engine
  - area/engine
---

# scheduler.md

> Scheduler — технічний порядок оновлення стану.

---

# Призначення

Scheduler оптимізує виконання systems. `Tick` є часом світу; scheduler phases є технічною організацією обчислень.

Фази scheduler не зобов'язані один-в-один збігатися з концептуальним описом Tick, але мають зберігати його інваріанти.

---

# Мінімальні Фази

```text
1. Snapshot read state.
2. Passive world updates.
3. Cell input collection.
4. Genome Runtime.
5. Planned action assembly.
6. Mandatory costs.
7. Post-mandatory state commit.
8. Feasibility Check for planned actions.
9. Action execution.
10. Lifecycle/decomposition updates.
11. Observer/trace output.
```

---

# Канонічні правила

- Same seed + same config => same result.
- Systems read stable snapshots where required by `world/tick-semantics.md`.
- Planned actions do not execute partially unless process contract says so.
- Mandatory costs are paid before planned action Feasibility.
- Feasibility Check for planned actions uses post-mandatory state.
- Scheduler optimizations must not change simulation semantics.
- Parallel execution must preserve deterministic reduction order.

---

# Заборонено

Не вводити:

- order-dependent priority from entity iteration;
- same-tick infinite feedback;
- optimization that changes Canon behavior;
- hidden retry/repair of invalid actions;
- organism-level control phase.

---

# Semantic Links

- implements: [[docs/world/tick|Tick]]
- orders: [[docs/world/tick-semantics|Tick Semantics]]
- invokes: [[docs/genetics/genome-runtime|Genome Runtime]]
- invokes: [[docs/biology/processes|Processes]]

# Пов'язані документи

- `world/tick.md`
- `world/tick-semantics.md`
- `biology/feasibility.md`
- `biology/process-progress.md`
- `genetics/genome-runtime.md`
