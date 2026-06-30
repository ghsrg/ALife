# stability_bounds.md

> **Stability Bounds — empirical config validation and calibration horizons**

---

# Призначення

`stability_bounds.md` описує, як поступово знаходити стабільні межі конфігів.

Стабільність не hardcoded наперед. Вона калібрується через smoke tests, isolated balance checks і scenario benchmarks.

---

# Base Approach

```text
initial assumptions
  ↓
primitive balance calculator
  ↓
isolated stability tests
  ↓
scenario seed configs
  ↓
observed stable / unstable ranges
  ↓
config validation rules
```

---

# Validation Levels

| Level | Meaning |
| --- | --- |
| fatal | config violates core invariants or cannot run |
| warning | config may run but likely creates unstable or hostile world |
| info | unusual but allowed experimental setting |

---

# Calibration Stages

1. Smoke stability.
2. Single-cell survival.
3. Single-cell division.
4. Multicellular stability.
5. Specialization potential.
6. Evolution run.
7. Cognitive potential.

---

# Balance Calculator

Primitive calculator estimates:

- maintenance cost per Tick;
- expected Energy intake;
- resource consumption rate;
- resource regeneration rate;
- reaction output balance;
- heat / waste accumulation;
- division affordability;
- material repair affordability;
- decay pressure.

It does not prove life will emerge. It detects impossible configs before simulation.

---

# Seed Configs

Reference scenarios:

- `smoke_world`
- `single_cell_survival`
- `single_cell_division`
- `multicellular_stability`
- `basic_evolution`
- `hostile_world`

Each seed config should define expected outcomes and tick horizon.

---

# Time Horizons

Initial calibration defaults:

| Scenario | Horizon |
| --- | --- |
| smoke | `1,000 ticks` |
| single-cell survival | `10,000 ticks` |
| single-cell division | `50,000 ticks` |
| basic evolution | `100,000..500,000 ticks` |

Default time scale:

```text
1 tick = 1 simulation second
online target = 10..30 ticks/sec wall-clock
```

These values are calibration defaults, not biological truth.

---

# Instability Categories

| Category | Meaning |
| --- | --- |
| instant invalidity | cannot initialize or violates invariants at tick 0 |
| immediate collapse | most life-critical state collapses in first 100 ticks |
| early instability | world fails before 1,000 ticks |
| no single-cell survival | no stable survival by 10,000 ticks |
| no sustainable evolution | no persistent lineages over evolution horizon |

Config can be stable for one horizon and unstable for another.

---

# Rules

## Rule 1. Stability is empirical

Stable ranges are empirical contracts, not guesses.

## Rule 2. Stability is scenario-specific

No config is simply stable or unstable without scenario and tick horizon.

## Rule 3. Invalid configs fail early

Fatal configs must fail validation before simulation.

## Rule 4. Hostile worlds are explicit

Hostile/experimental configs are allowed only when marked as such.

---

# Пов'язані документи

- `world/units.md`
- `config/world_config.md`
- `config/resources_config.md`
- `config/materials_config.md`
- `config/fields_config.md`
- `config/reactions_config.md`
