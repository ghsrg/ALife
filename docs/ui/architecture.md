---
tags:
  - alife
  - ui
  - canon
---

# UI Architecture

## Призначення

Цей документ визначає функціональну архітектуру `UI Layer`:

- головну оболонку застосунку;
- верхньорівневі робочі простори;
- спільний контекст між ними;
- глобальні функції;
- межі між monitoring, configuration, experiments, evolution та analytics;
- повторне використання `Viewer`, inspectors та інших спільних UI-компонентів.

Конкретний layout, розміщення панелей і правила переходів описуються в `navigation.md`.

## Головний застосунок

Canonical користувацький застосунок має назву:

```text
ALife Control Center
```

Співвідношення понять:

```text
UI Layer
  архітектурний користувацький шар

ALife Control Center
  основний користувацький застосунок

Viewer
  просторова карта World усередині застосунку
```

`ALife Control Center` є єдиною application shell, у межах якої користувач перемикається між робочими просторами.

UI не повинен бути набором ізольованих сторінок із незалежним станом.

## Application Shell

Application shell забезпечує спільний контекст для всіх робочих просторів.

Глобально доступні:

- active run або selected run;
- current Tick;
- current time context;
- run state;
- basic run controls;
- active warnings;
- active `Analysis Level`, коли він доречний;
- selected entity або selected analytical context;
- theme;
- language;
- UI scale;
- accessibility preferences;
- rendering preferences.

Application shell не зберігає authoritative simulation state.

## Верхньорівневі робочі простори

`ALife Control Center` має шість основних робочих просторів:

```text
Monitor
World Editor
Experiments
Evolution
Library
Analysis
```

Назви є canonical назвами верхнього рівня.

Внутрішні вкладки та підрозділи можуть уточнюватися без зміни цієї архітектури, якщо не змінюється призначення workspace.

---

## Monitor

### Призначення

Основний простір спостереження за поточним, paused, recorded або completed World.

### Основні функції

- `Viewer`;
- live або recorded World observation;
- layers;
- primary Field або color mode;
- simple overlays;
- filters;
- entity selection;
- contextual inspectors;
- event stream;
- current warnings;
- basic live metrics;
- tracking selected Cell або OrganismView;
- placement workflow через approved commands.

`Monitor` є головним workspace після відкриття застосунку.

---

## World Editor

### Призначення

Створення та валідація початкової конфігурації World до запуску simulation.

### Основні функції

- World dimensions та spatial rules;
- initial Resource distributions;
- Fields;
- hazards;
- obstacles;
- environment cycles;
- initial Cells, Species та Organisms;
- scenario parameters;
- deterministic seeds;
- preview;
- validation;
- config diff;
- config hash preview;
- export validated configuration.

`World Editor` не редагує active `WorldState` напряму.

Будь-які зміни створюють або оновлюють configuration artifact до запуску нового run.

---

## Experiments

### Призначення

Визначення того, що запускати, з якими параметрами та як організувати контрольоване порівняння.

### Основні функції

- single run setup;
- batch runs;
- parameter sweeps;
- matrix experiments;
- seed sets;
- control and intervention plans;
- checkpoints;
- branching;
- run queue;
- experiment definitions;
- experiment execution status;
- artifact collection;
- handoff selected runs to `Analysis`.

`Branching` є режимом `Experiments`, а не окремим верхньорівневим workspace.

`Experiments` відповідає на питання:

```text
що запустити
з якими параметрами
які runs входять до experiment
який control використовується
які branches потрібно створити
```

---

## Evolution

### Призначення

Спостереження за Genome, lineages, mutations, inheritance та emergent specialization.

### Основні функції

- Genome inspection;
- lineage trees;
- mutation history;
- inheritance;
- population change;
- Genome similarity;
- `Derived Classification`;
- `Potential Functional Role`;
- `Observed Functional Role`;
- `Behavior Profile`;
- spatial distribution of lineages and classifications;
- transition from evolutionary summary to selected entities in `Monitor`.

`Evolution` не визначає behavior і не змінює Genome Runtime.

---

## Library

### Призначення

Єдина точка доступу до системних визначень і збережених користувацьких artifacts.

`Library` має дві основні секції:

```text
System Catalog
Saved Assets
```

### System Catalog

Read-only або documentation-linked каталог зареєстрованих системних понять:

- Resource types;
- Material types;
- Fields;
- Processes;
- Capabilities;
- action/process registry entries;
- classification definitions;
- supported entity schemas;
- compatibility metadata.

`System Catalog` не замінює Canon-документацію та не редагує core registries через локальний UI state.

### Saved Assets

Користувацькі artifacts:

- saved Cells;
- Species;
- Organisms;
- scenario templates;
- configs;
- checkpoints;
- reusable experiment definitions;
- compatible import artifacts.

