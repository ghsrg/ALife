---
tags:
  - alife
  - worklog/plan
---

# PLAN: gray zones GZ-09..GZ-12 documentation additions

Date: 2026-06-30 11:25

## Goal

Підготувати план доповнення документації за відповідями на сірі зони:

- GZ-09 Materials-to-process capability matrix
- GZ-10 Resource reaction model
- GZ-11 Heat / temperature boundary
- GZ-12 Space representation and locality

Цей файл є третім робочим планом у серії. Canon-документи не змінюються до команди "виконуй".

## Dependencies On Previous Plans

Цей блок спирається на:

- `outputs/worklogs/2026-06-30-0837-PLAN-gray-zones-gz01-gz04.md`
  - `docs/world/units.md` для normalized capability values, rates, costs, field values.
  - `docs/world/tick-semantics.md` для visibility/commit правил.
  - `docs/biology/feasibility.md` для capability/resource/space rejection reasons.
- `outputs/worklogs/2026-06-30-1000-PLAN-gray-zones-gz05-gz08.md`
  - `docs/biology/process-progress.md` для long-running process effects.
  - `docs/genetics/regulatory-interface.md` для rule: Genome only prioritizes available capabilities.

## Accepted Direction From User Answers

### GZ-09

Створити `docs/biology/process-capabilities.md`.

Canon principle:

```text
No process may execute without a material mechanism.
Material properties define what process capabilities are available.
Genome outputs only prioritize available capabilities.
```

Capability is not boolean. It is a normalized capability level that influences:

- process efficiency;
- energy cost;
- max intensity;
- failure risk;
- degradation risk.

Runtime flow:

```text
cell materials
    ↓
derived capabilities
    ↓
allowed process set
    ↓
Genome priority can modulate only enabled processes
    ↓
Feasibility Check
    ↓
ProcessExecution
```

Core invariant:

```text
A process cannot be executed only because Genome requested it.
It must be enabled by material capability, available resources, Energy, local conditions, and lifecycle state.
```

Minimum capability matrix:

| Material property | Enables process | Example process | Also requires |
| --- | --- | --- | --- |
| `boundary_support` | boundary upkeep | maintain boundary | Energy + boundary material |
| `permeability` | uptake/export | resource intake/export | local Resource + Energy optional |
| `energy_conversion_efficiency` | energy conversion | convert resource to Energy Buffer | Resource with energy_value |
| `repair_support` | repair | restore damaged material | repair Resources + Energy |
| `movement_support` | movement | movement impulse | Energy + space + physics |
| `signal_sensitivity` | sensing | read local signal | local signal/field |
| `signal_conductivity` | signal transfer | internal/joint signal | Energy optional |
| `joint_affinity` | joint creation/upkeep | create/maintain Joint | compatible material + neighbor |
| `resource_transport_support` | transport | move Resources internally/via Joint | Resource + Energy optional |
| `storage_capacity` | storage | hold Resource/Material | free volume |
| `field_sensitivity` | field sensing/conversion | react to light/heat/etc. | local Field + material |
| `structural_strength` | resist pressure/damage | passive resistance | no active cost, but degradation risk |

### GZ-10

Створити `docs/world/reactions.md`.

Start with:

- threshold reactions + simple rate/probability;
- passive vs controlled reactions;
- no scientific full chemistry module;
- no toxicity field;
- no automatic Energy Buffer;
- all effects through products, heat, material damage, volume, blocking, or controlled conversion.

Reaction happens if:

- inputs are present;
- conditions are satisfied;
- catalyst/material exists if required;
- rate/probability allows it.

Minimal reaction contract:

```yaml
reaction:
  id: "nutrient_oxidation_A"
  type: "controlled"
  inputs:
    nutrient_A: 1.0
    oxidizer_A: 0.5
  conditions:
    heat:
      min: 0.2
      max: 0.8
  catalyst:
    material_id: "conversion_polymer_A"
    min_amount: 0.2
  products:
    waste_A: 0.6
  energy_release: 0.8
  heat_release: 0.1
  rate: 0.25
  probability: 0.8
```

Reaction types:

- `passive`
- `controlled`
- `degradation`
- `decay`
- `synthesis`
- `conversion`

