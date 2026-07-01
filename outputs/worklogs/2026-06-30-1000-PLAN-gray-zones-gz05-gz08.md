---
tags:
  - alife
  - worklog/plan
---

# PLAN: gray zones GZ-05..GZ-08 documentation additions

Date: 2026-06-30 10:00

## Goal

Підготувати план доповнення документації за відповідями на сірі зони:

- GZ-05 Process progress vs action execution
- GZ-06 Minimal viable cell formula
- GZ-07 Division partition rules
- GZ-08 Direct Regulatory Graph interface boundaries

Цей файл є другим робочим планом у серії. Canon-документи не змінюються до команди "виконуй".

## Dependencies On Previous Plan

Цей блок спирається на рішення з `outputs/worklogs/2026-06-30-0837-PLAN-gray-zones-gz01-gz04.md`:

- `docs/world/units.md` потрібен для cost/progress/division ranges.
- `docs/world/tick-semantics.md` потрібен для visibility rules і commit моментів.
- `docs/biology/feasibility.md` потрібен для rule: rejected action never increases progress and has no partial output.
- Mandatory/planned cost model потрібен для відокремлення existence cost від progress-building planned actions.

## Accepted Direction From User Answers

### GZ-05

Створити `docs/biology/process-progress.md`.

Прийняти hybrid rule:

- більшість базових дій atomic;
- `ProcessProgress` існує лише для явно позначених long-running процесів;
- progress як material/runtime state для складної біології лишається future option.

Atomic actions:

- uptake;
- simple synthesis;
- movement;
- signal emit;
- basic repair.

Long-running processes:

- genome copying;
- division preparation;
- large material synthesis;
- large structural repair;
- joint strengthening;
- specialized structure growth.

Core invariants:

1. Rejected action never increases progress.
2. Progress can increase only after successful Feasibility Check.
3. Progress is state, not partial side effect.
4. Progress must have owner: `cell_id + process_id + target`.
5. Progress may decay, pause, complete, or be cancelled by explicit rules.

Key distinction:

```text
paid work != partial final product
```

Allowed progress flow:

```text
action allowed
cost paid
progress += delta
result appears only when progress reaches threshold
```

Rejected action flow:

```text
planned action rejected
progress unchanged
no cost charged
no partial output
```

### GZ-06

Створити `docs/biology/cell-state.md`.

Не вводити `viability_score` або абстрактний `viability threshold` як Canon-рішення, бо це схоже на прихований HP.

Замінити "Minimal viable cell formula" на:

- `Cell Functional State Evaluation`, або
- `Living Function Continuity`.

Core rule:

```text
There is no abstract viability threshold.
A cell stops being alive when its living functions can no longer continue through material, energetic, regulatory, or boundary mechanisms.
```

Стан клітини не threshold-based, а functional:

- `active`
- `stalled`
- `stressed`
- `dormant`
- `damaged`
- `inert`
- `decomposing`
- `persistent_remains`

Living function evaluation має відповідати:

- які функції ще доступні?
- чи може клітина виконувати mandatory existence cost?
- чи може виконувати Genome Runtime?
- чи може тримати Boundary?
- чи може накопичувати або перетворювати Resources?
- чи стала структура inert material structure?

Dead cell / remains are physical:

- organic waste;
- mineral residue;
- stable shell;
- toxic-like local source;
- physical obstacle;
- resource patch;
- surface for attachment;
- protective scaffold.

### GZ-07

Створити `docs/biology/division-partition.md`.

Прийняти starting model:

- noisy proportional split;
- future-compatible with spatial split або regulated asymmetric split.

Core rule:

```text
Division = physical split of one living cell state into two cell states.
Nothing is copied except what is explicitly defined as copied.
Everything else is partitioned, degraded, duplicated with cost, or lost.
```

State handling:

- Resources -> partition.
- Materials -> partition.
- Energy -> partition.
- Damage -> partition / inherited condition.
- Genome -> copied with mutation risk.
- Epigenetics -> partial inheritance.
- Joints -> broken / reassigned / inherited only if spatially valid.
- ProcessProgress -> normally not inherited unless explicit rule exists.

Division outcomes:

- two viable cells;
- one viable + one weak cell;
- two weak cells;
- one inert / non-functional daughter;
- failed physical split.

Separate failure types:

1. Failed before partition:
   - Feasibility failed before physical split.
   - No daughter created.
   - No partition.
   - Progress may pause or decay.

