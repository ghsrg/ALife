# processes.md

> `Process` — локальне перетворення стану клітини або її взаємодія з середовищем.

---

# Призначення

Processes описують доменні дії клітини. Cross-cutting контракти винесені окремо:

- `biology/action-process-registry.md` — canonical process ids, Genome output bindings, Feasibility scope і duration.
- `biology/feasibility.md` — перевірка active planned actions.
- `biology/process-progress.md` — progress довгих процесів без часткового фінального результату.
- `biology/process-capabilities.md` — `Materials -> Capabilities -> allowed process set`.
- `world/reactions.md` — passive і controlled reactions.

---

# Process Contract

```text
Process
├── inputs
├── required_materials
├── required_energy
├── required_conditions
├── duration
├── outputs
├── side_effects
└── failure_mode
```

Genome Runtime задає priorities. Feasibility Check вирішує, чи може active process виконатися.

---

# Канонічні правила

- Process змінює стан клітини або локального середовища.
- Active process потребує Energy Buffer, materials/capabilities і regulatory priority.
- Passive process виникає через physics, diffusion, chemistry або degradation.
- Active process не виконується, якщо Energy недостатньо для повного виконання.
- Якщо у Tick Energy недостатньо для всього набору planned actions, planned actions не виконуються в цьому Tick.
- Mandatory costs обробляються окремо від planned actions.
- Process не може створити Resource, Material або Energy без визначеного механізму.
- Damage є material/physical state change, а не HP.
- Waste є Resource/Product state, а не окрема сутність.

---

# Process Groups

Canonical executable process ids живуть у `biology/action-process-registry.md`.

Conceptual groups:

```text
passive diffusion
passive uptake/leakage
active uptake/export
resource reaction
energy production/consumption
material synthesis
material repair/degradation
maintenance
growth
genome copying
division preparation
death/decomposition
```

Future-compatible conceptual groups:

```text
movement
joint creation/maintenance/breaking
signal adaptation
trace emission
HGT-related uptake
complex communication
```

---

# Failure Modes

Якщо process не може виконатися, він може:

- не стартувати;
- втратити progress згідно з `process-progress.md`;
- створити side effect;
- пошкодити Materials;
- створити Heat;
- перевести клітину в `stressed` або `dormant`.

Failure mode має бути описаний per-process або per-process-group.

---

# Заборонено

Не вводити:

- hardcoded behavior scripts;
- HP repair;
- poison damage поза reactions/material effects;
- magic detox;
- free Energy;
- direct species recognition;
- direct organism-level command;
- process without physical inputs.

---

# Open Questions

- Остаточні details per registry entry у `biology/action-process-registry.md`.
- Per-process failure modes.
- Формули для transfer, synthesis, repair, degradation і Energy production.
- Які future-compatible process groups залишити тільки в схемі.

---

# Пов'язані документи

- `biology/cell.md`
- `biology/feasibility.md`
- `biology/process-progress.md`
- `biology/process-capabilities.md`
- `biology/lifecycle.md`
- `biology/joint.md`
- `world/reactions.md`
- `world/energy.md`
- `world/materials.md`
- `world/resources.md`
- `engine/scheduler.md`
