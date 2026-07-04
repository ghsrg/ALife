---
tags:
  - alife
  - plan
  - visualization
  - control-center
---

# PLAN: Phase Visual Global Roadmap

## Goal

Define the visualization, inspection, experiment-control and evolution-observation roadmap as a separate progression that never becomes simulation authority.

Phase Visual is not a core behavior phase. It may develop in parallel with core phases once committed snapshots, events, run summaries, entity projections and approved runner/control APIs are stable.

The final visual environment must allow the user to:

```text
observe Cells, Organisms, Resources, Materials, Energy and Environment
inspect individual entities and aggregated populations
switch coloring and visualization modes
observe Genome, lineage, mutation and evaluation dynamics
classify and compare emergent behavior strategies
initialize spatial Resource distributions
run, pause, step, accelerate, restart and branch simulations
save Cells, Species and Organisms
load saved Cells, Species and Organisms at a selected world position
compare deterministic runs and controlled interventions
export screenshots, reports, configs and experiment artifacts
```

## Core Rule

```text
alife-core is the source of truth.
Viewer reads committed snapshots, events, summaries and projections.
Viewer invokes only explicit runner and approved command APIs.
Viewer must never mutate WorldState directly.
```

The visual stack remains:

```text
local Web Viewer
WebGL2 or Canvas/WebGL hybrid
Rust headless core / runner
snapshot or compact frame stream adapter
explicit scenario, control and placement command APIs
```

## Terminology

### Cell

A single simulated Cell with:

```text
position
radius
lifecycle state
Energy
internal Resources
Materials
Genome reference or Genome data when available
process state
damage
lineage metadata
```

### Species

A saved reusable definition consisting of:

```text
Genome
+
initial Cell material composition
+
initial Cell resource/energy configuration when required for reproducible spawning
```

A Species is not automatically an Organism and does not imply a hardcoded behavior class.

### Organism

A saved reusable multicellular structure consisting of:

```text
Cells
cell-relative positions
cell material compositions
Genome references/data
inter-cell topology or Joints when available
organism-level initialization metadata
```

### Behavior Profile

An observed or Genome-configured strategy summary, for example:

```text
producer-like
grazer-like
predator-like
scavenger-like
mixed
dormant
exploratory
defensive
```

Behavior profiles are projections or derived classifications. They must not become hidden simulation commands.

### Intervention

An approved, explicit command sent to the core and recorded in the run history.

For the current roadmap, supported manual intervention is intentionally narrow:

```text
load or spawn a saved Cell
load or spawn a saved Species
load or spawn a saved Organism
select the world position where it is inserted
```

Not required:

```text
dragging existing entities
moving existing Cells or Organisms manually
editing live Energy, Materials or Genome directly
painting live Resources during an active run
deleting arbitrary live entities through the viewer
```

## Global Visual Architecture

```text
alife-core
  -> committed simulation state
  -> snapshot/projection builder
  -> frame/event/summary transport
  -> local viewer

local viewer
  -> scenario/run requests
  -> approved control commands
  -> approved placement commands
  -> runner/core API
```

Required separation:

```text
Simulation state != Viewer state
Behavior metrics != Viewer display settings
Core config != UI-only preferences
Recorded intervention != hidden UI mutation
```

## Phase Visual A: Read-Only Debug Viewer

### Goal

Show the first meaningful simulation picture and prove that visual output matches headless simulation state.

### Build

```text
local web application
load committed snapshots or frame data
draw world bounds
draw Cells as circles
draw ResourceGrid heatmaps
draw Heat/Waste/Environment layers when available
draw lifecycle colors
show tick, run state and simulation time
show total Energy, Resources, Materials, Heat and Waste summaries
pause/play recorded frames
scrub recorded frames
viewport-scaled rendering instead of fixed 1:1 world-to-pixel mapping
fit world to current viewport
responsive window resize
full-screen view
zoom and pan
reset view
focus selected Cell
basic zoom-dependent Cell detail
select a Cell
export screenshot
```

### Initial Color Modes

```text
lifecycle state
Energy level
damage level
Cell age
selected Resource amount
selected Material amount
```

### Cell Inspector Baseline

```text
Cell id
position
radius
lifecycle
Energy
damage
internal Resource inventory
Material inventory
current processes
recent events
```

### Gate

