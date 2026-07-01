---
tags:
  - alife
  - implementation
  - tools
  - stability
---

# Early Stability Tool

> Handoff design for the early calculator and micro simulator under `tools/early-stability/`.

---

# Purpose

The Early Stability Tool is a separate implementation aid for checking whether candidate configs have a plausible viability range before the full simulation exists.

It is not Canon, not the source of truth for world laws and not part of the simulation hot path.

The tool must stay synchronized with [[docs/implementation/phase-1-design|Phase 1 Design]] for config field concepts, scenario ids, `survival_result` values and `collapse_reason` values.

---

# Location

Future implementation path:

```text
tools/early-stability/
```

Use plural `tools/` because this area may contain multiple helper modes, fixtures and scenario runners.

This documentation task does not create the directory or tool code.

---

# Implementation Stack

Recommended first implementation stack:

```text
Python
TOML input parsing
JSON output
Markdown report generation
deterministic local calculations
```

Reason:

```text
This tool is outside the simulation hot path.
It should be fast to modify while Phase 1 config fields are still stabilizing.
It can later be cross-checked against Rust alife-core outputs.
```

The tool must not implement Python behavior inside the future core Tick loop. It is an offline calibration/helper tool only.

---

# Tool Modes

## Evaluate Mode

Runs the current scenario/config and does not tune values.

Pipeline:

```text
input config
  -> static validation
  -> static calculator
  -> optional micro simulation
  -> machine-readable results
  -> human report
```

Evaluate mode answers:

```text
Is this config stable, fragile, collapse or invalid?
Why?
Which constraints were closest to failure?
Which metrics should be reviewed before tuning?
```

## Static Calculator

Runs without Tick simulation.

It checks direct budget and bound equations:

```text
mandatory_cost_per_tick <= energy_current + passive_energy_income
initial_used_capacity <= capacity_limit
heat_generated_per_tick <= heat_dissipation_rate or configured tolerance margin
waste_generated_per_tick <= waste_sink_rate or configured capacity margin
estimated_growth_budget <= resource_budget + energy_budget
estimated_division_loop_budget <= resource_inflow + free_capacity + energy_budget
estimated_joint_upkeep_budget <= multicellular_support_budget
```

Static Calculator is the first implementation target.

## Micro Headless Simulator

Runs small deterministic scenarios after Phase 1 design is stable.

It should not duplicate full `alife-core`. It is a boundary-check tool with a minimal local state model for viability experiments.

The micro simulator should use the same simplified Phase 1 accounting contract:

```text
energy_after_mandatory
mandatory_paid
used_capacity
free_capacity
heat_next
waste_next
```

## Tune Mode

Runs iterative candidate search over explicitly allowed parameters.

Pipeline:

```text
base config
  -> evaluate
  -> analyze failure or margins
  -> adjust allowed parameters
  -> evaluate candidate
  -> repeat until stable candidate found or budget exhausted
  -> write run history, recommended configs and empirical ranges
```

Tune mode must not change source-of-truth configs in place. It writes candidate configs and reports under `outputs/stability/<run_id>/`.

Tune mode should start with simple deterministic search:

```text
bounded grid search
coarse-to-fine range narrowing
stable sorted parameter order
fixed seed list
fixed max_iterations
```

Do not use nondeterministic optimizers for the first version.

Every candidate produced by Tune mode must be revalidated after applying candidate parameter values and before running the micro simulator.

Invalid mutated candidates must be recorded as:

```text
survival_result = invalid
collapse_reason = invalid_config
history = []
```

Invalid candidates must not enter the micro simulator. This keeps capacity, id, threshold and negative-value boundaries distinct from valid-but-fragile behavior.

---

# Inputs

The tool may accept:

```text
TOML scenario/config
normalized internal config snapshot
```

Minimum input fields should mirror Phase 1:

```text
scenario_id
seed
tick_count

world.size
world.boundary_mode

space.spatial_grid_size

resources.resource_type_ids
resources.initial_distribution
resources.optional_decay_rate
resources.passive_energy_income_placeholder

cell.initial_position
cell.radius
cell.initial_resources
cell.initial_materials
cell.initial_energy
cell.energy_capacity
cell.mandatory_cost_per_tick
cell.dormant_mandatory_cost_modifier
cell.capacity_limit
cell.minimum_viability_materials

environment.ambient_temperature
environment.heat_current
environment.heat_generated_per_tick
environment.heat_dissipation_rate
environment.heat_warning_threshold
environment.heat_death_threshold
environment.waste_current
environment.waste_generated_per_tick
environment.waste_sink_rate
environment.waste_warning_threshold
environment.waste_death_threshold

lifecycle.stress_energy_threshold
lifecycle.dormancy_allowed
lifecycle.critical_capacity_overrun

estimates.growth_cost_estimate
estimates.division_cost_estimate
estimates.resource_regeneration_or_inflow
estimates.population_space_limit
estimates.joint_count_estimate
estimates.joint_upkeep_cost
```

