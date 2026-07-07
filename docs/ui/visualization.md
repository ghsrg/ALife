---
tags:
  - alife
  - ui
  - canon
---

# UI Visualization

## Призначення

Цей документ визначає правила візуалізації `World`, `Cells`, `OrganismView`, `Resources`, `Materials`, `Fields`, events та observer-side analytics у `Viewer`.

Документ описує:

- просторову модель;
- viewport scaling;
- 2D та pseudo-3D presentation;
- Semantic Zoom;
- LOD;
- Cell та OrganismView rendering;
- heatmaps;
- color modes;
- overlays;
- labels;
- aggregation;
- interpolation;
- trajectories;
- spatial events;
- dead matter;
- legends;
- screenshots;
- debug visualization.

`Viewer` є read-only presentation layer і не є simulation authority.

## Базова просторова модель

Поточна canonical модель `Viewer`:

```text
2D top-down World
```

Simulation entities існують у simulation coordinates.

`Viewer` відображає їх через незалежний world-to-screen transform.

Поточна 2D-модель не повинна блокувати майбутню 3D-візуалізацію, якщо проєкт виросте до просторової 3D simulation model.

3D не є поточною вимогою.

## Pseudo-3D Presentation

2D World може використовувати pseudo-3D presentation.

Дозволені:

- perspective-like depth;
- shadows;
- layered surfaces;
- subtle parallax;
- volumetric-looking gradients;
- depth cues;
- highlights;
- material-like surfaces;
- pseudo-3D Cell rendering;
- pseudo-3D Cell preview в Inspector;
- pseudo-3D saved asset preview.

Pseudo-3D:

- не змінює simulation coordinates;
- не створює вигадану фізичну вісь;
- не повинно приховувати spatial relationships;
- не повинно змінювати apparent entity position без зрозумілого projection rule;
- не повинно імітувати internal structures, яких немає у Projection;
- може вимикатися через rendering settings, якщо це потрібно для performance або accessibility.

Scientific debug views можуть використовувати flat 2D rendering незалежно від selected visual style.

## World Bounds And Aspect Ratio

`Viewer` зберігає aspect ratio `World`.

Якщо пропорції `Viewport` та `World` відрізняються:

```text
preserve aspect ratio
+
use letterboxing or unused surrounding space
```

Заборонено розтягувати World незалежно по X та Y лише для заповнення Viewport.

Користувач може:

- zoom;
- pan;
- manually crop visible area;
- fit full World;
- focus selected entity;
- reset Viewport.

Простір поза World bounds повинен візуально відрізнятися від World.

## Initial Viewport State

При відкритті нового `Data Context`:

```text
fit World to Viewport
```

При переході між workspaces із тим самим run і compatible spatial context UI зберігає:

- zoom;
- pan;
- focus;
- selection;
- active spatial layers.

При переході на інший run або несумісний World:

```text
fit World to Viewport
```

Користувач може явно зберегти або відновити Viewport state як UI preference або saved view.

Viewport state не є simulation artifact.

## Semantic Zoom

Semantic Zoom змінює не лише screen-space size, а й смисловий рівень доступної інформації.

Canonical рівні:

```text
Overview
Entity
Structure
Internal Detail
```

Точні screen-space thresholds є configurable implementation detail.

### Overview

Показує:

- World bounds;
- population density;
- aggregated entity markers;
- large spatial patterns;
- primary Field або color metric;
- major warnings;
- selected population summaries.

Individual Cells можуть бути агреговані.

### Entity

Показує:

- окремі Cells;
- physical radius;
- lifecycle state;
- selection;
- basic movement indicator;
- basic contact indicator;
- simple OrganismView grouping;
- selected labels.

### Structure

Показує:

- Boundary;
- damage;
- dominant active Process;
- OrganismView structure;
- Joints;
- Resource summary;
- Material summary;
- division readiness;
- local movement;
- contact directions.

### Internal Detail

Показує:

- Material proportions;
- Resource proportions;
- Energy state;
- active Processes;
- detailed contacts;
- forces;
- damage;
- repair;
- signal projections;
- Resource flows;
- internal analytical overlays supported by Projection.

Semantic Zoom не створює simulation data.

## LOD

LOD визначає rendering complexity відповідно до:

- screen-space size;
- visible population;
- Viewport density;
- selected entities;
- rendering quality;
- target FPS;
- hardware limits.

Співвідношення:

