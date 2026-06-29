# docs/engine/rendering.md

> **Rendering — візуалізація симуляції без впливу на поведінку**

---

# Призначення

`rendering.md` описує мінімальні принципи візуалізації.

Rendering потрібен для:

* debug;
* спостереження evolution;
* аналізу emergent structures;
* демонстрації клітин, Resources, Fields, Joint;
* дослідження population dynamics.

Rendering не повинен впливати на симуляцію.

---

# Основна ідея

```text id="gp2ug4"
Simulation State
    ↓
Read-only View Model
    ↓
Rendering
```

Renderer читає стан, але не змінює його.

---

# Що Rendering НЕ є

Rendering не є:

* частиною physics;
* частиною biology;
* поведінковою системою;
* джерелом random;
* способом керувати клітинами;
* обов'язковим для headless simulation.

Симуляція має працювати без UI.

---

# Scope базової моделі

Для базової моделі достатньо:

```text id="jwdcnk"
2D viewport
cell circles
resource heatmap / layer view
field heatmap
joint lines
selected cell inspector
basic time controls
population metrics panel
```

Не потрібно у базовій моделі:

```text id="42p7ud"
3D
beautiful graphics
organism animation
particle effects
complex shaders
cinematic view
```

---

# Rendering Layers

Рекомендовані layers:

```text id="x527zv"
Fields layer
Resources layer
Cells layer
Joints layer
Signals layer
Debug overlay
Selection inspector
```

Layers мають вмикатися/вимикатися.

---

# Cells

Клітини можна показувати як circles.

Колір не повинен означати hardcoded species.

Колір може показувати обраний debug mode:

```text id="g6nnt8"
energy level
lifecycle state
material profile
lineage
stress level
observed role
```

`observed_role` — тільки аналітичний label.

---

# Resources

Resources краще показувати як heatmap або grid overlay.

Modes:

```text id="coyi1z"
resource_A concentration
total resource density
waste concentration
energy_value resources
```

---

# Fields

Fields показувати як heatmap/vector overlay.

Приклади:

```text id="5t746c"
light
heat
pressure
radiation
flow
```

---

# Joints

Joint показувати як lines між клітинами.

Debug modes:

```text id="59j9es"
joint strength
joint damage
resource flow
signal flow
mechanical tension
```

---

# Signals

Signals можна показувати тільки в debug mode.

Не вмикати повний signal trace завжди.

Modes:

```text id="tyg9na"
current joint signals
recent emitted signals
selected cell signal inputs
```

---

# Inspector

Selected Cell Inspector має показувати:

```text id="v2fbfr"
cell_id
position
energy_buffer
resources
materials
boundary state
lifecycle state
genome summary
current inputs
last action plan
lineage
joints
```

Inspector не повинен дозволяти непомітно змінювати simulation state, якщо не ввімкнено explicit debug edit mode.

---

# Headless Mode

Engine має запускатися без rendering.

```text id="xq1ktx"
headless simulation
    ↓
statistics
snapshots
event logs
```

Це важливо для довгих runs.

---

# Performance

Rendering не повинен ламати performance.

Правила:

```text id="ctn2do"
render не кожен simulation tick
downsample large layers
turn off expensive overlays by default
use sampled traces
```

---

# Rules

## Rule 1. Rendering is read-only

Renderer не змінює simulation state.

## Rule 2. Headless mode is required

Симуляція має працювати без UI.

## Rule 3. Debug labels do not affect biology

Observed roles, organism labels і species-like clusters не є input для клітин.

## Rule 4. Layers must be optional

Важкі overlays мають вимикатися.

## Rule 5. Visuals must not create canon

Колір або форма в UI не повинні закріплювати hardcoded species або cell types.

---

# Заборонено

Не вводити:

* rendering-driven simulation;
* UI state as biology state;
* species color as behavior;
* organism label as control;
* renderer that modifies cells silently;
* full trace rendering by default;
* dependency on rendering for tests.

---

# Як доопрацьовувати

Під час реалізації цей файл треба розширити, коли буде обрано:

* UI technology;
* render loop;
* viewport model;
* layer API;
* inspector schema;
* debug overlay format.

---

# Open Questions

* Чи rendering буде Canvas, WebGL, SVG або інше?
* Чи потрібен remote/headless viewer?
* Як синхронізувати simulation ticks і render frames?
* Які debug modes потрібні першими?
* Чи потрібен replay viewer для snapshots?

---