2. Failed after partition:
   - physical split happened;
   - partition committed;
   - one or both resulting cells may be unstable.

Core invariant:

```text
Division partition must be deterministic, conservative for matter and Energy unless explicit loss is configured, and must not guarantee viable offspring.
Rejected division before partition has no daughter and no partial physical result.
Division after partition is committed, even if one or both daughters are weak, leaking, inert, or doomed.
```

Starting partition table:

| State type | Starting partition rule | Conservation |
| --- | --- | --- |
| Energy Buffer | noisy proportional split | total conserved minus division cost |
| Resources | noisy proportional by target volume | conserved |
| Materials | noisy proportional by target volume | conserved unless division damage/loss configured |
| Boundary material | split into two boundary states | conserved, but may be insufficient for one or both |
| Genome | copied, not partitioned | copy cost paid before completion |
| Genome mutation | applied during copying | deterministic with seed |
| Epigenetic state | partial inherited / attenuated | not necessarily conserved |
| Damage | split or inherited by affected materials | conserved as material condition |
| Joints | usually broken or reassigned if still valid | no magic preservation |
| ProcessProgress | normally not inherited unless explicit | no hidden copy |

### GZ-08

Створити `docs/genetics/regulatory-interface.md`.

Core rule:

```text
Genome output != world action.
Genome output = regulatory intent / priority / modulation.
```

Genome Runtime produces regulatory outputs only. It never mutates world state directly.

All outputs must pass through:

```text
Genome outputs
    ↓
priority normalization
    ↓
ActionPlan candidates
    ↓
Feasibility Check
    ↓
Execution / rejection
```

Allowed input family:

- local_energy_level;
- local_resource_levels;
- internal_resource_levels;
- material_state;
- boundary_state;
- damage/stress signals;
- local_field_values;
- local_signal_inputs;
- crowding/pressure;
- lifecycle_state;
- epigenetic_state.

Forbidden inputs:

- global population;
- species_id;
- fitness_score;
- absolute world map;
- target coordinates;
- neighbor genome;
- organism command.

Output semantics:

- output is not arbitrary action;
- output is a bounded regulatory channel controlling process priorities;
- output scale is `-1.0..+1.0`;
- `-1.0` = suppress;
- `0.0` = neutral;
- `+1.0` = strongly prioritize;
- high priority does not guarantee execution.

Runtime model:

- bounded recurrent graph with fixed runtime steps;
- memory only through explicit epigenetic/runtime state;
- allowed activation functions: `linear_clamped`, `sigmoid`, `threshold`;
- arbitrary functions in config/code are forbidden because they become a scripting language;
- recurrent edges allowed only with fixed small runtime step count, e.g. `3..8`;
- state resets each Tick unless stored in explicit runtime/epigenetic state.

Cost model:

Genome Runtime has a small base cost or belongs to mandatory existence cost.

Genome complexity must have cost:

```text
runtime_cost =
base_cost
+ node_count_cost
+ edge_count_cost
+ runtime_step_cost
```

This prevents evolution from producing free unlimited regulatory graphs.

## Files To Create

### `docs/biology/process-progress.md`

Responsibility:

- define ProcessProgress as persistent state between Tick;
- distinguish progress from partial execution;
- define owner, target, cost, completion, decay, pause, cancellation;
- define which processes may be long-running in the base model.

Required sections:

- Purpose
- Why progress is not partial execution
- Atomic actions
- Long-running processes
- ProcessProgress identity
- Progress lifecycle
- Cost and Feasibility
- Completion rule
- Pause, decay, cancellation
- Relationship with Tick semantics
- Relationship with division partition
- Rules
- Related documents
- Open questions

Important rules:

- rejected action never increases progress;
- progress increase requires successful Feasibility Check and paid cost;
- final product appears only when progress reaches completion threshold;
- progress is committed state, not hidden side effect.

### `docs/biology/cell-state.md`

Responsibility:

- define functional living states without viability score or hidden HP;
- replace threshold-based "minimal viability" framing;
- explain living function continuity and physical remains.

Required sections:

- Purpose
- No abstract viability threshold
- Living Function Continuity
- Functional state list
- Active
- Stalled
- Stressed
- Dormant
- Damaged
- Inert
- Decomposing
- Persistent remains
- State transition causes
- Relationship with mandatory cost
- Relationship with lifecycle
- Relationship with selection
- Rules
- Related documents
- Open questions

Important rules:

- cell death is loss of living function continuity through material, energetic, regulatory, or boundary mechanisms;
- dead/inert cells remain physical;
- observer metrics may summarize states, but cells do not read viability score.

### `docs/biology/division-partition.md`

Responsibility:

- define division as physical partition of one cell state;
- define what is copied, partitioned, attenuated, lost, or broken;
- distinguish failed before partition vs failed after partition;
- preserve conservation and determinism.

Required sections:

- Purpose
- Division as physical split
- Copy vs partition vs loss
- Starting partition model
- State partition table
- Failed before partition
- Failed after partition
- Boundary and weak daughter cells
- Genome copy and mutation
- Epigenetic attenuation
- Joint handling
- ProcessProgress handling
- Determinism and conservation
- Rules
- Related documents
- Open questions

Important rules:

- division does not guarantee viable offspring;
- pre-partition rejection creates no daughter and no partial physical result;
- post-partition failure is committed and selection/lifecycle handles consequences;
- matter and Energy are conservative unless explicit loss is configured.

### `docs/genetics/regulatory-interface.md`

Responsibility:

- define interface between Direct Regulatory Graph, Genome Runtime, ActionPlan, and Feasibility;
- constrain inputs/outputs;
- define output scale, priority semantics, activation functions, recurrent bounds, and runtime cost.

Required sections:

- Purpose
- Genome output is not action
- Input vocabulary
- Forbidden inputs
- Output vocabulary
- Output scale
- Priority normalization
- ActionPlan mapping
- Feasibility boundary
- Runtime model
- Activation functions
- Recurrent graph bounds
- Runtime memory boundary
- Runtime cost
- Rules
- Related documents
- Open questions

Important rules:

- Genome Runtime never mutates world state directly;
- all outputs pass through ActionPlan and Feasibility Check;
- arbitrary activation functions are forbidden;
- recurrent execution is bounded;
- regulatory memory requires explicit epigenetic/runtime state.

## Files To Modify Later

### `docs/README.md`

Add new files to navigation:

- `biology/process-progress.md`
- `biology/cell-state.md`
- `biology/division-partition.md`
- `genetics/regulatory-interface.md`

### `docs/GLOSSARY.md`

Add or refine terms:

- `ProcessProgress`
- `Atomic Action`
- `Long-running Process`
- `Cell Functional State`
- `Living Function Continuity`
- `Inert`
- `Persistent Remains`
- `Division Partition`
- `Failed Before Partition`
- `Failed After Partition`
- `Regulatory Intent`
- `ActionPlan Candidate`
- `Priority`

Remove or reframe terms that imply hidden score:

- `viability score`
- `viability threshold`

### `docs/biology/processes.md`

Expected edits:

- link ProcessProgress details to `biology/process-progress.md`;
- keep process overview here;
- remove remaining ambiguity around partial output;
- classify current examples as atomic or long-running.

### `docs/biology/lifecycle.md`

Expected edits:

- replace "Minimal viability formula" with functional state language;
- link to `biology/cell-state.md`;
- link division details to `biology/division-partition.md`;
- separate pre-partition rejection from post-partition unstable daughter outcomes;
- remove wording like "viability threshold" where it implies hidden score.

### `docs/biology/cell.md`

Expected edits:

- link cell state details to `biology/cell-state.md`;
- describe `active/stalled/dormant/damaged/inert/decomposing/persistent_remains` as functional states;
- clarify that dead/inert remains may still be resources, obstacles, shells, scaffold, or local sources.

### `docs/biology/genome.md`

Expected edits:

- link Direct Regulatory Graph interface to `genetics/regulatory-interface.md`;
- state Genome outputs regulatory intent, not action;
- replace duplicated input/output details with links where appropriate;
- keep high-level Genome principles.

### `docs/genetics/genome-runtime.md`

Expected edits:

- align runtime execution model with `regulatory-interface.md`;
- add output scale `-1.0..+1.0`;
- define bounded recurrent execution;
- update activation functions to `linear_clamped`, `sigmoid`, `threshold`;
- clarify runtime cost formula.

### `docs/genetics/genome-representation.md`

Expected edits:

- replace stale "first implementation" wording if still present;
- ensure Direct Regulatory Graph is base model but adjustable;
- link IO/runtime boundary to `genetics/regulatory-interface.md`;
- keep fragment/HGT compatibility.

### `docs/genetics/regulatory-network.md`

Expected edits:

- align node/edge/runtime semantics with regulatory interface;
- ensure outputs are priorities/modulations, not direct actions;
- bounded recurrent graph rule;
- activation function restrictions.

