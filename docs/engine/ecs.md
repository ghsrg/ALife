# engine/ecs.md

> **ECS — базова архітектура Entity / Component / System**

---

# Призначення

Цей документ описує мінімальну ECS-архітектуру рушія.

Мета ECS — відокремити:

* дані;
* системи обробки;
* біологічну логіку;
* фізику;
* конфігурацію;
* аналітику.

ECS потрібен, щоб агент під час реалізації не створив монолітний `Cell` або `Organism`, який керує всім світом.

---

# Основна ідея

```text id="2hsr7d"
Entity = id
Component = data
System = logic
```

Entity не містить поведінки.

Component не виконує логіку.

System читає Components і змінює world state згідно з Tick schedule.

---

# Основні Entity

Для базової моделі достатньо:

```text id="9k1olc"
CellEntity
ResourcePatchEntity
JointEntity
FieldLayerEntity
```

Future-compatible:

```text id="7mhzm9"
GeneticFragmentEntity
OrganismView
TraceEntity
```

`OrganismView` не є активним керуючим Entity.

Це derived analytical view над Cells + Joints.

Observer/analytics views є read-only щодо поведінки клітин: їх не можна подавати як Genome Runtime input або використовувати як organism-level command.

---

# CellEntity

CellEntity — основна одиниця життя.

Можливі Components:

```text id="2la8le"
PositionComponent
PhysicsComponent
ResourceStorageComponent
MaterialStorageComponent
EnergyBufferComponent
GenomeComponent
BoundaryComponent
LifecycleComponent
SignalStateComponent
EpigeneticStateComponent
LineageComponent
```

CellEntity не повинен мати hardcoded тип:

```text id="jjm92p"
NeuronCell
MuscleCell
PlantCell
AnimalCell
PredatorCell
PreyCell
```

---

# ResourcePatchEntity

ResourcePatchEntity або Resource Layer описує локальну кількість Resource у світі.

Можливі Components:

```text id="yshwv2"
PositionComponent
ResourceAmountComponent
DiffusionComponent
DecayComponent
```

Для grid-based світу Resources можуть бути не Entity, а шарами grid.

Це рішення реалізації, але логічно Resource все одно не є Material і не є Energy.

---

# JointEntity

JointEntity описує зв'язок між клітинами.

Components:

```text id="o2sjj8"
JointComponent
JointMaterialComponent
JointSignalComponent
JointTransportComponent
JointDamageComponent
```

JointEntity не є нервом, судиною або органом.

Це універсальний матеріальний зв'язок між клітинами.

Один JointEntity може мати кілька каналів (`mechanical`, `resource`, `signal`, `heat`) як компоненти одного Joint.

---

# FieldLayerEntity

Fields можуть бути реалізовані як grid layers або функції.

Приклади:

```text id="qqygq9"
LightField
PressureField
RadiationField
FlowField
```

`HeatField` не входить до базової моделі. На поточному рівні клітина має локальну temperature, а Heat передається лише через контакт або Joint.

Field не є Resource.

Field не є Material.

Field не є Energy Buffer.

---

# Мінімальні Components для базової моделі

```text id="i2ahk9"
PositionComponent
PhysicsComponent
ResourceStorageComponent
MaterialStorageComponent
EnergyBufferComponent
GenomeComponent
BoundaryComponent
LifecycleComponent
JointComponent
SignalStateComponent
LineageComponent
```

---

# Мінімальні Systems для базової моделі

```text id="1znrfq"
FieldSystem
ResourceDiffusionSystem
CellDecisionSystem
ProcessExecutionSystem
EnergySystem
MaterialSystem
BoundarySystem
JointSystem
PhysicsSystem
LifecycleSystem
MutationSystem
StatisticsSystem
```

---

# System Responsibilities

## FieldSystem

Оновлює Fields:

* light;
* heat;
* pressure;
* radiation;
* flow.

Не змінює клітини напряму.

---

## ResourceDiffusionSystem

Оновлює Resources у середовищі:

* diffusion;
* decay;
* reactions;
* source/sink updates.

---

## CellDecisionSystem

Читає snapshot стану клітини й середовища.

Запускає Genome Runtime.

Формує action plan або process priorities.

Не виконує фізичні дії напряму.

---

## ProcessExecutionSystem

Виконує клітинні процеси:

* uptake;
* synthesis;
* repair;
* movement request;
* division preparation;
* signal output;
* dormancy;
* resource export.

Перевіряє Feasibility:

```text id="2x68s1"
Resources
Materials
Energy
Space
Lifecycle State
```

---

## JointSystem

Оновлює Joint:

* transfer Resources;
* transfer Signals;
* transfer Heat;
* apply damage;
* repair;
* break if needed.

---

## PhysicsSystem

Оновлює:

* movement;
* collision;
* pressure;
* deformation;
* Joint constraints;
* spatial index.

---

## LifecycleSystem

Оновлює:

* alive;
* stressed;
* dormant;
* dividing;
* dead;
* decomposing.

Death не є HP event.

Death — це structural/material failure.

---

## MutationSystem

Застосовує mutation під час copying, damage, repair або інших дозволених подій.

Mutation не є fitness-guided.

---

## StatisticsSystem

Рахує аналітику:

* cell count;
* population metrics;
* lineage metrics;
* organism-like components;
* resource totals;
* mutation events.

Statistics не повинна керувати клітинами.

---

# Правила

## Rule 1. Entity has no behavior

Entity — це id і набір Components.

## Rule 2. Component is data

Component не повинен містити складну логіку.

## Rule 3. System owns logic

Логіка Tick виконується Systems.

## Rule 4. No hardcoded biological classes

Не вводити класи для видів, органів, тканин або спеціалізованих клітин.

## Rule 5. Analytical views do not control behavior

Observed roles, organism-like labels і species-like clusters не можуть бути input для Genome Runtime.

---

# Заборонено

Не вводити:

* `Organism` як глобальний контролер;
* `Cell` як монолітний клас з усією логікою;
* `species_id` для поведінки;
* `fitness_score` як input;
* hardcoded organs;
* hardcoded cell roles;
* direct Energy sharing.

---

# Open Questions

* Чи Resources будуть Entity, grid layer або hybrid?
* Чи Fields будуть Entity, grid layer або function-based?
* Чи OrganismView буде кешованим, чи обчислюватиметься на вимогу?
* Яка мінімальна структура Components для першого vertical slice?

---


