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
    let csv_content = std::fs::read_to_string(&csv_path).unwrap();
    let header = csv_content.lines().next().unwrap();
    for column in [
        "death_tick",
        "first_decomposition_tick",
        "first_decomposed_tick",
        "decomposition_ticks",
        "decomposition_released_resources_per_tick",
        "time_to_decomposed",
        "remaining_dead_cell_resources",
        "remaining_dead_cell_materials",
    ] {
        assert!(
            header.split(',').any(|h| h == column),
            "missing decomposition CSV column: {}",
            column
        );
    }

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

#[test]
fn test_decomposition_sweep_rate_changes_timing_metrics() {
    let out_dir = "target/test_decomposition_sweep_outputs";
    std::fs::create_dir_all(out_dir).unwrap();

    let toml_str = r#"
[run]
output_dir = "target/test_decomposition_sweep_outputs"
seed = 42
ticks = 200

[cell]
radius = 1.0
initial_energy = 1.0
energy_capacity = 100.0
mandatory_cost_per_tick = 50.0
passive_energy_income = 0.0
capacity_limit = 30.0
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
energy_per_resource = 10.0
heat_per_resource = 0.1
waste_per_resource = 0.1
decay_rate = 0.0
default_resource_density = 0.001
default_max_uptake_per_tick = 1.0
default_metabolism_resource_per_tick = 0.5

[environment]
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_sink_rate = 0.1
waste_warning_threshold = 10.0
waste_death_threshold = 20.0

[scenarios.decomposition_viability]
world_size = [64.0, 64.0]
cell_position = [32.0, 32.0]
initial_resources = [0.001]
decomposition_enabled = true
initial_energy = 1.0
initial_cell_resources = 20.0
dormancy_allowed = false
mandatory_cost_per_tick = 50.0
decomposition_resources_per_tick = 1.0
decomposition_materials_per_tick = 1.0
continue_after_collapse_ticks = 80

[[sweep]]
name = "decomposition_rate_probe"
scenario = "decomposition_viability"
param = "decomposition_resources_per_tick"
from = 1.0
to = 5.0
steps = 2
"#;

    let cfg: alife::bin::sweep_analyzer::AnalyzerConfig = toml::from_str(toml_str).unwrap();
    let sweep = &cfg.sweep.as_ref().unwrap()[0];
    let preset = cfg
        .scenarios
        .as_ref()
        .unwrap()
        .get("decomposition_viability");

    alife::bin::sweep_analyzer::run_sweep(&cfg, sweep, preset, out_dir);

    let csv_path = format!("{}/decomposition_rate_probe.csv", out_dir);
    let csv_content = std::fs::read_to_string(&csv_path).unwrap();
    let mut lines = csv_content.lines();
    let header: Vec<&str> = lines.next().unwrap().split(',').collect();
    let rows: Vec<Vec<&str>> = lines.map(|line| line.split(',').collect()).collect();
    assert_eq!(rows.len(), 2);

    let idx = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .unwrap_or_else(|| panic!("missing column {name}"))
    };
    let value = |row: usize, name: &str| rows[row][idx(name)].parse::<f32>().unwrap();

    let slow_time = value(0, "time_to_decomposed");
    let fast_time = value(1, "time_to_decomposed");
    let slow_rate = value(0, "decomposition_released_resources_per_tick");
    let fast_rate = value(1, "decomposition_released_resources_per_tick");

    assert!(slow_time - fast_time >= 5.0 || slow_time >= fast_time * 2.0);
    assert!(fast_rate > slow_rate);
    assert!(value(0, "first_decomposition_tick") >= value(0, "death_tick"));
    assert!(value(1, "first_decomposition_tick") >= value(1, "death_tick"));
}

#[test]
fn test_division_sweep_has_clean_dividing_survivor() {
    let out_dir = "target/test_division_sweep_outputs";
    std::fs::create_dir_all(out_dir).unwrap();

    let toml_str = r#"
[run]
output_dir = "target/test_division_sweep_outputs"
seed = 42
ticks = 2000

[cell]
radius = 1.0
initial_energy = 50.0
energy_capacity = 120.0
mandatory_cost_per_tick = 2.0
passive_energy_income = 0.0
capacity_limit = 80.0
initial_metabolic_material = 1.0
initial_transport_material = 1.0
initial_boundary_material = 1.0
initial_structural_material = 1.0

[lifecycle]
stress_energy_threshold = 10.0
dormancy_allowed = true
dormant_mandatory_cost_modifier = 0.1
critical_capacity_overrun = 40.0

[resource_interaction]
energy_per_resource = 10.0
heat_per_resource = 0.1
waste_per_resource = 0.1
decay_rate = 0.0
default_resource_density = 10.0
default_max_uptake_per_tick = 1.0
default_metabolism_resource_per_tick = 0.5

[environment]
heat_dissipation_rate = 0.2
heat_warning_threshold = 50.0
heat_death_threshold = 80.0
waste_sink_rate = 0.8
waste_warning_threshold = 10.0
waste_death_threshold = 100.0

[scenarios.division_viability]
growth_enabled = true
division_enabled = true
initial_resources = [4000.0]
passive_energy_income = 0.0
max_uptake_per_tick = 0.55
metabolism_resource_per_tick = 0.5
capacity_limit = 80.0
critical_capacity_overrun = 40.0
heat_warning_threshold = 4000.0
heat_death_threshold = 5000.0
waste_death_threshold = 100.0
waste_sink_rate = 0.8
division_energy_cost = 10.0

[[sweep]]
name = "division_clean_probe"
scenario = "division_viability"
param = "max_division_pressure"
from = 0.1
to = 0.8
steps = 3
"#;

    let cfg: alife::bin::sweep_analyzer::AnalyzerConfig = toml::from_str(toml_str).unwrap();
    let sweep = &cfg.sweep.as_ref().unwrap()[0];
    let preset = cfg.scenarios.as_ref().unwrap().get("division_viability");

    alife::bin::sweep_analyzer::run_sweep(&cfg, sweep, preset, out_dir);

    let csv_path = format!("{}/division_clean_probe.csv", out_dir);
    let csv_content = std::fs::read_to_string(&csv_path).unwrap();
    let mut lines = csv_content.lines();
    let header: Vec<&str> = lines.next().unwrap().split(',').collect();
    let rows: Vec<Vec<&str>> = lines.map(|line| line.split(',').collect()).collect();
    assert_eq!(rows.len(), 3);

    let idx = |name: &str| {
        header
            .iter()
            .position(|h| *h == name)
            .unwrap_or_else(|| panic!("missing column {name}"))
    };

    let has_clean_survivor = rows.iter().any(|row| {
        row[idx("survived_to_end")] == "true"
            && !row[idx("warning_codes")].contains("LOW_INFORMATION_SWEEP")
            && row[idx("divisions_count")].parse::<u32>().unwrap() > 0
            && row[idx("births_count")].parse::<u32>().unwrap() > 0
            && row[idx("energy_spent_division")].parse::<f32>().unwrap() > 0.0
    });

    assert!(has_clean_survivor);
}
