---
tags:
  - alife
  - worklog/report
---

# REPORT: doc-cleanup-optimization

Дата: 2026-06-30 13:53

План: `outputs/worklogs/2026-06-30-1319-PLAN-doc-cleanup-optimization.md`

---

# Що виконано

## 1. Механічна чистка шуму

Прибрано службові маркери code fences виду:

```text
```text id="..."
```

Також прибрано механічні повтори на кшталт:

```text
Базова модель базової моделі
базова модель базова модель
Recommendation базової моделі
Recommended базова модель
```

Це зачепило частину файлів поза основними rewrite-блоками, бо чистка виконувалась по всіх `docs/**/*.md`.

## 2. Винесено приклади

Створено каталог:

```text
docs/examples/
```

Додано файли:

```text
docs/examples/README.md
docs/examples/biology-examples.md
docs/examples/genetics-examples.md
docs/examples/config-examples.md
docs/examples/engine-examples.md
```

Правило: examples не створюють Canon. Якщо приклад суперечить Canon, правильним вважається Canon.

## 3. Стиснуто biology Canon

Скорочено й сфокусовано:

```text
docs/biology/cell.md
docs/biology/communication.md
docs/biology/genome.md
docs/biology/joint.md
docs/biology/lifecycle.md
docs/biology/membrane.md
docs/biology/organism.md
docs/biology/processes.md
docs/biology/specialization.md
```

Основний результат:

- `cell.md` залишив мінімальний стан клітини й межу відповідальності.
- `genome.md` став мостом до `docs/genetics/`, без дублювання runtime/mutation/inheritance.
- `organism.md` зафіксував `Organism` як observer-side view, не engine-controller.
- `joint.md` залишив матеріальний контракт Joint і правила division/death.
- `communication.md` залишив локальні channels, signal contract і tick-causality.
- `processes.md` став описом process groups і посилається на `feasibility`, `process-progress`, `process-capabilities`.

## 4. Стиснуто genetics Canon

Скорочено:

```text
docs/genetics/genome-representation.md
docs/genetics/regulatory-network.md
docs/genetics/genome-runtime.md
docs/genetics/mutation.md
docs/genetics/inheritance.md
docs/genetics/heredity.md
docs/genetics/epigenetics.md
docs/genetics/recombination.md
docs/genetics/horizontal-transfer.md
```

Основний результат:

- `Genome as Direct Regulatory Graph` зафіксовано як поточну базову реалізацію, яку можна коригувати.
- Runtime, graph structure, representation, heredity, inheritance і mutation розділені по відповідальності.
- Recombination і HGT залишені як future-compatible механіки без вимоги реалізовувати першими.

## 5. Стиснуто config Canon

Скорочено:

```text
docs/config/world_config.md
docs/config/resources_config.md
docs/config/materials_config.md
docs/config/fields_config.md
docs/config/reactions_config.md
docs/config/stability_bounds.md
```

Основний результат:

- залишено мінімальні YAML-схеми;
- додано короткі validation rules;
- прибрано навчальні пояснення;
- `Space` явно не є `ResourceType`;
- `toxicity` не введено, шкідливість лишається через reactions/material/heat/capacity.

## 6. Стиснуто engine Canon

Скорочено:

```text
docs/engine/ecs.md
docs/engine/scheduler.md
docs/engine/chemistry.md
docs/engine/physics.md
docs/engine/performance.md
docs/engine/rendering.md
docs/engine/serialization.md
docs/engine/storage.md
```

Основний результат:

- engine-файли більше не дублюють біологію;
- scheduler відділено від tick як optimization/organization layer;
- observer views зафіксовані як read-only для behavior;
- performance не може міняти semantics.

## 7. Стиснуто world Canon

Скорочено:

```text
docs/world/space.md
docs/world/physics.md
docs/world/resources.md
docs/world/energy.md
docs/world/fields.md
docs/world/materials.md
```

Основний результат:

- `space.md` став відповіддю на координати, межі й локальність.
- `energy.md` зафіксував Energy Buffer, partition during division і planned actions rule.
- `physics.md` зафіксував `temperature` клітини та локальний Heat без обов'язкового глобального heat field.
- `resources.md`, `materials.md`, `fields.md` стали короткими контрактами.

## 8. Оновлено навігацію і стиль

Оновлено:

```text
docs/README.md
docs/STYLE_GUIDE.md
```

Додано `docs/examples/` у структуру.

У `STYLE_GUIDE.md` зафіксовано:

- Canon має бути коротким;
- довгі examples/traces/config walkthrough переносяться в `docs/examples/`;
- приклад не є правилом.

---

# Перевірка

Команди:

```powershell
rg -n 'id="' docs README.md
rg -n 'MVP|перша симуляція|first simulation|першої симуляції' docs README.md
rg -n 'базова модель базова модель|Базова модель базової моделі|Recommendation базової моделі|Recommended базова модель' docs README.md
Get-ChildItem docs -Recurse -Filter *.md | ForEach-Object { ... line count ... } | Sort-Object Lines -Descending | Select-Object -First 25
git diff --stat -- docs README.md
git status --short
```

Результати:

- `id="..."` не знайдено.
- MVP/first simulation згадки не знайдено.
- Механічні фрази `базова модель базова модель` та споріднені не знайдено.
- Найбільші Canon-файли після чистки стали значно коротшими; найбільші залишки тепер переважно navigation/principles/evolution/research, які не були основним rewrite-scope цього кроку.
- `git diff --stat -- docs README.md`: близько `1534 insertions`, `31200 deletions` у tracked docs.

---

# Відомі побічні ефекти

## Line endings

Git показав попередження:

```text
LF will be replaced by CRLF the next time Git touches it
```

Це наслідок робочого налаштування Git/Windows. Змістовно файли оновлені, але перед комітом варто вирішити, чи нормалізувати line endings через `.gitattributes`.

## Unrelated Obsidian changes

У working tree до цієї роботи вже були unrelated зміни:

```text
D docs/.obsidian/app.json
D docs/.obsidian/appearance.json
D docs/.obsidian/core-plugins.json
D docs/.obsidian/graph.json
D docs/.obsidian/workspace.json
?? .obsidian/
```

Я їх не чіпав і не відновлював.

## Link-audit

Obsidian-сумісні links/backlinks не перевірялися в цьому кроці за домовленістю. Це окремий наступний етап.

---

# Залишилось На Наступні Кроки

- Провести окремий link-audit і привести посилання до Obsidian-сумісної схеми.
- Окремо вирішити line endings policy.
- За потреби повторити аналогічну стислу чистку `evolution/`, `research/`, `world/laws.md`, `world/philosophy.md`, `PRINCIPLES.md`, `GLOSSARY.md`.