Fields under `estimates.*` are tool-only helpers. They must not be treated as Phase 1 engine state.

The tool must reject unknown Resource, Material, Field or process ids when those ids are provided by config.

Tune mode also needs a tuning request:

```text
tuning:
  max_iterations
  seeds
  objective
  allowed_parameters
  parameter_ranges
  candidate_profiles
```

Allowed `objective` values:

```text
find_first_stable
find_conservative_stable
map_stable_ranges
```

Candidate profiles:

```text
best_stable
conservative_stable
fragile_edge
```

---

# Outputs

Minimum per-run result:

```text
scenario_id
config_hash
seed
tick_count
survival_result
collapse_reason
metrics_summary
```

`survival_result` values:

```text
stable
fragile
collapse
invalid
```

Minimum `collapse_reason` values:

```text
none
invalid_config
energy_depleted
mandatory_cost_unpaid
capacity_exceeded
heat_limit_exceeded
waste_limit_exceeded
minimum_viability_materials_missing
population_unbounded
joint_upkeep_impossible
determinism_mismatch
```

`metrics_summary` should include only observer/tool diagnostics. It must not become behavior input for the engine.

Recommended artifact layout:

```text
outputs/stability/<run_id>/
  results.json
  REPORT.md
  ranges.json
  runs/
    run_0001.json
    run_0002.json
    run_0003.json
  recommended-configs/
    best_stable.toml
    conservative_stable.toml
    fragile_edge.toml
```

`results.json` contains the latest or aggregate result for the run.

`runs/*.json` contains every evaluated candidate, including failed and invalid candidates.

`REPORT.md` explains the run for a human reader.

`ranges.json` stores empirical tested and stable min/max ranges.

`recommended-configs/*.toml` are candidate configs only. They are not source-of-truth configs until reviewed and accepted.

Minimum `ranges.json` shape:

```text
parameter_id
tested_min
tested_max
stable_min
stable_max
recommended
confidence
notes
```

Minimum `REPORT.md` content:

```text
run_id
base_config
scenario list
mode: evaluate or tune
iteration count
stable / fragile / collapse / invalid summary
best candidate
recommended values
empirical min/max ranges
most sensitive parameters
failure reasons
warnings
limits of evidence
```

---

# Tunable Parameters

Tune mode may move only explicitly allowed parameters.

Recommended first tunable parameters:

```text
cell.initial_energy
cell.energy_capacity
cell.mandatory_cost_per_tick
cell.dormant_mandatory_cost_modifier
cell.capacity_limit
resources.initial_distribution
resources.passive_energy_income_placeholder
environment.heat_dissipation_rate
environment.heat_warning_threshold
environment.heat_death_threshold
environment.waste_sink_rate
environment.waste_warning_threshold
environment.waste_death_threshold
lifecycle.stress_energy_threshold
lifecycle.critical_capacity_overrun
estimates.growth_cost_estimate
estimates.division_cost_estimate
estimates.resource_regeneration_or_inflow
estimates.joint_upkeep_cost
```

Forbidden tuning targets:

```text
Canon rules
Energy Buffer semantics
direct Energy transfer rules
viewer authority
world law definitions
process semantics
collapse result vocabulary
unknown id acceptance
source-of-truth config files in docs/
```

If a parameter is not listed in `allowed_parameters`, the tool must not change it.

---

# Required Scenarios

Phase 1-aligned scenarios:

```text
single_cell_survival
single_cell_starvation
single_cell_over_capacity
single_cell_heat_stress
deterministic_replay_smoke
```

Tool-only estimate scenarios:

```text
single_cell_growth_budget
single_cell_division_loop_estimate
waste_heat_balance
population_growth_bound
joint_upkeep_budget
```

The tool answers:

```text
Can mandatory costs be paid?
Can Resources and Energy support growth?
Can a possible division loop remain bounded?
Can Heat and waste avoid unbounded accumulation?
Can a multicellular structure theoretically maintain Joints?
```

The tool does not answer whether life is intelligent, adaptive or evolutionarily interesting. It only rejects or warns about configs where baseline viability is impossible or obviously unstable.

---

# Expected Directory Shape

Future implementation should use a focused layout:

```text
tools/early-stability/
  README.md
  pyproject.toml or equivalent tool manifest
  scenarios/
    single_cell_survival.toml
    single_cell_starvation.toml
    single_cell_over_capacity.toml
    single_cell_heat_stress.toml
    single_cell_growth_budget.toml
    single_cell_division_loop_estimate.toml
    waste_heat_balance.toml
    population_growth_bound.toml
    joint_upkeep_budget.toml
  src/
    config_loader
    static_calculator
    micro_simulator
    tuner
    result_writer
    report_writer
  tests/
    static_calculator_tests
    tune_mode_tests
    scenario_validation_tests
    phase1_contract_sync_tests
```

Exact language and file extensions may be chosen during tool implementation. The interface and scenario responsibilities should remain.

---

# Expected CLI

Future CLI shape:

```text
early-stability evaluate --scenario scenarios/single_cell_survival.toml --out outputs/stability/<run_id>/
early-stability simulate --scenario scenarios/single_cell_survival.toml --ticks 1000 --out outputs/stability/<run_id>/
early-stability tune --scenario scenarios/single_cell_survival.toml --tuning tuning/single_cell.toml --out outputs/stability/<run_id>/
early-stability batch --scenarios scenarios/ --out outputs/stability/<run_id>/
```

Expected behavior:

```text
evaluate
  validation + static calculator + optional configured simulation

simulate
  bounded micro headless scenario

tune
  deterministic candidate search over allowed parameters

batch
  deterministic run over scenario files sorted by path
```

CLI must write all artifacts under the provided output directory and must not overwrite source configs unless a future explicit command is designed and accepted.

---

# Handoff Rules For Agent

The implementation agent for `tools/early-stability/` must:

- read `docs/PRINCIPLES.md`, `docs/GLOSSARY.md`, `docs/config/stability_bounds.md`, this document and `docs/implementation/phase-1-design.md`;
- keep the tool outside `alife-core`;
- not change Canon to make the tool easier;
- not treat viewer, SQLite, Parquet or analytics as source of truth;
- keep outputs deterministic for same input and seed;
- keep Phase 1-aligned scenario ids and result vocabulary synchronized with `phase-1-design.md`;
- keep tool-only estimates clearly separated under `estimates.*`;
- implement evaluate mode before tune mode;
- make tune mode deterministic and bounded by `max_iterations`;
- write all accumulated run artifacts to `outputs/stability/<run_id>/`;
- generate candidate configs as recommendations only;
- keep scenarios small and explicit;
- produce a report after implementation.

---

# Validation Rules

Static validation must mark config as `invalid` for:

```text
negative amount/rate/capacity
unknown ids
stored amount above capacity at initialization
mandatory costs with no possible Energy budget
unbounded Heat without dissipation/tolerance/abstracted rule
unbounded waste without sink/capacity/decay rule
division loop requiring impossible Resource/Energy budget
joint upkeep greater than available support budget
cell radius impossible for world size
```

Warnings may mark config as `fragile` if:

```text
survival margin is close to zero
growth leaves too little Energy reserve
population bound is extremely sensitive to Resource inflow
Heat or waste accumulates slowly but remains under threshold for the configured tick_count
Joint upkeep is possible only under narrow margins
```

---

# Sync Contract With Phase 1

The following must match Phase 1 exactly:

```text
survival_result values
Phase 1-aligned scenario ids
shared collapse_reason values
minimum output fields
accounting field concepts
observer-only status of metrics_summary
```

The following are tool-only and must not be copied into Phase 1 runtime state without a separate implementation decision:

```text
estimates.growth_cost_estimate
estimates.division_cost_estimate
estimates.resource_regeneration_or_inflow
estimates.population_space_limit
estimates.joint_count_estimate
estimates.joint_upkeep_cost
population_unbounded collapse estimate
joint_upkeep_impossible collapse estimate
```

---

# Completion Criteria For Tool Implementation

An implementation agent is done only when:

```text
evaluate mode can classify provided scenarios
tune mode can produce at least one stable candidate or explain failure
all candidate runs are recorded under outputs/stability/<run_id>/runs/
results.json is machine-readable
REPORT.md is human-readable
ranges.json contains empirical min/max tested and stable ranges
recommended-configs/ contains candidate TOML files when stable candidates exist
source configs are not modified in place
deterministic repeated run with same input produces same artifacts except run_id/timestamps
```

---

# Semantic Links

- supports: [[docs/implementation/phase-1-design|Phase 1 Design]]
- bounded by: [[docs/config/stability_bounds|Stability Bounds]]
- follows: [[docs/implementation/architecture|Architecture]]
- uses concepts from: [[docs/biology/cell|Cell]]
- uses concepts from: [[docs/world/energy|Energy Buffer]]
- prepares scenarios for: [[docs/implementation/implementation-phases|Implementation Phases]]

# Related Documents

- `docs/implementation/phase-1-design.md`
- `docs/config/stability_bounds.md`
- `docs/biology/cell.md`
- `docs/world/energy.md`
- `docs/world/physics.md`
