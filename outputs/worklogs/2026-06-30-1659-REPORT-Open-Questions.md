---
tags:
  - alife
  - worklog/report
---

# REPORT: Open Questions

## Goal

Поступово пройти `Open Questions`, погоджувати рішення в дискусії й одразу фіксувати їх у документації.

## OQ-01: Minimal Cell fields

Status: resolved.

File: `docs/biology/cell.md`.

Decision:

```text
Cell
├── id
├── position
├── radius
├── physical_state
├── resources
├── materials
├── energy_buffer
├── temperature
├── genome_state
├── epigenetic_state
├── runtime_state
├── joints
├── local_inputs
├── lifecycle_state
├── process_progress
└── debug_metrics
```

Notes:

- `temperature` is explicit local cell state.
- `debug_metrics` is observer-only and must not affect behavior, Genome Runtime or Feasibility Check.
- Boundary is not part of the minimal `Cell` field set; it is derived from Materials and `biology/membrane.md` unless a later decision changes that.

Documentation changes:

- Updated minimal `Cell` state.
- Added clarification for `temperature`.
- Added observer-only rule for `debug_metrics`.
- Removed the resolved Open Question: `Мінімальний набір fields для першої Cell entity`.

## Remaining nearby questions

- Minimal `runtime_state` for signal accumulation and cooldowns.
- Minimal debug metrics that can be stored without affecting behavior.

## OQ-02: Boundary model

Status: resolved.

Files:

- `docs/biology/cell.md`
- `docs/biology/membrane.md`

Decision:

Boundary remains a derived aggregate property of cell Materials. It is not a separate engine entity, component, organ, membrane object or hardcoded behavior holder.

Rationale:

- The term exists to explain which material properties separate the Cell interior from the environment.
- A separate component would add implementation complexity without adding a needed rule at the current level.
- Materials already provide the physical basis for permeability, protection, contact, sensing and Joint formation.

Documentation changes:

- Clarified in `cell.md` that Boundary is a name for an aggregate Material property.
- Clarified in `membrane.md` that Boundary is not a separate entity/component.
- Removed the resolved Boundary/component Open Questions from `cell.md` and `membrane.md`.

## Remaining nearby questions

- Minimal `runtime_state` for signal accumulation and cooldowns.
- Minimal debug metrics that can be stored without affecting behavior.
- Minimal permeability model for the first implementation.
- How Boundary damage affects leakage, uptake and death thresholds.

## OQ-03: Minimal runtime_state

Status: resolved.

Files:

- `docs/biology/cell.md`
- `docs/genetics/genome-runtime.md`

Decision:

```text
runtime_state
├── active_processes
│   └── long-running process progress
├── process_cooldowns
│   └── refractory/cooldown timers
├── signal_state
│   └── accumulated scalar signal, decay, threshold state
├── last_decision_inputs
│   └── committed local inputs used by Genome Runtime
├── last_regulatory_outputs
│   └── bounded Genome Runtime outputs / priorities
├── action_plan
│   └── planned candidate actions for this Tick
├── feasibility_result
│   └── allowed/rejected actions and reasons
└── runtime_flags
    └── temporary execution/lifecycle flags
```

Notes:

- `signal_state` is broader than signal accumulators and may include decay and threshold state.
- `last_regulatory_outputs` is explicitly last-tick/current-tick runtime output, not hereditary state or long-term memory.
- `action_plan` is the intermediate result between Genome Runtime priorities and Feasibility Check.
- `feasibility_result` stores allowed/rejected actions and reasons for debug and stable tick logic, not as cell memory.
- `runtime_flags` are temporary execution/lifecycle flags such as `stalled`, `mandatory_paid`, `dormant`, `inert`, `division_ready`, `over_capacity`, `blocked_by_cooldown`.

Documentation changes:

- Added minimal `runtime_state` tree to `cell.md`.
- Clarified that `runtime_state` is technical local execution state, not behavior script or long-term memory.
- Updated `genome-runtime.md` pipeline and trace names to use `last_decision_inputs`, `last_regulatory_outputs`, `action_plan` and `feasibility_result`.
- Removed the resolved runtime_state Open Question from `cell.md`.

## Remaining nearby questions

