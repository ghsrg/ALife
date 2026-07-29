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
- у Monitor займати сталий правий track без collapse або close;
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

For Monitor, this flexibility is constrained by the final layout contract:
Contextual Inspector always occupies its fixed right track, while only Focus is
an overlay. Inspector is neither collapsed nor converted into a drawer at a
supported viewport.

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
1366 × 862
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

Monitor exception: Contextual Inspector remains the fixed right track; compact
layout may reduce nonessential density, but never converts Inspector into an
overlay/drawer or collapses it.

### Below Minimum Viewport

Нижче canonical minimum допускається degraded layout, але UI повинен:

- залишатися керованим;
- не приховувати критичні controls;
- явно пріоритезувати Viewer або active task;
- уникати горизонтального overflow для основних application controls;
- не обіцяти повний Research workflow.

For Monitor below `1366 × 862` CSS px, retain minimum Grid tracks and use
root/page vertical scroll instead of shrinking panels further or introducing
Data Panel-only scroll for layout fit.

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

## Final Monitor Alignment Decisions

> Цей розділ фіксує узгоджений final contract для `Monitor`. Він уточнює
> загальні правила panel behavior саме для `Monitor`, але не скасовує
> тематичні UI Canon, Core/Observer authority або command boundary.

### Stable layout

На baseline viewport `1920 × 1080` Monitor має стабільні Grid tracks. Зміна
Level, layer/filter, selection, Inspector content, Data Panel content або
Focus Panel **не** змінює розмір Map чи інших blocks.

Map може змінити розмір лише через resize application viewport або explicit
full-screen mode. Кожен track має minimum useful size; жоден UI switch не може
звести canvas або panel до нульового розміру чи приховати його.
Users cannot drag-resize Monitor tracks; this prevents layout drift and
zero-size panels.

`Map fullscreen` is a distinct mode: only Map and its eligible Focus overlay
remain on screen. It preserves current viewport, layers, selection, Pin, and
Data Context. Contextual Inspector, Level, Layers, and application chrome do
not consume Map space. The same Data Panel content may be raised on demand as
a bottom overlay at its normal Monitor track height; opening it never resizes
Map. Fullscreen is view-only: Run commands and every other control surface
require return to normal Monitor.

Between `862` and `1080` CSS px of viewport height, vertical Grid tracks adapt
with bounded CSS sizing; additional height grows primarily Map/Data area. This
is not global UI scaling: text and interaction targets retain their minimum
readable size.

Під час bootstrap, scenario change або World initialization camera автоматично
виконує `Fit World`: увесь World займає максимум доступної Map area зі
збереженням aspect ratio. Після ручного pan/zoom live frames і panel switches
не скидають camera. При resize viewport зберігаються world-space center і zoom;
допускається лише clamp до World bounds. Окремий `Reset view` не існує.

### Global Navigation and application actions

Усі canonical workspaces завжди видимі та clickable у фіксованому порядку:

```text
Monitor | World Editor | Experiments | Evolution | Library | Analysis
```

Disabled decorative placeholders заборонені. Global actions:

| Action | Purpose and source | Result |
|---|---|---|
| Warnings | source-backed Core/Observer diagnostics with severity/count | opens `Diagnostics` filtered to warnings |
| Theme | local `Dark` / `Light` UI preference | never changes run, config hash, or Data Context |
| Locale | local `uk-UA` / `en-US` preference | technical canonical terms may remain English |
| Help | current workspace and active Level | opens non-blocking in-app Canon/docs help |
| Settings | UI scale, density, accessibility, shortcuts, connection target | simulation configuration remains in World Editor/run flows |

### Run and Data Context Bar

The persistent bar shows data source/state, run/scenario identity, displayed
Tick, simulation rate, visualization FPS, effective seed, Frame Age, and run
controls.

| Item | Source | Rule |
|---|---|---|
| `SEED` | Runner `effective_seed` after any override | read-only; changed only before launch through World Editor/launch flow; Bootstrap uses it as root seed |
| `FRAME AGE` | Runner latest committed Tick from independently refreshed live status minus displayed projection Tick; optional secondary delivery duration from local receive time minus `wall_clock_generated_at_ms` | primary indication of distance from live, not RTT/latency; shown as `N ticks · M ms` while Runner advances; Core `Paused` is explicit and does not accumulate Tick lag; unavailable status is `stale/disconnected`, never fake `0` |
| `VISUAL FPS` | UI renderer telemetry | independent from simulation rate |
| `Play/Pause` | approved run command/state | Pause freezes Core execution; Play resumes it |
| `Step` | approved run command | compatible paused state only |
| `Stop` | approved run command | standard confirmation includes run id, displayed Tick, and end-of-run warning |
| `Speed` | approved target simulation rate command | controls Core life/execution rate, separate from visualization FPS; default is contract real-time TPS; finite range is `1…10,000 ticks/s`; logarithmic slider plus editable TPS input; adjacent `Unlimited` is a distinct explicit command/state, not a slider value; disabling it restores previous finite TPS; separate `Real-time` resets finite rate to contract default and disables `Unlimited` |

