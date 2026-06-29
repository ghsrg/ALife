# engine/serialization.md

> **Serialization — збереження, завантаження і відтворюваність симуляції**

---

# Призначення

Цей документ описує принципи serialization для світу.

Serialization потрібна для:

* save/load;
* long-running simulations;
* reproducibility;
* debugging;
* snapshots;
* research analysis;
* comparing runs;
* replaying evolution.

---

# Основна ідея

Симуляція має бути відтворюваною.

```text id="vbxbwf"
Config
+
Initial State
+
Seed
+
Engine Version
    ↓
Reproducible Run
```

Snapshot повинен дозволити продовжити симуляцію з певного Tick.

---

# Що треба серіалізувати

Мінімально:

```text id="xcdclg"
world state
cells
resources
materials
energy buffers
genomes
joints
fields
lineage metadata
rng state
config references
current tick
```

---

# Що НЕ обов'язково серіалізувати як state

Не потрібно зберігати як поведінковий state:

```text id="pn1bh1"
observed_role
organism-like label
species-like cluster label
statistics-only values
debug UI state
temporary traces if disabled
```

Це можна відновити або перерахувати.

---

# Config Version

Кожен snapshot має містити посилання на версію config.

```yaml id="s9x63u"
config:
  world_config_version: "2026-06-29"
  resources_config_version: "2026-06-29"
  materials_config_version: "2026-06-29"
  fields_config_version: "2026-06-29"
```

Якщо config змінився, старий snapshot може стати несумісним.

---

# Engine Version

Snapshot має містити engine version.

```yaml id="86zttw"
engine:
  version: "0.1.0"
  schema_version: "snapshot-v1"
```

Це потрібно для міграцій.

---

# RNG State

Seed недостатній для продовження з середини симуляції.

Потрібно зберігати RNG state.

```yaml id="3njq03"
rng:
  world_rng_state: "..."
  mutation_rng_state: "..."
  weather_rng_state: "..."
```

Для MVP можна почати з одного RNG state.

---

# Genome Serialization

Genome повинен мати стабільну серіалізацію.

```yaml id="2meao1"
genome:
  id: "genome_001"
  nodes: []
  edges: []
  input_bindings: []
  output_bindings: []
  mutation_parameters: {}
```

Порядок nodes/edges має бути стабільним.

Не покладатися на порядок hash map.

---

# Cell Serialization

Cell state:

```yaml id="9c3fdv"
cell:
  id: "cell_001"
  position: { x: 10.0, y: 20.0 }
  resources: {}
  materials: {}
  energy_buffer: 0.45
  genome_id: "genome_001"
  boundary_state: {}
  lifecycle_state: "alive"
  lineage: {}
```

---

# Joint Serialization

Joint state:

```yaml id="obwb95"
joint:
  id: "joint_001"
  cell_a: "cell_001"
  cell_b: "cell_002"
  strength: 0.7
  damage: 0.1
  signal_state: {}
  transport_state: {}
```

---

# Field Serialization

Fields можуть бути великими.

Для MVP можна зберігати:

```text id="fpwecb"
field config
dynamic field layers if changed
heat layer
pressure layer if dynamic
```

Static fields можна відновлювати з config.

Dynamic fields треба зберігати.

---

# Snapshot Types

## Full Snapshot

Повний стан світу.

```text id="xzq36e"
used for save/load
large
less frequent
```

## Statistics Snapshot

Тільки метрики.

```text id="rk6d0j"
used for charts
small
frequent
```

## Event Log

Події:

```text id="9sh7e3"
birth
death
mutation
division
joint_created
joint_broken
fragmentation
extinction
catastrophe
```

---

# MVP Recommendation

Для MVP:

```text id="8vywwb"
JSON or MessagePack for snapshots
CSV/JSONL for event logs
separate statistics timeseries
stable ids
schema_version
seed and rng state
```

Формат можна змінити пізніше.

Головне — не втратити determinism і compatibility.

---

# Правила

## Rule 1. Snapshot must resume simulation

Full snapshot має дозволяти продовжити симуляцію.

## Rule 2. Store config references

Snapshot повинен знати, з яким config він створений.

## Rule 3. Store RNG state

Для точного resume потрібен RNG state.

## Rule 4. Analytical labels are not behavior state

Observed roles, organism labels і species-like clusters не повинні керувати клітинами.

## Rule 5. Use stable ids

Genome, Cell, Joint і Lineage повинні мати стабільні ids.

---

# Заборонено

Не вводити:

* serialization that depends on memory addresses;
* random unordered serialization;
* save files without schema version;
* snapshot without config reference;
* species-like labels as required behavior state;
* organism id as cell decision input.

---

# Open Questions

* Який формат snapshot для MVP: JSON, MessagePack, SQLite?
* Чи потрібна compression?
* Як зберігати великі Field layers?
* Як робити migration між schema versions?
* Як часто робити full snapshots?

---

