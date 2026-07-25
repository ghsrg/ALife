use alife::storage::analytics_export::{
    AnalyticsDataset, AnalyticsExportManifest, AnalyticsExporter, BalanceAnalyticsRow,
    EnvironmentAnalyticsRow, LineageAnalyticsRow, PopulationAnalyticsRow,
};
use std::fs;
use std::path::PathBuf;

#[test]
fn test_analytics_dataset_manifest_and_serialization() {
    let manifest = AnalyticsExportManifest {
        schema_version: "1.0".to_string(),
        run_id: "test_run_101".to_string(),
        scenario_id: "living_ecosystem".to_string(),
        config_hash: "abcd1234efgh".to_string(),
        effective_seed: 42,
        tick_start: 0,
        tick_end: 100,
        completeness: "full".to_string(),
        warning_codes: vec!["WARN_HIGH_HEAT".to_string()],
        rows_count: 0,
    };

    let mut dataset = AnalyticsDataset::new(manifest);

    // Populate sample analytics rows
    for tick in 0..=10 {
        dataset.population.push(PopulationAnalyticsRow {
            tick,
            alive_cells: 10 + (tick as usize) * 2,
            stressed_cells: 1,
            dead_cells: (tick / 5) as usize,
            total_cells_ever: 10 + (tick as usize) * 2 + (tick / 5) as usize,
            births_count: 2,
            deaths_count: if tick % 5 == 0 { 1 } else { 0 },
        });

        dataset.balance.push(BalanceAnalyticsRow {
            tick,
            total_system_mass: 1000.0,
            total_cell_resources: 200.0 + (tick as f64) * 10.0,
            total_environment_resources: 800.0 - (tick as f64) * 10.0,
            total_energy_capacity: 500.0,
            total_energy_current: 350.0,
            unaccounted_difference: 0.0,
        });

        dataset.lineage.push(LineageAnalyticsRow {
            tick,
            total_lineages: 3,
            active_genomes_count: 3,
            total_mutations_count: tick as usize,
            total_divisions_count: (tick as usize) * 2,
        });

        dataset.environment.push(EnvironmentAnalyticsRow {
            tick,
            heat_current: 25.0 + (tick as f32) * 0.5,
            heat_generated: 0.5,
            waste_current: 5.0 + (tick as f32) * 0.2,
            waste_generated: 0.2,
        });
    }

    dataset.update_rows_count();
    assert_eq!(dataset.manifest.rows_count, 11);

    // Test JSON Serialization
    let json_output = AnalyticsExporter::to_json(&dataset).expect("JSON serialization failed");
    assert!(json_output.contains("test_run_101"));
    assert!(json_output.contains("living_ecosystem"));
    assert!(json_output.contains("WARN_HIGH_HEAT"));

    // Test CSV Format Outputs
    let pop_csv = AnalyticsExporter::to_population_csv(&dataset);
    assert!(pop_csv.starts_with(
        "tick,alive_cells,stressed_cells,dead_cells,total_cells_ever,births_count,deaths_count\n"
    ));
    assert!(pop_csv.contains("0,10,1,0,10,2,1\n"));
    assert!(pop_csv.contains("10,30,1,2,32,2,1\n"));

    let bal_csv = AnalyticsExporter::to_balance_csv(&dataset);
    assert!(bal_csv.starts_with("tick,total_system_mass,total_cell_resources,total_environment_resources,total_energy_capacity,total_energy_current,unaccounted_difference\n"));
    assert!(
        bal_csv.contains("0,1000.000000,200.000000,800.000000,500.000000,350.000000,0.000000\n")
    );

    let lin_csv = AnalyticsExporter::to_lineage_csv(&dataset);
    assert!(lin_csv.starts_with(
        "tick,total_lineages,active_genomes_count,total_mutations_count,total_divisions_count\n"
    ));
    assert!(lin_csv.contains("10,3,3,10,20\n"));

    let env_csv = AnalyticsExporter::to_environment_csv(&dataset);
    assert!(
        env_csv.starts_with("tick,heat_current,heat_generated,waste_current,waste_generated\n")
    );
    assert!(env_csv.contains("0,25.0000,0.5000,5.0000,0.2000\n"));
}

#[test]
fn test_analytics_export_to_directory() {
    let output_dir = PathBuf::from("target/test_analytics_export");

    let manifest = AnalyticsExportManifest {
        schema_version: "1.0".to_string(),
        run_id: "export_dir_run".to_string(),
        scenario_id: "test_scenario".to_string(),
        config_hash: "hash123".to_string(),
        effective_seed: 99,
        tick_start: 0,
        tick_end: 5,
        completeness: "full".to_string(),
        warning_codes: Vec::new(),
        rows_count: 1,
    };

    let mut dataset = AnalyticsDataset::new(manifest);
    dataset.population.push(PopulationAnalyticsRow {
        tick: 0,
        alive_cells: 5,
        stressed_cells: 0,
        dead_cells: 0,
        total_cells_ever: 5,
        births_count: 5,
        deaths_count: 0,
    });
    dataset.update_rows_count();

    let export_files = AnalyticsExporter::export_to_dir(&dataset, &output_dir)
        .expect("Directory export must succeed");

    assert!(export_files.manifest_json_path.exists());
    assert!(export_files.dataset_json_path.exists());
    assert!(export_files.population_csv_path.exists());
    assert!(export_files.balance_csv_path.exists());
    assert!(export_files.lineage_csv_path.exists());
    assert!(export_files.environment_csv_path.exists());

    let read_manifest = fs::read_to_string(&export_files.manifest_json_path).unwrap();
    assert!(read_manifest.contains("export_dir_run"));

    let read_pop_csv = fs::read_to_string(&export_files.population_csv_path).unwrap();
    assert!(read_pop_csv.contains("0,5,0,0,5,5,0"));

    // Cleanup
    let _ = fs::remove_dir_all(&output_dir);
}
