# fields_config.md

> **Fields Config — конфігурація просторових впливів середовища**

---

# Призначення

`fields_config.md` описує, як задавати Fields без зміни рушія.

Field — це просторовий вплив середовища, який може змінювати умови для клітин, Resources, Materials і реакцій.

Fields можуть описувати:

* light;
* heat;
* pressure;
* radiation;
* flow;
* turbulence;
* chemical gradient;
* moisture-like influence;
* gravity-like direction;
* local hazard-like zones.

Field не є Resource.

Field не є Material.

Field не є Energy Buffer.

Field — це умова або вплив, який клітини можуть відчувати лише через Materials, Genome inputs і локальну фізику.

---

# Основна ідея

Один і той самий рушій повинен дозволяти різні світи:

```text id="qfop93"
bright shallow world
dark deep world
high-pressure world
radiation-heavy world
stormy world
volcanic world
ice-age world
```

Це задається через Fields.

Наприклад:

```text id="727xwy"
high light field
    ↓
light-sensitive Material can convert Resources / Field influence into Energy

high heat field
    ↓
Materials degrade faster

high pressure field
    ↓
Boundary and Joint damage increases
```

---

# Що Field Config НЕ є

Field Config не повинен описувати:

* готову погоду як поведінку клітин;
* direct damage;
* direct Energy Buffer transfer;
* direct mutation command;
* hardcoded poison;
* hardcoded biome behavior;
* species-specific effects;
* готові екологічні ролі.

Field змінює умови.

Наслідки виникають через Materials, Resources, Energy, physics і Genome Runtime.

---

# Загальна структура

Приклад:

```yaml id="3tg81x"
fields:
  - id: "light"
    name: "Light Field"
    type: "scalar"
    default_value: 0.6
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "gradient"
    temporal_mode: "cyclic"
    diffusion: 0.0
    decay_rate: 0.0

  - id: "heat"
    name: "Heat Field"
    type: "scalar"
    default_value: 0.4
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "dynamic"
    temporal_mode: "dynamic"
    diffusion: 0.15
    decay_rate: 0.01
```

Це приклад формату, а не остаточна схема.

---

# Field Fields

## id

Унікальний технічний ідентифікатор.

```yaml id="q0s383"
id: "light"
```

На нього можуть посилатися:

* `world_config.md`;
* `materials_config.md`;
* `resources_config.md`;
* Genome input bindings;
* climate events;
* weather events;
* catastrophe events;
* debug logs.

---

## name

Людська назва для UI.

```yaml id="rt70wh"
name: "Light Field"
```

Не повинна використовуватися в логіці.

---

## type

Тип значення Field.

```yaml id="gt8laq"
type: "scalar"
```

Можливі типи:

```text id="rjk4ys"
scalar
vector
gradient
mask
```

## scalar

Одне значення в точці простору.

Приклади:

* light;
* heat;
* pressure;
* radiation.

## vector

Напрямлений вплив.

Приклади:

* flow;
* wind-like current;
* gravity-like direction.

## gradient

Field, де важлива локальна різниця значень.

Приклади:

* chemical gradient;
* temperature gradient;
* moisture gradient.

## mask

Зона, де вплив увімкнений або вимкнений.

Приклади:

* volcanic zone;
* shadow zone;
* radiation zone.

---

# Value Range

Fields мають нормалізовані межі.

```yaml id="jt9fr9"
min_value: 0.0
max_value: 1.0
default_value: 0.5
```

Рекомендація для MVP:

```text id="58q0x5"
0.0 = мінімальний вплив
1.0 = максимальний вплив
```

Це спрощує config і Genome inputs.

---

# Spatial Mode

`spatial_mode` описує, як Field розподілений у просторі.

```yaml id="r3youu"
spatial_mode: "uniform"
```

Можливі значення:

```text id="v8li8c"
uniform
gradient
noise
zones
dynamic
from_sources
```

---

## uniform

Однакове значення по всьому світу.

```yaml id="erazaz"
spatial_mode: "uniform"
default_value: 0.5
```

Корисно для простих MVP-світів.

---

## gradient

Значення змінюється в одному напрямку.

```yaml id="u5womu"
spatial_mode: "gradient"
gradient:
  axis: "y"
  start_value: 0.9
  end_value: 0.1
```

Приклад: світло сильніше зверху, слабше знизу.

---

## noise

Значення має шумовий розподіл.

```yaml id="0yg1yr"
spatial_mode: "noise"
noise:
  seed: 101
  scale: 0.05
  smoothing: 0.4
```

Корисно для неоднорідного середовища.

---

## zones

