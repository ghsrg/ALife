---
tags:
  - alife
  - ui
  - canon
  - design
  - control-center
  - layout
  - visual-system
---

# ALife Control Center Design Specification

> Canonical contract for the shared application shell, layout, visual system and workspace composition of `ALife Control Center`.

## Призначення

Цей документ закриває проміжок між загальними UI-принципами та implementation plans.

Він визначає:

- canonical application shell;
- layout regions і visual hierarchy;
- composition rules для `Monitor` як першого основного workspace;
- поведінку panels;
- adaptive layout;
- базову visual system;
- presentation states;
- роль visual references.

Цей документ **не є повною специфікацією всього UI**.

Він не замінює:

- [[docs/ui/principles|UI Principles]];
- [[docs/ui/architecture|UI Architecture]];
- [[docs/ui/navigation|UI Navigation]];
- [[docs/ui/visualization|UI Visualization]];
- [[docs/ui/interaction|UI Interaction]];
- [[docs/ui/exploration|UI Exploration]];
- [[docs/ui/analytics|UI Analytics]];
- [[docs/ui/presentation|UI Presentation]];
- [[docs/ui/quality|UI Quality]];
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]].

Якщо правило вже визначене в тематичному UI Canon-документі, цей файл посилається на нього і не створює паралельної версії.

## Authority And Scope

Цей документ є canonical для:

```text
shared application composition
layout hierarchy
panel placement and behavior
visual-system direction
workspace presentation states
```

Він не є authority для:

```text
simulation semantics
Projection payloads or protocols
command validation
analytics definitions
World rendering semantics
accessibility requirements
implementation phases or task ordering
```

У разі конфлікту діє загальний authority order проєкту. Для UI-специфічного конфлікту пріоритет мають відповідні тематичні Canon-документи.

## Product Character

`ALife Control Center` має сприйматися як:

```text
scientific instrument
+
simulation control center
+
game-like visual interface
```

Візуальна подача повинна створювати відчуття живого World і складної системи, але не може:

- спотворювати simulation data;
- приховувати missing або unavailable data;
- створювати декоративні events, схожі на simulation events;
- підміняти committed values інтерпольованими presentation values;
- робити Debug або Research workflows менш точними.

## Shared Application Shell

Canonical application name:

```text
ALife Control Center
```

Shared shell складається з таких областей:

```text
Application Frame
├── Global Navigation
├── Run And Data Context Bar
├── Workspace Surface
│   ├── Primary Workspace Area
│   ├── Contextual Side Panel
│   └── Optional Bottom / Auxiliary Panel
├── Global Notifications And Confirmations
└── Status / Diagnostics Surface
```

Точна реалізація може об'єднувати сусідні області, якщо зберігаються їх функціональні ролі та visual hierarchy.

### Global Navigation

Global Navigation:

- перемикає доступні workspaces;
- не показує неіснуючі workspace лише як декоративні placeholders;
- зберігає зрозумілий active state;
- не конкурує візуально з primary workspace;
- залишається стабільною між workspaces.

Canonical workspace names і navigation semantics визначаються в [[docs/ui/architecture|UI Architecture]] та [[docs/ui/navigation|UI Navigation]].

### Run And Data Context Bar

Run And Data Context Bar повинен давати постійно доступний контекст:

- run state;
- current Tick;
- simulation rate;
- visualization FPS або projection health, якщо доступно;
- seed;
- active scenario/config;
- live, fixture, recorded або historical context;
- intervention state, якщо він існує;
- глобальні warnings про неповні або stale data.

Ця область не повинна перетворюватися на повну diagnostics dashboard.

### Workspace Surface

Workspace Surface має один primary focus.

Для `Monitor` primary focus — `Viewer`.

Інші panels повинні підтримувати primary focus, а не створювати кілька рівнозначних центрів уваги.

## Monitor Workspace Composition

`Monitor` є world-first workspace для live або recorded observation.

Canonical composition:

```text
Monitor
├── Viewer — primary surface
├── Run Controls — directly accessible
├── Layer / View Controls — contextual to Viewer
├── Selection Context
├── Cell / Entity Inspector
└── Optional lightweight status or event surface
```

### Viewer

`Viewer`:

- займає найбільшу частину доступної площі;
- залишається видимим під час основних observation workflows;
- не перекривається постійними великими panels без явної дії користувача;
- підтримує full-screen або focus mode;
- використовує один primary visual context та прості secondary overlays;
- показує лише data-bound presentation або чесно позначений unavailable state.

World rendering semantics визначаються в [[docs/ui/visualization|UI Visualization]].

### Run Controls

Основні controls `Play`, `Pause`, `Step`, `Stop` та пов'язані run actions повинні:

- бути доступними без переходу до іншого workspace;
- мати чіткий enabled/disabled state;
- показувати pending або rejected command state;
- не маскувати асинхронність між UI request і Core confirmation;
- не змішуватися з Viewer-only controls на кшталт zoom або layer visibility.

Command semantics визначаються в [[docs/ui/interaction|UI Interaction]].

### Inspector

Inspector є contextual panel, пов'язаною з selection.

Він повинен:

- відкриватися після selection або explicit action;
- залишати Viewer достатньо простору;
- підтримувати collapse або close;
- не показувати вигадані значення для unavailable fields;
- відрізняти raw, derived, approximate та presentation-only values;
- зберігати selection context під час допустимих layout transitions.

Inspector content rules визначаються в [[docs/ui/exploration|UI Exploration]].

### Layer And View Controls

Layer controls повинні бути близькими до Viewer, але не перекривати його постійно.

Рекомендовані форми:

- компактна toolbar;
- collapsible popover;
- contextual side section;
- keyboard-accessible command surface.

Вони не повинні виглядати як simulation controls.

## Panel Behavior

Panels можуть підтримувати:

```text
dock
collapse
resize
close
contextual open
overlay
focus / full-screen
```

Але кожна panel повинна мати одну canonical default role.

### Default Rules

- Primary Viewer не є dismissible panel у `Monitor`.
- Inspector за замовчуванням docked праворуч на широких екранах.
- Auxiliary timelines, logs або diagnostics можуть використовувати bottom panel.
- Overlay допускається для короткочасного interaction або на вузькому viewport.
- Persistent modal layout заборонений.
- Panels не повинні довільно змінювати порядок між sessions без explicit user customization.

### Resizing

Resizable panels повинні мати:

- minimum useful size;
- maximum size, яка не знищує primary focus;
- visible resize affordance;
- predictable keyboard-accessible alternative, якщо resize критичний;
- збереження local UI preference окремо від simulation artifacts.

## Adaptive Layout

Canonical minimum supported application viewport для повноцінного desktop UI:

```text
1024 × 768
```

Layout повинен адаптуватися за пріоритетами, а не лише масштабувати всі елементи.

### Wide Layout

На широкому viewport:

```text
navigation | primary workspace | docked inspector
                     + optional bottom panel
```

Viewer отримує основну площу.

### Compact Desktop Layout

На компактному desktop viewport:

- navigation може переходити в компактний режим;
- Inspector може звужуватися або ставати overlay/drawer;
- secondary labels можуть скорочуватися з доступним full hint;
- auxiliary panels за замовчуванням collapse;
- run state і критичні warnings залишаються видимими.

### Below Minimum Viewport

Нижче canonical minimum допускається degraded layout, але UI повинен:

- залишатися керованим;
- не приховувати критичні controls;
- явно пріоритезувати Viewer або active task;
- уникати горизонтального overflow для основних application controls;
- не обіцяти повний Research workflow.

## Visual Hierarchy

Visual hierarchy повинна мати такий порядок:

```text
1. Primary workspace focus
2. Current run / data state
3. Active selection or current task
4. Contextual controls
5. Secondary status and diagnostics
6. Decorative presentation
```

Не допускається:

- кілька однаково яскравих primary regions;
- постійне використання warning colors як decoration;
- великі декоративні headers, що зменшують Viewer;
- надлишок glowing borders, gradients або LED-like lines;
- анімація в кількох panels одночасно без інформаційної потреби.

## Visual System

### Design Tokens

Implementation повинна використовувати semantic design tokens, а не розрізнені hardcoded values.

Мінімальні групи tokens:

```text
color.background.*
color.surface.*
color.text.*
color.border.*
color.state.*
color.data.*
space.*
radius.*
elevation.*
typography.*
motion.*
```

Точні значення можуть розвиватися під час реалізації, але semantic meaning має залишатися стабільним.

### Typography

Typography повинна:

- підтримувати високу інформаційну щільність без втрати hierarchy;
- мати обмежену кількість text levels;
- використовувати monospaced style лише для identifiers, hashes, Tick, raw values або diagnostic data;
- не зменшувати основний UI text до нечитабельного розміру заради Viewer;
- підтримувати text scaling.

### Spacing And Density

Default density — compact scientific application, а не consumer landing page.

Водночас:

- controls повинні мати достатню hit area;
- related values групуються ближче, ніж unrelated sections;
- whitespace використовується для hierarchy, а не як декоративна порожнеча;
- panels повинні мати consistent internal rhythm.

### Surfaces And Elevation

Elevation використовується функціонально:

- base application surface;
- docked panel;
- floating contextual surface;
- modal/confirmation surface.

