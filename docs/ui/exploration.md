---
tags:
  - alife
  - ui
  - canon
---

# UI Exploration

## Призначення

Цей документ визначає правила контекстного дослідження сутностей і підмножин даних у `ALife Control Center`.

Документ описує:

- Inspector;
- full detail View;
- selection;
- filters;
- multi-selection;
- rectangular spatial selection;
- Selection Set;
- pinning;
- comparison;
- relations;
- history;
- Functional Cell Roles;
- spatial actions;
- filter presets;
- classification thresholds;
- context-loss handling.

Exploration працює лише з `Projection`, committed data, events та derived analytics results.

Exploration не є simulation authority.

## Inspector Framework

UI використовує єдиний Inspector framework для різних типів сутностей.

Canonical section model:

```text
Identity
Current State
Composition
Processes / Activity
Relations
History
Analytics
Events
Actions
```

Не всі sections є обов'язковими для кожного entity type.

Inspector показує лише sections, для яких доступні data або meaningful action.

## Cell Inspector

Cell Inspector може містити:

```text
Identity
Physical State
Lifecycle
Energy
Resources
Materials
Capabilities
Processes
Genome
Functional Roles
Relations
History
Events
Actions
```

Мінімально доступні:

- canonical Cell id;
- run id;
- Data Context;
- position;
- radius;
- lifecycle;
- Energy;
- damage;
- Resources;
- Materials;
- current Processes;
- Genome id;
- lineage id;
- OrganismView membership, якщо доступне;
- birth Tick;
- age;
- generation.

## OrganismView Inspector

OrganismView Inspector може містити:

- canonical OrganismView id;
- member Cells;
- Cell count;
- structure;
- Joints;
- center;
- bounding area;
- Energy;
- Resources;
- Materials;
- Genome composition;
- lineage composition;
- Potential Functional Roles;
- Observed Functional Roles;
- Behavior Profiles;
- age;
- history;
- related events;
- collapse state.

`OrganismView` залишається observer-side projection.

Inspector не перетворює його на authoritative simulation entity.

## Genome And Lineage Inspectors

Genome Inspector може містити:

- Genome id;
- version;
- parent Genome;
- mutation history;
- carriers;
- descendants;
- lineage;
- generation;
- regulation summary;
- associated roles;
- associated Behavior Profiles;
- population history.

Lineage Inspector може містити:

- lineage id;
- origin;
- parent lineage;
- descendant lineages;
- active population;
- historical population;
- Genome distribution;
- spatial distribution;
- role distribution;
- Behavior Profile distribution;
- important events;
- extinction state.

## Resource, Material And Field Inspectors

Resource Inspector може містити:

- canonical Resource type;
- local amount;
- World total;
- distribution;
- regeneration;
- consumption;
- release;
- top consumers;
- top producers;
- history;
- accounting.

Material Inspector може містити:

- canonical Material type;
- amount in living Cells;
- amount in dead or decomposing Cells;
- amount in MaterialFragments;
- synthesized amount;
- consumed amount;
- degraded amount;
- distribution;
- related Capabilities;
- related Processes.

Field Inspector може містити:

- canonical Field type;
- local value;
- distribution;
- source;
- propagation;
- decay;
- affected entities;
- sampling mode;
- history.

## Process Inspector

Process Inspector може містити:

- canonical Process id;
- execution count;
- success count;
- rejection count;
- RejectionReasons;
- Energy cost;
- Resource cost;
- Material cost;
- affected entities;
- recent executions;
- time distribution;
- classifier or analytics links.

## Data Context Binding

Inspector завжди прив'язаний до active `Data Context`.

Inspector показує сутність у конкретному:

- run;
- Tick;
- interval;
- checkpoint;
- branch;
- recorded frame.

Якщо timeline переміщено на historical Tick, Inspector показує historical state.

Live state не підміняє historical state мовчки.

Для повернення до live state використовується явна дія:

```text
Jump to Live
```

Inspector header повинен показувати active time context.

## Read-Only By Default

Inspector є read-only за замовчуванням.

Доступні actions можуть включати:

- focus;
- track;
- pin;
- compare;
- open full View;
- show relations;
- show events;
- export;
- save as asset;
- submit approved command.

Inspector не дозволяє напряму редагувати:

- Energy;
- Resources;
- Materials;
- Genome;
- lifecycle state;
- Derived Classification;
- Process state;
- World position.

Будь-яка simulation-changing action проходить approved command workflow.

## Full Detail View

Inspector дає швидкий context.

Full detail View використовується для глибокого дослідження.

Full View може містити:

- розширені sections;
- charts;
- tables;
- relation graph;
- history;
- classification explanation;
- export;
- comparison;
- linked Viewer context.

Full View зберігає canonical entity id і Data Context.

## Filter Display Modes

Для non-matching entities підтримуються три display modes:

