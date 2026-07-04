---
tags:
  - alife
  - ui
  - canon
---

# UI Analytics

## Призначення

Цей документ визначає аналітичні подання `ALife Control Center`.

Документ описує:

- структуру workspace `Analysis`;
- canonical dashboards;
- часовий контекст;
- Resource, Material та Energy flows;
- population distributions;
- Organism size analysis;
- Functional Cell Roles;
- sensory specialization;
- Behavior Profiles;
- run comparison;
- warnings;
- discoveries;
- Analytical Summary;
- cadence обчислень;
- balance accounting;
- raw data access.

Аналітика використовує лише committed data, events, summaries, Projections та derived analytics results.

Аналітика не є simulation authority.

## Структура Analysis Workspace

Canonical sections:

```text
Overview
Flows
Population
Specialization
Balance
Comparison
Events & Warnings
Raw Data
```

### Overview

Показує:

- active Data Context;
- current або selected time context;
- population;
- living, dead і dormant counts;
- total Resources;
- total Materials;
- total Energy;
- births;
- deaths;
- divisions;
- dominant lineages;
- dominant Functional Cell Roles;
- dominant Behavior Profiles;
- critical warnings;
- significant discoveries;
- recent Analytical Summary.

### Flows

Показує:

- Resource movement;
- Resource conversion;
- Material synthesis;
- Material degradation;
- Energy production;
- Energy spending;
- Heat;
- explicit sinks;
- rate charts;
- cycle diagrams;
- detailed accounting.

### Population

Показує:

- Cell count;
- OrganismView count;
- lifecycle distribution;
- age;
- lifespan;
- generation;
- Organism size distribution;
- lineage distribution;
- spatial density;
- births and deaths over time.

### Specialization

Показує:

- Potential Functional Roles;
- Observed Functional Roles;
- sensory specialization;
- Behavior Profiles;
- multi-label composition;
- role distribution by Organism size;
- role distribution by lineage;
- role distribution by Genome;
- appearance and disappearance of specializations.

### Balance

Показує:

- conservation;
- input;
- stored amount;
- conversion;
- release;
- explicit sinks;
- unaccounted difference;
- accumulation;
- depletion;
- collapse risk;
- starvation cascade;
- excessive dormancy;
- numerical drift.

### Comparison

Показує:

- run vs run;
- seed vs seed;
- control vs intervention;
- branch vs branch;
- interval vs interval;
- event-relative comparison;
- aggregate comparison across run sets.

### Events & Warnings

Показує:

- events;
- warnings;
- discoveries;
- anomaly summaries;
- collapse causes;
- related entities;
- related intervals;
- links to Viewer та detail views.

### Raw Data

Показує:

- metric tables;
- event tables;
- entity tables;
- logs;
- search;
- filters;
- sorting;
- exports.

## Canonical Dashboards

UI використовує curated canonical dashboards.

Користувач може:

- reorder widgets;
- resize widgets;
- hide widgets;
- pin widgets;
- save personal view;
- restore canonical layout.

Користувач не повинен змінювати calculation semantics через layout customization.

Custom view є UI preference і не є simulation artifact.

## Time Context

Усі аналітичні views працюють у явному спільному часовому контексті.

Canonical modes:

```text
Current Snapshot
Selected Tick
Selected Interval
Whole Run
Live Window
Compare Intervals
```

Зміна Time Context оновлює, якщо підтримується:

- charts;
- Viewer;
- Inspector;
- tables;
- Derived Classification;
- warnings;
- discoveries;
- Analytical Summary.

UI повинен показувати:

- selected Tick;
- selected interval;
- current run;
- live або historical state;
- data completeness;
- last calculated Tick.

## Compare Intervals

UI може порівнювати два часові інтервали одного run.

Приклад:

```text
Tick 10 000–20 000
vs
Tick 40 000–50 000
```

Comparison показує:

- absolute difference;
- relative difference;
- direction of change;
- statistical summary, якщо доступний;
- affected entities;
- related events.

## Matter And Energy Separation

Resource, Material та Energy не повинні об'єднуватися в одну accounting model.

### Matter Cycle

Matter Cycle може включати:

```text
Resources in World
Resources in Cells
Materials in living Cells
Materials in dead or decomposing Cells
MaterialFragments
released Resources
explicit sinks
```

