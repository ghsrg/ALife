# docs/engine/physics.md

> **Physics Engine — рух, простір, зіткнення, тиск, Joint і матеріальні наслідки**

---

# Призначення

`physics.md` описує відповідальність фізичного шару рушія.

Цей файл не задає повний physics solver.

Його задача — пояснити агенту, що physics має бути достатньо простою Для базової моделі, але всі біологічні наслідки мають проходити через матеріальну модель.

---

# Основна ідея

Physics Engine відповідає за:

* position;
* velocity;
* mass;
* volume;
* movement;
* collision;
* pressure;
* deformation;
* Joint constraints;
* spatial index;
* physical damage;
* boundary interactions.

```text id="sn6g3j"
Mass + Volume + Position + Materials
    ↓
Physics
    ↓
Movement / Pressure / Collision / Damage
```

---

# Що Physics Engine НЕ є

Physics Engine не є:

* реалістичним soft-body симулятором У базовій моделі;
* системою HP damage;
* поведінкою клітин;
* організмовим контролером;
* hardcoded movement system;
* hardcoded muscle system.

---

# Scope базової моделі

Для базової моделі достатньо:

```text id="btr58y"
2D position
simple velocity
cell radius / volume approximation
basic collision avoidance
spatial grid
simple pressure estimate
basic Joint distance constraint
movement cost support
material damage hooks
```

Не потрібно у базовій моделі:

```text id="yf3z2c"
full soft-body physics
complex fluid simulation
real gravity
advanced rigid body solver
3D
detailed deformation mesh
```

---

# Space

базова модель світ — 2D.

```text id="usys73"
Position:
  x
  y
```

Boundary світу задається в `world_config.md`:

```text id="u14mby"
wrapped
solid_wall
open
```

---

# Mass and Volume

Mass і volume мають походити з Resources і Materials.

```text id="wnom7r"
cell_mass =
sum(resources mass)
+
sum(materials mass)
```

```text id="h72cre"
cell_volume =
sum(resources volume)
+
sum(materials volume)
```

Не вводити arbitrary mass без зв’язку з речовиною.

---

# Movement

Movement є процесом клітини, але фізичне переміщення виконує Physics Engine.

```text id="4lgjkd"
Cell process requests movement
    ↓
Feasibility Check
    ↓
Physics applies movement
```

Movement має cost через Energy і Materials.

---

# Collision

Collision не повинен бути абстрактним damage.

Collision може викликати:

* pressure;
* deformation;
* Boundary damage;
* Joint damage;
* movement blocking.

```text id="6cexcc"
Collision
    ↓
Pressure / Deformation
    ↓
Material Damage
```

---

# Pressure

Pressure — локальний механічний вплив.

Він може виникати через:

* collision;
* crowding;
* Joint tension;
* world boundary;
* flow;
* growth in limited space.

Pressure впливає на клітини лише через Materials і Boundary.

---

# Joint Physics

Joint може працювати як просте distance constraint або spring-like connection.

базова модель:

```text id="6jup17"
Joint tries to keep cells within max_distance.
If stretch too high:
  joint damage increases.
```

Не потрібно одразу робити складний механічний solver.

---

# Damage

Physical damage — це зміна Materials.

```text id="94h9od"
high pressure
+
low material strength
    ↓
material degradation
```

Не вводити:

```text id="xqjlq5"
cell.hp -= damage
```

---

# Spatial Index

Physics Engine має підтримувати spatial index.

базова модель:

```text id="w242kh"
uniform grid
```

Використовується для:

* neighbor lookup;
* collision;
* resource lookup;
* Joint creation;
* local signals.

---

# Integration with Scheduler

Physics має виконуватися після Action Execution або у визначеній фазі Scheduler.

Рекомендований базова модель порядок:

```text id="7bdeiq"
Cell requests movement
Process Execution validates
Physics applies movement/collision
Lifecycle handles consequences
```

---

# Rules

## Rule 1. Physics is material-grounded

Mass, volume, pressure і damage мають походити з Resources і Materials.

## Rule 2. No HP

Фізична шкода — це Material damage.

## Rule 3. No all-to-all collision scan

Потрібен spatial index.

## Rule 4. Joint is physical

Joint має mechanical consequence і може рватися.

## Rule 5. базова модель physics must stay simple

Краще проста стабільна модель, ніж складний solver без користі.

---

# Заборонено

Не вводити:

* HP damage;
* hardcoded body movement;
* hardcoded muscle physics;
* global organism movement command;
* all-cells-to-all-cells collision checks;
* movement without Energy/Material cost;
* teleport movement.

---

# Як доопрацьовувати

Під час реалізації цей файл треба розширити, коли буде обрано:

* collision model;
* pressure formula;
* Joint constraint model;
* spatial grid size;
* movement integration method;
* damage thresholds.

---

# Open Questions

* Чи cell shape У базовій моделі — circle?
* Чи volume впливає на radius?
* Чи pressure буде Field, derived value або physics-only value?
* Чи Joint spring-like, чи distance constraint?
* Чи потрібен flow у Physics базова модель?

---



