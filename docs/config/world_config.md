# world_config.md

> **World Config — конфігурація світу, клімату, епох і глобальних сценаріїв**

---

# Призначення

`world_config.md` описує, як задавати світ симуляції без зміни коду рушія.

World Config визначає:

* розмір і форму світу;
* базові фізичні параметри;
* часовий масштаб;
* кліматичні режими;
* погоду;
* сезони;
* катастрофи;
* довгі еволюційні епохи;
* глобальні зміни середовища;
* сценарії для експериментів.

World Config не описує конкретні Resources, Materials або Fields детально.

Для цього є окремі файли:

```text id="n6avm9"
resources_config.md
materials_config.md
fields_config.md
```

---

# Основна ідея

Один і той самий engine повинен запускати різні світи:

```text id="hwj7ug"
stable warm pond
cold ocean world
volcanic world
dry desert world
seasonal planet
ice age scenario
mass extinction scenario
early Earth-like evolution
```

Рушій не повинен містити hardcoded “Землю”.

Землеподібна еволюція має бути лише одним із сценаріїв.

---

# Що World Config НЕ є

World Config не є:

* кодом рушія;
* списком видів;
* готовою екосистемою;
* hardcoded історією Землі;
* фіксованим набором рослин/тварин;
* скриптом поведінки клітин;
* способом напряму керувати еволюцією;
* способом гарантувати появу організмів.

World Config лише задає умови.

Життя, адаптація і еволюційні результати мають виникати з симуляції.

---

# Загальна структура

Рекомендована структура:

```yaml id="pzyvx0"
world:
  id: "early_earth_like"
  seed: 12345
  duration_ticks: 1000000

space:
  type: "2d"
  width: 1000
  height: 1000
  boundary: "wrapped"

time:
  tick_duration: 1.0
  max_ticks: 1000000

climate:
  base_temperature: 0.45
  temperature_variation: 0.15
  humidity: 0.60
  pressure: 0.50
  light_level: 0.70

weather:
  enabled: true
  update_interval_ticks: 100
  noise_strength: 0.1

seasons:
  enabled: true
  period_ticks: 10000

epochs:
  enabled: true
  schedule: []

catastrophes:
  enabled: true
  events: []
```

Це приклад структури, а не остаточний формат.

---

# World

Блок `world` задає загальні параметри симуляції.

```yaml id="tvfdwm"
world:
  id: "test_world"
  name: "Warm Pond"
  seed: 42
  duration_ticks: 500000
  description: "Stable warm world for early evolution experiments."
```

Поля:

```text id="hzot4n"
id              technical world id
name            readable name
seed            deterministic random seed
duration_ticks  intended simulation length
description     human-readable explanation
```

---

# Space

Блок `space` задає геометрію світу.

```yaml id="m8ivn5"
space:
  type: "2d"
  width: 2000
  height: 1000
  boundary: "wrapped"
  spatial_grid_size: 10
```

Можливі `boundary`:

```text id="rgq54d"
wrapped
solid_wall
open
custom
```

## wrapped

Край світу з'єднаний із протилежним краєм.

Корисно для стабільних експериментів.

## solid_wall

Клітини і Resources не виходять за межі.

Корисно для pond-like або container-like сценаріїв.

## open

Об'єкти можуть виходити або втрачатися за межами.

Корисно для flow-like середовищ.

---

# Time

Блок `time` задає часову рамку.

```yaml id="fy4ahv"
time:
  tick_duration: 1.0
  max_ticks: 1000000
  snapshot_interval: 1000
  statistics_interval: 100
```

Поля:

```text id="b4dako"
tick_duration        умовний масштаб одного Tick
max_ticks            максимальна довжина симуляції
snapshot_interval    як часто зберігати стан світу
statistics_interval  як часто рахувати метрики
```

`tick_duration` не повинен бути прив'язаний до реальних секунд.

Це масштаб симуляції.

---

# Climate

Блок `climate` задає базовий стан середовища.

```yaml id="y7bshf"
climate:
  base_temperature: 0.45
  humidity: 0.60
  pressure: 0.50
  light_level: 0.70
  radiation_level: 0.05
  turbulence: 0.10
```

Усі значення бажано нормалізувати:

```text id="hzg7lv"
0.0 = мінімум
1.0 = максимум
```

Це спрощує конфігурацію.

Конкретну фізичну інтерпретацію можна деталізувати в `fields_config.md`.

---

# Climate Zones

Світ може мати кліматичні зони.

```yaml id="lbn36b"
climate_zones:
  - id: "warm_shallow_zone"
    area:
      x_min: 0
      x_max: 700
      y_min: 0
      y_max: 1000
    temperature: 0.65
    light_level: 0.80
    humidity: 0.70

  - id: "cold_deep_zone"
    area:
      x_min: 700
      x_max: 2000
      y_min: 0
      y_max: 1000
    temperature: 0.25
    light_level: 0.20
    humidity: 0.90
```

Climate zones дозволяють створювати різні ecological niches.

---

# Weather

Weather — це короткострокові зміни середовища.

