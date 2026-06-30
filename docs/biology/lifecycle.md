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

Якщо Feasibility Check відхиляє division action до partition:

- дочірні клітини не створюються;
- state не partition-иться;
- progress preparation обробляється за `biology/process-progress.md`.

Якщо partition вже committed, результатом можуть бути слабкі, leaking, inert або dead daughters. Рушій не повинен автоматично ремонтувати їх до життєздатного стану.

---

# Death and Decomposition

Death може виникнути через втрату Boundary, critical Materials, Genome functionality, Energy path, internal capacity або maintenance capability.

Decomposition повертає MaterialFragments, Resources, Heat і можливі Genome fragments у локальний світ. Мертва клітина є матеріальним об'єктом, а не сміттям рушія.

MaterialFragment може деградувати або реагувати в Resources тільки через explicit reaction/degradation/conversion rules.

Living Cell не може поглинути dead remains як Resource без fragment breakdown, external digestion, surface degradation, material uptake capability або conversion process.

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

# Open Questions

- Стартові коефіцієнти для `biology/division-partition.md`.
- Наскільки dormancy зменшує Energy consumption, degradation, transport і synthesis.
- Decomposition rates для різних Materials.
- Чи Genome fragments після death входять у першу реалізацію.
- Мінімальні lifecycle metrics для debug UI: `age_ticks`, `divisions_count`, `stress_level`, `parent_id`, `lineage_id`.

---

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