```text
Semantic Zoom:
  визначає, який зміст доречний на поточному масштабі

LOD:
  визначає, наскільки детально цей зміст можна відрендерити
```

LOD може спрощувати presentation, але не може змінювати data values або simulation result.

Selected, pinned та tracked entities можуть отримувати вищий LOD за навколишні entities.

LOD transitions повинні:

- уникати flicker;
- використовувати hysteresis або smooth transition, якщо доречно;
- зберігати entity identity;
- не змінювати primary color meaning;
- не створювати псевдоподії.

## Cell Size And Interaction Area

Visual physical representation Cell відповідає simulation radius.

Для дуже малих Cells дозволяється окрема interaction area:

```text
physical representation:
  simulation radius

interaction area:
  minimum screen-space hit target
```

Interaction area може бути більшою за видиму Cell.

Вона не повинна створювати враження, що Cell фізично більша.

Для selection може використовуватися halo, outline або marker поза physical boundary.

## Cell Rendering

Cell rendering може використовувати:

- circle;
- segmented disc;
- layered membrane;
- radial composition;
- internal regions;
- pseudo-3D body;
- rings;
- wedges;
- bars;
- icons;
- directional overlays.

Rendering не повинен вигадувати anatomical structures.

Усі internal elements повинні відповідати:

- Projection;
- registered Material;
- registered Resource;
- Process state;
- Field interaction;
- observer-side classification.

## OrganismView Rendering

`OrganismView` є observer-side projection.

UI не визначає membership самостійно.

Semantic Zoom для OrganismView:

```text
far:
  aggregated organism marker or outline

medium:
  organism boundary and member Cell distribution

close:
  individual Cells and Joints

very close:
  per-Cell Resources, Materials, Processes and flows
```

Режими outline:

```text
selected only
all visible
off
```

Default:

```text
selected only
```

При `Analysis Level = Organisms` користувач може ввімкнути outlines для всіх видимих OrganismView.

## Heatmaps

Heatmaps використовуються для:

- Resources;
- Fields;
- Environment values;
- density;
- selected analytics projections.

Підтримуються два canonical режими:

```text
Exact
Smooth
```

### Exact

Показує фактичну grid або sample structure без interpolation.

Використовується для:

- debugging;
- scientific validation;
- exact local values;
- grid-bound analysis.

### Smooth

Використовує interpolation для показу continuous spatial pattern.

Smooth mode:

- не створює нові simulation values;
- повинен бути явно позначений як interpolated;
- не використовується як джерело exact readings;
- може мати configurable interpolation method.

Inspector і tooltip повинні показувати actual sampled value, а не лише visual interpolation result.

## Composite Resource Concentration

Primary World mode for displaying several Resource types in one spatial view.

```text
hue        = dominant or blended Resource types
brightness = total Resource concentration
saturation = dominance of the leading Resource
texture    = Resource gradient or flow
```

Default Overview presentation:

```text
Composite Resource Concentration · Smooth
```

Rules:

- visualizes Resources, not Materials;
- low concentrations fade into the neutral background;
- `Smooth` interpolation does not create simulation values;
- Inspector and tooltip show actual sampled values;
- texture may represent bounded gradient, diffusion or configured flow;
- Fields require separate explicitly enabled overlays;
- Resource colors and blending rules are versioned and reproducible;
- active Resources, units, scale, normalization and interpolation mode appear in the legend;
- `Exact` mode shows the original grid, sparse-grid or sample structure;
- decorative particles must not be confused with Resource data;
- visualization does not affect Resources, diffusion or simulation behavior.

## Default Visualization State

Default Cell color mode:

```text
lifecycle state
```

Default World background:

```text
neutral
no active heatmap
```

Scenario може рекомендувати primary Resource або Field, але UI не вмикає його мовчки без visible indication.

Default visualization не повинна приховувати significant entities.

## Color Configuration

Default semantic colors зберігаються у versioned UI color configuration.

Конфігурація може визначати:

- lifecycle colors;
- selection colors;
- warning colors;
- comparison colors;
- Resource palettes;
- Material palettes;
- Field palettes;
- event colors;
- dead matter colors;
- missing-data colors;
- light theme variants;
- dark theme variants;
- color-blind-safe variants.

Color configuration:

- належить до UI presentation;
- не входить у simulation config;
- не змінює data meaning;
- повинна мати stable semantic keys;
- може бути перевизначена theme або accessibility preset;
- повинна проходити contrast та distinguishability tests.