Core invariants:

```text
No Resource may produce Energy, damage, poisoning, synthesis, or degradation effect without an explicit reaction rule or material/process mechanism.
Controlled reactions require ActionPlan + Feasibility Check.
Passive reactions are environment-driven and do not require Genome decision.
```

Recommended architecture:

- `docs/world/reactions.md` = semantic contract.
- `docs/engine/chemistry.md` = how reactions are executed.
- `docs/config/reactions_config.md` = actual balanced reaction rules.
- `docs/config/resources_config.md` = resource properties.
- `docs/config/materials_config.md` = catalysts / capabilities.

### GZ-11

Створити `docs/world/field-semantics.md`.

GZ-11 is broader than Heat. The same logic should apply to all Fields.

Field semantic categories:

- `External Field`: global or spatial world condition.
- `Local State`: state of cell/material/resource/environment patch.
- `Derived Field`: computed field for rendering/debug/analytics.
- `Behavior Input`: locally sampled value that passes through material capability.

Core invariant:

```text
Fields are not commands.

A Field may affect a cell only if:
1. the cell locally samples it;
2. the cell has a Material or process capable of responding to it;
3. the effect passes through Feasibility / Process / Physics / Reaction rules.
```

Field examples:

- Light:
  - `LightField` exists in world.
  - cell samples local light.
  - only light-sensitive Material can use it.
  - effect becomes energy conversion / signal / degradation modifier.
- Heat:
  - reaction produces heat locally.
  - local temperature changes.
  - heat transfers by contact / Joint / environment.
  - optional `HeatField` can be derived for visualization.
- Radiation:
  - `RadiationField` exists in world.
  - cell samples local radiation.
  - effect passes through material/genome damage rules.
- Pressure:
  - pressure may be derived from crowding / collision / flow.
  - local pressure state affects boundary damage / joint stress / movement resistance.

Config addition for each Field:

```yaml
field_semantics:
  direct_behavior_effects_allowed: false
  local_sampling_required: true
  material_capability_required: true
```

Base rule:

```text
Fields exist globally/spatially, but affect cells only through local state, local sampling, materials, reactions, physics and feasibility.
```

### GZ-12

Do not create a new large file.

Add a major section to `docs/world/space.md`:

```text
# Locality Contract
```

Base representation:

| Object | Base representation | Reason |
| --- | --- | --- |
| Cells | entity-based | genome, materials, energy, lifecycle |
| Joints | entity-based edge | connects specific cells |
| Resources | grid-based / sparse grid | diffusion, uptake, decay, reactions |
| Fields | grid/function-based | spatial influence, local sampling |
| Signals/Traces | grid-based or joint-based | local communication |
| Zones | config geometry overlay | not behavior entity |
| Pressure | derived local state | from collision/crowding/physics |
| Heat/temperature | local state + optional derived field | not global command |

Locality Contract:

```text
position
    ↓
spatial grid cell
    ↓
neighboring grid cells by radius
    ↓
readable local state
    ↓
interaction candidates
    ↓
Feasibility / Physics / Reaction / Process execution
```

Cell can read only:

- local Fields;
- local Resources;
- local Traces;
- nearby Cells as physical presence/signals;
- connected Joints;
- local pressure/crowding;
- own internal state.

Cell cannot read:

- global resource totals;
- global population;
- all cells;
- all fields;
- species clusters;
- fitness metrics;
- full map.

Interaction candidates:

- Resource uptake candidates: resource grid cells within uptake_radius.
- Cell contact candidates: CellEntities from neighboring spatial grid cells.
- Joint candidates: nearby cells within joint_creation_radius.
- Signal candidates: trace grid cells or connected Joints.
- Reaction candidates: resources/materials in same local environment patch.

Base spatial rule:

```text
spatial_grid_size should be approximately 2..8 cell radius.
```

Boundary and Zones:

- boundary mode is fixed per run;
- `wrapped`, `solid_wall`, `open` are configured in `world_config.md`;
- Zone is named config area used by fields/resources/events;
- Zone is not territory, species area, behavior controller, or cell knowledge.

Core invariants:

```text
All world interactions must be resolved through the same locality contract:
position -> neighborhood -> local readable state -> interaction candidates.

Changing the representation of Resources, Fields or Traces must not change what a cell is semantically allowed to know.
```