```text
viewer cannot change simulation state
headless result and viewed result are identical
10-100 Cells are readable visually
small simulation worlds can scale to fill a large viewport
window resize and full-screen preserve spatial alignment
Cells scale correctly with zoom and are not permanently limited to one-pixel markers
ResourceGrid and Cells align spatially
zoom and viewport size change presentation only
selected Cell data matches committed snapshot
screenshot export does not affect simulation
```

## Phase Visual B: Scenario Runner UI

### Goal

Make it easy to launch, restart and inspect approved scenarios without editing files manually.

### Build

```text
scenario list
run selected TOML config
restart current scenario
restart with same seed
restart with a new seed
show deterministic seed and config hash
show run summary
show collapse reason
show event log summary
open run artifacts
compare two run summaries
```

### Time Controls

```text
start
pause
resume
stop
single Tick
speed x0.1
speed x1
speed x2
speed x10
speed x100 where core performance allows
run until selected Tick
run until selected event
run until selected Cell dies
run until population threshold
run until generation threshold
```

Rules:

```text
Speed changes affect scheduling, not simulation rules.
Single-step executes exactly one committed Tick.
Rewind initially means recorded-frame playback, not core rollback.
```

### Gate

```text
UI invokes runner only through explicit scenario/run API
same scenario from CLI and UI gives the same result
same seed/config produces the same deterministic run
restart behavior is explicit and reproducible
viewer timing does not change core Tick semantics
```

## Phase Visual C: Layered World Inspector

### Goal

Inspect simulation layers, Cells and local mechanisms in enough detail to debug balance, physics and process behavior.

### Layer Toggles

```text
Cells
Resources by type
total Resources
Energy
Materials by type
total Materials
Heat
Waste
toxicity or other Environment fields
contacts
contact pressure
SpatialIndex cells
processes
Feasibility rejections
events
movement vectors
Cell trajectories
damage
growth
division readiness
```

### Color Mode Selector

```text
lifecycle state
Energy level
selected internal Resource
total internal Resources
selected Material
total Materials
damage
age
generation
current process
recent dominant process
dormancy state
growth readiness
division readiness
Genome id
lineage
Genome similarity cluster
Organism membership
behavior profile
trophic role
evaluation or fitness result
reproductive success
```

Color modes are observer projections only.

### Multi-Scale Cell Detail

The viewer must use semantic zoom: increasing zoom reveals progressively richer Cell structure instead of only enlarging an unchanged dot or flat circle.

Detail selection should depend primarily on the Cell's current screen-space diameter.

```text
overview:
  Cell position
  Cell size
  selected color metric
  population-level aggregation where needed

intermediate:
  boundary or membrane
  lifecycle state
  dormancy
  damage
  current dominant process
  contact or movement indicators

close:
  relative Material composition
  relative internal Resource composition
  Energy state
  growth readiness
  division readiness

very close:
  detailed Material proportions
  detailed Resource proportions
  active process overlays
  contact directions
  movement or force vectors
  damage and repair visualization
  other internal projections supported by core data
```

Possible rendering forms:

```text
rings
wedges
segmented discs
layered boundaries
internal regions
small bars
icons
directional overlays
```

Starting screen-space thresholds may be configurable:

```text
less than 3 px:
  simplified point or aggregated marker

3-12 px:
  filled disc

12-32 px:
  boundary and primary overlays

32-96 px:
  Material, Resource and Energy composition

more than 96 px:
  detailed internal projection
```

These are starting defaults, not permanent hardcoded rules.

LOD transitions should be stable:

```text
avoid rapid flickering between detail levels
use hysteresis or smooth transitions where useful
preserve Cell identity and selected color mode during LOD changes
```

The viewer must not invent internal structures that are absent from core projections.

### Cell Inspector

```text
identity and lineage
position and physical state
Energy current/min/max
Energy production and spending breakdown
internal Resources by type
Resource intake history
Resource export and waste history
Materials by type
Material synthesis history
Material spending history
damage and repair history
current capabilities
current and recent processes
Feasibility accept/reject history
movement and contact history
Genome reference/data
parent and offspring references
birth Tick
age
generation
death cause when dead
```

### Resource Inspector

```text
Resource type
local concentration
world total
distribution heatmap
regeneration rate
consumption rate
depletion history
top consuming Cells/Organisms
top producing or releasing sources
```

