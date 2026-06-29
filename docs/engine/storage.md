# docs/engine/storage.md

> **Storage — збереження метрик, подій, snapshot і дослідницьких даних**

---

# Призначення

`storage.md` описує, які дані симуляції зберігати, як часто і для чого.

Storage потрібен для:

* long-running experiments;
* replay;
* debugging;
* population analysis;
* lineage tracking;
* comparing configs;
* detecting emergence.

Storage не повинен перетворитися на повний дамп усього світу кожен Tick.

---

# Основна ідея

Потрібні різні типи збереження:

```text id="hbx6e1"
Full Snapshot
Statistics Timeseries
Event Log
Trace Log
Config Archive
```

Кожен тип має різну частоту й призначення.

---

# Full Snapshot

Full Snapshot — повний стан, з якого можна продовжити симуляцію.

Зберігати рідко.

Приклад:

```text id="lhpyyj"
every 1_000 - 10_000 ticks
```

Містить:

```text id="pdqs93"
world state
cells
resources
materials
energy buffers
genomes
joints
fields
rng state
current tick
config reference
```

Детально описується в `serialization.md`.

---

# Statistics Timeseries

Statistics Timeseries — легкі числові метрики.

Зберігати часто.

Приклади:

```text id="f2zcoi"
living_cell_count
dead_cell_count
birth_rate
death_rate
resource_totals
average_energy
mutation_count
lineage_count
organism_like_component_count
largest_component_size
```

---

# Event Log

Event Log — послідовність важливих подій.

```text id="ukxriu"
tick
event_type
entity_id
details
```

Події:

```text id="frpm7n"
cell_birth
cell_death
division
mutation
joint_created
joint_broken
catastrophe_started
catastrophe_ended
lineage_extinct
component_fragmented
```

Event Log важливий для аналізу evolution.

---

# Trace Log

Trace Log — детальний debug.

Він дорогий.

Режими:

```text id="e1xa2r"
off
summary
focused_entity
sampled
full_debug
```

За замовчуванням не зберігати повний trace.

---

# Config Archive

Кожен run має зберігати копію config.

```text id="80kszq"
world_config
resources_config
materials_config
fields_config
engine version
schema version
seed
```

Без цього результати неможливо відтворити.

---

# Run Metadata

Кожен run повинен мати metadata.

```yaml id="h7j5a8"
run:
  id: "run_2026_06_29_001"
  created_at: "2026-06-29T00:00:00"
  engine_version: "0.1.0"
  config_hash: "..."
  seed: 12345
  notes: "early earth-like базова модель test"
```

---

# Storage Frequency

Частота має бути config-driven.

Приклад:

```yaml id="7mw56v"
storage:
  statistics_interval: 100
  event_log_enabled: true
  full_snapshot_interval: 5000
  trace_mode: "summary"
```

---

# What Not to Store Every Tick

Не зберігати кожен Tick за замовчуванням:

```text id="d0mpyx"
full cell state for all cells
full genome graph for every cell
full field layers
full resource grid
full signal trace
full rendering state
```

Це швидко зламає storage.

---

# Suggested базова модель Formats

Для базової моделі можна використати:

```text id="6wgtcd"
JSON / JSONL for metadata and event logs
CSV for statistics timeseries
JSON or MessagePack for snapshots
```

Формат можна змінити пізніше.

Головне — чітко розділити типи даних.

---

# Storage and Research

Storage має підтримувати дослідницькі питання:

```text id="0axq71"
чи зростає population?
чи виникають stable lineages?
чи зменшується OOS-like failure?
чи виникають organism-like components?
чи є selection після catastrophe?
чи з’являється specialization?
```

Для цього не потрібен повний дамп усього світу.

Потрібні метрики й event logs.

---

# Rules

## Rule 1. Store different data at different frequency

Snapshots, metrics, events і traces мають різну частоту.

## Rule 2. Full snapshot is for resume

Snapshot має дозволяти продовжити run.

## Rule 3. Metrics are for analysis

Statistics не повинні впливати на клітини.

## Rule 4. Config must be archived

Без config і seed run не має дослідницької цінності.

## Rule 5. Trace must be controlled

Full trace тільки для debug, не для кожного run.

---

# Заборонено

Не вводити:

* full dump every Tick by default;
* storage-driven behavior;
* statistics as Genome input;
* rendering state as required simulation state;
* snapshots without config;
* event logs without tick;
* unbounded trace.

---

# Як доопрацьовувати

Під час реалізації цей файл треба розширити, коли буде обрано:

* storage folder structure;
* snapshot format;
* statistics schema;
* event log schema;
* compression;
* retention policy;
* replay format.

---

# Open Questions

* Який формат storage обрати Для базової моделі?
* Чи потрібна SQLite/Parquet пізніше?
* Як зберігати великі grid layers?
* Чи потрібен replay без повного snapshot?
* Як зв’язувати event log із lineage tree?
* Які metrics є must-have для першого vertical slice?


