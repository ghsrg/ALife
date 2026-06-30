# tick-semantics.md

> **Tick Semantics — видимість стану, deltas і commit boundaries**

---

# Призначення

`tick-semantics.md` визначає semantic contract одного Tick.

`world/tick.md` описує концептуальний порядок світу. `engine/scheduler.md` описує виконання Systems. Цей документ визначає, які зміни коли стають видимими.

---

# Vocabulary

## Tick

Semantic time step світу.

## Phase

Semantic visibility boundary.

## System

Implementation unit рушія, який виконує частину логіки.

## Delta

Controlled write, який описує зміну стану, але не обов'язково одразу видимий іншим процесам.

## Commit

Момент, коли deltas застосовуються до committed state.

## Snapshot

Стабільний read state, який клітини або Systems можуть читати без same-phase race conditions.

---

# Phase Model

```text
Environment Phase
    ↓ commit environment snapshot
Decision Phase
    ↓ commit action plans
Execution Phase
    ↓ commit post-action state
Physics/Lifecycle Phase
    ↓ commit final state
Statistics Phase
```

Tick N starts from committed state N.

Environment Phase computes and commits environment snapshot N.

Decision Phase reads environment snapshot N plus committed cell/world state available at that phase.

Action Execution, Physics and Lifecycle produce committed state N+1.

## Environment Phase

Оновлює Fields, Resources, Traces та environment buffers.

Результат: committed environment snapshot.

Passive environment updates at the start of Tick N are part of preparing the readable environment snapshot for Tick N.

## Decision Phase

Клітини читають stable snapshot і пишуть лише Action Plans.

Decision Phase не змінює world state.

## Execution Phase

Systems читають Action Plans, виконують Feasibility Check, пишуть deltas, вирішують conflicts і commit-ять post-action state.

## Physics/Lifecycle Phase

Physics читає post-action state, пише physical/lifecycle deltas і commit-ить final state.

## Statistics Phase

Read-only phase для Observer Layer, snapshots, rendering, metrics.

---

# Same-Tick Visibility

Cell decision in Tick N may depend only on committed decision snapshot for Tick N.

Changes created by a cell during Decision Phase are not visible to any cell decision in the same Tick.

Signal emitted in Tick N becomes readable by other cells no earlier than Tick N+1, unless a specific phase-level exception is documented.

Same-tick phase commits are allowed only when explicitly defined.

Same-phase feedback is forbidden.

Decision behavior must not depend on iteration order, partial writes or uncommitted state.

---

# Conflict Resolution

No behavior-relevant result may depend on the order in which entities are iterated inside a System.

If multiple cells request the same Resource, Space, contact result or interaction target, allocation is resolved by deterministic conflict resolution, not by iteration order.

Stable id order can be one implementation detail, but the conflict rule must be explicit.

---

# Movement And Collision

Movement requests are collected first.

Physics resolves movement and collisions after collection.

No cell can observe intermediate movement of another cell during the same Decision Phase.

---

# Forbidden Implementation Leaks

Forbidden:

- unordered hash map iteration as behavior;
- system execution order changing biological outcome unless documented;
- cell directly mutating another cell during Decision Phase;
- same-tick signal ping-pong;
- statistics influencing cell decisions;
- renderer/UI changing simulation state;
- parallel race conditions.

---

# Rules

## Rule 1. Decisions read stable snapshots

Cell decisions never read partial same-phase updates.

## Rule 2. Writes are controlled

World mutation happens through deltas and commit boundaries.

## Rule 3. Scheduler implements semantics

`engine/scheduler.md` may choose efficient Systems, but must preserve this semantic contract.

## Rule 4. Observer is read-only

Statistics, rendering and observer metrics do not affect behavior during normal simulation.

## Rule 5. Changes are visible after phase commit

No entity may read uncommitted changes from the same phase.

---

# Пов'язані документи

- `world/tick.md`
- `engine/scheduler.md`
- `biology/feasibility.md`
- `biology/communication.md`
- `engine/ecs.md`
