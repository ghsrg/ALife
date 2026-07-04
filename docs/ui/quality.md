---
tags:
  - alife
  - ui
  - canon
---

# UI Quality

## Призначення

Цей документ визначає quality model для `ALife Control Center`.

Документ описує:

- підтримувані runtime environments;
- performance profiles;
- rendering degradation;
- visual regression;
- accessibility guidance;
- deterministic test fixtures;
- test strategy;
- connection loss;
- crash recovery;
- diagnostics;
- temporary history memory policy;
- release-blocking defects.

Quality requirements не повинні змінювати simulation semantics.

## Supported Runtime

Canonical target:

```text
Chromium-based runtime
```

Підтримувані варіанти можуть включати:

- Google Chrome;
- Microsoft Edge;
- Chromium-based desktop shell;
- інший сумісний Chromium runtime.

Початкова quality matrix не вимагає:

- Firefox;
- Safari;
- повноцінної mobile browser parity;
- touch-first runtime.

Архітектура не повинна навмисно блокувати інші runtimes, але вони не є release gate на старті.

## Desktop Target

UI є desktop-first.

Мінімальний supported viewport:

```text
1024×768
```

Recommended viewport:

```text
1920×1080
```

UI повинен масштабуватися на більші екрани без руйнування composition.

## Performance Profiles

### Minimum Profile

```text
Viewport:
  1024×768

GPU class:
  integrated GPU рівня Intel UHD 620

Expected presentation:
  Compact UI available
  adaptive LOD
  reduced decorative effects
  limited labels
  bounded overlays

Target visualization FPS:
  approximately 20–30 FPS under load
```

### Recommended Profile

```text
Viewport:
  1920×1080

GPU class:
  integrated GPU рівня Intel Iris Xe або вище

Expected presentation:
  pseudo-3D effects
  richer overlays
  smoother interpolation
  higher label budget

Target visualization FPS:
  up to 60 FPS when practical
```

Visualization FPS не є scientific correctness requirement.

UI може працювати на:

```text
60 FPS
30 FPS
20 FPS
```

без зміни simulation behavior.

## Core And Rendering Independence

Simulation calculation виконується в Core.

Rendering rate не визначає:

- Tick execution;
- Process order;
- physics result;
- Energy accounting;
- Resource accounting;
- event order;
- deterministic outcome.

UI може пропускати visual frames, але не змінює Core Ticks.

## Canonical Benchmarks

Не встановлюється universal hard limit кількості Cells.

Performance вимірюється на versioned benchmark scenarios.

Benchmark description повинна включати:

- World dimensions;
- Cell count;
- OrganismView count;
- active Fields;
- Resource layers;
- overlays;
- labels;
- Semantic Zoom;
- LOD;
- resolution;
- hardware profile;
- browser/runtime version;
- visualization FPS;
- frame time;
- memory usage.

Порівняння performance без однакового scenario та settings не вважається коректним.

## Rendering Degradation

При performance pressure UI знижує presentation cost у такому порядку:

```text
1. decorative effects
2. shadows and pseudo-3D depth
3. interpolation quality
4. non-essential animation
5. visible label count
6. secondary overlays
7. entity rendering detail
8. visualization FPS
```

Дозволяється adaptive reduction до 30 або 20 FPS.

Не дозволяється:

- змінювати simulation Tick rate без user action;
- пропускати Core Ticks;
- змінювати values;
- змінювати entity state;
- приховувати Critical warnings;
- показувати approximate value як exact;
- змінювати Data Context;
- вимикати primary scientific layer без visible indication.

## Adaptive Performance State

При automatic degradation UI показує:

- active performance preset;
- reduced-quality state;
- current visualization FPS;
- disabled effects;
- action to restore quality;
- reason, якщо відомий.

Adaptive degradation не повинна відбуватися мовчки для scientificly significant layers.

## Performance Presets

UI може підтримувати:

```text
Quality
Balanced
Performance
Custom
```

Preset належить до presentation settings.

Preset не змінює:

- simulation config;
- behavior hash;
- Core scheduling semantics;
- analytics values.

## Visual Quality Matrix

Canonical visual checks включають:

```text
Light
Dark

Comfortable
Compact

1024×768
1920×1080

pseudo-3D Viewer
flat Debug Visualization Mode
```

Для stable screens і components можуть використовуватися visual regression tests.

Pixel-perfect comparison не є вимогою для:

- animated simulation frames;
- interpolation;
- non-deterministic rendering timing;
- transient hover state;
- GPU-dependent antialiasing.

Для simulation views перевага надається:

- stable fixture state;
- disabled animation;
- deterministic camera;
- known Projection;
- bounded tolerance.

## Accessibility Guidance

Quality target орієнтується на:

```text
WCAG 2.2 AA
```

для звичайних application components:

- navigation;
- forms;
- dialogs;
- Inspectors;
- tables;
- messages;
- charts;
- controls.

