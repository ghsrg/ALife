---
tags:
  - alife
  - research
  - area/research
---

# docs/research/genome-representation-options.md

> **Genome Representation Options — альтернативи представлення Genome**

---

# Research Note

Research / design options.

Цей файл не є поточною implementation specification.

Канонічна реалізація Genome описується в:

```text
docs/biology/genome.md
docs/genetics/regulatory-network.md
docs/genetics/genome-runtime.md
```

---

# Призначення

Цей файл призначений для фіксації можливих способів представлення Genome.

Він потрібен, щоб:

* не змішувати поточну реалізацію з research-ідеями;
* зберігати альтернативи для майбутніх етапів;
* пояснити, чому Для базової моделі обрано простішу модель;
* не повертатися випадково до відхилених або надто складних варіантів.

---

# Поточна позиція

Для базової моделі використовується:

```text
Direct Regulatory Graph
```

Тобто Genome — це регуляторний граф, який:

* читає локальні inputs;
* формує process priorities;
* не є behavior tree;
* не є blueprint організму;
* не містить hardcoded organs або cell types.

---

# Альтернативи для майбутнього дослідження

Можливі варіанти:

```text
Direct Regulatory Graph
Linear Instruction Genome
Fragment-based Genome
Chemical Tag Genome
Plasmid-like Genome Pool
Hybrid Graph + Fragments
```

Ці варіанти не треба реалізовувати автоматично.

Вони мають розглядатися лише тоді, коли базова модель покаже обмеження поточної моделі.

---

# Коли доповнювати файл

Файл варто доповнювати, коли:

* стане зрозуміло, що Direct Regulatory Graph обмежує еволюцію;
* знадобиться складніша recombination або HGT model;
* виникне потреба в modular genome fragments;
* будуть результати експериментів із різними genome encodings;
* буде прийматися ADR щодо нового представлення Genome.

---

# Що НЕ робити зараз

Не потрібно зараз:

* детально проектувати всі genome encodings;
* реалізовувати кілька genome formats;
* додавати behavior tree genome;
* додавати blueprint організму;
* змінювати Genome базової моделі model без ADR.

---

# Semantic Links

- compares alternatives for: [[docs/genetics/genome-representation|Genome Representation]]
- informs: [[docs/biology/genome|Genome]]
- may require ADR in: [[docs/decisions/README|Decisions]]

# Пов'язані документи

* `docs/biology/genome.md`
* `docs/genetics/regulatory-network.md`
* `docs/genetics/genome-runtime.md`
* `docs/genetics/mutation.md`
* `docs/genetics/recombination.md`
* `docs/research/rejected-ideas.md`

---

# Notes for Agents

Якщо цей файл виглядає коротким, це очікувано.

Не розширювати його під час аудиту документації без конкретної задачі на research або ADR.

---



