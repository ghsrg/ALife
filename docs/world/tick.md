# tick.md

> `Tick` — логічний крок дискретного часу симуляції.

---

# Призначення

Tick визначає, коли світ переходить від одного committed state до наступного.

Tick не визначає повний implementation order engine systems.

Детальна видимість стану, phase commits і causality rules описані в `world/tick-semantics.md`.

Scheduler implementation описаний у `engine/scheduler.md`.

---

# Time Model

```text
Tick = logical simulation step
simulation time = configured interpretation of ticks
wall-clock time = real execution speed
```

Default calibration може використовувати:

```text
1 Tick = 1 simulation second
```

Це calibration default, а не біологічна або фізична істина.

---

# Conceptual Flow

На концептуальному рівні Tick включає:

```text
environment preparation
cell decision
planned action resolution
physics / lifecycle resolution
statistics / observer update
```

Це conceptual flow, не literal implementation order.

Concrete systems можуть бути split, merged або optimized, якщо вони зберігають `world/tick-semantics.md`.

---

# Causality

Cells не можуть читати uncommitted changes, partial writes або decisions/actions інших клітин з тієї самої phase.

Environment preparation може створювати committed environment snapshot для поточного Tick, якщо це явно визначено `world/tick-semantics.md`.

Cell decisions у Tick N можуть читати тільки committed state, доступний на Decision Phase.

Actions, movements, signals, divisions або deaths інших клітин з Tick N не видимі для Cell Decision до відповідного пізнішого commit boundary, зазвичай Tick N+1.

---

# Mandatory Costs And Planned Actions

Mandatory costs are resolved before Feasibility for planned actions.

Planned actions use only post-mandatory state.

Якщо mandatory costs не можуть бути оплачені, planned actions у цьому Tick skipped/rejected, а lifecycle rules визначають cell state.

---

# Base Model Exclusions

Base Tick model не потребує:

```text
HGT
recombination
advanced learning systems
specialized lifecycle phases for future genetic extensions
```

Такі механіки можуть бути додані пізніше тільки якщо їх visibility і commit rules явно визначені в `world/tick-semantics.md` і реалізовані в `engine/scheduler.md`.

---

# Invariant

```text
tick.md defines conceptual time.
tick-semantics.md defines visibility and causality.
engine/scheduler.md defines implementation.
No literal phase list in tick.md may override the Tick Semantics contract.
```

---

# Пов'язані документи

- `world/laws.md`
- `world/tick-semantics.md`
- `world/units.md`
- `biology/feasibility.md`
- `biology/process-progress.md`
- `world/energy.md`
- `engine/scheduler.md`
