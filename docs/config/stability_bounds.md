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

# Пов'язані документи

- `docs/examples/config-examples.md`
- `world/units.md`
- `world/field-semantics.md`
- `engine/scheduler.md`
- `engine/performance.md`
