# PLAN: Phase 2 Global Roadmap

## Goal

Define Phase 2 as the transition from one-cell viability smoke to many-cell physical lifecycle simulation:

```text
many Cells
deterministic spatial physics
registered processes
feasibility checks
growth
division
death/decomposition
first meaningful visual debug scenario
```

This is a global phase roadmap, not a detailed implementation plan. Before each subphase starts, create a separate TDD plan:

```text
PLAN-phase-2A-...
PLAN-phase-2B-...
PLAN-phase-2C-...
PLAN-phase-2D-...
PLAN-phase-2E-...
```

## Phase 2 Principle

Phase 2 must not become a full evolution system yet.

It should answer:

```text
Can many Cells coexist in one world?
Can Cells physically push/collide deterministically?
Can Materials define simple reflexive capabilities before Genome exists?
Can Cells react through material/process rules instead of scripted intent?
Can a Cell grow by accounting rules?
Can a Cell divide into daughters?
Can Cells interact locally before Genome-driven evolution starts?
Can lifecycle/death/decomposition remain deterministic and accountable?
Can we observe the system visually without viewer authority?
```

Out of scope for Phase 2:

```text
Genome Runtime
mutation/inheritance beyond deterministic config-driven division placeholders
Joints
signals
organism-level behavior
selection/fitness logic
large-scale optimization
GPU compute
```

## Reflexive Material Model

Phase 2 must treat Cells as material machines, not decision-making agents.

Base rule:

```text
Material composition + local physical state + Resource/Environment context
  -> available reflexive processes
  -> Feasibility
  -> deterministic execution
```

Genome is not required for base reflexes. A Cell can react because its Materials have physical/process capabilities.

Examples:

```text
boundary/permeability material + local Resource -> passive/active uptake candidate
metabolic material + internal Resource -> energy conversion candidate
structural material + growth allocation -> radius/material growth candidate
contractile-like material + Energy + pressure/gradient input -> movement-like displacement candidate
repair-capable material + damage + Resources/Energy -> repair candidate
```

Rejected model:

```text
Genome says "go to food"
Genome says "attack"
Genome says "become neuron"
Cell reads observer labels or fitness
hardcoded role classes decide behavior
```

Correct Phase 3 relationship:

```text
Genome regulates thresholds, weights, synthesis bias, process priority bias and timing.
Genome does not invent commands outside registered material/process capabilities.
```

## Local Interactions Before Genome

Phase 2 must include minimal local cell-cell interaction primitives before Phase 3 Genome.

Reason:

```text
Genome can regulate only existing material/process capabilities.
If Cells cannot contact, exchange, signal or bind locally before Genome, Phase 3 can evolve only isolated cells.
Multicellular construction needs core interaction primitives before scientific/evolution experiments.
```

These primitives are not organism logic and not science analytics. They are world mechanics.

## Core Architecture Decision

Use a custom deterministic 2D cell physics layer inside `alife-core`.

Do not use Bevy/Godot/Rapier as the simulation authority for Phase 2.

Reason:

```text
deterministic replay matters more than feature-rich physics
Cells are circles/discs with simple collision constraints
growth/division needs domain-specific accounting
external physics engines can be hard to keep deterministic across platforms/builds
viewer must not own behavior
```

The first physical model:

```text
Cell = soft circle/disc
position
radius
mass-like value derived from radius/materials
velocity optional, initially zero or simple damped displacement
collision = overlap resolution by deterministic pair order
world boundary = solid_wall
```

Use position correction first, not full rigid-body dynamics.

## Phase 2A: Multi-Cell Store, Spatial Index And Physical Solver Baseline

### Goal

Make the core support many Cells and deterministic local physical interaction.

### Build

```text
CellStore multi-cell initialization
config support for initial cell list or cell count pattern
SpatialIndex real rebuild using grid/counts/offsets/sorted CellIndex
neighbor/contact pair generation
circle overlap detection
deterministic overlap resolution
solid_wall boundary handling
physics summary metrics
multi-cell deterministic replay tests
```

### Physical Solver Scope

Implement:

```text
cell-cell overlap correction
cell-wall correction
fixed iteration count per Tick
stable pair ordering
no random pair order
no HashMap iteration as behavior input
no external physics engine authority
```

