---
tags:
  - alife
  - canon
  - area/evolution
---

# evolution/species-like-clusters.md

> **Species-like Clusters — видоподібні групи без hardcoded species_id**

---

# Призначення

Цей документ описує, як у симуляції можуть виникати `species-like clusters`.

У рушії немає hardcoded species.

Немає обов'язкового `species_id`, який визначає поведінку клітини.

Species-like cluster — це аналітична група lineage або organism-like structures, які стали схожими й частково ізольованими через heredity, selection, drift, compatibility і обмежений genetic exchange.

---

# Основна ідея

```text
shared heredity
+
limited gene flow
+
similar traits
+
selection / drift
+
time
    ↓
species-like cluster
```

Species-like cluster не створюється рушієм напряму.

Він виявляється аналітично.

---

# Що Species-like Cluster НЕ є

Species-like cluster не є:

* species_id;
* класом виду;
* hardcoded reproductive group;
* фіксованою назвою;
* поведінковим правилом;
* гарантією сумісності;
* забороною HGT;
* глобальною категорією, яку клітини читають.

Клітина не повинна знати:

```text
I belong to species X
```

Вона може реагувати лише на локальні Materials, Signals, Boundary compatibility, Joint compatibility і genetic fragments.

---

# Джерела divergence

Групи можуть розходитися через:

* mutation accumulation;
* recombination patterns;
* HGT isolation;
* different environments;
* resource specialization;
* organism-level structures;
* communication differences;
* Joint compatibility;
* epigenetic-developmental patterns;
* drift;
* bottlenecks.

---

# Genetic Exchange

Species-like clusters можуть відрізнятися рівнем genetic exchange.

```text
high exchange
    ↓
clusters merge or remain fuzzy

low exchange
    ↓
clusters diverge
```

Genetic exchange може зменшуватися через:

* genome isolation;
* HGT rejection;
* incompatible fragments;
* recombination failure;
* Boundary incompatibility;
* spatial separation;
* organism-level dependency.

---

# Compatibility без species_id

Compatibility повинна виникати з фізичних і регуляторних властивостей.

Приклади:

* Boundary Materials сумісні або ні;
* Joint creation працює або ні;
* Signals розпізнаються або ні;
* HGT fragments інтегруються або деградують;
* recombination дає viable або non-viable результат;
* mixed colonies стабільні або розпадаються.

Немає правила:

```text
if same_species:
    compatible
```

---

# Fuzzy Boundaries

Species-like clusters можуть мати нечіткі межі.

Можливі ситуації:

* lineage A частково сумісний з lineage B;
* HGT проходить між кількома групами;
* hybrid structures іноді життєздатні;
* organism-like clusters мають mixed ancestry;
* groups merge after environmental change.

Це нормально.

---

# Species-like Cluster Metrics

Для аналізу можна використовувати:

```text
genome_similarity
lineage_distance
fragment_sharing
HGT_rate_between_groups
recombination_success_rate
joint_compatibility
signal_compatibility
material_similarity
offspring_viability
ecological_niche_overlap
```

Ці метрики не повинні керувати поведінкою клітин.

---

# Cluster Detection

Species-like clusters можна визначати аналітично.

Можливі підходи:

```text
genome similarity clustering
lineage tree clustering
fragment sharing network
HGT flow graph
phenotype-like similarity
organism structure similarity
niche similarity
```

Для базової моделі це можна не реалізовувати.

---

# Species-like Cluster і Selection

Selection може підтримувати clusters, якщо певна комбінація спадковості, сумісності й середовища стабільно працює.

Наприклад:

* один cluster краще використовує Resource A;
* інший краще формує Joint;
* третій має нижчу HGT openness і захищений від harmful fragments.

---

# Species-like Cluster і Drift

Drift може створювати розходження навіть без явної переваги.

Особливо в малих популяціях або після bottleneck.

```text
small isolated population
    ↓
random variant fixation
    ↓
cluster divergence
```

---

# Species-like Cluster і Organism

Organism-like structures можуть підсилювати species-like boundaries.

Якщо клітини дуже залежать від власної structure, вони можуть гірше змішуватися з іншими lineage.

Але symbiosis-like або chimeric structures також можливі.

---

# Базова модель

Для базової моделі не треба вводити species system.

Достатньо:

```text
no species_id
lineage tracking
genome similarity metrics
HGT/recombination traces
organism-like component metrics
```

Species-like clusters можна аналізувати пізніше на основі логів.

---

# Правила

## Rule 1. No hardcoded species

Species не є engine-level класом.

## Rule 2. Species-like clusters are analytical

Видоподібні групи виявляються через similarity, lineage і exchange patterns.

## Rule 3. Compatibility is physical/regulatory

Сумісність виникає з Materials, Signals, Boundary, Joint, Genome і HGT mechanisms.

## Rule 4. Boundaries can be fuzzy

Species-like межі не повинні бути абсолютно жорсткими.

## Rule 5. Cells do not read species labels

Species-like label не може бути input для Genome Runtime.

---

# Заборонено

Не вводити:

* species_id as behavior input;
* hardcoded same-species compatibility;
* hardcoded reproductive isolation;
* forced species categories;
* species manager;
* fixed taxonomy;
* automatic naming of species as engine rule.

---

# Semantic Links

- derived from: [[docs/evolution/population-dynamics|Population Dynamics]]
- shaped by: [[docs/evolution/selection|Selection]]
- not hardcoded in: [[docs/biology/cell|Cell]]
- tracked through: [[docs/genetics/heredity|Heredity]]

# Пов'язані документи

* `evolution/selection.md`
* `evolution/population-dynamics.md`
* `genetics/heredity.md`
* `genetics/recombination.md`
* `genetics/horizontal-transfer.md`
* `biology/organism.md`
* `biology/specialization.md`

---

# Open Questions

* Чи потрібне species-like clustering У базовій моделі?
* Який metric краще: genome similarity, lineage distance чи fragment sharing?
* Як аналізувати mixed-genome organism-like structures?
* Чи потрібні fuzzy cluster labels у debug UI?