`Library` підтримує:

- search;
- tags;
- inspect;
- compare;
- validate;
- import;
- export;
- archive;
- delete;
- initiate placement.

`Resource Library` не є окремим верхньорівневим workspace.

---

## Analysis

### Призначення

Дослідження того, що сталося в одному або кількох runs, чому це сталося та які закономірності спостерігаються.

### Основні функції

- dashboards;
- time-series;
- Resource cycles;
- Energy flows;
- Material accounting;
- population distributions;
- Organism-size distributions;
- Functional Cell Role analysis;
- Behavior Profile analysis;
- balance and conservation views;
- warnings;
- anomaly summaries;
- control vs intervention comparison;
- run comparison;
- branch comparison;
- scientific reports;
- raw metrics and logs.

`Run Comparison` є режимом `Analysis`, а не окремим верхньорівневим workspace.

`Analysis` відповідає на питання:

```text
що сталося
чому сталося
які закономірності виникли
які стратегії домінують
де є imbalance
наскільки результат відтворюваний
```

## Analysis Level

Canonical `Analysis Levels`:

```text
World
Cells
Organisms
Lineages
Evolution
Analytics
```

`Analysis Level` є глобальною концепцією, але не повинен постійно відображатися в усіх workspaces.

Він доступний там, де реально змінює рівень спостереження:

- `Monitor`;
- `Evolution`;
- `Analysis`.

Він може бути прихований або замінений контекстною навігацією в:

- `World Editor`;
- `Library`;
- setup-частині `Experiments`.

Зміна `Analysis Level` не змінює simulation state.

## Data Context

`Data Context` — UI-специфічне поняття, яке визначає джерело та часовий контекст даних, показаних у workspace.

Можливі джерела:

- live run;
- paused run;
- recorded run;
- completed run;
- historical Tick;
- selected interval;
- checkpoint;
- branch;
- experiment result.

`Data Context` може містити:

```text
run id
branch id
checkpoint id
Tick або interval
live / recorded / historical mode
data completeness
projection version
```

UI повинен явно показувати активний `Data Context`.

Користувач не повинен плутати:

- active World;
- recorded frame;
- checkpoint;
- branch;
- historical interval;
- aggregated experiment result.

`Data Context` належить до UI Canon і не є обов'язковим глобальним терміном `GLOSSARY.md`.

## Cross-Workspace Context

Під час переходу між workspaces UI зберігає сумісний контекст:

- active run;
- selected Tick або interval;
- selected Cell;
- selected OrganismView;
- selected Genome;
- selected lineage;
- active `Analysis Level`;
- primary Field або color mode;
- compatible filters.

Приклад:

```text
select lineage in Evolution
  -> open Monitor
  -> highlight the same lineage in Viewer
```

Несумісний filter або selection:

- не повинен застосовуватися мовчки;
- може зберігатися як inactive context;
- повинен бути видимим, якщо впливає на очікування користувача;
- може бути відновлений після повернення до сумісного workspace.

## Shared Viewer

UI використовує один canonical `Viewer`, а не незалежні реалізації карти World.

`Viewer` повторно використовується в:

- `Monitor`;
- preview у `World Editor`;
- spatial views у `Evolution`;
- spatial analysis у `Analysis`;
- placement workflow.

Залежно від workspace змінюються:

- `Data Context`;
- available layers;
- available overlays;
- interaction permissions;
- preview state;
- allowed commands.

Rendering semantics, entity identity та world-to-screen mapping повинні залишатися узгодженими.

## Contextual Inspector Framework

UI використовує єдиний contextual inspector framework.

Inspector визначається типом selection:

- Cell;
- OrganismView;
- Genome;
- lineage;
- Resource;
- Material;
- Field;
- Process;
- run;
- experiment;
- checkpoint;
- saved asset.

Inspector може:

- бути docked panel;
- розгортатися в detail view;
- закріплюватися;
- брати участь у comparison;
- відкривати пов'язану сутність або workspace.

Inspector не є окремим верхньорівневим workspace.

## Placement Workflow

Placement можна почати з:

- `Monitor`;
- `Library`.

Обидва входи використовують один canonical workflow:

```text
select saved asset
  -> enter placement mode
  -> choose requested position
  -> preview footprint
  -> validate
  -> confirm
  -> submit approved command
  -> core applies at allowed Tick
  -> intervention is recorded
  -> Viewer shows committed result
```

Placement не є прямим редагуванням `WorldState`.

## Workspace Composition

У межах одного application window активний один верхньорівневий workspace.

Дозволені:

- docked panels;
- collapsible panels;
- temporary detail views;
- modal або non-modal contextual flows;
- full-screen `Viewer`;
- pinned inspector;
- internal split layout у межах одного workspace.

Довільний multi-workspace split у межах одного window не є вимогою.

