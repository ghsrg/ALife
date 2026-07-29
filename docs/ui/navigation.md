---
tags:
  - alife
  - ui
  - canon
---

# UI Navigation

## Призначення

Цей документ визначає:

- глобальну навігацію `ALife Control Center`;
- навігацію між workspaces;
- використання `Analysis Level`;
- базову композицію layout;
- поведінку при обмеженому viewport;
- full-screen mode;
- detail views;
- navigation history;
- entity search та recent history;
- cross-workspace transitions;
- відновлення navigation state.

Конкретні interaction gestures описуються в `interaction.md`.

## Глобальна навігація

Основна навігація між workspaces розташовується у верхній частині application shell.

Canonical порядок:

```text
Monitor
World Editor
Experiments
Evolution
Library
Analysis
```

Глобальна навігація:

- завжди відокремлена від внутрішньої навігації workspace;
- не повинна займати значну частину горизонтального або вертикального простору;
- не повинна конкурувати з primary content;
- повинна залишатися доступною при переході між workspaces;
- може переходити в compact mode при обмеженій ширині.

`Settings` і `Help` належать до application-level navigation, але не є simulation workspaces.

## Compact Navigation

При достатній ширині глобальна навігація показує:

```text
icon + label
```

При обмеженій ширині або за налаштуванням користувача:

```text
icon only
```

Для compact mode:

- назва показується через hover, focus або tap;
- active workspace має чітко виділятися;
- icons не повинні бути єдиним носієм значення без accessible label;
- порядок workspaces не змінюється.

Допускається overflow menu, якщо всі пункти не вміщуються без зменшення primary content.

## Analysis Level Navigation

Canonical `Analysis Levels`:

```text
World
Cells
Organisms
Lineages
Evolution
Analytics
```

`Analysis Level` не змішується з global workspace navigation.

Він показується лише там, де реально змінює рівень спостереження:

- `Monitor`;
- `Evolution`;
- `Analysis`.

Розташування може адаптуватися до workspace:

- горизонтальна context bar над primary content;
- вертикальна compact bar біля `Viewer`;
- contextual control у workspace header.

У всіх випадках:

- порядок рівнів однаковий;
- active level завжди видимий;
- недоступні рівні мають бути явно disabled;
- зміна рівня не змінює simulation state;
- `Data Context` зберігається, якщо він сумісний.

## Canonical Workspace Layout

Workspaces використовують спільний композиційний каркас:

```text
global application header
workspace header
left contextual controls
primary content
right contextual detail
optional bottom panel
```

Не всі області повинні бути одночасно видимими.

Panels можуть бути:

- expanded;
- collapsed;
- resized;
- hidden;
- opened as overlay.

Primary content завжди має пріоритет над secondary panels.

## Monitor Layout

Default layout `Monitor`:

```text
top:
  Data Context
  run state
  current Tick
  speed
  basic controls

left:
  layers
  primary Field or color mode
  filters

center:
  Viewer

right:
  contextual Inspector

bottom:
  timeline
  events
  charts
  warnings
```

Це canonical default, а не жорстко фіксований pixel layout.

Користувач може:

- згортати panels;
- змінювати їхній розмір;
- приховувати secondary areas;
- розгортати `Viewer` на весь доступний простір.

## Other Workspace Layouts

### World Editor

```text
left:
  configuration parameters

center:
  World preview

right:
  validation and selected object details

bottom:
  config diff, warnings or generated summary
```

### Experiments

```text
left:
  experiment structure and run groups

center:
  experiment configuration or execution view

right:
  selected run or branch details

bottom:
  queue, logs, warnings or progress
```

### Evolution

```text
left:
  filters, level selectors and lineage controls

center:
  lineage tree, Genome view, chart or spatial Viewer

right:
  selected entity Inspector

bottom:
  timeline, mutation events or population trends
```

### Library

```text
left:
  System Catalog or Saved Assets categories

center:
  grid, list or catalog

right:
  preview, validation and metadata
```

### Analysis

```text
left:
  analysis controls and filters

center:
  primary chart, dashboard or spatial result

right:
  explanation, selected series or selected entity

bottom:
  timeline, raw metrics, events or warnings
```

Відхилення дозволені, якщо nature of the entity або task вимагає іншої композиції.

## Internal Workspace Navigation

Кожен workspace може мати власні sections.

