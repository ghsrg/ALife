# feasibility.md

> **Feasibility Check — єдиний contract перевірки можливості planned actions**

---

# Призначення

`feasibility.md` визначає, як planned action перевіряється перед виконанням.

Feasibility Check не виконує дію. Він відповідає, чи дія може бути виконана, і чому вона відхилена.

---

# Scope

Feasibility Check застосовується до active planned actions:

- movement;
- uptake/export;
- synthesis;
- repair;
- signal emission;
- Joint creation/upkeep/strengthening;
- genome copying;
- division preparation;
- division execution;
- controlled reactions.

Mandatory existence costs обробляються перед planned actions і описані в `world/energy.md`.

---

# FeasibilityResult

```text
FeasibilityResult
├── status: allowed | rejected
├── action_id
├── required_energy
├── required_resources
├── required_materials
├── required_space
├── blocked_by
├── failure_reasons
└── diagnostics
```

`allowed` означає, що дія може перейти до ProcessExecution.

`rejected` означає, що дія не виконується, не має часткового результату і не збільшує ProcessProgress.

---

# Failure Reason Taxonomy

Базові reasons:

- `insufficient_energy`
- `insufficient_resource`
- `insufficient_material`
- `insufficient_space`
- `boundary_damaged`
- `material_incompatible`
- `physics_blocked`
- `joint_blocked`
- `lifecycle_state_blocked`
- `invalid_genome_request`
- `conflict_unresolved`

Failure reason має бути явним, щоб debug/observer могли пояснити, чому клітина не живе, не росте або не ділиться.

---

# No World Mutation

Feasibility Check не змінює:

- Resources;
- Materials;
- Energy;
- Physics;
- Lifecycle;
- Genome;
- Joint;
- ProcessProgress.

---

# Rejected Action Rule

Rejected action:

- не списує cost;
- не створює output;
- не має partial final product;
- не збільшує ProcessProgress;
- може бути записана в diagnostics/trace.

---

# Process Contract

Усі active process execution повинні проходити через цей contract.

Process не може мати приховану власну логіку "можна / не можна", яка обходить Feasibility Check.

---

# Future Reservation Model

Resource/Energy reservation може бути додана пізніше.

У базовій моделі Feasibility Check дає diagnostics і status, але не резервує state наперед.

---

# Rules

## Rule 1. Feasibility is read-only

Feasibility Check не змінює world state.

## Rule 2. Rejection is explicit

Rejected action має explicit failure reasons.

## Rule 3. No partial result

Rejected planned action не має часткового результату.

## Rule 4. Genome cannot bypass feasibility

Genome output є regulatory intent, а не дозвіл на зміну світу.

---

# Пов'язані документи

- `biology/processes.md`
- `biology/process-progress.md`
- `genetics/regulatory-interface.md`
- `world/energy.md`
- `world/tick-semantics.md`

