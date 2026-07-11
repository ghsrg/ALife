use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn parser_loads_local_interaction_config() {
    let toml = r#"
scenario_id = "local_interaction_parser"
seed = 7
tick_count = 10

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 1

[resources]
resource_type_ids = ["r"]
initial_distribution = [0.0]
optional_decay_rate = 0.0
passive_energy_income_placeholder = 0.0

[cell]
initial_position = [10.0, 10.0]
radius = 1.0
initial_resources = {"r" = 0.0}
initial_materials = {boundary = 1.0, transport = 1.0, metabolic = 0.0, storage = 0.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 1.0}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.0
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.0
heat_warning_threshold = 100.0
heat_death_threshold = 200.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.0
waste_warning_threshold = 100.0
waste_death_threshold = 200.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
dormant_mandatory_cost_modifier = 1.0
critical_capacity_overrun = 100.0

[resource_interaction]
enabled = false
uptake_layer_index = 0
max_uptake_per_tick = 0.0
metabolism_resource_per_tick = 0.0
energy_per_resource = 0.0
heat_per_resource = 0.0
waste_per_resource = 0.0

[local_interaction]
enabled = true
contact_exchange_rate = 0.5
max_exchange_per_pair = 2.0
min_boundary_capability = 0.1
min_transport_capability = 0.1
contact_stimulus_per_overlap = 0.25
stimulus_decay_per_tick = 0.5
"#;

    let config = RawScenarioConfig::parse(toml).unwrap();
    assert!(config.local_interaction.enabled);
    assert_eq!(config.local_interaction.contact_exchange_rate, 0.5);
    assert_eq!(config.local_interaction.max_exchange_per_pair.raw(), 2.0);
    assert_eq!(config.local_interaction.contact_stimulus_per_overlap, 0.25);
}
