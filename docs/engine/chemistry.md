# docs/engine/chemistry.md

> **Chemistry Engine — базовий шар реакцій між Resources, Materials і середовищем**

---

# Призначення

`chemistry.md` описує, за що відповідає хімічний шар рушія.

Цей файл не задає повну хімічну модель.

Його задача — пояснити агенту, що reactions мають бути універсальними, конфігурованими і не повинні перетворюватися на hardcoded біологію.

---

# Основна ідея

Chemistry Engine працює з:

* Resources;
* Materials;
* decay;
* reactions;
* synthesis inputs;
* degradation products;
* Heat release;
* Energy-related resource conversion;
* environment modifiers.

```text id="c1r9ws"
Resources
+
Materials
+
Fields / Heat / Conditions
    ↓
Reaction / Decay / Transformation
    ↓
Resources / Material Damage / Heat
```

---

# Що Chemistry Engine НЕ є

Chemistry Engine не є:

* реалістичним молекулярним симулятором;
* hardcoded списком речовин;
* системою poison/heal;
* прямим джерелом Energy Buffer;
* прямим HP damage;
* поведінковою логікою клітини;
* біологічним органом.

---

# MVP Scope

Для MVP достатньо:

```text id="ti0rls"
Resource decay
Resource diffusion modifiers
simple Resource reactions
Material degradation
Material synthesis cost validation
Heat release from reactions
Resource -> Energy conversion support
```

Не потрібно в MVP:

```text id="p9z1po"
complex chemistry
full reaction networks
molecular simulation
real pH model
enzymes as hardcoded classes
metabolic pathways
```

---

# Reaction Model

Реакція має бути config-driven.

Приклад:

```yaml id="yedq1u"
reaction:
  inputs:
    nutrient_A: 1.0
    oxidizer_A: 0.5
  outputs:
    inert_waste: 0.6
  energy_release: 0.3
  heat_release: 0.1
  probability: 0.4
```

Реакція не повинна запускатися магічно.

Вона залежить від:

* локальної наявності Resources;
* Materials;
* Heat;
* Field conditions;
* probability;
* seed;
* process rules.

---

# Resource Decay

Resource може розпадатися з часом.

```text id="wh2dgv"
Resource amount
    ↓
decay
    ↓
decay products
```

Decay не повинен порушувати conservation of matter, якщо тільки config явно не описує втрату за межі світу або абстрактну dissipated loss.

---

# Material Degradation

Material damage — це втрата або зміна Material.

```text id="q3nd9v"
Heat / Pressure / Reaction
    ↓
Material stability decreases
    ↓
Material amount or quality decreases
```

Не вводити HP.

---

# Energy

Chemistry може підтримувати Energy conversion, але не створює Energy Buffer напряму.

Правильно:

```text id="cscwh4"
Resource with energy_value
+
conversion-capable Material
+
cell process
    ↓
Energy Buffer increases
```

Неправильно:

```text id="j8lyhm"
reaction.energy_release -> magically add to any nearby cell
```

---

# Integration with Config

Chemistry Engine читає:

```text id="esqads"
resources_config.md
materials_config.md
fields_config.md
world_config.md
```

Типи Resources і Materials не повинні бути зашиті в код.

---

# Rules

## Rule 1. Chemistry is config-driven

Нові Resources, Materials і reactions додаються через config.

## Rule 2. No toxicity field

Шкідливість виникає через reactions, blocking, Heat, Material damage, decay products або volume.

## Rule 3. No direct HP damage

Damage — це зміна Materials, Boundary, Joint або Resources.

## Rule 4. No direct Energy Buffer magic

Energy Buffer змінюється лише через клітинні процеси й допустимі конверсії.

## Rule 5. Chemistry must be deterministic with seed

Stochastic reactions мають бути відтворюваними.

---

# Заборонено

Не вводити:

* poison_damage;
* healing_resource;
* direct cell kill reaction;
* hardcoded food;
* hardcoded oxygen;
* hardcoded sugar;
* hardcoded toxin;
* direct Energy creation;
* real-world chemistry dependency in MVP.

---

# Як доопрацьовувати

Під час реалізації цей файл треба розширити, коли з’являться:

* перша схема reactions;
* формула decay;
* правила heat release;
* material degradation model;
* test cases для conservation;
* рішення: grid-based чи entity-based reactions.

---

# Open Questions

* Чи reactions відбуваються в environment, всередині cell, або в обох місцях?
* Чи потрібна reaction priority?
* Чи треба reaction queue?
* Як обмежити computational cost reactions?
* Чи потрібні catalysts як Material property, а не hardcoded enzyme?

---