Field задається через зони.

```yaml id="m3py8d"
spatial_mode: "zones"
zones:
  - id: "hot_zone"
    area:
      x: 300
      y: 400
      radius: 150
    value: 0.8

  - id: "cold_zone"
    area:
      x: 900
      y: 200
      radius: 200
    value: 0.2
```

---

## dynamic

Field змінюється через physics, weather, cells або events.

```yaml id="37sw6d"
spatial_mode: "dynamic"
```

Наприклад, Heat може створюватися клітинами і поширюватися.

---

## from_sources

Field створюється джерелами.

```yaml id="72efez"
spatial_mode: "from_sources"
sources:
  - id: "light_source_top"
    position:
      x: 500
      y: 0
    intensity: 1.0
    radius: 1000
    falloff: "linear"
```

---

# Temporal Mode

`temporal_mode` описує зміну Field у часі.

```yaml id="k4lqw7"
temporal_mode: "constant"
```

Можливі значення:

```text id="92r2vn"
constant
cyclic
noise
scheduled
event_driven
dynamic
```

---

## constant

Field не змінюється сам по собі.

```yaml id="fnuj7v"
temporal_mode: "constant"
```

---

## cyclic

Field має цикл.

```yaml id="otdt5r"
temporal_mode: "cyclic"
cycle:
  period_ticks: 10000
  amplitude: 0.25
  phase: 0.0
  curve: "sinusoidal"
```

Приклади:

* day/night;
* seasons;
* periodic heat cycles.

---

## noise

Field випадково коливається.

```yaml id="qcg8bt"
temporal_mode: "noise"
temporal_noise:
  seed: 202
  amplitude: 0.05
  update_interval_ticks: 100
```

---

## scheduled

Field змінюється за розкладом.

```yaml id="kk5zt7"
temporal_mode: "scheduled"
schedule:
  - start_tick: 0
    end_tick: 200000
    value: 0.3

  - start_tick: 200000
    end_tick: 500000
    value: 0.6

  - start_tick: 500000
    end_tick: 1000000
    value: 0.9
```

Корисно для еволюційних епох.

---

## event_driven

Field змінюється через події з `world_config.md`.

```yaml id="5gqos0"
temporal_mode: "event_driven"
```

Наприклад:

```text id="808cl5"
impact_winter
    ↓
light decreases
temperature decreases
radiation increases
```

---

## dynamic

Field оновлюється рушієм через фізику.

Наприклад:

```text id="w60lbx"
cells produce Heat
Heat diffuses
Heat decays
```

---

# Diffusion

Деякі Fields можуть поширюватися.

```yaml id="xhh8q9"
diffusion: 0.15
```

Diffusion означає, що локальні значення вирівнюються між сусідніми клітинками простору.

Для MVP:

```text id="f6z0ql"
0.0 = не поширюється
1.0 = дуже швидке вирівнювання
```

---

# Decay

Field може згасати.

```yaml id="7nkb9o"
decay_rate: 0.01
```

Decay особливо важливий для:

* Heat;
* radiation spikes;
* local chemical-like influence;
* pressure waves;
* temporary weather effects.

---

# Falloff

Для sources або zones можна задавати falloff.

```yaml id="qexk3t"
falloff: "linear"
```

Можливі значення:

```text id="4a3f1w"
none
linear
exponential
inverse_square
custom
```

Для MVP достатньо:

```text id="svut4z"
none
linear
```

---

# Interaction with Materials

Fields впливають на клітини через Materials.

Наприклад:

```yaml id="is2gq1"
material_effects:
  - material_id: "light_pigment_A"
    field_id: "light"
    effect: "energy_conversion_modifier"
    multiplier: 0.8
```

Але краще основну чутливість Material задавати в `materials_config.md`:

```yaml id="zaucnd"
field_sensitivity:
  light: 0.7
  heat: 0.2
```

Field Config описує Field.

Material Config описує, як Material на нього реагує.

---

# Interaction with Resources

Fields можуть змінювати поведінку Resources.

Приклади:

```yaml id="9rrrw0"
resource_effects:
  - resource_id: "unstable_nutrient_A"
    field_id: "heat"
    decay_multiplier: 2.0

  - resource_id: "trace_signal_A"
    field_id: "light"
    decay_multiplier: 1.5
```

Основні Resource-specific правила краще тримати в `resources_config.md`.

Field Config лише задає доступні Field і їхню динаміку.

---

# Interaction with Cells

Клітини можуть читати Field лише через локальні inputs.

```text id="icm1rw"
Field value at cell position
    ↓
Material sensitivity
    ↓
Genome Runtime input
```