```yaml id="u8oqus"
weather:
  enabled: true
  update_interval_ticks: 100
  temperature_noise: 0.05
  light_noise: 0.10
  pressure_noise: 0.05
  humidity_noise: 0.08
```

Weather не повинен напряму змінювати клітини.

Він змінює Fields або умови середовища.

Клітини реагують через власні Materials і Genome Runtime.

---

# Weather Events

Можна задавати типи погодних подій.

```yaml id="vmultl"
weather_events:
  - id: "heat_wave"
    probability_per_tick: 0.0001
    duration_ticks:
      min: 200
      max: 2000
    effects:
      temperature_delta: 0.20
      humidity_delta: -0.10

  - id: "storm"
    probability_per_tick: 0.00005
    duration_ticks:
      min: 100
      max: 800
    effects:
      pressure_delta: 0.20
      turbulence_delta: 0.30
      light_delta: -0.20
```

Weather events мають бути локальними або глобальними залежно від конфігурації.

---

# Seasons

Seasons — це регулярні довгострокові цикли.

```yaml id="uw6s49"
seasons:
  enabled: true
  period_ticks: 20000
  curve: "sinusoidal"
  effects:
    temperature_amplitude: 0.20
    light_amplitude: 0.25
    humidity_amplitude: 0.10
```

Seasons можуть створювати selection pressure:

* dormancy;
* storage;
* heat resistance;
* cold resistance;
* timing of reproduction;
* resource accumulation.

---

# Epochs

Epochs — це довгі фази розвитку світу.

Вони потрібні, щоб сценарієм прокручувати великі еволюційні зміни.

```yaml id="q0aowc"
epochs:
  enabled: true
  schedule:
    - id: "hot_chemical_ocean"
      start_tick: 0
      end_tick: 200000
      climate:
        base_temperature: 0.75
        light_level: 0.40
        radiation_level: 0.30

    - id: "cooling_stable_ocean"
      start_tick: 200000
      end_tick: 600000
      climate:
        base_temperature: 0.50
        light_level: 0.60
        radiation_level: 0.12

    - id: "high_light_surface_world"
      start_tick: 600000
      end_tick: 1000000
      climate:
        base_temperature: 0.45
        light_level: 0.85
        radiation_level: 0.05
```

Epochs не створюють життя напряму.

Вони лише змінюють умови.

---

# Gradual Changes

Глобальні зміни можуть бути плавними.

```yaml id="r45hqb"
gradual_changes:
  - id: "long_term_cooling"
    start_tick: 0
    end_tick: 500000
    target:
      climate.base_temperature: 0.35
    curve: "linear"
```

Можливі `curve`:

```text id="3mfeqe"
linear
step
sinusoidal
exponential
noise
custom
```

Плавні зміни корисні для еволюційного тиску.

---

# Catastrophes

Catastrophes — це різкі події, які змінюють середовище.

```yaml id="kwz4y1"
catastrophes:
  enabled: true
  events:
    - id: "global_heat_spike"
      trigger_tick: 300000
      duration_ticks: 5000
      effects:
        temperature_delta: 0.40
        radiation_delta: 0.20
        resource_decay_multiplier: 2.0

    - id: "impact_winter"
      trigger_tick: 700000
      duration_ticks: 50000
      effects:
        light_delta: -0.70
        temperature_delta: -0.30
        turbulence_delta: 0.20
```

Catastrophe не повинна напряму вбивати всі клітини.

Вона змінює умови, а смерть виникає через фізику, Energy, Materials і Resources.

---

# Local Catastrophes

Катастрофи можуть бути локальними.

```yaml id="sml04t"
local_catastrophes:
  - id: "volcanic_zone"
    trigger_tick: 150000
    area:
      x: 500
      y: 500
      radius: 150
    duration_ticks: 20000
    effects:
      temperature_delta: 0.50
      new_resource_sources:
        - "reactive_mineral_A"
      radiation_delta: 0.10
```

Локальні катастрофи створюють міграцію, вимирання, bottleneck і нові niches.

---

# Resource Source Schedule

World Config може задавати появу або зміну джерел ресурсів, але самі типи ресурсів описуються в `resources_config.md`.

```yaml id="v8ej2p"
resource_sources:
  - resource_id: "resource_A"
    area:
      x: 200
      y: 300
      radius: 100
    rate: 0.5
    start_tick: 0
    end_tick: 500000

  - resource_id: "resource_B"
    area:
      x: 1200
      y: 600
      radius: 200
    rate: 0.2
    start_tick: 300000
    end_tick: null
```

Це дозволяє створювати епохи з різним хімічним середовищем.

---

# Field Schedule

World Config може задавати зміни Fields, але самі Field definitions описуються в `fields_config.md`.

```yaml id="gtqfrl"
field_schedule:
  - field_id: "light"
    start_tick: 0
    end_tick: 300000
    intensity: 0.30

  - field_id: "light"
    start_tick: 300000
    end_tick: 1000000
    intensity: 0.80
```

Так можна моделювати перехід до світу, де світло стає важливим джерелом енергії.

---

# Scenario

Scenario — це послідовність епох і подій.