Одна semantic category не повинна без пояснення змінювати meaning між themes.

## Primary Mode And Overlays

У Viewer одночасно використовується:

```text
one primary Field or Cell color mode
+
several simple overlays
```

Не встановлюється жорстка universal кількість overlays.

UI повинен:

- попереджати про visual conflict;
- не дозволяти indistinguishable encodings;
- автоматично формувати legend;
- давати дію `Clear overlays`;
- показувати active overlays;
- дозволяти швидко вимикати secondary layers.

Overlays не повинні приховувати primary spatial data.

## Labels

Default labels показуються для:

- hovered entity;
- selected entity;
- pinned entity;
- tracked entity;
- comparison entity.

Для інших entities labels з'являються:

- при достатньому zoom;
- за явним user setting;
- у filtered subset;
- у debug mode.

При великій population UI використовує:

- label collision avoidance;
- priority rules;
- bounded label count;
- aggregation;
- fade або hide.

Canonical ID повинна залишатися доступною в Inspector навіть якщо label у Viewer прихована.

## Aggregation

При далекому zoom або великій population Cells можуть агрегуватися у screen-space clusters.

Cluster показує щонайменше:

- entity count;
- density;
- lifecycle composition;
- selected або dominant metric;
- data completeness;
- aggregation level.

Cluster interaction підтримує обидві дії:

```text
inspect cluster
zoom to cluster
```

Конкретні gestures описуються в `interaction.md`.

Aggregated marker не повинен виглядати як реальна Cell або OrganismView.

## Snapshot Interpolation

Viewer може інтерполювати position між committed snapshots для плавного movement.

Interpolation:

- є presentation-only;
- не є simulation state;
- не створює Tick;
- не використовується для analytics;
- не використовується для exact event timing;
- вимикається у step/debug mode;
- може бути вимкнена користувачем;
- повинна зберігати committed positions на snapshot boundaries.

Якщо snapshots пропущені або data incomplete, Viewer не повинен вигадувати безперервну trajectory без позначення.

## Trajectories

Default trajectories показуються для:

- selected;
- pinned;
- tracked entities.

Дозволяється показувати trajectories для filtered subset.

Trajectory має:

- bounded history;
- visible time interval;
- optional sampling;
- direction;
- clear start/end context;
- data completeness state.

Не дозволяється показувати full unbounded trajectory для всієї population без aggregation або sampling.

## Spatial Events

Events із spatial location можуть відображатися у Viewer.

Приклади:

- birth;
- division;
- death;
- mutation;
- contact event;
- damage;
- repair;
- Resource depletion;
- manual placement.

Event marker:

- прив'язаний до фактичної location;
- має Tick або timestamp;
- може бути вимкнений;
- відкриває event details;
- не плутається з persistent entity;
- не замінюється декоративною animation.

Mutation показується spatially лише якщо вона пов'язана з конкретною Cell, inheritance event або birth event.

## Living, Dead And Decomposing Matter

Viewer повинен візуально розрізняти:

```text
living Cell
dead Cell
decomposing Cell
MaterialFragment
released Resource
```

Ці стани не повинні використовувати indistinguishable visual encoding.

Entity зникає з Viewer лише тоді, коли її більше немає у Projection.

Dead або decomposing entity не повинна автоматично виглядати як released Resource до explicit conversion.

## Selection Hierarchy

Canonical selection hierarchy:

```text
hover:
  subtle highlight

selected:
  strong outline or halo

pinned:
  persistent marker

tracked:
  selected marker plus tracking indicator

comparison:
  distinct indexed marker
```

Selection highlight не змінює primary color metric сутності.

Comparison markers повинні залишатися розрізнюваними у light, dark і color-accessible themes.

## Background And Visual Style

Canonical visual direction:

```text
neutral scientific background
+
subtle game-like depth
```

Дозволені:

- subtle coordinate grid;
- light texture;
- boundary glow;
- subtle Field animation;
- pseudo-3D depth;
- decorative background outside World bounds;
- controlled atmospheric effects;
- smooth visual transitions.

Заборонені:

- decorative particles усередині World, які можна сплутати з Resource, Cell або event;
- persistent noise, що приховує Fields;
- pseudo-events;
- animation, що змінює apparent data values;
- background effects, що погіршують scientific readability.

Decorative visual effects повинні мати on/off setting.

## Legend

