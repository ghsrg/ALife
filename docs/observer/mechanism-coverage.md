---
tags:
  - alife
  - observer
  - mechanism-coverage
  - analyzer
---

# Mechanism Coverage Contract

Mechanism Coverage is an Observer responsibility. It proves that registered mechanics are not merely present in code, but reachable, measurable and balanced enough to be trusted by phase planning.

It does not prove that a configuration is biologically interesting. It prevents silent gaps where a new mechanism exists but is never activated or measured.

Mechanism coverage is the lower-level gate for [[docs/observer/behavior-profile-balance|Behavior Profile Balance]]. A mechanism can be covered while the survival style that uses it is still unbalanced.

## Source Inputs

The analyzer should combine:

```text
Core or adapter mechanism registry
scenario definitions
scenario run outputs
raw metrics
config hashes
seed list
phase metadata
```

Current adapter source:

```text
tools/early-stability/mechanisms/*.toml
```

Future preferred source:

```text
Rust Core registry export
```

The adapter may normalize current TOML into the Observer mechanism record defined in [[docs/observer/observer-layer|Observer Layer]].

## Coverage Manifest

Required machine-readable fields:

```text
mechanism_id
category
introduced_in_phase
registered
enabled
activation_scenario
isolated_test
integration_test
raw_metrics
cost_metrics
benefit_metrics
balance_sweep
status
warning_codes
```

Allowed `status` values:

```text
covered
partially_covered
registered_but_disabled
not_activated
missing_scenario
missing_metrics
missing_balance_test
```

## Test Levels

Each executable mechanism should progress through four levels:

```text
Level 1: reachability
Level 2: directional effect
Level 3: trade-off
Level 4: integration
```

Full coverage requires all relevant levels.

For early phases, a mechanism can be accepted as `partially_covered` if the implementation phase only exposes reachability and the report explicitly names the missing higher levels.

## Relation To Balance Findings

Coverage answers:

```text
is this mechanism reachable, measurable and tested?
```

Balance findings answer:

```text
under equal requirements, are derived survival styles balanced against each other?
```

The analyzer should not hardcode organism types. It should derive behavior profiles from measured usage patterns and then compare those profiles.

If a report only writes mechanism coverage, it must mark itself as coverage-only and avoid claims such as `storage-heavy dominates fast-growth`.

## Warning Codes

Use stable codes in JSON, CSV and Markdown:

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

## Phase Increment Contract

When a phase adds Core mechanics, the analyzer must add:

```text
new mechanism registry entries
new activation scenarios
new raw metrics
new isolated sweeps
new integration scenarios
new balance expectations
new report explanations
```

The phase report must explain:

```text
what Core added
which new states became reachable
which costs appeared
which benefits appeared
which older scenarios need rerun
whether a dominant strategy appeared
```

## Recommended Reruns

If a mechanism changes, Observer should recommend reruns by dependency, not by running every matrix blindly.

Examples:

```text
Storage changed -> rerun scarcity, pulses, growth, dormancy
Division changed -> rerun abundance, competition, population growth, conservation
Repair changed -> rerun hazard, upkeep, growth, material synthesis
Contractility changed -> rerun spatial patches, collision, energy balance
```

The report path:

```text
outputs/reports/recommended-reruns-<timestamp>.md
```

## Candidate Configs

Analyzer may propose candidate configs when imbalance is detected.

It must write them separately:

```text
outputs/recommended_configs/
```

Each candidate needs metadata:

```text
source_report
detected_imbalance
changed_parameters
expected_effect
scenarios_to_rerun
confidence
```

Analyzer must not overwrite accepted baseline configs.

## Semantic Links

- base contract: [[docs/observer/observer-layer|Observer Layer]]
- source plan: [[outputs/worklogs/2026-07-04-1405-PLAN-sweep_scenario-eval-coverage_refactor|Sweep Scenario Eval Coverage Refactor]]
- current implementation docs: [[docs/implementation/mechanism-reachability|Mechanism Reachability]]
- stability tool: [[docs/implementation/early-stability-tool|Early Stability Tool]]
- process registry: [[docs/biology/action-process-registry|Action Process Registry]]
- behavior balance: [[docs/observer/behavior-profile-balance|Behavior Profile Balance]]