Matter Cycle повинен зберігати distinction між:

- Resource;
- Material;
- MaterialFragment;
- dead Cell;
- released Resource.

### Energy Flow

Energy Flow може включати:

```text
Energy production
Energy Buffer
mandatory upkeep
movement
growth
repair
division
other Process spending
Heat
explicit loss
```

Energy не показується як речовина.

Matter Cycle та Energy Flow можуть бути поруч у `Overview`, але мають окремі legends, units і accounting.

## Flow Presentation Levels

Для flows підтримуються три рівні представлення.

### Snapshot

Використовує:

- cycle diagram;
- Sankey;
- state composition chart.

Показує стан або потоки для конкретного Tick чи interval.

### Time Evolution

Використовує:

- stacked area;
- rate chart;
- input/output chart;
- cumulative chart.

Показує зміну в часі.

### Exact Accounting

Використовує:

- flow table;
- source/target breakdown;
- category totals;
- explicit sink rows;
- unaccounted difference.

Кожне visual summary повинно мати перехід до exact accounting, якщо data доступні.

## Resource And Material Accounting

Для кожного Resource та Material показуються:

```text
Input
Stored
Converted
Released
Explicit Sink
Unaccounted Difference
```

`Unaccounted Difference`:

- показується завжди;
- не приховується через округлення;
- має unit;
- має threshold status;
- відкриває calculation details;
- може мати tolerance, але не маскується як zero.

## Energy Accounting

Для Energy показуються:

```text
Produced
Stored
Spent by category
Converted to Heat
Explicit Loss
Unaccounted Difference
```

Energy spending categories повинні відповідати registered Processes або canonical accounting categories.

UI не повинен мовчки об'єднувати невідомі витрати в `Other` без можливості деталізації.

## Organism Size Distribution

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

Bins зберігаються у versioned analytics configuration.

Користувач може вибрати іншу predefined grouping або custom grouping, якщо implementation це підтримує.

Canonical Y-axis modes:

```text
Organism count
% of Organisms
Cell count
% of Cells
```

## Visible Bin Range

Histogram показує bins:

```text
від першого bin
до останнього bin, у якому є data
```

Порожній хвіст після останнього populated bin приховується.

Мінімально показуються п'ять bins.

Якщо populated range містить менше п'яти bins, UI додає наступні empty bins до мінімуму п'яти, якщо вони існують у configuration.

Приклади:

```text
data only in bins 1 and 2
  -> show first 5 bins

last data in bin 20–49
  -> show bins from 1 through 20–49

no data above 100–199
  -> hide bins 200–499, 500–999 and 1000+
```

Якщо data відсутні повністю, UI показує honest empty state, а не порожній chart без пояснення.

## Functional Cell Role Registry

UI не hardcode-ить повний перелік Functional Cell Roles.

Roles надходять із:

- classification registry;
- analytics Projection;
- classifier metadata.

Базові ролі можуть включати:

```text
contractile-like
signal-processing-like
sensory
structural
transport
metabolic
storage
repair
protective
reproductive
undifferentiated
mixed-function
```

Нові зареєстровані roles повинні з'являтися в UI без зміни chart implementation.

Для кожної role доступні:

- canonical id;
- localized label;
- description;
- classification source;
- confidence;
- classifier version;
- Potential або Observed context;
- visual encoding.

## Sensory Specialization

Sensory specialization є окремою classification dimension.

Приклад:

```text
Functional Cell Role:
  sensory

Sensory Specialization:
  pressure-sensitive

Secondary Functional Cell Role:
  signal-processing-like
```

Sensory subtypes надходять із registry або analytics Projection.

Базові subtypes можуть включати:

```text
light-sensitive
temperature-sensitive
pressure-sensitive
chemical-sensitive
resource-gradient-sensitive
damage-sensitive
contact-sensitive
signal-sensitive
mixed sensory
```

UI не припускає, що цей перелік є повним.

## Multi-Label Classification Modes

Для Functional Cell Roles, sensory specialization та Behavior Profiles підтримуються:

```text
Primary only
All matched labels
Fractional contribution
```

Default:

```text
Primary only
```

### Primary Only

Entity враховується лише у primary classification.

### All Matched Labels

Entity враховується в кожній classification, що пройшла configured threshold.

UI повинен попереджати, що totals можуть перевищувати entity count.

