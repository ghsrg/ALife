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
- Joint може деградувати, пошкоджуватися, ремонтуватися і розриватися.
- Joint після смерті клітини не зникає миттєво; він деградує або лишається матеріальним фрагментом.
- Heat transfer через Joint потребує heat_transfer capability/conductivity і підпорядковується `world/field-semantics.md`.

---

# Мінімальна модель

```text
Joint
├── cell_a
├── cell_b
├── strength
├── resource_transfer_rate
├── signal_value
├── signal_decay
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

Він або:

- reassigned до просторово валідної дочірньої клітини;
- breaks;
- becomes damaged fragment.

Новий Joint між дочірніми клітинами виникає лише через explicit process.

Після death клітини Joint може тимчасово лишатися фізичним об'єктом, проводити Heat або Resources і поступово деградувати.

---

# Заборонено

Не вводити:

- magic connection;
- direct Energy Buffer transfer;
- global organism bus;
- hardcoded nerve, vessel, muscle, tissue або organ;
- species_id compatibility;
- joint without Materials;
- joint without cost;
- instant disappearance after cell death.

---

# Open Questions

- Який default для division: reassign to closest valid daughter чи break external Joints unless maintained.
- Яка формула passive/active resource transfer.
- Чи signal delay входить у першу реалізацію.
- Які degradation rates використовуються після death.
- Чи HGT через Joint входить у першу реалізацію або лишається future extension.

---

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