```yaml id="xxybid"
scenario:
  id: "earth_like_long_run"
  description: "Long run with cooling, increasing light, oxygen-like shift and catastrophes."
  stages:
    - epoch: "hot_chemical_ocean"
    - epoch: "cooling_stable_ocean"
    - event: "global_heat_spike"
    - epoch: "high_light_surface_world"
    - event: "impact_winter"
```

Scenario не повинен містити конкретних організмів.

Він описує тільки умови світу.

---

# Earth-like Scenario

Приклад грубого сценарію “прокрутити еволюцію Землі” без hardcode Землі:

```yaml id="uc94fp"
scenario:
  id: "earth_like_evolution"
  stages:
    - id: "hot_early_world"
      start_tick: 0
      end_tick: 200000
      climate:
        base_temperature: 0.80
        radiation_level: 0.35
        light_level: 0.35
      notes: "unstable hot early environment"

    - id: "stable_ocean_phase"
      start_tick: 200000
      end_tick: 500000
      climate:
        base_temperature: 0.55
        radiation_level: 0.15
        light_level: 0.50
      notes: "more stable environment"

    - id: "light_energy_opportunity"
      start_tick: 500000
      end_tick: 800000
      climate:
        base_temperature: 0.45
        radiation_level: 0.08
        light_level: 0.85
      notes: "light becomes stronger selection factor"

    - id: "catastrophic_cooling"
      start_tick: 800000
      end_tick: 850000
      climate:
        base_temperature: 0.20
        light_level: 0.30
      notes: "mass extinction-like pressure"

    - id: "recovery_phase"
      start_tick: 850000
      end_tick: 1000000
      climate:
        base_temperature: 0.50
        light_level: 0.70
      notes: "post-catastrophe recovery"
```

Це не історична модель Землі.

Це earth-like evolutionary pressure scenario.

---

# Randomness

World Config повинен дозволяти контрольовану випадковість.

```yaml id="vww7mo"
randomness:
  seed: 12345
  weather_seed: 222
  catastrophe_seed: 333
  resource_seed: 444
```

За однакового seed симуляція має бути відтворюваною.

---

# Presets

Можна мати готові presets.

```yaml id="jv0m1u"
presets:
  - "stable_warm_world"
  - "seasonal_world"
  - "volcanic_world"
  - "ice_age_world"
  - "earth_like_evolution"
  - "catastrophe_stress_test"
```

Preset — це лише config.

Він не змінює engine.

---

# Validation

World Config повинен проходити validation.

Перевірки:

* `width > 0`;
* `height > 0`;
* `max_ticks > 0`;
* значення climate у допустимих межах;
* epoch ranges не конфліктують неконтрольовано;
* resource_id існує в `resources_config.md`;
* field_id існує в `fields_config.md`;
* catastrophe duration не від'ємна;
* probabilities у межах `0.0..1.0`;
* seed заданий або автоматично створений.

---

# Правила

## Rule 1. World Config changes conditions, not organisms

Config задає середовище, а не готові форми життя.

## Rule 2. Engine must not be edited for new worlds

Новий світ створюється через config.

## Rule 3. Earth-like is a scenario, not hardcoded history

Землеподібна еволюція має бути одним із сценаріїв.

## Rule 4. Catastrophes change environment

Катастрофи не повинні напряму вбивати клітини командою.

## Rule 5. Weather and climate are Fields/conditions

Погода й клімат мають впливати через Fields, Resources, Heat, Pressure та інші універсальні механізми.

## Rule 6. Config must be deterministic with seed

Однаковий config і seed мають давати відтворюваний результат.

## Rule 7. Separate worlds configs

Новий світ може бути створений паралельно з іншими тобто конфіги та поточні стани не перетираються

---

# Заборонено

Не вводити:

* hardcoded Earth;
* hardcoded species;
* hardcoded plants or animals;
* catastrophe as direct kill command;
* weather as direct behavior modifier;
* climate as hidden fitness score;
* scenario that spawns ready organisms;
* config values that bypass Materials, Resources, Fields or Energy.

---

# Пов'язані документи

* `config/resources_config.md`
* `config/materials_config.md`
* `config/fields_config.md`
* `world/fields.md`
* `world/resources.md`
* `world/materials.md`
* `world/energy.md`
* `world/physics.md`
* `world/tick.md`
* `evolution/selection.md`
* `evolution/population-dynamics.md`

---

# Open Questions

## Time scale

Потрібно визначити, що означає `tick_duration` для різних сценаріїв.

## Scenario format

Потрібно вирішити, чи сценарії будуть YAML, JSON або власний формат.

## Climate model

Потрібно визначити, чи climate є окремим шаром, чи лише набором Fields.

## Weather complexity

Потрібно вирішити, чи MVP має погодні події, чи лише noise + seasons.

## Catastrophe model

Потрібно визначити мінімальний набір catastrophe effects для MVP.

## Earth-like preset

Потрібно вирішити, чи `earth_like_evolution` буде офіційним preset або research scenario.

## Validation schema

Потрібно створити schema validation для world config.