- Minimal debug metrics that can be stored without affecting behavior.
- Minimal permeability model for the first implementation.
- How Boundary damage affects leakage, uptake and death thresholds.

## OQ-04: Minimal debug_metrics

Status: resolved.

File: `docs/biology/cell.md`.

Decision:

```text
debug_metrics
├── age_ticks
├── divisions_count
├── stress_level
├── last_feasibility_summary
├── last_rejection_reasons
├── energy_balance_snapshot
├── capacity_snapshot
└── lineage_ref
```

Rules:

- `debug_metrics` is observer-only.
- Genome Runtime, Feasibility Check and Processes must not read `debug_metrics` as input.
- `stress_level` is a derived debug summary for observer/debug UI, not real Cell state, not regulatory input and not a behavior cause.

Documentation changes:

- Added minimal `debug_metrics` tree to `cell.md`.
- Added explicit observer-only rule.
- Removed the resolved debug metrics Open Question from `cell.md`.

## Remaining nearby questions

- Minimal permeability model for the first implementation.
- How Boundary damage affects leakage, uptake and death thresholds.

## OQ-05: Boundary permeability and damage

Status: resolved.

Files:

- `docs/biology/membrane.md`
- `docs/config/resources_config.md`
- `docs/config/materials_config.md`
- `docs/biology/feasibility.md`

Decision:

No Resource crosses Boundary by default. Resource exchange requires a Boundary permeability rule:

```text
Resource physical traits + Boundary Material -> blocked | passive | active_required
```

Boundary state:

```text
boundary_state
├── integrity: 0.0..1.0
├── default_permeability: blocked
├── permeability_by_resource_class
├── permeability_by_resource_id
├── leakage_rate
├── uptake_modifier
├── export_modifier
└── failure_thresholds
    ├── leakage_threshold
    ├── uncontrolled_exchange_threshold
    └── death_threshold
```

Rules:

- `blocked`: Resource does not cross normal Boundary.
- `passive`: Resource may cross by gradient/diffusion without Genome decision.
- `active_required`: Resource may cross only through uptake/export process with Material capability, Energy and Feasibility.
- Boundary damage increases leakage and reduces control, but does not make all Resources freely pass.
- Strong damage may allow uncontrolled exchange only for physically compatible tiny/small Resource classes.
- Large fragments, genetic fragments and incompatible Resources remain blocked unless an explicit future process says otherwise.
- Reactive/corrosive Resources may damage Boundary from outside through reaction/material degradation rules.

Documentation changes:

- Added Boundary permeability invariant and damage rules to `membrane.md`.
- Added Resource physical traits to `resources_config.md`: `size_class`, `phase`, `reactivity_class`, optional polarity-like tag, `permeability_class`.
- Added Boundary Material permeability rules to `materials_config.md`.
- Added Feasibility checks for uptake/export in `feasibility.md`.
- Removed resolved permeability/damage Open Questions from `membrane.md`.

## Remaining nearby questions

- None in `cell.md`.
- None in `membrane.md`.

## OQ-06: Base communication signal model

Status: resolved.

Files:

- `docs/biology/communication.md`
- `docs/biology/cell.md`
- `docs/biology/joint.md`

Decision:

Base communication uses one scalar signal type only:

```text
signal_level: 0.0..1.0
```

There are no typed signals such as `damage`, `food`, `pain`, `move`, `stress` or `resource_need`.

Signal meaning is not stored in the signal itself. Meaning emerges from the receiver's Materials, RuntimeState, local context and Genome Runtime.

Typed or vector signals are not part of the base model and require an explicit ADR.

If future work needs different chemical-like signals, they should be modeled as different Resource-like trace substances with physical properties, not as typed commands.

Rules:

- Signal is a physical local scalar impulse.
- Signal may be transmitted through contact, Joint or local material trace.
- Signals emitted during Tick N become external input for receivers at the start of Tick N+1.
- Same-tick command loops are forbidden.
- Base model uses RuntimeState for short-term signal accumulation, MaterialState for physical signal-sensitive/plastic changes and JointState for propagation through joints.
- EpigeneticState is not modified by ordinary signals in the base model.
- Material trace that affects cells must be physical `signal_trace_resource`; debug-only trace must not be readable by cells.
- Different chemical-like signal traces are modeled as different Resource-like trace substances, not typed semantic messages.
- Communication debug trace is observer-only.

