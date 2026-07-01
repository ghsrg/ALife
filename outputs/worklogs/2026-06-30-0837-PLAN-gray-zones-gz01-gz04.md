---
tags:
  - alife
  - worklog/plan
---

# PLAN: gray zones GZ-01..GZ-04 documentation additions

Date: 2026-06-30 08:37

## Goal

Підготувати план доповнення документації за відповідями на перші чотири сірі зони:

- GZ-01 Units and scales
- GZ-02 Tick-to-Scheduler semantic contract
- GZ-03 Feasibility Check contract
- GZ-04 Mandatory costs vs planned actions

Цей файл є накопичувальним робочим планом. Canon-документи не змінюються до команди "виконуй".

## Accepted Direction From User Answers

### GZ-01

Створити `docs/world/units.md`.

Прийняти hybrid unit model:

- Simulation units: space, amount, volume, mass, energy, time.
- Normalized units: fields, signals, material properties, efficiencies, probabilities.
- Per-Tick rates: diffusion, decay, maintenance, process rates.

Зафіксувати baseline ranges:

- Cell radius: `1.0..2.0 su`
- Cell volume capacity: `10.0 vu`
- Cell energy capacity: `10.0 eu`
- Resource packet: `0.1..1.0 au`
- Resource volume per unit: `0.5..2.0 vu`
- Resource density: `0.5..3.0`
- Resource energy value: `0.0..2.0 eu/au`
- Material amount: `0.1..10.0 mu`
- Material volume per unit: `0.5..2.0 vu`
- Material synthesis energy cost: `0.1..2.0 eu`
- Passive maintenance: `0.001..0.05 eu/tick`
- Active action: `0.05..1.0 eu`
- Division: `3.0..8.0 eu`
- Field value: `0.0..1.0`
- Decay rate: `0.0001..0.05 per Tick`
- Diffusion rate: `0.01..0.5 per Tick`

### GZ-02

Створити `docs/world/tick-semantics.md`.

Прийняти hybrid execution semantics:

- Tick = semantic time step.
- Phases = semantic visibility boundaries.
- Systems = implementation units.
- Deltas = controlled writes.
- Commit = only at defined boundaries.

Phases:

- Environment Phase: update fields/resources into environment buffers, commit environment snapshot.
- Decision Phase: cells read stable snapshot, write action plans only, no world mutation.
- Execution Phase: systems read action plans, write deltas, resolve conflicts, commit post-action state.
- Physics/Lifecycle Phase: physics reads post-action state, writes physical/lifecycle deltas, commit final state.
- Statistics Phase: read-only.

Core invariant:

```text
No behavior-relevant result may depend on the order in which entities are iterated inside a System.
```

### GZ-03

Створити `docs/biology/feasibility.md`.

Прийняти модель:

- feasibility + graded diagnostics;
- reservation model лише як future option, не зараз.

Core invariants:

1. Feasibility Check не змінює world state.
2. Rejected action не має часткового результату.
3. Failure reason має бути явним.
4. Усі process execution проходять через єдиний Feasibility contract.
5. Process не може мати приховану власну логіку "можна / не можна".

Diagnostics мають відповідати на питання:

- не вистачає Energy?
- не вистачає Resource?
- немає Space?
- Boundary пошкоджена?
- Material incompatible?
- Genome просить неможливі actions?

### GZ-04

Уточнити `mandatory costs` як cost of existing, а `planned actions` як cost of doing.

Core invariant:

```text
Mandatory cost is the cost of remaining an alive cell in the current world conditions.
It is charged before any planned action.
All actions that create change, growth, movement, repair, communication, reproduction or specialization are planned actions.
```

Mandatory мінімально включає:

- basic boundary stability;
- basic internal material stability;
- minimal homeostasis / leakage compensation.

Mandatory не включає:

- movement;
- uptake;
- synthesis;
- repair;
- signal emission;
- joint creation;
- joint strengthening;
- genome copying;
- division preparation;
- specialization;
- active transport;
- resource export.

Joint rule:

- joint passive decay = environment/material process;
- joint repair/upkeep = planned action;
- unsupported Joint degrades or breaks.