### `docs/genetics/epigenetics.md`

Expected edits:

- clarify memory boundary:
  - recurrent runtime state resets each Tick unless stored in explicit runtime/epigenetic state;
  - epigenetics can modulate Genome Runtime but is not Genome mutation.

### `docs/biology/joint.md`

Expected edits:

- link joint strengthening as long-running ProcessProgress if configured;
- link division Joint handling to `biology/division-partition.md`;
- default Joint behavior during division: broken or reassigned only if spatially valid.

### `docs/biology/specialization.md`

Expected edits:

- link specialized structure growth to ProcessProgress;
- ensure specialization remains emergent from materials, signals, epigenetics, and regulatory outputs.

### `docs/biology/communication.md`

Expected edits:

- align signal inputs with regulatory-interface allowed inputs;
- ensure signal outputs are planned actions and not direct commands.

### `docs/engine/ecs.md`

Expected edits:

- if future ECS docs mention components, ensure `ProcessProgress` is state, not partial output;
- ensure analytical state labels do not influence behavior unless represented in allowed local cell state.

### `docs/engine/scheduler.md`

Expected edits:

- link ProcessProgress commit timing to Tick Semantics Contract;
- ensure Genome Runtime runs during Decision and does not mutate world state.

### Config docs

Expected edits after all 18 zones are collected:

- `docs/config/materials_config.md`: large synthesis/repair costs and progress increments.
- `docs/config/world_config.md`: division/noise defaults if scenario-specific.
- `docs/config/resources_config.md`: resource amount/volume needed by partition and process progress.

## Proposed Implementation Order After All 18 Answers Are Collected

Do not apply yet. When user says "виконуй", use this order for GZ-05..GZ-08:

1. Create `docs/biology/process-progress.md`.
2. Create `docs/biology/cell-state.md`.
3. Create `docs/biology/division-partition.md`.
4. Create `docs/genetics/regulatory-interface.md`.
5. Update `docs/README.md` navigation.
6. Update `docs/GLOSSARY.md`.
7. Update `docs/biology/processes.md`.
8. Update `docs/biology/cell.md` and `docs/biology/lifecycle.md`.
9. Update `docs/biology/joint.md` and `docs/biology/specialization.md`.
10. Update `docs/biology/genome.md`.
11. Update `docs/genetics/genome-runtime.md`.
12. Update `docs/genetics/genome-representation.md` and `docs/genetics/regulatory-network.md`.
13. Update `docs/genetics/epigenetics.md`.
14. Update `docs/biology/communication.md`.
15. Update engine/config references only where they point to the new contracts.
16. Run verification searches.
17. Create implementation report after applying.

## Verification Plan For Future Execution

Run after applying changes:

```powershell
rg -n --encoding utf-8 "process-progress.md|cell-state.md|division-partition.md|regulatory-interface.md" docs README.md
rg -n --encoding utf-8 "partial final product|partial output|viability_score|viability threshold|hidden HP|Genome output =|Regulatory Intent|ProcessProgress" docs
rg -n --encoding utf-8 "first implementation|MVP|ADR-000X|TODO|TBD" docs
git status --short
```

Expected:

- new docs are linked from navigation and relevant Canon files;
- partial execution is not reintroduced as rejected-action behavior;
- no hidden viability score/threshold remains as Canon state;
- Genome outputs are regulatory priorities/modulations only;
- division partition distinguishes pre-partition rejection from post-partition unstable outcomes.

## Non-Blocking Clarifications To Resolve Later

### C-01. Runtime cost placement

User answer allows Genome Runtime to have small base cost or be included in mandatory existence cost.

Recommended resolution during final plan:

- base runtime cost belongs to mandatory existence cost for living cells with active Genome Runtime;
- complexity surcharge can be charged as part of Genome Runtime cost or as genome maintenance/copying cost;
- exact formula should reference `docs/world/units.md`.

### C-02. `viability` term cleanup

Existing docs contain terms like `viability`, `offspring_viability`, and `Organism viability`.

Recommended resolution:

- allow `viability` only as observer/research metric;
- Canon cell state should use functional continuity and material consequences;
- cells and Genome Runtime must never read viability labels or scores.

### C-03. ProcessProgress inheritance

User answer says ProcessProgress normally not inherited unless explicit.

Recommended resolution:

- default: ProcessProgress is not inherited during division;
- exception must be process-specific and documented in `division-partition.md`;
- no hidden copy of progress to daughters.
