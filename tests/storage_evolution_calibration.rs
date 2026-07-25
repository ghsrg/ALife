use alife::storage::analytics_export::{
    AnalyticsDataset, AnalyticsExportManifest, BalanceAnalyticsRow, PopulationAnalyticsRow,
};
use alife::storage::evolution_calibration::{compare_run_datasets, generate_calibration_report};

#[test]
fn test_evolution_calibration_comparison_and_report() {
    let mut baseline = AnalyticsDataset::new(AnalyticsExportManifest {
        schema_version: "1.0".to_string(),
        run_id: "run-baseline-01".to_string(),
        scenario_id: "long_run_stable".to_string(),
        config_hash: "hash123".to_string(),
        effective_seed: 42,
        tick_start: 0,
        tick_end: 1000,
        completeness: "full".to_string(),
        warning_codes: vec![],
        rows_count: 2,
    });
    baseline.population.push(PopulationAnalyticsRow {
        tick: 1000,
        alive_cells: 20,
        stressed_cells: 0,
        dead_cells: 2,
        total_cells_ever: 22,
        births_count: 5,
        deaths_count: 2,
    });
    baseline.balance.push(BalanceAnalyticsRow {
        tick: 1000,
        total_system_mass: 100.0,
        total_cell_resources: 50.0,
        total_environment_resources: 50.0,
        total_energy_capacity: 200.0,
        total_energy_current: 180.0,
        unaccounted_difference: 0.0001,
    });

    let mut candidate = AnalyticsDataset::new(AnalyticsExportManifest {
        schema_version: "1.0".to_string(),
        run_id: "run-candidate-02".to_string(),
        scenario_id: "long_run_stable".to_string(),
        config_hash: "hash456".to_string(),
        effective_seed: 42,
        tick_start: 0,
        tick_end: 1000,
        completeness: "full".to_string(),
        warning_codes: vec![],
        rows_count: 2,
    });
    candidate.population.push(PopulationAnalyticsRow {
        tick: 1000,
        alive_cells: 35,
        stressed_cells: 0,
        dead_cells: 1,
        total_cells_ever: 36,
        births_count: 15,
        deaths_count: 1,
    });
    candidate.balance.push(BalanceAnalyticsRow {
        tick: 1000,
        total_system_mass: 100.0,
        total_cell_resources: 60.0,
        total_environment_resources: 40.0,
        total_energy_capacity: 200.0,
        total_energy_current: 190.0,
        unaccounted_difference: 0.0001,
    });

    let comparison = compare_run_datasets(&baseline, &candidate);

    assert_eq!(comparison.baseline_run_id, "run-baseline-01");
    assert_eq!(comparison.candidate_run_id, "run-candidate-02");
    assert_eq!(comparison.pop_growth_delta, 15.0);
    assert_eq!(comparison.stability_classification, "improved_stability");

    let report = generate_calibration_report(&comparison);
    assert!(report.contains("# ALife Evolution Calibration & Run Comparison Report"));
    assert!(report.contains("run-baseline-01"));
    assert!(report.contains("run-candidate-02"));
    assert!(report.contains("improved_stability"));
}
