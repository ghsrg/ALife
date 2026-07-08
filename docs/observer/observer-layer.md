---
tags:
  - alife
  - observer
  - contract
  - architecture
---

# Observer Layer

Observer Layer is a read-only boundary between `alife-core` committed outputs and external consumers: UI, storage, reports, debug tools, sweep analyzers and research scripts.

It exists because several project concepts are intentionally not Core behavior:

- `OrganismView`
- lineage summaries
- species-like clusters
- selection interpretation
- specialization labels
- fitness-like metrics
- mechanism coverage and balance analysis
- UI projections

These are useful for humans and agents, but cells must not read them.

## Authority Contract

```text
Core committed state + registries + run artifacts
  -> Observer Layer
  -> projections / metrics / reports / recommendations
```

Observer has no authority over simulation behavior.

Allowed:

- read committed snapshots;
- read event logs;
- read config, seed, engine version and config hash;
- read Core registries exported for analysis;
- compute observer-only derived views;
- write reports, projection frames, coverage manifests and recommended reruns.

Forbidden:

- mutate `WorldState`;
- change committed tick state;
- generate hidden Cell inputs;
- influence Genome Runtime, Feasibility, Scheduler, lifecycle or selection;
- use `OrganismView`, cluster labels, selection metrics or coverage status as behavior input;
- treat UI state, storage state or analytics output as source of truth.

## Inputs

Observer may consume:

```text
CommittedSnapshot
EventLog
RunMetadata
RuntimeConfig or normalized config snapshot
config_hash
seed
engine_version
mechanism registries
scenario outputs
raw metrics
```

Mechanism registries include current and future categories:

```text
MaterialType registry
MaterialCapability registry
ProcessRegistry
Action/result types
Environment mechanism registry
Lifecycle mechanism registry
Interaction mechanism registry
Genome mechanism registry
Organism-view mechanism registry
```

If a registry does not exist in Core yet, the Observer documentation may define the expected export contract, but implementation must not fake Core authority.

## Outputs

Observer may produce:

```text
viewer projection frames
entity inspector projections
metrics summaries
OrganismView projections
lineage and population summaries
selection/drift interpretation reports
mechanism coverage manifests
balance analyzer reports
recommended rerun lists
candidate config recommendations
debug traces
```

All outputs are derived artifacts. They are not Canon state.

## Modes

### Live Observer

Live Observer supports UI and debug during a run.

It should be bounded and sampled:

- no unbounded full-state stream by default;
- viewer receives projection frames, not mutable Core state;
- heavy analytics are delayed, sampled or moved offline;
- dropped observer frames must not change simulation behavior.

### Offline Observer

Offline Observer supports sweep analysis, coverage checks and research reports.

It may process saved snapshots, event logs and raw metrics after the run. It may recommend config or mechanic changes, but it must not rewrite accepted configs automatically.

## Mechanism Coverage Requirement

Observer is the correct home for analyzer-facing coverage because coverage is not a Cell behavior. It observes whether Core mechanics are registered, activated, measured and balanced.

The coverage contract is derived from [[outputs/worklogs/2026-07-04-1405-PLAN-sweep_scenario-eval-coverage_refactor|Sweep Scenario Eval Coverage Refactor]].

Main invariant:

```text
No registered simulation mechanism without:
- activation scenario
- isolated mechanic test
- measurable benefit
- measurable cost
- relevant raw metrics
- integration coverage
- balance interpretation
```

If Core exposes a registered mechanism and analyzer cannot see coverage for it, the report must include:

```text
UNTESTED_REGISTERED_MECHANISM
```

This does not stop Core execution by itself. It blocks claiming the phase is fully covered by balance analysis.

## Minimum Mechanism Record

Each Observer-visible mechanism entry should provide:

```text
mechanism_id
category
introduced_in_phase
registered
enabled
required_capabilities
required_inputs
consumed_resources
consumed_energy
consumed_materials
produced_outputs
state_changes
activation_conditions
source_registry
```

For Phase 1 and current Phase 2 tooling, the existing `tools/early-stability/mechanisms/*.toml` registry may act as an adapter-level registry. Later Core should export the same logical fields from real Rust registries.

## Coverage Statuses

Observer coverage uses these statuses:

```text
covered
partially_covered
registered_but_disabled
not_activated
missing_scenario
missing_metrics
missing_balance_test
```

Warnings use stable machine-readable codes:

```text
UNTESTED_REGISTERED_MECHANISM
DIRECT_STATE_MUTATION_OUTSIDE_PROCESS_PIPELINE
SCENARIO_MECHANISM_NOT_ACTIVATED
PARAMETER_HAS_NO_EFFECT
METRIC_MISSING
SCENARIO_COVERAGE_MISSING
MECHANIC_TRADEOFF_MISSING
CONFIG_TUNING_RECOMMENDED
IMPLEMENTATION_SUSPECTED
```

## Required Coverage Artifacts

The analyzer should be able to produce:

```text
outputs/raw_data/mechanism_coverage.csv
outputs/reports/mechanism-coverage-<timestamp>.json
outputs/reports/mechanism-coverage-<timestamp>.md
outputs/reports/recommended-reruns-<timestamp>.md
outputs/recommended_configs/*.toml
```

For phase completion, it should also produce:

```text
phase_mechanism_delta.csv
phase_test_coverage_delta.csv
phase_balance_impact.md
```

## Phase Boundary

Current immediate need:

```text
Observer contract for mechanism coverage and analyzer reports.
```

Later needs:

```text
Live UI projection adapter.
OrganismView projection.
Population and selection analytics.
Species-like cluster projections.
Control Center read model.
```

This allows Phase 2 testing work to finish without blocking on full UI or full Observer service architecture.

## Acceptance Criteria

Observer documentation is sufficient when an implementation agent can answer:

- which Core registries must be visible to coverage analysis;
- which outputs are observer-only;
- which warnings indicate missing analyzer coverage;
- where coverage artifacts are written;
- why UI, analytics and recommendations cannot affect behavior;
- how to extend coverage when a new phase adds mechanics.

## Semantic Links

- governed by: [[docs/PRINCIPLES|Principles]]
- pre-flight: [[docs/mechanics/observer-projection|Observer Projection]]
- mechanism coverage: [[docs/observer/mechanism-coverage|Mechanism Coverage Contract]]
- projection shape: [[docs/observer/projection-contract|Projection Contract]]
- organism view: [[docs/biology/organism|Organism View]]
- selection analysis: [[docs/evolution/selection|Selection]]
- storage boundary: [[docs/engine/storage|Storage]]
- rendering boundary: [[docs/engine/rendering|Rendering]]
