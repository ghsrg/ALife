---
tags:
  - alife
  - config
  - area/config
---

# stability_bounds.md

> Межі параметрів для стабільної та діагностованої симуляції.

---

# Призначення

Цей файл фіксує типи меж, які потрібні, щоб світ не розвалювався чисельно або логічно.

Точні значення належать окремим scenario/config файлам і мають перевірятися експериментально.

---

# Категорії Меж

```text
time_step bounds
space/cell density bounds
resource concentration bounds
material stability bounds
energy capacity and flow bounds
heat/temperature bounds
reaction rate bounds
mutation rate bounds
joint density/strength bounds
process duration bounds
scheduler budget bounds
```

---

# Канонічні правила

- Межа має мати одиниці або нормалізований діапазон.
- Значення поза межами не повинні мовчки прийматися.
- Stability config не є Canon закону світу; це експериментальна рамка.
- Для кожного boundary test має бути описано очікування: stable, fragile, collapse або invalid.

---

# Мінімальні Виходи Тесту

```text
scenario_id
config_hash
seed
tick_count
survival_result
collapse_reason
metrics_summary
```

---

# Hard Invalid Examples

Config або scenario invalid, якщо:

```text
negative amount/rate/capacity
unknown Resource/Material/Field/process id
reaction creates products without inputs
reaction uses unknown Resources/Materials
configured sink/loss is required but missing
Field has unbounded value without clamp/decay/abstracted rule
capacity is too high for radius/materials without explicit override
stored amount exceeds capacity at initialization
cell radius is impossible for world size
division creates daughters below minimum viable footprint/capacity
mutation can create graph beyond validation limits
```

---

# Warning Ranges

Config або scenario should warn, якщо:

```text
diffusion/decay above safe per-Tick bound
reaction rate too high for dt
heat transfer/dissipation creates likely instability
resource density likely causes immediate clogging
cell density likely causes immediate collision collapse
energy capacity far exceeds storage-capable Materials
process duration/cost likely prevents any lifecycle progress
unbalanced reaction accounting has explicit configured sink/loss
```

Exact warning thresholds are scenario-specific and should be calibrated experimentally.

---

# Прийняті Експериментальні Межі

Ці значення не є Canon законами світу. Це зафіксований результат `early-stability` calibration pass для `single_cell_survival`.

Source config не змінюється автоматично. Поточний baseline лишається прийнятним, якщо він лежить всередині stable range і негативні сценарії продовжують падати очікувано.

## single_cell_survival: energy/heat smoke pass

Джерело:

```text
outputs/worklogs/2026-07-01-1805-REPORT-early-stability-parameter-tuning.md
outputs/stability/single_cell_tune/ranges.json
```

Комбінація сценарію:

```text
scenario: tools/early-stability/scenarios/single_cell_survival.toml
tuning: tools/early-stability/tuning/single_cell.toml
runs: 3000
stable: 1350
fragile: 750
collapse: 900
invalid: 0
```

Accepted stable ranges:

| Parameter | Current Baseline | Stable Min | Stable Max | Tool Recommended | Decision |
| --- | ---: | ---: | ---: | ---: | --- |
| `cell.initial_energy` | `50.0` | `10.0` | `50.0` | `30.0` | Keep current baseline; accept range. |
| `cell.mandatory_cost_per_tick` | `2.0` | `1.0` | `5.0` | `3.0` | Keep `2.0`; accept range. |
| `environment.heat_dissipation_rate` | `0.2` | `0.05` | `0.5` | `0.275` | Keep current baseline; accept range. |

Interpretation:

```text
cell.mandatory_cost_per_tick = 2.0 is inside stable range and remains the baseline.
Tool midpoint recommendations are not automatically better defaults.
Ranges are accepted as current experimental bounds for Phase 1 smoke calibration.
```

Not covered by this pass:

```text
cell.radius
cell.energy_capacity
cell.capacity_limit
cell.initial_resources
cell.initial_materials
resources.initial_distribution
resources.passive_energy_income_placeholder
environment.heat_generated_per_tick
environment.heat_warning_threshold
environment.heat_death_threshold
environment.waste_generated_per_tick
environment.waste_sink_rate
environment.waste_warning_threshold
environment.waste_death_threshold
lifecycle.stress_energy_threshold
cell.dormant_mandatory_cost_modifier
estimates.*
world.size
space.spatial_grid_size
```

---

# Scenario-Specific Experimental Ranges

Scenario configs may define experimental ranges for:

```text
single_cell_survival
single_cell_division
heat_stress
resource_starvation
reaction_balance
multicellular_stability
joint_density
mutation_stability
```

Experimental range is not Canon law. It is a calibration boundary for a specific scenario.

---

# Semantic Links

- bounds: [[docs/config/world_config|World Config]]
- bounds: [[docs/config/resources_config|Resources Config]]
- bounds: [[docs/config/materials_config|Materials Config]]
- bounds: [[docs/config/reactions_config|Reactions Config]]
- protects stability of: [[docs/biology/cell|Cell]]

# Пов'язані документи

- `docs/examples/config-examples.md`
- `world/units.md`
- `world/field-semantics.md`
- `engine/scheduler.md`
- `engine/performance.md`