Клітина не повинна читати глобальну карту Field.

Вона бачить лише локальні значення або локальні gradients, якщо має відповідні Materials.

---

# Light Field

Light Field може бути джерелом можливості для Energy conversion.

```yaml id="r7uo4h"
- id: "light"
  type: "scalar"
  default_value: 0.7
  min_value: 0.0
  max_value: 1.0
  spatial_mode: "gradient"
  gradient:
    axis: "y"
    start_value: 1.0
    end_value: 0.1
  temporal_mode: "cyclic"
  cycle:
    period_ticks: 10000
    amplitude: 0.2
```

Light не створює Energy напряму.

Потрібні light-sensitive Materials і процес Energy conversion.

---

# Heat Field

Heat Field описує локальне тепло.

```yaml id="e7hc81"
- id: "heat"
  type: "scalar"
  default_value: 0.4
  min_value: 0.0
  max_value: 1.0
  spatial_mode: "dynamic"
  temporal_mode: "dynamic"
  diffusion: 0.20
  decay_rate: 0.01
```

Heat може:

* пошкоджувати Materials;
* прискорювати reactions;
* змінювати Resource decay;
* впливати на dormancy;
* передаватися через Joint або середовище.

Heat не є Energy Buffer.

---

# Pressure Field

Pressure Field описує локальне стискання або механічний вплив.

```yaml id="ly5zep"
- id: "pressure"
  type: "scalar"
  default_value: 0.3
  min_value: 0.0
  max_value: 1.0
  spatial_mode: "gradient"
  gradient:
    axis: "y"
    start_value: 0.1
    end_value: 0.8
  temporal_mode: "constant"
```

Pressure може впливати на:

* Boundary damage;
* Joint damage;
* movement;
* deformation;
* pressure-sensitive Materials;
* specialization.

---

# Radiation Field

Radiation Field може підвищувати degradation або mutation risk.

```yaml id="1aadkn"
- id: "radiation"
  type: "scalar"
  default_value: 0.05
  min_value: 0.0
  max_value: 1.0
  spatial_mode: "noise"
  temporal_mode: "scheduled"
  schedule:
    - start_tick: 0
      end_tick: 200000
      value: 0.30
    - start_tick: 200000
      end_tick: 1000000
      value: 0.05
```

Radiation не повинна напряму “покращувати” або “псувати” Genome командою.

Вона може збільшувати ймовірність damage або mutation через відповідні механізми.

---

# Flow Field

Flow Field — напрямлений вплив середовища.

```yaml id="3f9m7i"
- id: "flow"
  type: "vector"
  default_value:
    x: 0.05
    y: 0.0
  spatial_mode: "noise"
  temporal_mode: "cyclic"
  cycle:
    period_ticks: 5000
    amplitude: 0.10
```

Flow може впливати на:

* movement;
* diffusion Resources;
* transport of dead remains;
* trace movement;
* collision;
* organism shape.

---

# Turbulence Field

Turbulence може бути шумовим механічним впливом.

```yaml id="pvysw7"
- id: "turbulence"
  type: "scalar"
  default_value: 0.05
  spatial_mode: "noise"
  temporal_mode: "noise"
  temporal_noise:
    amplitude: 0.10
    update_interval_ticks: 200
```

Turbulence може створювати нестабільність середовища без hardcoded storms.

---

# Moisture-like Field

Moisture-like Field може задавати умови для diffusive або drying worlds.

```yaml id="iunxm4"
- id: "moisture"
  type: "scalar"
  default_value: 0.6
  spatial_mode: "zones"
  temporal_mode: "cyclic"
```

Це не обов'язково реальна вода.

Це узагальнений параметр середовища, який впливає на diffusion, decay або Materials.

---

# MVP Field Set

Для MVP достатньо 4–5 Fields:

```yaml id="yhhu6d"
fields:
  - id: "light"
  - id: "heat"
  - id: "pressure"
  - id: "flow"
  - id: "radiation"
```

Мінімальні поля для кожного Field:

```yaml id="7e679d"
id: "light"
type: "scalar"
default_value: 0.5
min_value: 0.0
max_value: 1.0
spatial_mode: "uniform"
temporal_mode: "constant"
diffusion: 0.0
decay_rate: 0.0
```

---

# Example Early Earth-like Fields

