---
tags:
  - alife
  - canon
  - area/evolution
---

# evolution/selection.md

> **Selection — відбір через виживання, розмноження і зникнення ліній**

---

# Призначення

Цей документ описує `Selection` як популяційний наслідок роботи світу.

Selection не є окремим алгоритмом, який вибирає “кращих”.

Selection виникає тоді, коли різні клітини, колонії або organism-like structures мають різну здатність:

* виживати;
* підтримувати структуру;
* отримувати Resources;
* виробляти Energy;
* ремонтувати пошкодження;
* розмножуватися;
* передавати спадковий стан.

---

# Основна ідея

```text
variation
    ↓
different survival / reproduction
    ↓
population changes
```

Варіації виникають через:

* mutation;
* recombination;
* horizontal transfer;
* asymmetric inheritance;
* epigenetic differences;
* material state differences;
* environmental conditions.

Selection лише проявляється через наслідки цих варіацій.

---

# Що Selection НЕ є

Selection не є:

* fitness function;
* score;
* нагородою;
* алгоритмом оптимізації;
* свідомим вибором;
* забороною поганих мутацій;
* прямим керуванням Genome;
* глобальним суддею.

Рушій не повинен робити:

```text
if genome_is_good:
    keep it
else:
    remove it
```

Клітина просто живе або не живе в умовах світу.

---

# Рівні Selection

Selection може проявлятися на різних рівнях:

```text
cell level
colony level
organism-like structure level
lineage level
population level
```

Але базовий механізм завжди той самий:

```text
world conditions
+
cell/organism properties
    ↓
survival and reproduction difference
```

---

# Cell-level Selection

Клітини відбираються через здатність:

* підтримувати Boundary;
* виробляти Energy;
* уникати clogging;
* ремонтувати Materials;
* ділитися;
* не перегріватися;
* не втрачати Genome;
* реагувати на локальні умови.

---

# Organism-level Selection

Organism-like structure може мати перевагу, якщо:

* клітини краще ділять Resources;
* Joint стабілізують структуру;
* communication поширює stress signals;
* specialization підвищує ефективність;
* repair працює на рівні структури;
* fragmentation створює нові життєздатні структури.

Але selection не діє на organism як магічний об'єкт.

Вона діє через виживання і розмноження клітин та lineage.

---

# Selection і Fitness

`Fitness` можна використовувати лише як аналітичний термін.

Наприклад:

```text
descendant_count
survival_time
division_success
lineage_persistence
offspring_viability
```

Але клітини не повинні читати `fitness_score`.

Genome не повинен мати input:

```text
fitness
```

---

# Selection Analysis Metrics

Base selection analysis is observer-only and lineage-first.

Minimum metrics:

```text
survival_time_by_lineage
division_count_by_lineage
offspring_count_by_lineage
death_rate_by_lineage
lineage_frequency_over_time
resource_efficiency_by_lineage
stress_survival_by_lineage
```

Це аналітика після факту, не fitness input.

Base level:

```text
selection analysis = lineage-level first
organism-like analysis = derived observer view
```

OrganismView можна аналізувати окремо:

```text
component_lifetime
component_cell_count
fragmentation_events
viable_child_components
collapse_reason
```

Базова selection-аналітика йде від lineage/cell reproduction. Organism-like metrics не є окремою selection mechanism.

---

# Selection vs Drift Logs

Логи не повинні одразу доводити selection. Спочатку фіксується нейтральна подія:

```text
observed_frequency_shift
├── tick_range
├── lineage_ref
├── frequency_before
├── frequency_after
├── survival_context
├── division_context
├── resource_efficiency_context
├── population_size_context
└── environment_context
```

Interpretation:

```text
if lineage частішає разом із кращим survival/division/resource efficiency:
  possible selection

if lineage частішає без стабільної переваги або при малих числах:
  possible drift
```

Selection is an analytical interpretation, not simulation mechanics.

---

# Population-Level Metrics

Minimum population-level metrics for selection analysis:

```text
population_count
births_per_window
deaths_per_window
divisions_per_window
lineage_count
lineage_frequency_distribution
extinction_events
average_survival_time
average_division_rate
resource_pressure_summary
environment_context
```

These metrics align with `evolution/population-dynamics.md` and remain observer-only.

---

# Selection і Neutrality

Не всі зміни одразу корисні або шкідливі.

Можливі:

* neutral mutations;
* silent fragments;
* слабкі epigenetic effects;
* матеріальні стани без помітного впливу.

Neutral variation може зберігатися і стати важливою пізніше.

---

# Selection і Environment

Selection завжди залежить від середовища.

Те, що корисне в одному світі, може бути шкідливим в іншому.

```text
high mutation rate
    useful in changing environment
    harmful in stable environment
```

```text
strong Boundary
    useful under pressure
    costly in resource-poor world
```

---

# Правила

## Rule 1. Selection is emergent

Selection виникає з різниці у виживанні й розмноженні.

## Rule 2. No fitness input

Genome не повинен читати fitness score.

Adaptation metrics, including any future `adaptation_score`, must not become selection inputs or behavior inputs.

## Rule 3. Bad variants are allowed

Шкідливі, нейтральні й lethal варіанти не забороняються рушієм.

## Rule 4. Selection acts through consequences

Відбір відбувається через фізичні, енергетичні, ресурсні й репродуктивні наслідки.

## Rule 5. Selection can operate at multiple levels

Cell, colony, organism-like і lineage рівні можуть давати різні selection effects.

## Rule 6. Frequency shifts are logged before interpretation

Frequency shifts should be logged first, then interpreted as possible selection or possible drift using survival, division, resource and population-size context.

---

# Заборонено

Не вводити:

* direct fitness function;
* automatic survival reward;
* useful mutation filter;
* global evaluator;
* hardcoded best strategy;
* organism HP as selection;
* species-based selection rule;
* adaptation metric as a selection controller;
* direct selection label without observed frequency shift context.

---

# Semantic Links

- filters variation from: [[docs/genetics/mutation|Mutation]]
- acts on viability of: [[docs/biology/cell|Cell]]
- acts on derived: [[docs/biology/organism|Organism View]]
- changes: [[docs/evolution/population-dynamics|Population Dynamics]]

# Пов'язані документи

* `genetics/mutation.md`
* `genetics/heredity.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `biology/organism.md`
* `evolution/adaptation.md`
* `evolution/population-dynamics.md`
* `evolution/species-like-clusters.md`


