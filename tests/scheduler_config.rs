use alife::runner::config_parser::RawScenarioConfig;

fn base_config(extra: &str) -> String {
    format!(
        r#"
scenario_id = "scheduler_config_test"
seed = 7
tick_count = 100

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [10.0]
optional_decay_rate = 0.0

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 2.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 1.0, structural = 1.0, repair = 1.0, contractile = 0.0, sensory = 0.0 }}
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.1
capacity_limit = 30.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 20.0
heat_death_threshold = 40.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 20.0
waste_death_threshold = 40.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 40.0

[resource_interaction]
enabled = true
uptake_layer_index = 0
max_uptake_per_tick = 1.0
metabolism_resource_per_tick = 1.0
energy_per_resource = 1.0
heat_per_resource = 0.0
waste_per_resource = 0.0

{extra}
"#
    )
}

#[test]
fn scheduler_config_defaults_to_current_compatibility_when_missing() {
    let config = RawScenarioConfig::parse(&base_config("")).unwrap();

    assert_eq!(config.scheduler.cell.genome_runtime_base_ticks, 1);
    assert_eq!(config.scheduler.world.resource_diffusion_ticks, 1);
    assert_eq!(config.scheduler.observer.resource_totals_ticks, 1);
}

#[test]
fn scheduler_config_parses_explicit_cadence_blocks() {
    let config = RawScenarioConfig::parse(&base_config(
        r#"
[time]
tick_duration_ms = 100

[scheduler.cell]
genome_runtime_base_ticks = 10
genome_runtime_ticks_per_layer = 10

[scheduler.world]
resource_diffusion_ticks = 2
resource_decay_ticks = 5
passive_reactions_ticks = 2

[scheduler.observer]
observer_metrics_ticks = 10
resource_totals_ticks = 10
graph_analysis_ticks = 50
"#,
    ))
    .unwrap();

    assert_eq!(config.simulation_time.tick_duration_ms, 100);
    assert_eq!(config.scheduler.cell.genome_runtime_base_ticks, 10);
    assert_eq!(config.scheduler.world.resource_diffusion_ticks, 2);
    assert_eq!(config.scheduler.world.resource_decay_ticks, 5);
    assert_eq!(config.scheduler.observer.graph_analysis_ticks, 50);
}

#[test]
fn scheduler_config_rejects_zero_cadence() {
    let err = RawScenarioConfig::parse(&base_config(
        r#"
[scheduler.cell]
genome_runtime_base_ticks = 0
"#,
    ))
    .unwrap_err();

    assert!(format!("{err:?}").contains("InvalidSchedulerCadence"));
}
