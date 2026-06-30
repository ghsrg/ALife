---
tags:
  - alife
  - canon
  - area/biology
---

# lifecycle.md

> `Cell Lifecycle` — життєвий цикл клітини як наслідок її матеріального стану.

---

# Призначення

Lifecycle описує, як клітина виникає, підтримує себе, росте, ділиться, переходить у dormancy, пошкоджується, помирає і розкладається.

Lifecycle не є сценарієм і не використовує HP. Стан клітини визначається Materials, Resources, Energy, Boundary, Genome, Heat, Pressure, Joint і Feasibility Check.

Функціональний контракт станів описаний у `biology/cell-state.md`. Partition під час division описаний у `biology/division-partition.md`.

---

# Lifecycle States

Мінімальні стани:

```text
alive
stressed
dormant
division_preparing
dividing
dead
decomposing
```

Технічний `lifecycle_state` допомагає організувати процеси, але не замінює фізичну модель.

---

# Канонічні правила

- Клітина жива, поки може підтримувати мінімальну функціональну структуру.
- Growth є збільшенням матеріальної структури, а не просто `size`.
- Division є фізичним partition локального стану parent cell.
- Energy Buffer під час division розподіляється як частина локального стану, а не передається між незалежними клітинами.
- Death є втратою мінімальної життєздатності, а не `kill()` command.
- Dead cell не зникає миттєво; вона переходить у MaterialFragment, Resource або decomposition state згідно з explicit rules.
- Errors during division дозволені й фільтруються selection.

---

# Мінімальна Життєздатність

Клітина може вважатися живою, якщо має достатньо:

```text
Boundary integrity
critical Materials
internal composition stability
Energy Buffer or Energy production path
Genome or regulatory carrier
maintenance capability
free_capacity for critical processes
```

Точні пороги належать конфігурації.

---

# Transitions

```text
alive -> stressed
  if Energy low OR Boundary damaged OR Heat high OR critical resources missing

stressed -> dormant
  if low activity is feasible and structure remains viable

dormant -> alive
  if conditions improve and regulation resumes activity

alive -> division_preparing
  if growth, genome copy and regulation support division

division_preparing -> dividing
  if Feasibility Check passes

any living state -> dead
  if minimum viable structure is lost

dead -> decomposing
  through MaterialFragment/resource degradation
```

---

# Division

Division потребує Energy, Boundary materials, Resources, Genome copy/fragments, внутрішнього об'єму, зовнішнього простору і достатньої стабільності.

Starting partition coefficients are defined in `biology/division-partition.md`:

```text
default_split = 0.5
split_noise = ±0.15
allowed_split_range = 0.35..0.65
```

Resources, Materials, Energy Buffer and MaterialState split noisy-proportionally.

Genome information is not partitioned. It is copied onto a second physical carrier before partition.

Якщо Feasibility Check відхиляє division action до partition:

- дочірні клітини не створюються;
- state не partition-иться;
- progress preparation обробляється за `biology/process-progress.md`.

Якщо partition вже committed, результатом можуть бути слабкі, leaking, inert або dead daughters. Рушій не повинен автоматично ремонтувати їх до життєздатного стану.

---

# Dormancy

Dormancy reduces activity, but does not make the Cell free to maintain.

Starting modifiers:

```text
mandatory_energy_cost_modifier = 0.25
degradation_rate_modifier = 0.5
active_transport_modifier = 0.0
synthesis_modifier = 0.0
signal_emit_modifier = 0.0
repair_modifier = 0.25
```

A dormant Cell mostly does not act, grow, synthesize, move or emit active signals. It still pays minimal stability costs and continues to degrade slowly.

---

# Death and Decomposition

Death може виникнути через втрату Boundary, critical Materials, Genome functionality, Energy path, internal capacity або maintenance capability.

Decomposition повертає MaterialFragments, Resources, Heat і можливі Genome fragments у локальний світ. Мертва клітина є матеріальним об'єктом, а не сміттям рушія.

MaterialFragment може деградувати або реагувати в Resources тільки через explicit reaction/degradation/conversion rules.

Decomposition rates come from Material properties, not a single global rate:

```text
material.decay_rate
material.stability
environment_modifiers
temperature
reactivity
```

Starting decomposition categories:

```text
soft/internal material      -> fast decay
boundary material           -> medium decay
structural/joint material   -> slow decay
inert/mineral-like material -> very slow decay
```

Genome fragments after death exist in the first implementation only as inert Resource-like particles:

```text
dead genome carrier
  -> genetic fragments
  -> decay over time
  -> inert waste/resources
```

They do not integrate into living cells automatically, do not mutate a living Genome and do not start HGT without a separate explicit process.

Living Cell не може поглинути dead remains як Resource без fragment breakdown, external digestion, surface degradation, material uptake capability або conversion process.

---

# Invariant

```text
Division partitions matter, but Genome information is copied onto a physical carrier.
Dormancy reduces activity, but does not stop all costs.
Decomposition follows Material rules.
Genome fragments after death are inert unless explicit future integration rules exist.
```

---

# Lifecycle Debug Metrics

Lifecycle debug metrics reuse observer-only `debug_metrics` from `biology/cell.md`.

Minimum lifecycle-related fields:

```text
age_ticks
divisions_count
stress_level
lineage_ref
```

`stress_level` is a derived debug summary, not real Cell state or regulatory input.

`lineage_ref` may contain `parent_id` and `lineage_id` for observer/debug UI and research metrics. Cells, Genome Runtime, Feasibility and Processes must not read lineage/debug metrics as behavior input.

---

# Заборонено

Не вводити:

- HP;
- instant death без матеріальної причини;
- disappearance on death;
- guaranteed equal division;
- guaranteed viable offspring;
- hardcoded reproduction mode;
- age timer як самостійну причину смерті;
- species-based lifecycle.

---

# Semantic Links

- evaluates viability of: [[docs/biology/cell|Cell]]
- depends on: [[docs/world/materials|Materials]]
- depends on: [[docs/world/energy|Energy]]
- depends on: [[docs/biology/genome|Genome]]
- triggers: [[docs/biology/division-partition|Division Partition]]
- produces remains through: [[docs/world/reactions|Reactions]]

# Пов'язані документи

- `biology/cell.md`
- `biology/cell-state.md`
- `biology/division-partition.md`
- `biology/feasibility.md`
- `biology/process-progress.md`
- `biology/processes.md`
- `biology/joint.md`
- `world/resources.md`
- `world/materials.md`
- `world/energy.md`
- `world/tick.md`
- `docs/examples/biology-examples.md`
