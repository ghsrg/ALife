---
tags:
  - alife
  - implementation
  - ui
  - roadmap
  - control-center
---

# UI Implementation Plan

> **For agentic workers:** це high-level parent plan, а не готовий task-by-task план. Перед початком кожного implementation slice потрібно створити окремий `PLAN` worklog із checkbox-кроками, файлами, тестами та acceptance gate. Після завершення slice створюється відповідний `REPORT`.

## Призначення

Цей документ визначає загальну послідовність реалізації `ALife Control Center`.

Він переробляє попередній `Phase Visual Global Roadmap`, який був розділений на `Visual A–J`, у три продуктові етапи:

```text
1. Start
   WOW effect, демонстрація і перевірка наскрізного контуру

2. Debug
   точні дані, звірка Core/UI, балансування, конфігурація і тестування

3. Research
   повноцінний науковий інструмент для експериментів та аналізу еволюції
```

Цей plan:

- не є UI Canon;
- не змінює Core architecture;
- не визначає simulation laws;
- не є детальним списком coding tasks;
- визначає порядок продуктового нарощування UI;
- є основою для майбутніх детальних worklogs.

При конфлікті пріоритет мають:

```text
docs/PRINCIPLES.md
docs/GLOSSARY.md
docs/ui/*
accepted ADR
docs/engine/technology-stack.md
```

## Головна ідея плану

UI не потрібно одразу будувати як повний research suite.

Правильний порядок:

```text
спочатку показати живий і привабливий World
потім довести, що даним можна довіряти
після цього перетворити UI на повний research instrument
```

Кожна частина має завершуватися самостійним корисним продуктом.

## Core Contract

```text
Viewer observes.
UI requests.
Core validates.
Core applies.
Core records.
UI observes result.
```

`alife-core` залишається єдиним джерелом simulation truth.

UI:

- не змінює `WorldState` напряму;
- не впливає на Tick semantics;
- не змінює behavior через visibility, filters або selection;
- не підміняє committed data інтерпольованими values;
- не повторює command без explicit confirmation від Core.

## Прийнятий технологічний напрям

```text
local Web Viewer
Chromium-based runtime
WebSocket binary frame stream
WebGL2 instanced rendering
optional Canvas overlays
HTML/CSS application shell
2D simulation
pseudo-3D presentation
3D-compatible boundaries, але без 3D dependency
```

Для charts і складних UI components може використовуватися окрема web library, але вона не повинна входити в simulation hot path.

## Три частини

| Частина | Продуктова мета | Основне питання |
| --- | --- | --- |
| `UI-1 Start` | Показати проєкт, отримати WOW effect і довести наскрізну працездатність | «Це виглядає живим і справді працює?» |
| `UI-2 Debug` | Дати точний інструмент для звірки, пошуку помилок і балансування | «Чи можемо ми довіряти кожному значенню та пояснити результат?» |
| `UI-3 Research` | Дати повний інструмент експериментів, еволюційного аналізу й порівняння | «Чи можна на цьому проводити відтворювані дослідження?» |

## Consolidated Worklog Inputs

Цей документ є canonical implementation roadmap для UI phases.

Попередні worklogs:

- `outputs/worklogs/2026-07-02-1935-PLAN-phase-visual-global-roadmap.md`;
- `outputs/worklogs/2026-07-02-1936-PLAN-phase-visual-global-UIUX.md`;

є historical planning inputs. Вони не створюють окремий паралельний roadmap.
Якщо між worklog і цим документом є розбіжність, agent повинен оновити цей
implementation plan або створити новий detailed worklog, а не трактувати
старий worklog як вищий authority.

### Visual A-J To UI-1/2/3 Mapping

| Historical phase | Canonical destination | Notes |
| --- | --- | --- |
| Visual A Read-Only Debug Viewer | `UI-1 Start` + `UI-2 Debug` | `Start` бере живий Viewer, базові layers, selection і screenshot. Exact debug parity, recorded scrubbing і exhaustive correctness переходять у `Debug`. |
| Visual B Scenario Runner UI | `UI-1 Start` + `UI-2 Debug` | `Start` бере scenario list, launch, Play/Pause/Step/Stop, seed/hash display. Run summaries, collapse analysis, comparisons і advanced time controls переходять у `Debug`/`Research`. |
| Visual C Layered World Inspector | `UI-1 Start` + `UI-2 Debug` | `Start` бере мінімальні live layers і basic Cell Inspector. Exact grid, resources/materials/process/contact inspectors, filters і raw overlays переходять у `Debug`. |
| Visual D World Initialization Editor | `UI-2 Debug` | Pre-run config editor, resource distribution preview, validation і config hash належать до Debug. Live-world mutation заборонена. |
| Visual E Experiment Dashboard | `UI-2 Debug` + `UI-3 Research` | Debug бере basic multi-seed/scenario runs і balance tables. Full sweeps, matrix experiments і long-running comparison належать до Research. |
| Visual F Genome And Evolution Observatory | `UI-3 Research` | Genome, mutation, lineage, similarity, selection і evolution views не входять у Start. |
| Visual G Organism Inspector And Observatory | `UI-3 Research` | OrganismView є observer-side derived view. У Start/Debug допускаються лише мінімальні placeholders або raw cell/joint projections, якщо вони потрібні для перевірки. |
| Visual H Library, Save And Placement Center | `UI-3 Research` | Saved Cells/Species/Organisms і placement/intervention workflows вимагають explicit command API та Research-level provenance. |
| Visual I Advanced Analysis And Reporting | `UI-3 Research` | Narrative reports, discoveries, advanced statistics і publication-style export належать до Research. |
| Visual J Polishing, Accessibility And Scale | Cross-part | Базові empty/error/loading states, Light/Dark і usable 1024x768 потрібні з Start. Full accessibility, scale hardening і performance envelopes завершуються поступово. |

