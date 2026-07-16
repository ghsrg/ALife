---
tags:
  - alife
  - ui
  - canon
---

# UI Presentation

## Призначення

Цей документ визначає правила presentation layer у `ALife Control Center`.

Документ описує:

- themes;
- visual language;
- localization;
- UI text registry;
- contextual help;
- message semantics;
- accessibility;
- number, unit і date formatting;
- typography;
- UI density;
- UI scale;
- text overflow;
- presentation consistency.

Presentation layer не змінює simulation state, analytics values або behavior.

## Visual Language

Canonical visual direction:

```text
scientific instrument
+
game-like interface
```

UI повинен бути:

- візуально привабливим;
- цілісним;
- інформаційно насиченим;
- читабельним;
- точним;
- придатним для тривалого спостереження;
- достатньо виразним для демонстрації складної simulation behavior.

Game-like presentation не може спотворювати data або підміняти scientific meaning.

## UI-1C Design Alignment

Ця секція фіксує phase-specific design decisions для `UI-1C`.

Вона не замінює [[docs/ui/control-center-design-spec|Control Center Design Specification]].
`control-center-design-spec.md` залишається foundation для shared shell, layout,
panel behavior і visual hierarchy. Ця секція визначає лише те, що потрібно
зафіксувати перед `UI-1C`, щоб не переносити зайвий scope із visual reference у
implementation plan.

Ця секція також не дублює:

- simulation truth та observer boundary;
- projection protocol;
- command semantics;
- детальні visualization, analytics або accessibility rules.

Для цих тем діють відповідні canonical docs.

### UI-1C Visual Goal

`UI-1C` має створити `World-first WOW` для `Monitor`.

Обов'язковий фокус:

- dominant World View;
- atmospheric, data-bound 2D field map;
- visible Cells, selection, zoom/pan і focus feedback;
- minimal selected-entity showcase;
- compact control-room context тільки для запуску, зупинки та базового стану
  світу.

Control-room density із visual reference використовується тільки настільки, щоб:

- керувати run;
- бачити scenario/data context;
- бачити Tick/run state;
- бачити мінімальні world stats, наприклад alive/dead counts, total Energy або
  інші доступні aggregate values.

### Visual Reference Treatment

`docs/ui/control-center-monitor-v3.png` є structural target для `UI-1C`, але не
pixel-perfect target.

`UI-1C` повинен зберегти основну композицію:

```text
top context / control strip
left layer controls
dominant central World View
right contextual Inspector
compact bottom stats strip
```

Якщо просте атмосферне рішення наближає UI до reference без великої ціни, його
можна брати. Якщо схожість вимагає fake analytics, complex subsystem або
непідтверджені дані, рішення відкладається або виноситься на окреме обговорення.

### Visual Direction

Shell direction:

```text
dark cyan control center
restrained neon
professional scientific dashboard
```

World View direction:

```text
rich bioluminescent world
atmospheric fields
clear selected entity focus
```

Dark theme є primary для `UI-1C` WOW acceptance.

Light theme повинна залишатися usable:

- без layout breakage;
- без unreadable controls;
- без прихованих critical states;
- без requirement на full WOW parity у `UI-1C`.

### Selected Entity Showcase

`UI-1C` використовує small floating focus card над або біля World View selection.

Focus card показує лише доступні data-bound values:

- selected entity identity;
- position/radius або інший доступний spatial summary;
- Energy/lifecycle/integrity, якщо доступно;
- compact bars або schematic encoding для доступних values;
- explicit unavailable/missing state для відсутніх fields.

Right Inspector залишається detail surface для selected entity. Large cinematic
internal organism/cell panel відкладається до фази, де Observer projection має
достатньо composition/process data.

### Bottom Stats Strip

`UI-1C` має compact bottom strip, а не повну analytics dashboard.

Допускаються 3-5 базових world stats, якщо вони доступні з поточного projection
або summary:

- alive/dead counts;
- total Energy;
- current Tick/run state;
- population count;
- projection health або missing-data state.

Charts, cycles і distributions не входять у `UI-1C`, якщо вони не backed by
accepted Observer/Runner data.

### Missing Data Stop Rule

У live `UI-1C` не можна імітувати missing data.

Якщо потрібного projection field немає:

```text
show panel/card/field as Unavailable or Missing projection
-> record dependency
-> create Runner or Observer plan before implementing that visualization
```

Placeholder panel дозволений тільки як explicitly unavailable state. Він не
повинен виглядати як working chart, real metric або observed process.

### Screenshot Acceptance

