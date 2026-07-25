# TDD Plan: AL-005-S04 Evolution Calibration And Comparison Tools

## Context
Researchers need offline comparison and calibration workflows over indexed runs and `AnalyticsDataset` exports. The tools must calculate baseline-vs-candidate metric deltas (population growth, mass balance drift, lineage diversity shift), derive stability classification labels (`"improved_stability"`, `"increased_fragility"`, `"equivalent_stability"`), generate rerun recommendations and provenance links without mutating simulation state or automatically rewriting accepted scenario configs.

## Objectives
1. **Evolution Calibration Module (`src/storage/evolution_calibration.rs`)**:
   - Model `RunCalibrationComparison`: baseline vs candidate manifest metadata, population growth delta, mass balance drift delta, stability classification label, rerun recommendation, provenance links.
   - Implement `compare_run_datasets(baseline: &AnalyticsDataset, candidate: &AnalyticsDataset) -> RunCalibrationComparison`.
   - Implement `generate_calibration_report(comparison: &RunCalibrationComparison) -> String` generating structured Markdown comparison reports.
2. **Storage Module Exposure (`src/storage/mod.rs`)**:
   - Expose `pub mod evolution_calibration;`.
3. **Automated Verification**:
   - Contract test suite in `tests/storage_evolution_calibration.rs`.
   - `cargo test --test storage_evolution_calibration`
   - `cargo fmt --check`
   - `cargo clippy --workspace --all-targets --all-features -- -D warnings`

## Verification Plan
- Run `cargo test --test storage_evolution_calibration`
- Run `cargo fmt --check`
- Run `cargo clippy --workspace --all-targets --all-features -- -D warnings`