### Fractional Contribution

Entity розподіляє contribution між кількома labels.

Приклад:

```text
Cell:
  60% contractile-like
  30% sensory
  10% structural
```

Сума fractional contributions повинна бути пояснена classifier method або normalization rule.

## Counting Modes

Для selected role або subtype підтримуються:

```text
Organisms containing at least one selected Cell
Total selected Cells
Average selected Cells per Organism
Percentage of selected Cells inside Organism
Dominant-role Organisms
```

Active counting mode завжди показується в chart title, legend або header.

UI не повинен порівнювати різні counting modes як однакову metric.

## Role And Organism Size Views

Для analysis `Organism size × Functional Cell Role` підтримуються:

- stacked bars;
- grouped bars;
- heatmap;
- distribution curve;
- table.

Canonical dimensions:

```text
X:
  Organism size bin

Y or segment:
  Functional Cell Role or Sensory Specialization

Value:
  selected counting mode
```

Chart selection може:

- highlight matching OrganismView у Viewer;
- open aggregated Inspector;
- show matching entities;
- pin selection for comparison.

## Run Comparison

Canonical alignment modes:

```text
By Tick
By simulation time
By selected event
```

### By Tick

Порівнює однакові Tick values.

### By Simulation Time

Порівнює однаковий simulation time, якщо Tick duration або scheduling differs.

### By Selected Event

Вирівнює runs відносно визначеної event type або конкретної event occurrence.

Приклад:

```text
1000 Ticks before collapse
collapse
1000 Ticks after collapse
```

UI повинен показувати alignment method.

Якщо event відсутня у run, UI не повинен вигадувати alignment point.

## Detailed Comparison Size

Detailed visual comparison використовує bounded small set of runs.

Точний limit є configurable UI setting або chart capability.

Більші набори використовують:

- aggregate statistics;
- distribution across runs;
- seed grouping;
- summary tables;
- outlier detection;
- representative samples.

UI не повинен перевантажувати chart необмеженою кількістю series.

## Statistical Analysis

Advanced statistical views можуть включати:

```text
mean
median
min/max
percentiles
variance
standard deviation
confidence intervals
distribution across seeds
```

Ці functions є бажаними, але не є обов'язковими для базової UI implementation, якщо їх реалізація суттєво ускладнює першу версію.

Для multi-seed experiments UI не повинен покладатися лише на mean, якщо distribution істотно відрізняється.

До появи advanced statistics UI повинен щонайменше показувати individual runs або базовий range.

## Warnings

Warning означає потенційну проблему.

Приклади:

- population collapse;
- starvation cascade;
- unbounded Resource accumulation;
- unbounded Material accumulation;
- unbounded Energy accumulation;
- conservation mismatch;
- excessive dormancy;
- numerical drift;
- unstable oscillation;
- missing analytics data;
- failed classifier;
- invalid run artifact.

Warning містить:

- type;
- severity;
- Tick або interval;
- explanation;
- affected metrics;
- affected entities;
- confidence, якщо warning derived;
- evidence;
- related chart;
- action `Show in Viewer`, якщо є spatial context.

## Discoveries

Discovery означає значущу спостережену подію або pattern, а не проблему.

Приклади:

- new lineage became dominant;
- first multicellular OrganismView appeared;
- new Functional Cell Role appeared;
- new Behavior Profile emerged;
- new sensory subtype appeared;
- long-lived stable lineage formed;
- new maximum Organism size reached.

Discovery містить:

- type;
- Tick або interval;
- explanation;
- affected entities;
- confidence;
- evidence;
- related chart;
- action `Show in Viewer`, якщо є spatial context.

Warning та Discovery повинні мати різне visual encoding.

## Analytical Summary

`Analytical Summary` — observer-generated короткий опис значущих змін у selected Data Context.

Приклади:

```text
Population declined by 34% after Resource A depletion.

Lineage L-17 became dominant during the selected interval.

Pressure-sensitive Cells first appeared at Tick 84 220.
```

Analytical Summary:

- базується лише на metrics, events та derived analytics;
- має посилання на evidence;
- показує time context;
- не видає correlation за доведену causation;
- позначає probable cause окремо від observed fact;
- може мати confidence;
- не є authoritative scientific conclusion;
- може бути regenerated для іншого interval;
- може експортуватися разом із report.