Do not implement:

```text
growth
division
joints
signals
complex momentum
friction model
fluid pressure
```

### Acceptance Gates

```text
10-100 Cells initialize deterministically
overlapping Cells separate deterministically
same seed/config produces same positions
no Cell leaves solid_wall world
SpatialIndex queries local neighbors
no O(N^2) default path for normal interaction
snapshot/viewer projection can expose many Cell positions/radii
```

### Viewer Relevance

After 2A, a minimal viewer becomes useful for the first time:

```text
draw Cells as circles
draw resource grid as heatmap
draw world bounds
color lifecycle state
show tick/result metrics
```

But 2A viewer is optional. If implemented, it must be read-only.

## Phase 2B: Material Capabilities, Process Registry And Feasibility Check

### Goal

Replace Phase 1 direct hardcoded actions with material-capability-driven registered process candidates and explicit feasibility decisions.

### Build

```text
MaterialTypeId registry boundary
MaterialCapability flags/properties
material inventory capability lookup
ProcessId
ProcessRegistry
ProcessKind
ActionCandidate
ActionPlan placeholder from material reflex policy
FeasibilityInput
FeasibilityResult
rejection reasons
process execution buffers
mandatory upkeep as explicit mandatory consumer
registered resource uptake
registered metabolism
registered material synthesis baseline
registered repair placeholder if cheap
```

### Material Capabilities

Minimum Phase 2 material capabilities:

```text
boundary_permeability
resource_uptake
metabolism
structural_growth
storage_capacity
repair
contractile_displacement optional
```

These are not cell classes. They are properties of Materials inside the Cell.

Rules:

```text
No Material capability -> no related process candidate.
Capability does not bypass Feasibility.
Capability does not create Resources, Energy or Materials for free.
Capability can be represented as compact flags/properties in Phase 2.
```

### Initial Processes

Now:

```text
mandatory_upkeep
local_resource_uptake
metabolism_energy_conversion
material_synthesis
growth_resource_allocation
reflexive_displacement optional if contractile material is enabled
```

Future-compatible but disabled:

```text
division
repair
export
joint_create
signal_emit
hgt
```

### Feasibility Rules

Every process must answer:

```text
is process registered?
is process enabled?
does Cell have a Material capability for this process?
does Cell have required internal/external resources?
does Cell have Energy budget if cost exists?
does Cell have free capacity?
does local ResourceGrid contain available amount?
does Cell lifecycle allow execution?
what is rejected and why?
```

### Acceptance Gates

```text
Genome is still absent
ActionPlan may be material-reflex/config only
Feasibility accepts/rejects registered actions deterministically
rejected actions have explicit reasons
old Phase 1 resource loop can be represented through registered processes
Cells can react through Material capabilities without Genome
no behavior reads observer metrics
no per-action heap object spam in hot path
```

## Phase 2C: Reflexive Material Actions, Growth And Division Preparation

### Goal

Introduce actual Cell growth as physical/material state, add minimal material-driven reflex actions, and prepare deterministic division.

### Build

```text
material-driven reflex policy
local pressure/contact input
local Resource/Environment sampling input
optional contractile displacement process
movement-like displacement from material capability, not intent
growth_config
growth_progress
target_radius or target_mass-like accounting
resource/material allocation into growth
radius update from growth state
capacity update or capacity pressure from radius/materials
physics solver handles growing overlap pressure
division_ready flag as behavior state
division feasibility check
```

### Reflex Rules

Reflexive action must require:

```text
registered process
Material capability
local physical/context input
Feasibility pass
Energy/Resource/Material budget if configured
deterministic priority/order
```

Allowed Phase 2C reflex examples:

```text
uptake when Resource is local and boundary/metabolic capability exists
metabolism when internal Resource exists and metabolic capability exists
growth allocation when structural capability and budget exist
small displacement when contractile capability and pressure/gradient input exist
repair if repair capability and damage state exist
```

Not allowed:

```text
goal-directed food seeking
pathfinding
predator/prey behavior
typed command signals
Genome-directed action selection
observer metric inputs
```

### Growth Rules

Growth must require:

```text
internal Resources
Materials or material precursor budget
Energy if configured
free capacity or explicit capacity transformation rule
registered growth process
Feasibility pass
```