## Files To Create

### `docs/biology/process-capabilities.md`

Responsibility:

- define the material capability layer between Materials, Genome outputs, Feasibility Check, and ProcessExecution;
- define capability as normalized degree, not boolean;
- provide minimum capability matrix;
- define how capabilities affect cost, intensity, efficiency, failure risk, and degradation risk.

Required sections:

- Purpose
- Canon principle
- Capability is not boolean
- Material properties and derived capabilities
- Runtime flow
- Capability vs Genome priority
- Capability vs Feasibility Check
- Capability matrix
- Efficiency/cost/intensity effects
- Forbidden shortcuts
- Rules
- Related documents
- Open questions

Important rules:

- Genome output cannot create capability.
- Capability cannot bypass Energy, Resources, Space, Lifecycle, or Feasibility.
- `movement_support = 0` means movement cannot execute even with priority `+1.0`.
- Low capability may increase cost, reduce intensity, raise degradation/failure risk.

### `docs/world/reactions.md`

Responsibility:

- define universal reaction semantics;
- split passive vs controlled reactions;
- define minimum reaction schema;
- define no automatic Energy Buffer rule;
- define poisoning-by-reaction without toxicity field.

Required sections:

- Purpose
- Why reaction semantics, not full chemistry
- Reaction contract
- Reaction types
- Passive reactions
- Controlled reactions
- Energy release vs Energy Buffer
- Heat release
- Products and waste
- Poisoning-by-reaction
- Location and locality
- Rate/probability and determinism
- Config placement
- Rules
- Related documents
- Open questions

Important rules:

- Passive reactions do not require Genome priority.
- Controlled reactions require ActionPlan + Feasibility Check.
- `reaction.energy_release` does not automatically charge any cell's Energy Buffer.
- Resource harmfulness is caused by products, blocking, heat, material damage, reactions, or local condition changes.

### `docs/world/field-semantics.md`

Responsibility:

- define Field semantics shared by Light, Heat, Radiation, Pressure, Flow and future Fields;
- distinguish external Field, local state, derived Field and behavior input;
- define local sampling and material mediation;
- prevent direct field-as-command behavior.

Required sections:

- Purpose
- Field categories
- External Field
- Local State
- Derived Field
- Behavior Input
- Local sampling
- Material mediation
- Field examples
- Behavior restrictions
- Config semantics
- Rules
- Related documents
- Open questions

Important rules:

- Fields are not commands.
- A Field affects a cell only through local sampling and a compatible material/process mechanism.
- Derived fields are safe for rendering/debug/analytics, but do not automatically become behavior inputs.
- Direct behavior effects are disallowed unless a future explicit rule changes this.

### `docs/config/reactions_config.md`

Responsibility:

- define the config shape for balanced reaction rules;
- keep full reaction rules out of `resources_config.md`;
- reference resource properties, material catalysts and world reaction semantics.

Required sections:

- Purpose
- What reactions_config is
- What reactions_config is not
- Basic schema
- Passive reaction examples
- Controlled reaction examples
- Catalyst/material references
- Conditions
- Products
- Energy/heat release
- Rate/probability
- Validation
- Rules
- Related documents
- Open questions

Important rules:

- config defines reaction rules, not hardcoded biology;
- reactions reference existing resource/material IDs;
- invalid reactions fail config validation;
- no `toxicity: true` shortcut.

## Files To Modify Later

### `docs/README.md`

Add to navigation:

- `world/reactions.md`
- `world/field-semantics.md`
- `biology/process-capabilities.md`
- `config/reactions_config.md`

Add note that `world/space.md` contains the Locality Contract.

### `docs/GLOSSARY.md`

Add or refine terms:

- `Capability`
- `Material Capability`
- `Derived Capability`
- `Passive Reaction`
- `Controlled Reaction`
- `Reaction Rule`
- `External Field`
- `Local State`
- `Derived Field`
- `Behavior Input`
- `Locality Contract`
- `Interaction Candidate`
- `Zone`

Clarify that `toxicity` is not Canon property.

### `docs/world/materials.md`

Expected edits:

- link capability semantics to `biology/process-capabilities.md`;
- state Material properties derive capabilities;
- avoid duplicating the full matrix if moved to process-capabilities.

### `docs/biology/processes.md`

Expected edits:

- link process availability to `biology/process-capabilities.md`;
- link reactions to `world/reactions.md`;
- clarify Resource Reaction vs Controlled Reaction;
- ensure active process execution still goes through Feasibility Check.

### `docs/biology/genome.md`

Expected edits:

- state Genome can prioritize only available material capabilities;
- link to `biology/process-capabilities.md`;
- link regulatory outputs to `genetics/regulatory-interface.md` from previous plan.

### `docs/biology/cell.md`

Expected edits:

- link local readable state and capabilities;
- clarify cells read local Fields/Resources/Traces only via Locality Contract.

### `docs/biology/communication.md`

Expected edits:

- align Signals/Traces with Locality Contract;
- field/signal inputs require local sampling and compatible material.

### `docs/biology/joint.md`

Expected edits:

- joint creation/upkeep requires `joint_affinity` or compatible material capability;
- joint signal/resource transfer uses material capabilities and locality.

### `docs/biology/specialization.md`

Expected edits:

- specialization emerges from different material capabilities and local contexts;
- no cell class or role bypasses capability checks.

### `docs/world/resources.md`

Expected edits:

- move semantic reaction contract to `world/reactions.md`;
- keep Resource properties here;
- clarify `reaction_profile` is simple local shorthand or reference, while complete reaction rules belong in `config/reactions_config.md`;
- reinforce no toxicity property.

### `docs/world/energy.md`

Expected edits:

- link Energy generation through controlled reactions and compatible Materials;
- clarify `reaction.energy_release != automatic Energy Buffer`;
- keep Energy Buffer increase as cell process result after Feasibility.

### `docs/world/fields.md`

Expected edits:

- keep Field descriptions here;
- link behavior semantics to `world/field-semantics.md`;
- clarify direct Field effects are forbidden unless mediated through sampling/material/process/reaction/physics.

### `docs/world/physics.md`

Expected edits:

- align Pressure as derived local state where appropriate;
- align Heat/temperature with field-semantics;
- link movement/contact candidate discovery to Locality Contract in `world/space.md`.

### `docs/world/space.md`

Expected edits:

- add `# Locality Contract`;
- define representation table;
- define readable local state;
- define forbidden global reads;
- define interaction candidates;
- define `spatial_grid_size` guideline as `2..8 cell radius`;
- resolve open questions:
  - Resources: grid/sparse grid.
  - Fields: grid/function layers.
  - Boundary changes: fixed per run.
  - Zones: config overlay.

### `docs/config/materials_config.md`

Expected edits:

- align `functional` block with minimum capability matrix;
- add missing capability fields:
  - `boundary_support`
  - `repair_support`
  - `signal_sensitivity`
  - `signal_conductivity`
  - `joint_affinity`
  - `resource_transport_support`
  - `storage_capacity`
  - `field_sensitivity`
  - `structural_strength`
- keep values normalized `0.0..1.0`;
- explain capability values influence cost/efficiency/intensity/risk.

### `docs/config/resources_config.md`

Expected edits:

- reduce full reaction logic here;
- link full reactions to `config/reactions_config.md`;
- keep simple `reaction_profile` only as lightweight shorthand/reference;
- keep no toxicity rule.

### `docs/config/fields_config.md`

Expected edits:

- add `field_semantics` block:

```yaml
field_semantics:
  direct_behavior_effects_allowed: false
  local_sampling_required: true
  material_capability_required: true
```

- align Heat as local state / optional derived field;
- distinguish external fields from derived/debug fields.

### `docs/config/world_config.md`

Expected edits:

- boundary mode fixed per run;
- zones are config overlays for fields/resources/events;
- spatial grid guideline references `world/space.md`.

### `docs/engine/chemistry.md`

Expected edits:

- align with `world/reactions.md`;
- describe execution of passive/controlled reactions;
- remove or reframe stale open question "first schema reactions";
- clarify no direct Energy Buffer from passive reaction.

### `docs/engine/physics.md`

Expected edits:

- pressure as local/derived physics state;
- contact/collision candidates via Locality Contract.