Documentation changes:

- Replaced open communication questions with scalar `signal_level` contract.
- Removed typed signals from base/future channel list.
- Added ADR requirement for future typed/vector signal models.
- Clarified that future chemical-like signals should be Resource-like trace substances with physical properties.
- Added Tick N -> Tick N+1 visibility rule.
- Added State Layers, Material Trace, Debug Communication Trace and invariant sections.
- Updated `cell.md` `runtime_state.signal_state` wording to use `signal_level`.
- Aligned `joint.md` signal channel with Tick N -> Tick N+1 readability and removed the resolved Joint signal delay Open Question.

## OQ-07: Cleanup of answered or non-question Open Questions

Status: resolved.

Files:

- `docs/PRINCIPLES.md`
- `docs/world/laws.md`
- `docs/world/philosophy.md`
- `docs/GLOSSARY.md`
- `docs/biology/lifecycle.md`

Decision:

Several `Open Questions` sections were not active unresolved requirements:

- `PRINCIPLES.md`, `world/laws.md` and `world/philosophy.md` state that their core model is stable and only implementation details remain.
- `GLOSSARY.md` listed future glossary terms, not unresolved model questions.
- `lifecycle.md` lifecycle debug metrics were already mostly resolved by `cell.debug_metrics`.

Documentation changes:

- Renamed non-question sections to `Implementation Notes` or `Future Glossary Terms`.
- Added `Lifecycle Debug Metrics` to `lifecycle.md`.
- Clarified that lifecycle debug fields are observer-only and must not be read by cells, Genome Runtime, Feasibility or Processes.
- Removed the resolved lifecycle metrics Open Question from `lifecycle.md`.

## Remaining open-question groups

- `docs/biology/genome.md`
- `docs/biology/joint.md`
- `docs/biology/lifecycle.md`
- `docs/biology/organism.md`
- `docs/biology/processes.md`
- `docs/biology/specialization.md`
- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-08: Joint base behavior

Status: resolved.

Files:

- `docs/biology/joint.md`
- `docs/biology/division-partition.md`

Decision:

- External Joints break by default during division.
- Joint preservation/reassignment during division requires an explicit future rule.
- First implementation uses passive Resource transfer only, if Joint has `resource_channel`.
- Active directed Resource transfer is post-MVP/future extension.
- Joint does not transfer Energy Buffer directly.
- Endpoint death disables active channels immediately.
- Remaining Joint material may persist as inert material connection and degrade by `joint_material.decay_rate`, environment modifiers, heat/pressure/reaction damage and missing upkeep.
- HGT through Joint is disabled in the first implementation.
- Any future HGT through Joint must be explicit and must not reuse normal resource transfer silently.

Passive transfer:

```text
passive_transfer =
  min(
    gradient * transfer_rate * joint_integrity,
    max_transfer_per_tick,
    available_source_amount,
    free_target_capacity
  )

gradient = max(0, source_concentration - target_concentration)
```

Documentation changes:

- Added Resource Transfer, Division and Death, HGT and Summary rules to `joint.md`.
- Removed all Open Questions from `joint.md`.
- Updated `division-partition.md` to match default external Joint break behavior.

## Remaining open-question groups

- `docs/biology/genome.md`
- `docs/biology/lifecycle.md`
- `docs/biology/organism.md`
- `docs/biology/processes.md`
- `docs/biology/specialization.md`
- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-09: Genome physical carrier, damage and trace

Status: resolved.

Files:

- `docs/biology/genome.md`
- `docs/genetics/mutation.md`

Decision:

Genome in the first implementation is:

```text
Genome information + physical carrier state
```

Minimum carrier state:

```text
genome_carrier
├── integrity: 0.0..1.0
├── amount
├── material_id
└── copy_progress
```

Rules:

- Genome information is stored on a physical carrier.
- The carrier occupies capacity, has integrity and can be damaged.
- `genome_damage = degradation of genome carrier`.
- Genome carrier damage is tracked as `genome_carrier_state`, not as mutation of the regulatory graph.
- Carrier damage may cause runtime errors, reduced regulatory stability, copy errors, mutation during explicit repair/copying mechanisms or nonfunctional Genome.
- Carrier damage is not automatically mutation.
- Mutation, repair, copying and HGT are explicit mechanisms.
- Genome Trace is observer-only.

