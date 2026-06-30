---
tags:
  - alife
  - canon
  - area/biology
---

# joint.md

> `Joint` — матеріальний локальний зв'язок між клітинами.

---

# Призначення

`Joint` дозволяє клітинам утворювати connected structures, передавати локальні впливи й підтримувати багатоклітинність без global organism controller.

Joint має фізичну або матеріальну основу. Він не є нервом, судиною, органом, тканиною або магічним каналом.

---

# Канонічні правила

- Joint створюється тільки локально: через контакт, близькість або вже наявну фізичну структуру.
- Joint має матеріальну основу і cost створення, підтримки та ремонту.
- Один Joint object може мати кілька каналів: `mechanical`, `resource`, `signal`, `heat`.
- Канали є параметрами одного Joint, а не окремими сутностями, якщо немає окремої фізичної причини.
- Joint може передавати Resources, Signals, Heat і mechanical force.
- Joint не передає `Energy Buffer` напряму.
- У першій реалізації Joint Resource transfer є passive-only.
- Joint може передавати Resources тільки через explicit `resource_channel`.
- Joint не передає Genome, Genome fragments або heritable information у першій реалізації.
- Joint може деградувати, пошкоджуватися, ремонтуватися і розриватися.
- Joint після смерті клітини не зникає миттєво; він деградує або лишається матеріальним фрагментом.
- Heat transfer через Joint потребує heat_transfer capability/conductivity і підпорядковується `world/field-semantics.md`.
- Signal transfer через Joint підпорядковується `biology/communication.md`: signal emitted during Tick N becomes readable input at Tick N+1.

---

# Мінімальна модель

```text
Joint
├── cell_a
├── cell_b
├── strength
├── resource_channel
├── resource_transfer_rate
├── max_transfer_per_tick
├── signal_level
├── signal_decay
├── signal_readable_from_tick
├── heat_conductivity
├── damage
└── active
```

Мінімальна update-логіка:

```text
1. Apply mechanical constraint.
2. Transfer Resources if allowed.
3. Transfer Signal if present.
4. Transfer Heat if enabled.
5. Apply degradation.
6. Apply repair if requested and feasible.
7. Break if damage or stretch exceeds threshold.
```

Порядок має бути узгоджений з `world/tick.md`, `world/tick-semantics.md` і `engine/scheduler.md`.

---

# Resource Transfer

У першій реалізації Joint підтримує тільки passive Resource transfer, якщо Joint має `resource_channel`.

```text
passive_transfer =
  min(
    gradient * transfer_rate * joint_integrity,
    max_transfer_per_tick,
    available_source_amount,
    free_target_capacity
  )
```

```text
gradient = max(0, source_concentration - target_concentration)
```

Resources move from higher local concentration to lower local concentration.

Joint does not transfer Energy Buffer directly.

Active directed transfer is post-MVP/future extension. It requires:

```text
Genome priority
ActionPlan
Feasibility Check
Energy cost
resource_channel
transport-capable Material
```

Invariant:

```text
Joint may transfer Resources only through an explicit channel.
Energy Buffer is never transferred through Joint.
```

---

# Creation

Joint creation можлива, якщо:

```text
cells are close enough
compatible Boundary materials exist
Energy is sufficient
Resources are sufficient
Genome output requests joint creation
physical space allows connection
Feasibility Check passes
```

Joint не створюється дистанційно.

---

# Division and Death

Під час division existing Joint не дублюється.

Default:

```text
parent cell divides
  -> external Joints break
```

Reassigning Joints to the closest daughter is more realistic, but it introduces spatial ambiguity and can preserve multicellular structures too magically.

Future extension may allow:

```text
reassign to closest valid daughter
```

but only if:

```text
distance is valid
anchor remains physically possible
Joint material survives division
reassignment rule is explicitly enabled
```

Invariant:

```text
Division does not duplicate or preserve Joints by default.
Joint preservation during division requires an explicit rule.
```

Новий Joint між дочірніми клітинами виникає лише через explicit process.

Після endpoint death активні канали Joint зупиняються одразу. Фізичний Joint може лишатися inert material connection і деградувати за матеріальними правилами.

```text
cell endpoint dies
  -> active channels disabled
  -> Joint becomes inert or unstable
  -> degradation follows joint_material.decay_rate
```

Recommended behavior:

```text
if one endpoint dead:
  disable resource/signal transfer
  keep physical Joint for decay_ticks if material allows

if both endpoints dead:
  Joint becomes MaterialFragment or breaks during decomposition
```

Degradation source:

```text
joint_material.decay_rate
environment modifiers
heat / pressure / reaction damage
missing upkeep if upkeep exists
```

Invariant:

```text
Death does not require instant Joint deletion.
Death stops living transfer.
Remaining Joint material degrades as physical material.
```

---

# HGT

HGT through Joint is not part of the first implementation.

```text
HGT through Joint = disabled
```

Future HGT through Joint may require:

```text
genetic_fragment_channel
physical genetic fragment
compatible carrier
uptake/integration process
Feasibility Check
Energy and material cost
mutation/integration rules
```

Invariant:

```text
Joint cannot transfer Genome, Genome fragments or heritable information in MVP.
Any future HGT through Joint must be explicit and must not reuse normal resource transfer silently.
```

---

# Summary

```text
Joint is a simple physical connection between two cells:
- binary exists/broken state
- optional passive resource channel
- no direct Energy transfer
- no HGT
- no automatic preservation during division
- death disables active transfer
- remaining material may decay or become MaterialFragment
```

---

# Заборонено

Не вводити:

- magic connection;
- direct Energy Buffer transfer;
- Genome, Genome fragments or heritable information transfer in first implementation;
- global organism bus;
- hardcoded nerve, vessel, muscle, tissue або organ;
- species_id compatibility;
- joint without Materials;
- joint without cost;
- instant disappearance after cell death.

# Semantic Links

- connects: [[docs/biology/cell|Cells]]
- made from: [[docs/world/materials|Materials]]
- transfers: [[docs/world/resources|Resources]]
- transfers heat under: [[docs/world/field-semantics|Field Semantics]]
- carries signals for: [[docs/biology/communication|Communication]]
- constrained by: [[docs/world/physics|Physics]]

# Пов'язані документи

- `biology/cell.md`
- `biology/division-partition.md`
- `biology/communication.md`
- `biology/processes.md`
- `biology/organism.md`
- `world/materials.md`
- `world/resources.md`
- `world/energy.md`
- `world/physics.md`
- `world/tick.md`
- `docs/examples/biology-examples.md`
