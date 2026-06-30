# process-progress.md

> **ProcessProgress — накопичуваний прогрес довгих процесів без partial execution**

---

# Призначення

`ProcessProgress` описує persistent state між Tick для явно довгих процесів.

Progress не є partial final product. Це оплачена робота, яка стане результатом лише після completion threshold.

---

# Atomic Actions

У базовій моделі atomic:

- uptake;
- simple synthesis;
- movement;
- signal emit;
- basic repair.

Atomic action або виконується повністю, або відхиляється Feasibility Check.

---

# Long-Running Processes

Long-running candidates:

- genome copying;
- division preparation;
- large material synthesis;
- large structural repair;
- Joint strengthening;
- specialized structure growth.

Тільки явно позначений процес може мати ProcessProgress.

---

# ProcessProgress Identity

```text
ProcessProgress
├── cell_id
├── process_id
├── target
├── progress_value
├── threshold
├── state
└── decay_rule
```

Owner обов'язковий: `cell_id + process_id + target`.

---

# Progress Flow

```text
ActionPlan
    ↓
Feasibility Check allowed
    ↓
cost paid
    ↓
progress += delta
    ↓
if progress >= threshold:
  completion action is evaluated
  final result committed
```

Rejected action:

```text
progress unchanged
no cost charged
no partial output
```

---

# Pause, Decay, Cancel

Progress may:

- pause if conditions disappear;
- decay if process is unstable;
- be cancelled by explicit rule;
- complete only through documented completion action.

---

# Division

ProcessProgress normally is not inherited during division.

Exception requires explicit rule in `division-partition.md`.

---

# Rules

## Rule 1. Rejected action never increases progress

Progress can increase only after successful Feasibility Check.

## Rule 2. Progress is state

Progress is not hidden side effect and not partial final product.

## Rule 3. Paid work is not partial output

Paid progress work can be stored, but final result appears only at completion.

---

# Пов'язані документи

- `biology/processes.md`
- `biology/feasibility.md`
- `biology/division-partition.md`
- `world/tick-semantics.md`

