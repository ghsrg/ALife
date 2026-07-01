---
tags:
  - alife
  - worklog/plan
---

# PLAN: gray zones GZ-13..GZ-18 documentation additions

Date: 2026-06-30 12:48

## Goal

Підготувати план доповнення документації за відповідями на останні сірі зони:

- GZ-13 Joint behavior at lifecycle boundaries
- GZ-14 Communication signal semantics
- GZ-15 Epigenetic / runtime / learning state boundaries
- GZ-16 Genetic fragments and HGT inclusion boundary
- GZ-17 Observer views and analytical metrics
- GZ-18 Config validation and stability bounds

Цей файл є четвертим робочим планом у серії. Canon-документи не змінюються до команди "виконуй".

## Dependencies On Previous Plans

Цей блок спирається на:

- `outputs/worklogs/2026-06-30-0837-PLAN-gray-zones-gz01-gz04.md`
  - `docs/world/units.md`
  - `docs/world/tick-semantics.md`
  - `docs/biology/feasibility.md`
  - mandatory/planned cost model
- `outputs/worklogs/2026-06-30-1000-PLAN-gray-zones-gz05-gz08.md`
  - `docs/biology/process-progress.md`
  - `docs/biology/cell-state.md`
  - `docs/biology/division-partition.md`
  - `docs/genetics/regulatory-interface.md`
- `outputs/worklogs/2026-06-30-1125-PLAN-gray-zones-gz09-gz12.md`
  - `docs/biology/process-capabilities.md`
  - `docs/world/reactions.md`
  - `docs/world/field-semantics.md`
  - `docs/world/space.md` Locality Contract
  - config reaction/capability/field semantics

## Blocking Questions

No blocking questions.

There are non-blocking clarifications listed at the end. They can be resolved when the final consolidated plan is assembled.

## Accepted Direction From User Answers

### GZ-13

Do not create a new file.

Add structured sections to `docs/biology/joint.md`:

- Joint Contract
- Joint Channels
- Joint Creation
- Joint Upkeep
- Joint Damage and Break
- Joint During Division
- Joint During Death

Base decision:

```text
one Joint object with channel flags derived from material state
```

Do not introduce separate classes/concepts like:

- `ForceJoint`
- `SignalJoint`
- `ResourceJoint`

Joint is one material connection between two cells, and its capabilities are derived from Materials.

Minimum Joint contract:

```text
endpoints:
  cell_a
  cell_b

material_basis:
  joint_materials
  strength
  elasticity
  permeability
  signal_conductivity
  resource_transport_support
  heat_conductivity
  genetic_fragment_permeability

physics:
  rest_length
  current_length
  strain
  max_strain

state:
  damage
  stability
  age
  active_channels

cost:
  upkeep_cost
  repair_cost

break:
  break_condition
```

Start simple:

- Joint shows which two cells are connected.
- Joint can enable exchange through material-derived channels.
- Joint can break under physical/material conditions.
- Avoid turning Joint into a small physics engine before base life works.

Core rules:

1. Joint has exactly two endpoints in the base model.
2. Joint capabilities are derived from material properties.
3. Joint never transfers Energy Buffer directly.
4. Joint may transfer Resources, Signals, Heat, force, or genetic fragments only if corresponding channel exists.
5. Joint creation, repair and strengthening are planned actions.
6. Joint decay and break can happen passively.
7. Cell division does not duplicate Joints by default.
8. Cell death stops living transfer but may leave inert material connection.

### GZ-14

Do not create a new large file unless cleanup later shows `communication.md` becomes too large.

Primary target: `docs/biology/communication.md`.

Base decision:

```text
Signal = local scalar stimulus
```

Signal is not a typed command. It has no semantic marker such as:

- `damage`
- `food`
- `move`
- `pain`
- `resource_need`

Signal value:

```text
value = 0.0..1.0
```

or, where suppression/excitation is needed:

```text
value = -1.0..+1.0
```

Signal may be transmitted through:

- local environment;
- contact;
- Joint.

Signal fields:

- value;
- source;
- medium;
- lifetime_ticks;
- decay_rate;
- delay_ticks;
- emission_cost.

Receiver rule:

Incoming signal values are accumulated, decayed and normalized into local `signal_state`.

The meaning of a signal is not stored in the signal itself. Meaning emerges from receiver Materials, internal state, Genome Runtime and local context.

Same-tick rule:

```text
A signal emitted in Tick N is not visible to other cell decisions until Tick N+1,
unless a future explicit fast-conduction extension is defined.
```

Core invariant:

```text
Signal is not a command.
Signal is a scalar local stimulus.
Its effect is determined by receiver Materials, local state, Genome Runtime, Physics, Feasibility and Tick/Scheduler rules.
```

Future extension:

- fast nervous-system-like behavior requires specialized morphologies;
- typed/vector signals require explicit ADR only after scalar signals are proven insufficient;
- never introduce hardcoded `Neuron` class.

### GZ-15

Define separate state layers with explicit inheritance policy:

```text
Genome
EpigeneticState
RuntimeState
MaterialState
```

Primary targets:

- `docs/genetics/epigenetics.md`
- `docs/genetics/inheritance.md`
- `docs/biology/processes.md`
- `docs/biology/cell.md`
- `docs/biology/specialization.md`

Layer rules:

- Genome:
  - stable regulatory structure;
  - changes only through explicit mutation, repair, recombination or HGT;
  - heritable.
- EpigeneticState:
  - regulatory modifier over Genome Runtime;
  - may change during life;
  - may optionally be inherited according to explicit inheritance rules;
  - does not rewrite Genome.
- RuntimeState:
  - short-term simulation memory;
  - current signal accumulation, temporary activation, cooldowns, process timers, refractory state, recent local inputs;
  - usually not inherited;
  - may reset, decay or be partitioned only if explicitly configured.
- MaterialState:
  - physical/adaptive state of Materials;
  - damage, fatigue, conductivity, stored signal, deformation, permeability, sensitivity, hardening, softening;
  - may persist during life and be physically partitioned with Materials;
  - not genetic inheritance.

Inheritance policy:

```text
Genome          -> copied with mutation/recombination/HGT rules
EpigeneticState -> optionally inherited, attenuated or reset
RuntimeState    -> usually reset; explicit exceptions only
MaterialState   -> physically partitioned with Materials
```

Core invariant:

```text
Learning is not mutation.
Runtime memory is not heredity.
Material plasticity is not Genome change.
Only Genome and explicitly inheritable EpigeneticState may act as heredity.
```

Do not introduce a separate abstract `LearningState` unless these layers are proven insufficient.

### GZ-16

Base decision:

```text
GeneticFragment = inert Resource-like particle
```

Primary targets:

- `docs/genetics/horizontal-transfer.md`
- `docs/genetics/inheritance.md`
- `docs/biology/lifecycle.md`
- `docs/world/resources.md`
- `docs/config/resources_config.md`

GeneticFragment may have:

- `source_genome_ref` or `fragment_payload`;
- amount;
- location;
- stability;
- decay_rate;
- carrier_material;
- age;
- integrity.

Behavior:

- does not act;
- does not replicate;
- does not infect;
- does not mutate cells;
- does not modify Genome by itself;
- may diffuse, decay, be damaged, be blocked by boundaries, or be physically taken up only by explicit process.

Base HGT boundary:

- HGT is not active in the base model.
- Cell cannot integrate environmental fragments unless explicit future HGT process is enabled.
- Integration must pass Material capability, Feasibility, compatibility, cost and mutation/integration rules.

Core invariant:

```text
Genetic fragments are physical remnants, not active heredity.
They cannot change a living Genome without an explicit HGT/integration process.
They decay unless preserved by configured carrier material.
```