Minimum Genome Trace:

```text
genome_trace
├── genome_id
├── parent_genome_id
├── carrier_integrity
├── copy_progress
├── mutation_events
├── repair_events
├── copy_errors
├── hgt_events
└── division_copy_result
```

Documentation changes:

- Added `genome_carrier`, Carrier Damage, Genome Trace, Future Carrier Extensions and Invariant sections to `genome.md`.
- Clarified in `mutation.md` that carrier damage is not automatically mutation.
- Removed all Open Questions from `genome.md`.

## Remaining open-question groups

- `docs/biology/lifecycle.md`
- `docs/biology/organism.md`
- `docs/biology/processes.md`
- `docs/biology/specialization.md`
- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-10: Lifecycle partition, dormancy and decomposition

Status: resolved.

Files:

- `docs/biology/lifecycle.md`
- `docs/biology/division-partition.md`

Decision:

Starting division partition coefficients:

```text
default_split = 0.5
split_noise = ±0.15
allowed_split_range = 0.35..0.65
```

Resources, Materials, Energy Buffer and MaterialState split noisy-proportionally.

Genome information is not partitioned. It is copied onto a second physical carrier before partition.

Dormancy modifiers:

```text
mandatory_energy_cost_modifier = 0.25
degradation_rate_modifier = 0.5
active_transport_modifier = 0.0
synthesis_modifier = 0.0
signal_emit_modifier = 0.0
repair_modifier = 0.25
```

Decomposition rates come from Material properties:

```text
material.decay_rate
material.stability
environment_modifiers
temperature
reactivity
```

Starting material decay categories:

```text
soft/internal material      -> fast decay
boundary material           -> medium decay
structural/joint material   -> slow decay
inert/mineral-like material -> very slow decay
```

Genome fragments after death exist only as inert Resource-like particles in the first implementation:

```text
dead genome carrier
  -> genetic fragments
  -> decay over time
  -> inert waste/resources
```

Rules:

- Dormancy reduces activity, but does not stop all costs.
- Dormant cells do not grow, synthesize, move, actively transport or emit active signals by default.
- Dormant cells still pay minimal stability costs and degrade slowly.
- Genome fragments after death do not integrate into living cells automatically, do not mutate a living Genome and do not start HGT without a separate explicit process.

Documentation changes:

- Added partition coefficients to `division-partition.md`.
- Added division coefficients, Dormancy, decomposition rates, genome fragments after death and invariant sections to `lifecycle.md`.
- Removed all Open Questions from `lifecycle.md`.

## Remaining open-question groups

- `docs/biology/organism.md`
- `docs/biology/processes.md`
- `docs/biology/specialization.md`
- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-11: OrganismView detection and metrics

Status: resolved.

File:

- `docs/biology/organism.md`

Decision:

Detection in the first implementation:

```text
OrganismView = connected component of Cell-Joint graph
```

Dependency score is not used as an existence condition for organism-like structures. It may be computed separately for debug/research.

Minimum dependency/debug metrics:

```text
cell_count
joint_count
component_age_ticks
connectedness
shared_lineage_ratio
resource_transfer_edges_count
signal_edges_count
fragmentation_events_count
merge_events_count
collapse_reason
```

Starting connectedness:

```text
connectedness = joint_count / max(1, cell_count - 1)
```

View lineage uses observer-only events:

```text
component_created
component_split
component_merged
component_collapsed
component_extinct
```

Minimum OrganismView:

```text
OrganismView
├── view_id
├── tick_created
├── tick_updated
├── cell_ids optional
├── cell_count
├── joint_count
├── bounding_box
├── center_of_mass
├── dominant_lineage_refs
├── component_age_ticks
├── resource_flow_summary
├── signal_flow_summary
└── event_refs
```

Rules:

- OrganismView is observer-only.
- Dependency metrics describe structure, but do not define behavior.
- Cells cannot read OrganismView, dependency score, fitness or component id.
- `cell_ids` may be stored only in selected/debug mode for performance.

Documentation changes:

- Replaced candidate/dependency detection ambiguity with connected-component detection.
- Added dependency metrics, view lineage events, minimum OrganismView and invariant.
- Removed all Open Questions from `organism.md`.

