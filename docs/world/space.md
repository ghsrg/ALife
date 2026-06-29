# space.md

> **Space — простір симуляції, локальність і межі світу**

---

# Призначення

`space.md` описує, як у світі задається простір.

Space визначає:

* де знаходяться клітини;
* де знаходяться Resources;
* де діють Fields;
* що означає local neighborhood;
* як працюють межі світу;
* як простір пов’язаний із Physics, Fields, Resources і Rendering.

Space не описує детально рух, collision, pressure або deformation.

Це відповідальність:

* `world/physics.md` — концептуальна фізика світу;
* `engine/physics.md` — реалізація фізичного шару;
* `engine/performance.md` — продуктивний spatial index.

---

# Основна ідея

У базовій моделі світ є двовимірним простором.

```text id="0zf8sw"
World Space
├── width
├── height
├── coordinate system
├── boundary mode
├── spatial grid
└── local neighborhoods
```

Кожна клітина, Resource patch, Joint або Field value має просторовий контекст.

```text id="0n91av"
Cell
Resource
Field value
Joint
Trace
    ↓
exist somewhere in Space
```

---

# Що Space НЕ є

Space не є:

* Physics Engine;
* rendering canvas;
* organism body;
* species territory;
* глобальною картою, яку бачить клітина;
* способом дати клітині global knowledge;
* поведінковою системою.

Клітина не бачить весь Space.

Вона має доступ лише до локального середовища.

---

# Відмінність від Physics

Space відповідає на питання:

```text id="xxb3l4"
де щось існує?
що означає "поруч"?
які межі світу?
як поділений простір?
як знайти локальних сусідів?
```

Physics відповідає на питання:

```text id="4u3i6d"
як щось рухається?
як стикається?
як виникає pressure?
як Joint передає force?
як Materials руйнуються від фізичного впливу?
```

Тому `space.md` і `physics.md` мають бути окремими документами.

---

# Space Model базової моделі

Для базової моделі використовується 2D space.

```text id="ny43ho"
position:
  x
  y
```

Розмір світу задається в `config/world_config.md`.

Приклад:

```yaml id="j5svo0"
space:
  type: "2d"
  width: 512
  height: 512
  boundary: "wrapped"
  spatial_grid_size: 8
```

---

# Coordinate System

У базовій моделі координати умовні.

```text id="i127ne"
x: horizontal position
y: vertical position
```

Рушій не повинен прив’язувати координати до реальних метрів.

Це simulation units.

---

# Space Units

Усі просторові величини мають бути в умовних одиницях:

```text id="n21t42"
position
distance
radius
cell size
grid size
movement distance
joint length
```

Конкретне співвідношення з реальним світом не потрібне Для базової моделі.

---

# Boundary Modes

Світ може мати різні boundary modes.

```text id="d2gr2c"
wrapped
solid_wall
open
```

---

## wrapped

Край світу з’єднаний із протилежним краєм.

```text id="pkwz7z"
left edge  -> right edge
top edge   -> bottom edge
```

Корисно для стабільних експериментів, де не треба втрачати клітини через край.

---

## solid_wall

Край світу блокує рух.

```text id="455svd"
cell reaches wall
    ↓
movement blocked
    ↓
possible pressure / collision consequence
```

Корисно для pond-like або container-like сценаріїв.

---

## open

Об’єкти можуть залишати світ або втрачатися за межами.

```text id="5my9o7"
cell / resource leaves world
    ↓
removed or marked as out-of-world
```

Корисно для flow-like середовищ.

---

# Spatial Grid

Для продуктивності простір має бути поділений на grid або інший spatial index.

```text id="z7w4yh"
Space
└── Grid cells
    ├── Cells
    ├── Resources
    ├── Field values
    └── local events
```

Spatial grid потрібен для:

* neighbor lookup;
* Resource lookup;
* Field sampling;
* collision;
* Joint creation;
* local communication;
* diffusion;
* trace propagation.

---

# Grid Cell

Grid cell — це технічний елемент spatial index.

Він не є біологічною клітиною.

```text id="a8gp7p"
GridCell != CellEntity
```

Grid cell може містити посилання на:

```text id="g3plgc"
CellEntity ids
Resource amounts
Field values
Trace values
local events
```

---

# Locality

Усі взаємодії мають бути локальними.

Клітина може взаємодіяти лише з:

```text id="vx6swu"
nearby Resources
nearby Fields
nearby Cells
connected Joints
local Traces
local Pressure
```

Заборонено глобальне знання простору.

---

# Local Neighborhood

Local neighborhood — це область навколо клітини, яку вона може відчувати або з якою може взаємодіяти.

```text id="kaai9r"
Cell position
    ↓
nearby grid cells
    ↓
local inputs
```

Розмір neighborhood залежить від:

* cell sensing Materials;
* Resource diffusion;
* Field sampling;
* Joint distance;
* communication channel;
* physics radius.

---

# Cell Position

Кожна жива клітина має Position.

```yaml id="x5ioll"
position:
  x: 120.5
  y: 48.0
```

Position використовується для:

* Field sampling;
* Resource uptake;
* collision;
* Joint creation;
* communication;
* rendering;
* statistics.

Position не є поведінковим знанням клітини про весь світ.

---

# Resource Position

Resources можуть бути представлені як:

```text id="6rk6gh"
grid-based layer
patch-based entities
particle-like entities
hybrid model
```