Це є design guidance, а не жорстка вимога першої release version.

Не потрібно окремо вбудовувати повний accessibility subsystem, якщо це суттєво ускладнює стартову реалізацію.

Базові практики бажані:

- keyboard-accessible controls;
- visible focus;
- reasonable contrast;
- non-color encoding;
- readable text;
- reduced motion;
- accessible labels;
- chart data table або summary, якщо це нескладно.

## Viewer Accessibility

Не вимагається screen-reader representation кожної Cell або spatial pixel.

Viewer повинен, наскільки практично, підтримувати:

- keyboard focus selected entity;
- entity search;
- Inspector;
- accessible legend;
- textual summary;
- focus action;
- visible selection;
- non-color distinction для critical states.

## Deterministic Test Fixtures

Рекомендується створити versioned deterministic fixtures.

Canonical scenarios:

```text
small World
large population
Resource starvation
division
death and decomposition
multicellular OrganismView
mutation
lineage emergence
population collapse
placement
intervention
missing data
partial Projection
```

Fixture може містити:

- scenario config;
- seed;
- config hash;
- recorded Projections;
- engine keyframes;
- events;
- expected metrics;
- expected classifications;
- expected warnings;
- screenshots або visual references;
- fixture version.

## Fixture Stability

При зміні fixture потрібно:

- підвищити fixture version;
- зафіксувати причину;
- оновити expected results;
- не переписувати baseline мовчки;
- перевірити, чи змінилася simulation semantics.

## Test Strategy

Бажана test structure:

```text
unit tests
component tests
integration tests
localization validation
accessibility smoke tests
critical visual regression
end-to-end tests
performance benchmarks
```

Це recommended quality model, а не абсолютна вимога для кожного merge.

Команда може застосовувати risk-based gates.

## Unit Tests

Unit tests доцільні для:

- formatting;
- filter logic;
- selection logic;
- state reducers;
- color mapping;
- legend generation;
- classification presentation;
- command serialization;
- error mapping;
- UI state restoration.

## Component Tests

Component tests доцільні для:

- Inspector;
- chart;
- legend;
- confirmation dialog;
- filter panel;
- Data Context indicator;
- warning card;
- Contextual Help;
- run controls;
- entity search.

## Integration Tests

Integration tests доцільні для:

- Projection loading;
- Viewer and Inspector synchronization;
- chart and Viewer cross-filtering;
- keyframe loading;
- placement request;
- intervention rejection;
- reconnection;
- export.

## End-To-End Tests

Critical flows можуть включати:

```text
open run
select Cell
focus in Viewer
inspect historical keyframe
Jump to Live
compare entities
perform placement
receive rejection
create checkpoint
export artifact
recover after reconnect
```

Не всі flows повинні запускатися на кожен commit.

## Merge And Release Gates

Merge gates визначаються risk profile change.

Critical paths бажано перевіряти автоматично.

Performance benchmark, full visual suite і long-running E2E можуть запускатися:

- scheduled;
- перед release;
- для high-risk change;
- вручну за потреби.

Відсутність повного test pyramid не повинна блокувати ранній development prototype, але production-critical flows мають отримувати coverage поступово.

## Localization Validation

Localization checks можуть перевіряти:

- missing keys;
- invalid placeholders;
- broken pluralization;
- invalid locale fallback;
- accidental hardcoded text;
- layout overflow;
- untranslated critical messages.

Missing required key у production-critical flow є release-blocking defect.

## Data Correctness

UI correctness має пріоритет над visual polish.

Повинні перевірятися:

- displayed Tick;
- run id;
- branch id;
- Data Context;
- units;
- scale;
- aggregation;
- sampling;
- classification version;
- metric version;
- raw value;
- rounded value;
- missing-data state.

UI не повинно silently reinterpret data.

## Connection Loss

При втраті зв'язку з Core UI:

```text
preserve last committed Projection
enter read-only state
show Disconnected
block new commands
mark cached data as stale
```

UI не повинно показувати cached state як live.

## Reconnection

Після reconnect UI:

```text
request current Core state
verify active run
verify Data Context
refresh command availability
reconcile displayed Tick
```

Pending command не повторюється автоматично.

UI повинно показати, якщо result попереднього command невідомий.

## Reconnection Conflicts

Після reconnect можливі стани:

```text
command confirmed
command rejected
command status unknown
run advanced
run ended
branch changed
Projection incompatible
```

UI не повинно робити припущення без Core confirmation.

## Crash Recovery

UI може відновлювати:

- останній workspace;
- layout;
- theme;
- locale;
- density;
- UI scale;
- filters;
- recent entities;
- pinned entities;
- draft World Editor state;
- unfinished export parameters.

## Recovered Draft

Unsaved `World Editor` draft може відновлюватися через versioned autosave.

Recovered draft:

- не застосовується автоматично;
- має timestamp;
- має schema version;
- порівнюється із current base config;
- відкривається як draft;
- може бути discarded.

