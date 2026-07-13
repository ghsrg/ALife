# PLAN: Phase 3 Global Roadmap

## Goal

Define Phase 3 as the Genome phase: heritable regulatory control, mutation, inheritance and lineage over the material, chemistry and persistent-interaction body completed in Phase 2.

Phase 3 depends on the Phase 2 material/process body being stable enough for registered process prioritization. It regulates registered reactions, material repair and Joint intents; it does not introduce their physical mechanisms.

Runner can start after Phase 3A when demo scenarios can create initial Cells with deterministic Genome-derived process priorities. Runner must not depend on Genome internals directly; this is a scenario/config readiness dependency, not a Runner architecture dependency.

## Core Principle

Genome is not a brain, behavior script or command source.

Correct model:

```text
Materials + local state + context define possible processes.
Genome regulates priorities, thresholds, synthesis bias, cadence and sensitivity.
Feasibility decides what can actually execute.
```

Genome cannot:

```text
create missing material capability
bypass Feasibility
read observer metrics
issue commands outside registered processes
encode a hardcoded body plan
create a Reaction, ResourceType, MaterialType or Joint channel outside the registered model
```

## Phase 3A: Genome Bootstrap Vertical Slice

### Goal

Introduce the smallest canonical Genome slice that can seed initial Cells with deterministic variation and affect feasible process ordering.

This phase intentionally changes behavior through registered priority outputs. It is not a full Genome Runtime, inheritance or mutation phase.

### Build

```text
GenomeId
GenomeCarrierState
carrier integrity
carrier amount/capacity cost
minimal physical carrier assignment for initial Cells
genome template config
seeded per-Cell template variation
constant-output direct regulatory graph representation
bounded regulatory output values
registered output vocabulary validation
capability/material mask before executable intent
ActionPlan from priorities
deterministic process priority sorting
genome registry/storage
observer-only genome/runtime trace
config loading for initial genome templates
deterministic replay coverage
```

### Gate

```text
Genome occupies physical carrier/capacity
Genome is copied/stored as state, not pure JSON in the air
initial Cells can reference a genome_template
same seed + same config creates the same per-Cell Genome outputs
different initial Cell ordinals receive deterministic bounded variation
Genome outputs only registered priorities
values are clamped to the canonical output range
Genome outputs priorities, not direct actions
ActionPlan order can differ between Cells because of Genome priorities
missing Material/capability still rejects or masks the process
Feasibility remains final authority
Genome/runtime trace is observer-only
Genome-disabled scenarios preserve prior deterministic behavior
```

### Minimal Config Shape

```toml
[genome_templates.balanced]
variation_amplitude = 0.08
runtime_interval_ticks = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.7
energy_conversion_priority = 0.7
repair_priority = 0.8
material_synthesis_priority = 0.3
division_preparation_priority = 0.1
```

Per-Cell variation:

```text
output_value =
  clamp(
    template_value
    + deterministic_noise(world_seed, initial_cell_ordinal, output_id) * variation_amplitude,
    canonical_min,
    canonical_max
  )
```

Noise streams must be independent per `output_id` so adding a new output does not shift all existing values.

### Explicit Non-Goals

```text
local input nodes
internal/recurrent regulatory nodes
epigenetic modifiers
runtime memory
copying
lifetime mutation
inheritance
lineage
recombination
horizontal transfer
population placement generator
unregistered process outputs
```

## Phase 3B: Local Inputs And Scheduled Runtime

### Goal

Replace the Phase 3A constant-output bootstrap with scheduled Genome Runtime evaluation over normalized local inputs.

### Build

```text
next_genome_tick per Cell
local input snapshot
input normalization
material/capability-gated sensing
runtime_state.last_decision_inputs
runtime_state.last_regulatory_outputs
scheduled runtime evaluation
stable output commit
runtime_interval_ticks enforcement
debug comparison against Phase 3A constant-output templates
priority outputs for registered processes, controlled reactions, repair and Joint actions only
```

### Gate

```text
Genome Runtime does not run every Tick unless configured
same seed/config reproduces outputs
Genome reads local normalized snapshot inputs only
missing sensing Material/capability masks the input
Genome outputs priorities, not actions
unregistered output is rejected
Feasibility remains final authority
Genome-disabled passive chemistry and existing Joints remain deterministic
```