`UI-1C` design acceptance має включати screenshot set:

```text
1920x1080 dark
  primary WOW target

1366x768 dark
  usability minimum, no incoherent overlap, controls usable, World View dominant

1920x1080 light
  basic usable state, not full WOW parity
```

Screenshot acceptance не замінює tests, але блокує завершення `UI-1C`, якщо
composition або hierarchy очевидно суперечать цій секції.

### Explicitly Deferred From Reference

Не входить у `UI-1C`, навіть якщо присутнє або натякається у visual reference:

- full analytics charts;
- resource cycle diagram;
- behavior або cell-class distribution charts;
- cinematic organism/cell internals;
- full warning center;
- library, evolution або research tabs as working surfaces;
- advanced accessibility/polish pass;
- full design-system token pass beyond what `UI-1C` needs.

## Themes

Обов'язкові themes:

```text
Light
Dark
```

Theme preference може підтримувати:

```text
System
```

але `System` не є обов'язковим для першої реалізації.

Якщо `System` не підтримується, UI використовує:

1. останню вибрану користувачем theme;
2. default theme application;
3. безпечний fallback.

Light і Dark themes повинні мати однакову функціональність.

Theme не повинна змінювати:

- metric meaning;
- warning severity;
- classification meaning;
- data state;
- chart interpretation.

## One Visual Style

UI використовує одну canonical visual language.

Не вимагаються окремі skins на кшталт:

```text
Scientific
Control Center
Minimal
```

Архітектура може бути розширюваною для майбутніх visual styles, але базова implementation не повинна підтримувати кілька skins.

Розширення visual style не може змінювати semantic meaning.

## Semantic Design Tokens

Presentation повинна використовувати stable semantic tokens.

Приклади:

```text
surface
surface-elevated
panel
border
text-primary
text-secondary
accent
selection
warning
error
critical
success
discovery
missing-data
disabled
```

Colors не повинні hardcode-итися напряму в UI components.

Theme визначає конкретне значення semantic token.

## UI Density

Підтримуються режими:

```text
Comfortable
Compact
```

### Comfortable

Використовує більші spacing, padding і touch targets.

### Compact

Зменшує:

- spacing;
- padding;
- panel gaps;
- table row height;
- toolbar density.

Compact mode не повинен:

- зменшувати text до непридатного розміру;
- приховувати critical information;
- порушувати minimum interaction targets;
- погіршувати accessibility.

## UI Scale

UI має окремий scale control.

Scale реалізується як simple slider.

UI scale може впливати на:

- text;
- controls;
- icons;
- panels;
- spacing;
- hit areas.

UI scale не впливає на:

- simulation coordinates;
- Viewer zoom;
- Cell physical size;
- analytics values;
- behavior.

UI scale і Viewer zoom є різними settings.

## Localization

UI є localization-ready з першої версії.

Початкові locale:

```text
uk-UA
en-US
```

Locale selection priority:

```text
1. last selected locale
2. system language, якщо підтримується
3. en-US fallback
```

Якщо користувач раніше обрав мову, вона має пріоритет над system language.

## Canonical Terms In UI

За замовчуванням UI показує canonical English term.

Приклади:

```text
Resource
Material
Energy
Cell
Genome
```

Localized explanation або переклад доступні в detail view, contextual help або expanded text.

Приклад:

```text
Label:
  Resource

Expanded explanation:
  Ресурс — рухома речовина світу...
```

UI не повинен створювати різні technical concepts через вільний переклад одного canonical term.

## UI Text Registry

Усі user-facing strings надходять із `ui-text`.

UI components не повинні містити hardcoded user-facing text.

UI text registry може бути організований через logical namespaces.

Приклад:

```text
ui-text/
  uk-UA/
    common
    navigation
    monitor
    editor
    experiments
    evolution
    library
    analysis
    help

  en-US/
    common
    navigation
    monitor
    editor
    experiments
    evolution
    library
    analysis
    help
```

Конкретний file format не фіксується Canon.

Можливі:

- JSON;
- YAML;
- TOML;
- generated resource bundle;
- typed localization module.

Обов'язкові властивості:

- stable keys;
- logical namespaces;
- locale fallback;
- pluralization support;
- formatting parameters;
- completeness validation;
- missing-key detection.

## Relationship With GLOSSARY.md

`GLOSSARY.md` використовується людьми та AI-агентами під час створення й перевірки UI texts.

UI runtime не знає про `GLOSSARY.md`.

Runtime dependency:

```text
UI component
  -> ui-text key
  -> localized text
```

Не використовується:

```text
UI component
  -> parse GLOSSARY.md at runtime
```

`GLOSSARY.md` допомагає забезпечити consistent terminology, але `ui-text` є єдиним runtime source для labels, hints і messages.

## Contextual Help

Canonical help mechanism:

```text
Contextual Help
```

Він включає:

```text
Help Indicator
Expanded Hint
Documentation Link
```

## Help Indicator

Для змістовного або неочевидного component при hover або keyboard focus з'являється маленька напівпрозора `i`.

`i`:

- не відображається постійно;
- з'являється лише при hover/focus на component;
- розташовується в передбачуваному кутку component;
- не запускає hint автоматично лише через рух мишки;
- має окремий hover/focus target;
- не перекриває data;
- має accessible name.

Це зменшує visual noise і водночас показує, що для element доступне пояснення.

## Elements Requiring Contextual Help

Contextual Help обов'язкова для:

- chart;
- metric card;
- classification;
- derived value;
- composite score;
- filter group;
- complex control;
- warning;
- discovery;
- Field layer;
- Resource layer;
- workspace section;
- uncommon action;
- data state;
- interpolation mode;
- aggregation mode.

Окрема `i` не потрібна для очевидних actions:

- Close;
- Back;
- Search;
- Pause;
- простого checkbox із повною зрозумілою назвою.

## Expanded Hint

Після click, focus action або explicit activation `i` відкриває один Expanded Hint.

Окремі рівні Short Hint і Expanded Explanation не є обов'язковими.

Expanded Hint містить лише релевантні fields.

Можливі fields:

- що це;
- що показує;
- як працює;
- як рахується;
- як читати;
- unit;
- Time Context;
- aggregation;
- confidence;
- limitations;
- data state;
- чи впливає на simulation;
- Documentation Link.

Для простого element hint може містити лише 1–2 короткі речення.

Для derived metric, classification або composite score hint містить повнішу методику.

## Help Interaction

Desktop behavior:

```text
hover/focus component
  -> show subtle i

activate i
  -> open Expanded Hint
```

Hint не відкривається автоматично лише через переміщення pointer над component.

Popover може бути pinned.

Закриття:

- `Escape`;
- click outside;
- explicit Close;
- повторна activation.

Touch support реалізується лише там, де це критично або реально використовується.

Touch implementation не повинна суттєво ускладнювати базову desktop-first architecture.

## Documentation Link

Expanded Hint може містити link на:

- Canon documentation;
- methodology;
- glossary entry;
- metric definition;
- classifier description;
- troubleshooting.

Documentation Link не є обов'язковим для простих controls.

## Full-Screen Help

У full-screen Viewer Contextual Help залишається доступною.

Використовується:

- compact Help toggle;
- contextual `i` для active controls;
- help для legend;
- help для selected entity;
- help для active Field або color mode.

Help не повинна постійно займати значну частину Viewport.

## Accessibility

UI повинен підтримувати:

- keyboard navigation;
- visible focus;
- sufficient contrast;
- text scaling;
- non-color encoding;
- reduced motion;
- screen-reader semantics;
- chart summaries;
- accessible names;
- predictable focus order.

Color не може бути єдиним носієм meaning.

Controls, inspectors, charts, messages та actions повинні мати semantic accessibility metadata.

Viewer може мати aggregated textual description.

## Focus

Visible focus:

- не приховується;
- має достатній contrast;
- не залежить лише від color;
- не плутається з selection;
- зберігає predictable order.

Help Indicator повинен бути keyboard-accessible.

## Reduced Motion

UI повинен підтримувати reduced motion.

Reduced motion може:

- вимикати decorative animation;
- зменшувати transition duration;
- вимикати parallax;
- вимикати non-essential interpolation effects;
- зберігати functional feedback.

Reduced motion не повинно приховувати state changes.

## Number Formatting

Звичайний UI використовує locale-aware formatting.

Приклади:

```text
uk-UA:
  1 234,56

en-US:
  1,234.56
```

Canonical machine format використовується для:

- IDs;
- raw config values;
- raw data export;
- machine-readable JSON;
- deterministic artifacts.

Scientific notation допускається для дуже великих або малих values.

Rounding не повинно створювати false precision.

## Units

Value і unit показуються разом.

Приклади:

```text
12.4 eu
48 ticks
7.2 su
```

Unit може бути винесена в axis, table header або section header, якщо context однозначний.

UI не перетворює simulation units у:

- seconds;
- meters;
- joules;
- kilograms;

без explicit mapping.

Unit formatting надходить із UI text та unit registry.

## Date And Time Formatting

