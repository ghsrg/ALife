---
tags:
  - alife
  - canon
  - area/evolution
---

# evolution/population-dynamics.md

> **Population Dynamics — зміни чисельності, складу і структури популяцій**

---

# Призначення

Цей документ описує `Population Dynamics` — те, як змінюються групи клітин, lineage і organism-like structures у часі.

Це аналітичний рівень над механікою клітин, genetics і world.

Population Dynamics не є окремим керуючим алгоритмом.

---

# Основна ідея

Популяція змінюється через:

* birth;
* death;
* division;
* resource competition;
* mutation;
* inheritance;
* selection;
* drift;
* migration;
* clustering;
* organism formation;
* collapse;
* decomposition.

```text
individual events
    ↓
population-level pattern
```

---

# Що Population Dynamics НЕ є

Population Dynamics не є:

* глобальним балансером;
* системою спавну;
* штучним обмежувачем;
* менеджером видів;
* direct selection engine;
* способом підтримувати “цікаву” симуляцію.

Якщо популяція вимирає, це допустимий результат.

Якщо один lineage домінує, це також допустимий результат.

---

# Required Population Metrics

Мінімальні observer-only population metrics:

```text
population_count
alive_cells_count
dead_cells_count
births_per_window
deaths_per_window
divisions_per_window
lineage_count
extinct_lineages_count
average_cell_age
resource_pressure_summary
```

Ці метрики є аналітикою, а не механізмом керування популяцією.

Вони не повинні керувати клітинами, Genome Runtime, Feasibility, selection або behavior.

---

# Lineage Dynamics

Lineage може:

* рости;
* стабілізуватися;
* дробитися;
* зникати;
* поглинати HGT fragments;
* утворювати organism-like structures;
* втрачати diversity;
* накопичувати variation.

```text
Founder Cell
    ↓
descendants
    ↓
lineage trajectory
```

Lineage tree should be logged as events, not as a full tree in every snapshot:

```text
cell_birth
cell_division
cell_death
parent_cell_id
daughter_cell_ids
lineage_ref
tick
```

Повне дерево lineage можна відновити з подій, не зберігаючи важку структуру кожен Tick.

---

# Genome Variant Dynamics

Genome variants можуть змінювати частоту.

```text
Variant A: 10% -> 70%
Variant B: 60% -> 5%
Variant C: appears by mutation
```

Причини:

* selection;
* drift;
* environmental change;
* mutation rate;
* reproduction rate;
* death rate;
* organism-level advantages.

Base genome variant tracking:

```text
genome_id
parent_genome_id
mutation_count
lineage_ref
```

Окрема система genome variant clustering не потрібна в базовій моделі.

Genome similarity clustering належить observer/research layer. Future може додати richer clustering, але клітини не повинні читати ці кластери.

---

# Drift

`Drift` — випадкова зміна частоти варіантів, не обов'язково через перевагу.

Приклади:

* маленька популяція випадково втратила корисний variant;
* neutral mutation поширилась;
* bottleneck зменшив diversity;
* випадкові смерті змінили склад lineage.

Drift не потребує окремого механізму.

Він виникає з випадковості й малих чисел.

---

# Bottleneck

Bottleneck виникає, коли популяція різко зменшується.

```text
large diverse population
    ↓
environmental collapse
    ↓
few survivors
    ↓
reduced diversity
```

Після bottleneck популяція може стати менш різноманітною навіть без selection.

---

# Expansion

Expansion виникає, коли lineage або population швидко росте.

Причини:

* багато Resources;
* вдалий mutation;
* краща Energy production;
* успішна organism-like structure;
* слабка конкуренція;
* новий ecological niche.

---

# Extinction

Extinction — зникнення lineage, variant або всієї популяції.

Причини:

* resource exhaustion;
* failed reproduction;
* high mutation instability;
* damage cascade;
* loss of critical Materials;
* competition;
* environmental stress;
* organism collapse.

Extinction не треба штучно запобігати.

---

# Carrying Capacity-like Effects

Світ може мати обмежену місткість через:

* Resources;
* space;
* Energy sources;
* waste accumulation;
* Heat;
* collisions;
* environmental degradation.

Не треба вводити окрему змінну `carrying_capacity`.

Вона має виникати з world constraints.

---

# Population і Organism-like Structures

Population може складатися не лише з окремих клітин, а й з багатоклітинних структур.

Потрібно відстежувати:

* кількість connected components;
* середній розмір component;
* lifetime components;
* fragmentation;
* merging;
* collapse;
* organism-like dependency.

---

# Population Trace

Для аналізу корисно зберігати Population Trace.

Приклад:

```text
Tick 1000
Living cells: 12_430
Lineages: 84
Genome variants: 311
Organism-like components: 42
Largest component: 950 cells
Birth rate: 0.031
Death rate: 0.027
Extinctions: 3
```

Adaptive shifts should be logged as rolling-window events, not as single-tick conclusions:

```text
adaptive_shift_event
├── tick_range
├── lineage_ref
├── population_before
├── population_after
├── dominant_material_changes
├── dominant_genome_changes
├── survival_change
├── division_change
└── environment_context
```

Starting comparison windows:

```text
last 1_000 ticks
last 10_000 ticks
```

These events are observer-only and must not drive Cell behavior, Genome Runtime, Feasibility or selection.

Frequency changes should be logged neutrally as `observed_frequency_shift`; selection/drift interpretation belongs to `evolution/selection.md`.

---

# Population Snapshots

Щоб не перевантажувати storage, базова модель зберігає агрегати по інтервалах, а не повний стан популяції кожен Tick.

```text
every N ticks:
  population totals
  lineage summaries
  birth/death/division counters
  top lineage counts
  resource/environment summary
```

Full snapshots are rare or manually requested. Detailed traces are enabled only for debug or selected runs.

---

# Правила

## Rule 1. Population dynamics is emergent

Population-level patterns виникають з індивідуальних клітинних подій.

## Rule 2. No population manager

Рушій не повинен штучно підтримувати баланс популяції.

## Rule 3. Extinction is allowed

Вимирання є нормальним результатом симуляції.

## Rule 4. Drift is allowed

Не всі зміни частот мають бути adaptive.

## Rule 5. Metrics are analytical

Population metrics не повинні бути входами для клітин.

## Rule 6. Population traces are observer-only

Population dynamics are observer-only. Population metrics describe survival, reproduction and extinction; they must not influence Genome Runtime, Feasibility, selection or behavior.

---

# Заборонено

Не вводити:

* global population balancing;
* forced diversity;
* artificial spawn control;
* automatic extinction prevention;
* species manager;
* population fitness controller;
* hardcoded ecological roles;
* full lineage tree snapshot every Tick by default;
* genome variant clustering as a Cell-readable system.

---

# Semantic Links

- observes populations of: [[docs/biology/cell|Cell]]
- observes derived: [[docs/biology/organism|Organism View]]
- shaped by: [[docs/evolution/selection|Selection]]
- may form: [[docs/evolution/species-like-clusters|Species-like Clusters]]

# Пов'язані документи

* `evolution/selection.md`
* `evolution/adaptation.md`
* `evolution/species-like-clusters.md`
* `genetics/heredity.md`
* `biology/organism.md`
* `world/resources.md`
* `world/energy.md`