Приклади:

```text
Library:
  System Catalog
  Saved Assets

Experiments:
  Runs
  Sweeps
  Branches
  Artifacts

Analysis:
  Overview
  Flows
  Population
  Balance
  Comparison
```

Internal navigation:

- повинна бути однотипною в межах одного workspace;
- не повинна конкурувати з global workspace navigation;
- може використовувати tabs, segmented controls, left sections або інший стабільний pattern;
- може відрізнятися для окремих сутностей, якщо стандартний pattern погіршує usability;
- не повинна створювати приховану глибоку ієрархію без breadcrumbs або back navigation.

## Responsive Behavior

Мінімальний підтримуваний viewport:

```text
1366x862
```

При обмеженому просторі UI деградує в такому порядку:

```text
1. global navigation переходить у compact mode;
2. right Inspector стає drawer або temporary overlay;
3. bottom panel згортається у tabs або compact strip;
4. left contextual panel стає drawer;
5. primary content зберігає максимальний можливий розмір.
```

`Viewer` або primary chart не повинні стискатися до непридатного стану лише заради постійної видимості всіх panels.

Monitor exception: at or above `1366x862` CSS px, Contextual Inspector remains
the fixed right track and Data Panel remains in the Grid. Below that threshold,
the full Monitor grid is not an acceptance target; root/page vertical scroll is
preferred over shrinking Monitor tracks to zero.

Panels, відкриті як overlay:

- не змінюють simulation state;
- легко закриваються;
- не приховують критичний system state без альтернативного indicator;
- підтримують keyboard focus management.

## Full-Screen Viewer

`Viewer` підтримує full-screen mode.

У full-screen:

- `Viewer` займає весь доступний простір;
- global shell може тимчасово приховуватися;
- мінімальні run controls доступні як overlay;
- active `Data Context` і run state залишаються доступними;
- basic layer controls залишаються доступними;
- selection зберігається;
- contextual Inspector відкривається як temporary overlay;
- `Escape` або явна action повертає normal layout.

Full-screen mode змінює лише presentation.

## Inspector And Full Detail

`Inspector` є швидким contextual view.

Для глибокого дослідження користувач може відкрити full detail view.

Модель:

```text
selection
  -> quick Inspector

open details
  -> full View
```

Full detail view:

- відкривається у межах current workspace або найбільш відповідного workspace;
- має власний navigation state;
- підтримує back navigation;
- може містити internal sections;
- може бути відкритий в окремому window;
- не створює окрему копію authoritative data.

## Selection, Open, Focus And Track

Це різні navigation actions.

### Selection

```text
select entity
  -> highlight entity
  -> update Inspector
```

### Open

```text
open entity
  -> show full detail View
```

### Focus

```text
focus entity in Viewer
  -> center or fit entity in Viewport
```

### Track

```text
track entity
  -> keep entity visible or centered while simulation advances
```

Selection не повинна автоматично запускати `track`.

Focus не повинен змінювати simulation priority.

## Navigation History

UI підтримує:

- Back;
- Forward;
- breadcrumbs для глибокого detail context;
- recent entity history.

Приклад:

```text
Evolution
  > Lineage A12
  > Genome G-438
  > Cell C-1902
```

Navigation history відображає UI context, а не simulation time travel.

`Back` не змінює current Tick, якщо попередній navigation state явно не містив іншого `Data Context`.

## Recent Entities

UI зберігає список останніх переглянутих сутностей.

Recent history може містити:

- Cell;
- OrganismView;
- Genome;
- lineage;
- Resource;
- Material;
- Field;
- Process;
- run;
- checkpoint;
- saved asset.

Для кожного item показуються базові attributes, достатні для ідентифікації.

Кількість recent items є bounded і може налаштовуватися.

Recent history:

- не є simulation artifact;
- може зберігатися як user preference;
- не повинна впливати на behavior;
- дозволяє повторно відкрити entity;
- може використовуватися для pinning або comparison.

## Entity Search

UI має глобальний або context-aware search для сутностей.

Пошук підтримує щонайменше:

- exact ID;
- partial ID;
- canonical name, якщо entity має name;
- entity type;
- basic lifecycle state;
- Genome id;
- lineage id;
- OrganismView id;
- run id;
- saved asset name;
- selected базові attributes, якщо вони індексуються.

