# TDD Plan: AL-005-S03 Long-Run Evolution Scenario Suite

## Context
Researchers need deterministic, replayable long-run evolution scenarios that track population stability, classify run outcomes (`Collapse`, `Stable`, `Fragile`, `Invalid`), record lineage frequency sampling windows, and log neutral `observed_frequency_shift` records without adding hardcoded selection or fitness shortcuts to the Core simulation loop.

## Objectives
1. **Evolution Suite Model & Classification (`src/observer/evolution_suite.rs`)**:
   - Model `EvolutionRunOutcome` (`Collapse`, `Stable`, `Fragile`, `Invalid`).
   - Model `LongRunScenarioConfig` (scenario ID, seed matrix, max ticks, minimum population threshold, frequency window size).
   - Implement `LineageFrequencyWindow` & `observed_frequency_shift` neutral observer logs.
   - Implement `evaluate_evolution_suite(...)` for automated evaluation.
2. **Evolution Scenario Configs (`config/scenarios/evolution/`)**:
   - `long_run_stable.toml` (stable long-run evolution baseline).
   - `long_run_stress.toml` (high-stress environment testing fragility/collapse).
3. **Module & Integration Exposure**:
   - Expose `pub mod evolution_suite;` in `src/observer/mod.rs`.
   - Unit & contract tests in `tests/evolution_suite_contract.rs`.
4. **Automated Verification**:
   - `cargo test --test evolution_suite_contract`
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Verification Plan
- Run `cargo test --test evolution_suite_contract`
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