Future:

- separate `GeneticFragmentEntity` only if fragments need richer identity, lineage, recombination, carrier shells or active mobility.

### GZ-17

Define a read-only Observer Layer.

Primary targets:

- `docs/engine/ecs.md`
- `docs/biology/organism.md`
- `docs/evolution/selection.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/species-like-clusters.md`
- `docs/engine/rendering.md`
- `docs/engine/storage.md`
- `docs/engine/serialization.md`

Observer flow:

```text
Simulation state -> Observer Layer -> metrics / views / logs / rendering / export
```

Observer may compute:

- lineage ids;
- birth/death events;
- cell age;
- survival duration;
- division count;
- connected components;
- organism-like component size;
- resource totals;
- population counts;
- fragmentation events;
- extinction events;
- basic spatial distribution.

Rules:

- `OrganismView` is analytical view over connected Cell-Joint components.
- Fitness is post-hoc analytical metric, not input.
- Species-like clusters are observer-side groupings, not behavioral `species_id`.
- Observer metrics collected after committed simulation phases, preferably Statistics/Snapshot phase.
- Observer reads stable committed state only.

Export/reintroduction:

- allowed only as explicit research/scenario intervention;
- must be recorded in event log;
- must not happen automatically because of fitness, species cluster or observer metric values.

Core invariant:

```text
Observer views are read-only during normal simulation.
Metrics describe what happened; they do not cause what happens.
No cell may read observer metrics as local input.
Export/reintroduction is allowed only as explicit scenario intervention.
```

### GZ-18

Create `docs/config/stability_bounds.md`.

Do not hardcode final stability ranges upfront.

Define calibration and validation process:

```text
initial assumptions
  ↓
primitive balance calculator
  ↓
isolated stability tests
  ↓
scenario seed configs
  ↓
observed stable / unstable ranges
  ↓
config validation rules
```

Validation levels:

- fatal = config violates core invariants or cannot run;
- warning = config may run but likely creates unstable or hostile world;
- info = unusual but allowed experimental setting.

Calibration stages:

1. Smoke stability.
2. Single-cell survival.
3. Single-cell division.
4. Multicellular stability.
5. Specialization potential.
6. Evolution run.
7. Cognitive potential.

Balance calculator estimates:

- maintenance cost per tick;
- expected Energy intake;
- resource consumption rate;
- resource regeneration rate;
- reaction output balance;
- heat / waste accumulation;
- division affordability;
- material repair affordability;
- decay pressure.

Reference seed configs:

- `smoke_world`
- `single_cell_survival`
- `single_cell_division`
- `multicellular_stability`
- `basic_evolution`
- `hostile_world`

Time horizons:

- default `1 tick = 1 simulation second`;
- online target `10..30 ticks/sec` wall-clock;
- smoke horizon `1,000 ticks`;
- single-cell survival horizon `10,000 ticks`;
- division horizon `50,000 ticks`;
- basic evolution horizon `100,000..500,000 ticks`.

Core invariant:

```text
Stable config ranges are empirical contracts, not guesses.
A parameter becomes validated only after isolated tests and scenario runs show its effect.
Invalid configs must fail early or be clearly marked as hostile/experimental.
Stability is scenario-specific and horizon-specific.
```

## Files To Create

### `docs/config/stability_bounds.md`

Responsibility:

- define how stable/unstable config ranges are discovered;
- define validation levels;
- define calibration stages and time horizons;
- track empirical safe/warning/fatal ranges over time;
- document seed config expectations.

Required sections:

- Purpose
- Stability is empirical
- Scenario-specific and horizon-specific stability
- Validation levels
- Calibration pipeline
- Balance calculator
- Calibration stages
- Seed configs
- Earth-like baseline
- Collapse / instability categories
- Default time horizons
- Rules
- Related documents
- Open questions

Important rules:

- stability bounds are empirical contracts, not guesses;
- invalid configs fail early;
- hostile/experimental configs are allowed only when explicitly marked;
- defaults are calibration defaults, not biological truth.

### Optional later file: `docs/engine/observer.md`

Do not create immediately unless `engine/ecs.md`, `organism.md`, and evolution docs become too duplicated.

Possible responsibility:

- define Observer Layer, event logs, read-only metrics, export/reintroduction.

For current plan, prefer integrating Observer Layer rules into existing engine/evolution/biology docs first.

## Files To Modify Later

### `docs/README.md`

Add navigation for:

- `docs/config/stability_bounds.md`

If an observer file is created later, add it under `engine/`.

### `docs/GLOSSARY.md`

Add or refine terms:

- `Joint Channel`
- `Joint Endpoint`
- `Signal`
- `Signal State`
- `EpigeneticState`
- `RuntimeState`
- `MaterialState`
- `GeneticFragment`
- `Observer Layer`
- `OrganismView`
- `Fitness`
- `Species-like Cluster`
- `Stability Bound`
- `Validation Level`
- `Seed Config`
- `Scenario Horizon`

Clarify:

- signal is scalar stimulus, not command;
- fitness/species-like labels are observer metrics, not behavior inputs;
- genetic fragments are inert Resource-like particles unless explicit HGT process exists.

### `docs/biology/joint.md`

Expected edits:

- add Joint Contract;
- add Joint Channels;
- add Joint Creation;
- add Joint Upkeep;
- add Joint Damage and Break;
- add Joint During Division;
- add Joint During Death;
- align open questions with accepted rules;
- keep advanced strength/elasticity/damage accumulation minimal or marked future.

### `docs/biology/communication.md`

Expected edits:

- define base Signal as scalar local stimulus;
- remove/avoid typed-command signal semantics;
- define signal fields: value, source, medium, lifetime_ticks, decay_rate, delay_ticks, emission_cost;
- define receiver accumulation into `signal_state`;
- define same-tick rule;
- forbid direct movement/division/repair/attack/resource-selection commands.

### `docs/biology/processes.md`

Expected edits:

- signal production is planned action;
- signal adaptation uses RuntimeState/MaterialState, not Genome mutation;
- Joint creation/maintenance/strengthening are planned actions;
- Joint passive decay/break can be passive.

### `docs/biology/cell.md`

Expected edits:

- add/align state layers: Genome, EpigeneticState, RuntimeState, MaterialState;
- describe signal_state as RuntimeState;
- clarify cell does not read observer metrics.

### `docs/biology/lifecycle.md`

Expected edits:

- death may leave inert Joint/material connections;
- decomposition may produce GeneticFragments if configured;
- connect stability bounds with lifecycle horizons only by reference, not behavior.

### `docs/biology/organism.md`

Expected edits:

- clarify `OrganismView` as observer layer view;
- metrics are read-only;
- connected components are analysis, not controllers;
- no organism-level command bus.

### `docs/biology/specialization.md`

Expected edits:

- learning-like/specialization behavior through MaterialState/RuntimeState/EpigeneticState;
- no abstract `LearningState`;
- neural-like behavior from materials, shape, Joints and selection.

### `docs/genetics/epigenetics.md`

Expected edits:

- define EpigeneticState as regulatory modifier;
- distinguish EpigeneticState from RuntimeState and MaterialState;
- define inheritance policy: optionally inherited, attenuated or reset;
- forbid epigenetics rewriting Genome.

### `docs/genetics/inheritance.md`

Expected edits:

- add state-layer inheritance table:
  - Genome copied with mutation/recombination/HGT rules.
  - EpigeneticState optionally inherited/attenuated/reset.
  - RuntimeState usually reset.
  - MaterialState physically partitioned with Materials.
- align Joint inheritance with division partition and GZ-13.
- replace `viability check` wording if it implies hidden score.