Failure flow:

```text
Energy at Tick start
    ↓
pay existence cost
    ↓
if paid:
  evaluate planned actions
else:
  skip planned actions
  apply survival failure consequences
```

## Files To Create

### `docs/world/units.md`

Responsibility:

- define all canonical simulation units;
- distinguish simulation units, normalized units, per-Tick rates;
- provide baseline ranges and interpretation rules;
- define what is a hard bound, soft recommended range, and scenario-specific range.

Required sections:

- Purpose
- Unit families
- Simulation units
- Normalized units
- Per-Tick rates
- Baseline ranges
- Conversion and non-conversion rules
- Config validation implications
- Rules
- Related documents
- Open questions

Important rules to include:

- `su`, `vu`, `au`, `mu`, `eu`, `tick` are simulation units.
- Fields/signals/probabilities are normalized unless explicitly documented otherwise.
- Rates are per Tick unless documented otherwise.
- Baseline ranges are starting stable-world ranges, not universal physical laws.

Open question to preserve:

- чи Energy Buffer occupies free internal volume directly, or only its capacity is determined by Materials. Current principles say Energy occupies internal volume, while other docs emphasize Energy is not matter. This needs a precise wording.

### `docs/world/tick-semantics.md`

Responsibility:

- define semantic visibility boundaries for Tick;
- define phase/delta/commit model;
- separate Tick semantics from Scheduler implementation;
- define same-tick visibility and deterministic conflict rules.

Required sections:

- Purpose
- Vocabulary: Tick, Phase, System, Delta, Commit, Snapshot
- Phase model
- Read/write rules
- Same-tick visibility
- Signals
- Resource allocation
- Movement/collision
- Deterministic conflict resolution
- Forbidden implementation leaks
- Relationship with `engine/scheduler.md`
- Rules
- Related documents
- Open questions

Important rules to include:

- Decision Phase writes Action Plans only.
- Cell decisions in Tick N read only committed decision snapshot for Tick N.
- Signal emitted in Tick N becomes readable no earlier than Tick N+1 unless a documented exception exists.
- Resource uptake conflicts are resolved deterministically, not by iteration order.
- Movement requests are collected before Physics resolves them.
- Statistics, renderer, UI, and observer views are read-only for behavior.

### `docs/biology/feasibility.md`

Responsibility:

- define universal Feasibility Check contract for active planned actions;
- define diagnostics and rejection reasons;
- prevent hidden per-process "can/cannot" logic;
- define relationship between Feasibility, ActionPlan, EnergyGate, ProcessExecution.

Required sections:

- Purpose
- Scope
- Feasibility input
- Feasibility result
- Failure reason taxonomy
- Diagnostics
- No world mutation rule
- Rejected action rule
- Process contract
- Relationship with mandatory costs
- Relationship with ProcessProgress
- Future reservation model
- Rules
- Related documents
- Open questions

Recommended result shape for documentation:

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

Suggested failure reason taxonomy:

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

## Files To Modify Later

### `docs/README.md`

Add new files to structure:

- `world/units.md`
- `world/tick-semantics.md`
- `biology/feasibility.md`

Add short descriptions under relevant catalogue sections.

### `docs/GLOSSARY.md`

Add or refine terms:

- `Simulation Unit`
- `Normalized Unit`
- `Per-Tick Rate`
- `Delta`
- `Commit`
- `Snapshot`
- `Feasibility Check`
- `Mandatory Cost`
- `Planned Action`

Clarify Energy/volume wording after deciding the open question from `units.md`.

### `docs/world/tick.md`

Keep high-level Tick model, but link semantic details to `world/tick-semantics.md`.

Expected edits:

- add "semantic details are defined in `tick-semantics.md`";
- remove duplicate same-tick or scheduler details if they become redundant;
- preserve conceptual stage explanation.

### `docs/engine/scheduler.md`

Link to `world/tick-semantics.md`.

Expected edits:

- state Scheduler must implement Tick Semantics Contract;
- keep engine-level systems and order;
- ensure no wording implies Scheduler phases are world laws.

### `docs/world/energy.md`

Refine mandatory/planned cost section.

