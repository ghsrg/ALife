---
tags:
  - alife
  - canon
  - area/genetics
---

# genome-runtime.md

> `Genome Runtime` — виконання регуляторного графа в межах Tick.

---

# Призначення

Genome Runtime читає normalized local inputs, застосовує regulatory graph і повертає process priorities. Він не виконує процеси напряму.

---

# Runtime Pipeline

```text
1. Collect local inputs.
2. Normalize inputs.
3. Apply material/capability masks.
4. Apply epigenetic modifiers.
5. Evaluate regulatory graph.
6. Produce output priorities.
7. Pass planned actions to Feasibility Check.
```

---

# Канонічні правила

- Runtime читає snapshot стану, а не live mutations під час Tick.
- Missing material basis не дає клітині “відчути” input.
- Output priority не гарантує action.
- Energy/resource/material/space constraints перевіряються після runtime.
- Epigenetic State модифікує runtime, але не переписує Genome.
- Runtime trace є debug artifact і не керує поведінкою.

---

# Energy Conflict

Якщо planned actions потребують більше Energy, ніж доступно після mandatory costs, planned actions у цьому Tick не виконуються як набір.

Це правило прибирає прихований пріоритет порядку ітерації.

Genome Runtime may propose priorities before mandatory costs are applied, but planned action Feasibility must evaluate only the post-mandatory state.

---

# Trace

Мінімальний debug trace:

```text
tick
cell_id
inputs
epigenetic_modifiers
outputs
planned_actions
feasibility_result
```

Trace може бути sampled або вимкнений для performance.

---

# Заборонено

Не вводити:

- direct process execution from Genome;
- global world reads;
- organism_id reads;
- order-dependent action priority;
- free sensing without material basis;
- mutation during runtime evaluation unless explicitly modeled.

---

# Semantic Links

- executes: [[docs/biology/genome|Genome]]
- reads state from: [[docs/biology/cell|Cell]]
- emits through: [[docs/genetics/regulatory-interface|Regulatory Interface]]
- influences: [[docs/biology/processes|Processes]]

# Пов'язані документи

- `genetics/regulatory-network.md`
- `genetics/regulatory-interface.md`
- `genetics/epigenetics.md`
- `biology/feasibility.md`
- `biology/processes.md`
- `world/tick-semantics.md`
- `docs/examples/genetics-examples.md`