### UI/UX Specialization Analytics Mapping

`outputs/worklogs/2026-07-02-1936-PLAN-phase-visual-global-UIUX.md` описує
аналітику:

```text
Organism size
×
Functional Cell Role
×
count / percentage
```

Canonical destination: `UI-3 Research -> Specialization Analytics`.

Цей worklog не є вимогою для `UI-1 Start` або `UI-2 Debug`.

До `UI-3` дозволені лише передумови:

- projection fields, які не hardcode-ять roles;
- explicit classifier metadata, якщо classifier уже існує;
- raw Cell/Material/Process data для Debug Inspectors;
- OrganismView як observer-side graph, без behavior authority.

Заборонено переносити labels на кшталт `neural-like`, `muscle-like`,
`predator-like`, `sensory` у Core behavior або scenario shortcuts. У Research
вони можуть бути тільки derived classifications із criteria, confidence,
version і provenance.

### Current Start Slice Status

Стан на момент sync:

- `UI-1A Application Shell And Deterministic Fixture Viewer` реалізовано.
- `UI-1B Live Projection Transport And Run Controls` реалізовано локально:
  HTTP bootstrap, scenario list, run controls, WebSocket `ALIF v2`, live frame
  adapter, CORS для local Control Center.
- `UI-1B-Cleanup Live State Clarity` реалізовано як bridge worklog після
  `UI-1B`: state clarity для fixture/live/stale, reconnect retry, explicit
  unavailable resource projection і `Step 1` label.
- `UI-1 Start` ще не завершено повністю.

`UI-1B-Cleanup` дозволений як bridge worklog перед `UI-1C`. Він може виправляти
state clarity, reconnect behavior і misleading labels із завершеного `UI-1B`
slice, але не повинен розширювати `ALIF`, resource projection, semantic zoom або
Inspector scope. Це належить до `UI-1C` або пізніших фаз.

Known gaps before `UI-1 Start` acceptance:

- live resource grid/heatmap не входить у поточний `ALIF v2` payload;
- live adapter поки повертає `resources: []`;
- live Cell visual radius має presentation minimum і може перебільшувати
  фізичний overlap;
- speed controls, semantic zoom, full-screen, richer Inspector і final Start
  demo hardening ще попереду.

Отже, наступний canonical detailed worklog після `UI-1B`:

```text
UI-1C:
WOW World Rendering, Projection Truthfulness, Semantic Zoom And Cell Inspector
```

`UI-1C` повинен закрити truthfulness gap перед глибоким поверненням до Genome:
показувати лише data-bound layers, явно маркувати missing projection fields,
узгодити live radius/render scale, і підготувати resource projection contract
або чесний unavailable state.

# Спільна архітектурна основа

Ці компоненти створюються поступово, але не повинні мати окремі несумісні реалізації для кожної частини.

## Application Shell

Canonical application:

```text
ALife Control Center
```

Спільна оболонка включає:

- global top navigation;
- active run і `Data Context`;
- run state;
- Tick;
- simulation rate;
- visualization FPS;
- settings;
- help;
- global warnings;
- shared notification and confirmation layer.

У `Start` відображаються лише реально реалізовані workspaces.

Не потрібно показувати порожні placeholder-pages лише для демонстрації майбутньої архітектури.

До завершення `Research` повинні бути доступні всі canonical workspaces:

```text
Monitor
World Editor
Experiments
Evolution
Library
Analysis
```

## Shared Viewer

Один canonical `Viewer` використовується для:

- live World;
- recorded World;
- World Editor preview;
- Evolution spatial view;
- Analysis spatial results;
- placement;
- Debug Visualization Mode.

Не допускається створення різних renderer semantics для різних workspaces.

## Projection Gateway

UI працює через versioned projection boundary.

```text
Core / Runner
  -> committed snapshots
  -> frame projections
  -> event projections
  -> summary projections
  -> analytics projections
  -> UI
```

Потрібні adapters:

```text
recorded fixture adapter
live WebSocket adapter
engine keyframe adapter
analytics result adapter
```

Усі adapters повинні подавати data в одну UI model.

Recorded fixtures дозволяють почати UI раніше, але `Start` не вважається завершеним до підключення live Core.

## Local And Remote Viewer Modes

ALife Control Center must support two projection connection modes.

### Local default mode

Safe default mode for development and normal local use.

```toml
[server]
bind_host = "127.0.0.1"
port = 8080
allow_remote_viewer = false
```

Rules:
accessible only from the same machine;
no LAN exposure by default;
suitable for local Core + local UI;
must be the default generated configuration.

### Remote viewer mode

Explicit LAN mode for running Core/Runner on one machine and viewing from another machine.

```toml
[server]
bind_host = "0.0.0.0"
port = 8080
allow_remote_viewer = true
read_only_by_default = true
allowed_origins = ["http://192.168.1.51:5173"]
access_token_required = true
```
Rules:

must be opt-in;
intended for trusted LAN only;
remote clients are read-only by default;
simulation-changing commands require explicit permission and Core validation;
UI must show the active connection target;
reconnect must not repeat pending commands automatically;
Core must not block on Viewer rendering.

