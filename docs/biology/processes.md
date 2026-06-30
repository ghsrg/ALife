---
tags:
  - alife
  - canon
  - area/biology
---

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
├── process_id
├── kind
├── status: now | future
├── duration: atomic | long_running
├── required_capabilities
├── required_inputs
├── energy_cost
├── material_cost
├── output/effect
├── feasibility_rules
└── failure_modes
```

Повні per-process entries живуть у `biology/action-process-registry.md`.
`processes.md` описує логіку процесів і не дублює повний реєстр.

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
- Executable process має бути визначений у Action / Process Registry.
- Genome outputs є тільки priorities для registered allowed processes.
- Feasibility Check приймає тільки registered actions або controlled reactions.
- Future process groups можуть існувати у схемі, але не виконуються, доки не будуть явно enabled у registry.

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
HGT / genome integration
recombination
advanced learning/plasticity
fast signal conduction
complex joint remodeling
specialized structure growth
long-range sensing
multi-cell coordinated development
```

Future-compatible groups are schema placeholders only. They must not be allowed Genome outputs or Feasibility actions until explicitly enabled in `biology/action-process-registry.md`.

---

# Failure Modes

Failure modes мають бути визначені per-process або per-process-group:

```text
rejected_no_effect
partial_progress_not_added
progress_paused
progress_decayed
material_degraded
resource_wasted
heat_generated
cell_stressed
```

Base rule:

```text
Feasibility reject before execution -> no effect
failure during execution -> explicit consequences from process rule
```

Long-running process failure follows `biology/process-progress.md`, but each process must say whether progress is paused, decayed, discarded or converted into damage/waste/heat.

---

# Placeholder Formulas

Формули нижче задають мінімальний accounting contract, а не фінальну математику:

```text
transfer_amount =
  min(available, capacity_free, transfer_rate * modifier)

synthesis_output =
  min(input_resources * efficiency, process_capacity)

repair_amount =
  min(damage, repair_materials_available, energy_limited_repair)

degradation_amount =
  material_amount * decay_rate * environment_modifier

energy_gain =
  resource_amount * energy_value * conversion_efficiency
```

Energy production does not create matter. It only converts Resource potential through Material/process rules into Energy Buffer or Heat according to explicit accounting.

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
- process without physical inputs;
- future process executing as base behavior without registry enablement.

---

# Semantic Links

- consume: [[docs/world/resources|Resources]]
- consume: [[docs/world/energy|Energy]]
- require capabilities from: [[docs/biology/process-capabilities|Process Capabilities]]
- regulated by: [[docs/biology/genome|Genome]]
- gated by: [[docs/biology/feasibility|Feasibility Check]]
- tracked by: [[docs/biology/process-progress|Process Progress]]

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
