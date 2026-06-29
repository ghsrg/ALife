# docs/research/rejected-ideas.md

> **Rejected Ideas — ідеї, які не входять у канон або MVP**

---

# Статус

Active guardrail document.

Цей файл потрібен постійно.

Його задача — захищати архітектуру від повернення до простих, але неправильних рішень.

---

# Призначення

`rejected-ideas.md` фіксує підходи, які не можна вводити без окремого ADR.

Цей файл особливо важливий для агентів, які під час реалізації можуть запропонувати “зручні” shortcuts.

---

# Rejected: species_id as behavior input

Не вводити:

```text id="ydgbh7"
species_id
```

як поведінковий input для клітини.

Причина:

* species-like groups мають виникати аналітично;
* compatibility має бути фізичною або регуляторною;
* клітина не повинна знати “якого вона виду”.

---

# Rejected: fitness_score input

Не вводити:

```text id="tojm45"
fitness_score
```

як input для Genome Runtime.

Причина:

* selection має бути emergent;
* fitness можна рахувати лише аналітично;
* Genome не повинен бачити глобальну оцінку.

---

# Rejected: HP damage

Не вводити:

```text id="0t2imp"
cell.hp
organism.hp
damage_points
```

Причина:

* damage має бути Material damage;
* death має виникати через structural failure;
* organ-level HP суперечить фізичній моделі.

---

# Rejected: hardcoded cell types

Не вводити:

```text id="glep42"
NeuronCell
MuscleCell
SkinCell
PlantCell
AnimalCell
PredatorCell
PreyCell
```

Причина:

* specialization має бути emergent;
* roles — це observed analytics;
* функція виникає з Materials, Genome Runtime і environment.

---

# Rejected: organism as global controller

Не вводити `Organism`, який керує клітинами.

Причина:

* cells remain primary agents;
* organism-like structure — це Cell-Joint graph;
* communication локальна;
* global brain не є MVP.

---

# Rejected: behavior tree genome

Не вводити Genome як behavior tree.

Причина:

* Genome має регулювати процеси;
* не має бути script of behavior;
* не має бути AI-controller для клітини.

---

# Rejected: direct poison / healing

Не вводити:

```text id="ewqo96"
toxicity
poison_damage
healing_resource
```

Причина:

* шкідливість має виникати через реакції, volume, blocking, Heat, Material damage;
* repair має бути Material repair із cost.

---

# Rejected: direct Energy transfer

Не вводити пряме передавання Energy Buffer між клітинами.

Причина:

* Energy Buffer локальний;
* через Joint можуть передаватися Resources, Heat, Signals, але не Energy Buffer напряму.

---

# Rejected: hardcoded organs and tissues

Не вводити:

```text id="8ukl5g"
Heart
Brain
Root
Leaf
Skin
Vessel
Nerve
```

Причина:

* tissue-like і organ-like patterns мають бути emergent;
* renderer/debug може показувати observed patterns, але не керувати ними.

---

# Rejected: guaranteed useful mutation

Не вводити mutation, яка “покращує” Genome.

Причина:

* mutation випадкова;
* harmful і lethal mutations дозволені;
* selection фільтрує наслідки.

---

# Rejected: scenario-spawned ready organisms

World scenario не повинен спавнити готові species або organisms.

Причина:

* scenario задає conditions;
* organisms мають виникати з simulation.

---

# Коли доповнювати файл

Файл треба доповнювати кожного разу, коли:

* агент пропонує shortcut, що суперечить канону;
* прийнято рішення не реалізовувати певний підхід;
* з'являється повторювана спокуса спростити модель;
* створюється ADR із відхиленням альтернативи.

---

# Як використовувати

Під час аудиту агент має перевіряти нові пропозиції проти цього файлу.

Якщо ідея є тут, її не можна реалізовувати без нового ADR.

---

# Пов'язані документи

* `docs/PRINCIPLES.md`
* `docs/world/laws.md`
* `docs/biology/cell.md`
* `docs/biology/organism.md`
* `docs/genetics/genome-runtime.md`
* `docs/evolution/selection.md`
* `docs/research/genome-representation-options.md`

---

# Notes for Agents

Цей файл не є порожнім placeholder.

Це активний guardrail document.

---

