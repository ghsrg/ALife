use alife::core::fields::FieldLayerIndex;
use alife::core::tick::TickExecutor;
use alife::core::units::GridCoord;
use alife::runner::config_parser::RawScenarioConfig;

fn config_with_world_cadence(decay_ticks: u64) -> alife::core::config::RuntimeConfig {
    RawScenarioConfig::parse(&format!(
        r#"
scenario_id = "scheduler_world_cadence"
seed = 12
tick_count = 20

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [100.0]
optional_decay_rate = 0.01

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 1.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }}
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

[scheduler.world]
resource_decay_ticks = {decay_ticks}
"#
    ))
    .unwrap()
}

fn config_with_field_cadence(field_update_ticks: u64) -> alife::core::config::RuntimeConfig {
    RawScenarioConfig::parse(&format!(
        r#"
scenario_id = "scheduler_field_cadence"
seed = 12
tick_count = 20

[world]
size = [32.0, 32.0]

[space]
spatial_grid_size = 8.0
physics_solver_iterations = 2

[resources]
resource_type_ids = ["nutrient_A"]
initial_distribution = [100.0]
optional_decay_rate = 0.0

[fields.temperature]
kind = "scalar"
initial_value = 80.0
diffusion_rate = 0.0
decay_rate = 0.1
min_value = 0.0
max_value = 100.0
effect_profile = "temperature"
conserved_behavior = "abstracted"

[cell]
initial_position = [16.0, 16.0]
radius = 1.0
initial_resources = {{ nutrient_A = 1.0 }}
initial_materials = {{ boundary = 1.0, transport = 1.0, metabolic = 1.0, storage = 1.0, synthesis = 0.0, structural = 1.0, repair = 0.0, contractile = 0.0, sensory = 0.0 }}
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

[scheduler.world]
resource_decay_ticks = 1
field_update_ticks = {field_update_ticks}
"#
    ))
    .unwrap()
}

#[test]
fn resource_decay_runs_only_when_due_and_reports_elapsed_ticks() {
    let mut executor = TickExecutor::new(config_with_world_cadence(5)).unwrap();

    for tick in 1..5 {
        let summary = executor.step().unwrap();
        assert_eq!(
            summary.metrics.resource_decay_scheduler_elapsed_ticks, 0,
            "tick {tick}"
        );
        assert_eq!(summary.metrics.resource_decay_amount, 0.0, "tick {tick}");
    }

    let fifth = executor.step().unwrap();
    assert_eq!(fifth.metrics.resource_decay_scheduler_elapsed_ticks, 5);
    assert!(fifth.metrics.resource_decay_amount > 0.0);
}

#[test]
fn field_update_runs_only_when_due_and_integrates_elapsed_ticks() {
    let mut executor = TickExecutor::new(config_with_field_cadence(5)).unwrap();
    let layer = FieldLayerIndex::from_raw(0);
    let coord = GridCoord::new(0, 0);

    for tick in 1..5 {
        executor.step().unwrap();
        let value = executor
            .world()
            .fields()
            .unwrap()
            .value_at(layer, coord)
            .unwrap();
        assert_eq!(value.raw(), 80.0, "tick {tick}");
    }

    executor.step().unwrap();
    let value = executor
        .world()
        .fields()
        .unwrap()
        .value_at(layer, coord)
        .unwrap();
    assert!(
        (value.raw() - 47.2392).abs() < 0.001,
        "field decay must integrate elapsed ticks instead of slowing decay"
    );
}

#[test]
fn scheduled_decay_preserves_elapsed_tick_semantics() {
    let mut every_tick = TickExecutor::new(config_with_world_cadence(1)).unwrap();
    let mut scheduled = TickExecutor::new(config_with_world_cadence(5)).unwrap();

    for _ in 0..5 {
        every_tick.step().unwrap();
        scheduled.step().unwrap();
    }

    let every_tick_total = every_tick.world().resources().total_amount_for_test();
    let scheduled_total = scheduled.world().resources().total_amount_for_test();
    assert!(
        (every_tick_total - scheduled_total).abs() < 0.001,
        "scheduled decay must integrate elapsed ticks instead of slowing decay"
    );
}