## Analytics Cadence

Analytics не зобов'язана перераховувати всі metrics кожен Tick.

Canonical cadence categories:

```text
lightweight live metrics:
  every rendered frame or frequent interval

aggregated metrics:
  every configured N Ticks

Derived Classification:
  every configured N Ticks or on demand

heavy comparison:
  on demand or post-run
```

Для кожного result UI показує:

- last calculated Tick;
- calculation interval;
- live, cached, calculating, partial або unavailable state;
- classifier або metric version;
- input Data Context.

Cadence configuration не змінює simulation behavior.

## Live Analytics

Live analytics повинна:

- не блокувати Tick execution;
- не змінювати scheduling semantics;
- знижувати own refresh rate при performance pressure;
- явно показувати delayed state;
- не показувати cached value як current без indication.

UI rendering FPS, analytics cadence та simulation Tick rate є різними величинами.

## Cross-Filtering

Chart, Viewer, Inspector та tables мають бути пов'язані, якщо data мають спільну entity або spatial identity.

Приклади:

```text
click role segment
  -> highlight matching Cells or OrganismView

select OrganismView
  -> highlight its size bin and role composition

select warning
  -> open related interval and entities
```

Cross-filtering:

- не змінює simulation state;
- показує active filter;
- дозволяє clear selection;
- не застосовує hidden filter;
- зберігає compatible Data Context.

## Metric Provenance

Для кожної derived metric доступні:

- canonical metric id;
- definition;
- unit;
- calculation method;
- aggregation;
- Time Context;
- data completeness;
- metric version;
- source Projection;
- known limitations.

Для simple raw values достатньо короткого hint.

Для derived metrics, classifications і composite scores потрібна повна методика.

## Chart Requirements

Кожен chart повинен мати:

- title;
- metric;
- unit;
- Time Context;
- legend;
- scale type;
- missing-data encoding;
- aggregation state;
- data state;
- export action;
- contextual help.

Chart не повинен покладатися лише на color.

Для accessibility має бути доступний textual summary або data table.

## Raw Data

`Raw Data` підтримує:

- table або grid;
- search;
- filters;
- sorting;
- column selection;
- copy value;
- open entity;
- show in Viewer;
- CSV export;
- JSON export.

Для великих datasets використовуються:

- pagination;
- virtualization;
- bounded queries;
- analytics-side filtering;
- progressive loading.

UI не підтримує arbitrary user-defined calculated columns у базовій analytics architecture.

Calculated expressions можуть бути додані пізніше як окремий validated analytics expression layer.

## Export

Analytics export може включати:

- chart image;
- chart data;
- dashboard snapshot;
- CSV;
- JSON;
- run comparison report;
- balance report;
- warning list;
- discovery list;
- Analytical Summary;
- selected Data Context metadata.

Export повинен містити:

- run ids;
- seed;
- config hash;
- Tick або interval;
- metric versions;
- classifier versions;
- aggregation;
- sampling;
- data completeness.

## Honest Empty And Partial States

UI розрізняє:

```text
zero
no data
not calculated
calculating
partial
sampled
unavailable
unsupported
```

Відсутність data не показується як zero.

Chart із partial data має visible indication.

## Архітектурні обмеження

Заборонено:

- змішувати Matter Cycle та Energy Flow як одну conserved substance;
- hardcode-ити повний перелік Functional Cell Roles;
- приховувати classification mode;
- показувати multi-label totals як entity count без пояснення;
- приховувати empty histogram tail всередині populated range;
- показувати менше п'яти configured bins, якщо bins існують;
- вигадувати event alignment;
- видавати Analytical Summary за доведену causation;
- показувати cached analytics як live;
- приховувати Unaccounted Difference;
- використовувати arbitrary calculated columns без validated expression layer;
- змінювати simulation behavior через analytics cadence.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- uses workspace from: [[docs/ui/architecture|UI Architecture]]
- uses navigation from: [[docs/ui/navigation|UI Navigation]]
- links to spatial data in: [[docs/ui/visualization|UI Visualization]]
- drives inspection in: [[docs/ui/exploration|UI Exploration]]
- uses text and themes from: [[docs/ui/presentation|UI Presentation]]
- delegates chart gestures to: [[docs/ui/interaction|UI Interaction]]
- constrained by: [[docs/ui/quality|UI Quality]]
