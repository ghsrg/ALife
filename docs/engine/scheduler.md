# engine/scheduler.md

> **Scheduler — порядок виконання Systems у Tick**

---

# Призначення

Цей документ описує мінімальний порядок виконання Systems у межах одного Tick.

Tick — це модель часу світу.

Scheduler — це виконавча оптимізація рушія, яка визначає порядок Systems, snapshots, buffers і кешованих views.

Scheduler не є окремим законом світу. Його фази можуть не збігатися один-в-один із концептуальними фазами `world/tick.md`, але результат має зберігати семантику Tick.

Обов'язковий семантичний контракт Scheduler описаний у `world/tick-semantics.md`: stable snapshot, read-only Decision, delta collection, deterministic commit і заборона same-tick feedback loops.

Scheduler потрібен, щоб уникнути хаосу:

* same-tick feedback loops;
* випадкового порядку оновлення клітин;
* недетермінізму;
* ситуацій, де одна клітина реагує на дію іншої в тому самому Tick.

---

# Базова структура Scheduler

Орієнтовний порядок виконання Systems:

```text id="ocfl2s"
1. Environment Update
2. Cell Decision
3. Action Execution
4. Physics and Cleanup
5. Statistics and Snapshots
```

---

# Phase 1. Environment Update

Оновлюється середовище.

Systems:

```text id="4leu1y"
FieldSystem
WeatherSystem
ResourceSourceSystem
ResourceDiffusionSystem
ResourceDecaySystem
LocalHeatTransferSystem
TraceDecaySystem
```

Ця фаза готує snapshot, який клітини читатимуть під час Cell Decision.

Глобальний `HeatDiffusionSystem` можливий пізніше, якщо Heat стане окремим Field.

---

# Phase 2. Cell Decision

Клітини читають локальний snapshot.

Systems:

```text id="by2bf9"
CellInputSystem
GenomeRuntimeSystem
EpigeneticModifierSystem
DecisionBufferSystem
```

Результат:

```text id="f1ji2a"
CellActionPlan
ProcessPriorities
RequestedActions
```

У цій фазі заборонено напряму змінювати світ.

Клітина лише формує план.

---

# Phase 3. Action Execution

Виконуються заплановані дії.

Systems:

```text id="5f3qsi"
ProcessExecutionSystem
EnergySystem
MaterialSynthesisSystem
BoundarySystem
JointCreationSystem
SignalEmissionSystem
ResourceUptakeSystem
ResourceExportSystem
DivisionSystem
MutationSystem
```

Кожна дія проходить Feasibility Check.

```text id="mxskf6"
requested action
+
Resources
+
Materials
+
Energy
+
Space
+
Lifecycle
    ↓
executed or rejected
```

---

# Phase 4. Physics and Cleanup

Оновлюється фізичний стан.

Systems:

```text id="6tp89i"
PhysicsSystem
JointSystem
CollisionSystem
PressureSystem
DamageSystem
LifecycleSystem
DecompositionSystem
SpatialIndexSystem
```

Тут можуть:

* рухатися клітини;
* розриватися Joint;
* виникати pressure;
* пошкоджуватись Materials;
* помирати клітини;
* оновлюватися spatial index.

---

# Phase 5. Statistics and Snapshots

Оновлюється аналітика.

Systems:

```text id="7jnt55"
StatisticsSystem
LineageTrackingSystem
PopulationMetricsSystem
OrganismViewSystem
SnapshotSystem
EventLogSystem
```

Ця фаза не повинна змінювати поведінковий state клітин.

---

# Double Buffering

Scheduler повинен підтримувати double buffering.

```text id="xi8zpf"
read_state_current
write_state_next
```

Клітини читають стабільний стан.

Нові зміни стають видимі в наступній фазі або наступному Tick згідно з правилами.

---

# Same-tick Feedback

Заборонено:

```text id="x8qgwi"
Cell A emits signal.
Cell B reads it immediately.
Cell B emits response.
Cell A reads response in same Tick.
```

Дозволено:

```text id="o8sxud"
Tick N:
  Cell A emits signal.

Tick N+1:
  Cell B reads signal.
```

Якщо потрібні винятки, вони мають бути явно описані.

---

# Deterministic Order

Оновлення повинно бути deterministic.

Рекомендація:

```text id="9bb46c"
sort entities by stable id before processing
use seeded RNG
avoid iteration order from hash maps
```

Це критично для reproducibility.

---

# Conflict Resolution

Якщо кілька клітин претендують на один Resource або простір, потрібне стабільне правило.

Приклади:

```text id="6x97le"
deterministic order by entity_id
proportional allocation
randomized but seeded order
```

Базова реалізація може використовувати deterministic order.

Пізніше можна додати більш фізичне proportional allocation.

---

# Action Plan

Cell Decision створює Action Plan.

```text id="c6ehpt"
ActionPlan
├── process_priorities
├── movement_request
├── uptake_requests
├── synthesis_requests
├── repair_requests
├── signal_outputs
├── joint_requests
└── division_request
```

Action Plan — тимчасовий об'єкт.

Він не є спадковим.

---

# Базовий порядок Scheduler

Мінімальний порядок для базової моделі:

```text id="a4nx18"
1. Update Fields
2. Diffuse / decay Resources
3. Build Cell Inputs
4. Run Genome Runtime
5. Execute Cell Processes
6. Update Energy / Materials / Boundary
7. Handle Division and Mutation
8. Update Joints
9. Run Physics
10. Update Lifecycle / Death / Decomposition
11. Update Statistics
12. Save Snapshot if needed
```

---

# Правила

## Rule 1. Decision and execution are separated

Клітина спочатку вирішує, потім дія виконується окремою фазою.

## Rule 2. Cells read stable snapshots

Cell Decision не читає напівоновлений світ.

## Rule 3. No same-tick infinite loops

Сигнали й реакції не повинні нескінченно ходити в одному Tick.

## Rule 4. Scheduler must be deterministic

Однаковий seed і state мають давати однаковий результат.

## Rule 5. Analytics do not affect behavior

Statistics і views не змінюють клітинний state.

---

# Заборонено

Не вводити:

* unordered updates;
* клітини, які напряму змінюють інші клітини під час Decision;
* same-tick signal feedback loops;
* random iteration without seed;
* statistics as behavior input;
* organism-level command phase.

---

# Open Questions

* Чи Joint transfer відбувається до або після Cell Process Execution у конкретній реалізації Scheduler, якщо це не порушує `world/tick-semantics.md`?
* Чи Physics має бути до або після LifecycleSystem у конкретній реалізації Scheduler?
* Яка deterministic conflict resolution стратегія потрібна для першої реалізації: stable id, proportional allocation або seeded arbitration?

---



