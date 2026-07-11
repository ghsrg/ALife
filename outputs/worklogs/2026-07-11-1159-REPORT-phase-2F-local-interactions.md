---
tags:
  - alife
  - worklog/report
  - phase/2F
---

# Phase 2F Local Cell-Cell Interaction Report

## Status

Phase 2F is reachability-complete and analyzer-measurable.

## Implemented

- deterministic derived contact pairs;
- contact pressure summary;
- material-gated passive contact Resource exchange;
- delayed scalar contact stimulus;
- local interaction sweep_analyzer guardrail.

## Explicitly Not Implemented

- full JointStore;
- organism-level control;
- semantic command signals;
- direct Energy or Genome transfer;
- persistent adhesion/binding.

## Notes

- `ContactPair` is derived runtime/cache state, not a persistent Joint.
- Passive contact exchange moves only `ResourceAmount` and removes from source only the amount accepted by target capacity.
- Scalar contact stimulus is generated into a next buffer and becomes readable only on the following Tick.
- Analyzer `local_interaction_viability` uses two overlapping cells with explicit radius, resources, and local interaction enabled.

## Verification

- Baseline `cargo test --workspace --all-targets` passed before implementation.
- Targeted RED/GREEN tests passed for contact cache, pressure metrics, config parser, exchange, negative controls, stimulus, analyzer output, warnings, and reachability.
- Final `cargo fmt --check` passed.
- Final `cargo clippy --workspace --all-targets --all-features -- -D warnings` passed.
- Final `cargo test --workspace --all-targets` passed.
- `cargo run --bin sweep_analyzer -- config/analyzer/sweep_analyzer.toml` completed all sweep scenarios through `local_interaction_viability` and wrote `outputs/raw_data/local_interaction_viability.csv`; the command timed out during the pre-existing `resource_abundance` matrix after 15 minutes.

## Phase Gate

Phase 2F can proceed when `local_interaction_viability.csv` proves:

- contact pairs are detected;
- contact pressure is observable;
- exchange requires material capability;
- exchange rate changes measurable output;
- scalar contact stimulus is measurable and delayed;
- analyzer does not report `LOW_INFORMATION_SWEEP` for the valid scenario.