Expected edits:

- define mandatory as cost of existing;
- define planned as cost of doing;
- add failure flow;
- explicitly exclude Joint upkeep from mandatory;
- route process feasibility details to `biology/feasibility.md`.

### `docs/biology/processes.md`

Refine process taxonomy.

Expected edits:

- passive process;
- mandatory existence cost;
- planned active action;
- long-running process progress;
- failure/rejection rules delegated to `biology/feasibility.md`.

### `docs/biology/cell.md`

Refine cell-level lifecycle around mandatory existence cost.

Expected edits:

- cell pays mandatory cost each Tick if alive;
- if unpaid, planned actions skipped and survival consequences applied;
- clarify this is not just action rejection but inability to pay existence cost.

### `docs/biology/lifecycle.md`

Connect mandatory cost failure to stress/dormancy/death.

Expected edits:

- add failure path from unpaid mandatory cost;
- keep death as structural failure, not direct Energy kill;
- define consequences as boundary/material degradation leading to lifecycle state changes.

### `docs/biology/genome.md`

Clarify that Genome can request actions but Feasibility Check may reject them with explicit diagnostics.

Expected edits:

- link to `biology/feasibility.md`;
- add `invalid_genome_request` as possible diagnostic, not a special punishment.

### `docs/biology/joint.md`

Refine Joint upkeep rule.

Expected edits:

- Joint passive decay is environment/material process;
- Joint repair/upkeep is planned action;
- unsupported Joint degrades or breaks.

### `docs/config/world_config.md`

Link to `world/units.md` for tick duration, seed, space and environment parameter ranges.

### `docs/config/resources_config.md`

Link amount, density, volume, energy_value, diffusion_rate, decay_rate to `world/units.md`.

### `docs/config/materials_config.md`

Link material amount, volume_per_unit, stability, synthesis cost, repair cost to `world/units.md`.

### `docs/config/fields_config.md`

Link field values to normalized unit scale `0.0..1.0`.

## Proposed Implementation Order After All 18 Answers Are Collected

Do not apply yet. When user says "виконуй", use this order for GZ-01..GZ-04:

1. Create `docs/world/units.md`.
2. Create `docs/world/tick-semantics.md`.
3. Create `docs/biology/feasibility.md`.
4. Update navigation in `docs/README.md`.
5. Update glossary terms in `docs/GLOSSARY.md`.
6. Update `docs/world/tick.md` and `docs/engine/scheduler.md` to reference the Tick Semantics Contract.
7. Update `docs/world/energy.md` for mandatory/planned costs.
8. Update `docs/biology/processes.md`, `docs/biology/cell.md`, `docs/biology/lifecycle.md`.
9. Update `docs/biology/genome.md` and `docs/biology/joint.md`.
10. Update config docs with unit references.
11. Run link/term checks for new docs and old duplicated sections.
12. Create implementation report.

## Verification Plan For Future Execution

Run after applying changes:

```powershell
rg -n --encoding utf-8 "units.md|tick-semantics.md|feasibility.md" docs README.md
rg -n --encoding utf-8 "partial execution|часткове виконання|stable id order|mandatory costs|planned actions|Feasibility Check" docs
rg -n --encoding utf-8 "TODO|TBD|ADR-000X|MVP" docs
git status --short
```

Expected:

- new docs are linked from navigation and relevant Canon files;
- no reintroduced `partial execution` as allowed rejected-action behavior;
- no `ADR-000X` or MVP wording;
- only intentional references to mandatory/planned actions and Feasibility Check remain.

## Non-Blocking Clarification To Resolve Later

Energy/volume wording needs careful handling:

- `PRINCIPLES.md` says Energy occupies internal volume.
- Current clarifications say Energy is not matter and Energy Buffer capacity comes from Materials.

Recommended resolution:

- Energy itself is not matter and is not a `Resource`;
- Energy Buffer capacity is provided by Materials and therefore indirectly consumes cell volume through those Materials;
- stored Energy may have a configured capacity limit but should not be counted as an additional material volume unless a future rule explicitly adds that.

This can be confirmed before applying `docs/world/units.md`.