Не слід створювати окремий elevation level для кожного card або metric.

## Theme Semantics

Dark і Light themes повинні мати однакову функціональну hierarchy.

Semantic colors не змінюють значення між themes.

Особливо стабільними мають бути:

- selected state;
- warning;
- error;
- success/confirmed;
- paused/running/stopped;
- missing/unavailable data;
- stale or disconnected state;
- primary data encodings.

Повні presentation і accessibility rules визначаються в [[docs/ui/presentation|UI Presentation]].

## Presentation States

Application shell і `Monitor` повинні явно розрізняти такі стани:

### Idle

- немає active run;
- доступний запуск scenario або відкриття recorded artifact;
- Viewer не імітує live World.

### Loading / Connecting

- показано, що bootstrap, connection або initial projection ще не завершені;
- попередній frame не видається за current live state;
- controls мають коректний pending state.

### Live Running

- live context видимий;
- current Tick і run state оновлюються;
- stale projection або degraded projection rate позначаються.

### Paused

- paused state візуально очевидний;
- Viewer може залишатися інтерактивним для observation;
- simulation-changing commands залишаються під Core authority.

### Recorded / Fixture

- recorded або fixture data не можуть виглядати як live connection;
- source, time context і provenance повинні бути доступні;
- unsupported live controls disabled або відсутні.

### Disconnected / Stale

- останній frame може залишатися видимим лише з явною stale/disconnected ознакою;
- reconnect state і доступні дії зрозумілі;
- UI не продовжує локально імітувати simulation progress.

### Unavailable Data

- layer, metric або field не показуються як zero;
- UI використовує explicit unavailable state;
- пояснюється, чи причина в projection contract, loading, permissions або unsupported feature.

### Error

- error не знищує доступний validated context;
- повідомлення містить короткий user-facing summary;
- technical details доступні окремо;
- recoverable і terminal errors візуально відрізняються.

## Motion And Decorative Effects

Motion може використовуватися для:

- transitions між states;
- підтвердження actions;
- focus guidance;
- showing continuity;
- живої presentation World.

Motion не може:

- імітувати entity movement, process або event без відповідних data;
- приховувати latency;
- затримувати critical controls;
- бути обов'язковим для розуміння state;
- ігнорувати reduced-motion preference.

Декоративні effects повинні бути optional і відокремлені від data encodings.

## Visual Reference Policy

`docs/ui/control-center-monitor-v3.png` є **informative visual direction reference**.

Він:

- показує бажаний характер, hierarchy та загальний visual direction;
- може використовуватися як reference під час UI-1A/UI-1 Start development;
- не є pixel-perfect contract;
- не визначає функціональність, якої немає в Canon;
- не визначає точні dimensions, colors, fonts або component APIs;
- може містити exploratory elements, які ще не прийняті як requirement.

У разі конфлікту:

```text
textual UI Canon
> accepted ADR
> implementation plan
> visual reference image
```

Зміна PNG сама по собі не змінює Canon.

Якщо visual decision має стати обов'язковим, воно повинно бути перенесене до відповідного текстового Canon-документа.

## Phase Boundary

Цей документ задає shared design foundation для всього Control Center, але не переносить майбутню функціональність у ранні phases.

Для `UI-1 Start` обов'язковими є лише ті composition і visual-system rules, які потрібні реалізованим workspaces та controls.

Не потрібно показувати:

- майбутні Research workspaces як active placeholders;
- неіснуючі analytics;
- unavailable layers як fake demo data;
- design-only controls без working contract.

Phase scope і sequencing визначає [[docs/implementation/implementation-plan-ui|UI Implementation Plan]].

## Change Rules

Зміна цього документа потрібна, якщо змінюється:

- shared application shell;
- canonical layout regions;
- primary focus `Monitor`;
- default panel roles;
- adaptive-layout strategy;
- visual token semantics;
- visual reference authority.

Зміна не потрібна для:

- локального styling fix без semantic impact;
- нового chart type;
- нового projection field;
- implementation refactor;
- окремого experiment workflow;
- зміни, яка вже повністю належить іншому Canon-документу.

## Related Documents

- [[docs/ui/README|UI Layer]]
- [[docs/ui/principles|UI Principles]]
- [[docs/ui/architecture|UI Architecture]]
- [[docs/ui/navigation|UI Navigation]]
- [[docs/ui/visualization|UI Visualization]]
- [[docs/ui/interaction|UI Interaction]]
- [[docs/ui/exploration|UI Exploration]]
- [[docs/ui/presentation|UI Presentation]]
- [[docs/ui/quality|UI Quality]]
- [[docs/implementation/implementation-plan-ui|UI Implementation Plan]]