### Material Inspector

```text
Material type
world total inside living Cells
world total inside dead/decomposing Cells
amount synthesized
amount consumed
amount released
distribution by Cell
distribution by Organism
distribution by Species/lineage
```

### Process and Feasibility Inspector

```text
process execution counts
process success rate
rejection count by RejectionReason
Energy cost
Resource cost
Material cost
affected entities
recent execution timeline
```

### Contact and Physics Inspector

```text
contact pair overlay
contact pressure
overlap correction
active displacement
passive displacement
wall contact
collision events
```

### Search and Tracking

```text
search by Cell id
search by Genome id
search by lineage
search by Organism id
search by Species name
filter by lifecycle
filter by behavior profile
filter by Resource/Material thresholds
pin selected Cell
follow selected Cell
show movement trajectory
compare two selected Cells
```

### Gate

```text
inspector reads projections only
no observer metric becomes behavior input
large frame payloads are bounded or sampled
per-entity history has bounded retention
display filters do not affect behavior
```

## Phase Visual D: World Initialization Editor

### Goal

Create reproducible starting worlds with explicit spatial distributions of Resources, Environment fields and initial entities.

### Resource Initialization Modes

For every Resource type:

```text
uniform field
single high-concentration zone
radial gradient
linear gradient
high -> medium -> sparse zones
multiple patches
random sparse points
clustered random patches
noise field
resource veins
empty exclusion zones
periodic regeneration zones
finite source zones
```

### Resource Distribution Parameters

```text
Resource type
initial amount
center or region
radius
shape
peak concentration
minimum concentration
falloff function
random seed
patch count
patch size range
regeneration rate
maximum local capacity
depletion behavior
```

### Environment Initialization

```text
Heat
Waste
toxicity
temperature if modeled
obstacles
solid walls
environment gradients
hazard zones
```

### Initial Entity Placement

```text
built-in Cell template
saved Cell
saved Species
saved Organism
count
position pattern
orientation where applicable
spacing
placement region
placement seed
collision-safe placement option
```

### Preview

```text
Resource heatmap preview
Environment layer preview
initial Cell/Organism positions
estimated totals
validation errors
config diff
config hash preview
```

### Output

```text
validated scenario config
initialization artifact
reproducible seed
config hash
optional human-readable summary
```

### Gate

```text
editor creates config before run
editor does not mutate an active WorldState
preview matches generated initialization
CLI can run the generated config
same config/seed produces the same initial world
```

## Phase Visual E: Experiment Dashboard

### Goal

Support batch experiments, parameter sweeps, stability analysis and comparison of simulation outcomes.

### Build

```text
batch run launcher
parameter sweep form
matrix sweep form
stable/fragile/collapse/invalid classification
min/max parameter ranges
reachability checklist
run artifact browser
comparison reports
long-run metrics
balance/invariant panel
warnings panel
```

### Core Time-Series Charts

```text
population
living/dead/dormant counts
total Energy
Energy production and spending
Resources by type
Materials by type
Heat and Waste
birth rate
death rate
division rate
average age
average lifespan
generation
Genome diversity
lineage count
Species/cluster count
behavior profile distribution
trophic role distribution
evaluation/fitness distribution
```

### Balance and Conservation

```text
Resource input
Resource stored
Resource metabolized
Resource converted
Resource released
Resource lost to explicit sinks

Energy produced
Energy stored
Energy spent on upkeep
Energy spent on movement
Energy spent on growth
Energy spent on repair
Energy spent on division
Energy lost to explicit sinks

Material synthesized
Material stored
Material consumed
Material released
Material decomposed
Material lost to explicit sinks
```

### Warnings

```text
unbounded Energy accumulation
unbounded Resource accumulation
unbounded Material accumulation
starvation cascade
population explosion
population collapse
excessive dormancy
oscillating lifecycle state
numerical drift
conservation mismatch
order-dependent resource allocation
invalid Genome or organism structure
```

### Comparison Modes

```text
run A vs run B
seed A vs seed B
config A vs config B
control vs intervention
Species A vs Species B
Organism A vs Organism B
lineage A vs lineage B
```

### Gate

```text
batch dashboard calls explicit runner/tool APIs
results are reproducible by CLI
charts are derived from committed summaries
dashboard never changes core rules
```

