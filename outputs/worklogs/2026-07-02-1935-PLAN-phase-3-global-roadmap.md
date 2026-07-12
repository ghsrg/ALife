# PLAN: Phase 3 Global Roadmap

## Goal

Define Phase 3 as the Genome phase: heritable regulatory control, mutation, inheritance and lineage over the material, chemistry and persistent-interaction body completed in Phase 2.

Phase 3 depends on Phase 2G and Phase 2H. It regulates registered reactions, material repair and Joint intents; it does not introduce their physical mechanisms.

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

## Phase 3A: Genome Carrier And Representation

### Goal

Introduce physical Genome state and the accepted direct regulatory graph representation without changing behavior yet.

### Build

```text
GenomeId
GenomeCarrierState
carrier integrity
carrier amount/capacity cost
regulatory graph representation
genome registry/storage
observer-only genome trace
config loading for initial genome
```

### Gate

```text
Genome occupies physical carrier/capacity
Genome is copied/stored as state, not pure JSON in the air
Genome trace is observer-only
Phase 2 behavior unchanged while runtime disabled
```

## Phase 3B: Scheduled Genome Runtime

### Goal

Run Genome Runtime at configured cadence to produce bounded priorities for existing registered processes.

### Build

```text
next_genome_tick per Cell
local input snapshot
runtime_state.last_decision_inputs
runtime_state.last_regulatory_outputs
priority outputs for registered processes only
priority outputs for registered controlled reactions, repair and Joint actions only
deterministic scheduled execution
stable output commit
```

### Gate

```text
Genome Runtime does not run every Tick unless configured
same seed/config reproduces outputs
Genome outputs priorities, not actions
unregistered output is rejected
Feasibility remains final authority
Genome-disabled passive chemistry and existing Joints remain deterministic
```

## Phase 3C: Genome-Guided ActionPlan And Material Bias

### Goal

Connect Genome outputs to ActionPlan construction and material/process bias without creating new capabilities.

### Build

```text
ActionPlan from priorities
process priority sorting
synthesis bias
threshold modulation
reflex sensitivity modulation
growth/division priority modulation
controlled reaction, repair and Joint creation/upkeep priority modulation
cooldowns/cadence integration
debug comparison of material reflex vs genome-modulated response
```

### Gate

```text
Cells with different Genome priorities choose different feasible process ordering
missing Material capability still rejects process
Genome can increase/decrease bias but not bypass costs
Genome can regulate a registered Joint or controlled reaction but cannot bypass locality, Material gates or accounting
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

## Open Questions Before Phase 3A

```text
How compact should the first regulatory graph encoding be?
Which Phase 2 process priorities are exposed first?
Which registered controlled reactions and Joint actions are safe to expose in the first Genome action vocabulary?
How many inputs can Genome Runtime read in the first implementation?
What is the first mutation operator for direct regulatory graph?
How much epigenetic state enters Phase 3 versus later science experiments?
```

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