Для базової моделі бажано почати з grid-based або sparse grid-based Resources.

```text id="tkm0f8"
Resource layer
├── resource_A amount per grid cell
├── resource_B amount per grid cell
└── waste amount per grid cell
```

---

# Field Position

Fields мають просторове значення.

```text id="hbpk58"
position
    ↓
field value at position
    ↓
cell local input
```

Приклади:

```text id="zzt1vm"
light at position
heat at position
pressure at position
radiation at position
flow at position
```

Клітина читає лише локальне значення Field або локальний gradient, якщо має відповідні Materials.

---

# Joint and Space

Joint з’єднує дві клітини, які мають Position.

```text id="hj6558"
Cell A position
Cell B position
    ↓
Joint distance / tension / constraint
```

Joint може мати просторові наслідки:

* force transfer;
* distance constraint;
* heat transfer;
* signal transfer;
* resource transfer;
* mechanical tension.

Але Joint не створює global organism space.

---

# Space and Organism-like Structures

Organism-like structure — це connected component клітин і Joint у Space.

```text id="l4uk32"
Cells in Space
+
Joints between nearby cells
    ↓
organism-like component
```

Organism не повинен мати окрему глобальну карту.

Організмоподібна форма виникає з позицій клітин, Joint і Physics.

Світ не знає `Organism` як активну сутність.

Observer або analytics layer може розпізнавати organism-like components для досліджень і lineage tracking.

---

# Space and Rendering

Rendering використовує Space для візуалізації.

Але rendering canvas не є source of truth.

```text id="sk84hv"
Simulation Space
    ↓
read-only view
    ↓
Rendering
```

Не можна, щоб UI координати керували біологією.

---

# Space and Config

Space задається через `config/world_config.md`.

Приклад:

```yaml id="dpx1ce"
space:
  type: "2d"
  width: 1024
  height: 1024
  boundary: "solid_wall"
  spatial_grid_size: 16
```

Новий розмір світу або boundary mode не повинен вимагати зміни engine code.

---

# Space and Performance

Space має підтримувати продуктивний local lookup.

Заборонено:

```text id="uaz5lv"
for each cell:
  for each other cell:
    check distance
```

Потрібно:

```text id="j31c3c"
cell position
    ↓
grid cell
    ↓
neighboring grid cells
    ↓
local candidates only
```

Це критично для CPU-first симуляції.

---

# Space and Scale

Цільовий масштаб базової моделі може бути таким:

```text id="n25o0l"
small world:
  256 x 256

default world:
  512 x 512

large world:
  1024 x 1024
```

Реальний performance залежить не тільки від розміру Space, а від:

* кількості active cells;
* кількості Joints;
* кількості Resource layers;
* кількості dynamic Fields;
* Genome Runtime cost;
* rendering mode;
* trace mode.

---

# Zones

Zones — це іменовані області Space.

Вони можуть використовуватися для config, climate або debug.

```yaml id="th3kro"
zone:
  id: "warm_shallow_zone"
  area:
    x_min: 0
    x_max: 400
    y_min: 0
    y_max: 512
```

Zone не повинна бути species territory.

Zone не повинна напряму керувати клітинами.

Вона лише задає просторову область для Fields, Resources або events.

---

# Local Events

Події можуть мати просторову область.

```yaml id="11np48"
event:
  id: "volcanic_zone"
  area:
    x: 300
    y: 300
    radius: 120
```

Подія змінює локальні умови.

Вона не повинна напряму вбивати клітини командою.

---

# Rules базової моделі

## Rule 1. Space defines locality

Усі взаємодії мають проходити через локальний просторовий контекст.

## Rule 2. Cells have no global map

Клітини не можуть бачити весь світ.

## Rule 3. Space is config-driven

Розмір, boundary і grid parameters задаються через config.

## Rule 4. Space is not Physics

Space описує “де”.

Physics описує “як рухається і взаємодіє”.

## Rule 5. Spatial index is required

Для базової моделі потрібен spatial grid або інший spatial index.

## Rule 6. Grid cell is not biological Cell

Grid cell — технічний елемент простору.

CellEntity — біологічна одиниця симуляції.

---

# Заборонено

Не вводити:

* global cell awareness;
* all-cells-to-all-cells scan;
* species territory as hardcoded space rule;
* organism-level global coordinates as behavior input;
* rendering canvas as simulation truth;
* teleport movement без process/physics;
* global resource lookup для клітини;
* global field map як Genome input.

---

# Пов’язані документи

* `world/physics.md`
* `world/fields.md`
* `world/resources.md`
* `world/tick.md`
* `config/world_config.md`
* `config/fields_config.md`
* `engine/physics.md`
* `engine/performance.md`
* `engine/ecs.md`
* `engine/rendering.md`

---

# Open Questions

## Grid size

Потрібно визначити default `spatial_grid_size` Для базової моделі.

## Resource representation

Потрібно вирішити, чи Resources будуть grid-based, entity-based або hybrid.

## Field representation

Потрібно вирішити, чи Field layers зберігаються як grid, function або hybrid.

## Boundary changes

Потрібно вирішити, чи boundary mode може змінюватися під час scenario.

## Zones

Потрібно визначити, чи Zones є частиною world config, field config або окремого spatial layer.

## Nested spaces

у майбутньому питання: чи потрібні nested spaces або compartments.

## 3D

у майбутньому питання: чи буде 3D space, і що для цього треба змінити.


