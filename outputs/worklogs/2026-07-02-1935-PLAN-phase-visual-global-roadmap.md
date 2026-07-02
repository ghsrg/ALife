# PLAN: Phase Visual Global Roadmap

## Goal

Define the visualization/control roadmap as a separate read-only-to-control progression that never becomes simulation authority.

Phase Visual is not a core behavior phase. It can run in parallel with Phase 2/3 once committed snapshots and run summaries are stable.

## Core Rule

```text
alife-core is source of truth.
Viewer reads committed snapshots, events, summaries and approved control APIs.
Viewer must not mutate WorldState directly.
```

The visual stack remains:

```text
local Web Viewer
WebGL2 or Canvas/WebGL hybrid
Rust headless core / runner
snapshot or frame stream adapter
```

## Phase Visual A: Read-Only Debug Viewer

### Goal

Show the first meaningful simulation picture after Phase 2A.

### Build

```text
local web app
load committed snapshot/frame data
draw world bounds
draw Cells as circles
draw lifecycle colors
draw resource grid heatmap
show tick and run status
show Energy/Heat/Waste summary
pause/play/replay local frames only
```

### Gate

```text
viewer cannot change simulation state
headless result and viewed result are identical
10-100 Cells are readable visually
resource grid and Cells align spatially
```

## Phase Visual B: Scenario Runner UI

### Goal

Make it easy to run approved scenarios and inspect outputs without editing files manually.

### Build

```text
scenario list
run selected TOML config
show run summary
show collapse reason
show deterministic seed/config hash
open event log summary
compare two run summaries
```

### Gate

```text
UI invokes runner only through explicit scenario/run API
UI does not edit live WorldState
same scenario from CLI and UI gives same summary
```

## Phase Visual C: Layered Inspector

### Goal

Inspect simulation layers and entities in a way useful for debugging Phase 2/3 mechanisms.

### Build

```text
toggle layers: Cells, Resources, Heat/Waste, contacts, processes, events
selected Cell inspector
resource layer inspector
process/feasibility rejection panel
contact pair overlay
time scrubber for recorded frames
```

### Gate

```text
inspector reads projections only
no observer metric becomes behavior input
large frame payloads are bounded or sampled
```

## Phase Visual D: Experiment Dashboard

### Goal

Support running and comparing batches of approved configs and sweeps.

### Build

```text
batch run launcher
parameter sweep form
stable/fragile/collapse/invalid chart
min/max parameter range display
reachability checklist display
run artifact browser
```

### Gate

```text
batch dashboard calls runner/tools through explicit APIs
results are reproducible by CLI
dashboard never changes core rules
```

## Phase Visual E: Control Center

### Goal

Become the main local workbench for configuring, launching, viewing and analyzing simulation runs.

### Build

```text
config editor with validation
scenario templates
run queue
artifact browser
viewer playback
metrics dashboards
comparison reports
links to worklogs/docs
approved export actions
```

### Control Boundary

Allowed:

```text
create/edit configs before run
start/stop/pause runner
change viewer layer settings
launch sweeps/tools
export reports
```

Not allowed:

```text
mutate active WorldState directly
inject Cells into running deterministic simulation unless core has explicit approved command API
change physics/process rules through UI-only state
hide config changes from config_hash/replay metadata
```

## Cross-Phase Visual Requirements

```text
read-only by default
bounded payloads
viewport/LOD path for scale
no full-world JSON as long-term normal path
binary or compact frame path for larger worlds
viewer settings excluded from behavior hash
config changes included in config hash
```

## Open Questions

```text
Should Phase Visual A use Canvas 2D first or WebGL2 immediately?
Should viewer consume files first, localhost stream first, or both?
When do we add binary frames instead of JSON debug frames?
Which artifacts should be opened directly from the Control Center?
```

