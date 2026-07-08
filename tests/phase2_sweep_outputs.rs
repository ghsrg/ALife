use std::io::BufRead;

#[test]
fn test_raw_csv_contains_no_comment_lines() {
    let out_dir = "target/test_sweep_outputs";
    std::fs::create_dir_all(out_dir).unwrap();

    let toml_str = r#"
[run]
output_dir = "target/test_sweep_outputs"
seed = 42
ticks = 10

[cell]
radius = 1.0
initial_energy = 50.0
energy_capacity = 100.0
mandatory_cost_per_tick = 1.0
passive_energy_income = 0.0
capacity_limit = 20.0
initial_metabolic_material = 1.0
initial_transport_material = 1.0
initial_boundary_material = 1.0
initial_structural_material = 1.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.1
critical_capacity_overrun = 5.0

[resource_interaction]
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0
decay_rate = 0.0
default_resource_density = 1.0
default_max_uptake_per_tick = 1.0
default_metabolism_resource_per_tick = 1.0

[environment]
heat_dissipation_rate = 0.1
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[[sweep]]
name = "test_smoke_sweep"
param = "resource_density"
from = 1.0
to = 2.0
steps = 2
"#;

    let cfg: alife::bin::sweep_analyzer::AnalyzerConfig = toml::from_str(toml_str).unwrap();
    let sweep = &cfg.sweep.as_ref().unwrap()[0];

    // Run the sweep
    alife::bin::sweep_analyzer::run_sweep(&cfg, sweep, None, out_dir);

    // Read the generated CSV file
    let csv_path = format!("{}/test_smoke_sweep.csv", out_dir);
    let file = std::fs::File::open(&csv_path).unwrap();
    let reader = std::io::BufReader::new(file);

    for line in reader.lines() {
        let line = line.unwrap();
        assert!(
            !line.trim().starts_with('#'),
            "CSV file contains commented summary lines: {}",
            line
        );
    }

    // Also assert that the sweep_scenario_summary.csv is created in output dir
    let summary_path = format!("{}/sweep_scenario_summary.csv", out_dir);
    assert!(
        std::path::Path::new(&summary_path).exists(),
        "sweep_scenario_summary.csv was not created!"
    );
}
