---
tags:
  - alife
  - ui
  - canon
---

# UI Principles

## Призначення

Цей документ визначає незмінні принципи `UI Layer`.

Конкретні layouts, назви робочих просторів, графіки, thresholds та implementation details описуються в інших UI Canon-документах.

Будь-яке UI-рішення повинно оцінюватися за цими принципами.

## Пріоритети

Пріоритети `UI Layer`:

```text
1. Коректно показувати simulation state.
2. Допомагати розуміти причинно-наслідкові зв'язки.
3. Давати змогу проводити та відтворювати експерименти.
4. Забезпечувати зрозумілу навігацію.
5. Бути візуально привабливим і цікавим.
```

Візуальна якість є критичною для успіху UI.

Водночас візуальна привабливість не може:

- спотворювати simulation data;
- приховувати невизначеність;
- створювати фальшиві events;
- підміняти фактичний state декоративним ефектом;
- робити аналітичні дані менш читабельними.

## Scientific Instrument With Game-Like Interface

UI поєднує:

```text
scientific instrument
+
simulation control center
+
game-like visual presentation
```

Game-like presentation може використовувати:

- виразні анімації;
- переходи;
- візуальні акценти;
- живу подачу World;
- атмосферні effects;
- reward-like feedback для підтвердження дій.

Декоративні анімації дозволені, якщо вони:

- не імітують simulation event;
- не спотворюють значення;
- не приховують data;
- можуть бути вимкнені;
- враховують `reduced motion`.

Нейтральні transition effects не є simulation data.

## World-First Visualization

`World` є головним просторовим контекстом UI.

Якщо дані мають просторову прив'язку, analytics, filters, charts та inspectors повинні залишатися пов'язаними з `Viewer`.

Приклади:

```text
chart selection
  -> highlight matching Cells or OrganismView in Viewer

Viewer selection
  -> highlight matching category in charts and analytics
```

Просторовий зв'язок не є обов'язковим лише для даних, які не мають коректної просторової інтерпретації.

## Observer Boundary

`UI Layer` не є simulation authority.

```text
UI не визначає behavior.
UI не змінює entity priority.
UI не впливає на Tick execution.
UI filters не змінюють simulation population.
Derived Classification не є Process input.
Screen visibility не впливає на simulation.
```

Будь-яка дія, що реально змінює `World`, проходить через:

```text
UI request
  -> approved command
  -> core validation
  -> commit
  -> intervention log
  -> updated Projection
```

Direct mutation of `WorldState` через UI заборонена.

## Reproducibility

Кожна дія, що може вплинути на simulation result, повинна бути:

- явною;
- валідованою;
- записаною;
- відтворюваною.

Суто UI-речі не входять у simulation hash:

- theme;
- zoom;
- pan;
- filters;
- open panels;
- selected chart;
- selected entity;
- local layout preferences.

UI settings і simulation artifacts повинні зберігатися окремо.

## Scientific Transparency

UI має показувати не лише результат, а й достатній контекст для його правильного розуміння.

Для кожного неочевидного елемента має бути доступний короткий hint.

Для derived metrics, classifications та composite scores додатково повинні бути доступні:

- definition;
- calculation method;
- unit;
- aggregation;
- time interval;
- data completeness;
- confidence;
- classifier version;
- main contributing factors;
- known limitations.

Для `Derived Classification` UI повинен показувати щонайменше:

```text
label
confidence
time interval
classifier version
main contributing factors
```

## No False Precision

UI не повинен показувати більшу точність, ніж має simulation або analytics result.

Приклади:

```text
0.823741
```

може відображатися як:

```text
0.82
```

якщо додаткові знаки не мають практичного значення.

Приблизна або probabilistic classification не повинна виглядати як абсолютний факт.

Невизначеність має бути видимою.

## Progressive Disclosure

На першому рівні UI показує головне.

Деталі відкриваються через:

- zoom;
- selection;
- expand;
- inspector;
- drill-down;
- contextual help.

UI не повинен одночасно показувати всі Resources, Materials, Processes, classifications та metrics без потреби.

