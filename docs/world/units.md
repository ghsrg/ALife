# units.md

> **Units — умовні одиниці симуляції, нормалізовані значення та per-Tick rates**

---

# Призначення

`units.md` визначає базову систему одиниць для документації та майбутніх конфігів.

Це не прив'язка до SI або реальної біології. Це спільна шкала, щоб Energy, Resources, Materials, Space, Fields і process costs можна було порівнювати без прихованих припущень.

---

# Unit Families

У документації використовуються три родини величин:

```text
Simulation units
Normalized units
Per-Tick rates
```

## Simulation units

Simulation units описують фізичні величини світу:

| Unit | Meaning |
| --- | --- |
| `su` | space unit |
| `vu` | volume unit |
| `au` | amount unit для Resource |
| `mu` | material amount unit |
| `eu` | energy unit |
| `tick` | discrete simulation time step |

## Normalized units

Normalized units мають діапазон `0.0..1.0`, якщо інше явно не описано:

- Field values;
- Signals;
- Material properties;
- efficiencies;
- probabilities;
- capability levels.

Для регуляторних виходів Genome Runtime може використовуватися діапазон `-1.0..+1.0`, де `-1.0` означає suppression, `0.0` neutral, `+1.0` strong priority.

## Per-Tick rates

Rates задаються на один Tick:

- diffusion;
- decay;
- maintenance;
- process rates;
- heat transfer rates;
- signal decay.

Якщо rate не є per-Tick, це повинно бути явно вказано.

---

# Baseline Ranges

Ці межі є стартовими діапазонами для стабільних конфігів, а не фундаментальними законами.

## Cell

| Parameter | Baseline |
| --- | --- |
| radius | `1.0..2.0 su` |
| volume_capacity | `10.0 vu` |
| energy_capacity | `10.0 eu` |

## Resource

| Parameter | Baseline |
| --- | --- |
| common amount packet | `0.1..1.0 au` |
| volume_per_unit | `0.5..2.0 vu` |
| density | `0.5..3.0` |
| energy_value | `0.0..2.0 eu/au` |

## Material

| Parameter | Baseline |
| --- | --- |
| amount | `0.1..10.0 mu` |
| volume_per_unit | `0.5..2.0 vu` |
| synthesis energy cost | `0.1..2.0 eu` |

## Process

| Parameter | Baseline |
| --- | --- |
| passive maintenance | `0.001..0.05 eu/tick` |
| active action | `0.05..1.0 eu` |
| division | `3.0..8.0 eu` |

## Fields

| Parameter | Baseline |
| --- | --- |
| value | `0.0..1.0` |

## Rates

| Parameter | Baseline |
| --- | --- |
| decay_rate | `0.0001..0.05 per Tick` |
| diffusion_rate | `0.01..0.5 per Tick` |

---

# Energy And Volume

Energy не є `Resource` і не є `Material`.

Energy Buffer capacity задається Materials, які займають внутрішній volume клітини.

Stored Energy не додає окремий material volume, якщо майбутнє правило явно не вводить таку модель.

---

# 2D Capacity

У 2D base model `volume_capacity` є abstract internal capacity unit, not SI volume.

Starting relation:

```text
cell_area = π * radius²

volume_capacity =
  base_capacity_per_area
  * cell_area
  * storage_material_modifier
```

Capacity must remain bounded by radius, footprint and storage-capable Materials.

Mass is derived from contained Resources, Materials and configured density values.

Energy Buffer does not add separate mass or volume.

---

# Config Validation

Config може мати:

- hard invalid bounds;
- warning ranges;
- scenario-specific ranges;
- experimental ranges.

Стабільні межі уточнюються в `config/stability_bounds.md`.

---

# Rules

## Rule 1. Units are explicit

Кожна кількісна величина повинна мати зрозумілу одиницю або бути normalized value.

## Rule 2. Rates are per Tick

Rate вважається per-Tick, якщо інше явно не описано.

## Rule 3. Baselines are not laws

Baseline ranges є стартовими межами калібрування, а не фізичною істиною.

## Rule 4. Fields and signals are normalized

Fields, signals, probabilities, efficiencies і material capabilities мають normalized scale.

---

# Пов'язані документи

- `world/space.md`
- `world/energy.md`
- `world/resources.md`
- `world/materials.md`
- `config/stability_bounds.md`
- `config/world_config.md`
- `config/resources_config.md`
- `config/materials_config.md`
- `config/fields_config.md`