Calendar date і real-time timestamp використовують locale-aware formatting.

Simulation time, Tick і interval не повинні форматуватися як реальний календарний час без explicit mapping.

UI повинен чітко розрізняти:

- real timestamp;
- simulation time;
- Tick;
- interval.

## Tone Of Voice

Canonical tone:

```text
neutral
clear
scientific
concise
```

Більш жива мова допускається для:

- discoveries;
- onboarding;
- empty states;
- successful actions;
- non-critical confirmations.

Warnings, errors, Critical messages і Analytical Summary мають залишатися точними та нейтральними.

UI не повинен антропоморфізувати simulation state без явної метафори.

## Message Types

Canonical message types:

```text
Info
Success
Warning
Error
Critical
Discovery
```

`Discovery` не є severity level.

### Info

Нейтральний context або explanation.

### Success

Підтвердження завершеної user action.

### Warning

Потенційна проблема або ризик.

### Error

Action або calculation не виконано.

### Critical

Стан, який потребує негайної уваги.

### Discovery

Значуща спостережена подія або pattern без негативної severity.

## Critical Messages

Critical message не може бути доступна лише через hover або Contextual Help.

Вона повинна мати:

- visible icon;
- severity;
- short title;
- explanation;
- affected Data Context;
- recommended action;
- dismissal або acknowledgement rule;
- link to evidence або details.

Critical state повинен залишатися видимим, доки він актуальний або acknowledged згідно з interaction rules.

## Empty States

Empty state повинен пояснювати:

- що відсутнє;
- чому це могло статися;
- що можна зробити далі;
- чи потрібні data, configuration або calculation.

Відсутність data не показується як zero.

Empty state може мати живішу мову, але не приховує technical reason.

## Error Messages

Error message повинно містити:

- що не виконано;
- коротку причину;
- Data Context;
- recovery action;
- technical details on demand;
- stable error id, якщо доступний.

UI не повинен показувати stack trace як primary user message.

## Typography

Canon не фіксує конкретний font family.

Font повинен підтримувати:

- Unicode;
- Cyrillic;
- Latin;
- tabular numbers;
- clear distinction між:
  - `0` і `O`;
  - `1`, `l` і `I`;
  - similar punctuation.

Monospace використовується для:

- IDs;
- config values;
- raw data;
- code;
- hashes;
- machine-readable values.

## Text Hierarchy

UI повинна мати стабільну hierarchy:

```text
workspace title
section title
component title
label
value
secondary explanation
metadata
```

Typography не повинна створювати кілька однаково сильних visual focal points.

## Text Overflow

Canonical ID не повинна обрізатися без можливості переглянути її повністю.

Friendly name може використовувати ellipsis.

Повний text доступний через:

- Expanded Hint;
- Inspector;
- copy action;
- full View.

Tables можуть використовувати:

- column resize;
- horizontal scroll;
- wrapping;
- ellipsis;
- full-value tooltip.

Critical values не повинні ставати невидимими через overflow.

## Icons

Icon не повинен бути єдиним носієм meaning без accessible label або adjacent text.

Canonical actions мають stable icons.

Один icon не повинен використовуватися для різних unrelated meanings.

## Presentation State

Presentation preferences можуть включати:

- theme;
- locale;
- density;
- UI scale;
- reduced motion;
- contrast mode;
- decorative animation;
- number formatting preference.

Presentation state:

- належить до user preferences;
- не входить у simulation config;
- не входить у behavior hash;
- не змінює analytics values;
- може зберігатися між sessions.

## Архітектурні обмеження

Заборонено:

- hardcode-ити user-facing text у components;
- парсити `GLOSSARY.md` у runtime UI;
- показувати `i` постійно на всіх components;
- відкривати hint автоматично при кожному pointer movement;
- використовувати лише color для meaning;
- змінювати semantic meaning між themes;
- приховувати Critical state в tooltip;
- перекладати canonical term так, що виникає інший concept;
- змішувати simulation time і real time;
- обрізати canonical ID без доступу до full value;
- використовувати presentation preference як simulation input.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/analytics.md`
- `docs/ui/exploration.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- presents: [[docs/ui/architecture|UI Architecture]]
- presents: [[docs/ui/navigation|UI Navigation]]
- styles: [[docs/ui/visualization|UI Visualization]]
- styles: [[docs/ui/analytics|UI Analytics]]
- styles: [[docs/ui/exploration|UI Exploration]]
- provides text for: [[docs/ui/interaction|UI Interaction]]
- validated by: [[docs/ui/quality|UI Quality]]
