---
tags:
  - alife
  - canon
  - area/evolution
---

# evolution/adaptation.md

> **Adaptation — зміни, які підвищують відповідність клітини, lineage або структури до середовища**

---

# Призначення

Цей документ описує `Adaptation` як результат різних механізмів зміни.

Adaptation не є одним процесом.

Це загальний ефект, який може виникати на різних рівнях:

* cell state;
* epigenetic state;
* material state;
* specialization;
* organism structure;
* lineage heredity;
* population composition.

---

# Основна ідея

```text
environmental pressure
+
variation
+
selection
    ↓
adaptation
```

Adaptation означає, що певні стани, властивості або lineage краще зберігаються в конкретному середовищі.

---

# Що Adaptation НЕ є

Adaptation не є:

* свідомою відповіддю;
* прямим покращенням Genome;
* гарантією виживання;
* learning у вузькому сенсі;
* mutation;
* epigenetics;
* selection;
* hardcoded strategy.

Adaptation — це результат, а не окремий engine command.

---

# Debug UI Levels

Для debug UI достатньо чотирьох рівнів:

```text
cell_state_adaptation
material_adaptation
lineage_adaptation
population_shift
```

Meaning:

```text
cell_state_adaptation =
  зміни runtime/epigenetic/material state в межах життя клітини

material_adaptation =
  зміни Materials: sensitivity, conductivity, storage, damage tolerance

lineage_adaptation =
  спадкові зміни Genome/EpigeneticState у нащадків

population_shift =
  зміна частот lineage/genome/material profiles у популяції
```

Ці рівні є observer/debug interpretation, а не inputs для клітини.

---

# Lifetime vs Lineage Adaptation

## Lifetime adaptation

Lifetime adaptation відбулась у межах життя однієї клітини і не обов'язково передається нащадкам.

Приклади:

* накопичення signal-sensitive Materials;
* зміна threshold через repeated signals;
* спеціалізація в колонії;
* перехід у storage-like state.

Це не змінює Genome напряму.

## Lineage adaptation

Lineage adaptation повторюється або закріплюється через поділ і видима як зміна властивостей нащадків, lineage або population.

Приклади:

* mutation;
* recombination;
* HGT;
* зміна mutation rate;
* зміна HGT openness;
* стабільніші Joint;
* краща Boundary regulation.

Це довготривала еволюційна adaptation.

Short form:

```text
lifetime = cell changed during life
lineage = descendants changed across generations
```

---

## Population adaptation

Популяція адаптується, коли частота варіантів змінюється.

```text
Variant A becomes common
Variant B disappears
```

Це може відбуватися без зміни кожної окремої клітини.

---

## Organism-level adaptation

Organism-like structures можуть адаптуватися через:

* кращу resource sharing;
* specialization;
* communication;
* redundancy;
* repair coordination;
* fragmentation reproduction;
* mechanical stability.

---

# Observer Metrics

Не вводити один загальний `adaptation_score` у базовій моделі. Він легко стає pseudo-fitness.

Замість цього використовувати окремі observer-only metrics:

```text
survival_delta
division_rate_delta
material_profile_shift
genome_profile_shift
resource_efficiency_shift
stress_resilience_shift
population_frequency_shift
```

Future може додати aggregate `adaptation_score` тільки як observer-only metric. Він не може бути input для Genome Runtime, Feasibility, selection або behavior.

---

# Adaptive Shift Events

Adaptive shifts логуються як events і aggregates по rolling windows, а не як single-tick conclusion.

Minimum event:

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

Starting windows:

```text
last 1_000 ticks
last 10_000 ticks
```

---

# Adaptation і Learning-like Behavior

Learning-like behavior — це один із можливих short-term або lifetime adaptation mechanisms.

Але не вся adaptation є learning.

```text
learning-like state
    ⊂
adaptation
```

---

# Adaptation і Mutation

Mutation може створити спадковий варіант.

Але mutation сама по собі не є adaptation.

Мутація стає частиною adaptation лише якщо її наслідки зберігаються через selection.

---

# Adaptation і Epigenetics

Epigenetics може підтримувати short-term або lifetime adaptation.

Але epigenetic change не дорівнює evolution.

Воно може зникнути, скинутись або частково успадкуватися.

---

# Правила

## Rule 1. Adaptation is an outcome

Adaptation — це результат взаємодії variation, environment і selection.

## Rule 2. Adaptation has levels

Потрібно розрізняти cell-level, lifetime, lineage, population і organism-level adaptation.

## Rule 3. Not all change is adaptation

Зміна може бути нейтральною або шкідливою.

## Rule 4. Not all adaptation is genetic

Матеріальні, epigenetic і структурні зміни теж можуть давати адаптивний ефект.

## Rule 5. Long-term adaptation requires heredity

Щоб adaptation зберігалася в lineage, потрібен спадковий канал.

## Rule 6. Adaptation metrics are observer-only

Adaptation is observer interpretation, not a Cell, Genome Runtime, Feasibility, selection or behavior input.

---

# Заборонено

Не вводити:

* adaptation as magic improvement;
* direct Genome correction;
* automatic useful response;
* global adaptive controller;
* fitness-guided mutation;
* inherited learning as Genome rewrite;
* adaptation_score as behavior or selection input.

---

# Semantic Links

- emerges from: [[docs/evolution/selection|Selection]]
- depends on variation from: [[docs/genetics/mutation|Mutation]]
- can stabilize: [[docs/biology/specialization|Specialization]]
- measured through: [[docs/evolution/population-dynamics|Population Dynamics]]

# Пов'язані документи

* `genetics/epigenetics.md`
* `genetics/mutation.md`
* `genetics/heredity.md`
* `biology/specialization.md`
* `biology/organism.md`
* `evolution/selection.md`
* `evolution/population-dynamics.md`


