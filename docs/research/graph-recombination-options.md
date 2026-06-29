# docs/research/graph-recombination-options.md

> **Graph Recombination Options — варіанти рекомбінації для graph-based Genome**

---

# Research Note

Research / future design.

Цей файл не є базова модель specification.

Канонічний опис recombination знаходиться в:

```text id="bvbjlp"
docs/genetics/recombination.md
```

---

# Призначення

Цей файл призначений для майбутнього аналізу способів recombination для Genome, який представлений як граф.

Він потрібен, щоб не змішувати:

```text id="xv82oz"
recombination as biological/evolutionary concept
```

з конкретними технічними алгоритмами:

```text id="ytjupi"
graph crossover
subgraph insertion
fragment replacement
edge merge
node alignment
```

---

# Поточна позиція

Для базової моделі повна graph recombination не є обов'язковою.

базова модель має бути сумісним із майбутньою recombination, але не повинен реалізовувати складний graph crossover на старті.

Поточний фокус:

```text id="fe1udq"
mutation
inheritance
simple genome copying
future-compatible genome structure
```

---

# Можливі варіанти

Майбутні алгоритми можуть включати:

```text id="no81pj"
subgraph insertion
subgraph replacement
edge weight merging
node duplication
fragment deletion
partial overwrite
homology-like matching
tag-based alignment
random graph splice
```

Жоден із цих варіантів не є затвердженим Для базової моделі.

---

# Основний ризик

Graph recombination може легко стати надто “розумною”.

Заборонено робити recombination, яка:

* шукає найкращий варіант;
* оптимізує fitness;
* виправляє Genome;
* гарантує viable offspring;
* використовує species_id;
* вирівнює органи або cell types, яких у каноні немає.

---

# Коли доповнювати файл

Файл варто доповнювати, коли:

* буде реалізовано базову mutation + inheritance;
* з'явиться потреба змішувати Genome між lineages;
* буде готовий fragment model;
* HGT почне інтегрувати genetic fragments;
* буде обрано перший graph recombination algorithm;
* буде потрібен ADR щодо graph crossover.

---

# Що НЕ робити зараз

Не потрібно зараз:

* обирати остаточний graph crossover;
* реалізовувати sexual reproduction;
* вводити male/female;
* вводити species compatibility;
* робити recombination fitness-guided;
* блокувати базова модель через складну recombination.

---

# Пов'язані документи

* `docs/genetics/recombination.md`
* `docs/genetics/mutation.md`
* `docs/genetics/inheritance.md`
* `docs/genetics/horizontal-transfer.md`
* `docs/research/genome-representation-options.md`
* `docs/research/rejected-ideas.md`

---

# Notes for Agents

Цей файл є placeholder для майбутніх алгоритмічних варіантів.

Не вважати його порожнім або незавершеним базова модель-документом.

---