`Jump to Live` and RTT/`Latency` are not Monitor controls. Normal Monitor
renders the latest received committed projection; missed live frames are not
replayed. Debug buffer return to the latest sample uses its `LIVE` marker, not
a Core command.

### Analysis Level and selection

`World`, `Cells`, `Organisms`, `Lineages`, `Evolution`, and `Analytics` are
active research lenses, not global workspace navigation. They retain Data
Context and change Map interpretation, permitted Layers/Filters, Inspector
schema, and Data Panel content. The Map viewport itself remains unchanged.

| Level | Selection | Map/Focus result |
|---|---|---|
| World | one canonical `World block` (one Resource/Field grid cell) | aggregates Resources, Fields, Cells, and remains in that grid cell; double click opens World Focus. Multi-selection stays in Inspector and never opens Focus |
| Cells | Cell | Cell Inspector and Cell Focus |
| Organisms | observer-side `OrganismView` at displayed Tick | Organism Inspector and Focus; never Core authority |
| Lineages | Cell selects its `lineage_id` | highlights carriers of that lineage in current run/displayed Tick; OrganismView is not a lineage selector |
| Evolution | Cell selects its Genome | highlights currently available Genome carriers; Focus shows Genome provenance/graph |
| Analytics | chart/bar/segment selects an analytical subset | Map uses `Highlight` by default; `Hide` and `Isolate` are explicit alternatives; no Focus by default |

Incompatible selection is cleared with an explicit reason when Level changes.

Clicking and dragging without a modifier pans Map; wheel/scroll zooms it.
`Shift + click` adds or removes one compatible Map target from the current
selection set. `Shift + drag-select` draws a selection frame and adds every
compatible target it intersects to the set. A multi-selection is rendered and
analysed through Inspector/Data Panel only; it never opens Focus. Clicking
empty Map clears the selection and returns Inspector to `World total`.

On a live projection, an unpinned selection follows the same entity or set and
refreshes Inspector/Data Panel on every displayed Tick. `Pause` freezes the
displayed context; `Pin` is a separate read-only comparison baseline.
`Dead` is still a valid lifecycle state of the selected entity, so the
selection remains and shows its final data. If an entity genuinely disappears
from the projection, its selection is cleared, Focus closes, and a temporary
reason is shown.

### Layers and filters

`Cells` are the structural foreground and are never visually covered by
Resource/Field layers. Resource and Field layers are data-bound background
layers; multiple active layers composite with their own semantic colours. There
is no `Primary Color Mode`.

The panel is one vertical list with groups `Fields`, `Resources`, `Cell Energy`,
`Structure`, and `Selection`. Only the dynamic Fields/Resources rows scroll.
Each active dynamic layer has a compact row (swatch, name, toggle, gradient).
Expanding it reveals unit, min/max, normalization, and full legend.
Those values affect colour normalization only: they never hide or exclude Map
values.

Layers and Filters are Map presentation state only: they neither change Data
Context nor remove data from Inspector/Data Panel or selection. The sole
cross-surface exception is an explicit `Analytics` interaction: a selected
metric segment/series may highlight its source-backed subset on Map according
to the Analytics rule. It is a presentation highlight only and leaves current
selection and Inspector unchanged.

`Cell Energy` is mutually exclusive:

```text
Cells     direct per-Cell Energy encoding
Heatmap   observer-side aggregate by the same `World block` grid
```

Heatmap must state its unit, bin size, sampling, and aggregate provenance. It
is not a physical World Energy field. `Joints` are initially one neutral
structural layer; future channel/type colours require source-backed projection.
Trail is a polyline for one explicitly selected target only, from retained
samples after selection; it never interpolates missing points.

`Cell Energy`, `Joints`, and `Trail` are available only at `Cells` and
`Organisms` Level. At every other Level they are absent and not rendered; their
user state is retained and restored on return to a compatible Level. Their
shared state also persists when switching directly between `Cells` and
`Organisms`.

`Fields` and `Resources` remain available at every Level. They are enabled by
default at `World`, `Cells`, and `Organisms` to preserve environmental context;
at `Lineages`, `Evolution`, and `Analytics` they are initially disabled but
available on demand, so they do not obscure carrier or analytical highlights.
Once the user changes a common layer, that state persists across Level changes;
the defaults apply only until a user override exists.
`Lineages`, `Evolution`, and `Analytics` add no separate Layer by default:
their carrier/metric highlighting is presentation state, not a duplicated
layer. A contextual Level-specific layer may be added only for a distinct
source-backed raster or geometry.

### Inspector, Pin, and Focus

Contextual Inspector is a fixed right track. Without selection it shows
`World total`: lifecycle counts, total Cell Energy, source-backed Resource
totals, Joint count, displayed Tick, run state, projection completeness, and
warnings. With selection it shows the compatible entity or selection-set
aggregate.

`Pin` stores one read-only baseline snapshot:

```text
entity or selection set + Data Context + displayed Tick + completeness
```

New selection becomes the current comparison target. Data Panel compares only
`Pinned baseline` and `Current selection`; incompatible entity types never get
a fake delta table. If the current live target disappears, its selection clears
but the pinned baseline remains until `Unpin` removes it.
`Stop` followed by a new run clears Pin, current selection, and Focus because
their identities belong to the prior run.
For a compatible Data Panel comparison, current data remains the primary chart
encoding; `Pinned baseline` overlays it as a labelled outline/dashed contour on
the same scale. This is a separate selection-comparison surface for the pinned
and current targets, not a recalculation of Level baseline distributions or
histograms. UI does not create a second duplicate chart card.

Focus is a `413 × 399` stable overlay in the upper-right of Map. It never
resizes Map or moves with the selected entity. Single click selects; double
click on the selected compatible target opens/closes Focus; `Escape` or close
closes it. Multi-selection uses Inspector only.

Focus is Level-bound and procedural/data-bound: geometry, Materials, Joints,
Processes, Genome and other supported projection data define its pseudo-3D
presentation. Behavior Profile/role may add a provenance-marked accent with
confidence, interval, and classifier version; it must not generate fake
anatomy or create biological types.
At `Analytics` Level, Focus is absent by default: metric provenance,
aggregation, interval, and detail belong to Data Panel instead.

### Data Panel and analytical truthfulness

Data Panel has no tabs. Its content changes only from:

```text
active Analysis Level + analysis scope + optional Pin
```

`analysis scope` is `World total` without a World selection, one selected
`World block` at World Level, or the current multi-selection at any compatible
Level. A single Cell/Organism selection does not rebuild distribution charts or
histograms; its detail belongs to Inspector/Focus. With a compatible `Pin`, it
also supplies the current side of the separate selection-comparison surface.

Vertical overflow of Data Panel content belongs to root/page scrolling, never a
Data Panel-only layout scrollbar. The only local vertical scrolling in Monitor
is the dynamic `Fields`/`Resources` list.

`Raw Data` belongs to Diagnostics. At `World` level Data Panel includes a
`Population Lifecycle` stacked bar sourced from counts of canonical
`Alive/Stressed/Dormant/Dead` states in the same displayed projection; labels
and exact counts remain visible.

With one selected `World block`, every World-level aggregate in Data Panel is
scoped to that block. With no selection, it is scoped to `World total`.
The same rule applies at every Level: a compatible entity or selection set
scopes all Data Panel aggregates, distributions, and histograms to itself;
without selection, the Level baseline context is used.

World accounting has an explicit `Accounting target` selector:

```text
Resource | Material | Energy
```

`Energy` is the default accounting target for a new run. After an explicit user
choice, the target is retained for the rest of that run.
Selecting `Resource` or `Material` requires a second explicit selector for one
registry type. UI does not combine all types into one Cycle without a separate
validated accounting contract; `Energy` requires no second selector.

`Matter Cycle` shows source-backed Resource/Material locations and flows. It
keeps Resources, Materials, MaterialFragments, decomposing Cells, and explicit
sinks distinct. `Energy Flow` is separate: Core/Observer must supply produced,
stored Cell Energy Buffer, spending by registered category, Heat, explicit loss,
and unaccounted difference. Until that contract exists, it is `unavailable`;
UI must not estimate it from resource energy values.

Cycle shows location shares together with absolute target total. Its companion
time chart is a stacked `100%` distribution across those locations; absolute
target amount is shown separately, so distribution and total change are not
confused.

Every time series uses one UI RRD history, not `Recent`/`Since start` modes.
It stores at most 1,000 compact metric/trajectory samples: 100 newest
consecutive samples, then successively decimated 10× tiers. It is not a store
of full World frames. Its time axis uses actual Tick/time positions and visibly
communicates the changing sampling density; it must not imply equal intervals
between decimated samples. For numeric time series, a collapsed RRD interval
stores its mean value; the chart joins those aggregated samples with a line as
a trend, not as a claim of continuous observation. Trail stores only `(Tick,
x, y)` for the selected target after selection; a collapsed Trail interval
stores mean `(x, y)`, making older trajectory smooth but approximate. Chart and
Trail tooltips identify the represented Tick/time interval and aggregation.

Data Panel per-level baseline:

| Level | Required content |
|---|---|
| World | Population Lifecycle; selected Matter Cycle/Energy Flow; paired time evolution |
| Cells | Observed primary-role distribution as bars; Potential-role share as labelled markers; Cell radius distribution |
| Organisms | primary observed Behavior Profile distribution; organism-size bins by Cell count |
| Lineages | current population, history, compact genealogy, spatial footprint |
| Evolution | Genome provenance, mutation history, diversity, carrier history |
| Analytics | selected metric plus definition, unit, aggregation, interval, sampling/completeness, and applicable classifier/source version |

Potential and Observed roles use the same Data Context, interval, and classifier
version but are distinct encodings. Potential markers may total above 100%
because a Cell may have several capabilities.

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