## Remaining open-question groups

- `docs/biology/processes.md`
- `docs/biology/specialization.md`
- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-12: Process registry contract, failure modes and formulas

Status: resolved.

Files:

- `docs/biology/processes.md`
- `docs/biology/action-process-registry.md`

Decision:

Each executable process is defined by the Action / Process Registry. `processes.md` describes process logic and shared rules, but does not duplicate the full registry.

Minimum registry entry:

```text
process_id
kind
status: now | future
duration: atomic | long_running
required_capabilities
required_inputs
energy_cost
material_cost
output/effect
feasibility_rules
failure_modes
```

Failure mode rules:

- Feasibility reject before execution -> `rejected_no_effect`.
- Failure during execution -> explicit consequences from the process rule.
- Long-running failures must say whether progress is paused, decayed, discarded or converted into damage/waste/heat.

Allowed failure outcomes:

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

Placeholder formulas:

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

Rules:

- Energy production does not create matter.
- Genome outputs only priorities for registered allowed processes.
- Feasibility accepts only registered actions or controlled reactions.
- Future process groups may exist in schema, but must not execute until explicitly enabled in the registry.

Documentation changes:

- Replaced unresolved Open Questions in `processes.md` with process contract, failure modes, placeholder formulas and future-process invariant.
- Replaced `implementation_level` with `status: now | future` in `action-process-registry.md`.
- Expanded the base process table so each current process has minimum fields: capabilities, inputs, costs, effects, feasibility rules and failure modes.
- Marked future-compatible process groups as non-executable until promoted to `status: now`.

## Remaining open-question groups

- `docs/biology/specialization.md`
- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-13: SpecializationProfile and signal-plastic Materials

Status: resolved.

Files:

- `docs/biology/specialization.md`
- `docs/world/materials.md`
- `docs/config/materials_config.md`
- `docs/biology/division-partition.md`

Decision:

`SpecializationProfile` is observer-only. It is not a cell type and must not affect behavior.

Minimum observer metrics:

```text
SpecializationProfile
├── dominant_materials
├── dominant_processes
├── average_regulatory_outputs
├── signal_sensitivity_level
├── signal_conductivity_level
├── storage_ratio
├── repair_ratio
├── movement_ratio
├── boundary_ratio
├── joint_ratio
└── stability_ticks
```

Minimum signal-plastic Material properties:

```text
signal_sensitivity
signal_storage
signal_conductivity
```

Minimum signal-plastic MaterialState:

```text
stored_signal
fatigue
conductivity_modifier
```

Temporary state vs stable specialization:

```text
temporary state:
  short-lived changes in runtime_state or MaterialState

stable specialization:
  repeated process bias + stable material composition + persistence over N ticks
```

Debug UI may use:

```text
state_now
profile_window
stability_ticks
profile_confidence
```

Rules:

- Specialization is inferred from persistent material/process patterns.
- Temporary RuntimeState is not specialization.
- No cell may read its `SpecializationProfile` as behavior input.
- Asymmetric inheritance is future-compatible only and must not appear implicitly in the base division model.

Documentation changes:

- Replaced all Open Questions in `specialization.md` with observer metrics, signal-plastic Material rules, inheritance boundary and temporary/stable distinction.
- Added signal-plastic minimum properties to `materials.md` and `materials_config.md`.
- Clarified in `division-partition.md` that asymmetric inheritance is not base behavior and requires an explicit future rule.

## Remaining open-question groups

- `docs/world/space.md`
- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-14: Space partition, radii and boundary modes

Status: resolved.

Files:

- `docs/world/space.md`
- `docs/config/world_config.md`
- `docs/world/units.md`
- `docs/engine/physics.md`

Decision:

Base spatial partition:

```text
spatial_partition = uniform_spatial_grid

Cells      -> entity + spatial grid index
Joints     -> entity links between cells
Resources  -> grid / sparse grid
Fields     -> grid or function layer
Traces     -> grid / sparse grid
```

Starting radii and grid size:

```text
cell_radius_default = 1.0 su
cell_radius_min = 0.5 su
cell_radius_max = 3.0 su
sense_radius = 4.0 su
uptake_radius = 1.5 su
contact_radius = cell_radius_a + cell_radius_b
joint_creation_radius = contact_radius + 0.5 su
signal_radius = 4.0 su
spatial_grid_size = 8.0 su
```