## Phase 3C: Dynamic Regulatory Graph And Modulation

### Goal

Move beyond constant output priorities into dynamic regulatory graphs and bias modulation without creating new capabilities.

### Build

```text
non-constant direct regulatory graph nodes
input-to-output edges
threshold modulation
synthesis bias
reflex sensitivity modulation
cooldowns/cadence integration
controlled reaction priority modulation
repair priority modulation
Joint creation/upkeep priority modulation where registered
material/process bias trace
regulatory reachability checks
```

### Gate

```text
Cells with different Genome graph structure respond differently to the same local state
Cells with the same Genome graph respond differently to different local inputs
missing Material capability still rejects process
Genome can increase/decrease bias but not bypass costs
Genome can regulate registered Joint or controlled reaction outputs but cannot bypass locality, Material gates or accounting
observer metrics remain unreadable by Cells/Genome
```

## Phase 3D: Genome Copying, Mutation And Inheritance

### Goal

Make regulatory state heritable and mutable through explicit physical copying.

### Build

```text
genome copying process
copy progress
copy cost in Resources/Materials/Energy
carrier integrity checks
mutation during copying/repair
daughter genome assignment
lineage event log
mutation event log
```

### Gate

```text
division requires valid Genome copy or explicit nonfunctional daughter outcome
mutation is deterministic for same seed/config
Genome carrier damage is not automatically mutation
lineage replay reconstructs genome ancestry
```

## Phase 3E: Evolution-Ready Genome Experiments

### Goal

Run small deterministic populations where Genome variation changes survival/reproduction outcomes through existing material/process rules.

### Build

```text
small population genome variation scenarios
genome variant summaries
lineage/genome event export
regulatory reachability checks
mutation rate sweeps
selection/drift candidate logs as observer-only output
```

### Gate

```text
Genome affects outcomes through process priorities and material bias
mutations can be replayed
lineage/genome analytics do not affect behavior
at least one scenario shows different survival/division outcome from different genome settings
```

## Runner Unlock Criteria

Runner-1 headless work may start after Phase 3A when this exists:

```text
scenario config can declare genome_templates
initial Cells can reference genome_template ids
config parser rejects unknown Genome output ids
same seed/config replays the same initial Genome outputs and final tick summary
at least one demo scenario shows priority-driven process ordering without hardcoded Cell roles
```

Runner must treat Genome as Core scenario state. It should load config, start/pause/step runs and expose committed snapshots, but it must not special-case biological behavior.

## Open Questions Before Phase 3A

```text
Which exact material_id should represent the first genome carrier in starter scenarios?
Should Phase 3A accept only outputs already listed in action-process-registry.md, or first update the registry for growth/Joint creation names used by current Rust ProcessId?
What canonical numeric range should config validation expose to users: raw -1..+1 regulatory outputs, or normalized 0..1 priority weights mapped internally?
Should runtime_interval_ticks exist in Phase 3A config if outputs are constant, or be reserved for Phase 3B while accepted but inert?
Which trace fields are required for Runner demo debugging without making trace part of behavior?
```

---

# Superseded Phase Split

The previous Phase 3 split was horizontal:

```text
3A carrier/representation without behavior
3B scheduled runtime priorities
3C ActionPlan and material bias
3D copying/mutation/inheritance
3E experiments
```

That split delayed useful Runner scenarios because Cells still needed temporary non-Genome process choices. The accepted split starts with a narrow vertical bootstrap instead.

```text
3A Genome Bootstrap Vertical Slice
3B Local Inputs And Scheduled Runtime
3C Dynamic Regulatory Graph And Modulation
3D Copying, Mutation And Inheritance
3E Evolution-Ready Genome Experiments
```

## Rejected Phase 3A Scope

```text
pure JSON profile with no physical carrier
hardcoded Cell role/class behavior
Genome commands that execute processes directly
temporary Runner-only process priorities outside Core
unbounded priority values such as 1.1 in regulatory output config
global or observer-derived Genome inputs
```

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