## Command Gateway

Simulation-changing requests проходять через один command boundary:

```text
run control
checkpoint
branch
placement
approved intervention
```

UI-side preview або validation ніколи не замінює Core validation.

## UI State Separation

UI state має бути розділена щонайменше на:

```text
Simulation Projection State
Navigation State
Selection State
Presentation State
Temporary Historical State
Pending Command State
```

Не допускається змішування:

```text
UI preference
simulation config
authoritative WorldState
```

## UI Text

Усі user-facing texts надходять із `ui-text`.

Початкові locale:

```text
en-US
uk-UA
```

Canonical English terms залишаються основними technical labels.

## Design System

З першого етапу потрібні:

- semantic design tokens;
- Light theme;
- Dark theme;
- Comfortable density;
- Compact density;
- UI scale;
- stable status colors;
- reusable panels;
- Inspector shell;
- in-app dialogs;
- loading, empty, partial та error states.

Візуальна мова одна:

```text
scientific instrument
+
game-like interface
```

Архітектура може дозволяти майбутні skins, але вони не входять у цей plan.

## Interface Design Alignment Gates

`docs/ui/control-center-monitor-v3.png` is sufficient as the visual direction
for `UI-1A`, but it is not a complete design specification.

The first separate interface design session is required after `UI-1A` produces
a working Chromium shell with deterministic fixture rendering, and before
`UI-1C` starts WOW rendering, semantic zoom, high-detail Inspector work, and
finalized design-system choices.

Recommended sequence:

```text
UI-1A Application Shell And Deterministic Fixture Viewer
  -> Interface Design Alignment Session
  -> UI-1B Live Projection Transport And Run Controls
  -> UI-1C WOW World Rendering, Semantic Zoom And Cell Inspector
```

`UI-1B` may proceed before or in parallel with the design session only if it
stays within existing shell/control placeholders and does not lock in visual
language, layout density, or renderer style decisions. `UI-1C` must not start
without the alignment result.

The first design session must decide:

- Monitor layout acceptance against `control-center-monitor-v3.png`;
- Light/Dark visual direction;
- base design tokens;
- panel density, spacing, and typography scale;
- toolbar and run-control states;
- Viewer layer style and color semantics;
- Inspector hierarchy;
- what remains in `Start` scope versus what is deferred to `Debug` or `Research`.

Alignment result:

- phase-specific `UI-1C` design decisions are canonical in
  [[docs/ui/presentation#UI-1C Design Alignment|UI Presentation: UI-1C Design Alignment]];
- shared Control Center shell/layout foundation remains in
  [[docs/ui/control-center-design-spec|Control Center Design Specification]];
- `UI-1C` implementation plans must treat these design decisions as the gate
  between the visual reference and executable scope.

Later mandatory design gates:

- before `UI-2 Debug`: dense debug information architecture, exact layers,
  expanded Inspectors, tables, warnings, and raw-data affordances;
- before `UI-3 Research`: experiment design, evolution views, library,
  reporting, saved assets, and long-running research workflows.

# UI-1 Start — WOW, Demo And End-To-End Proof

## Мета

Створити перший продукт, який:

- можна показати іншій людині без довгих пояснень;
- візуально передає ідею живого World;
- доводить, що Core, transport, Viewer і Inspector працюють разом;
- дає змогу запустити simulation і побачити реальні committed data;
- створює мотивацію продовжувати проєкт.

`Start` не повинен бути широким.

Він повинен бути цілісним.

## Головний критерій WOW

WOW effect виникає не від кількості screens, а від одного завершеного demo flow:

```text
відкрити ALife Control Center
-> обрати scenario
-> запустити World
-> побачити живі Cells і Resource Field
-> змінити speed
-> наблизити Cell
-> побачити pseudo-3D detail
-> відкрити точний Inspector
-> Pause / Step
-> перейти у full-screen
-> зробити screenshot
```

Усі візуальні елементи demo повинні бути data-bound.

Fake simulation events для WOW effect заборонені.

## Primary Workspace

Основний workspace:

```text
Monitor
```

Допускається compact scenario launcher як частина:

```text
Monitor empty state
або
minimal Experiments view
```

Повний `Experiments` workspace відкладається.

## Scope

### Application Shell

Реалізувати:

- top navigation;
- Monitor;
- Settings;
- Help baseline;
- active scenario/run;
- Tick;
- run state;
- simulation rate;
- visualization FPS;
- global status indicator.

### Scenario Start

Користувач може:

- переглянути короткий список demo scenarios;
- запустити selected TOML scenario;
- бачити seed;
- бачити config hash;
- restart із тим самим seed;
- restart із новим seed.

На першому slice допускається recorded deterministic fixture.

До acceptance gate потрібен live run через Runner/Core API.

### Run Controls

```text
Play
Pause
Step 1
Step N
Ticks per second
Frames per second
Stop
```

`Step N` is a UI placeholder for now. Current Runner Canon exposes only
`StepRun`, which executes exactly one committed Tick and is valid only while
Paused. Until a bounded multi-step Runner command is accepted, UI may display
`Step N` as disabled or implement it later as an explicit, bounded sequence of
single-step commands with visible progress and cancellation.

Simulation rate і visualization frame rate показуються окремо.

### World Rendering

Реалізувати:

- 2D top-down World;
- pseudo-3D Cell presentation;
- preserve aspect ratio;
- fit World to Viewport;
- zoom;
- pan;
- reset Viewport;
- responsive resize;
- full-screen;
- smooth movement interpolation між committed snapshots;
- adaptive rendering до 30 або 20 FPS без впливу на Core.

### Initial Layers

Мінімальний набір:

```text
Cells
World bounds
Composite Resource Concentration · Smooth
lifecycle color mode
selection highlight
basic event markers
```

Додатково, якщо data вже доступні:

```text
Energy color mode
damage color mode
Heat / Waste layer
```

### Semantic Zoom

Мінімально реалізувати переходи:

```text
Overview
Entity
Structure
```

`Internal Detail` може залишатися частковим до `Debug`.

На близькому zoom Cell повинна виглядати змістовніше, ніж просте збільшене коло.

### Cell Inspector

Мінімально:

```text
Cell id
position
radius
lifecycle
Energy
damage
Resources
Materials
current Process summary
Genome id, якщо доступний
recent event summary
```

Inspector:

- read-only;
- прив'язаний до displayed Tick;
- показує canonical ID;
- може Focus і Track entity.

### Historical Interaction

На `Start` достатньо:

- Pause;
- current frozen UI frame;
- bounded temporary frozen snapshot;
- engine keyframes, якщо вже доступні;
- `Jump to Live`.

Не потрібно зберігати Projection кожного Tick.

### Presentation

Обов'язково:

- Light;
- Dark;
- configurable semantic colors;
- English canonical labels;
- Ukrainian localized explanation;
- Contextual Help для основних controls і metrics;
- in-app confirmations;
- polished empty/loading/error states.

### Export

Мінімально:

- screenshot current Viewport;
- screenshot full World;
- optional legend;
- run id;
- Tick;
- active layer metadata.

## Start Demo Scenario

Потрібен один versioned deterministic showcase scenario.

Він має демонструвати, наскільки Core уже підтримує:

- рух;
- Resource distribution;
- Energy changes;
- lifecycle state;
- contact;
- growth або division;
- death або depletion;
- кілька візуально різних Cells.

Scenario не повинен приховувати нестабільність Core декоративними ефектами.

## Start Non-Goals

Не входять:

- full World Editor;
- advanced balance dashboard;
- complete Raw Data workspace;
- Genome graph;
- lineage tree;
- advanced OrganismView;
- saved asset Library;
- placement;
- checkpoint branching UI;
- persisted Selection Sets;
- full multi-run research comparison;
- advanced statistics;
- full accessibility subsystem;
- 3D simulation.

## Start Acceptance Gate

`UI-1 Start` завершено, коли:

```text
application starts in a Chromium-based runtime
demo scenario can be launched from UI
live Core data reaches Viewer through the accepted projection boundary
Viewer never mutates WorldState
World scales correctly to Viewport
Cells, Resource heatmap and selection align spatially
selected Inspector values match committed Core data
Play, Pause, Step, Speed and Stop work through approved APIs
Ticks per second and Frames per second are distinct
Pause or explicit selection can freeze a bounded UI frame
Jump to Live works
Light and Dark themes are usable
1024×768 remains usable
20–30 FPS is acceptable under minimum profile
screenshot export works
same seed/config can reproduce the demo
no browser-native alert/confirm is used
```

## Start Detailed Worklogs

Рекомендовані child plans:

```text
UI-1A:
  Application Shell And Deterministic Fixture Viewer

UI-1B:
  Live Projection Transport And Run Controls

UI-1C:
  WOW World Rendering, Semantic Zoom And Cell Inspector

UI-1D:
  Start Demo, Export And Acceptance Hardening
```

Не потрібно реалізовувати всі `Start` requirements одним worklog.

# UI-2 Debug — Verification, Balancing And Engineering Control

## Мета

Перетворити привабливий Viewer на інструмент, якому можна довіряти під час розробки Core.

`Debug` повинен відповідати на питання:

```text
що саме відбулося
на якому Tick
з якою Cell
чому Process виконався або був відхилений
куди поділись Resource, Material і Energy
чи збігаються Core state, events, summaries і UI
чи стабільний World
```

## Primary Workspaces

```text
Monitor
World Editor
Experiments
Analysis
```

`Evolution` і `Library` можуть ще бути недоступними.

## Exact Debug Visualization

Реалізувати `Debug Visualization Mode`.

Можливі layers:

- exact Resource grid;
- Field samples;
- grid coordinates;
- SpatialIndex cells;
- collision bounds;
- Cell bounds;
- raw vectors;
- contact points;
- Joint lines, якщо доступні;
- Process activity;
- Feasibility rejections;
- snapshot boundaries;
- interpolation state;
- LOD level;
- projection ids;
- missing Projection fields;
- frame timing.

Debug mode може використовувати flat 2D rendering.

## Exact And Smooth Heatmaps

Для Fields і Resources реалізувати:

```text
Exact
Smooth
```

Правила:

- `Exact` показує реальну grid/sample structure;
- `Smooth` показує interpolation;
- tooltip та Inspector показують actual sampled value;
- interpolation mode завжди видимий.

## Expanded Inspectors

### Cell

Додати:

- full Resource inventory;
- full Material inventory;
- Energy accounting;
- current і recent Processes;
- Capabilities;
- movement/force vector;
- contacts;
- growth readiness;
- division readiness;
- Feasibility rejection history;
- local Fields;
- history summary.

### Resource / Material / Field

Додати:

- local amount;
- World total;
- distribution;
- input;
- output;
- conversion;
- consumers;
- producers;
- sinks;
- history.

### Process

Додати:

- execution count;
- success/rejection count;
- RejectionReasons;
- input/output;
- Energy cost;
- affected Cells;
- recent executions.

## Search, Filters And Selection

Реалізувати:

- exact ID search;
- partial ID search;
- recent entities;
- basic attribute search;
- single selection;
- multi-selection;
- rectangle selection;
- filters;
- `Highlight`;
- `Hide`;
- `Isolate`;
- pinning;
- bounded entity comparison.

Persisted `Selection Set` залишається для `Research`.

## Historical Debugging

Потрібні:

- engine-created keyframes;
- event markers;
- checkpoint markers, якщо Core підтримує;
- bounded temporary frozen frames;
- visible unavailable Ticks;
- historical Inspector;
- last available state;
- no silent nearest-frame substitution.

Keyframe cadence визначає engine/storage, а не UI.

## Scenario Runner

Розширити scenario controls:

- scenario list;
- validation result;
- config hash;
- seed;
- same-seed restart;
- new-seed restart;
- run summary;
- collapse reason;
- event summary;
- artifact links;
- basic run comparison;
- config diff.

## World Editor

Реалізувати pre-run editing:

- draft TOML configuration;
- World dimensions;
- initial Resource distributions;
- fields;
- Environment parameters;
- initial Cell placement;
- initial saved config preview;
- validation;
- config hash preview;
- versioned autosave;
- recovered draft.

World Editor не редагує active `WorldState`.

## Debug Analytics

### Overview

Показати:

- Cell count;
- lifecycle counts;
- total Energy;
- total Resources;
- total Materials;
- Heat;
- Waste;
- births;
- deaths;
- divisions;
- rejected Processes;
- collapse status.

### Balance

Окремо:

```text
Matter Cycle
Energy Flow
```

Для Resources і Materials:

```text
Input
Stored
Converted
Released
Explicit Sink
Unaccounted Difference
```

Для Energy:

```text
Produced
Stored
Spent by category
Heat
Explicit Loss
Unaccounted Difference
```

`Unaccounted Difference` не приховується.

### Charts

Мінімально:

- population over time;
- lifecycle over time;
- total Energy;
- Resource totals;
- Material totals;
- Heat/Waste;
- birth/death/division rates;
- Process success/rejection;
- balance difference.

### Warnings

Додати engineering warnings:

- conservation mismatch;
- unbounded accumulation;
- starvation cascade;
- excessive dormancy;
- numerical drift;
- missing data;
- stale Projection;
- failed classifier або metric;
- corrupted artifact.

## Raw Data

Реалізувати:

- table/grid;
- filtering;
- sorting;
- column selection;
- copy value;
- open entity;
- show in Viewer;
- CSV export;
- JSON debug export;
- virtualization;
- bounded queries.

Arbitrary calculated columns не входять.

## Basic Experiment Support

Для балансування потрібен мінімальний `Experiments` baseline:

- запуск групи scenarios;
- кілька seeds;
- bounded queue;
- run status;
- summary table;
- basic config comparison;
- export results;
- links to existing headless sweep artifacts.

Повний experiment designer відкладається до `Research`.

## Diagnostics And Recovery

Реалізувати:

- local diagnostics;
- Core/UI version;
- runtime version;
- visualization FPS;
- frame time;
- memory estimate;
- request failures;
- Data Context;
- connection state;
- diagnostic export;
- disconnected read-only mode;
- reconnect;
- no automatic command retry.

External telemetry залишається disabled by default.

## Debug Test Foundation

Створити versioned deterministic fixtures:

```text
small World
large population
Resource starvation
division
death and decomposition
Process rejection
contact / collision
multicellular structure, якщо доступна
missing data
partial Projection
connection loss
placement rejection, коли placement буде реалізовано
```

Для fixture зберігати:

- scenario config;
- seed;
- expected events;
- expected metrics;
- expected warnings;
- expected keyframes;
- visual reference, де це стабільно.

## Debug Non-Goals

Не входять повністю:

- advanced Genome Observatory;
- lineage analytics;
- registry-driven Functional Cell Roles;
- sensory specialization;
- Behavior Profile research;
- saved Cell/Species/Organism Library;
- full placement workflow;
- checkpoints and branches as research workflow;
- persisted Selection Sets;
- full statistical comparison across many runs;
- Analytical Summary як research narrative.

## Debug Acceptance Gate

`UI-2 Debug` завершено, коли:

```text
exact and smooth modes are clearly distinguished
all displayed debug values can be traced to Projection or event source
selected historical state shows the correct Tick
unavailable Tick is never shown as a complete state
Cell, Resource, Material and Process Inspectors expose exact data
Feasibility rejection reasons can be inspected
Matter and Energy accounting are separate
Unaccounted Difference is visible
World Editor produces a validated pre-run config
same config and seed can be relaunched
basic multi-seed runs can be compared
connection loss produces a stale read-only state
reconnect never repeats a pending command silently
debug layers do not affect simulation behavior
deterministic fixtures cover critical flows
diagnostic export works
```

## Debug Detailed Worklogs

Рекомендовані child plans:

```text
UI-2A:
  Versioned Projections, Keyframes And Historical Data

UI-2B:
  Debug Visualization Mode And Exact Layers

UI-2C:
  Inspectors, Search, Filters And Entity Comparison

UI-2D:
  Balance Analytics, Warnings And Raw Data

UI-2E:
  World Editor And Scenario Runner

UI-2F:
  Debug Experiments, Fixtures, Diagnostics And Recovery
```

# UI-3 Research — Full Scientific Instrument

## Мета

Перетворити `ALife Control Center` на повний інструмент для:

- планування експериментів;
- порівняння runs;
- аналізу population dynamics;
- дослідження Genome та lineage;
- аналізу OrganismView;
- виявлення функціональної спеціалізації;
- аналізу Behavior Profiles;
- контрольованих interventions;
- checkpoints і branching;
- відтворюваного scientific export.

## Full Workspace Set

На цьому етапі доступні всі canonical workspaces:

```text
Monitor
World Editor
Experiments
Evolution
Library
Analysis
```

## Experiments

Реалізувати:

- experiment definitions;
- multiple scenarios;
- seed sets;
- parameter sweeps;
- experiment matrix;
- queue;
- progress;
- control run;
- intervention run;
- checkpoints;
- branches;
- run groups;
- artifacts;
- notes;
- reproducibility metadata;
- experiment export.

## Run Comparison

Canonical alignment:

```text
By Tick
By simulation time
By selected event
```

Підтримати:

- control vs intervention;
- seed vs seed;
- branch vs branch;
- interval vs interval;
- bounded detailed run set;
- aggregate comparison для великих наборів;
- config diff;
- metric diff;
- event diff;
- spatial comparison.

## Evolution Observatory

Реалізувати:

- Genome Inspector;
- Genome parent/descendant relations;
- mutation history;
- carriers;
- lineage tree;
- lineage population over time;
- Genome similarity;
- diversity;
- spatial lineage distribution;
- extinction;
- dominance changes;
- inheritance events;
- generation analysis.

Genome та lineage views залишаються observer-side.

## Organism Observatory

Реалізувати:

- OrganismView outline;
- member Cells;
- Joints;
- structure;
- Genome composition;
- Resource composition;
- Material composition;
- Energy;
- internal flows;
- Functional Cell Roles;
- Behavior Profiles;
- OrganismView history;
- comparison;
- collapse analysis.

`OrganismView` не стає behavior entity.

## Derived Classification

Реалізувати registry-driven:

```text
Potential Functional Roles
Observed Functional Roles
Sensory Specialization
Behavior Profiles
```

Для classification показувати:

- canonical id;
- primary label;
- secondary labels;
- confidence;
- components;
- interval;
- classifier version;
- Potential / Observed distinction;
- methodology;
- limitations.

UI не hardcode-ить повний список classes.

## Specialization Analytics

Підтримати:

```text
Organism size
×
Functional Cell Role
×
count / percentage
```

Canonical default bins:

```text
1
2
3–4
5–9
10–19
20–49
50–99
100–199
200–499
500–999
1000+
```

Rules:

- grouping зберігається в versioned analytics configuration;
- показ від першого bin до останнього populated;
- мінімум п'ять bins;
- порожній хвіст після останнього populated bin приховується.

Counting modes:

- Organisms containing at least one selected Cell;
- total selected Cells;
- average selected Cells per Organism;
- percentage inside Organism;
- dominant-role Organisms.

Classification modes:

- Primary only;
- All matched labels;
- Fractional contribution.

## Advanced Analysis

Реалізувати:

- flows;
- population;
- specialization;
- balance;
- run comparison;
- interval comparison;
- warnings;
- discoveries;
- exact accounting;
- advanced cross-filtering;
- metric provenance;
- report generation.

Advanced statistics можуть включати:

- mean;
- median;
- min/max;
- percentiles;
- standard deviation;
- confidence intervals;
- distribution across seeds.

Вони бажані, але можуть реалізовуватися окремим detailed plan після базового research workflow.

## Analytical Summary

Додати `Analytical Summary`.

Summary:

- базується лише на metrics/events;
- посилається на evidence;
- показує interval;
- розрізняє observed correlation і probable cause;
- не є authoritative scientific conclusion;
- може бути exported.

## Selection Sets

Реалізувати persisted `Selection Set`.

Selection Set може бути створений через:

- manual multi-selection;
- rectangle selection;
- filter;
- chart;
- analytical query.

Зберігає:

- canonical ids;
- selection method;
- Data Context;
- filters;
- missing member state;
- compatible application rules.

Selection Set не є simulation input.

## Library

Один workspace:

```text
Library
```

Sections:

```text
System Catalog
Saved Assets
```

Saved Assets:

- Cell;
- Species;
- Organism;
- scenario;
- config;
- checkpoint;
- experiment definition;
- saved view;
- filter preset;
- Selection Set.

## Placement

Canonical flow:

```text
select saved asset
-> Placement Mode
-> ghost preview
-> choose position
-> UI validation
-> confirm
-> Core validation
-> Core applies
-> Core records
-> Viewer observes
```

Базова модель:

- one placement per mode;
- no canonical `Place Multiple`;
- invalid position disables confirmation;
- intervention is logged;
- no direct moving of existing Cells.

## Checkpoints And Branching

Реалізувати:

- create checkpoint;
- checkpoint metadata;
- branch from checkpoint;
- branch lineage;
- compare parent and branch;
- checkpoint compatibility;
- intervention history;
- replay links;
- storage status.

Checkpoint policy для intervention:

```text
low risk:
  optional

medium risk:
  recommended

high risk:
  required when supported
```

## Research Export

Підтримати:

- screenshot;
- full World image;
- chart image;
- chart data;
- CSV;
- JSON;
- Parquet link/artifact;
- dashboard snapshot;
- run comparison report;
- balance report;
- warnings;
- discoveries;
- Analytical Summary;
- entity report;
- Genome/lineage report;
- experiment manifest.

Metadata:

- run id;
- branch;
- seed;
- config hash;
- Core version;
- UI version;
- Tick/interval;
- filters;
- metric version;
- classifier version;
- aggregation;
- sampling;
- data completeness.

## Research Non-Goals

Не входять автоматично:

- 3D simulation;
- game engine migration;
- GPU simulation compute;
- arbitrary live state editing;
- user-defined unvalidated formulas;
- hidden AI-generated scientific conclusions;
- automatic behavior control from classifications;
- full mobile parity.

## Research Acceptance Gate

`UI-3 Research` завершено, коли:

```text
all six workspaces form one coherent application
experiment can be defined, launched and reproduced
runs can be compared by Tick, simulation time and event
Genome, mutation and lineage can be explored
OrganismView can be inspected without affecting behavior
Functional Roles and Behavior Profiles are explainable and versioned
Selection Sets can support repeatable analysis
saved assets can be placed only through recorded Core commands
checkpoints and branches can be created and compared
all derived metrics expose provenance
research reports include reproducibility metadata
UI can answer scientific questions without requiring manual raw-file inspection for normal workflows
```

## Research Detailed Worklogs

Рекомендовані child plans:

```text
UI-3A:
  Experiment Definitions, Queue And Run Groups

UI-3B:
  Checkpoints, Branching And Controlled Interventions

UI-3C:
  Genome, Mutation And Lineage Observatory

UI-3D:
  OrganismView And Deep Entity Exploration

UI-3E:
  Functional Roles, Sensory Specialization And Behavior Profiles

UI-3F:
  Advanced Analytics, Comparisons And Analytical Summary

UI-3G:
  Library, Saved Assets And Placement

UI-3H:
  Research Export, Reproducibility And Final Hardening
```

# Відповідність старому Visual Roadmap

| Попередня частина | Нове місце |
| --- | --- |
| `Visual A: Read-Only Debug Viewer` | `Start` — polished basic Viewer; `Debug` — exact debug layers |
| `Visual B: Scenario Runner UI` | `Start` — demo launch і controls; `Debug` — full scenario workflow |
| `Visual C: Layered World Inspector` | `Debug` |
| `Visual D: World Initialization Editor` | `Debug` |
| `Visual E: Experiment Dashboard` | `Debug` — balance baseline; `Research` — full experiment analytics |
| `Visual F: Genome And Evolution Observatory` | `Research` |
| `Visual G: Organism Inspector And Observatory` | `Research` |
| `Visual H: Library, Save And Placement Center` | `Research` |
| `Visual I: Control Center` | Shell починається в `Start`, розширюється в `Debug`, завершується в `Research` |
| `Visual J: Checkpoints And Branching Experiments` | `Research` |

Попередній roadmap можна залишити як historical worklog.

Цей документ стає актуальним high-level UI implementation plan.

# Core/API Dependencies By Part

## Start

Потрібні мінімально:

```text
scenario list
start / pause / resume / stop / step
set Tick rate
World Frame Projection
selected Cell Detail Projection
basic run summary
basic event stream
config hash
seed
```

## Debug

Додатково:

```text
engine keyframes
exact Resource/Field projection
Resource Detail Projection
Material Detail Projection
Process / rejection projection
contact / physics debug projection
balance summaries
raw event access
scenario validation API
artifact index
diagnostics
```

## Research

Додатково:

```text
Genome Detail Projection
lineage projection
Organism Detail Projection
classification registry
Behavior Profile results
experiment metadata
checkpoint API
branch API
saved asset API
placement API
advanced analytics results
```

UI development не повинна блокуватися очікуванням усіх API.

До готовності API використовуються deterministic fixtures через той самий Projection Gateway.

# Worklog Model

## Parent Plan

Canonical path:

```text
docs/implementation/implementation-plan-ui.md
```

Цей parent plan не повинен перетворюватися на гігантський checkbox-файл.

## Detailed Plans

Перед кожним slice створюється:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-PLAN-ui-<part>-<slice>.md
```

Приклад:

```text
outputs/worklogs/2026-07-XX-HHMM-PLAN-ui-start-fixture-viewer.md
```

Після завершення:

```text
outputs/worklogs/YYYY-MM-DD-HHMM-REPORT-ui-<part>-<slice>.md
```

## Required Detailed Plan Structure

Кожен detailed plan повинен містити:

```text
Goal
Architecture
Dependencies
Files
Task-by-task checklist
Tests before or together with implementation
Commands
Acceptance Gate
Documentation updates
Report requirement
```

Task steps використовують:

```text
- [ ]
```

## Worklog Registration

Нові `PLAN` і `REPORT` додаються до:

```text
outputs/worklogs/index.md
```

## Worklog Scope Rule

Один worklog повинен мати один demoable outcome.

Не потрібно змішувати в одному plan:

```text
renderer
World Editor
analytics
Genome Observatory
Library
```

## Worklog Completion Report

Кожен `REPORT` повинен містити:

- виконані tasks;
- changed files;
- tests;
- commands;
- screenshots, де доречно;
- performance result;
- unresolved issues;
- deviations від plan;
- next recommended slice.

# Cross-Part Quality Requirements

Кожна частина повинна включати, наскільки це відповідає її scope:

```text
Projection version check
Core/UI compatibility check
data correctness checks
command safety
Chromium smoke test
1024×768 check
Light/Dark check
localization key validation
error and empty states
performance smoke
documentation update
phase report
```

Повний test pyramid є бажаним, але застосовується risk-based.

## Release Blocking

Незалежно від частини блокуються:

- incorrect Tick;
- incorrect run або branch;
- data value mismatch;
- duplicate command;
- silently repeated intervention;
- stale data shown as live;
- corrupted export;
- simulation behavior affected by UI setting;
- Critical warning hidden;
- crash у primary demo flow.

# Frozen Decisions

Цей plan фіксує:

```text
three product parts: Start / Debug / Research
local Chromium-based Web Viewer
WebGL2 instanced rendering
WebSocket binary frames
2D simulation with pseudo-3D presentation
shared Viewer
top global navigation
UI text registry
English canonical terms
Light and Dark themes
simulation rate separate from visualization FPS
bounded temporary UI history
engine keyframes for durable history
read-only Viewer by default
Core-approved commands only
```

# Deferred Decisions

В окремих detailed plans можна визначити:

- binary frame codec details;
- exact WebSocket protocol;
- exact design tokens;
- exact package manager and Node runtime pinning;
- exact PixiJS batch, mesh and shader strategy;
- exact asset pipeline;
- exact screenshot comparison tolerances;
- exact FPS adaptation thresholds;
- exact LOD thresholds;
- keyframe cadence;
- exact comparison limits;
- advanced statistics implementation;
- `System` theme support;
- 3D research.

Deferred decision не повинно порушувати UI Canon або accepted technology stack.

Frontend framework, component primitives, charting library and state-management baseline are defined in [[docs/implementation/ui-technology-stack|UI Technology Stack]].

# Recommended Immediate Next Plan

`UI-1A` і `UI-1B` уже виконані як локальні implementation slices.

Historical starting slice:

```text
UI-1A:
Application Shell And Deterministic Fixture Viewer
```

Його minimum outcome:

```text
Chromium application starts
Monitor workspace opens
deterministic fixture loads
WebGL2 Viewer draws World and Cells
one Resource heatmap is visible and Composite Resource Concentration · Smooth 
zoom/pan/full-screen work
Cell can be selected
Inspector shows fixture data
Light/Dark work
screenshot can be exported
```

Visual target for this slice is `docs/ui/control-center-monitor-v3.png`.
It is a product direction reference, not a full implementation contract.
`UI-1A` should take the shell density, Monitor layout, layer panel, world view,
Inspector position, and bottom data-panel structure from this image, but defer
OrganismView detail, rich analytics, warning depth, classification labels, and
live-only metrics until later slices.

Completed follow-up slice:

```text
UI-1B:
Live Projection Transport And Run Controls
```

Current next slice:

```text
UI-1C:
WOW World Rendering, Projection Truthfulness, Semantic Zoom And Cell Inspector
```

Minimum outcome:

```text
live/idle/fixture states are visually distinct
missing live projection fields are explicit, not silently hidden
Cell visual scale does not misrepresent physical radius or overlap
Resource layer is either live data-bound or marked unavailable
semantic zoom starts replacing flat circles with data-bound detail
Inspector exposes live position, radius, lifecycle, Energy, available Resources/Materials and projection provenance
Monitor remains usable at 1024x768
```

Після `UI-1C` створюється:

```text
UI-1D:
Start Demo, Export And Acceptance Hardening
```

Такий порядок дозволяє швидко отримати живий результат, але не закріпити
неточну візуальну інтерпретацію projection data як canonical behavior.

# Global Completion

UI implementation plan виконаний повністю, коли:

```text
Start:
  проєкт можна переконливо показати

Debug:
  Core можна перевіряти й балансувати через UI

Research:
  відтворювані експерименти можна проводити без ручного складання даних із різних файлів
```

# Пов'язані документи

- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/analytics.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`
- `docs/implementation/implementation-phases.md`
- `docs/implementation/architecture.md`
- `docs/engine/technology-stack.md`
- `outputs/worklogs/2026-07-02-1935-PLAN-phase-visual-global-roadmap.md`
- `outputs/worklogs/2026-07-02-1936-PLAN-phase-visual-global-UIUX.md`
- `outputs/worklogs/index.md`

# Semantic Links

- implements: [[docs/ui/README|UI Layer]]
- follows: [[docs/ui/principles|UI Principles]]
- follows architecture: [[docs/ui/architecture|UI Architecture]]
- follows navigation: [[docs/ui/navigation|UI Navigation]]
- implements visualization: [[docs/ui/visualization|UI Visualization]]
- implements analytics: [[docs/ui/analytics|UI Analytics]]
- implements exploration: [[docs/ui/exploration|UI Exploration]]
- implements presentation: [[docs/ui/presentation|UI Presentation]]
- implements interaction: [[docs/ui/interaction|UI Interaction]]
- constrained by: [[docs/ui/quality|UI Quality]]
- aligns with: [[docs/implementation/implementation-phases|Implementation Phases]]
- follows stack: [[docs/engine/technology-stack|Technology Stack]]
- supersedes planning order of: [[outputs/worklogs/2026-07-02-1935-PLAN-phase-visual-global-roadmap|Phase Visual Global Roadmap]]