```yaml id="x1pjjw"
fields:
  - id: "light"
    type: "scalar"
    default_value: 0.35
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "gradient"
    gradient:
      axis: "y"
      start_value: 0.8
      end_value: 0.1
    temporal_mode: "cyclic"
    cycle:
      period_ticks: 10000
      amplitude: 0.15

  - id: "heat"
    type: "scalar"
    default_value: 0.65
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "zones"
    zones:
      - id: "volcanic_zone"
        area:
          x: 400
          y: 500
          radius: 180
        value: 0.9
    temporal_mode: "noise"
    temporal_noise:
      amplitude: 0.08
      update_interval_ticks: 500
    diffusion: 0.15
    decay_rate: 0.005

  - id: "pressure"
    type: "scalar"
    default_value: 0.4
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "gradient"
    gradient:
      axis: "y"
      start_value: 0.2
      end_value: 0.7
    temporal_mode: "constant"

  - id: "radiation"
    type: "scalar"
    default_value: 0.25
    min_value: 0.0
    max_value: 1.0
    spatial_mode: "noise"
    noise:
      seed: 912
      scale: 0.08
    temporal_mode: "scheduled"
    schedule:
      - start_tick: 0
        end_tick: 300000
        value: 0.25
      - start_tick: 300000
        end_tick: 1000000
        value: 0.08

  - id: "flow"
    type: "vector"
    default_value:
      x: 0.02
      y: 0.00
    spatial_mode: "noise"
    temporal_mode: "cyclic"
    cycle:
      period_ticks: 20000
      amplitude: 0.05
```

---

# World Config Integration

`world_config.md` може змінювати Fields через:

* climate;
* weather;
* seasons;
* epochs;
* catastrophes;
* gradual changes.

Приклад:

```yaml id="n777xc"
field_schedule:
  - field_id: "light"
    start_tick: 0
    end_tick: 300000
    intensity: 0.3

  - field_id: "light"
    start_tick: 300000
    end_tick: 1000000
    intensity: 0.8
```

`fields_config.md` описує, що таке `light`.

`world_config.md` описує, як `light` змінюється у конкретному сценарії.

---

# Validation

Field Config має проходити validation.

Перевірки:

* `id` унікальний;
* `type` належить до дозволених значень;
* `default_value` у межах `min_value..max_value`;
* `min_value <= max_value`;
* `spatial_mode` відомий;
* `temporal_mode` відомий;
* `diffusion >= 0`;
* `decay_rate >= 0`;
* schedule ranges валідні;
* zones мають валідну area;
* referenced field_id існує;
* vector fields мають коректні `x`, `y`;
* немає NaN або infinite values.

---

# Правила

## Rule 1. Fields are config-defined

Нові Fields додаються через config, а не через зміну engine.

## Rule 2. Field is not Resource

Field не має amount, не займає cell storage і не є сировиною.

## Rule 3. Field is not Material

Field не виконує функції клітини.

Materials визначають, як клітина реагує на Field.

## Rule 4. Field is not Energy Buffer

Light або Heat не є прямим Energy Buffer клітини.

## Rule 5. Cells read only local Field values

Клітина не бачить глобальну карту Field.

## Rule 6. World scenarios modify Fields

Погода, клімат, епохи і катастрофи мають змінювати Fields через config.

---

# Заборонено

Не вводити:

* field as direct cell command;
* field as direct kill switch;
* field as direct Energy Buffer;
* species-specific field;
* hardcoded Earth climate;
* weather that bypasses Materials;
* radiation as guaranteed useful mutation;
* pressure as direct HP damage;
* light as automatic Energy.

---

# Пов'язані документи

* `config/world_config.md`
* `config/resources_config.md`
* `config/materials_config.md`
* `world/fields.md`
* `world/resources.md`
* `world/materials.md`
* `world/energy.md`
* `world/physics.md`
* `world/tick.md`
* `biology/cell.md`
* `biology/processes.md`

---

# Open Questions

## Field model

Потрібно вирішити, чи Field зберігається як grid layer, continuous function або hybrid.

## MVP spatial modes

Потрібно вибрати мінімальні `spatial_mode` для MVP:

```text id="721ivh"
uniform
gradient
zones
dynamic
```

## MVP temporal modes

Потрібно вибрати мінімальні `temporal_mode` для MVP:

```text id="m6z1zp"
constant
cyclic
scheduled
dynamic
```

## Heat implementation

Потрібно вирішити, чи Heat є Field, окремим physics layer або обома.

## Pressure implementation

Потрібно вирішити, чи Pressure є Field, результатом physics, чи окремим derived layer.

## Flow implementation

Потрібно визначити, чи Flow впливає на Resources, cells, traces або все разом.

## Field interaction

Потрібно вирішити, чи Fields можуть напряму впливати один на одного.

## Config schema

Потрібно створити формальну schema validation для Fields.