Користувач повинен мати змогу поступово перейти:

```text
overview
  -> context
  -> detail
  -> explanation
```

## Semantic Zoom

Зміна масштабу повинна відкривати новий зміст, а не лише збільшувати ті самі символи.

При наближенні можуть ставати доступними:

- Boundary;
- Materials;
- Resources;
- Energy;
- active Processes;
- contacts;
- movement;
- damage;
- repair.

Технічні правила Semantic Zoom та LOD описуються в `visualization.md`.

## Structured Information Density

UI може бути інформаційно насиченим.

Інформаційна щільність допускається лише якщо вона:

- структурована;
- контекстна;
- має чітку hierarchy;
- може бути згорнута або прихована;
- не створює кілька рівнозначних центрів уваги.

Кожний workspace повинен мати один primary focus.

Приклади:

```text
Main Monitor:
  Viewer

Analytics:
  primary chart or analysis view

World Editor:
  World preview
```

## One Primary Analytical Context

У `Viewer` одночасно використовується:

```text
one primary Field or color mode
+
several simple overlays
```

Не допускається одночасне використання кількох рівнозначних primary heatmaps, color modes або classifications, якщо це робить World нерозбірливим.

Secondary overlays повинні бути простими, керованими та чітко відрізнятися від primary context.

## Consistent Terminology

Один concept повинен мати однакову canonical назву в:

- UI;
- documentation;
- reports;
- exports;
- contextual help.

Короткі labels допускаються, якщо:

- вони не створюють іншого значення;
- повний canonical term доступний у hint;
- зв'язок із `GLOSSARY.md` залишається однозначним.

## Contextual Help By Design

Кожен неочевидний control, metric, chart, classification, warning або analytical view повинен мати доступне пояснення.

Help має бути доступним через:

- hover;
- keyboard focus;
- tap;
- explicit help action.

Help не повинна бути доступна лише через hover.

## Theme Parity

`UI Layer` повинен підтримувати:

- dark theme;
- light theme.

Обидві теми реалізуються відразу для повноцінного UI.

Themes повинні мати однакову функціональність і однаково коректно передавати data.

Не допускається, щоб:

- heatmap була читабельна лише в одній theme;
- warning губився в одній theme;
- selected entity була нерозрізнюваною;
- semantic colors змінювали значення між themes.

## Accessibility

UI повинен підтримувати:

- keyboard navigation;
- visible focus;
- text scaling;
- sufficient contrast;
- color-blind-safe encoding;
- reduced motion;
- touch-accessible help;
- textual chart summaries;
- non-color indicators.

Color не може бути єдиним носієм значення.

`Viewer` може мати агрегований textual description.

Controls, inspectors, charts та actions повинні мати повну accessibility semantics.

## Localization-Ready From The Beginning

User-facing text не повинен бути hardcoded у UI components.

Через text keys повинні проходити:

- labels;
- hints;
- errors;
- warnings;
- empty states;
- validation messages;
- action confirmations;
- chart titles;
- legends.

Навіть якщо початкова реалізація має одну мову, architecture повинна підтримувати localization з першої версії.

## Visible System State

UI завжди повинен чітко показувати:

- run state;
- current Tick;
- paused, running або stopped;
- current seed;
- active config;
- selected time context;
- live data або recorded playback;
- control run або intervened run;
- checkpoint або branch context.

Користувач не повинен плутати:

```text
current World
recorded frame
checkpoint
branch
historical analytics interval
```

## Explicit Data State

Якщо data:

- ще не розраховані;
- частково завантажені;
- aggregated;
- sampled;
- cached;
- застарілі;
- unavailable;

UI повинен це показувати явно.

UI не може показувати стару, sampled або partial metric як актуальну й повну.

Можливі data states визначаються в `presentation.md` та `quality.md`.

## Graceful Degradation

При великих populations UI може:

- спрощувати rendering;
- агрегувати entities;
- приховувати labels;
- зменшувати update frequency;
- downsample charts;
- lazy-load details;
- використовувати viewport filtering.