Default boundary mode:

```text
boundary_mode = solid_wall
```

Boundary semantics:

- `wrapped`: Cells, Resources and Fields wrap around world edges.
- `solid_wall`: Cells collide/stop; Resources reflect, accumulate or stop by resource rule; Fields are clamped or use configured edge values.
- `open`: Cells cannot leave unless explicit outflow/removal rule exists; Resources may leave and be removed; Fields may use external/ambient boundary values.

Rules:

- Space locality is resolved through one spatial grid contract.
- Cells are entities indexed by position.
- Resources/traces are grid or sparse-grid quantities.
- Fields are sampled locally.
- Boundary mode must explicitly define behavior for Cells, Resources and Fields.
- Chunks/streaming/adaptive indexes are future optimizations, not base semantics.

Documentation changes:

- Replaced all Open Questions in `space.md` with spatial partition, locality radii, boundary modes and invariant.
- Updated `world_config.md` default schema from `closed`/old radius max to `solid_wall`, `uniform_spatial_grid`, `cell_radius_max = 3.0` and starting radii.
- Updated `units.md` baseline ranges for cell radius, sensing, uptake, signal radius and grid size.
- Added physics note that first implementation locality queries use the uniform spatial grid contract.

## Remaining open-question groups

- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-15: Adaptation debug levels, metrics and shift logs

Status: resolved.

Files:

- `docs/evolution/adaptation.md`
- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`

Decision:

Debug UI adaptation levels:

```text
cell_state_adaptation
material_adaptation
lineage_adaptation
population_shift
```

Definitions:

```text
cell_state_adaptation =
  зміни runtime/epigenetic/material state в межах життя клітини

material_adaptation =
  зміни Materials: sensitivity, conductivity, storage, damage tolerance

lineage_adaptation =
  спадкові зміни Genome/EpigeneticState у нащадків

population_shift =
  зміна частот lineage/genome/material profiles у популяції
```

Lifetime vs lineage:

```text
lifetime = cell changed during life
lineage = descendants changed across generations
```

No single base `adaptation_score`. Use observer-only metrics:

```text
survival_delta
division_rate_delta
material_profile_shift
genome_profile_shift
resource_efficiency_shift
stress_resilience_shift
population_frequency_shift
```

Adaptive shifts are logged as rolling-window events:

```text
adaptive_shift_event
├── tick_range
├── lineage_ref
├── population_before
├── population_after
├── dominant_material_changes
├── dominant_genome_changes
├── survival_change
├── division_change
└── environment_context
```

Starting windows:

```text
last 1_000 ticks
last 10_000 ticks
```

Rules:

- Adaptation is observer interpretation, not a cell input.
- Lifetime adaptation changes an individual during life.
- Lineage adaptation is visible through descendants and population shifts.
- No adaptation metric may affect Genome Runtime, Feasibility, selection or behavior.
- Future aggregate `adaptation_score` is allowed only as observer-only analysis.

Documentation changes:

- Replaced all Open Questions in `adaptation.md` with debug levels, lifetime/lineage boundary, observer metrics and adaptive shift event contract.
- Added adaptive shift event logging to `population-dynamics.md`.
- Clarified in `selection.md` that adaptation metrics must not become selection inputs/controllers.

## Remaining open-question groups

- `docs/evolution/population-dynamics.md`
- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-16: Population metrics, lineage events and bounded snapshots

Status: resolved.

Files:

- `docs/evolution/population-dynamics.md`
- `docs/engine/storage.md`

Decision:

Minimum observer-only population metrics:

```text
population_count
alive_cells_count
dead_cells_count
births_per_window
deaths_per_window
divisions_per_window
lineage_count
extinct_lineages_count
average_cell_age
resource_pressure_summary
```

Lineage tree is logged as events, not as a full tree in every snapshot:

```text
cell_birth
cell_division
cell_death
parent_cell_id
daughter_cell_ids
lineage_ref
tick
```

Base genome variant tracking:

```text
genome_id
parent_genome_id
mutation_count
lineage_ref
```

Population snapshots store interval aggregates:

```text
every N ticks:
  population totals
  lineage summaries
  birth/death/division counters
  top lineage counts
  resource/environment summary