Результати можуть показуватися як grid або list.

Search result має містити:

- entity type;
- ID;
- короткий current state;
- relevant `Data Context`;
- основні identifying attributes;
- доступні actions: select, open, focus, compare.

Search не змінює active filters мовчки.

## ID Visibility

Canonical ID сутності повинна бути чітко видимою в `Inspector` та full detail View.

ID:

- підсвічується в header Inspector;
- доступна для копіювання;
- не маскується friendly label;
- використовується в search;
- може використовуватися у deep link або navigation descriptor;
- показується в comparison.

Friendly name або derived label не замінює canonical ID.

## Comparison Navigation

Entity може бути:

- pinned;
- added to comparison;
- opened from recent history;
- opened from search;
- selected from `Viewer`.

Comparison не використовує довільні internal browser-like tabs.

Основні patterns:

```text
current entity
+
pinned entities
+
comparison view
```

Для паралельного незалежного перегляду може використовуватися окреме browser або application window.

## Cross-Workspace Transitions

Canonical переходи:

```text
Genome or lineage
  -> Evolution

run comparison
  -> Analysis

checkpoint or branch
  -> Experiments

saved Cell, Species or Organism
  -> Library

spatial entity
  -> Monitor
```

При переході зберігаються, якщо сумісні:

- run;
- Tick або interval;
- selected entity;
- `Analysis Level`;
- filters;
- primary Field або color mode.

UI не повинен мовчки перемикати run або `Data Context`.

## Global Indicators

Global navigation може показувати compact indicators для:

- active run;
- running experiments;
- critical warnings;
- failed run;
- pending validation;
- unavailable data.

Indicators використовуються лише для значущих system states.

Не використовуються декоративні notification counters або badges без operational meaning.

## Settings Navigation

`Settings` є application-level view, а не simulation workspace.

Основні categories:

```text
Appearance
Language
Accessibility
Rendering
Behavior
Storage / Export
```

Після закриття `Settings` користувач повертається до попереднього workspace та navigation state.

Зміна UI settings не змінює simulation state.

## Help Navigation

`Help` є application-level view.

Вона може містити:

- contextual help index;
- UI concepts;
- keyboard shortcuts;
- links to Canon documentation;
- glossary lookup;
- troubleshooting;
- version information.

Contextual help може відкривати конкретний Help section без втрати current workspace context.

## Navigation State

Navigation state має бути відновлюваним, якщо це підтримує обрана frontend architecture.

Мінімально відновлюються:

- workspace;
- `Data Context`;
- `Analysis Level`;
- selected entity;
- detail view;
- Tick або interval;
- compatible filters;
- active internal section.

Необов'язково відновлюються:

- hover state;
- temporary animation;
- open transient tooltip;
- exact pointer position;
- temporary drag state.

У web implementation navigation state може використовувати URL або route state.

В іншій implementation може використовувати navigation descriptor або saved view reference.

Це правило не фіксує конкретний routing framework.

## Deep Links And Separate Windows

UI може підтримувати shareable або restorable links на:

- workspace;
- run;
- entity;
- Tick;
- interval;
- comparison;
- analysis view.

Окреме browser або application window може відкривати інший workspace або `Data Context`.

Separate windows:

- не повинні неявно ділити mutable local UI state;
- повинні явно показувати свій `Data Context`;
- можуть читати той самий run;
- не створюють нову simulation authority.

## Архітектурні обмеження

Заборонено:

- змішувати global workspace navigation з `Analysis Level`;
- приховувати active workspace;
- мовчки змінювати `Data Context`;
- використовувати selection як simulation input;
- створювати довільну кількість internal tabs як основну navigation model;
- замінювати canonical ID лише friendly label;
- втрачати selection без видимого пояснення;
- стискати primary content до непридатного стану;
- створювати окрему navigation semantics для кожного workspace без потреби.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/visualization.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- structures: [[docs/ui/architecture|UI Architecture]]
- navigates: [[docs/ui/visualization|UI Visualization]]
- navigates: [[docs/ui/analytics|UI Analytics]]
- navigates: [[docs/ui/exploration|UI Exploration]]
- uses presentation rules from: [[docs/ui/presentation|UI Presentation]]
- delegates gestures to: [[docs/ui/interaction|UI Interaction]]
- constrained by: [[docs/ui/quality|UI Quality]]