## Phase Visual F: Genome And Evolution Observatory

### Goal

Observe Genome behavior, inheritance, mutation, lineage dynamics, evaluations and emergent ecological strategies.

### Genome Inspector

```text
Genome id
Genome content/version
parent Genome
lineage
generation
mutation history
mutation count
active regulation parameters
enabled capabilities/process biases
initial material composition reference
birth and extinction Ticks
population count
descendant count
evaluation history
```

### Lineage Views

```text
lineage tree
parent-child graph
mutation event markers
population over time
birth/death timeline
extinction markers
dominant lineage timeline
selected lineage spatial distribution
```

### Genome Similarity And Species Views

```text
Genome similarity clusters
cluster population
cluster age
cluster persistence
cluster geographic distribution
cluster material composition
cluster Energy strategy
cluster Resource strategy
cluster evaluation statistics
```

A Species saved by the user is:

```text
Genome
+
initial Cell material composition
+
optional initial Resource/Energy state required for reproducibility
```

Automatic Genome clusters may be presented as:

```text
Species candidate
Genome cluster
lineage group
```

They must not be silently treated as authoritative biological species unless explicit rules are defined.

### Evaluation Views

```text
evaluation result by Cell
evaluation result by Organism
evaluation result by Genome
evaluation history
evaluation components
selection event
reproduction eligibility
failure/rejection reason
```

### Behavior Profile Classification

Behavior must be viewable at:

```text
Cell level
Organism level
Genome level
lineage level
population level
```

Potential derived profiles:

```text
producer-like
grazer-like
predator-like
scavenger-like
mixed
resource-conserving
high-growth
high-reproduction
dormancy-oriented
movement-oriented
repair-oriented
```

Classification inputs may include:

```text
actual Resource source breakdown
actual process execution distribution
Energy acquisition path
Material synthesis path
interaction history
Genome regulation parameters
evaluation outputs
```

Required display:

```text
current behavior profile
confidence or score
profile history
reason/components
percentage of population by profile
spatial map by profile
```

A label such as predator, herbivore or plant must not be a hidden hardcoded role unless the core explicitly models it. Prefer observed trophic profiles.

### Gate

```text
Genome and evaluation displays are projections
classification method is explicit
derived labels do not feed back into behavior
lineage and mutation history are reproducible
saved Species contains Genome plus initial Cell material composition
```

## Phase Visual G: Organism Inspector And Observatory

### Goal

Inspect multicellular structures as entities without losing visibility into their Cells.

### Organism Visualization

```text
organism outline
organism bounding area
organism center
organism orientation
Cells colored by Cell state
whole Organism colored by selected Organism metric
inter-cell Joints
signal paths when available
resource/material flow paths when available
```

### Entity Level Selector

```text
Cell
Organism
Genome
lineage
Species/cluster
population
```

### Organism Inspector

```text
Organism id
Cell count
Cell type/material distribution
total Energy
Energy by Cell
Resources by type
Materials by type
Genome composition
lineage
age
generation
offspring count
growth state
reproduction state
damage state
current behavior profile
evaluation result
birth history
death/collapse reason
```

### Internal Structure Views

```text
Cell graph
Joint graph
signal graph
Resource exchange graph
Energy distribution
Material distribution
damage distribution
process distribution
```

### Organism Comparison

```text
structure
Cell count
Genome composition
Materials
Resources
Energy strategy
movement
behavior profile
evaluation
reproduction
survival
```

### Gate

```text
OrganismView is derived from core state
viewer does not invent organism membership
Cell-level detail remains available
organism aggregation is deterministic
```

## Phase Visual H: Library, Save And Placement Center

### Goal

Save reusable Cells, Species and Organisms and place them into a new or active run through an explicit recorded command.

### Saved Asset Types

#### Saved Cell

```text
Cell state snapshot or approved reusable Cell template
Genome
Materials
Resources
Energy
physical parameters
required version metadata
```

#### Saved Species

```text
Genome
initial Cell material composition
optional initial Resource/Energy state
spawn defaults
compatibility/version metadata
```

#### Saved Organism

```text
Cells
relative positions
Materials per Cell
Genome data/references
Joints/topology when available
Organism initialization metadata
compatibility/version metadata
```

### Library Functions

```text
list
search
tag
rename
duplicate
inspect
compare
export
import
validate
archive
delete from library
```