```

Rules:

- Population dynamics are observer-only.
- Population metrics describe survival, reproduction and extinction.
- Metrics must not influence Genome Runtime, Feasibility, selection or behavior.
- Genome variant clustering is observer/research layer only.
- Full snapshots are rare or manually requested; detailed traces are debug/selected-run features.

Documentation changes:

- Replaced all Open Questions in `population-dynamics.md` with required metrics, lineage event log, genome variant tracking and bounded snapshot policy.
- Added population event/aggregate storage contract to `storage.md`.

## Remaining open-question groups

- `docs/evolution/selection.md`
- `docs/evolution/species-like-clusters.md`

## OQ-17: Selection metrics, levels and drift distinction

Status: resolved.

Files:

- `docs/evolution/selection.md`
- `docs/evolution/population-dynamics.md`

Decision:

Base selection analysis is observer-only and lineage-first.

Minimum selection metrics:

```text
survival_time_by_lineage
division_count_by_lineage
offspring_count_by_lineage
death_rate_by_lineage
lineage_frequency_over_time
resource_efficiency_by_lineage
stress_survival_by_lineage
```

Analysis level:

```text
selection analysis = lineage-level first
organism-like analysis = derived observer view
```

OrganismView may be analyzed separately with:

```text
component_lifetime
component_cell_count
fragmentation_events
viable_child_components
collapse_reason
```

Selection vs drift logging:

```text
observed_frequency_shift
├── tick_range
├── lineage_ref
├── frequency_before
├── frequency_after
├── survival_context
├── division_context
├── resource_efficiency_context
├── population_size_context
└── environment_context
```

Interpretation:

```text
if lineage частішає разом із кращим survival/division/resource efficiency:
  possible selection

if lineage частішає без стабільної переваги або при малих числах:
  possible drift
```

Minimum population-level metrics for selection analysis:

```text
population_count
births_per_window
deaths_per_window
divisions_per_window
lineage_count
lineage_frequency_distribution
extinction_events
average_survival_time
average_division_rate
resource_pressure_summary
environment_context
```

Rules:

- Selection is an observer interpretation of differential survival and reproduction.
- Cells do not read selection metrics.
- No fitness score is used as behavior input.
- Frequency shifts should be logged first, then interpreted as possible selection or drift.

Documentation changes:

- Replaced all Open Questions in `selection.md` with lineage-first metrics, organism-like derived analysis, observed frequency shift logs and population-level metrics.
- Added `observed_frequency_shift` note to `population-dynamics.md`.

## Remaining open-question groups

- `docs/evolution/species-like-clusters.md`

## OQ-18: Species-like clusters as observer-only analysis

Status: resolved.

Files:

- `docs/evolution/species-like-clusters.md`
- `docs/biology/organism.md`

Decision:

Species-like clustering is not base behavior.

Base identity data:

```text
lineage_ref
genome_id
parent_genome_id
mutation_count
```

Base cluster metric:

```text
lineage_distance
```

Optional metric:

```text
genome_similarity
```

`genome_similarity` may be computed periodically or for selected samples, not every Tick by default.

Future metric:

```text
fragment_sharing
```

Mixed organism-like structures are analyzed as component composition:

```text
OrganismView
├── dominant_lineage_refs
├── lineage_distribution
├── genome_distribution
└── mixedness_score
```

Fuzzy debug labels are allowed:

```text
cluster_A
cluster_B
mixed_cluster
unclassified
```

They must be shown as:

```text
observer-only inferred cluster
```

Rules:

- Species-like clusters are observer-only.
- They are inferred from lineage/genome similarity, not used by cells.
- No `species_id` exists in behavior.
- Mixed organisms are analyzed by lineage/genome composition, not assigned a hard species.
- Cluster labels do not affect compatibility, behavior or reproduction.

Documentation changes:

- Replaced all Open Questions in `species-like-clusters.md` with base identity data, metrics policy, mixed structure analysis and debug label rules.
- Added `lineage_distribution`, `genome_distribution` and `mixedness_score` to `OrganismView` observer metrics.

## Remaining open-question groups

- None in domain documentation.
- `docs/STYLE_GUIDE.md` still has a non-domain `Open Questions` section.