### `docs/engine/performance.md`

Expected edits:

- spatial grid implementation belongs here;
- representation can be optimized without changing semantic Locality Contract.

### `docs/engine/ecs.md`

Expected edits:

- align Cells/Joints as entities, Resources/Fields as grid/layers/function where appropriate;
- derived fields/views remain non-behavioral unless explicitly sampled through Canon rules.

## Proposed Implementation Order After All 18 Answers Are Collected

Do not apply yet. When user says "виконуй", use this order for GZ-09..GZ-12:

1. Create `docs/biology/process-capabilities.md`.
2. Create `docs/world/reactions.md`.
3. Create `docs/world/field-semantics.md`.
4. Create `docs/config/reactions_config.md`.
5. Add Locality Contract to `docs/world/space.md`.
6. Update `docs/README.md` navigation.
7. Update `docs/GLOSSARY.md`.
8. Update core Canon links:
   - `docs/world/materials.md`
   - `docs/world/resources.md`
   - `docs/world/fields.md`
   - `docs/world/energy.md`
   - `docs/world/physics.md`
   - `docs/biology/processes.md`
   - `docs/biology/genome.md`
   - `docs/biology/cell.md`
9. Update biology integration:
   - `docs/biology/communication.md`
   - `docs/biology/joint.md`
   - `docs/biology/specialization.md`
10. Update config docs:
    - `docs/config/materials_config.md`
    - `docs/config/resources_config.md`
    - `docs/config/fields_config.md`
    - `docs/config/world_config.md`
11. Update engine docs:
    - `docs/engine/chemistry.md`
    - `docs/engine/physics.md`
    - `docs/engine/performance.md`
    - `docs/engine/ecs.md`
12. Run verification searches.
13. Create implementation report after applying.

## Verification Plan For Future Execution

Run after applying changes:

```powershell
rg -n --encoding utf-8 "process-capabilities.md|reactions.md|field-semantics.md|reactions_config.md|Locality Contract" docs README.md
rg -n --encoding utf-8 "toxicity: true|direct_behavior_effects_allowed: true|automatic Energy Buffer|global resource totals|full map|Genome outputs only prioritize" docs
rg -n --encoding utf-8 "reaction.energy_release|Controlled reactions require|Fields are not commands|No process may execute without" docs
rg -n --encoding utf-8 "MVP|ADR-000X|TODO|TBD|перша реалізація" docs
git status --short
```

Expected:

- new docs are linked from navigation and relevant Canon/config/engine files;
- no toxicity shortcut is introduced;
- no direct Field command behavior is introduced;
- no Resource or reaction can magically charge Energy Buffer;
- Locality Contract is present and referenced by Space, Processes, Physics, and Performance;
- Genome priority remains separate from capability and Feasibility.

## Non-Blocking Clarifications To Resolve Later

### C-01. Capability aggregation formula

User answer defines capability as level, not boolean, but not the aggregation formula for multiple Materials.

Recommended future resolution:

- start with additive or weighted aggregate capped to `0.0..1.0`;
- document exact formula in `process-capabilities.md` only after Units and material amount semantics are stable;
- avoid choosing a formula before GZ-18 stability bounds.

### C-02. Reaction config file path

User proposed `config/reactions_config.md`; repository currently stores config docs under `docs/config/`.

Recommended resolution:

- create documentation file `docs/config/reactions_config.md`;
- later implementation may use actual runtime config path separately.

### C-03. Heat as environment transfer

User allows Heat transfer by contact / Joint / environment, while previous base rule limited Heat to contact/Joint and optional future Field.

Recommended resolution:

- for base Canon: local temperature + contact/Joint transfer;
- environment Heat transfer can be a controlled optional rule in `field-semantics.md` / `reactions.md`, not automatic global Heat command;
- derived HeatField is allowed for visualization/debug/analytics.

### C-04. Resource `reaction_profile` vs full reactions

Existing docs already use `reaction_profile` in Resources.

Recommended resolution:

- keep `reaction_profile` as lightweight local shorthand or reference;
- move complete reaction semantics and balanced rules to `world/reactions.md` and `config/reactions_config.md`;
- avoid duplicating full reaction rules inside every Resource definition.
