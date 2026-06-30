---
tags:
  - alife
  - docs/index
---

# Документація Artificial Life Engine

> Навігатор по поточній документації проєкту.

---

# Obsidian

Цей файл лишається навігатором, але предметний граф має будуватися з семантичних ребер у самих документах.

Робочі плани й звіти зібрані окремо: [[outputs/worklogs/README|outputs/worklogs]].

---

# Призначення

Каталог `docs/` містить специфікацію світу, біології, генетики, еволюції, конфігурацій, рушія та дослідницьких напрямків Artificial Life Engine.

Документація є основним джерелом істини для майбутньої реалізації.

Якщо майбутній код суперечить документації, спочатку треба виправити код або змінити документацію через прийняте рішення.

Поточні статуси документів, етапи розвитку та пріоритети ведуться в `docs/ROADMAP.md`.

---

# Рівні документації

```text
Principles
    ↓
Canon
    ↓
Research
    ↓
ADR
    ↓
Implementation
```

## Principles

`PRINCIPLES.md` — верхній рівень правил.

Жоден нижчий документ не повинен йому суперечити.

## Canon

Canon-документи описують поточні правила світу та майбутньої реалізації.

До Canon належать основні документи в:

* `world/`
* `biology/`
* `genetics/`
* `evolution/`
* `config/`
* `engine/`

## Research

`research/` містить гіпотези, альтернативи, відкладені ідеї та майбутні напрямки.

Research не є специфікацією для реалізації.

## ADR

`decisions/` містить Architecture Decision Records.

ADR створюється лише тоді, коли є реальна причина зафіксувати рішення: фундаментальна зміна, вибір між альтернативами, відхилення важливої ідеї або перенесення Research-рішення в Canon.

Неіснуючий ADR не вважається знанням проєкту.

---

# Як читати документацію

## Новий учасник проєкту

```text
README.md
    ↓
docs/PRINCIPLES.md
    ↓
docs/GLOSSARY.md
    ↓
docs/ROADMAP.md
    ↓
docs/world/
    ↓
docs/biology/
    ↓
docs/genetics/
    ↓
docs/evolution/
    ↓
docs/config/
    ↓
docs/engine/
```

## Перед зміною документації або реалізацією

1. Прочитати `PRINCIPLES.md`.
2. Перевірити терміни в `GLOSSARY.md`.
3. Прочитати відповідний Canon-документ.
4. Перевірити пов'язані Research-документи.
5. Перевірити наявні ADR у `decisions/`.

Якщо потрібного правила немає, його не можна вигадувати мовчки. Треба створити TODO, Open Question або ADR-пропозицію.

---

# Поточна структура

```text
docs/
├── README.md
├── PRINCIPLES.md
├── GLOSSARY.md
├── ROADMAP.md
├── STYLE_GUIDE.md
│
├── world/
│   ├── philosophy.md
│   ├── laws.md
│   ├── tick.md
│   ├── tick-semantics.md
│   ├── space.md
│   ├── units.md
│   ├── fields.md
│   ├── field-semantics.md
│   ├── resources.md
│   ├── reactions.md
│   ├── materials.md
│   ├── energy.md
│   └── physics.md
│
├── biology/
│   ├── action-process-registry.md
│   ├── cell.md
│   ├── cell-state.md
│   ├── communication.md
│   ├── division-partition.md
│   ├── feasibility.md
│   ├── genome.md
│   ├── joint.md
│   ├── lifecycle.md
│   ├── membrane.md
│   ├── organism.md
│   ├── process-capabilities.md
│   ├── process-progress.md
│   ├── processes.md
│   └── specialization.md
│
├── genetics/
│   ├── epigenetics.md
│   ├── genome-representation.md
│   ├── genome-runtime.md
│   ├── heredity.md
│   ├── horizontal-transfer.md
│   ├── inheritance.md
│   ├── mutation.md
│   ├── regulatory-interface.md
│   ├── recombination.md
│   └── regulatory-network.md
│
├── evolution/
│   ├── adaptation.md
│   ├── population-dynamics.md
│   ├── selection.md
│   └── species-like-clusters.md
│
├── config/
│   ├── fields_config.md
│   ├── materials_config.md
│   ├── reactions_config.md
│   ├── resources_config.md
│   ├── stability_bounds.md
│   └── world_config.md
│
├── engine/
│   ├── chemistry.md
│   ├── ecs.md
│   ├── performance.md
│   ├── physics.md
│   ├── rendering.md
│   ├── scheduler.md
│   ├── serialization.md
│   └── storage.md
│
├── examples/
│   ├── README.md
│   ├── biology-examples.md
│   ├── config-examples.md
│   ├── engine-examples.md
│   └── genetics-examples.md
│
├── research/
│   ├── genome-representation-options.md
│   ├── graph-recombination-options.md
│   ├── mobile-genetic-elements.md
│   ├── rejected-ideas.md
│   └── reproduction-strategy-options.md
│
└── decisions/
    └── README.md
```

---

# Основні документи

## `PRINCIPLES.md`

Конституція світу.

Містить фундаментальні принципи, які визначають усю архітектуру проєкту.

## `GLOSSARY.md`

Єдиний словник термінів.

Кожне поняття повинно мати одне офіційне визначення.

## `ROADMAP.md`

Поточний стан проєкту.

Саме тут ведуться статуси, етапи, ризики, відкриті дослідження та базова модель-напрямок.

## `STYLE_GUIDE.md`

Правила написання й підтримки документації.

---

# Каталоги

## `world/`

Універсальні закони симуляції: час, простір, фізика, поля, ресурси, матеріали та енергія.

## `biology/`

Механіка життя: клітина, мембрана, Joint, організм, процеси, реєстр actions/processes, комунікація, спеціалізація та життєвий цикл.

## `genetics/`

Модель спадковості, геному, регуляторної мережі, мутацій, рекомбінації, епігенетики та горизонтального переносу.

## `evolution/`

Оглядовий рівень очікуваних еволюційних явищ на рівні популяцій.

Цей каталог не повинен дублювати механіки `genetics/`.

## `config/`

Документація майбутніх конфігурацій світу, ресурсів, матеріалів і полів.

## `engine/`

Технічна архітектура майбутнього рушія.

Engine не визначає законів світу, а реалізує їх.

## `examples/`

Ілюстративні сценарії для читання разом із Canon.

Examples не створюють нових правил. Якщо приклад суперечить Canon, правильним вважається Canon.

## `research/`

Лабораторія ідей.

Тут можна зберігати альтернативи, відкладені рішення, rejected ideas і майбутні напрямки, але не реалізаційні правила.

## `decisions/`

Журнал ADR.

Порожній журнал не означає, що рішення вже прийняті. Якщо ADR-файл відсутній, його не можна цитувати як джерело істини.

---

# Правила ведення документації

## Canon

Містить актуальні правила.

Не повинен містити неперевірені гіпотези або альтернативи.

## Research

Містить дослідження й альтернативи.

Може суперечити поточній архітектурі.

## ADR

Фіксує причину прийнятого рішення.

ADR потрібен для фундаментальних змін, вибору між альтернативами або відхилення важливої ідеї.

---

# Правила для AI-агентів

Перед зміною документації або майбутнього коду агент повинен:

1. прочитати `PRINCIPLES.md`;
2. перевірити `GLOSSARY.md`;
3. прочитати відповідний Canon-документ;
4. перевірити пов'язані Research-документи;
5. перевірити наявні ADR.

Якщо правило відсутнє:

* не вигадувати його самостійно;
* додати TODO або Open Question;
* або запропонувати ADR, якщо є реальна причина для рішення.

