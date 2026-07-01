---
tags:
  - alife
  - worklog/plan
---

# PLAN: p0-accepted-clarifications

Дата: 2026-06-30 14:57

Джерело: user clarification for P0 items after `outputs/worklogs/2026-06-30-1405-PLAN-physics-logic-repeat-audit.md`.

Мета: зафіксувати прийняті P0-рішення перед внесенням правок у Canon.

---

# Scope

Виправляти тільки P0:

1. Energy Buffer capacity vs volume.
2. Genome copying material basis.
3. Phase-based Tick causality.
4. Mandatory costs before planned action Feasibility.

P1/P2 з попереднього аудиту не включати в цей крок.

---

# P0. Energy Buffer capacity vs volume

## Проблема

`docs/PRINCIPLES.md` досі каже, що Energy займає внутрішній об'єм. Поточний Canon каже, що Energy Buffer не є речовиною і не займає volume напряму.

## Вплив

Суперечність на найвищому рівні правил може змусити реалізацію трактувати Energy Buffer як Resource/Material або як окрему речовину.

## Пропозиція

Оновити `docs/PRINCIPLES.md`.

Зафіксувати:

```text
Energy Buffer is a local state of a Cell.
It is not a Resource, not a Material, and not a transferable substance.
Energy Buffer does not directly occupy internal volume.
Energy capacity is provided by storage-capable Materials, and those Materials occupy volume inside the Cell.
A Cell cannot store unlimited Energy.
If a future model needs an energy-rich substance, it must be represented as a Resource or Material, not as Energy Buffer itself.
```

Перевірити після правки:

- `docs/PRINCIPLES.md`;
- `docs/world/energy.md`;
- `docs/biology/cell.md`;
- `docs/world/units.md`;
- `docs/GLOSSARY.md`.

---

# P0. Genome copying material basis

## Проблема

Genome copy описаний як copied/not partitioned, але фізичне джерело матерії для другої копії недостатньо зафіксоване.

## Вплив

Ризик порушення conservation of matter: функціональна Genome copy може виникати з Energy cost без physical carrier.

## Пропозиція

Оновити:

- `docs/biology/division-partition.md`;
- `docs/genetics/inheritance.md`;
- `docs/biology/genome.md`.

Зафіксувати:

```text
Genome is not pure information.
Genome consists of:
  Genome information
  physical genome carrier
  genome runtime machinery

Genome information is not a substance.
The genome carrier is a physical structure made from Resources/Materials/internal precursor fragments.
Genome is not a Material category like boundary/strength/movement material.
Genome has physical carrier made from Materials/Resources.

Genome copying is a long-running controlled process.
Energy powers copying, repair and organization, but does not create matter.
A Genome copy is synthesized from configured Resources, Materials or internal precursor fragments according to recipe/cost.
```

Division invariant:

```text
No daughter Cell receives a functional Genome unless a physical Genome copy or explicitly valid inherited carrier exists before partition.
```

Allowed daughter outcomes:

```text
complete functional Genome
damaged Genome
incomplete Genome
nonfunctional Genome carrier
no viable Genome copy
```

---

# P0. Phase-based Tick causality

## Проблема

Старий закон "зміни Tick N впливають лише на Tick N+1" конфліктує з моделлю `Environment Update -> Decision` в одному Tick.

## Вплив

Без уточнення незрозуміло, чи passive environment updates на старті Tick N видимі клітинам у Decision Phase Tick N.

## Пропозиція

Оновити:

- `docs/world/laws.md`;
- `docs/world/tick.md`;
- `docs/world/tick-semantics.md`;
- `docs/engine/scheduler.md`.

Прийнята модель:

```text
Tick N starts from committed state N.
Environment Phase computes and commits environment snapshot N.
Decision Phase reads environment snapshot N.
Action Execution, Physics and Lifecycle produce committed state N+1.
```

Corrected causality law:

```text
Changes are visible only after their phase commit.
No entity may read uncommitted changes from the same phase.
Cell decisions in Tick N read only the committed environment snapshot N and committed cell/world state available at Decision Phase.
```

Signal rule:

```text
A signal emitted in Tick N is not visible to other cell decisions until Tick N+1,
unless a future explicit fast-conduction extension defines a different phase rule.
```

Invariant:

```text
Same-tick phase commits are allowed only when explicitly defined.
Same-phase feedback is forbidden.
Decision behavior must not depend on iteration order, partial writes or uncommitted state.
```

---

# P0. Mandatory costs before planned action Feasibility

## Проблема

Документи місцями описують Feasibility перед mandatory costs, тоді як Genome Runtime і Energy model очікують post-mandatory available state.

## Вплив

Planned actions можуть бути allowed на Energy/Resources, які потрібні для mandatory existence. Це створює hidden retry або order-dependent execution.

## Пропозиція

Оновити:

- `docs/engine/scheduler.md`;
- `docs/biology/feasibility.md`;
- `docs/world/energy.md`;
- `docs/world/tick.md`;
- за потреби `docs/genetics/genome-runtime.md`.

Прийнятий порядок:

```text
1. Compute mandatory costs.
2. Apply/pay mandatory costs.
3. If mandatory costs cannot be paid, mark cell as stalled, damaged, degrading, dormant or inert according to lifecycle rules.
4. Compute post-mandatory available state.
5. Run Feasibility Check for planned actions against post-mandatory state.
6. Execute only the planned action set that is affordable and feasible as a whole.
```

Mandatory costs:

```text
Mandatory costs are the cost of remaining an alive cell in current local conditions.
They include minimal boundary stability, basic internal stability, Genome Runtime baseline cost if configured, and other required existence costs.
```

Feasibility rule:

```text
available_for_planned_actions =
  committed_state_after_mandatory_costs
```

Invariant:

```text
Mandatory costs are paid before planned action Feasibility.
Planned actions cannot consume Energy or Resources required for mandatory existence.
Feasibility must not approve an action set using pre-mandatory state.
```

---

# Verification After Implementation

Після внесення правок перевірити:

```powershell
rg -n "займає внутрішній об.?єм|occupy internal volume|Energy.*volume" docs/PRINCIPLES.md docs/world/energy.md docs/biology/cell.md docs/world/units.md docs/GLOSSARY.md
rg -n "Genome copying|physical Genome copy|physical genome carrier|Energy.*source of matter|functional Genome" docs/biology/division-partition.md docs/genetics/inheritance.md docs/biology/genome.md
rg -n "Tick N|phase commit|Environment Phase|Decision Phase|same-phase|same-tick" docs/world/laws.md docs/world/tick.md docs/world/tick-semantics.md docs/engine/scheduler.md
rg -n "Mandatory costs|post-mandatory|planned action Feasibility|available_for_planned_actions" docs/engine/scheduler.md docs/biology/feasibility.md docs/world/energy.md docs/world/tick.md docs/genetics/genome-runtime.md
```

Також перевірити, що не повернулися:

```text
Energy as Resource
Energy as Material
Genome copy from Energy only
same-phase feedback
Feasibility using pre-mandatory state
```
