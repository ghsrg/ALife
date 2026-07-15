use alife::core::tick::TickExecutor;
use alife::runner::config_parser::RawScenarioConfig;

#[test]
fn resource_totals_are_marked_stale_between_observer_cadence_ticks() {
    let config = RawScenarioConfig::parse(
        r#"
scenario_id = "scheduler_observer_cadence"
seed = 13
tick_count = 20

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
initial_resources = { nutrient_A = 1.0 }
initial_materials = { boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }
initial_energy = 10.0
energy_capacity = 20.0
mandatory_cost_per_tick = 0.01
capacity_limit = 20.0

[environment]
heat_current = 0.0
heat_generated_per_tick = 0.0
heat_dissipation_rate = 0.1
heat_warning_threshold = 50.0
heat_death_threshold = 100.0
waste_current = 0.0
waste_generated_per_tick = 0.0
waste_sink_rate = 0.1
waste_warning_threshold = 50.0
waste_death_threshold = 100.0

[lifecycle]
stress_energy_threshold = 1.0
dormancy_allowed = false
critical_capacity_overrun = 50.0

[scheduler.observer]
resource_totals_ticks = 10
"#,
    )
    .unwrap();

    let mut executor = TickExecutor::new(config).unwrap();
    for _ in 0..9 {
        let summary = executor.step().unwrap();
        assert!(summary.tick_accounting.conservation_delta_abs < 0.001);
        assert!(!summary.observer_projection.resource_totals_recomputed);
    }

    let tenth = executor.step().unwrap();
    assert!(tenth.observer_projection.resource_totals_recomputed);
}