### Placement Mode

The user may:

```text
select a saved Cell, Species or Organism
choose a position in the world by mouse click
preview placement footprint
see collision or boundary validation
confirm insertion
```

The user does not need to:

```text
drag existing entities
move live entities
rotate or deform live entities after insertion
directly edit live entity internals
```

Optional before confirmation:

```text
choose orientation for an Organism
choose count
choose deterministic placement pattern
choose collision-safe fallback
```

### Placement Command Requirements

Every insertion must:

```text
go through an explicit core command API
target a specific Tick or next safe Tick
record asset id and asset hash
record requested position
record resolved position
record orientation when used
record validation result
record command in intervention log
be replayable
affect run/intervention metadata
```

### Placement Validation

```text
world boundary
collision constraints
required Resource/Environment conditions
asset version compatibility
Genome compatibility
Material/resource validity
Organism topology validity
maximum population constraints
```

### Run Classification

A run with manual placement must be marked as:

```text
intervened run
```

It remains deterministic when replayed with:

```text
same initial config
same seed
same ordered intervention log
same saved asset hashes
```

### Gate

```text
saved asset can be validated before use
placement is explicit and replayable
viewer never edits WorldState directly
requested and resolved positions are recorded
same intervention log reproduces the same result
```

## Phase Visual I: Control Center

### Goal

Become the main local workbench for configuring, launching, viewing, controlling and analyzing simulation runs.

### Build

```text
config editor with validation
scenario templates
World Initialization Editor
run queue
active run controls
viewer
layer and color controls
Cell/Organism/Genome inspectors
Evolution Observatory
experiment dashboards
saved asset library
placement mode
artifact browser
comparison reports
links to worklogs/docs
approved export actions
```

### Allowed Control Actions

```text
create and edit configs before run
start run
pause run
resume run
stop run
single-step
change execution speed
restart run
restart from same config
restart from checkpoint
branch from checkpoint
launch sweeps
export artifacts
save Cell
save Species
save Organism
load/place saved Cell
load/place saved Species
load/place saved Organism
```

### Not Required

```text
drag existing live entities
move existing live entities manually
paint Resources into an active world
edit live Genome directly
edit live Materials directly
edit live Energy directly
change physics/process rules through UI-only state
hide config or intervention changes from metadata
video recording
```

### Screenshot And Export

Required:

```text
export current viewport screenshot
export selected layer screenshot
export high-resolution world screenshot when feasible
export charts as images
export run summary
export comparison report
export scenario config
export intervention log
export saved asset
```

Not required:

```text
video recording
```

## Phase Visual J: Checkpoints And Branching Experiments

### Goal

Allow controlled experiments from a shared historical state.

### Build

```text
save world checkpoint
load checkpoint
branch run from checkpoint
name branch
attach intervention plan
compare branches
control vs placement branch
checkpoint timeline markers
checkpoint artifact browser
```

### Branch Examples

```text
same checkpoint + no intervention
same checkpoint + saved Species placement
same checkpoint + saved Organism placement
same checkpoint + alternative config allowed by branch rules
```

### Gate

```text
checkpoint contains sufficient core state
branch metadata records parent run/checkpoint
intervention logs are separate and reproducible
viewer playback is not confused with core rollback
```

## Cross-Phase Visual Requirements

### Read-Only By Default

```text
all inspection is read-only
controls use explicit APIs
placement requires confirmation
no hidden mutable UI state affects behavior
```

### Determinism And Replay

```text
config changes included in config hash
saved asset hashes recorded
intervention commands ordered and timestamped by Tick
viewer settings excluded from behavior hash
same run inputs reproduce the same result
```

### Payload And Performance

```text
bounded snapshots
bounded histories
viewport filtering
LOD for large populations
aggregation for distant entities
no full-world JSON as long-term normal path
compact or binary frame transport for larger worlds
separate low-frequency metrics stream
separate event stream
```

### Viewport Scaling And Multi-Scale Rendering

The visual representation of the world must not use a fixed 1:1 mapping between simulation units and screen pixels.

A simulation world such as `100x100` may be rendered across a viewport such as `1920x1080`, while preserving spatial relationships and deterministic visual positioning.

Core principles:

```text
world coordinates remain simulation-space coordinates
viewport rendering uses an independent world-to-screen transform
window dimensions do not define simulation dimensions
window resize changes presentation only
zoom changes visible detail only
the same world remains meaningful across multiple scales
```

Required viewport behavior:

```text
fit world to viewport
responsive resize
full-screen view
zoom in and out
pan
reset view
focus selected Cell or Organism
preserve aspect ratio or use explicit letterboxing rules
maintain stable world-to-screen mapping
```

Example:

```text
simulation world:
  100x100

viewer:
  1920x1080

The world is scaled to the available viewport.
A Cell is not permanently limited to one screen pixel.
Zooming in increases its screen-space size and reveals additional detail.
```

Semantic zoom must expose additional information by scale:

```text
far zoom:
  simplified Cells or aggregated population markers
  high-level color metric

medium zoom:
  Cell radius
  membrane or boundary
  lifecycle, damage, dormancy and process overlays

close zoom:
  Material proportions
  Resource proportions
  Energy state

very close zoom:
  detailed Materials and Resources
  active processes
  local contacts
  movement or force vectors
  damage and repair
```

For Organisms:

```text
far zoom:
  Organism marker or outline

medium zoom:
  Organism boundary and member Cell distribution

close zoom:
  individual Cells
  Joints or topology
  Energy, Resource or signal flows

very close zoom:
  per-Cell Materials, Resources and processes
```

Performance rules:

```text
distant entities use simplified or aggregated rendering
nearby visible entities use individual rendering
fine internal detail is requested and rendered only when screen-space size justifies it
labels appear only when readable or explicitly requested
world-to-screen transforms must not rebuild or mutate simulation state
```

Observer boundary:

```text
zoom level must not affect behavior
window size must not affect Tick execution
screen visibility must not affect simulation priority
hidden entities continue to simulate normally
viewer detail selection never becomes Genome or process input
```

Acceptance expectations:

```text
a small simulation world can fill a large viewport
window resizing preserves correct world alignment
full-screen mode preserves spatial proportions
Cells are not permanently represented as one-pixel objects
zooming reveals additional meaningful detail
zooming out restores simplified population rendering
Material and Resource proportions match snapshot data
render scale does not affect deterministic core results
```

### Scale Targets

```text
10-100 Cells: full detail
1,000 Cells: selective labels and bounded histories
20,000 Cells: LOD, aggregation, viewport filtering and compact transport
large lineage trees: virtualized rendering and clustering
```

### Observer Boundary

Viewer and analytics must never become behavior authority.

Forbidden feedback paths:

```text
viewer color -> behavior
derived predator label -> process selection
selected Cell -> simulation priority
screen visibility -> Tick execution
observer-only fitness chart -> Genome input
```

### Versioning

```text
snapshot schema version
event schema version
saved Cell schema version
saved Species schema version
saved Organism schema version
checkpoint schema version
intervention log schema version
migration or explicit incompatibility reporting
```

## Required Core Projection APIs

The viewer will require stable read models.

### World Frame Projection

The normal world frame should remain lightweight.

```text
Tick
world bounds
Cell ids
Cell positions/radii
lifecycle
selected color metrics
Resource layers
Environment layers
Organism ids when available
LOD or aggregation metadata when required
```

### Cell Detail Projection

Rich Cell detail should be requested only for selected Cells or Cells whose screen-space size justifies it.

```text
identity
physical state
boundary or membrane projection
Energy and Energy ratio
Resources and Resource proportions by type
Materials and Material proportions by type
damage ratio
capabilities
processes
dominant active process
movement or force vector
contact directions
growth readiness
division readiness
Genome
lineage
evaluation
history summary
available visual-detail flags
```

### Organism Detail Projection

```text
identity
member Cells
structure
Energy
Resources
Materials
Genome composition
behavior
evaluation
history summary
```

### Population Summary Projection

```text
counts
totals
rates
Genome diversity
lineages
behavior profiles
evaluation distributions
balance/invariant values
```

### Event Projection

```text
birth
division
death
decomposition
mutation
Genome inheritance
Organism creation/collapse
evaluation
selection
process rejection
resource depletion
manual placement
checkpoint
```

## Required Approved Command APIs

```text
start scenario
pause
resume
stop
single-step
set execution speed
restart
create checkpoint
branch from checkpoint
place saved Cell
place saved Species
place saved Organism
```

