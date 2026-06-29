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

## Rule 3. Bad variants are allowed

Шкідливі, нейтральні й lethal варіанти не забороняються рушієм.

## Rule 4. Selection acts through consequences

Відбір відбувається через фізичні, енергетичні, ресурсні й репродуктивні наслідки.

## Rule 5. Selection can operate at multiple levels

Cell, colony, organism-like і lineage рівні можуть давати різні selection effects.

---

# Заборонено

Не вводити:

* direct fitness function;
* automatic survival reward;
* useful mutation filter;
* global evaluator;
* hardcoded best strategy;
* organism HP as selection;
* species-based selection rule.

---

# Пов'язані документи

* `genetics/mutation.md`
* `genetics/heredity.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `biology/organism.md`
* `evolution/adaptation.md`
* `evolution/population-dynamics.md`
* `evolution/species-like-clusters.md`

---

# Open Questions

* Які metrics потрібні для аналізу selection?
* Чи рахувати selection на рівні cell lineage або organism-like structures?
* Як відрізняти selection від drift у логах?
* Які population-level метрики потрібні Для базової моделі?

---


