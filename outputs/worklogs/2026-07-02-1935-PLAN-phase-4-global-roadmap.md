# PLAN: Phase 4 Global Roadmap

## Goal

Define Phase 4 as the science/research layer: experiments, analytics, calibration, population/evolution analysis and reports over the core mechanisms from Phases 1-3.

Phase 4 should not introduce missing core mechanics as a surprise. If a mechanism changes behavior, it belongs to Phase 2 or Phase 3 first.

## Core Rule

```text
Science layer observes, runs experiments, compares outputs and produces reports.
It does not become Cell input or simulation authority.
```

## Phase 4A: Rust Reachability And Mechanism Coverage

### Goal

Systematically verify which implemented mechanisms are reachable in Rust scenarios.

### Build

```text
Rust reachability scenario suite
mechanism coverage report
stable/fragile/collapse classification
unreachable mechanism list
config adjustment recommendations
comparison with Python early-stability estimator
```

### Gate

```text
implemented mechanisms have at least one passing reachability scenario
unreachable mechanisms are listed with reason
Python is treated as estimator, Rust as behavior authority
```

## Phase 4B: Stability Sweeps And Config Bounds

### Goal

Find practical min/max ranges for stable worlds and expose warnings for future UI/config tooling.

### Build

```text
parameter sweep runner
stable range extraction
fragile boundary detection
collapse reason distributions
recommended baseline configs
artifact reports
```

### Gate

```text
stable bounds are derived from Rust runs
recommended configs include evidence
warnings map to real collapse/fragile mechanisms
```

## Phase 4C: Population, Lineage And Selection Analytics

### Goal

Analyze population dynamics after Phase 3 Genome and Phase 2 lifecycle are in place.

### Build

```text
population counters
birth/death/division windows
lineage event reconstruction
genome variant summaries
frequency shift logs
possible selection vs possible drift labels
```

### Gate

```text
analytics are observer-only
no fitness score affects behavior
lineage/genome summaries are reproducible from events
```

## Phase 4D: Specialization And Multicellular Structure Analysis

### Goal

Measure whether material/process/genome dynamics create persistent specialization or multicellular structure.

### Build

```text
SpecializationProfile observer metrics
material/process profile windows
contact/local interaction structure metrics
OrganismView as observer-only connected component if core interactions support it
stability_ticks and confidence metrics
```

### Gate

```text
temporary state is separated from stable specialization
OrganismView does not affect behavior
specialization labels are observer-only
```

## Phase 4E: Scientific Experiment Reports

### Goal

Answer research questions with reproducible evidence.

### Questions

```text
Can a single Cell remain viable over long runs?
Can a small population remain stable without exploding or collapsing?
Can Genome variation improve survival/reproduction beyond drift candidates?
Can material-driven reflexes produce useful behavior?
Can local interactions support stable multicellular structures?
What mechanisms block more complex behavior?
```

### Outputs

```text
experiment configs
run artifacts
markdown reports
charts/tables
open blockers
next mechanism recommendations
```

### Gate

```text
reports cite exact configs/seeds/engine version
claims are backed by reproducible runs
open blockers feed back into future implementation plans
```

## Open Questions Before Phase 4A

```text
What artifact format is enough for first Rust reachability reports?
Should Phase 4 use Python notebooks, Rust reports, or both?
When do we introduce Parquet exports?
Which research question is the first formal report target?
```

---

# Worklog Navigation

- index: [[outputs/worklogs/index|Worklogs Index]]