### `docs/genetics/heredity.md`

Expected edits:

- align heredity with state-layer policy;
- clarify RuntimeState and MaterialState are not heredity unless explicit rule says otherwise;
- keep observer viability metrics non-behavioral.

### `docs/genetics/horizontal-transfer.md`

Expected edits:

- define GeneticFragment as inert Resource-like particle in base model;
- HGT inactive in base model;
- integration requires explicit future HGT process;
- no silent integration through resource uptake;
- separate future `GeneticFragmentEntity`.

### `docs/genetics/genome-runtime.md`

Expected edits:

- RuntimeState as short-term memory;
- signal_state as runtime input;
- no hidden fitness/observer metric input;
- runtime memory not heredity.

### `docs/world/resources.md`

Expected edits:

- genetic fragments may be Resource-like physical remnants;
- they are not ordinary nutrients unless decay/digestion rule converts them;
- no active heredity without HGT process.

### `docs/config/resources_config.md`

Expected edits:

- if GeneticFragment is configured, define stability/decay/carrier fields;
- mark base fragments inert and non-executable.

### `docs/config/materials_config.md`

Expected edits:

- include signal-plastic material properties for MaterialState;
- include Joint channel-supporting properties where already planned:
  - signal_conductivity;
  - resource_transport_support;
  - heat_conductivity;
  - genetic_fragment_permeability if future-enabled.

### `docs/config/world_config.md`

Expected edits:

- link `tick_duration` and validation expectations to `stability_bounds.md`;
- clarify `tick_duration` default may be `1 simulation second`, but not biological truth;
- configs can be hostile/experimental if marked.

### `docs/config/fields_config.md`

Expected edits:

- validation levels for field intensity, decay, temporal modes;
- reference stability bounds.

### `docs/config/reactions_config.md`

Expected edits after creation from GZ-10:

- reference stability bounds for reaction rates, heat release, waste accumulation.

### `docs/engine/ecs.md`

Expected edits:

- Observer Layer / OrganismView read-only;
- no metrics as behavior input;
- Joint remains one entity/object with material-derived channels, not separate hardcoded biological joint classes.

### `docs/engine/scheduler.md`

Expected edits:

- Observer/Statistics phase reads committed state only;
- signals emitted Tick N visible no earlier than Tick N+1 unless explicit extension;
- export/reintroduction is scenario action, not normal phase feedback.

### `docs/engine/rendering.md`

Expected edits:

- rendering reads Observer Layer / committed state;
- renderer/UI cannot mutate simulation state.

### `docs/engine/storage.md`

Expected edits:

- snapshot export/reintroduction as explicit scenario intervention;
- record event logs for manual reintroduction.

### `docs/engine/serialization.md`

Expected edits:

- include observer/export snapshots;
- clarify simulation state vs observer metrics persistence.

### `docs/evolution/selection.md`

Expected edits:

- fitness as post-hoc observer metric;
- no fitness input;
- export/reintroduction is explicit intervention, not selection pressure.

### `docs/evolution/population-dynamics.md`

Expected edits:

- observer-side metrics list;
- population totals, extinction events, lineage survival, division counts.

### `docs/evolution/species-like-clusters.md`

Expected edits:

- species-like clusters are observer groupings;
- no species_id behavior;
- no automatic compatibility effects unless physical/genetic mechanism exists.

## Proposed Implementation Order After All 18 Answers Are Collected

Do not apply yet. When user says "виконуй", use this order for GZ-13..GZ-18:

1. Create `docs/config/stability_bounds.md`.
2. Update `docs/biology/joint.md`.
3. Update `docs/biology/communication.md`.
4. Update state-layer docs:
   - `docs/biology/cell.md`
   - `docs/genetics/epigenetics.md`
   - `docs/genetics/inheritance.md`
   - `docs/genetics/heredity.md`
   - `docs/genetics/genome-runtime.md`
