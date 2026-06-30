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

# Що відстежувати

Корисні population metrics:

```text
cell_count
living_cell_count
dead_cell_count
birth_rate
death_rate
division_rate
average_lifespan
lineage_count
genome_variant_count
organism_like_component_count
average_component_size
resource_consumption_rate
mutation_count
extinction_events
fragmentation_events
```

Ці метрики не повинні керувати клітинами.

Вони потрібні для аналізу.

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

---

# Заборонено

Не вводити:

* global population balancing;
* forced diversity;
* artificial spawn control;
* automatic extinction prevention;
* species manager;
* population fitness controller;
* hardcoded ecological roles.

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

---

# Open Questions

* Які population metrics обов'язкові Для базової моделі?
* Чи логувати lineage tree?
* Чи потрібна окрема система для genome variant clustering?
* Як зберігати population snapshots без перевантаження storage?

---




