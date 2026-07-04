---
tags:
  - alife
  - ui
  - canon
---

# UI Layer

## Призначення

`UI Layer` — користувацький шар Artificial Life Engine для спостереження, дослідження, конфігурації, запуску та порівняння симуляцій.

`UI Layer` дозволяє:

- спостерігати за `World`, `Cells`, `OrganismView`, `Resources`, `Materials`, `Fields`, `Genome`, lineages та подіями;
- запускати, призупиняти, продовжувати, покроково виконувати й перезапускати simulation runs;
- створювати та перевіряти конфігурації World;
- проводити експерименти та порівнювати результати;
- досліджувати окремі сутності й агреговані population views;
- працювати із saved Cells, Species, Organisms, checkpoints та experiment artifacts;
- переглядати analytics, Derived Classification, Functional Cell Roles і Behavior Profiles;
- експортувати screenshots, configs, reports та інші artifacts.

`UI Layer` може мати web implementation, але UI Canon визначає поведінку та вимоги незалежно від конкретного frontend stack.

## Основна межа

```text
Viewer observes.
UI requests.
Core validates.
Core applies.
Core records.
UI observes the result.
```

Обов'язкові правила:

```text
UI не змінює WorldState напряму.
UI state не є simulation state.
Viewer не є simulation authority.
Observer analytics не є behavior input.
Derived Classification не керує Genome Runtime або Process selection.
Display settings не впливають на Tick execution.
```

`UI Layer` використовує лише:

- read-only `Projection`;
- committed snapshots;
- events;
- summaries;
- derived analytics results;
- explicit runner APIs;
- approved command APIs.

## Склад UI Layer

До `UI Layer` належать:

- `Viewer`;
- глобальна навігація;
- робочі простори;
- inspectors;
- dashboards;
- analytics views;
- World configuration tools;
- experiment tools;
- saved asset libraries;
- run controls;
- checkpoint та branch tools;
- themes;
- localization;
- contextual help.

До `UI Layer` не належать:

- `alife-core`;
- `Observer Layer`;
- analytics module;
- simulation rules;
- classification algorithms;
- authoritative `WorldState`;
- behavior decision logic.

Архітектурна залежність:

```text
alife-core
  -> фактичний стан і події

Observer Layer / analytics module
  -> Projection
  -> summaries
  -> Derived Classification
  -> derived analytics results

UI Layer
  -> відображення
  -> навігація
  -> аналіз
  -> explicit requests and approved commands
```

## Аудиторія

UI створюється для таких сценаріїв використання:

- розробка та debugging;
- наукове спостереження;
- створення й проведення експериментів;
- аналіз результатів;
- перевірка відтворюваності;
- демонстрація simulation behavior;
- науковий перегляд і пояснення результатів.

Ці сценарії не створюють окремих permission roles.

## Термінологія

UI Canon використовує концептуальні терміни з `GLOSSARY.md`.

UI-документи можуть визначати терміни, специфічні для інтерфейсу, якщо вони не є загальними концептами рушія.

Новий термін додається до `GLOSSARY.md`, якщо він:

- має архітектурне або концептуальне значення;
- може мати кілька трактувань;
- потребує стабільної межі з іншим поняттям;
- використовується поза межами одного UI-документа;
- впливає на розуміння моделі системи.

Тексти пояснень пишуться українською.

Canonical terms, назви класів, API, state та technical identifiers можуть залишатися англійською.

## Структура UI Canon

Усі файли каталогу `docs/ui/` є Canon-документами.

### `README.md`

Точка входу до UI Canon.

Визначає:

- призначення `UI Layer`;
- межі відповідальності;
- структуру документації;
- порядок читання;
- зв'язок із глобальною документацією.

### `principles.md`

Визначає незмінні принципи UI:

- observer-only behavior;
- world-first visualization;
- scientific transparency;
- reproducibility;
- progressive disclosure;
- consistent terminology;
- відсутність прихованих simulation mutations.

### `architecture.md`

Визначає функціональну архітектуру UI:

- категорії робочих просторів;
- їх призначення;
- основні сутності;
- основні потоки переходів;
- межі між monitoring, editing, experiments та analytics.

Остаточні назви робочих просторів визначаються в цьому документі.

### `navigation.md`

Визначає:

- глобальну навігацію;
- `Analysis Level`;
- просторову структуру екрана;
- основні області layout;
- переходи між рівнями та робочими просторами;
- responsive behavior;
- full-screen та panel behavior.

### `visualization.md`

Визначає:

- правила відображення World;
- world-to-screen mapping;
- `Viewport`;
- Semantic Zoom;
- LOD;
- heatmaps;
- overlays;
- color modes;
- Cell та Organism rendering;
- aggregation і viewport scaling.

### `analytics.md`

Визначає:

- dashboards;
- charts;
- Resource та Energy flows;
- time-series;
- distributions;
- dominance views;
- Organism-size analysis;
- Functional Cell Role analysis;
- experiment comparison;
- warnings та event summaries.

### `exploration.md`

Визначає контекстне дослідження даних:

- inspectors;
- filters;
- selection;
- multi-selection;
- pinning;
- comparison;
- chart-to-world linking;
- world-to-chart linking;
- entity tracking.

### `presentation.md`

Визначає правила сприйняття UI:

- dark, light та system themes;
- accessibility;
- localization behavior;
- UI text organization;
- зв'язок із `GLOSSARY.md`;
- contextual help;
- tooltip та інформаційні підказки;
- formatting of numbers, units, dates and messages.

### `interaction.md`

Визначає:

- hover, focus, click, selection, zoom та pan;
- confirmation rules;
- UI commands;
- read-only interactions;
- intervention-producing actions;
- UI state;
- відокремлення UI state від simulation state;
- persistence of user preferences.

### `quality.md`

Визначає технічну якість UI:

- payload limits;
- performance;
- virtualization;
- lazy loading;
- rendering budgets;
- visual regression;
- interaction testing;
- theme parity;
- localization completeness;
- contextual-help coverage;
- Projection correctness.

## Рекомендований порядок читання

```text
1. README.md
2. principles.md
3. architecture.md
4. navigation.md
5. visualization.md
6. analytics.md
7. exploration.md
8. presentation.md
9. interaction.md
10. quality.md
```

Порядок відображає залежність:

```text
межі
  -> принципи
  -> функціональна архітектура
  -> навігація
  -> візуалізація
  -> аналітика
  -> дослідження об'єктів
  -> представлення для користувача
  -> взаємодія
  -> технічна якість
```

## Зв'язок із глобальною документацією

### `GLOSSARY.md`

Містить загальні концептуальні терміни проєкту.

UI Canon використовує ці визначення та розширює їх лише UI-специфічними правилами.

### `PRINCIPLES.md`

Містить глобальні принципи Artificial Life Engine.

`docs/ui/principles.md` деталізує їх застосування до UI.

UI principles не можуть суперечити global principles.

### `STYLE_GUIDE.md`

Визначає правила написання Canon-документації.

Усі файли `docs/ui/` повинні відповідати цьому стандарту.

### Plans

Plans визначають:

- порядок реалізації;
- фази;
- залежності;
- Gates;
- TDD-плани;
- worklogs.

UI Canon визначає:

- затверджену поведінку;
- стабільні межі;
- структуру;
- UX-контракти;
- критерії правильності.

Якщо Plan суперечить UI Canon, правильним вважається UI Canon, а Plan потрібно оновити.

### Research

Research містить:

- альтернативи;
- прототипи;
- порівняння;
- неперевірені ідеї;
- відкриті варіанти дизайну.

Research не є вимогою до реалізації, доки рішення не перенесене до UI Canon.

## Правила змін

Зміни в UI Canon повинні:

1. використовувати терміни з `GLOSSARY.md`;
2. не суперечити `PRINCIPLES.md`;
3. не передавати UI simulation authority;
4. не перетворювати observer-side labels на behavior input;
5. оновлювати пов'язані UI-документи, якщо змінюється спільний контракт;
6. оновлювати Plans, якщо змінюються вимоги або порядок реалізації.

## Пов'язані документи

- `GLOSSARY.md`
- `PRINCIPLES.md`
- `STYLE_GUIDE.md`
- `ROADMAP.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/analytics.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`

# Semantic Links

- governed by: [[docs/PRINCIPLES|Principles]]
- uses terms from: [[docs/GLOSSARY|Glossary]]
- follows: [[docs/STYLE_GUIDE|Documentation Style Guide]]
- defines index for: [[docs/ui/principles|UI Principles]]
- defines index for: [[docs/ui/architecture|UI Architecture]]
- defines index for: [[docs/ui/navigation|UI Navigation]]
- defines index for: [[docs/ui/visualization|UI Visualization]]
- defines index for: [[docs/ui/analytics|UI Analytics]]
- defines index for: [[docs/ui/exploration|UI Exploration]]
- defines index for: [[docs/ui/presentation|UI Presentation]]
- defines index for: [[docs/ui/interaction|UI Interaction]]
- defines index for: [[docs/ui/quality|UI Quality]]