```text
Highlight
Hide
Isolate
```

### Highlight

Matching entities показуються яскраво.

Non-matching entities залишаються видимими, але приглушуються.

Default:

```text
Highlight
```

### Hide

Non-matching entities не відображаються.

UI повинен показувати, що active filter приховує entities.

### Isolate

Показується selected subset і необхідний spatial або relational context.

UI повинен пояснювати, який context збережено поза subset.

## Filter Logic

Canonical filter logic:

```text
between different dimensions:
  AND

inside one dimension:
  OR
```

Приклад:

```text
lineage = A OR B
AND
lifecycle = living
AND
Functional Cell Role = sensory OR signal-processing-like
```

UI повинен показувати filter expression у зрозумілому вигляді.

Складні filter groups можуть бути додані пізніше.

## Global And Local Filters

### Global Filters

Можуть зберігатися між compatible workspaces:

- run;
- Tick або interval;
- lineage;
- Genome;
- lifecycle;
- entity type;
- Environment zone;
- selected Resource;
- selected Material;
- Functional Cell Role;
- Behavior Profile.

### Local Filters

Діють лише в конкретному view:

- chart series;
- table columns;
- Inspector history type;
- local confidence threshold;
- selected event type;
- view-specific aggregation.

Active filters завжди видимі через:

- chips;
- summary;
- filter panel;
- active-count indicator.

Hidden filters заборонені.

## Selection Modes

UI підтримує:

```text
single selection
multi-selection
rectangular spatial selection
filtered selection
```

### Single Selection

Визначає primary selected entity.

### Multi-Selection

Дозволяє вибрати кілька entities для:

- highlight;
- pinning;
- comparison;
- bulk observer-side actions;
- export id list.

### Rectangular Spatial Selection

Canonical spatial selection mode:

```text
rectangle
```

Rectangle selection:

- працює у Viewer;
- використовує screen-space drag;
- перетворюється на world-space bounds;
- показує кількість selected entities;
- не змінює World;
- може створити temporary selection subset;
- враховує active filters;
- дозволяє include або exclude aggregated clusters лише через явне правило.

Lasso та radius selection не є вимогою Canon.

## Selection Set

`Selection Set` — іменована або temporary група entities, відібрана:

- вручну;
- filter;
- rectangular spatial selection;
- chart selection;
- analytical query.

Selection Set може:

- підсвічуватися;
- аналізуватися;
- порівнюватися;
- експортувати список canonical ids;
- зберігатися як UI artifact;
- повторно застосовуватися до compatible Data Context;
- використовуватися в scientific analysis.

Selection Set:

- не є simulation entity;
- не є behavior input;
- не змінює priority entities;
- має зберігати selection method;
- має зберігати Data Context;
- має показувати missing або unavailable members.

Повна підтримка persisted Selection Set належить до наукової фази UI та не є обов'язковою для першої базової реалізації.

Базова реалізація повинна підтримувати temporary multi-selection.

## Pinning

Canonical model:

```text
selected:
  one active primary entity

pinned:
  entity remains available after new selections

comparison set:
  bounded set of pinned entities selected for comparison
```

Pinned entity може:

- залишатися видимою у Viewer;
- мати persistent marker;
- відображатися у compact pinned panel;
- відкриватися з іншого workspace;
- бути доданою до comparison.

Якщо pinned entity відсутня в selected Tick:

```text
Not present at this Tick
```

UI не прибирає її мовчки.

## Comparison Set Size

Comparison використовує bounded small set.

Точний limit залежить від:

- entity type;
- screen size;
- comparison view;
- chart capability.

Якщо set завеликий, UI:

- пропонує скоротити set;
- переходить до aggregated comparison;
- створює summary table;
- не перевантажує detail comparison.

## Comparison Structure

Comparison показує:

- common attributes;
- different attributes;
- missing attributes;
- absolute difference;
- relative difference;
- Data Context;
- classification differences;
- history comparison;
- relation differences;
- data completeness.

Повне comparison дозволене для compatible entity types:

```text
Cell vs Cell
Genome vs Genome
OrganismView vs OrganismView
lineage vs lineage
run vs run
```

Для incompatible entity types UI показує relation view або limited common concepts.

Приклад:

```text
Cell vs Genome
  relation and shared identifiers
  not full attribute comparison
```

## Relations

Inspector і full View дозволяють переходити по relations.

### Cell Relations

- parent;
- offspring;
- Genome;
- lineage;
- OrganismView;
- Joints;
- contacts;
- recent interacting entities.

### Genome Relations

- parent Genome;
- mutation events;
- carriers;
- descendants;
- lineage;
- related Behavior Profiles.

### OrganismView Relations

- member Cells;
- Joints;
- Genome composition;
- lineages;
- predecessor або successor views, якщо визначено;
- related events.

### Run Relations