5. Update HGT/genetic fragment docs:
   - `docs/genetics/horizontal-transfer.md`
   - `docs/world/resources.md`
   - `docs/config/resources_config.md`
6. Update lifecycle/specialization:
   - `docs/biology/lifecycle.md`
   - `docs/biology/processes.md`
   - `docs/biology/specialization.md`
7. Update observer/evolution docs:
   - `docs/biology/organism.md`
   - `docs/engine/ecs.md`
   - `docs/evolution/selection.md`
   - `docs/evolution/population-dynamics.md`
   - `docs/evolution/species-like-clusters.md`
8. Update engine IO docs:
   - `docs/engine/scheduler.md`
   - `docs/engine/rendering.md`
   - `docs/engine/storage.md`
   - `docs/engine/serialization.md`
9. Update config docs:
   - `docs/config/world_config.md`
   - `docs/config/fields_config.md`
   - `docs/config/materials_config.md`
   - `docs/config/reactions_config.md` after it exists.
10. Update `docs/README.md` and `docs/GLOSSARY.md`.
11. Run verification searches.
12. Create implementation report after applying.

## Verification Plan For Future Execution

Run after applying changes:

```powershell
rg -n --encoding utf-8 "stability_bounds.md|Joint Contract|Joint Channels|Signal = local scalar stimulus|RuntimeState|MaterialState|GeneticFragment|Observer Layer|Scenario Horizon" docs README.md
rg -n --encoding utf-8 "ForceJoint|SignalJoint|ResourceJoint|typed command|LearningState|fitness_score input|species_id behavior|automatic.*observer|silent integration" docs
rg -n --encoding utf-8 "Energy Buffer directly|Signal is not a command|Learning is not mutation|Genetic fragments are physical remnants|Observer views are read-only|Stability is scenario-specific" docs
rg -n --encoding utf-8 "MVP|ADR-000X|TODO|TBD|перша реалізація" docs
git status --short
```

Expected:

- no hardcoded Joint subclasses are introduced;
- signal remains scalar stimulus, not typed command;
- learning/runtime/material plasticity does not mutate Genome;
- genetic fragments are inert without explicit HGT process;
- observer metrics cannot feed back into behavior;
- stability bounds are empirical and scenario/horizon-specific;
- no MVP or ADR placeholder wording is reintroduced.

## Non-Blocking Clarifications To Resolve Later

### C-01. Joint advanced physics scope

User explicitly warns not to turn Joint into a small physics engine before base life exists.

Recommended resolution:

- base Joint tracks endpoints, channels, simple validity and break conditions;
- advanced strength/elasticity/damage accumulation can be documented as future-compatible fields, but not required for first stable life documentation.

### C-02. Signal scale choice

User allows `0.0..1.0` or `-1.0..+1.0`.

Recommended resolution:

- use `-1.0..+1.0` for normalized internal/regulatory signal_state because suppression and excitation are both useful;
- allow raw emitted signal packet to be `0.0..1.0` for intensity when medium does not support negative signal;
- document mapping explicitly in `communication.md`.

### C-03. GeneticFragment as Resource-like but not ordinary nutrient

Recommended resolution:

- Canon term: `Resource-like particle`, not `ResourceType`, unless config explicitly models it as a Resource;
- it may decay into ordinary Resources through reaction/decomposition rules;
- normal resource uptake cannot integrate it into Genome.

### C-04. Export/reintroduction boundary

Recommended resolution:

- normal simulation cannot use Observer metrics for behavior;
- explicit scenario intervention can import/export snapshots;
- import/reintroduction must create event log record and should be disabled in ordinary autonomous runs unless scenario config allows it.

### C-05. Stability horizon defaults

The proposed horizons are calibration defaults, not biological truth.

Recommended resolution:

- put these in `stability_bounds.md` as initial defaults;
- later configs may override scenario horizon;
- never treat "stable" without scenario + tick horizon.