UI не може:

- мовчки змінювати значення;
- показувати sampled data як complete;
- приховувати пропущені events без позначення;
- змінювати simulation behavior через performance mode.

## Purposeful Motion

Animation використовується для:

- показу зміни;
- підтвердження дії;
- переходу між Analysis Levels;
- відстеження movement;
- пояснення flow;
- підтримки spatial continuity.

Не використовуються:

- постійні беззмістовні пульсації;
- надмірне мерехтіння;
- motion, що маскує числові зміни;
- animation, яку можна сприйняти як simulation event.

UI повинен підтримувати `reduced motion` і вимкнення decorative animation.

## Interaction Safety

Read-only actions не потребують confirmation.

Для дій, що:

- змінюють simulation;
- створюють intervention;
- зупиняють або перезапускають run;
- запускають дорогий experiment;
- видаляють artifact;

UI повинен:

1. показати наслідок;
2. виконати validation;
3. запросити confirmation, якщо дія ризикована;
4. зафіксувати результат у відповідній history.

## Safe And Informative Defaults

Початковий UI state повинен бути:

- зрозумілим;
- безпечним;
- інформативним;
- не перевантаженим;
- без прихованої значної частини World;
- без активних destructive actions.

Конкретні default values визначаються у відповідних Canon-документах.

## User Preference Separation

User preferences можуть зберігатися між сесіями.

Приклади:

- theme;
- language;
- panel sizes;
- last workspace;
- chart settings;
- favorite filters.

User preferences повинні бути чітко відокремлені від:

- simulation config;
- run artifacts;
- checkpoints;
- intervention logs;
- behavior hash.

## Desktop-First

UI є desktop-first.

Мінімальний підтримуваний viewport:

```text
1366x862
```

UI повинен автоматично масштабувати layout від `1366x862` до більших desktop resolutions та full-screen режиму. `1920x1080` є primary visual target.

Desktop-first означає:

- пріоритет великих viewport;
- підтримку щільної analytics layout;
- можливість collapse panels;
- збереження primary focus;
- відсутність обов'язкової повної mobile parity.

## Direct Manipulation Through Core Commands

Direct manipulation не може обходити core commands.

Дозволене UI interaction може включати:

```text
select position
preview action
submit approved command
observe committed result
```

Заборонено напряму:

- пересувати live Cells;
- змінювати live Energy;
- змінювати live Materials;
- змінювати live Genome;
- малювати Resources в active World;
- редагувати authoritative state через local UI state.

## Cross-View Consistency

Один і той самий state у різних views повинен збігатися з урахуванням time context, aggregation та rounding.

Це стосується:

- Viewer;
- Inspector;
- charts;
- reports;
- exports;
- contextual help.

Приклад:

```text
Cell Energy in Viewer
=
Cell Energy in Inspector
=
Cell Energy in exported Projection
```

Допустимі відмінності повинні бути явно пояснені через:

- aggregation;
- sampling;
- time interval;
- rounding;
- cached data state.

## Honest Empty States

Якщо mechanism, Projection або data відсутні, UI повинен це пояснювати.

Не допускається підміняти відсутність даних нульовим значенням.

Правильно:

```text
No signal data available.
```

Неправильно:

```text
Signal activity: 0
```

якщо data не збиралися або mechanism ще не реалізований.

## Пов'язані документи

- `GLOSSARY.md`
- `PRINCIPLES.md`
- `STYLE_GUIDE.md`
- `docs/ui/README.md`
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
- indexed by: [[docs/ui/README|UI Layer]]
- governs: [[docs/ui/architecture|UI Architecture]]
- governs: [[docs/ui/navigation|UI Navigation]]
- governs: [[docs/ui/visualization|UI Visualization]]
- governs: [[docs/ui/analytics|UI Analytics]]
- governs: [[docs/ui/exploration|UI Exploration]]
- governs: [[docs/ui/presentation|UI Presentation]]
- governs: [[docs/ui/interaction|UI Interaction]]
- governs: [[docs/ui/quality|UI Quality]]