## Non-Restorable State

Не відновлюється як authoritative state:

- temporary frozen snapshot, якщо evicted;
- pending simulation command;
- optimistic entity state;
- unavailable historical frame;
- incomplete placement;
- pointer drag state;
- open transient tooltip.

## Local Diagnostics

Local diagnostics рекомендовано залишати доступними.

Вони можуть містити:

- UI version;
- Core version;
- runtime version;
- OS summary;
- viewport;
- FPS;
- frame time;
- memory estimate;
- failed requests;
- error ids;
- current Data Context metadata;
- active rendering preset;
- disabled effects;
- keyframe load status.

## External Telemetry

External telemetry:

```text
disabled by default
explicit opt-in only
```

Без explicit consent не надсилаються:

- private files;
- arbitrary user content;
- full scenario data;
- research artifacts;
- Genome data;
- exported reports;
- screenshots;
- identifying metadata.

Telemetry preference належить до user settings.

## Diagnostic Export

Користувач може експортувати diagnostic bundle.

Bundle повинен показати склад перед export.

Може містити:

- versions;
- settings;
- error ids;
- performance data;
- sanitized logs;
- Data Context summary.

Не повинен автоматично включати full research dataset.

## Temporary Snapshot Memory

Temporary frozen UI snapshots використовують:

```text
bounded memory budget
+
LRU eviction
```

Пріоритет утримання:

```text
current frozen snapshot
pinned snapshot
recently viewed snapshot
ordinary temporary snapshot
```

Точна кількість snapshots не фіксується Canon.

Обмеження визначається:

- snapshot size;
- memory budget;
- device profile;
- active workspace;
- browser limitations.

## Snapshot Eviction

Engine keyframes не видаляються UI eviction policy.

При eviction UI:

- не показує snapshot як доступний;
- оновлює timeline;
- очищає unavailable Redo target;
- показує reason on demand;
- не підмінює його nearest snapshot мовчки.

Для current або pinned snapshot UI може попередити про memory pressure.

## Memory Pressure

При memory pressure UI спочатку:

```text
1. evicts ordinary temporary snapshots
2. reduces decoded off-screen assets
3. releases inactive chart caches
4. reduces rendering caches
5. lowers visual quality
```

UI не повинно видаляти persistent run artifacts.

## Export Quality

Export повинен перевіряти:

- correct run id;
- Tick або interval;
- branch;
- config hash;
- active filters;
- metric version;
- classifier version;
- unit;
- data completeness;
- file integrity.

Corrupted або semantically incorrect export є critical defect.

## Release-Blocking Defects

Release блокується при:

```text
data value mismatch
incorrect Tick or Data Context
duplicate simulation command
lost or silently repeated intervention
application crash in critical flow
corrupted export
missing required localization key
Critical warning not visible
keyboard trap in critical workflow
simulation behavior affected by UI setting
cached data shown as live
wrong run or branch shown
silent command retry
silent historical-frame substitution
```

## Non-Blocking Defects

Некритичний visual defect може бути released як documented known issue, якщо він:

- не спотворює data;
- не приховує Critical state;
- не блокує core workflow;
- не змінює command;
- не пошкоджує export;
- не створює false scientific interpretation.

## Quality Evidence

Перед release бажано мати:

- test summary;
- known issues;
- supported runtime list;
- benchmark result;
- localization status;
- critical-flow status;
- export validation;
- Core/UI compatibility matrix.

## Compatibility

UI і Core повинні мати version compatibility check.

При incompatible version UI:

- не продовжує command submission мовчки;
- показує compatibility error;
- дозволяє diagnostics;
- може залишити read-only access, якщо Projection format compatible.

## Quality Priorities

Canonical order:

```text
1. data correctness
2. command safety
3. Data Context correctness
4. recoverability
5. usability
6. performance
7. visual polish
```

Visual attractiveness є важливою для успіху продукту, але не може виправдовувати incorrect data або unsafe command behavior.

## Архітектурні обмеження

Заборонено:

- використовувати rendering FPS як simulation clock;
- вимагати 60 FPS для correctness;
- повторювати pending command автоматично після reconnect;
- показувати stale Projection як live;
- зберігати unbounded UI history;
- видаляти engine keyframes через UI LRU;
- надсилати external telemetry без opt-in;
- включати private artifacts у diagnostics мовчки;
- переписувати regression baseline без version change;
- блокувати ранній prototype лише через відсутність повної quality matrix;
- випускати known defect, що спотворює scientific data.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/analytics.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- validates: [[docs/ui/architecture|UI Architecture]]
- validates: [[docs/ui/navigation|UI Navigation]]
- validates: [[docs/ui/visualization|UI Visualization]]
- validates: [[docs/ui/analytics|UI Analytics]]
- validates: [[docs/ui/exploration|UI Exploration]]
- validates: [[docs/ui/presentation|UI Presentation]]
- validates: [[docs/ui/interaction|UI Interaction]]