Для паралельного перегляду користувач може відкрити інший workspace, run або view в окремому browser window чи application window.

Окремі windows не повинні ділити mutable UI state неявно.

## Initial Application State

Після запуску застосунок відкриває `Monitor`.

Якщо існує active або recent run:

```text
Monitor shows that run and its Data Context.
```

Якщо run відсутній:

```text
Monitor shows an honest empty state.
```

Empty state пропонує основні дії:

- create World;
- open scenario;
- run example;
- open completed run;
- open Library.

Окремий обов'язковий Home workspace не використовується.

## Global Command Surface

Application shell містить лише базові й часто потрібні run controls:

- start;
- pause;
- resume;
- stop;
- single-step;
- execution speed;
- current Tick;
- run state.

Контекстні дії залишаються у відповідних workspaces:

```text
branch
place asset
edit config
launch sweep
compare runs
export
delete artifact
```

Top-level command surface не повинен перетворюватися на перелік усіх можливих дій системи.

## Global UI Settings

UI має окремий settings surface, доступний із application shell.

Settings можуть містити:

### Language And Text

- interface language;
- number formatting;
- date and time formatting;
- unit formatting;
- text size where supported.

### Appearance

- light theme;
- dark theme;
- system theme preference, якщо реалізація її підтримує;
- UI scale;
- density;
- contrast preference;
- color accessibility options.

### Motion

- decorative animations on/off;
- reduced motion;
- transition intensity, якщо підтримується.

### Rendering

- target FPS;
- rendering quality;
- anti-aliasing preference;
- maximum visible labels;
- adaptive LOD;
- performance preset;
- frame interpolation, якщо реалізація її підтримує.

### Behavior Of The UI

- restore last workspace;
- restore compatible filters;
- persist panel sizes;
- default full-screen behavior;
- confirmation preferences для non-destructive actions;
- default export location, якщо frontend platform це дозволяє.

UI settings:

- не входять у simulation config;
- не входять у behavior hash;
- не змінюють Tick semantics;
- не змінюють simulation priority;
- не повинні змінювати значення analytics;
- можуть впливати лише на presentation, rendering frequency та локальну взаємодію.

Зниження target FPS або rendering quality не повинно зменшувати частоту Tick чи змінювати simulation result.

## Navigation State

Workspace, selected entity, selected run та time context повинні мати відновлюваний navigation state, якщо це підтримує обрана frontend architecture.

У web implementation це може бути:

- URL;
- route state;
- shareable deep link.

В іншій реалізації це може бути:

- navigation descriptor;
- restore token;
- saved view reference.

Ця вимога не нав'язує конкретний routing framework і не повинна перешкоджати кращій архітектурі frontend.

Мета:

- повернення до попереднього view;
- відновлення після reload;
- відкриття конкретного run;
- відкриття конкретної entity;
- передача відтворюваного аналітичного контексту;
- відкриття view в окремому window.

Navigation state не є simulation artifact, якщо користувач явно не експортує view configuration.

## Functional Flow Between Workspaces

Основний потік:

```text
World Editor
  -> validated scenario config
  -> Experiments
  -> run or batch execution
  -> Monitor
  -> Evolution / Analysis
  -> Library or exported artifacts
```

Альтернативні переходи:

```text
Library
  -> select asset
  -> Monitor placement workflow

Evolution
  -> select lineage or Genome
  -> Monitor spatial highlight

Analysis
  -> detect interesting run
  -> Monitor selected Tick
  -> Experiments branch from checkpoint

Monitor
  -> save Cell / Species / Organism
  -> Library
```

Переходи повинні зберігати сумісний `Data Context`.

## Архітектурні обмеження

Заборонено:

- створювати окрему simulation truth у кожному workspace;
- мати різні семантики `Viewer` для різних workspaces;
- дублювати classification logic у UI components;
- змішувати UI settings із scenario config;
- використовувати active filter як simulation input;
- редагувати core registries через presentation-only controls;
- приховувати intervention за звичайною UI interaction;
- змінювати simulation result через rendering settings;
- мовчки перемикати run або time context.

## Пов'язані документи

- `GLOSSARY.md`
- `PRINCIPLES.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
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
- indexed by: [[docs/ui/README|UI Layer]]
- follows: [[docs/ui/principles|UI Principles]]
- defines structure for: [[docs/ui/navigation|UI Navigation]]
- defines shared Viewer for: [[docs/ui/visualization|UI Visualization]]
- defines analytics workspace for: [[docs/ui/analytics|UI Analytics]]
- defines inspectors for: [[docs/ui/exploration|UI Exploration]]
- defines settings scope for: [[docs/ui/presentation|UI Presentation]]
- defines command contexts for: [[docs/ui/interaction|UI Interaction]]
- constrains implementation quality in: [[docs/ui/quality|UI Quality]]
