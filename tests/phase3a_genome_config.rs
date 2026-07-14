use alife::core::genome::GenomeOutputId;
use alife::runner::config_parser::{ParseError, RawScenarioConfig};

fn fixture_with_genome(extra: &str) -> String {
    format!(
        r#"
scenario_id = "phase3a_genome_bootstrap"
seed = 42
tick_count = 5
legacy_material_distribution = false

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 1

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 2.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 1.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 1.0
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 2.0
dormancy_allowed = false
critical_capacity_overrun = 5.0

{extra}
"#
    )
}

#[test]
fn parser_loads_genome_template_and_cell_assignment() {
    let config = RawScenarioConfig::parse(&fixture_with_genome(
        r#"
[genome_templates.balanced]
variation_amplitude = 0.08
runtime_interval_ticks = 1

[genome_templates.balanced.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0

[genome_templates.balanced.outputs]
resource_uptake_priority = 0.7
energy_conversion_priority = 0.6
material_synthesis_priority = 0.2
repair_priority = 0.1

[cell.genome]
template = "balanced"
"#,
    ))
    .unwrap();

    assert_eq!(config.genome_templates.len(), 1);
    assert_eq!(config.genome_templates[0].id().as_str(), "balanced");
    assert_eq!(
        config.genome_templates[0].outputs()[0].0,
        GenomeOutputId::EnergyConversionPriority
    );
    assert_eq!(
        config.initial_cell_genome_templates[0]
            .as_ref()
            .unwrap()
            .as_str(),
        "balanced"
    );
}

#[test]
fn parser_rejects_unknown_genome_output_id() {
    let err = RawScenarioConfig::parse(&fixture_with_genome(
        r#"
[genome_templates.bad]
variation_amplitude = 0.08
runtime_interval_ticks = 1
[genome_templates.bad.carrier]
material_id = "genome_carrier_A"
amount = 1.0
integrity = 1.0
[genome_templates.bad.outputs]
joint_create_priority = 0.2
[cell.genome]
template = "bad"
"#,
    ))
    .unwrap_err();

    assert!(
        matches!(err, ParseError::ValidationError(message) if message.contains("Unknown Genome output"))
    );
}

#[test]
fn parser_rejects_unknown_cell_genome_template() {
    let err = RawScenarioConfig::parse(&fixture_with_genome(
        r#"
[cell.genome]
template = "missing"
"#,
    ))
    .unwrap_err();

    assert!(
        matches!(err, ParseError::ValidationError(message) if message.contains("Unknown Genome template"))
    );
}

#[test]
fn phase3a_demo_scenario_parses_with_genome_template() {
    let text = std::fs::read_to_string("config/scenarios/genome/phase3a_genome_bootstrap.toml")
        .unwrap();
    let config = RawScenarioConfig::parse(&text).unwrap();

    assert_eq!(config.genome_templates.len(), 1);
    assert!(
        config
            .initial_cell_genome_templates
            .iter()
            .any(|id| id.is_some())
    );
}