- parent checkpoint;
- branch;
- control run;
- intervention run;
- experiment;
- artifacts.

Relation presentation залежить від nature relation:

```text
list
graph
tree
spatial highlight
timeline
```

## Inspector History

Inspector використовує три рівні history:

```text
summary
recent events
full history
```

### Summary

Bounded trends і key changes.

### Recent Events

Bounded list of recent relevant events.

### Full History

On-demand analytics query або full View.

Inspector не завантажує повну історію автоматично.

Для history показуються:

- interval;
- sampling;
- aggregation;
- data completeness;
- last available Tick.

## Functional Cell Roles

Cell Inspector показує окремо:

```text
Potential Functional Roles
Observed Functional Roles
```

Для кожної classification доступні:

- primary role;
- secondary roles;
- confidence;
- contributing components;
- classifier version;
- time interval;
- classification mode.

Potential та Observed roles не об'єднуються в одну role без пояснення.

## Behavior Profile

Behavior Profile Inspector може показувати:

- primary profile;
- secondary profiles;
- confidence;
- contributing components;
- time interval;
- classifier version;
- related Processes;
- related Resources;
- related interactions;
- comparison with previous interval.

Behavior Profile не є hardcoded entity type.

## Spatial Actions

Для entity із spatial identity доступні:

```text
Show in Viewer
Focus
Track
Show trajectory
Show neighbors
Show local Fields
Show related events
```

Для non-spatial entity:

```text
Genome
lineage
Behavior Profile
Functional Cell Role
```

`Show in Viewer` підсвічує:

- carriers;
- members;
- descendants;
- matching entities;
- related spatial events.

UI повинен пояснювати, що саме підсвічено.

## Saved Filter Presets

Користувач може зберігати filter presets.

Приклади:

```text
Living sensory Cells
Lineage A in hot zones
Large Organisms with contractile-like Cells
```

Preset містить:

- filter dimensions;
- values;
- classification mode;
- confidence thresholds;
- display mode;
- compatible Analysis Level;
- optional visualization settings.

Filter preset:

- є UI artifact;
- не є scenario config;
- не є behavior input;
- може бути exported або shared;
- повинен перевіряти compatibility з Data Context.

## Classification Filters

Для classifications користувач може задавати:

- minimum confidence;
- Primary only;
- All matched labels;
- Fractional contribution;
- Potential;
- Observed;
- classifier version, якщо доступно.

Якщо threshold або mode відрізняється від default, UI повинен це явно показувати.

UI не повинен мовчки змінювати classification threshold між views.

## Context Loss Handling

Якщо selected entity більше не доступна, UI не очищає selection мовчки.

Можливі причини:

```text
Entity did not exist at this Tick
Entity excluded by active filter
Entity unavailable in this Projection
Entity belongs to another run
Entity was aggregated
Entity was removed after degradation
```

UI показує причину та можливі actions:

- show birth Tick;
- show death Tick;
- clear conflicting filter;
- return to original run;
- change Data Context;
- inspect aggregated cluster;
- open last available state.

## Search And Recent History Integration

Entity можна відкрити через:

- Viewer;
- search;
- recent history;
- relations;
- chart;
- event;
- warning;
- discovery;
- comparison.

Усі входи повинні вести до одного canonical Inspector і full View.

Canonical ID має бути видимою незалежно від entry point.

## Export

Exploration export може включати:

- entity summary;
- entity state;
- history;
- relation graph;
- comparison report;
- Selection Set ids;
- active filters;
- Data Context;
- classification details.

Export повинен містити достатню metadata для відтворення context.

## Honest Missing States

UI розрізняє:

```text
attribute is zero
attribute is absent
attribute is not supported
attribute is not calculated
entity is unavailable
entity did not exist
```

Missing value не показується як zero.

## Архітектурні обмеження

Заборонено:

- редагувати authoritative entity state через Inspector;
- приховувати active filters;
- очищати unavailable selection мовчки;
- об'єднувати Potential і Observed roles без пояснення;
- використовувати Selection Set як simulation input;
- завантажувати unbounded history автоматично;
- порівнювати incompatible entities як повністю equivalent;
- приховувати canonical ID;
- змінювати classification threshold без visible indication;
- використовувати pinned entity для simulation priority;
- використовувати spatial selection для mutation of World.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/visualization.md`
- `docs/ui/analytics.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- uses workspace model from: [[docs/ui/architecture|UI Architecture]]
- uses navigation from: [[docs/ui/navigation|UI Navigation]]
- connects to: [[docs/ui/visualization|UI Visualization]]
- connects to: [[docs/ui/analytics|UI Analytics]]
- uses text and themes from: [[docs/ui/presentation|UI Presentation]]
- delegates gestures to: [[docs/ui/interaction|UI Interaction]]
- constrained by: [[docs/ui/quality|UI Quality]]