No general-purpose direct state mutation API should be exposed to the viewer.

## Suggested Artifact Structure

```text
outputs/runs/<run-id>/
  config.toml
  config-hash.txt
  seed.txt
  summary.json
  metrics/
  events/
  frames/
  screenshots/
  checkpoints/
  interventions.jsonl
  reports/

outputs/library/cells/
outputs/library/species/
outputs/library/organisms/
```

## Suggested Visual Layout

### Main Workspace

```text
top bar:
  scenario
  run state
  Tick
  speed
  start/pause/step/restart

left panel:
  layers
  color mode
  filters
  entity level

center:
  world viewport

right panel:
  selected Cell/Organism/Genome inspector

bottom panel:
  timeline
  events
  charts
  warnings
```

### Specialized Workspaces

```text
World Editor
Experiment Dashboard
Evolution Observatory
Organism Inspector
Saved Asset Library
Run Comparison
Checkpoint Branching
```

## Global Acceptance

Phase Visual is complete when:

```text
the user can observe Cells and Organisms spatially
the simulation world can scale to the available viewport and full-screen mode
Cells remain meaningful across multiple zoom levels
zooming in reveals membrane, Materials, Resources, Energy and process detail
zooming out provides efficient population-level rendering
the user can color entities by Energy, Resources, Materials, Genome, lineage, behavior and evaluation
the user can inspect exact Resource and Material composition
the user can see Resource distribution by type across the world
the user can see Energy, Resource and Material flows over time
the user can inspect Genome, mutations, lineages and evaluations
the user can inspect emergent behavior profiles such as producer-like, predator-like or grazer-like
the user can initialize reproducible Resource concentration zones and gradients
the user can run, pause, step, accelerate and restart simulations
the user can save Cells
the user can save Species as Genome plus initial Cell material composition
the user can save whole Organisms with Cell structure
the user can select a world position and place a saved Cell, Species or Organism
all placements go through a recorded and replayable core command
the user can save checkpoints and branch experiments
the user can compare control and intervention runs
the user can export screenshots, charts and reports
video recording is not required
viewer remains non-authoritative
headless and viewed runs remain reproducible
```

## Open Questions Before Detailed TDD Plans

```text
Should Phase Visual A use Canvas 2D first or WebGL2 immediately?
Should the viewer consume files first, localhost streaming first, or both?
When should binary frames replace JSON debug frames?
How much per-Cell history should snapshots retain?
Should behavior profiles be rule-based first, statistical, or both?
How should Genome similarity be calculated and visualized?
Should a saved Cell preserve its current dynamic state or only a reusable initialization state?
Which initial Resource/Energy fields belong inside a saved Species?
Can a saved Species include configurable spawn defaults without changing its identity?
How should Organism orientation be chosen during placement?
Should placement fail on collision or use a deterministic nearest-valid-position fallback?
At which Tick are user placement commands applied?
Which checkpoints are full state and which are lightweight references?
Which metrics must be streamed live and which may be computed after the run?
Which screenshot formats and maximum resolutions are required?
Which default screen-space thresholds should select each Cell LOD level?
Should semantic-zoom transitions use hard thresholds, hysteresis, smooth interpolation, or a hybrid?
Should detailed Cell projections be pushed automatically for visible Cells or requested on demand?
Should viewport fitting preserve the full world with letterboxing or allow optional cropping?
```

## Recommended Implementation Order

```text
Visual A:
  read-only Cells + Resource heatmap + basic inspector

Visual B:
  scenario runner + time controls + restart

Visual C:
  layers + color modes + detailed Cell inspector

Visual D:
  Resource/world initialization editor

Visual E:
  experiment dashboard + balance charts

Visual F:
  Genome/evolution observatory

Visual G:
  Organism inspector

Visual H:
  save library + placement commands

Visual I:
  integrated Control Center

Visual J:
  checkpoints + branching experiments
```

## Rendering Architecture Principle

```text
simulation-space defines where entities exist
screen-space defines how they are displayed
semantic zoom defines how much detail is visible
LOD defines how much rendering work is justified
alife-core remains the only simulation authority
```

## Main Risk

The main architectural risk is allowing visualization convenience to leak into simulation authority.

The correct rule remains:

```text
Viewer requests.
Core validates.
Core applies.
Core records.
Viewer observes.
```
