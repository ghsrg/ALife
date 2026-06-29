# engine/performance.md

> **Performance — обмеження продуктивності для CPU-first симуляції**

---

# Призначення

Цей документ описує базові performance-принципи для базової моделі.

Мета — не максимальна оптимізація, а уникнення очевидно поганих рішень, які зламають симуляцію на ранньому етапі.

Проєкт має бути CPU-first.

---

# Основна ідея

Симуляція може мати багато:

* клітин;
* Resources;
* Materials;
* Joint;
* Fields;
* events;
* mutations;
* traces.

Тому треба одразу уникати алгоритмів, які масштабуються погано.

---

# Головні ризики

```text id="paxjhf"
O(N²) перевірки всіх клітин з усіма
deep copy всього світу кожен Tick
повний trace для кожної клітини кожен Tick
велика кількість дрібних об'єктів без потреби
недетермінований parallel update
занадто часті full snapshots
```

---

# Spatial Index

Потрібен spatial index.

Мінімально:

```text id="9s4w02"
uniform grid
```

Він потрібен для:

* пошуку сусідів;
* collision;
* Resource lookup;
* Joint creation;
* local signals;
* local Fields.

Заборонено Для базової моделі:

```text id="nrw173"
for each cell:
  for each other cell:
    check distance
```

---

# Tick Budget

Кожен Tick має виконувати лише необхідну роботу.

Рекомендації:

```text id="xyhlsd"
оновлювати тільки active cells
не рахувати full analytics кожен Tick
не зберігати full snapshot кожен Tick
не перераховувати static Fields
```

---

# Data Locality

Для великої кількості клітин бажано зберігати Components компактно.

Рекомендація:

```text id="9q89t3"
arrays / typed arrays / tables
```

а не велика кількість вкладених об'єктів, якщо це шкодить продуктивності.

---

# Double Buffering без Deep Copy

Double buffering не означає копіювати весь світ кожен Tick.

Краще:

```text id="30m7u1"
read snapshot references
write change buffers
apply changes in controlled phase
```

Для базової моделі можна простіше, але не робити повний deep copy всіх даних без потреби.

---

# Tracing

Trace потрібен для debugging, але він дорогий.

Режими:

```text id="4sitlt"
off
summary
sampled
focused_entity
full_debug
```

За замовчуванням:

```text id="1zkz5d"
summary
```

Full trace тільки для малих світів або конкретних entity.

---

# Snapshots

Full snapshots дорогі.

Рекомендація:

```text id="g197tq"
statistics every 100 ticks
event log continuously
full snapshot every 1000-10000 ticks
```

Точні значення мають бути config-driven.

---

# Fields

Static Fields не треба перераховувати кожен Tick.

```text id="27yfr3"
static light gradient -> precompute
dynamic heat layer -> update each Tick
```

---

# Resources

Resource diffusion може бути дорогим.

варіанти базової моделі:

```text id="t66l41"
grid-based diffusion
update only non-empty cells
skip tiny amounts
batch updates
```

---

# Joints

Joint update масштабується з кількістю Joint.

```text id="0gmkgk"
O(number_of_joints)
```

Це прийнятно.

Але не треба для кожної клітини шукати всі Joint через scan.

Потрібен index:

```text id="h71ccb"
cell_id -> joint_ids
```

---

# Genome Runtime

Genome Runtime виконується для кожної active cell.

Потрібні обмеження:

```text id="49c0rk"
max_nodes_per_genome
max_edges_per_genome
max_runtime_steps
```

Без цього mutation може створити надто дорогі Genome.

---

# Population Limits

Не треба штучно обмежувати еволюцію, але технічні ліміти потрібні.

```text id="23b3qe"
max_cells
max_joints
max_resources_per_cell
max_materials_per_cell
max_genome_nodes
max_genome_edges
```

Якщо ліміт досягнуто, поведінка має бути визначена config.

---

# Determinism vs Parallelism

Для базової моделі важливіше determinism, ніж parallel performance.

Спочатку:

```text id="adg3g5"
single-thread deterministic
```

Пізніше:

```text id="jqxfsh"
parallel systems with deterministic reduction
```

Не вводити недетермінований parallel update на старті.

---

# базова модель Performance Target

Для першого vertical slice:

```
online target:

world:
  1024 × 1024 grid

cells:
  100 000 active cells target

joints:
  up to 50 000

fields:
  5 layers:
    light
    heat
    pressure
    radiation
    flow

resources:
  4–8 resource types
  sparse grid updates

genome:
  max 32 nodes
  max 64 edges

performance:
  30 ticks/sec with rendering
  1000+ ticks/sec offline without rendering
  
CPU-only
deterministic replay
```

Це не жорстка вимога, а орієнтир.

---

# Правила

## Rule 1. Avoid O(N²) cell scans

Пошук сусідів має йти через spatial index.

## Rule 2. Do not deep copy the whole world each Tick

Використовувати change buffers або часткові оновлення.

## Rule 3. Trace must be configurable

Full trace не повинен бути увімкнений завжди.

## Rule 4. Static data should be cached

Static Fields і config-derived data не треба перераховувати постійно.

## Rule 5. Determinism first

базова модель має бути deterministic before parallel.

---

# Заборонено

Не вводити:

* all-cells-to-all-cells distance checks;
* full snapshot every Tick by default;
* full debug trace by default;
* nondeterministic parallel updates;
* unbounded genome growth;
* unbounded joint creation;
* unbounded resource types per cell;
* performance hacks that break canon.

---

# Open Questions

* Який target cell count Для базової моделі?
* Чи потрібен одразу binary snapshot?
* Який spatial grid size обрати?
* Коли вводити parallel processing?
* Які ліміти genome size ставити в config?
* Який мінімальний profiling набір потрібен?