Кожен active primary mode та overlay має legend.

Legend показує:

- metric або Field;
- unit;
- scale;
- min/max;
- linear, logarithmic або diverging scale;
- interpolation mode;
- missing-data encoding;
- data state;
- aggregation або sampling state, якщо застосовано.

Legend може бути collapsed.

Вона не повинна зникати без явної user action, якщо visual encoding без неї неоднозначне.

## Tooltip Values

Tooltip або Inspector для spatial value повинні показувати:

- actual sampled value;
- coordinates;
- Tick або interval;
- unit;
- data state;
- interpolation status;
- aggregation status.

Visual smooth color не є заміною exact value.

## Screenshot And Export

Viewer підтримує export:

- current Viewport;
- full World;
- selected layers;
- high-resolution image;
- image with legend;
- image without UI chrome.

Screenshot artifact повинен мати доступну metadata:

- run id;
- branch id, якщо є;
- Tick;
- interval, якщо є;
- `Data Context`;
- active layers;
- primary color mode;
- overlays;
- Viewport bounds;
- theme;
- timestamp;
- Projection version.

Metadata може зберігатися:

- у sidecar file;
- в artifact manifest;
- у embedded metadata, якщо формат підтримує.

Metadata не обов'язково друкувати на самому image.

## Debug Visualization Mode

UI має окремий `Debug Visualization Mode`.

Він може показувати:

- SpatialIndex cells;
- collision bounds;
- raw vectors;
- grid coordinates;
- projection ids;
- Feasibility rejections;
- snapshot boundaries;
- raw contacts;
- entity bounds;
- aggregation regions;
- LOD level;
- interpolation state;
- missing Projection fields;
- frame timing.

Debug mode:

- не є default observation mode;
- чітко позначається;
- може використовувати flat 2D rendering;
- не змінює simulation state;
- не змінює simulation priority;
- може знижувати rendering performance;
- не повинен змішувати debug overlays з scientific export без visible indication.

## 3D Future Compatibility

Майбутня 3D-візуалізація може бути додана, якщо core simulation model підтримуватиме 3D Space.

3D implementation повинна зберегти:

- Observer Boundary;
- deterministic entity identity;
- exact coordinate interpretation;
- Analysis Levels;
- Data Context;
- selection semantics;
- legends;
- accessibility alternatives;
- screenshot metadata;
- scientific transparency.

Pseudo-3D presentation поточної 2D simulation не вважається 3D simulation.

## Архітектурні обмеження

Заборонено:

- використовувати window size як simulation dimensions;
- змінювати simulation behavior через zoom;
- змінювати Tick execution через visibility;
- вигадувати internal structures;
- показувати smooth interpolation як exact data;
- змішувати dead matter та released Resource;
- приховувати active color scale;
- використовувати decorative effect як simulation event;
- змінювати physical Cell radius лише для clickability;
- створювати різні spatial semantics у різних workspaces;
- використовувати UI classification як authoritative entity type.

## Приклад

Visual reference:

[[docs/ui/control-center-monitor-v3.png|Control Center Monitor v3]]

Це цільова композиція `Monitor`: application shell, layer controls, central Viewer, contextual Inspector і нижні data panels.

Reference визначає бажаний напрямок композиції та visual density, але не є pixel-perfect specification. Реалізація повинна слідувати UI Canon, responsive rules, data-bound rendering і performance constraints.

## Пов'язані документи

- `GLOSSARY.md`
- `docs/ui/README.md`
- `docs/ui/principles.md`
- `docs/ui/architecture.md`
- `docs/ui/navigation.md`
- `docs/ui/analytics.md`
- `docs/ui/exploration.md`
- `docs/ui/presentation.md`
- `docs/ui/interaction.md`
- `docs/ui/quality.md`

# Semantic Links

- indexed by: [[docs/ui/README|UI Layer]]
- governed by: [[docs/ui/principles|UI Principles]]
- uses architecture from: [[docs/ui/architecture|UI Architecture]]
- uses navigation from: [[docs/ui/navigation|UI Navigation]]
- provides spatial views for: [[docs/ui/analytics|UI Analytics]]
- provides selection context for: [[docs/ui/exploration|UI Exploration]]
- uses themes from: [[docs/ui/presentation|UI Presentation]]
- delegates gestures to: [[docs/ui/interaction|UI Interaction]]
- constrained by: [[docs/ui/quality|UI Quality]]