Growth must not:

```text
increase radius for free
create matter
ignore capacity
teleport Cell through collisions
use observer growth_readiness as input
```

### Division Preparation

In 2C, division may stop at:

```text
division_ready = true
division_feasible = true
```

or may include dry-run calculation:

```text
daughter radius estimate
partition resource estimate
partition energy estimate
placement candidate positions
```

Actual daughter creation can be deferred to 2D if 2C becomes too large.

### Acceptance Gates

```text
Cell radius can grow from resource/material accounting
growing Cell pushes neighbors through physical solver
material capabilities gate growth/metabolism/reflexive actions
optional displacement, if enabled, is material-capability-driven and deterministic
growth stops/rejects when resources or capacity are insufficient
division_ready is reached deterministically
no daughter Cells are created unless 2C explicitly includes a tested dry-run-only boundary
```

## Phase 2D: Division, Death And Decomposition

### Goal

Complete the individual Cell lifecycle loop: Cells can live, grow, divide, die and decompose under deterministic accounting.

### Build

```text
division execution
daughter Cell insertion
partition Energy Buffer
partition internal Resources
partition Materials
partition lifecycle/runtime flags
daughter placement with collision-safe fallback
death cleanup behavior
decomposition into ResourceGrid or MaterialFragment placeholder
population-level run summary
```

### Division Rules

Division must:

```text
be an explicit registered process
require Feasibility pass
copy/partition only Phase 2 available state
not duplicate Energy for free
not duplicate Resources/Materials for free
place daughters physically near parent
resolve collisions deterministically
emit deterministic birth/division events
```

Since Genome is Phase 3:

```text
genome copy is a placeholder/no-op or single inherited config reference
no mutation
no inheritance variation
no HGT
```

### Death And Decomposition

Death must:

```text
set lifecycle Dead
stop active process execution
emit CellDead event
leave or release Resources/Materials according to explicit decomposition config
not erase matter silently unless configured sink exists
```

Decomposition can be simple:

```text
dead Cell internal Resources return to local ResourceGrid over N ticks
Materials degrade into generic waste/resource amount
Cell removed only after decomposition_complete
```

### Lifecycle Scenario Suite

Required scenarios:

```text
multi_cell_collision_relaxation
single_cell_growth_to_division_ready
single_cell_division_creates_two_daughters
resource_limited_growth_stalls
overcrowded_division_rejected_or_delayed
death_releases_or_decomposes_resources
small_population_survives_for_N_ticks
small_population_collapses_when_resource_exhausted
```

### Acceptance Gates

```text
Cell can live, grow, divide and die
division conserves or explicitly partitions Energy/Resources/Materials
daughter placement is deterministic
death/decomposition accounting is explicit
small population scenario is deterministic
Phase 1 scenarios still pass
Phase 2 scenario report exists
```

## Phase 2E: Local Cell-Cell Interaction Primitives

### Goal

Add the minimal local interaction primitives needed before Genome can evolve multicellular construction.

### Build

```text
contact detection as derived state from physics/spatial index
contact pressure summary
contact pair cache with deterministic ordering
passive contact exchange candidate if Boundary/Material allows
minimal scalar contact stimulus placeholder
optional simple adhesion/binding primitive without full Joint behavior
local interaction events/summaries
OrganismView remains absent or debug-only placeholder
Phase 2 reachability suite
```

### Scope

Implement:

```text
contact pairs
pressure/contact metrics
material-gated passive contact exchange
material-gated scalar contact stimulus
deterministic pair order
no organism-level control
```

Keep deferred unless explicitly needed:

```text
full JointStore with long-lived constraints
resource channels through Joints
delayed signal traces
OrganismView connected components
multicellular reproduction
```

These deferred concepts remain candidates for Phase 3/4 depending on the final phase split, but Phase 2E must leave clean extension points.

### Acceptance Gates

```text
contact pairs are deterministic
contact pressure is observable
passive contact exchange requires material capability
scalar contact stimulus is local and non-command
no Cell reads observer-only organism labels
small population has local interaction metrics
Phase 1 and Phase 2A-D scenarios still pass
```

## Visualization Timing

First meaningful visualization should happen after 2A or during early 2B.

Recommended split:

```text
Phase 2A first:
  headless multi-cell physics and snapshots

Phase 2V / Debug Viewer Smoke after 2A:
  WebGL2 local viewer
  read-only snapshot stream
  circles for Cells
  resource grid heatmap
  collision/growth states by color
  tick and summary metrics
```

If 2A exposes good snapshots, viewer can be developed in parallel with 2B by a separate agent.

Viewer must not:

```text
mutate WorldState
drive physics
own timing decisions
feed UI state back into behavior
```

## Rust Reachability After Phase 2

After each Phase 2 subphase, run Rust-side reachability instead of treating Python as source of truth.

Minimum after 2A:

```text
spatial_index_reachable
collision_resolution_reachable
wall_collision_reachable
multi_cell_determinism_reachable
```

Minimum after 2B:

```text
material_capability_reachable
material_capability_missing_reject_reachable
registered_process_reachable
feasibility_accept_reachable
feasibility_reject_reachable
resource_uptake_process_reachable
metabolism_process_reachable
material_synthesis_process_reachable
```

Minimum after 2C:

```text
material_reflex_policy_reachable
contractile_displacement_reachable_if_enabled
growth_reachable
growth_stall_reachable
physical_push_from_growth_reachable
division_ready_reachable
```

Minimum after 2D:

```text
division_execution_reachable
daughter_partition_reachable
birth_event_reachable
death_event_reachable
decomposition_reachable
small_population_stable_reachable
small_population_collapse_reachable
```

Minimum after 2E:

```text
contact_pair_reachable
contact_pressure_reachable
passive_contact_exchange_reachable
contact_exchange_reject_without_capability_reachable
scalar_contact_stimulus_reachable
local_interaction_determinism_reachable
```

Python `early-stability` remains useful as an estimator/calibration assistant, not as the behavior authority.

## Performance Guardrails

Phase 2 must preserve paths to the target scale:

```text
20k Cells target
20k-40k Joints later
30+ rendered ticks/sec later
100+ headless ticks/sec target
```

Do now:

```text
dense CellIndex loops
flat arrays
preallocated contact pairs
stable pair ordering
SpatialIndex broad phase
no per-cell object graph
no Rc/RefCell/Arc/Mutex hot state
no global synchronized event/delta push in hot path
```

Measure in every subphase:

```text
ticks/sec for 1, 10, 100, 1_000 Cells
collision pairs per Tick
physics iterations per Tick
snapshot extraction cost
allocations per Tick if practical
```

Do later only after measurement:

```text
parallel domain decomposition
SIMD
compact numeric representation
sparse/chunked fields
viewer LOD
```

## Documentation Updates Needed

Create or update before detailed implementation:

```text
docs/implementation/phase-2-design.md
docs/implementation/phase-2-data-model.md
docs/implementation/phase-2-module-api.md
docs/implementation/implementation-phases.md
```

Optional after 2A:

```text
docs/implementation/phase-2-viewer-smoke.md
```

## Global Acceptance For Phase 2

Phase 2 is complete when:

```text
many Cells can coexist in one world
Cells collide/push deterministically
Cells act through material capabilities, registered processes and Feasibility
Cells have simple reflexive material behavior before Genome exists
Cells can grow from accounted resources/materials
Cells can divide with deterministic partitioning
Cells can die and decompose through explicit rules
Cells can interact locally through contact/material primitives
small population scenarios are replayable
first meaningful read-only viewer can show the result
Phase 1 scenarios still pass
Phase 2 reachability report exists
```

## Open Questions Before 2A TDD

These do not block the global plan, but should be answered before writing the detailed 2A implementation plan:

```text
Do we allow velocity in 2A, or only positional correction?
How many physics solver iterations per Tick should be the default?
Should radius derive from material/resource mass in 2C, or stay a configured/growth state value until Phase 3?
Which minimal Material capability set belongs in 2B, and which stays disabled until later?
Do we include contractile/reflexive displacement in 2C, or defer active movement until after growth/division?
Should Phase 2E include a minimal adhesion primitive, or only contact exchange/stimulus?
Do full Joints belong in late Phase 2E or early Phase 3/4 after Genome?
Do we create the first viewer immediately after 2A, or run 2B first and then viewer?
```

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
