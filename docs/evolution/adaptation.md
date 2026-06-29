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

# Рівні Adaptation

## Short-term adaptation

Клітина змінює поточний стан.

Приклади:

* stress response;
* dormancy;
* repair priority;
* reduced growth;
* changed signal sensitivity.

Механізми:

* Epigenetic State;
* Runtime State;
* Material State;
* Energy balance.

---

## Lifetime adaptation

Клітина змінює свою поведінку або структуру протягом життя.

Приклади:

* накопичення signal-sensitive Materials;
* зміна threshold через repeated signals;
* спеціалізація в колонії;
* перехід у storage-like state.

Це не змінює Genome напряму.

---

## Lineage adaptation

Lineage змінюється через спадкові варіації.

Приклади:

* mutation;
* recombination;
* HGT;
* зміна mutation rate;
* зміна HGT openness;
* стабільніші Joint;
* краща Boundary regulation.

Це довготривала еволюційна adaptation.

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

---

# Заборонено

Не вводити:

* adaptation as magic improvement;
* direct Genome correction;
* automatic useful response;
* global adaptive controller;
* fitness-guided mutation;
* inherited learning as Genome rewrite.

---

# Пов'язані документи

* `genetics/epigenetics.md`
* `genetics/mutation.md`
* `genetics/heredity.md`
* `biology/specialization.md`
* `biology/organism.md`
* `evolution/selection.md`
* `evolution/population-dynamics.md`

---

# Open Questions

* Які рівні adaptation треба показувати в debug UI?
* Як відрізняти lifetime adaptation від lineage adaptation?
* Чи потрібна аналітична metric `adaptation_score`, якщо вона не впливає на клітини?
* Як логувати adaptive shifts у population?

---

